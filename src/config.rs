use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}


impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let database = DatabaseConfig {
            url: env::var("DATABASE_URL")?,
        };
        
        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| String::from("8000"))
                .parse::<u16>()
                .expect("Invalid SERVER_PORT value"),
        };
        
        Ok(Config {
            database,
            server,
        })
    }
}