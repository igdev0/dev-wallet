use core::fmt;

const BITCOIN: &str = "Bitcoin";
const TESTNET: &str = "Testnet";
const MAINNET: &str = "Mainnet";

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

#[derive(Debug, Default)]

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
}

#[derive(Debug, Default)]
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

#[derive(Default, Debug)]
pub struct StoreAccountInput {
    wallet_id: String,
    address: String,
    encrypted_path: String,
    blockchain: Blockchain,
    network: Network,
}

#[derive(Default, Debug)]
pub struct AccountInputBuilder {
    path: String,
    blockchain: Blockchain,
    network: Network,
}

impl AccountInputBuilder {
    pub fn new() {}
}
