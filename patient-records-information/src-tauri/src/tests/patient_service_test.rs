


// use sea_orm::{Database, ConnectionTrait, DbBackend, Schema, EntityTrait, DatabaseConnection, DbErr};
// use chrono::NaiveDate;
// use uuid::Uuid;
// use std::fs;
// use std::path::Path;

// use patient_records_information_lib::models::patient_tb::Entity as PatientEntity;

// use patient_records_information_lib::handlers::{
//     CreatePatientRequest,
//     UpdatePatientRequest,
//     create_patient,
//     get_patient,
//     get_all_patients,
//     update_patient,
//     delete_patient,
// };

// async fn setup_test_db() -> Result<DatabaseConnection, DbErr> {
//     // Create a unique database file for each test run
//     let test_db_path = "test_patients.db";
    
//     // Remove the database file if it exists from previous tests
//     if Path::new(test_db_path).exists() {
//         fs::remove_file(test_db_path).expect("Failed to remove existing test database");
//     }
    
//     // Connect to the SQLite database
//     let db = Database::connect(&format!("sqlite:{}", test_db_path)).await?;
    
//     // Create the patient table
//     let backend = db.get_database_backend();
//     let schema = Schema::new(backend);
//     let stmt = schema.create_table_from_entity(PatientEntity);
    
//     db.execute(backend.build(&stmt)).await?;
    
//     Ok(db)
// }

// /// Create a sample patient request for testing
// fn create_sample_request() -> CreatePatientRequest {
//     CreatePatientRequest {
//         first_name: "John".to_string(),
//         last_name: "Doe".to_string(),
//         middle_name: Some("Michael".to_string()),
//         birth_date: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
//         csd_id_or_pwd_id: Some("CSD12345".to_string()),
//         mobile_number: Some("+1234567890".to_string()),
//         residential_address: Some("123 Main St".to_string()),
//     }
// }

// #[tokio::test]
// async fn test_create_patient_success() {
//     let db = setup_test_db().await.unwrap();
//     let request = create_sample_request();
    
//     let result = create_patient(&db, request).await;
//     assert!(result.is_ok());
    
//     let patient = result.unwrap();
//     assert_eq!(patient.first_name, "John");
//     assert_eq!(patient.last_name, "Doe");
//     assert_eq!(patient.middle_name, Some("Michael".to_string()));
// }

// #[tokio::test]
// async fn test_get_patient_exists() {
//     let db = setup_test_db().await.unwrap();
//     let request = create_sample_request();
    
//     let created_patient = create_patient(&db, request).await.unwrap();
//     let patient_id = created_patient.patient_id;
    
//     let result = get_patient(&db, patient_id).await;
//     assert!(result.is_ok());
    
//     let patient = result.unwrap();
//     assert!(patient.is_some());
//     assert_eq!(patient.unwrap().first_name, "John");
// }

// #[tokio::test]
// async fn test_get_patient_not_exists() {
//     let db = setup_test_db().await.unwrap();
//     let non_existent_id = Uuid;
    
//     let result = get_patient(&db, non_existent_id).await;
//     assert!(result.is_ok());
    
//     let patient = result.unwrap();
//     assert!(patient.is_none());
// }

// #[tokio::test]
// async fn test_get_all_patients_empty() {
//     let db = setup_test_db().await.unwrap();
    
//     let result = get_all_patients(&db).await;
//     assert!(result.is_ok());
    
//     let patients = result.unwrap();
//     assert!(patients.is_empty());
// }

// #[tokio::test]
// async fn test_get_all_patients_with_data() {
//     let db = setup_test_db().await.unwrap();
    
//     // Create multiple patients
//     let request1 = create_sample_request();
//     create_patient(&db, request1).await.unwrap();
    
//     let mut request2 = create_sample_request();
//     request2.first_name = "Jane".to_string();
//     create_patient(&db, request2).await.unwrap();
    
//     let result = get_all_patients(&db).await;
//     assert!(result.is_ok());
    
//     let patients = result.unwrap();
//     assert_eq!(patients.len(), 2);
//     assert!(patients.iter().any(|p| p.first_name == "John"));
//     assert!(patients.iter().any(|p| p.first_name == "Jane"));
// }

// #[tokio::test]
// async fn test_update_patient_exists() {
//     let db = setup_test_db().await.unwrap();
//     let request = create_sample_request();
    
//     let created_patient = create_patient(&db, request).await.unwrap();
//     let patient_id = created_patient.patient_id;
    
//     let update_request = UpdatePatientRequest {
//         first_name: Some("Jonathan".to_string()),
//         last_name: None,
//         middle_name: None,
//         birth_date: None,
//         csd_id_or_pwd_id: None,
//         mobile_number: None,
//         residential_address: None,
//     };
    
//     let result = update_patient(&db, patient_id, update_request).await;
//     assert!(result.is_ok());
    
//     let updated_patient = result.unwrap();
//     assert!(updated_patient.is_some());
//     assert_eq!(updated_patient.unwrap().first_name, "Jonathan");
// }

// #[tokio::test]
// async fn test_update_patient_not_exists() {
//     let db = setup_test_db().await.unwrap();
//     let non_existent_id = Uuid::new_v4();
    
//     let update_request = UpdatePatientRequest {
//         first_name: Some("Jonathan".to_string()),
//         ..Default::default()
//     };
    
//     let result = update_patient(&db, non_existent_id, update_request).await;
//     assert!(result.is_ok());
    
//     let updated_patient = result.unwrap();
//     assert!(updated_patient.is_none());
// }

// #[tokio::test]
// async fn test_delete_patient_exists() {
//     let db = setup_test_db().await.unwrap();
//     let request = create_sample_request();
    
//     let created_patient = create_patient(&db, request).await.unwrap();
//     let patient_id = created_patient.patient_id;
    
//     let result = delete_patient(&db, patient_id).await;
//     assert!(result.is_ok());
//     assert!(result.unwrap());
    
//     // Verify the patient is gone
//     let get_result = get_patient(&db, patient_id).await.unwrap();
//     assert!(get_result.is_none());
// }

// #[tokio::test]
// async fn test_delete_patient_not_exists() {
//     let db = setup_test_db().await.unwrap();
//     let non_existent_id = Uuid::new_v4();
    
//     let result = delete_patient(&db, non_existent_id).await;
//     assert!(result.is_ok());
//     assert!(!result.unwrap());
// }

// // Clean up after tests
// #[cfg(test)]
// #[ctor::ctor]
// fn cleanup() {
//     // This will run when the test process exits
//     let test_db_path = "test_patients.db";
//     if Path::new(test_db_path).exists() {
//         let _ = fs::remove_file(test_db_path);
//     }
// }