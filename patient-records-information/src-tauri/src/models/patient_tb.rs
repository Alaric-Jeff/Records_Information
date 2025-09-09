use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;


//patients model, but also a derived entity because of DeriveEntityModel macro
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "patients_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub patient_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub birth_date: Date,
    pub csd_id_or_pwd_id: Option<String>,
    pub mobile_number: Option<String>,
    pub residential_address: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    //I'll just define the definitions later on
}


#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert {
            self.updated_at = Set(chrono::Utc::now());
        }
        Ok(self)
    }
}