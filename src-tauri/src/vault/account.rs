#[derive(Default, Debug)]
pub struct AccountModel {
    pub id: String,
    pub address: &'static str,
    pub index: usize,
    pub path: &'static str,
    pub created_at: Option<&'static str>,
}

impl From<StoreAccountInput> for AccountModel {
    fn from(value: StoreAccountInput) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            address: value.address,
            path: value.encrypted_path,
            index: value.index,
            created_at: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct StoreAccountInput {
    pub wallet_id: &'static str,
    pub address: &'static str,
    pub encrypted_path: &'static str,
    pub index: usize,
}
