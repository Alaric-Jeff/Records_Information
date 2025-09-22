use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::models::medical_services::{
    Entity as ServiceEntity, Model as ServiceModel, ActiveModel as ServiceActiveModel
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::medical_services::ServiceCategory;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    pub ms_name: String,
    pub ms_category: ServiceCategory,
    pub ms_price: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServiceRequest {  // Fixed typo: CreateUpdateRequest → UpdateServiceRequest
    pub ms_id: Uuid,
    pub ms_name: Option<String>,
    pub ms_category: Option<ServiceCategory>,
    pub ms_price: Option<f32>,
}

pub async fn create_service(
    db: &DatabaseConnection,
    request: CreateServiceRequest,
) -> Result<ServiceModel, sea_orm::DbErr> {
    let now = Utc::now();

    let service = ServiceActiveModel {
        ms_id: Set(Uuid::new_v4()), 
        ms_name: Set(request.ms_name),
        ms_category: Set(request.ms_category),
        ms_price: Set(request.ms_price),
        created_at: Set(now),
        updated_at: Set(now),
    };

    service.insert(db).await
}

pub async fn update_service(
    db: &DatabaseConnection,
    req: UpdateServiceRequest,  // Fixed type name
) -> Result<Option<ServiceModel>, sea_orm::DbErr> {
    let service = ServiceEntity::find_by_id(req.ms_id).one(db).await?;

    if let Some(service) = service {
        let mut service: ServiceActiveModel = service.into();  // Added 'mut'
        
        // Fixed assignment syntax - you need to assign to the service fields
        if let Some(ms_name) = req.ms_name {
            service.ms_name = Set(ms_name);  // Fixed: service.ms_name = Set(value)
        }
        if let Some(ms_category) = req.ms_category {
            service.ms_category = Set(ms_category);  // Fixed assignment
        }
        if let Some(ms_price) = req.ms_price { 
            service.ms_price = Set(ms_price);  // Fixed assignment
        }
        
        // Update the timestamp
        service.updated_at = Set(Utc::now());
        
        let updated_service = service.update(db).await?;
        Ok(Some(updated_service))  
    } else {
        Ok(None)  
    }
}

pub async fn get_all_service(
    db: &DatabaseConnection
) -> Result<Vec<ServiceModel>, sea_orm::DbErr> {
    ServiceEntity::find().all(db).await
}