use std::str::FromStr;
use std::vec;

use crate::account::{Account, AccountBuilder};

use crate::config::Config;
use crate::storage::DbFacadePool;
use crate::utils::{decrypt, encrypt};
use crate::WalletError;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use serde_json::{json, Value};
use sqlx::Row;

pub struct Wallet {
    pub name: String,
    seed: Option<String>,
    pub id: Option<String>,
    pub accounts: Vec<Account>, // Using tokio::sync::Mutex
    passphrase: Option<String>,
}

impl Wallet {
    pub fn create_account(&self) -> AccountBuilder {
        let mut account_builder = AccountBuilder::new();
        let id = &self.id.clone().expect("Wasn't able to initiate the account builder, make sure you save the wallet first, before trying to create the account.");
        let seed = &self.seed.clone().unwrap();
        account_builder.wallet_id(id.to_string());
        account_builder.seed(&seed);
        account_builder
    }

    fn encrypted_seed(&self) -> String {
        let config = Config::from_env();
        let mut key = [0u8; 32];
        let seed = &self.seed.clone().unwrap();
        key.copy_from_slice(config.database_key.as_bytes());
        encrypt(&key, seed.as_bytes()).to_hex_string(Case::Lower)
    }

    pub async fn load_accounts(&mut self, db: &DbFacadePool) -> Wallet {
        let account_results = sqlx::query("SELECT * from accounts WHERE wallet_id = ?;")
            .bind(&self.id)
            .fetch_all(db)
            .await
            .expect("Wasn't able to fetch accounts for wallet");
        let mut accounts = vec![];
        for acc in account_results.iter() {
            accounts.push(Account::from_entry(acc));
        }

        Wallet {
            accounts,
            id: self.id.clone(),
            name: self.name.clone(),
            passphrase: self.passphrase.clone(),
            seed: self.seed.clone(),
        }
    }

    pub fn serialize_res(&self) -> Value {
        json!({
        "id": self.id,
        "name": self.name,
        })
    }

    pub async fn authenticate(
        &mut self,
        password: &str,
        db: &DbFacadePool,
    ) -> Result<Wallet, WalletError> {
        let query = {
            if let Some(_) = &self.id {
                "SELECT * FROM wallets WHERE wallet_id = ?;"
            } else {
                "SELECT * FROM wallets WHERE name = ?;"
            }
        };

        let bind = {
            if let Some(id) = &self.id {
                id
            } else {
                &self.name
            }
        };

        let result = sqlx::query(query).bind(bind).fetch_one(db).await;

        if let Ok(data) = result {
            let id: String = data.get("id");
            let wallet_name: String = data.get("name");
            let password_hash: String = data.get("password");
            let argon2 = Argon2::default();
            let parsed_password =
                PasswordHash::new(&password_hash).expect("Failed to parse password");

            let is_valid = argon2.verify_password(password.as_bytes(), &parsed_password);
            if let Err(_) = is_valid {
                return Err(WalletError::AuthenticationFailed(
                    "Password invalid".to_string(),
                ));
            }
            let seed: String = data.get("seed");
            let seed = hex::decode(seed).unwrap();
            let config = Config::from_env();

            let mut key = [0u8; 32];
            key.copy_from_slice(config.database_key.as_bytes());

            return Ok(Wallet {
                id: Some(id),
                name: wallet_name,
                passphrase: None,
                seed: Some(decrypt(&key, &seed).to_hex_string(Case::Lower)),
                accounts: vec![],
            });
        }
        Err(WalletError::NotFound)
    }

    pub async fn save(&self, db: &DbFacadePool) -> Result<Wallet, WalletError> {
        let id = uuid::Uuid::new_v4().to_string();
        let password = &self.passphrase.as_ref().unwrap();
        let salt = SaltString::generate(OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed hashing password")
            .to_string();
        let result = sqlx::query("INSERT into wallets (id, name, seed, password) values(?,?,?,?);")
            .bind(id.clone())
            .bind(self.name.clone())
            .bind(&self.encrypted_seed())
            .bind(password)
            .execute(db)
            .await;
        if let Err(_) = result {
            return Err(WalletError::InvalidInput(
                "Invalid input provided".to_string(),
            ));
        }

        Ok(Wallet {
            accounts: vec![],
            name: self.name.clone(),
            id: Some(id),
            seed: self.seed.clone(),
            passphrase: self.passphrase.clone(),
        })
    }
}

pub struct WalletBuilder {
    name: Option<String>,
    mnemonic: Option<String>,
    passphrase: Option<String>,
}

impl WalletBuilder {
    pub fn new(mnemonic: &str) -> WalletBuilder {
        WalletBuilder {
            name: Some("Default".to_string()),
            mnemonic: Some(mnemonic.to_string()),
            passphrase: Some("".to_string()),
        }
    }

    pub fn from_existing(name: &str) -> Wallet {
        Wallet {
            id: None,
            name: name.to_string(),
            accounts: vec![], // Use tokio::sync::Mutex here too
            passphrase: Some("passpharse".to_string()),
            seed: None,
        }
    }

    pub fn from_existing_id(id: &str) -> Wallet {
        Wallet {
            id: Some(id.to_string()),
            name: "".to_string(),
            accounts: vec![], // Use tokio::sync::Mutex here too
            passphrase: Some("passpharse".to_string()),
            seed: None,
        }
    }

    pub fn passphrase(&mut self, pass: &str) {
        self.passphrase = Some(pass.to_string());
    }

    pub fn mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = Some(mnemonic);
    }

    pub fn name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn build(self) -> Wallet {
        let passphrase = self.passphrase.unwrap();
        let mnemonic = &self.mnemonic.unwrap();
        let seed = Mnemonic::from_str(&mnemonic)
            .unwrap()
            .to_seed(passphrase.to_string());

        Wallet {
            id: None,
            name: self.name.unwrap(),
            seed: Some(seed.to_hex_string(Case::Lower)),
            passphrase: Some(passphrase),
            accounts: vec![], // Use tokio::sync::Mutex
        }
    }
}
