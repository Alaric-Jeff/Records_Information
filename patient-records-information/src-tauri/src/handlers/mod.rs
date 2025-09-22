pub mod patient_handlers;
pub use patient_handlers::{
    CreatePatientRequest,
    UpdatePatientRequest,
    create_patient,
    get_patient,
    get_all_patients,
    update_patient,
    delete_patient,
};

pub mod medical_services_handler;