use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MedicalRecordsTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MedicalRecordsTable::MedicalId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MedicalRecordsTable::PatientId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MedicalRecordsTable::Assessment).text().null())
                    .col(ColumnDef::new(MedicalRecordsTable::Diagnosis).text().null())
                    .col(ColumnDef::new(MedicalRecordsTable::Treatment).text().null())
                    .col(ColumnDef::new(MedicalRecordsTable::Prescription).text().null())
                    .col(ColumnDef::new(MedicalRecordsTable::FirstAuditedBy).string().not_null())
                    .col(ColumnDef::new(MedicalRecordsTable::LastAuditedBy).string().null())
                    .col(
                        ColumnDef::new(MedicalRecordsTable::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MedicalRecordsTable::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_medical_records_patient_id")
                            .from(MedicalRecordsTable::Table, MedicalRecordsTable::PatientId)
                            .to(PatientsTable::Table, PatientsTable::PatientId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MedicalRecordsTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MedicalRecordsTable {
    Table,
    MedicalId,
    PatientId,
    Assessment,
    Diagnosis,
    Treatment,
    Prescription,
    FirstAuditedBy,
    LastAuditedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PatientsTable {
    Table,
    PatientId,
}

