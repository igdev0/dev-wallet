use bitcoin_wallet::storage;
use sqlx::{Pool, Sqlite};

async fn create_db() -> Pool<Sqlite> {
    let mut connection = storage::DbFacade::new(Some("sqlite::memory:")).await;
    connection.migrate().await;
    connection.pool
}
