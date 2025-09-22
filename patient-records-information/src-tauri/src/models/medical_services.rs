use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set; // ✅ this is where Set comes from
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, PartialEq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "service_category")]
pub enum ServiceCategory {
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

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "medical_services_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ms_id: Uuid,
    pub ms_name: String,
    pub ms_category: ServiceCategory,
    pub ms_price: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(
        mut self,
        _db: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();

        if insert {
            // generate UUID automatically
            self.ms_id = Set(Uuid::new_v4()); // ✅ safer unless you enabled v7
            self.created_at = Set(now);
        }

        self.updated_at = Set(now);
        Ok(self)
    }
}
