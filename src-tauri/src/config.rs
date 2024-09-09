use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub database_key: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();
        Config {
            database_url: "".to_string(),
            database_key: "".to_string(),
        }
    }
}
