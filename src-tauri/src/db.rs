pub use sqlx::{Connection, SqliteConnection};

struct Ab {
    db: SqliteConnection,
}

pub async fn create_db_connection() -> Result<SqliteConnection, sqlx::Error> {
    let conn = SqliteConnection::connect("sqlite://database.db").await?;
    Ok(conn)
}
