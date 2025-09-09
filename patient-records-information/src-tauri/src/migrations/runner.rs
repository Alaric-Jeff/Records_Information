use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;
use crate::migrations::Migrator;

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}

pub async fn rollback_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::down(db, None).await
}

pub async fn reset_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::reset(db).await
}

pub async fn status_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::status(db).await
}
