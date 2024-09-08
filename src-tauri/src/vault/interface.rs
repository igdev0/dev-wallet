use async_trait::async_trait;
use thiserror::Error;

use super::{
    account::{AccountModel, StoreAccountInput},
    wallet::{StoreWalletInput, WalletModel},
};

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Entity not found")]
    NotFound,
    #[error("Failed listing")]
    Listing,
    #[error("Parsing error")]
    Parser
}

pub type VaultResult<T> = Result<T, VaultError>;

#[async_trait]
pub trait VaultInterface {
    async fn get_wallet_by_id(&self, id: &str) -> VaultResult<WalletModel>;

    async fn get_wallet_by_name(&self, name: &str) -> VaultResult<WalletModel>;

    async fn get_account_by_id(&self, id: &str) -> VaultResult<AccountModel>;

    async fn get_all_accounts(&self, id: &str) -> VaultResult<Vec<AccountModel>>;

    async fn remove_account_by_id(&self, id: &str) -> VaultResult<()>;

    async fn remove_wallet_by_id(&self, id: &str) -> VaultResult<()>;

    async fn insert_wallet(&self, input: StoreWalletInput) -> VaultResult<()>;

    async fn insert_account(&self, input: StoreAccountInput) -> VaultResult<()>;
}
