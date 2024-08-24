use crate::account::{Account, AccountBuilder, AccountType};
use crate::storage::{DbFacadePool, StorageMethods};
use async_std::sync::Mutex;
use bip32::Prefix;
use bip32::{
    secp256k1::{ecdsa::SigningKey, elliptic_curve::PublicKey, SecretKey},
    DerivationPath, ExtendedPrivateKey, ExtendedPublicKey, PrivateKey,
};
use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use bitcoin::{hashes::sha256::Hash, taproot::NodeInfo, Address, NetworkKind, PubkeyHash};
use sqlx::database::HasArguments;
use sqlx::{any, SqliteConnection};
use std::borrow::Borrow;
use std::str::FromStr;

pub struct Wallet {
    // We need the seed to create the master key
    name: String,
    seed: [u8; 64],
    master: ExtendedPrivateKey<SecretKey>,
    // We need the accounts field to store the keys a.k.a "accounts"
    accounts: Mutex<Vec<Account>>,
    passphrase: String,
    network_kind: NetworkKind,
}

impl Wallet {
    pub fn create_account(&self, account_type: AccountType) -> Account {
        AccountBuilder::build()
    }

    pub fn remove_account(&self) {}
}

impl StorageMethods for Wallet {
    async fn load(&self, db: &DbFacadePool) {
        let result = sqlx::query("SELECT * from wallets WHERE name = ?;")
            .bind(&self.name)
            .execute(db)
            .await;

        if let Ok(data) = result {
            dbg!(data);
        }
    }

    async fn save(&self, db: &DbFacadePool) {
        let id = uuid::Uuid::new_v4().to_string();
        let master = self.master.to_string(Prefix::XPRV);
        let master = master.as_str();
        let password = &self.passphrase;
        sqlx::query("INSERT into wallets (id, name, seed, master, password) values(?,?,?,?,?);")
            .bind(id)
            .bind(self.name.clone())
            .bind(self.seed.to_hex_string(Case::Lower))
            .bind(master)
            .bind(password)
            .execute(db)
            .await
            .expect("Wasn't able to save");
    }
}

pub struct WalletBuilder {
    name: Option<String>,
    mnemonic: Option<String>,
    network_kind: Option<NetworkKind>,
    passphrase: Option<String>,
}

impl WalletBuilder {
    pub fn new(mnemonic: &str) -> WalletBuilder {
        WalletBuilder {
            name: Some("Default".to_string()),
            network_kind: Some(NetworkKind::Test),
            mnemonic: Some(mnemonic.to_string()),
            passphrase: Some("".to_string()),
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

    pub fn network_kind(&mut self, nk: NetworkKind) {
        self.network_kind = Some(nk);
    }

    pub fn build(self) -> Wallet {
        let passphrase = self.passphrase.unwrap();
        let mnemonic = &self.mnemonic.unwrap();
        let seed = Mnemonic::from_str(&mnemonic)
            .unwrap()
            .to_seed(passphrase.to_string());

        let master = bip32::ExtendedPrivateKey::new(seed).unwrap();

        Wallet {
            name: self.name.unwrap(),
            seed,
            master,
            passphrase: passphrase,
            network_kind: self.network_kind.unwrap(),
            accounts: Mutex::new(Vec::new()),
        }
    }
}
