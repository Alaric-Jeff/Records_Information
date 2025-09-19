use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::server::state::AppState;
use uuid::Uuid;

use crate::handlers::{
    CreatePatientRequest, UpdatePatientRequest,
    create_patient, get_patient, get_all_patients, update_patient, delete_patient
};

/// Health check endpoint for API monitoring
///
/// # Returns
/// - `HttpResponse::Ok()` with JSON payload containing:
///   - `status`: Always returns "healthy"
///   - `timestamp`: Current UTC timestamp in ISO 8601 format
///
/// # Example
/// ```
/// GET /health
/// Response: {"status": "healthy", "timestamp": "2023-09-15T12:34:56.789Z"}
/// ```
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}

/// Database connectivity status endpoint
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
///
/// # Returns
/// - `HttpResponse::Ok()` with JSON payload containing:
///   - `local_db`: Status of local database connection
///   - `cloud_db`: Status of cloud database connection
///
/// # Errors
/// - Always returns `Ok` as this endpoint doesn't perform actual database operations
///
/// # Example
/// ```
/// GET /db-status
/// Response: {
///   "local_db": {"available": true, "status": "connected"},
///   "cloud_db": {"available": false, "status": "disconnected"}
/// }
/// ```
pub async fn db_status(state: web::Data<AppState>) -> Result<HttpResponse> {
    let local_available = true;
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

/// Creates a new patient record in the local database and synchronizes to cloud if available
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
/// - `req`: JSON payload containing patient creation data wrapped in `web::Json`
///
/// # Returns
/// - `HttpResponse::Created()` with the created patient data if successful
/// - `HttpResponse::InternalServerError()` if database operation fails
///
/// # Errors
/// - Returns 500 Internal Server Error if local database operation fails
/// - Cloud synchronization failures are logged but don't affect the primary response
///
/// # Synchronization Behavior
/// - Primary creation happens in local database
/// - Async synchronization to cloud database if available
/// - Cloud errors are non-blocking for the client response
///
/// # Example
/// ```
/// POST /patients
/// Request Body: {"first_name": "John", "last_name": "Doe", ...}
/// Response: 201 Created with patient data
/// ```
pub async fn create_patient_handler(
    state: web::Data<AppState>,
    req: web::Json<CreatePatientRequest>,
) -> Result<HttpResponse> {
    let db = state.get_local_db().await;
    let create_req = req.into_inner();
    match create_patient(&db, create_req.clone()).await {
        Ok(patient) => {
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

/// Retrieves a specific patient by their UUID
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
/// - `path`: Path parameter containing the patient's UUID (`web::Path<Uuid>`)
///
/// # Returns
/// - `HttpResponse::Ok()` with patient data if found
/// - `HttpResponse::NotFound()` if patient doesn't exist
/// - `HttpResponse::InternalServerError()` if database operation fails
///
/// # Errors
/// - Returns 404 if patient with specified UUID doesn't exist
/// - Returns 500 if database query fails
///
/// # Example
/// ```
/// GET /patients/{uuid}
/// Response: 200 OK with patient data or 404 Not Found
/// ```
pub async fn get_patient_handler(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
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

/// Retrieves all patients from the local database
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
///
/// # Returns
/// - `HttpResponse::Ok()` with array of patient data
/// - `HttpResponse::InternalServerError()` if database operation fails
///
/// # Notes
/// - Only queries local database (does not attempt cloud synchronization)
/// - Returns empty array if no patients exist
///
/// # Example
/// ```
/// GET /patients
/// Response: 200 OK with array of patient objects
/// ```
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

/// Updates an existing patient's information
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
/// - `path`: Path parameter containing the patient's UUID (`web::Path<Uuid>`)
/// - `req`: JSON payload containing update data wrapped in `web::Json`
///
/// # Returns
/// - `HttpResponse::Ok()` with updated patient data if successful
/// - `HttpResponse::NotFound()` if patient doesn't exist
/// - `HttpResponse::InternalServerError()` if database operation fails
///
/// # Synchronization Behavior
/// - Primary update happens in local database
/// - Async synchronization to cloud database if available
/// - Cloud errors are non-blocking for the client response
///
/// # Example
/// ```
/// PATCH /patients/{uuid}
/// Request Body: {"first_name": "NewName", ...}
/// Response: 200 OK with updated data or 404 Not Found
/// ```
pub async fn update_patient_handler(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdatePatientRequest>,
) -> Result<HttpResponse> {
    let patient_id = path.into_inner();
    let db = state.get_local_db().await;
    let update_req = req.into_inner();
    match update_patient(&db, patient_id, update_req.clone()).await {
        Ok(Some(patient)) => {
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

/// Deletes a patient record by UUID
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
/// - `path`: Path parameter containing the patient's UUID (`web::Path<Uuid>`)
///
/// # Returns
/// - `HttpResponse::Ok()` with success message if deleted
/// - `HttpResponse::NotFound()` if patient doesn't exist
/// - `HttpResponse::InternalServerError()` if database operation fails
///
/// # Synchronization Behavior
/// - Primary deletion happens in local database
/// - Async synchronization to cloud database if available
/// - Cloud errors are non-blocking for the client response
///
/// # Example
/// ```
/// DELETE /patients/{uuid}
/// Response: 200 OK with success message or 404 Not Found
/// ```
pub async fn delete_patient_handler(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let patient_id = path.into_inner();
    let db = state.get_local_db().await;
    match delete_patient(&db, patient_id).await {
        Ok(true) => {
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

/// Manual synchronization endpoint to push all local data to cloud
///
/// # Parameters
/// - `state`: Web-wrapped application state containing database connections
///
/// # Returns
/// - `HttpResponse::Ok()` with synchronization results
/// - `HttpResponse::ServiceUnavailable()` if cloud database is not available
/// - `HttpResponse::InternalServerError()` if local database query fails
///
/// # Behavior
/// - Retrieves all patients from local database
/// - Attempts to create each patient in cloud database
/// - Returns summary of synchronization operations
///
/// # Example
/// ```
/// POST /sync-to-cloud
/// Response: 200 OK with {"message": "Sync completed", "synced_count": 5, ...}
/// ```
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