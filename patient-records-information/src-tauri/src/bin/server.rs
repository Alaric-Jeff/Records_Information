use patient_records_information_lib::server::{start_server, config::ServerConfig};
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //I'm guessing this is the actix server for cloud? if not check /server it might be a duplicate or rebundance.
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Load configuration
    let config = ServerConfig::default();
    
    log::info!("🏥 Patient Records Information Server");
    log::info!("📋 Configuration:");
    log::info!("   Host: {}", config.host);
    log::info!("   Port: {}", config.port);
    log::info!("   Cloud Sync: {}", config.enable_cloud_sync);
    log::info!("   Health Check Interval: {}s", config.cloud_sync_interval_seconds);
    

    log::info!("Server is starting");
    // Start the server
    start_server(config).await
}
