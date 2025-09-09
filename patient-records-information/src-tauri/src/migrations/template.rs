// Migration Template
// Copy this file and rename it with the format: m{YYYYMMDD}_{HHMMSS}_{description}.rs
// Example: m20240115_143000_add_email_to_patients.rs

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add your migration logic here
        // Example: Add a new column
        // manager
        //     .alter_table(
        //         Table::alter()
        //             .table(YourTable::Table)
        //             .add_column(ColumnDef::new(YourTable::NewColumn).string().null())
        //             .to_owned(),
        //     )
        //     .await
        
        // Example: Create a new table
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(YourTable::Table)
        //             .if_not_exists()
        //             .col(ColumnDef::new(YourTable::Id).integer().not_null().primary_key())
        //             .col(ColumnDef::new(YourTable::Name).string().not_null())
        //             .to_owned(),
        //     )
        //     .await
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add your rollback logic here
        // This should undo what the up() method does
        
        // Example: Drop a column
        // manager
        //     .alter_table(
        //         Table::alter()
        //             .table(YourTable::Table)
        //             .drop_column(YourTable::NewColumn)
        //             .to_owned(),
        //     )
        //     .await
        
        // Example: Drop a table
        // manager
        //     .drop_table(Table::drop().table(YourTable::Table).to_owned())
        //     .await
        
        Ok(())
    }
}

// Define your table columns here
// #[derive(DeriveIden)]
// enum YourTable {
//     Table,
//     Id,
//     Name,
//     NewColumn, // Add new columns here
// }
