#[derive(Default, Debug)]
pub struct AccountModel {
    pub id: String,
    pub wallet_id: String,
    pub address: String,
    pub path: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Default)]
pub enum Blockchain {
    #[default]
    Bitcoin,
}

#[derive(Debug, Default)]
pub enum Network {
    #[default]
    Mainnet,
    Testnet,
}

impl From<StoreAccountInput> for AccountModel {
    fn from(value: StoreAccountInput) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            wallet_id: value.wallet_id,
            address: value.address,
            path: value.encrypted_path,
            created_at: None,
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
