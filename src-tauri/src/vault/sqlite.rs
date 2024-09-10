use async_trait::async_trait;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqlitePoolOptions, SqliteRow},
    Pool, Row, Sqlite,
};
use std::ops::Deref;

use super::{
    account::{AccountModel, Blockchain, Network, StoreAccountInput},
    interface::{VaultError, VaultInterface, VaultResult},
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
        let res = sqlx::query("SELECT * FROM accounts WHERE id = ?;")
            .bind(id)
            .fetch_one(&self.0)
            .await;
        if let Err(_) = res {
            return Err(VaultError::NotFound);
        }

        let entry = res.expect("Failed to unwrap account by id query");

        SqliteVault::parse_account(&entry)
    }

    async fn get_all_accounts(&self, wallet_id: &str) -> VaultResult<Vec<AccountModel>> {
        let res = sqlx::query("SELECT * from accounts WHERE wallet_id = ?;")
            .bind(wallet_id)
            .fetch_all(&self.0)
            .await;

        if let Err(_) = res {
            return Err(VaultError::Listing);
        }

        let results = res.unwrap();

        let mut accounts = vec![];
        for acc in results.iter() {
            let account = SqliteVault::parse_account(acc);
            if let Err(err) = account {
                return Err(err);
            }
            accounts.push(account.unwrap());
        }

        Ok(accounts)
    }

    async fn get_wallet_by_id(&self, id: &str) -> VaultResult<WalletModel> {
        let res = sqlx::query("SELECT * FROM accounts WHERE id = ?;")
            .bind(id)
            .fetch_one(&self.0)
            .await;
        if let Err(_) = res {
            return Err(VaultError::NotFound);
        }

        let result = res.unwrap();
        SqliteVault::parse_wallet(&result)
    }

    async fn get_wallet_by_name(&self, name: &str) -> VaultResult<WalletModel> {
        let res = sqlx::query("SELECT * FROM wallets WHERE name = ?;")
            .bind(name)
            .fetch_one(&self.0)
            .await;
        if let Err(_) = res {
            return Err(VaultError::NotFound);
        }

        let result = res.unwrap();
        SqliteVault::parse_wallet(&result)
    }

    async fn remove_account_by_id(&self, id: &str) -> VaultResult<()> {
        let result = sqlx::query("DELETE FROM accounts WHERE id = ?;")
            .bind(id)
            .execute(&self.0)
            .await;

        if let Err(_) = result {
            return Err(VaultError::Removing);
        }

        Ok(())
    }

    async fn remove_wallet_by_id(&self, id: &str) -> VaultResult<()> {
        let result = sqlx::query(
            "
            BEGIN TRANSACTION;
    -- Delete associated accounts first
    DELETE FROM accounts
    WHERE wallet_id = ?;

    -- Delete the wallet after associated accounts are deleted
    DELETE FROM wallets
    WHERE id = ?;

    COMMIT;
        ",
        )
        .bind(id)
        .execute(&self.0)
        .await;
        if let Err(_) = result {
            return Err(VaultError::Removing);
        }
        Ok(())
    }

    async fn insert_wallet(&self, input: StoreWalletInput) -> VaultResult<WalletModel> {
        let id = uuid::Uuid::new_v4().to_string();
        let result = sqlx::query("INSERT into wallets (id, name, seed, password) values (?,?,?,?)")
            .bind(&id)
            .bind(&input.name)
            .bind(&input.encrypted_seed)
            .bind(&input.encrypted_pass)
            .execute(&self.0)
            .await;

        if let Err(err) = result {
            dbg!(err);
            return Err(VaultError::Inserting);
        }

        Ok(WalletModel {
            id,
            name: input.name,
            password: input.encrypted_pass,
            seed: input.encrypted_seed,
        })
    }

    async fn insert_account(&self, input: StoreAccountInput) -> VaultResult<AccountModel> {
        let AccountModel {
            id,
            address,
            blockchain,
            wallet_id,
            network,
            created_at: _,
            path,
        } = AccountModel::from(input.clone());

        let res = sqlx::query("INSERT into accounts (id, wallet_id, address, path, blockchain, network) values (?,?,?,?,?,?)")
            .bind(id)
            .bind(wallet_id)
            .bind(address)
            .bind(path)
            .bind(blockchain)
            .bind(network)
            .execute(&self.0)
            .await;

        if let Err(_) = res {
            return Err(VaultError::Inserting);
        }

        Ok(AccountModel::from(input))
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

    pub fn parse_wallet(entry: &SqliteRow) -> VaultResult<WalletModel> {
        let id: String = entry.get("id");
        let name: String = entry.get("name");

        let seed: String = entry.get("seed");
        let password: String = entry.get("password");

        Ok(WalletModel {
            id,
            name,
            seed,
            password,
        })
    }

    pub fn parse_account(entry: &SqliteRow) -> VaultResult<AccountModel> {
        let id: String = entry.get("id");
        let path: String = entry.get("path");
        let address: String = entry.get("address");
        let blockchain: String = entry.get("blockchain");
        let network: String = entry.get("network");
        let wallet_id: String = entry.get("wallet_id");
        let created_at: String = entry.get("created_at");

        let blockchain = Blockchain::from_string(&blockchain);

        if let Err(_) = blockchain {
            return Err(VaultError::Parser);
        }

        let network = Network::from_string(&network);
        if let Err(_) = network {
            return Err(VaultError::Parser);
        }

        Ok(AccountModel {
            id,
            address,
            blockchain: blockchain.unwrap().to_string(),
            network: network.unwrap().to_string(),
            wallet_id,
            path,
            created_at: Some(created_at),
        })
    }

    pub async fn migrate(&self) {
        sqlx::migrate!().run(&self.0).await.unwrap()
    }
}
