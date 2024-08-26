use bip32::{DerivationPath, PublicKey, XPrv};
use bitcoin::{hashes::Hash, Address, CompressedPublicKey, Network, PubkeyHash};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;

use crate::{path_builder::PathBuilder, storage::DbFacadePool, WalletError};

pub enum AccountType {
    Receiving,
    Spending,
}

pub enum AddressKind {
    Legacy,       // P2PKH
    SegWit,       // P2SH-P2WPKH
    NativeSegWit, // P2WPKH
    Unknown,
}

trait AddressType_ {
    fn address_type(&self) -> AddressKind {
        AddressKind::SegWit
    }
}

impl AddressType_ for DerivationPath {
    fn address_type(&self) -> AddressKind {
        let path = &self.to_string();
        let parts: Vec<&str> = path.split('/').collect();

        // Check if the path is valid (at least 5 components)
        if parts.len() < 5 {
            return AddressKind::Unknown;
        }

        // Check if the first part is "m" (master key)
        if parts[0] != "m" {
            return AddressKind::Unknown;
        }

        // Match based on the purpose field (second part)
        match parts[1] {
            "44'" => AddressKind::Legacy,
            "49'" => AddressKind::SegWit,
            "84'" => AddressKind::NativeSegWit,
            // "86'" => AddressKind::Taproot, // we don't support taproot just yet
            _ => AddressKind::Unknown,
        }
    }
}

pub struct Account {
    // "path" is a string that contains the path for the derived key e.g: m/44/0'/0'/0'/1
    pub path: String,
    pub index: u32,
    pub address: String,
}

impl Account {
    fn new() {}
    fn create_next(&self) -> Account {
        Account {
            address: "".to_string(),
            path: "".to_string(),
            index: 0,
        }
    }

    async fn save(&self, db: &DbFacadePool) {
        // let a =
    }
}

pub struct AccountBuilder {
    path: DerivationPath,
    index: Option<u32>,
    network: Network,
    seed: Option<[u8; 64]>,
    address: Option<String>,
}

impl AccountBuilder {
    pub fn new() -> Self {
        AccountBuilder {
            index: Some(0),
            network: Network::Testnet,
            address: None,
            path: PathBuilder::new().build(),
            seed: None,
        }
    }
    pub fn path(&mut self, path: DerivationPath) {
        self.path = path
    }
    pub fn seed<'a>(&mut self, seed: &'a str) {
        let s = seed.as_bytes();
        self.seed.unwrap().copy_from_slice(&s);
    }

    pub fn index(mut self, index: u32) {
        self.index = Some(index);
    }

    pub fn build(self) -> Result<Account, WalletError> {
        let path = &self.path;
        let xprv = XPrv::derive_from_path(&self.seed.unwrap(), &path)
            .expect("Unable to derive from given path");
        let xpub = xprv.public_key();
        let pk = xpub.public_key().to_owned();
        let pk_hash = PubkeyHash::hash(&pk.to_bytes());
        let address = match path.address_type() {
            AddressKind::Legacy => Ok(Address::p2pkh(pk_hash, self.network)),
            AddressKind::NativeSegWit => {
                let xprv = xprv.private_key();
                let d = &xprv.to_bytes();
                let c_pk = CompressedPublicKey::from_slice(&d)
                    .expect("Failed while attempting to create compressed pub key from slice.");

                Ok(Address::p2wpkh(&c_pk, self.network))
            }
            AddressKind::Unknown => Err(WalletError::InvalidInput(path.to_string())),
            _ => Err(WalletError::InvalidInput(path.to_string())),
        };

        if let Ok(add) = address {
            Ok(Account {
                address: add.to_string(),
                path: path.to_string(),
                index: 0,
            })
        } else {
            let err = address.err().unwrap();
            Err(err)
        }
    }
}
