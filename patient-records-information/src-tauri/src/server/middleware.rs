use actix_web::{dev::ServiceRequest, Error, Result};
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::HttpMessage;
use crate::server::state::AppState;
use std::time::Instant;

/// Custom middleware to log request timing and database status
pub fn setup_middleware() -> Logger {
    Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
}

/// Middleware to check cloud database availability
pub async fn cloud_availability_checker(
    req: ServiceRequest,
    next: actix_web::middleware::Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let start = Instant::now();
    
    // Check if this is a cloud-dependent operation
    let path = req.path();
    let is_cloud_operation = path.contains("/sync") || path.contains("/cloud");
    
    if is_cloud_operation {
        if let Some(app_state) = req.app_data::<Data<AppState>>() {
            if !app_state.is_cloud_available().await {
                return Ok(ServiceResponse::new(
                    req.into_parts().0,
                    actix_web::HttpResponse::ServiceUnavailable()
                        .json(serde_json::json!({
                            "error": "Cloud database is not available",
                            "timestamp": chrono::Utc::now()
                        }))
                ));
            }
        }
    }
    
    let res = next.call(req).await?;
    let duration = start.elapsed();
    
    // Log slow requests
    if duration.as_millis() > 1000 {
        log::warn!("Slow request: {}ms for {}", duration.as_millis(), res.request().path());
    }
    
    Ok(res)
}

