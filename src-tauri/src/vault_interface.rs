use async_trait::async_trait;
use thiserror::Error;

use super::{
    account::{AccountModel, StoreAccountInput},
    wallet::{StoreWalletInput, WalletModel},
};

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Entity not found {0}")]
    NotFound(String),
    #[error("Failed listing: {0}")]
    Listing(String),
    #[error("Parsing error, details {0}")]
    Parser(String),
    #[error("Failed inserting: {0}")]
    Inserting(String),
    #[error("Failed removing: {0}")]
    Removing(String),
    #[error("Failed migrating, cause: {0}")]
    Migrating(String)
}

pub type VaultResult<T> = Result<T, VaultError>;

#[async_trait]
pub trait VaultInterface {
    async fn get_wallet_by_id(&self, id: &str) -> VaultResult<WalletModel>;

    async fn get_wallet_by_name(&self, name: &str) -> VaultResult<WalletModel>;

    async fn get_account_by_id(&self, id: &str) -> VaultResult<AccountModel>;

    async fn get_all_accounts(&self, id: &str) -> VaultResult<Vec<AccountModel>>;

    async fn get_all_wallets(&self) -> VaultResult<Vec<WalletModel>>;

    async fn remove_account_by_id(&self, id: &str) -> VaultResult<()>;

    async fn remove_wallet_by_id(&self, id: &str) -> VaultResult<()>;

    async fn insert_wallet(&self, input: StoreWalletInput) -> VaultResult<WalletModel>;

    async fn insert_account(&self, input: StoreAccountInput) -> VaultResult<AccountModel>;
}
