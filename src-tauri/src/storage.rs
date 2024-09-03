use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

pub enum ConnectionType {
    InMemory,
    OnDisk,
}

pub type DbFacadePool = Pool<Sqlite>;
pub struct DbFacade {
    pub pool: DbFacadePool,
}

impl DbFacade {
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
        DbFacade { pool: connection }
    }

    pub async fn migrate(&self) {
        sqlx::migrate!().run(&self.pool).await.unwrap()
    }
}
