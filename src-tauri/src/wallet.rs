use crate::account::{Account, AccountBuilder, AccountType};
use crate::storage::StorageMethods;
use async_std::sync::Mutex;
use bip32::{
    secp256k1::{ecdsa::SigningKey, elliptic_curve::PublicKey, SecretKey},
    DerivationPath, ExtendedPrivateKey, ExtendedPublicKey, PrivateKey,
};
use bip39::Mnemonic;
use bitcoin::{hashes::sha256::Hash, taproot::NodeInfo, Address, NetworkKind, PubkeyHash};
use sqlx::{any, SqliteConnection};
use std::str::FromStr;

pub struct Wallet {
    // We need the seed to create the master key
    seed: [u8; 64],
    master: ExtendedPrivateKey<SecretKey>,
    // We need the accounts field to store the keys a.k.a "accounts"
    accounts: Mutex<Vec<Account>>,
    network_kind: NetworkKind,
}

impl Wallet {
    pub fn create_account(&self, account_type: AccountType) -> Account {
        AccountBuilder::build()
    }

    pub fn remove_account(&self) {}
}

impl StorageMethods for Wallet {
    fn load(&self, db: SqliteConnection) {}

    fn save(&self, db: SqliteConnection) {}
}

pub struct WalletBuilder {
    mnemonic: Option<String>,
    network_kind: Option<NetworkKind>,
    passphrase: Option<String>,
}

impl WalletBuilder {
    pub fn new(mnemonic: &str) -> WalletBuilder {
        WalletBuilder {
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

    pub fn network_kind(&mut self, nk: NetworkKind) {
        self.network_kind = Some(nk);
    }

    pub fn build(mut self) -> Wallet {
        let passphrase = &self.passphrase.unwrap();
        let mnemonic = &self.mnemonic.unwrap();
        let seed = Mnemonic::from_str(&mnemonic)
            .unwrap()
            .to_seed(passphrase.to_string());

        let master = bip32::ExtendedPrivateKey::new(seed).unwrap();

        Wallet {
            seed,
            master,
            network_kind: self.network_kind.unwrap(),
            accounts: Mutex::new(Vec::new()),
        }
    }
}
