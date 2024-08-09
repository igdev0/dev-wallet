use sqlx::{Connection, SqliteConnection};

pub async fn create_db_connection() -> Result<SqliteConnection, sqlx::Error> {
    let conn = SqliteConnection::connect("sqlite::memory:").await?;

    Ok(conn)
}
