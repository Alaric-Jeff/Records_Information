use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::server::state::AppState;
use crate::handlers::{
    CreatePatientRequest, UpdatePatientRequest,
    create_patient, get_patient, get_all_patients, update_patient, delete_patient
};

/// Health check endpoint
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}

/// Database status endpoint
pub async fn db_status(state: web::Data<AppState>) -> Result<HttpResponse> {
    let local_available = true; // Local is always available
    let cloud_available = state.is_cloud_available().await;
    
    Ok(HttpResponse::Ok().json(json!({
        "local_db": {
            "available": local_available,
            "status": if local_available { "connected" } else { "disconnected" }
        },
        "cloud_db": {
            "available": cloud_available,
            "status": if cloud_available { "connected" } else { "disconnected" }
        }
    })))
}

/// Create a new patient
pub async fn create_patient_handler(
    state: web::Data<AppState>,
    req: web::Json<CreatePatientRequest>,
) -> Result<HttpResponse> {
    let db = state.get_local_db().await;
    
    let create_req = req.into_inner();
    match create_patient(&db, create_req.clone()).await {
        Ok(patient) => {
            // If cloud is available, also sync to cloud
            if let Some(cloud_db) = state.get_cloud_db().await {
                let _ = create_patient(&cloud_db, create_req).await;
            }
            
            Ok(HttpResponse::Created().json(patient))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to create patient: {}", e)
        })))
    }
}

/// Get a patient by ID
pub async fn get_patient_handler(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let patient_id = path.into_inner();
    let db = state.get_local_db().await;
    
    match get_patient(&db, patient_id).await {
        Ok(Some(patient)) => Ok(HttpResponse::Ok().json(patient)),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Patient not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get patient: {}", e)
        })))
    }
}

/// Get all patients
pub async fn get_all_patients_handler(
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let db = state.get_local_db().await;
    
    match get_all_patients(&db).await {
        Ok(patients) => Ok(HttpResponse::Ok().json(patients)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get patients: {}", e)
        })))
    }
}

/// Update a patient
pub async fn update_patient_handler(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    req: web::Json<UpdatePatientRequest>,
) -> Result<HttpResponse> {
    let patient_id = path.into_inner();
    let db = state.get_local_db().await;
    
    let update_req = req.into_inner();
    match update_patient(&db, patient_id, update_req.clone()).await {
        Ok(Some(patient)) => {
            // If cloud is available, also sync to cloud
            if let Some(cloud_db) = state.get_cloud_db().await {
                let _ = update_patient(&cloud_db, patient_id, update_req).await;
            }
            
            Ok(HttpResponse::Ok().json(patient))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Patient not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to update patient: {}", e)
        })))
    }
}

/// Delete a patient
pub async fn delete_patient_handler(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let patient_id = path.into_inner();
    let db = state.get_local_db().await;
    
    match delete_patient(&db, patient_id).await {
        Ok(true) => {
            // If cloud is available, also delete from cloud
            if let Some(cloud_db) = state.get_cloud_db().await {
                let _ = delete_patient(&cloud_db, patient_id).await;
            }
            
            Ok(HttpResponse::Ok().json(json!({
                "message": "Patient deleted successfully"
            })))
        }
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Patient not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to delete patient: {}", e)
        })))
    }
}

/// Sync data from local to cloud (manual sync endpoint)
pub async fn sync_to_cloud_handler(
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    if !state.is_cloud_available().await {
        return Ok(HttpResponse::ServiceUnavailable().json(json!({
            "error": "Cloud database is not available"
        })));
    }

    let local_db = state.get_local_db().await;
    let cloud_db = state.get_cloud_db().await.unwrap();
    
    match get_all_patients(&local_db).await {
        Ok(patients) => {
            let mut synced_count = 0;
            let mut errors = Vec::new();
            
            for patient in &patients {
                // Try to sync each patient to cloud
                let create_request = CreatePatientRequest {
                    first_name: patient.first_name.clone(),
                    last_name: patient.last_name.clone(),
                    middle_name: patient.middle_name.clone(),
                    birth_date: patient.birth_date,
                    csd_id_or_pwd_id: patient.csd_id_or_pwd_id.clone(),
                    mobile_number: patient.mobile_number.clone(),
                    residential_address: patient.residential_address.clone(),
                };
                
                match create_patient(&cloud_db, create_request).await {
                    Ok(_) => synced_count += 1,
                    Err(e) => errors.push(format!("Failed to sync patient {}: {}", patient.patient_id, e)),
                }
            }
            
            Ok(HttpResponse::Ok().json(json!({
                "message": "Sync completed",
                "synced_count": synced_count,
                "total_patients": patients.len(),
                "errors": errors
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to get patients for sync: {}", e)
        })))
    }
}

