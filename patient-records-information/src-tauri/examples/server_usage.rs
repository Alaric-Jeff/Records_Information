// Example of how to use the Patient Records Information Server
// This example demonstrates the dual database setup and API usage

use patient_records_information_lib::server::{start_server, config::ServerConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Example: Custom server configuration
    let config = ServerConfig {
        host: "0.0.0.0".to_string(),  // Listen on all interfaces
        port: 3000,                   // Custom port
        workers: Some(4),             // Custom worker count
        enable_cloud_sync: true,      // Enable cloud synchronization
        cloud_sync_interval_seconds: 60, // Check cloud health every minute
    };
    
    println!("🏥 Starting Patient Records Information Server");
    println!("📋 Server Configuration:");
    println!("   Host: {}", config.host);
    println!("   Port: {}", config.port);
    println!("   Workers: {:?}", config.workers);
    println!("   Cloud Sync: {}", config.enable_cloud_sync);
    println!("   Health Check Interval: {}s", config.cloud_sync_interval_seconds);
    
    // Example: Setting environment variables programmatically
    env::set_var("DATABASE_URL_LOCAL", "postgres://postgres:password@localhost/patient_records");
    env::set_var("DATABASE_URL_CLOUD", "postgres://postgres:password@cloud.example.com/patient_records");
    
    // Start the server
    start_server(config).await?;
    
    Ok(())
}

// Example API usage with reqwest (add to Cargo.toml dependencies):
/*
use reqwest;
use serde_json::json;

async fn example_api_calls() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080/api/v1";
    
    // Health check
    let health_response = client.get(&format!("{}/health", base_url))
        .send().await?;
    println!("Health check: {:?}", health_response.status());
    
    // Database status
    let db_status = client.get(&format!("{}/db-status", base_url))
        .send().await?
        .json::<serde_json::Value>().await?;
    println!("Database status: {}", db_status);
    
    // Create a patient
    let new_patient = json!({
        "first_name": "John",
        "last_name": "Doe",
        "middle_name": "Michael",
        "birth_date": "1990-01-15",
        "csd_id_or_pwd_id": "CSD123456",
        "mobile_number": "+1234567890",
        "residential_address": "123 Main St, City, State"
    });
    
    let create_response = client.post(&format!("{}/patients", base_url))
        .json(&new_patient)
        .send().await?;
    println!("Create patient: {:?}", create_response.status());
    
    // Get all patients
    let patients = client.get(&format!("{}/patients", base_url))
        .send().await?
        .json::<serde_json::Value>().await?;
    println!("All patients: {}", patients);
    
    // Manual sync to cloud
    let sync_response = client.post(&format!("{}/patients/sync", base_url))
        .send().await?;
    println!("Sync to cloud: {:?}", sync_response.status());
    
    Ok(())
}
*/

