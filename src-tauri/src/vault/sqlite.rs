use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::ops::Deref;

use super::{
    account::{AccountModel, StoreAccountInput},
    interface::{VaultInterface, VaultResult},
    wallet::{StoreWalletInput, WalletModel},
};

pub type DatabasePool = Pool<Sqlite>;

pub struct SqliteVault(DatabasePool);

impl Deref for SqliteVault {
    type Target = DatabasePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl VaultInterface for SqliteVault {
    async fn get_account_by_id(&self, id: &str) -> VaultResult<AccountModel> {
        Ok(AccountModel::default())
    }

    async fn get_all_accounts(&self, id: &str) -> VaultResult<Vec<AccountModel>> {
        Ok(vec![AccountModel::default()])
    }

    async fn get_wallet_by_id(&self, id: &str) -> VaultResult<WalletModel> {
        Ok(WalletModel::default())
    }

    async fn get_wallet_by_name(&self, name: &str) -> VaultResult<WalletModel> {
        Ok(WalletModel::default())
    }

    async fn remove_account_by_id(&self, id: &str) -> VaultResult<()> {
        Ok(())
    }

    async fn remove_wallet_by_id(&self, id: &str) -> VaultResult<()> {
        Ok(())
    }

    async fn insert_wallet(&self, input: StoreWalletInput) -> VaultResult<()> {
        Ok(())
    }

    async fn insert_account(&self, input: StoreAccountInput) -> VaultResult<()> {
        Ok(())
    }
}

impl SqliteVault {
    pub async fn new(url: Option<&str>) -> Self {
        let connection_url = url.unwrap_or("sqlite://database.db");
        let db_exists = Sqlite::database_exists(&connection_url)
            .await
            .expect("Database exist checking failed");
        if !db_exists {
            Sqlite::create_database(&connection_url)
                .await
                .expect("Creating database failure!");
            println!("The datbase does not exist, therefore it was just created")
        }

        let connection = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await
            .unwrap();
        Self(connection)
    }

    pub async fn migrate(&self) {
        sqlx::migrate!().run(&self.0).await.unwrap()
    }
}
