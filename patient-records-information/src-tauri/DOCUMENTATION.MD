# Patient Records Information - Project Structure Documentation

## Overview
This is a Tauri-based desktop application with an optional Actix web server that manages patient records with dual database support (local + optional cloud).

## Core Entry Points

### `main.rs`
- **Purpose**: Application entry point
- **Responsibilities**: 
  - Parse command line arguments
  - Choose between Tauri desktop app or Actix web server
  - Initialize the appropriate runtime
- **Usage**: 
  - `cargo run` → Starts Tauri desktop app
  - `cargo run -- --server` → Starts Actix web server

### `lib.rs`
- **Purpose**: Library entry point and module declarations
- **Responsibilities**:
  - Declare all project modules
  - Define Tauri commands
  - Initialize Tauri application
- **Modules**: `database`, `handlers`, `migrations`, `models`, `server`

## Source Code Structure (`/src`)

### `/database/` - Database Layer
**Purpose**: Database connection management and configuration

#### `connection.rs`
- **Responsibilities**:
  - Create and manage database connections (local + cloud)
  - Handle connection pooling and timeouts
  - Graceful degradation when cloud database is unavailable
  - Run database migrations on startup
- **Key Functions**:
  - `create_connections()` - Establishes both local and optional cloud connections
  - `DbConnections` struct - Holds local (required) and cloud (optional) connections

#### `mod.rs`
- **Purpose**: Database module declarations and exports

### `/handlers/` - Business Logic Layer
**Purpose**: Core business logic for patient record operations

#### `patient_handlers.rs`
- **Responsibilities**:
  - CRUD operations for patient records
  - Data validation and transformation
  - Database interaction logic
- **Key Functions**:
  - `create_patient()` - Create new patient record
  - `get_patient()` - Retrieve patient by ID
  - `get_all_patients()` - Retrieve all patients
  - `update_patient()` - Update existing patient
  - `delete_patient()` - Delete patient record
- **Data Structures**:
  - `CreatePatientRequest` - Input validation for patient creation
  - `UpdatePatientRequest` - Input validation for patient updates

#### `mod.rs`
- **Purpose**: Handler module declarations and public exports

### `/migrations/` - Database Schema Management
**Purpose**: Database schema versioning and migration management

#### `cli.rs`
- **Responsibilities**: Command-line interface for running migrations

#### `m20240101_000001_create_patients_table.rs`
- **Purpose**: Initial database schema migration
- **Responsibilities**: Create the patients table with all required fields

#### `runner.rs`
- **Responsibilities**: Execute database migrations programmatically

#### `template.rs`
- **Purpose**: Template for creating new migrations

#### `mod.rs`
- **Purpose**: Migration module declarations and configuration

### `/models/` - Data Models
**Purpose**: Database entity definitions and data structures

#### `patient_tb.rs`
- **Responsibilities**:
  - Define patient table structure
  - Entity relationships and constraints
  - ActiveModel definitions for database operations

#### `accounts.rs`
- **Purpose**: User account management (if implemented)

#### `medical_bill_record.rs`
- **Purpose**: Medical billing record management (if implemented)

#### `medical_record_tb.rs`
- **Purpose**: Medical record management (if implemented)

#### `mod.rs`
- **Purpose**: Model module declarations

### `/server/` - Web Server Layer
**Purpose**: Actix web server implementation with dual database support

#### `config.rs`
- **Responsibilities**:
  - Server configuration management
  - Environment variable handling
  - Default configuration values
- **Key Structures**:
  - `ServerConfig` - Server host, port, worker count, cloud sync settings

#### `state.rs`
- **Responsibilities**:
  - Application state management
  - Database connection access
  - Cloud availability tracking
- **Key Structures**:
  - `AppState` - Shared application state with database connections

#### `handlers.rs`
- **Responsibilities**:
  - HTTP request handlers
  - API endpoint implementations
  - Response formatting
- **Key Endpoints**:
  - Health check and database status
  - Patient CRUD operations
  - Manual cloud synchronization

#### `middleware.rs`
- **Responsibilities**:
  - Custom middleware for logging and monitoring
  - Cloud availability checking
  - Request timing and performance monitoring

#### `mod.rs`
- **Purpose**: Server module declarations and exports

#### `server.rs`
- **Responsibilities**:
  - Server startup and configuration
  - Route registration
  - Background health check tasks
  - Cloud database monitoring

### `/bin/` - Executable Binaries
**Purpose**: Standalone executable programs

#### `migrate.rs`
- **Purpose**: Database migration CLI tool
- **Usage**: `cargo run --bin migrate`

#### `server.rs`
- **Purpose**: Standalone web server binary
- **Usage**: `cargo run --bin server`
- **Features**: Dedicated server with custom configuration

## Configuration Files

### `Cargo.toml`
- **Purpose**: Rust project configuration and dependencies
- **Key Dependencies**:
  - `tauri` - Desktop app framework
  - `actix-web` - Web server framework
  - `sea-orm` - Database ORM
  - `tokio` - Async runtime
  - `serde` - Serialization/deserialization

### `tauri.conf.json`
- **Purpose**: Tauri application configuration
- **Responsibilities**: Window settings, permissions, build configuration

## Database Files

### `patient_records.db`
- **Purpose**: SQLite database file (if using SQLite)
- **Note**: This appears to be a local database file

## Documentation Files

### `SERVER_SETUP.md`
- **Purpose**: Comprehensive server setup and usage guide
- **Contents**: API documentation, configuration, deployment instructions

### `MIGRATIONS.md`
- **Purpose**: Database migration documentation and procedures

## Build and Development

### `build.rs`
- **Purpose**: Build script for Tauri compilation
- **Responsibilities**: Tauri build configuration

### `/target/`
- **Purpose**: Compiled artifacts and build cache
- **Note**: Generated during compilation, not part of source code

### `/gen/`
- **Purpose**: Generated code and schemas
- **Contents**: Tauri-generated configuration and capability files

## Key Features

### Dual Database Architecture
- **Local Database**: Always required, primary data store
- **Cloud Database**: Optional, used for synchronization and backup
- **Graceful Degradation**: App continues working even if cloud is unavailable

### Multiple Run Modes
1. **Tauri Desktop App**: `cargo run`
2. **Web Server**: `cargo run -- --server`
3. **Standalone Server**: `cargo run --bin server`

### API Endpoints
- Health check and database status
- Full CRUD operations for patient records
- Manual cloud synchronization
- Real-time cloud availability monitoring

## Development Workflow

1. **Database Changes**: Create migrations in `/migrations/`
2. **Business Logic**: Implement in `/handlers/`
3. **API Endpoints**: Add to `/server/handlers.rs`
4. **Configuration**: Update environment variables or `config.rs`
5. **Testing**: Use provided examples and API endpoints


