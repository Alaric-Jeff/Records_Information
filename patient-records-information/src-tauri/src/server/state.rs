use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::database::connection::DbConnections;

#[derive(Clone)]
pub struct AppState {
    pub db_connections: Arc<RwLock<DbConnections>>,
    pub cloud_available: Arc<RwLock<bool>>,
}

impl AppState {
    pub fn new(db_connections: DbConnections) -> Self {
        Self {
            db_connections: Arc::new(RwLock::new(db_connections)),
            cloud_available: Arc::new(RwLock::new(true)), // Will be updated based on connection status
        }
    }

    pub async fn get_local_db(&self) -> DatabaseConnection {
        let connections = self.db_connections.read().await;
        connections.local.clone()
    }

    pub async fn get_cloud_db(&self) -> Option<DatabaseConnection> {
        let connections = self.db_connections.read().await;
        let cloud_available = *self.cloud_available.read().await;
        
        if cloud_available {
            connections.cloud.clone()
        } else {
            None
        }
    }

    pub async fn set_cloud_availability(&self, available: bool) {
        let mut cloud_available = self.cloud_available.write().await;
        *cloud_available = available;
    }

    pub async fn is_cloud_available(&self) -> bool {
        *self.cloud_available.read().await
    }
}
