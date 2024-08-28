use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

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
use sqlx::Row;

pub struct Wallet {
    pub name: String,
    seed: String,
    id: Option<String>,
    pub accounts: Rc<RefCell<Vec<Account>>>,
    passphrase: Option<String>,
}

impl Wallet {
    pub fn create_account(&self) -> AccountBuilder {
        let mut account_builder = AccountBuilder::new();
        let id = &self.id.clone().expect("Wasn't able to initiate the account builder, make sure you save the wallet first, before trying to create the account.");
        account_builder.wallet_id(id.to_string());
        account_builder.seed(&self.seed);
        account_builder
    }

    fn encrypted_seed(&self) -> String {
        let config = Config::from_env();
        let mut key = [0u8; 32];
        key.copy_from_slice(config.database_key.as_bytes());
        encrypt(&key, self.seed.as_bytes()).to_hex_string(Case::Lower)
    }

    async fn load_accounts(&self, id: &String, db: &DbFacadePool) -> Vec<Account> {
        let account_results = sqlx::query("SELECT * from accounts WHERE wallet_id = ?;")
            .bind(&id)
            .fetch_all(db)
            .await
            .expect("Wasn't able to fetch accounts for wallet");
        let mut accounts = vec![];
        for acc in account_results.iter() {
            accounts.push(Account::from_entry(acc));
        }

        accounts
    }

    pub async fn authenticate(
        &self,
        password: &str,
        db: &DbFacadePool,
    ) -> Result<Wallet, WalletError> {
        let result = sqlx::query("SELECT * FROM wallets WHERE name = ?;")
            .bind(&self.name)
            .fetch_one(db)
            .await;

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
            let accounts = self.load_accounts(&id, db).await;

            let mut key = [0u8; 32];
            key.copy_from_slice(config.database_key.as_bytes());

            return Ok(Wallet {
                id: Some(id),
                name: wallet_name,
                passphrase: None,
                seed: decrypt(&key, &seed).to_hex_string(Case::Lower),
                accounts: Rc::new(RefCell::new(accounts)),
            });
        } else {
            Err(WalletError::NotFound)
        }
    }

    pub async fn save(&self, db: &DbFacadePool) {
        let id = uuid::Uuid::new_v4().to_string();
        let password = &self.passphrase.as_ref().unwrap();
        let salt = SaltString::generate(OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed hashing password")
            .to_string();
        sqlx::query("INSERT into wallets (id, name, seed, password) values(?,?,?,?);")
            .bind(id)
            .bind(self.name.clone())
            .bind(&self.encrypted_seed())
            .bind(password)
            .execute(db)
            .await
            .expect("Wasn't able to save");
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
            accounts: Rc::new(RefCell::new(Vec::new())),
            passphrase: Some("".to_string()),
            seed: "".to_string(),
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
            seed: seed.to_hex_string(Case::Lower),
            passphrase: Some(passphrase),
            accounts: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
