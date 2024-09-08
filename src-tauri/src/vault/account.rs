#[derive(Default, Debug)]
pub struct AccountModel {
    pub id: String,
    pub wallet_id: String,
    pub address: String,
    pub index: usize,
    pub path: String,
    pub created_at: Option<String>,
}

impl From<StoreAccountInput> for AccountModel {
    fn from(value: StoreAccountInput) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            wallet_id: value.wallet_id,
            address: value.address,
            path: value.encrypted_path,
            index: value.index,
            created_at: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct StoreAccountInput {
    wallet_id: String,
    address: String,
    encrypted_path: String,
    index: usize,
}
