use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;


#[derive(Debug, Clone, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_status_enum")]
pub enum Payment_status {
    #[sea_orm(string_value = "Paid")]
    Paid,
    #[sea_orm(string_value = "Unpaid")]
    Unpaid,
    #[sea_orm(string_value = "Partially Paid")]
    Partially Paid
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_bill_records_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub medical_bill_id: i32,
    #[sea_orm(foreign_key = "patients_table")]
    pub patient_id: i32,
    #[sea_orm(foreign_key = "medical_records_table")]
    pub medical_id: i32,
    #[sea_orm(foreign_key = "medical_services_provided_table")]
    pub mrs_id: i32,
    pub consultation_fee: Option<f32>,
    pub remarks: Option<String>,
    pub payment_status: Payment_status,
    pub total_amount: f32

    // pub medication_fee: Option<f32>,
    // pub laboratory_fee: Option<f32>,
    // pub procedure_fee: Option<f32>,
    // pub injection_fee: Option<f32>,
    // pub medical_certificate_fee: Option<f32>,
    // pub others_fee: Option<f32>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {

}