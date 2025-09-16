# Patient Records Information Server

This document explains how to set up and run the Actix web server that handles both local and optionally cloud database connections.

## Architecture

The server is designed with a **dual-database architecture**:

- **Local Database**: Always required, primary data store
- **Cloud Database**: Optional, used for synchronization and backup

### Key Features

- ✅ **Graceful Degradation**: App continues to work even if cloud database is unavailable
- ✅ **Automatic Health Checks**: Monitors cloud database connectivity
- ✅ **Data Synchronization**: Syncs data between local and cloud when both are available
- ✅ **RESTful API**: Full CRUD operations for patient records
- ✅ **Real-time Status**: Endpoints to check database connectivity status

## Configuration

### Environment Variables

```bash
# Server Configuration
SERVER_HOST=127.0.0.1          # Server bind address
SERVER_PORT=8080               # Server port
ENABLE_CLOUD_SYNC=true         # Enable cloud synchronization
CLOUD_SYNC_INTERVAL=300        # Health check interval in seconds

# Database Configuration
DATABASE_URL_LOCAL=postgres://postgres:password@localhost/patient_records
DATABASE_URL_CLOUD=postgres://postgres:password@cloudhost/patient_records
```

### Default Values

If environment variables are not set, the server uses these defaults:

- Host: `127.0.0.1`
- Port: `8080`
- Cloud Sync: `true`
- Health Check Interval: `300` seconds (5 minutes)

## Running the Server
 
### Option 1: Standalone Server Binary

```bash
# Build and run the dedicated server binary
cargo run --bin server
```

### Option 2: Main Binary with --server Flag

```bash
# Run the main binary with server mode
cargo run -- --server
```

### Option 3: Tauri App (Default)

```bash
# Run the Tauri desktop application
cargo run
```

## API Endpoints

### Health & Status

- `GET /api/v1/health` - Server health check
- `GET /api/v1/db-status` - Database connectivity status

### Patient Management

- `POST /api/v1/patients` - Create a new patient
- `GET /api/v1/patients` - Get all patients
- `GET /api/v1/patients/{id}` - Get patient by ID
- `PUT /api/v1/patients/{id}` - Update patient
- `DELETE /api/v1/patients/{id}` - Delete patient

### Synchronization

- `POST /api/v1/patients/sync` - Manual sync from local to cloud

## Database Behavior

### Local Database
- **Always Required**: Server will not start without local database
- **Primary Operations**: All CRUD operations use local database first
- **Migrations**: Automatically run on startup

### Cloud Database
- **Optional**: Server starts even if cloud database is unavailable
- **Automatic Sync**: When available, operations are synced to cloud
- **Health Monitoring**: Continuous health checks with automatic reconnection
- **Graceful Degradation**: Operations continue normally when cloud is down

## Example Usage

### Starting the Server

```bash
# Set environment variables
export DATABASE_URL_LOCAL="postgres://postgres:password@localhost/patient_records"
export DATABASE_URL_CLOUD="postgres://postgres:password@cloud.example.com/patient_records"
export SERVER_PORT=8080

# Start the server
cargo run --bin server
```

### Testing the API

```bash
# Health check
curl http://localhost:8080/api/v1/health

# Database status
curl http://localhost:8080/api/v1/db-status

# Create a patient
curl -X POST http://localhost:8080/api/v1/patients \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "birth_date": "1990-01-01",
    "mobile_number": "+1234567890"
  }'

# Get all patients
curl http://localhost:8080/api/v1/patients

# Manual sync to cloud
curl -X POST http://localhost:8080/api/v1/patients/sync
```

## Error Handling

The server handles various scenarios gracefully:

1. **Cloud Database Unavailable**: Operations continue with local database only
2. **Network Issues**: Automatic retry and reconnection attempts
3. **Database Errors**: Proper HTTP status codes and error messages
4. **Validation Errors**: Detailed error responses with field-specific messages

## Logging

The server provides comprehensive logging:

- ✅ Connection status (local and cloud)
- ⚠️ Cloud database warnings
- 🔄 Reconnection attempts
- 📊 Request timing and performance
- 🚀 Server startup information

## Development

### Adding New Endpoints

1. Add handler functions in `src/server/handlers.rs`
2. Register routes in `src/server.rs`
3. Update this documentation

### Database Schema Changes

1. Create migration files in `src/migrations/`
2. Update models in `src/models/`
3. Test with both local and cloud databases

## Production Deployment

### Docker Example

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin server

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/
EXPOSE 8080
CMD ["server"]
```

### Environment Configuration

```bash
# Production environment variables
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
DATABASE_URL_LOCAL=postgres://user:pass@db-host:5432/patient_records
DATABASE_URL_CLOUD=postgres://user:pass@cloud-host:5432/patient_records
RUST_LOG=info
```

## Troubleshooting

### Common Issues

1. **Server won't start**: Check local database connectivity
2. **Cloud sync not working**: Verify cloud database URL and network access
3. **High memory usage**: Adjust connection pool settings in `connection.rs`
4. **Slow responses**: Check database indexes and query performance

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin server
```
