use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_records_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub medical_id: i32,
    #[sea_orm(foreign_key = "patients_table" //I think name of the table?) also is this indexed by default for joins?]
    pub patient_id: i32,
    pub assessment: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
    pub prescription: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]