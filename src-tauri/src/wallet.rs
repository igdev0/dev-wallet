use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use crate::account::{Account, AccountBuilder};

use crate::config::Config;
use crate::storage::DbFacadePool;
use crate::utils::{decrypt, encrypt};
use crate::WalletError;

use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};

use sqlx::Row;

pub struct Wallet {
    // We need the seed to create the master key
    pub name: String,
    seed: String,
    id: Option<String>,
    // We need the accounts field to store the keys a.k.a "accounts"
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

    pub fn remove_account(&self) {}

    fn encrypted_seed(&self) -> String {
        let config = Config::from_env();
        let mut key = [0u8; 32];
        key.copy_from_slice(config.database_key.as_bytes());
        encrypt(&key, self.seed.as_bytes()).to_hex_string(Case::Lower)
    }

    pub async fn load(&self, db: &DbFacadePool) -> Result<Wallet, WalletError> {
        let config = Config::from_env();
        let result = sqlx::query("SELECT * from wallets WHERE name = ?;")
            .bind(&self.name)
            .fetch_one(db)
            .await;
        if let Ok(data) = result {
            let id: String = data.get("id");
            let account_results = sqlx::query("SELECT * from accounts WHERE wallet_id = ?;")
                .bind(&id)
                .fetch_all(db)
                .await
                .expect("Wasn't able to fetch accounts for wallet");
            let mut accounts = vec![];
            for acc in account_results.iter() {
                accounts.push(Account::from_entry(acc));
            }

            let wallet_name: String = data.get("name");
            let seed: String = data.get("seed");
            let mut key = [0u8; 32];
            key.copy_from_slice(config.database_key.as_bytes());
            let decoded_seed = hex::decode(seed).unwrap();
            return Ok(Wallet {
                name: wallet_name,
                id: Some(id),
                accounts: Rc::new(RefCell::new(accounts)),
                passphrase: None,
                seed: decrypt(&key, &decoded_seed).to_hex_string(Case::Lower),
            });
        }
        Err(WalletError::NotFound)
    }

    pub async fn authenticate() {}

    pub async fn save(&self, db: &DbFacadePool) {
        let id = uuid::Uuid::new_v4().to_string();
        let password = &self.passphrase.as_ref().unwrap();
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

    pub fn passphrase(&mut self, pass: String) {
        self.passphrase = Some(pass);
    }

    pub fn mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = Some(mnemonic);
    }

    pub fn name(&mut self, name: String) {
        self.name = Some(name);
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
