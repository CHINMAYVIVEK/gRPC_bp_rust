use std::fs;
use std::env;
use serde::Deserialize;

pub const CLIENT_ADDRESS: &str = "localhost";
pub const CLIENT_PORT: u16 = 50051;

pub const DB_USER: &str = "db_user";
pub const DB_PASSWORD: &str = "db_password";
pub const DB_HOST: &str = "localhost";
pub const DB_PORT: u16 = 5432;
pub const DB_NAME: &str = "my_database";

pub const PRODUCT_API_ENDPOINT: &str = "https://api.example.com/products";
pub const USER_API_ENDPOINT: &str = "https://api.example.com/users";



#[derive(Deserialize)]
pub struct Config {
    pub client: ClientConfig,
    pub database: DatabaseConfig,
    pub api: ApiConfig,
}

#[derive(Deserialize)]
pub struct ClientConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub product_service: String,
    pub user_service: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Determine which configuration file to load
    let env = env::var("APP_ENV").unwrap_or_else(|_| "staging".to_string());
    
    let config_file = if env == "production" {
        "config.yaml"
    } else {
        // Default to staging if not production
        "config_stage.yaml"
    };

    let config_str = fs::read_to_string(config_file)?;
    let config: Config = serde_yaml::from_str(&config_str)?;
    Ok(config)
}
