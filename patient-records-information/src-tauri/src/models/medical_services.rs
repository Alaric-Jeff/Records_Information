use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "service_category")]
pub enum ServiceCategory {
    #[sea_orm(string_value = "Laboratory")]
    Laboratory,
    #[sea_orm(string_value = "Vaccine")]
    Vaccine,
}

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_services_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ms_id: i32,
    pub ms_name: String,
    pub ms_category: ServiceCategory,
    pub ms_price: f32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}