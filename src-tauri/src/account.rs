use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    secp256k1, Address, CompressedPublicKey, Network, NetworkKind, PrivateKey, PubkeyHash,
};
use serde_json::{json, Value};
use sqlx::sqlite::SqliteRow;

use crate::{path_builder::PathBuilder, storage::DbFacadePool, WalletError};
use sqlx::Row;

pub enum AccountType {
    Receiving,
    Spending,
}

pub enum AddressKind {
    Legacy,       // P2PKH
    SegWit,       // P2SH-P2WPKH
    NativeSegWit, // P2WPKH
}

trait AddressMethod {
    fn address_type(&self) -> AddressKind {
        AddressKind::SegWit
    }
}

impl AddressMethod for DerivationPath {
    fn address_type(&self) -> AddressKind {
        let path = &self.to_string();
        let parts: Vec<&str> = path.split('/').collect();

        // Check if the first part is "m" (master key)
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
    pub id: Option<String>,
    pub path: String,
    pub wallet_id: String,
    pub index: u32,
    pub address: String,
}

impl Account {
    pub async fn save(&self, db: &DbFacadePool) {
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT into accounts (id, wallet_id, address, 'index', path) values (?,?,?,?,?)",
        )
        .bind(id)
        .bind(&self.wallet_id)
        .bind(&self.address)
        .bind(&self.index)
        .bind(&self.path)
        .execute(db)
        .await
        .unwrap();
    }

    pub fn from_entry(entry: &SqliteRow) -> Account {
        let path: String = entry.get("path");
        let address: String = entry.get("address");
        let index: u32 = entry.get("index");
        let id: String = entry.get("id");
        let wallet_id: String = entry.get("wallet_id");
        Account {
            address,
            id: Some(id),
            index: index,
            path,
            wallet_id,
        }
    }

    pub fn parse_as_json(&self) -> Value {
        json!({
            "address": self.address,
            "id": self.id.clone().unwrap(),
            "path": self.path,
            "wallet_id": self.wallet_id,
            "index": self.index
        })
    }
}

pub struct AccountBuilder {
    path: DerivationPath,
    wallet_id: String,
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
            wallet_id: String::new(),
            network: Network::Bitcoin,
            network_kind: NetworkKind::Main,
            address: None,
            path: PathBuilder::new().build(),
            seed: Vec::new(),
        }
    }

    pub fn wallet_id(&mut self, id: String) {
        self.wallet_id = id
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
            id: None,
            wallet_id: self.wallet_id,
            address: address.to_string(),
            path: path.to_string(),
            index: self.index.unwrap(),
        })
    }
}
