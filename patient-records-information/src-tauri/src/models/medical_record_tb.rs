use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_records_table")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub medical_id: i32,
    #[sea_orm(
        foreign_key = "crate::patient_tb::Entity",
        on_delete = "Cascade",
        on_update = "Cascade",
        indexed
    )]
    pub patient_id: i32,
    pub assessment: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
    pub prescription: Option<String>,
    pub first_audited_by: String,
    pub last_audited_by: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::patient_tb::Entity",
        from = "Column::PatientId",
        to = "super::patient_tb::Column::PatientId"
    )]
    Patient,
}

impl Related<super::patient_tb::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Patient.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}