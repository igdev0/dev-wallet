use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    secp256k1, Address, CompressedPublicKey, Network, NetworkKind, PrivateKey, PubkeyHash,
};

use crate::{path_builder::PathBuilder, storage::DbFacadePool, WalletError};

pub enum AccountType {
    Receiving,
    Spending,
}

pub enum AddressKind {
    Legacy,       // P2PKH
    SegWit,       // P2SH-P2WPKH
    NativeSegWit, // P2WPKH
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

        // Check if the first part is "m" (master key)
        // if parts[0] != "m" {
        //     return AddressKind::Unknown;
        // }

        // Match based on the purpose field (second part)
        match parts[0] {
            "44'" => AddressKind::Legacy,
            "49'" => AddressKind::SegWit,
            "84'" => AddressKind::NativeSegWit,
            // "86'" => AddressKind::Taproot, // we don't support taproot just yet
            _ => {
                panic!("Unable to parse address")
            }
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
    network_kind: NetworkKind,
    seed: Vec<u8>,
    address: Option<String>,
}

impl AccountBuilder {
    pub fn new() -> Self {
        AccountBuilder {
            index: Some(0),
            network: Network::Bitcoin,
            network_kind: NetworkKind::Main,
            address: None,
            path: PathBuilder::new().build(),
            seed: Vec::new(),
        }
    }
    pub fn path(&mut self, path: DerivationPath) {
        self.path = path
    }
    pub fn seed<'a>(&mut self, seed: &'a str) {
        let s = seed.as_bytes();
        self.seed.extend_from_slice(s);
    }

    pub fn index(mut self, index: u32) {
        self.index = Some(index);
    }

    pub fn network_kind(mut self, network_kind: NetworkKind) {
        self.network_kind = network_kind;
    }

    pub fn build(self) -> Result<Account, WalletError> {
        let path = &self.path;
        let secp = secp256k1::Secp256k1::new();
        let xprv = Xpriv::new_master(self.network_kind, &self.seed)
            .expect("Unable to derive from given path")
            .derive_priv(&secp, path)
            .expect("failed deriving the private key from path");

        let pk = PrivateKey::new(xprv.private_key, self.network);
        let c_pk = CompressedPublicKey::from_private_key(&secp, &pk)
            .expect("Failed while attempting to create compressed pub key from slice.");

        let pkh = PubkeyHash::from(c_pk);

        let address = match path.address_type() {
            AddressKind::Legacy => Address::p2pkh(pkh, self.network),
            AddressKind::NativeSegWit => Address::p2wpkh(&c_pk, self.network),
            AddressKind::SegWit => Address::p2wpkh(&c_pk, self.network),
        };

        Ok(Account {
            address: address.to_string(),
            path: path.to_string(),
            index: 0,
        })
    }
}
