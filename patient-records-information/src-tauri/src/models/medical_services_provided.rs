use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;


#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_services_provided_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub mrs_id: i32,
    #[sea_orm(foreign_key)]
    pub medical_bill_id: i32,
    #[sea_orm(foreign_key)]
    pub ms_id: i32,
    pub service_name: String,
    pub service_category: String
    pub price: f32,
}

// this is still to be revisioned.