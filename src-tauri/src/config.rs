use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub database_key: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();
        let database_url = dotenv!("DATABASE_URL");
        let database_key: &str = dotenv!("DATABASE_KEY");
        Config {
            database_url: database_url.to_string(),
            database_key: database_key.to_string(),
        }
    }
}
