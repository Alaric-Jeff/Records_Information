use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_enum")]
pub enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Medtech")]
    Medtech,
}

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub account_id: i32,
    pub role: Role,
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {

}