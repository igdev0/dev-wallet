#[derive(Default, Debug)]
pub struct WalletModel {
    pub id: String,
    pub name: &'static str,
    pub password: &'static str,
    pub seed: &'static str,
}

impl From<StoreWalletInput> for WalletModel {
    fn from(value: StoreWalletInput) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            password: value.encrypted_pass,
            seed: value.encrypted_seed,
        }
    }
}

#[derive(Default, Debug)]
pub struct StoreWalletInput {
    pub name: &'static str,
    pub encrypted_pass: &'static str,
    pub encrypted_seed: &'static str,
}
