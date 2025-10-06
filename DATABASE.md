# Database Implementation

This project uses a local SQLite database via `tauri-plugin-sql` and `sqlx` to store prediction records.

## Database Schema

### Predictions Table

The `predictions` table stores all prediction records with the following schema:

```sql
CREATE TABLE predictions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    port TEXT NOT NULL,
    baud_rate INTEGER NOT NULL,
    collection_duration_ms INTEGER NOT NULL,
    prediction_result TEXT,
    confidence REAL,
    raw_response TEXT,
    status TEXT NOT NULL,
    error_message TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Indexes:**
- `idx_predictions_uuid` on `uuid`
- `idx_predictions_created_at` on `created_at`
- `idx_predictions_status` on `status`

## Database Location

The database file is stored in the application's data directory:
- **Windows**: `%APPDATA%\com.lukemalcom.ebers\ebers.db`
- **macOS**: `~/Library/Application Support/com.lukemalcom.ebers/ebers.db`
- **Linux**: `~/.local/share/com.lukemalcom.ebers/ebers.db`

## Backend (Rust) Usage

### Database Module (`src-tauri/src/db.rs`)

The database module provides helper functions for interacting with the database:

#### PredictionRecord Struct

```rust
pub struct PredictionRecord {
    pub id: Option<i64>,
    pub uuid: String,
    pub port: String,
    pub baud_rate: i32,
    pub collection_duration_ms: i64,
    pub prediction_result: Option<String>,
    pub confidence: Option<f64>,
    pub raw_response: Option<String>,
    pub status: String, // "pending", "success", "error"
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

#### Helper Methods

```rust
// Create a new pending prediction record
let record = PredictionRecord::new_pending(
    uuid,
    port,
    baud_rate,
    collection_duration_ms,
);

// Mark as successful
record.mark_success(prediction_result, confidence, raw_response);

// Mark as error
record.mark_error(error_message);
```

#### Database Operations

```rust
use db::Database;

// Insert a new prediction
let id = Database::insert_prediction(&pool, &record).await?;

// Update an existing prediction
Database::update_prediction(&pool, &record).await?;

// Get a prediction by UUID
let record = Database::get_prediction_by_uuid(&pool, &uuid).await?;

// Get all predictions
let records = Database::get_all_predictions(&pool).await?;

// Get predictions by status
let records = Database::get_predictions_by_status(&pool, "success").await?;
```

## Frontend (JavaScript/TypeScript) Usage

### Tauri Commands

The following Tauri commands are available from the frontend:

#### Get All Predictions

```javascript
import { invoke } from '@tauri-apps/api/core';

const predictions = await invoke('get_all_predictions');
// Returns: Array<PredictionRecord>
```

#### Get Prediction by UUID

```javascript
const prediction = await invoke('get_prediction_by_uuid', {
    uuid: 'some-uuid-here'
});
// Returns: PredictionRecord | null
```

#### Get Predictions by Status

```javascript
const successfulPredictions = await invoke('get_predictions_by_status', {
    status: 'success'
});
// Returns: Array<PredictionRecord>

// Valid status values: "pending", "success", "error"
```

### Example Usage in Frontend

```javascript
// Fetch all predictions and display them
async function loadPredictions() {
    try {
        const predictions = await invoke('get_all_predictions');
        console.log('All predictions:', predictions);
        
        // Filter by status
        const pending = predictions.filter(p => p.status === 'pending');
        const successful = predictions.filter(p => p.status === 'success');
        const failed = predictions.filter(p => p.status === 'error');
        
        console.log(`Pending: ${pending.length}, Success: ${successful.length}, Failed: ${failed.length}`);
    } catch (error) {
        console.error('Failed to load predictions:', error);
    }
}

// Get a specific prediction
async function getPrediction(uuid) {
    try {
        const prediction = await invoke('get_prediction_by_uuid', { uuid });
        if (prediction) {
            console.log('Found prediction:', prediction);
        } else {
            console.log('Prediction not found');
        }
    } catch (error) {
        console.error('Failed to get prediction:', error);
    }
}

// Get only successful predictions
async function getSuccessfulPredictions() {
    try {
        const predictions = await invoke('get_predictions_by_status', {
            status: 'success'
        });
        console.log('Successful predictions:', predictions);
    } catch (error) {
        console.error('Failed to get successful predictions:', error);
    }
}
```

## Database Initialization

The database is automatically initialized when the application starts:

1. The database file is created in the app data directory if it doesn't exist
2. The `predictions` table and indexes are created if they don't exist
3. The database pool is managed by Tauri's state management system

## Migration Strategy

The current implementation uses manual migrations in the `setup` function. If you need to add new tables or modify the schema:

1. Add new SQL statements in the `setup` function in `src-tauri/src/lib.rs`
2. Consider using version checks to avoid re-running migrations
3. For production, consider using `sqlx` migrations with the `sqlx-cli` tool

## Direct SQL Access (Frontend)

You can also use the `tauri-plugin-sql` directly from the frontend for custom queries:

```javascript
import Database from '@tauri-apps/plugin-sql';

// Load the database
const db = await Database.load('sqlite:ebers.db');

// Execute a custom query
const result = await db.select(
    'SELECT * FROM predictions WHERE status = $1 ORDER BY created_at DESC LIMIT 10',
    ['success']
);

console.log('Recent successful predictions:', result);
```

## Best Practices

1. **Always use parameterized queries** to prevent SQL injection
2. **Handle errors gracefully** - database operations can fail
3. **Use transactions** for multiple related operations (not yet implemented)
4. **Index frequently queried columns** - already done for uuid, created_at, and status
5. **Keep the database pool managed** - don't create multiple pools
6. **Use the helper functions** in `db.rs` for common operations

## Future Enhancements

- Add transaction support for atomic operations
- Implement database backup/export functionality
- Add data retention policies (e.g., auto-delete old records)
- Implement full-text search for prediction results
- Add database statistics and analytics queries

