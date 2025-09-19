use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Enum representing the payment status for a medical bill record.
#[derive(Debug, Clone, PartialEq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_status_enum")]
pub enum PaymentStatus {
    /// The bill has been fully paid.
    #[sea_orm(string_value = "Paid")]
    Paid,
    /// The bill has not been paid.
    #[sea_orm(string_value = "Unpaid")]
    Unpaid,
    /// The bill has been partially paid.
    #[sea_orm(string_value = "Partially Paid")]
    PartiallyPaid,
}

/// Entity model for a medical bill record.
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_bill_records_table")]
pub struct Model {
    /// Unique identifier for the medical bill record.
    #[sea_orm(primary_key)]
    pub medical_bill_id: i32,
    /// Foreign key to the patient.
    #[sea_orm(foreign_key = "patients_table")]
    pub patient_id: i32,
    /// Foreign key to the medical record.
    #[sea_orm(foreign_key = "medical_records_table")]
    pub medical_id: i32,
    /// Foreign key to the medical services provided record.
    #[sea_orm(foreign_key = "medical_services_provided_table")]
    pub mrs_id: i32,
    /// Optional consultation fee.
    pub consultation_fee: Option<f32>,
    /// Optional remarks for the bill.
    pub remarks: Option<String>,
    /// Payment status for the bill.
    pub payment_status: PaymentStatus,
    /// Total amount for the bill.
    pub total_amount: f32,
}
/// Enum for defining entity relations (currently empty).
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}