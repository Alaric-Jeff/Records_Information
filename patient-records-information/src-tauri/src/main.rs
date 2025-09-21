// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
mod tests;

fn main() {
    // Check if we should start the server instead of the Tauri app
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--server" {
        // Start the Actix web server
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            if let Err(e) = patient_records_information_lib::server::start_server(
                patient_records_information_lib::server::config::ServerConfig::default()
            ).await {
                eprintln!("Failed to start server: {}", e);
                std::process::exit(1);
            }
        });
    } else {
        // Start the Tauri desktop app
        patient_records_information_lib::run()
    }
}
