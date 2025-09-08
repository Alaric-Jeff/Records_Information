use sea_orm::{Database, DatabaseConnection, DbErr, Schema};
use crate::models::patient_tb::Entity as PatientEntity;

pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = "sqlite://./patient_records.db";
    let db = Database::connect(database_url).await?;
    
    // Create tables if they don't exist
    create_tables(&db).await?;
    
    Ok(db)
}

//nah I'll use migrations instead

// async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
//     let schema = Schema::new(db.get_database_backend());
    
//     // Create patients table
//     let stmt = schema.create_table_from_entity(PatientEntity);
//     db.execute(db.get_database_backend().build(&stmt)).await?;
    
//     Ok(())
// }