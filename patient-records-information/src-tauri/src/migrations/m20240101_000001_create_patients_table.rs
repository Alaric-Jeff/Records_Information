use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PatientsTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PatientsTable::PatientId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PatientsTable::FirstName).string().not_null())
                    .col(ColumnDef::new(PatientsTable::LastName).string().not_null())
                    .col(ColumnDef::new(PatientsTable::MiddleName).string().null())
                    .col(ColumnDef::new(PatientsTable::Age).integer().not_null())
                    .col(ColumnDef::new(PatientsTable::BirthDate).date().not_null())
                    .col(ColumnDef::new(PatientsTable::CsdIdOrPwdId).string().null())
                    .col(ColumnDef::new(PatientsTable::MobileNumber).string().null())
                    .col(ColumnDef::new(PatientsTable::ResidentialAddress).string().null())
                    .col(
                        ColumnDef::new(PatientsTable::IsArchived)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PatientsTable::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PatientsTable::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PatientsTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PatientsTable {
    Table,
    PatientId,
    FirstName,
    LastName,
    MiddleName,
    Age,
    BirthDate,
    CsdIdOrPwdId,
    MobileNumber,
    ResidentialAddress,
    IsArchived,
    CreatedAt,
    UpdatedAt,
}
