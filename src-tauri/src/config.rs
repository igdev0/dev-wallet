use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();
        Config {
            database_url: "".to_string(),
        }
    }
}
