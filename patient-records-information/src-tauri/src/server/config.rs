use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub enable_cloud_sync: bool,
    pub cloud_sync_interval_seconds: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            workers: None, // Let Actix decide
            enable_cloud_sync: env::var("ENABLE_CLOUD_SYNC")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            cloud_sync_interval_seconds: env::var("CLOUD_SYNC_INTERVAL")
                .unwrap_or_else(|_| "300".to_string()) // 5 minutes
                .parse()
                .unwrap_or(300),
        }
    }
}

