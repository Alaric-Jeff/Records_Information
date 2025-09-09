use sea_orm::{Database, DbErr};
use crate::migrations::runner;

pub async fn run_migration_cli() -> Result<(), DbErr> {
    let database_url = "sqlite://patient_records.db";
    let db = Database::connect(database_url).await?;
    
    println!("Running migrations...");
    runner::run_migrations(&db).await?;
    println!("Migrations completed successfully!");
    
    Ok(())
}

pub async fn rollback_migration_cli() -> Result<(), DbErr> {
    let database_url = "sqlite://patient_records.db";
    let db = Database::connect(database_url).await?;
    
    println!("Rolling back migrations...");
    runner::rollback_migrations(&db).await?;
    println!("Migrations rolled back successfully!");
    
    Ok(())
}

pub async fn reset_migration_cli() -> Result<(), DbErr> {
    let database_url = "sqlite://patient_records.db";
    let db = Database::connect(database_url).await?;
    
    println!("Resetting migrations...");
    runner::reset_migrations(&db).await?;
    println!("Migrations reset successfully!");
    
    Ok(())
}

pub async fn status_migration_cli() -> Result<(), DbErr> {
    let database_url = "sqlite://patient_records.db";
    let db = Database::connect(database_url).await?;
    
    println!("Checking migration status...");
    runner::status_migrations(&db).await?;
    
    Ok(())
}
