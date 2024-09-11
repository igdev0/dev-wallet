use core::fmt;

use crate::{
    path_builder::PathBuilder,
    utils::{decrypt, encrypt, AESKey},
};
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    hex::DisplayHex,
    secp256k1, Address, CompressedPublicKey, Network as BitcoinNetwork, NetworkKind, PrivateKey,
};
use hex::decode;
use thiserror::Error;

use super::wallet::WalletModel;

const BITCOIN: &str = "Bitcoin";
const TESTNET: &str = "Testnet";
const MAINNET: &str = "Mainnet";

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Failed building: {0}")]
    Building(String),
    #[error("Path invalid: {0}")]
    Path(String),
    #[error("Failed deriving key form path: {0}")]
    Derivation(String),
}

#[derive(Default, Debug)]
pub struct AccountModel {
    pub id: String,
    pub wallet_id: String,
    pub address: String,
    pub path: String,
    pub network: String,
    pub blockchain: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Default, Clone, Copy)]

pub enum Blockchain {
    #[default]
    Bitcoin,
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Blockchain::Bitcoin => BITCOIN,
        };
        write!(f, "{}", output)
    }
}

impl Blockchain {
    pub fn from_string(text: &str) -> Result<Self, &'static str> {
        match text {
            BITCOIN => Ok(Blockchain::Bitcoin),
            _ => Err("Error parsing"),
        }
    }

    pub fn to_bitcoin_network(&self) -> BitcoinNetwork {
        match self {
            Blockchain::Bitcoin => BitcoinNetwork::Bitcoin,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Network {
    #[default]
    Mainnet,
    Testnet,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Network::Mainnet => MAINNET,
            Network::Testnet => TESTNET,
        };
        write!(f, "{}", output)
    }
}

impl Network {
    pub fn from_string(text: &str) -> Result<Self, &'static str> {
        match text {
            MAINNET => Ok(Network::Mainnet),
            TESTNET => Ok(Network::Testnet),
            _ => Err("Error parsing"),
        }
    }

    pub fn to_bitcoin_network_kind(&self) -> NetworkKind {
        match self {
            Network::Mainnet => NetworkKind::Main,
            Network::Testnet => NetworkKind::Test,
        }
    }
}

impl From<StoreAccountInput> for AccountModel {
    fn from(value: StoreAccountInput) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            wallet_id: value.wallet_id,
            address: value.address,
            path: value.encrypted_path,
            created_at: None,
            blockchain: value.blockchain.to_string(),
            network: value.network.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct StoreAccountInput {
    pub wallet_id: String,
    pub address: String,
    pub encrypted_path: String,
    pub blockchain: Blockchain,
    pub network: Network,
}

type AccountInputBuilderResult = Result<StoreAccountInput, AccountError>;

#[derive(Default, Debug)]
pub struct AccountInputBuilder {
    pub path: DerivationPath,
    pub blockchain: Blockchain,
    pub network: Network,
    pub encrypted_seed: String,
    pub wallet_id: String,
}

impl From<DerivationPath> for AccountInputBuilder {
    fn from(value: DerivationPath) -> Self {
        Self {
            path: value,
            ..Default::default()
        }
    }
}

impl From<WalletModel> for AccountInputBuilder {
    fn from(value: WalletModel) -> Self {
        Self {
            path: PathBuilder::new().build(),
            wallet_id: value.id,
            encrypted_seed: value.seed,
            ..Default::default()
        }
    }
}

impl AccountInputBuilder {
    pub fn path(&mut self, path: DerivationPath) -> &mut Self {
        self.path = path;
        self
    }

    pub fn wallet_id(&mut self, id: &str) {
        self.wallet_id = id.to_string();
    }

    pub fn blockchain(&mut self, blockchain: Blockchain) {
        self.blockchain = blockchain;
    }

    pub fn network(&mut self, network: Network) {
        self.network = network;
    }

    pub fn encrypted_seed(&mut self, encrypted_seed: &str) {
        self.encrypted_seed = encrypted_seed.to_string();
    }

    pub fn build(&self, key: AESKey) -> AccountInputBuilderResult {
        let path = &self.path;
        let secp = secp256k1::Secp256k1::new();
        let seed = decode(&self.encrypted_seed).unwrap();
        let seed = decrypt(&key, &seed);
        if let Err(err) = seed {
            return Err(AccountError::Building(err.to_string()));
        }
        let bitcoin_network = self.blockchain.to_bitcoin_network();
        let xprv = Xpriv::new_master(bitcoin_network, &seed.unwrap());

        if let Err(err) = xprv {
            return Err(AccountError::Path(err.to_string()));
        }

        let xprv = xprv.unwrap().derive_priv(&secp, path);

        if let Err(err) = xprv {
            return Err(AccountError::Derivation(err.to_string()));
        }

        let xprv = xprv.unwrap();

        let pk = PrivateKey::new(xprv.private_key, self.network.to_bitcoin_network_kind());
        let c_pk = CompressedPublicKey::from_private_key(&secp, &pk)
            .expect("Failed while attempting to create compressed pub key from slice.");
        let address = match &self.blockchain {
            Blockchain::Bitcoin => Address::p2wpkh(&c_pk, bitcoin_network),
        };

        let encrypted_path = &self.path.to_string();
        let encrypted_path = encrypted_path.as_bytes();
        let encrypted_path = encrypt(&key, encrypted_path);
        if let Err(err) = encrypted_path {
            return Err(AccountError::Building(err.to_string()));
        }
        let encrypted_path = encrypted_path
            .unwrap()
            .to_hex_string(bitcoin::hex::Case::Lower);

        Ok(StoreAccountInput {
            address: address.to_string(),
            blockchain: self.blockchain,
            encrypted_path,
            network: Network::Mainnet,
            wallet_id: self.wallet_id.clone(),
        })
    }
}
