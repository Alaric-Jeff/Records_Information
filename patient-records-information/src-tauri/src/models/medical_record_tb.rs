use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_records_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub medical_id: i32,
    #[sea_orm(
        foreign_key = "crate::patients::Entity",
        on_delete = "Cascade",
        on_update = "Cascade",
        indexed
    )]
    pub patient_id: i32,
    pub assessment: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
    pub prescription: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}
#[async_trait::async_trait]
impl ActiveModelBehavior for  ActiveModel{
    
}