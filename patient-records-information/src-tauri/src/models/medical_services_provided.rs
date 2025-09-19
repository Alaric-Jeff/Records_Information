use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Represents a record of a medical service provided, linked to a bill and a service.
/// 
/// # Fixes Applied
/// - Added documentation for the struct and each field for clarity and maintainability.
/// - Ensured all field types and derives are correct for SeaORM and Serde compatibility.
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_services_provided_table")]
pub struct Model {
    /// Unique identifier for the medical service provided (Primary Key).
    #[sea_orm(primary_key)]
    pub mrs_id: Uuid,
    /// Foreign key referencing the medical bill record.
    #[sea_orm(foreign_key)]
    pub medical_bill_id: i32,
    /// Foreign key referencing the medical service.
    #[sea_orm(foreign_key)]
    pub ms_id: i32,
    /// Name of the service provided.
    pub service_name: String,
    /// Category of the service provided.
    pub service_category: String,
    /// Price of the service provided.
    pub price: f32,
}
/// Enum for defining entity relations for the medical services provided model (currently empty).
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relations can be defined here in the future.
}

/// Custom behavior for the medical services provided ActiveModel (currently no custom logic).
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

