use std::env;
use anyhow::{Context, Result};

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .context("APP_PORT must be a number")?;

        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL is required in .env")?;

        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        Ok(Self {
            host,
            port,
            database_url,
            redis_url,
        })
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
