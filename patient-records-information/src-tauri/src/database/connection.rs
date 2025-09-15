// use sea_orm::{Database, DatabaseConnection, DbErr};
// use crate::migrations::runner;

// pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
//     // Use a more reliable path for the database
//     let database_url = "sqlite://patient_records.db";
//     let db = Database::connect(database_url).await?;
    
//     // Run migrations to create/update tables
//     runner::run_migrations(&db).await?;
    
//     Ok(db)
// }

//this'll be changed into a local-first postgresql and online postgresql