use sea_orm::{Database, DatabaseConnection, DbErr, ConnectOptions};
use std::env;
use std::time::Duration;
use crate::migrations::runner;

pub struct DbConnections {
    pub local: DatabaseConnection,
    pub cloud: Option<DatabaseConnection>,
}

pub async fn create_connections() -> Result<DbConnections, DbErr> {
    // Local database (always required)
    let local_url = env::var("DATABASE_URL_LOCAL")
        .unwrap_or_else(|_| "postgres://postgres:Aspiras123@localhost/leonardo_medical_services".to_string());

    let mut local_opt = ConnectOptions::new(local_url);
    local_opt.max_connections(20)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(true);

    let local = Database::connect(local_opt).await?;
    runner::run_migrations(&local).await?;

    // Cloud database (optional – app must still work if unreachable)
    let cloud_url = env::var("DATABASE_URL_CLOUD")
        .unwrap_or_else(|_| "postgres://postgres:password@cloudhost/patient_records".to_string());

    let mut cloud_opt = ConnectOptions::new(cloud_url);
    cloud_opt.max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(true);

    // Try cloud connection, but don't fail the whole app if it's down
    let cloud = match Database::connect(cloud_opt).await {
        Ok(conn) => {
            log::info!("✅ Successfully connected to cloud database");
            Some(conn)
        },
        Err(err) => {
            log::warn!("⚠️ Could not connect to cloud DB: {}. App will continue with local-only mode.", err);
            None
        }
    };

    Ok(DbConnections { local, cloud })
}
