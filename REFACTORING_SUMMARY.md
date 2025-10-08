# Refactoring Summary

This document summarizes the major refactoring changes made to the codebase.

## Overview

Three major changes were implemented:
1. **Renamed everything from "prediction" to "detection"** across the entire codebase
2. **Refactored lib.rs** into separate, focused modules for better code organization
3. **Renamed backend modules** to have more descriptive names

---

## 1. Prediction → Detection Renaming

All references to "prediction" have been renamed to "detection" throughout the codebase to better reflect the application's purpose.

### Backend (Rust/Tauri)

#### Renamed Types and Functions:
- `PredictionRequest` → `DetectionRequest`
- `PredictionResponse` → `DetectionResponse`
- `PredictionErrorResponse` → `DetectionErrorResponse`
- `PredictionResult` → `DetectionResult`
- `PredictionApiClient` → `DetectionApiClient`
- `PredictionRecord` → `DetectionRecord`
- `PredictionLoading` → `DetectionLoading`
- `PredictionError` → `DetectionError`
- `PredictionData` → `DetectionData`

#### Renamed Methods:
- `predict()` → `detect()`
- `create_prediction_request()` → `create_detection_request()`
- `insert_prediction()` → `insert_detection()`
- `update_prediction()` → `update_detection()`
- `get_prediction_by_uuid()` → `get_detection_by_uuid()`
- `get_all_predictions()` → `get_all_detections()`
- `get_predictions_by_status()` → `get_detections_by_status()`

#### Database Changes:
- Table: `predictions` → `detections`
- Indexes:
  - `idx_predictions_uuid` → `idx_detections_uuid`
  - `idx_predictions_created_at` → `idx_detections_created_at`
  - `idx_predictions_status` → `idx_detections_status`

#### Event Names:
- `serial:prediction_loading` → `serial:detection_loading`
- `serial:prediction_result` → `serial:detection_result`
- `serial:prediction_error` → `serial:detection_error`

### Frontend (Leptos)

#### Renamed Components:
- `PredictionsPage` → `DetectionsPage`
- `PredictionCard` → `DetectionCard`
- `Page::Predictions` → `Page::Detections`

#### Renamed Files:
- `src/app/pages/predictions.rs` → `src/app/pages/detections.rs`

#### UI Text Updates:
- "AI PREDICTION" → "AI DETECTION"
- "Prediction Probability" → "Detection Probability"
- "View Prediction History" → "View Detection History"
- "Processing prediction..." → "Processing detection..."

### Configuration

- **.env**
  - `PREDICTION_API_ENDPOINT` → `DETECTION_API_ENDPOINT`
  - Default endpoint: `/api/predict` → `/api/detect`

---

## 2. lib.rs Refactoring

The monolithic `lib.rs` file has been split into three focused modules for better organization and maintainability.

### Module Structure

#### **src-tauri/src/serial_handler.rs** (formerly `api.rs`)
**Purpose**: Handles all serial communication and API orchestration

**Contents**:
- Serial configuration and state management
- Serial port connection handling
- Serial data processing and buffering
- Detection API calls and error handling
- Serial monitoring loop with auto-reconnection
- `start_serial` Tauri command

**Key Functions**:
- `load_serial_config()` - Load configuration from environment
- `try_open_serial_port()` - Attempt serial port connection
- `process_serial_data_chunk()` - Process incoming serial data
- `handle_detection_api_call()` - Make API calls for completed datasets
- `run_serial_monitor_loop()` - Main serial monitoring loop
- `start_serial()` - Tauri command to start serial monitoring

#### **src-tauri/src/commands.rs** (formerly `database.rs`)
**Purpose**: Handles all database-related Tauri commands

**Contents**:
- Database query commands exposed to frontend
- Test data insertion

**Tauri Commands**:
- `get_all_detections` - Fetch all detection records
- `get_detection_by_uuid` - Fetch specific detection by UUID
- `get_detections_by_status` - Fetch detections filtered by status
- `insert_test_detection` - Insert sample test data

#### **src-tauri/src/detection_client.rs** (formerly `api_client.rs`)
**Purpose**: HTTP client for making detection API requests

**Contents**:
- Request/Response types for detection API
- HTTP client with retry logic (3 attempts)
- Error handling and timeout management
- Request builder functions

**Key Types**:
- `DetectionRequest` - API request payload
- `DetectionResponse` - API response payload
- `DetectionApiClient` - HTTP client with retry logic

**Key Functions**:
- `detect()` - Make detection API call with retries
- `create_detection_request()` - Build request from CSV data

#### **src-tauri/src/models.rs** (formerly `db.rs`)
**Purpose**: Database models and operations

**Contents**:
- Database record types
- Database helper struct
- CRUD operations for detections

**Key Types**:
- `DetectionRecord` - Database record structure
- `Database` - Database operations helper

**Key Functions**:
- `insert_detection()` - Insert new detection record
- `update_detection()` - Update existing detection
- `get_detection_by_uuid()` - Fetch by UUID
- `get_all_detections()` - Fetch all records
- `get_detections_by_status()` - Filter by status

#### **src-tauri/src/lib.rs**
**Purpose**: Application setup and initialization (admin tasks)

**Contents**:
- Module declarations
- Environment variable loading (`try_load_dotenv()`)
- Application setup and configuration
- Database initialization
- Plugin registration
- Tauri command handler registration
- `run()` function - main entry point

**Key Responsibilities**:
- Load and configure plugins (serial, SQL)
- Set up database connection pool
- Run database migrations
- Register all Tauri commands
- Initialize the Tauri application

---

## 3. Backend Module Renaming

To make the purpose of each module clearer, the backend files have been renamed:

### File Renames

| Old Name | New Name | Purpose |
|----------|----------|---------|
| `api.rs` | `serial_handler.rs` | Serial communication and API orchestration |
| `api_client.rs` | `detection_client.rs` | HTTP client for detection API requests |
| `db.rs` | `models.rs` | Database models and operations |
| `database.rs` | `commands.rs` | Tauri command handlers for database |

### Benefits of New Names

1. **`serial_handler.rs`** - Clearly indicates this module handles serial communication
2. **`detection_client.rs`** - Explicitly states it's a client for the detection API
3. **`models.rs`** - Standard naming convention for database models
4. **`commands.rs`** - Clear that these are Tauri command handlers

### Updated Import Statements

All import statements have been updated throughout the codebase:

**In `lib.rs`:**
```rust
mod commands;
mod detection_client;
mod models;
mod serial_handler;

use tauri::Manager;
```

**In `serial_handler.rs`:**
```rust
use crate::detection_client::{create_detection_request, DetectionApiClient};
use crate::models::{Database, DbState, DetectionRecord};
```

**In `commands.rs`:**
```rust
use crate::models::{Database, DbState, DetectionRecord};
```

**Note:** `DbState` is a type alias defined in `models.rs`:
```rust
pub type DbState = Mutex<SqlitePool>;
```

---

## Benefits of All Refactoring

1. **Separation of Concerns**: Each module has a clear, single responsibility
2. **Improved Maintainability**: Easier to locate and modify specific functionality
3. **Better Code Organization**: Related code is grouped together
4. **Self-Documenting**: File names clearly indicate their purpose
5. **Easier Testing**: Modules can be tested independently
6. **Reduced Complexity**: Smaller, focused files are easier to understand
7. **Scalability**: New features can be added to appropriate modules without cluttering lib.rs
8. **Clearer Architecture**: The module structure reflects the application's architecture

---

## Final Module Structure

```
src-tauri/src/
├── lib.rs                  # App initialization and setup
├── main.rs                 # Entry point
├── serial_handler.rs       # Serial communication & API orchestration
├── detection_client.rs     # HTTP client for detection API
├── models.rs               # Database models and operations
└── commands.rs             # Tauri command handlers
```

---

## Migration Notes

### For Developers

1. **Import Updates**: If you have custom code importing these modules, update:
   - `crate::api` → `crate::serial_handler`
   - `crate::api_client` → `crate::detection_client`
   - `crate::db` → `crate::models`
   - `crate::database` → `crate::commands`

2. **Database Migration**: The database table has been renamed from `predictions` to `detections`. Existing databases will need to be migrated or recreated.

3. **API Endpoint**: Update your backend API to use `/api/detect` instead of `/api/predict`, or update the `.env` file to point to the correct endpoint.

4. **Environment Variables**: Update `.env` file to use `DETECTION_API_ENDPOINT` instead of `PREDICTION_API_ENDPOINT`.

---

## Testing Checklist

- [ ] Verify serial connection works
- [ ] Verify data collection and CSV aggregation
- [ ] Verify API calls to detection endpoint
- [ ] Verify database operations (insert, update, query)
- [ ] Verify frontend displays detection results correctly
- [ ] Verify detection history page loads and displays records
- [ ] Verify error handling and retry logic
- [ ] Verify event listeners receive correct events
- [ ] Verify navigation between pages works
- [ ] Test with actual hardware/serial device
- [ ] Verify all imports resolve correctly after renaming
- [ ] Run full build to ensure no compilation errors

