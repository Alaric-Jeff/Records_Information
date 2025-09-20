use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Represents a record of a medical service provided, linked to a bill and a service.
/// 
/// 
/// 
/// 

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_enum")]
pub enum AvailableServices {
    #[sea_orm(string_value = "Hematology")]
    Hematology,
    #[sea_orm(string_value = "Bacteriology")]
    Bacteriology,
    #[sea_orm(string_value = "Clinical Microscopy")]
    ClinicalMicroscopy,
    #[sea_orm(string_value = "24 Hour Urine Test")]
    TwentyFourHourUrineTest,
    #[sea_orm(string_value = "Serology and Immunology")]
    SerologyAndImmunology,
    #[sea_orm(string_value = "Clinical Chemistry")]
    ClinicalChemistry,
    #[sea_orm(string_value = "Electrolytes")]
    Electrolytes,
    #[sea_orm(string_value = "Vaccine")]
    Vaccine,
    #[sea_orm(string_value = "Hispatology")]
    Hispatology,
    #[sea_orm(string_value = "To be read by pathologist")]
    ToBeReadByPathologist,
    #[sea_orm(string_value = "Tumor Markers")]
    TumorMarkers,
    #[sea_orm(string_value = "Thyroid Function test")]
    ThyroidFunctionTest,
    #[sea_orm(string_value = "Hormones")]
    Hormones,
    #[sea_orm(string_value = "Hepatitis")]
    Hepatitis,
    #[sea_orm(string_value = "Enzymes")]
    Enzymes,
    #[sea_orm(string_value = "Others")]
    Others,
}


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
    pub service_category: AvailableServices,
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

