pub mod config;
pub mod handlers;
pub mod middleware;
pub mod state;

pub use config::ServerConfig;
pub use state::AppState;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::{DefaultHeaders, Compress};
use tokio::time::{interval, Duration};
use crate::database::connection::create_connections;
use crate::server::{handlers::*, middleware::setup_middleware};

/// Start the Actix web server with dual database support
pub async fn start_server(config: ServerConfig) -> std::io::Result<()> {
    // Initialize database connections
    let db_connections = create_connections().await
        .map_err(|e| {
            log::error!("Failed to create database connections: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

    // Create application state
    let app_state = AppState::new(db_connections);
    
    // Set initial cloud availability
    let cloud_available = app_state.db_connections.read().await.cloud.is_some();
    app_state.set_cloud_availability(cloud_available).await;

    // Start cloud health check task
    let health_check_state = app_state.clone();
    let health_check_interval = config.cloud_sync_interval_seconds;
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(health_check_interval));
        loop {
            interval.tick().await;
            check_cloud_health(&health_check_state).await;
        }
    });

    log::info!("🚀 Starting server on {}:{}", config.host, config.port);
    log::info!("📊 Cloud sync enabled: {}", config.enable_cloud_sync);
    log::info!("🔄 Cloud health check interval: {}s", health_check_interval);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(setup_middleware())
            .wrap(DefaultHeaders::new().add(("X-Version", "1.0")))
            .wrap(Compress::default())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/patients")
                            .route("", web::post().to(create_patient_handler))
                            .route("", web::get().to(get_all_patients_handler))
                            .route("/{id}", web::get().to(get_patient_handler))
                            .route("/{id}", web::put().to(update_patient_handler))
                            .route("/{id}", web::delete().to(delete_patient_handler))
                            .route("/sync", web::post().to(sync_to_cloud_handler))
                    )
                    .route("/health", web::get().to(health_check))
                    .route("/db-status", web::get().to(db_status))
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .workers(config.workers.unwrap_or_else(|| num_cpus::get()))
    .run()
    .await
}

/// Check cloud database health and update availability status
async fn check_cloud_health(app_state: &AppState) {
    if let Some(cloud_db) = app_state.get_cloud_db().await {
        // Health check using SeaORM connection ping
        match cloud_db.ping().await {
            Ok(()) => {
                if !app_state.is_cloud_available().await {
                    log::info!("🔄 Cloud database is back online");
                    app_state.set_cloud_availability(true).await;
                }
            }
            Err(e) => {
                if app_state.is_cloud_available().await {
                    log::warn!("⚠️ Cloud database health check failed: {}", e);
                    app_state.set_cloud_availability(false).await;
                }
            }
        }
    } else {
        // Try to reconnect to cloud database
        match create_connections().await {
            Ok(connections) => {
                if connections.cloud.is_some() {
                    log::info!("🔄 Reconnected to cloud database");
                    // Update the connections in app state
                    *app_state.db_connections.write().await = connections;
                    app_state.set_cloud_availability(true).await;
                }
            }
            Err(_) => {
                // Cloud still unavailable, keep status as is
            }
        }
    }
}

