use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::ops::Deref;
use thiserror::Error;

pub enum ConnectionType {
    InMemory,
    OnDisk,
}

#[derive(Error, Debug)]
pub enum VaultError {}

type VaultResult<T> = Result<T, VaultError>;
pub type DatabasePool = Pool<Sqlite>;

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
pub struct StoreWalletInput {
    pub name: &'static str,
    pub encrypted_pass: &'static str,
    pub encrypted_seed: &'static str,
}

#[derive(Default, Debug)]
pub struct StoreAccountInput {
    pub wallet_id: &'static str,
    pub address: &'static str,
    pub encrypted_path: &'static str,
    pub index: usize,
}

#[async_trait]
pub trait VaultMethods {
    async fn get_wallet_by_id(&self, id: &str) -> VaultResult<WalletModel>;
    async fn get_wallet_by_name(&self, name: &str) -> VaultResult<WalletModel>;
    async fn get_account_by_id(&self, id: &str) -> VaultResult<AccountModel>;
    async fn remove_account_by_id(&self, id: &str) -> VaultResult<()>;
    async fn remove_wallet_by_id(&self, id: &str) -> VaultResult<()>;
    async fn insert_wallet(&self, input: StoreWalletInput) -> VaultResult<()>;
    async fn insert_account(&self, input: StoreAccountInput) -> VaultResult<()>;
}

pub struct SqliteVault(DatabasePool);

impl Deref for SqliteVault {
    type Target = DatabasePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl VaultMethods for SqliteVault {
    async fn get_account_by_id(&self, id: &str) -> VaultResult<AccountModel> {
        Ok(AccountModel::default())
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
