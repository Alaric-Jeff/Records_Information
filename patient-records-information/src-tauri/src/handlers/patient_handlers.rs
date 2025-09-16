use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::models::patient_tb::{Entity as PatientEntity, Model as PatientModel, ActiveModel as PatientActiveModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePatientRequest {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub birth_date: chrono::NaiveDate,
    pub csd_id_or_pwd_id: Option<String>,
    pub mobile_number: Option<String>,
    pub residential_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePatientRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub csd_id_or_pwd_id: Option<String>,
    pub mobile_number: Option<String>,
    pub residential_address: Option<String>,
}

pub async fn create_patient(
    db: &DatabaseConnection,
    request: CreatePatientRequest,
) -> Result<PatientModel, sea_orm::DbErr> {
    let patient = PatientActiveModel {
        first_name: Set(request.first_name),
        last_name: Set(request.last_name),
        middle_name: Set(request.middle_name),
        birth_date: Set(request.birth_date),
        csd_id_or_pwd_id: Set(request.csd_id_or_pwd_id),
        mobile_number: Set(request.mobile_number),
        residential_address: Set(request.residential_address),
        ..Default::default()
    };

    patient.insert(db).await
}

pub async fn get_patient(
    db: &DatabaseConnection,
    patient_id: i32,
) -> Result<Option<PatientModel>, sea_orm::DbErr> {
    PatientEntity::find_by_id(patient_id).one(db).await
}

pub async fn get_all_patients(
    db: &DatabaseConnection,
) -> Result<Vec<PatientModel>, sea_orm::DbErr> {
    PatientEntity::find().all(db).await
}

pub async fn update_patient(
    db: &DatabaseConnection,
    patient_id: i32,
    request: UpdatePatientRequest,
) -> Result<Option<PatientModel>, sea_orm::DbErr> {
    let patient = PatientEntity::find_by_id(patient_id).one(db).await?;
    
    if let Some(patient) = patient {
        let mut patient: PatientActiveModel = patient.into();
        
        if let Some(first_name) = request.first_name {
            patient.first_name = Set(first_name);
        }
        if let Some(last_name) = request.last_name {
            patient.last_name = Set(last_name);
        }
        if let Some(middle_name) = request.middle_name {
            patient.middle_name = Set(Some(middle_name));
        }
        if let Some(birth_date) = request.birth_date {
            patient.birth_date = Set(birth_date);
        }
        if let Some(csd_id_or_pwd_id) = request.csd_id_or_pwd_id {
            patient.csd_id_or_pwd_id = Set(Some(csd_id_or_pwd_id));
        }
        if let Some(mobile_number) = request.mobile_number {
            patient.mobile_number = Set(Some(mobile_number));
        }
        if let Some(residential_address) = request.residential_address {
            patient.residential_address = Set(Some(residential_address));
        }

        let updated_patient = patient.update(db).await?;
        Ok(Some(updated_patient))
    } else {
        Ok(None)
    }
} 

pub async fn delete_patient(
    db: &DatabaseConnection,
    patient_id: i32,
) -> Result<bool, sea_orm::DbErr> {
    let result = PatientEntity::delete_by_id(patient_id).exec(db).await?;
    Ok(result.rows_affected > 0)
}