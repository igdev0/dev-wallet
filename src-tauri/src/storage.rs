use sqlx::SqliteConnection;

pub trait StorageMethods {
    fn load(&self, db: SqliteConnection) {
        // This function will provide the load functionality
    }
    fn save(&self, db: SqliteConnection) {
        // This function will provide the save functionality
    }
}
