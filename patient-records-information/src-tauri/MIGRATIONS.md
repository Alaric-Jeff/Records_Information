# Database Migrations

This project uses SeaORM migrations to manage database schema changes.

## Overview

The migration system is set up in the `src/migrations/` directory and includes:

- `m20240101_000001_create_patients_table.rs` - Initial migration to create the patients table
- `mod.rs` - Migration module configuration
- `runner.rs` - Migration runner utilities
- `cli.rs` - CLI utilities for running migrations
- `template.rs` - Template for creating new migrations

## Running Migrations

### Using the CLI Tool

A CLI tool is available for managing migrations:

```bash
# Run all pending migrations
cargo run --bin migrate up

# Rollback the last migration
cargo run --bin migrate down

# Reset all migrations (drops all tables and re-runs migrations)
cargo run --bin migrate reset

# Check migration status
cargo run --bin migrate status
```

### Programmatically

Migrations are automatically run when the database connection is established in `src/database/connection.rs`.

## Creating New Migrations

1. Copy the template file:
   ```bash
   cp src/migrations/template.rs src/migrations/m{YYYYMMDD}_{HHMMSS}_{description}.rs
   ```

2. Update the migration name and implement the `up()` and `down()` methods

3. Add the new migration to `src/migrations/mod.rs`:
   ```rust
   mod m{YYYYMMDD}_{HHMMSS}_{description};
   
   // In the MigratorTrait implementation:
   Box::new(m{YYYYMMDD}_{HHMMSS}_{description}::Migration),
   ```

## Migration Best Practices

1. **Always implement both `up()` and `down()` methods** - The `down()` method should undo what `up()` does
2. **Use descriptive names** - Include the date and a clear description
3. **Test migrations** - Test both up and down migrations before deploying
4. **Backup data** - Always backup your database before running migrations in production
5. **Order matters** - Migrations run in the order they're defined in the `migrations()` function

## Current Schema

The patients table includes:
- `patient_id` (Primary Key, Auto-increment)
- `first_name` (String, Not Null)
- `last_name` (String, Not Null)
- `middle_name` (String, Nullable)
- `birth_date` (Date, Not Null)
- `csd_id_or_pwd_id` (String, Nullable)
- `mobile_number` (String, Nullable)
- `residential_address` (String, Nullable)
- `created_at` (Timestamp with Timezone, Not Null)
- `updated_at` (Timestamp with Timezone, Not Null)

## Database

The application uses SQLite with the database file located at `./patient_records.db`.
