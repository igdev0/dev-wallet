use crate::account::{Account, AccountBuilder, AccountType};
use crate::storage::DbFacadePool;
use crate::WalletError;
use async_std::sync::Mutex;
use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use sqlx::Row;
use std::str::FromStr;

pub struct Wallet {
    // We need the seed to create the master key
    pub name: String,
    seed: String,
    // We need the accounts field to store the keys a.k.a "accounts"
    accounts: Mutex<Vec<Account>>,
    passphrase: Option<String>,
}

impl Wallet {
    pub fn create_account(&self, account_type: AccountType) -> Account {
        AccountBuilder::build()
    }

    pub fn remove_account(&self) {}
    pub async fn load(&self, db: &DbFacadePool) -> Result<Wallet, WalletError> {
        let result = sqlx::query("SELECT * from wallets WHERE name = ?;")
            .bind(&self.name)
            .fetch_one(db)
            .await;

        if let Ok(data) = result {
            let wallet_name: String = data.get("name");
            let seed: String = data.get("seed");
            return Ok(Wallet {
                name: wallet_name,
                accounts: Mutex::new(Vec::new()),
                passphrase: None,
                seed,
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
            .bind(&self.seed)
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
            name: name.to_string(),
            accounts: Mutex::new(Vec::new()),
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

        // let master_parsed = master;
        Wallet {
            name: self.name.unwrap(),
            seed: seed.to_hex_string(Case::Lower),
            // master: master_parsed.clone().to_string(),
            passphrase: Some(passphrase),
            accounts: Mutex::new(Vec::new()),
        }
    }
}
