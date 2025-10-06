# Database Integration with API Predictions

## Overview
The application now automatically saves all prediction API results to the local SQLite database.

## How It Works

### 1. Prediction Lifecycle

When serial data is collected and sent to the prediction API, the following happens:

#### Step 1: Pending Record Created
- As soon as the API call is initiated, a **pending** prediction record is inserted into the database
- Contains: UUID (dataset_id), port, baud_rate, collection_duration_ms, status="pending"

#### Step 2: API Call Made
- The prediction request is sent to the API with retry logic (3 attempts)
- The pending record remains in the database during this time

#### Step 3: Result Saved
- **If successful**: Record is updated with:
  - `status` = "success"
  - `prediction_result` = "Probability: X.XX"
  - `confidence` = confidence score (if provided)
  - `raw_response` = full JSON response from API
  - `updated_at` = current timestamp

- **If failed**: Record is updated with:
  - `status` = "error"
  - `error_message` = error details
  - `updated_at` = current timestamp

### 2. Database Schema

Each prediction record contains:

```sql
CREATE TABLE predictions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,              -- Dataset ID from API
    port TEXT NOT NULL,                     -- Serial port (e.g., "COM3")
    baud_rate INTEGER NOT NULL,             -- Baud rate (e.g., 9600)
    collection_duration_ms INTEGER NOT NULL, -- How long data was collected
    prediction_result TEXT,                 -- Human-readable result
    confidence REAL,                        -- Confidence score (0.0-1.0)
    raw_response TEXT,                      -- Full JSON response
    status TEXT NOT NULL,                   -- "pending", "success", or "error"
    error_message TEXT,                     -- Error details if failed
    created_at TEXT NOT NULL,               -- When record was created
    updated_at TEXT NOT NULL                -- When record was last updated
);
```

### 3. Code Flow

**File**: `src-tauri/src/lib.rs`

**Function**: `handle_prediction_api_call()`

```rust
// 1. Create pending record
let prediction_record = PredictionRecord {
    uuid: dataset_id,
    port: port,
    baud_rate: baud_rate,
    status: "pending",
    // ...
};

// 2. Insert into database
Database::insert_prediction(&pool, &prediction_record).await;

// 3. Call API
match api_client.predict(request).await {
    Ok(response) => {
        // 4a. Update with success
        prediction_record.status = "success";
        prediction_record.prediction_result = Some(format!("Probability: {}", response.probability));
        prediction_record.confidence = response.confidence;
        Database::update_prediction(&pool, &prediction_record).await;
    }
    Err(err) => {
        // 4b. Update with error
        prediction_record.status = "error";
        prediction_record.error_message = Some(err);
        Database::update_prediction(&pool, &prediction_record).await;
    }
}
```

## Benefits

### 1. Complete Audit Trail
- Every prediction attempt is recorded
- Can see which predictions succeeded or failed
- Timestamps show when predictions were made

### 2. Historical Analysis
- View all past predictions
- Filter by status (success/pending/error)
- Analyze confidence scores over time

### 3. Debugging
- Failed predictions are saved with error messages
- Can identify patterns in failures
- Pending records show if API calls are stuck

### 4. Data Persistence
- Predictions survive app restarts
- Can export data for analysis
- No data loss

## Viewing Predictions

### In the UI
1. Navigate to home page
2. Click "📊 View Prediction History"
3. See all predictions with filtering options

### Direct Database Access
```bash
# Windows
cd %APPDATA%\com.lukemalcom.ebers
sqlite3 ebers.db "SELECT * FROM predictions ORDER BY created_at DESC LIMIT 10;"

# macOS/Linux
cd ~/Library/Application\ Support/com.lukemalcom.ebers
sqlite3 ebers.db "SELECT * FROM predictions ORDER BY created_at DESC LIMIT 10;"
```

## Example Records

### Successful Prediction
```json
{
  "id": 1,
  "uuid": "dataset_20250106_123456_abc123",
  "port": "COM3",
  "baud_rate": 9600,
  "collection_duration_ms": 5000,
  "prediction_result": "Probability: 0.95",
  "confidence": 0.95,
  "raw_response": "{\"success\":true,\"probability\":0.95,\"confidence\":0.95,...}",
  "status": "success",
  "error_message": null,
  "created_at": "2025-01-06T12:34:56Z",
  "updated_at": "2025-01-06T12:34:58Z"
}
```

### Failed Prediction
```json
{
  "id": 2,
  "uuid": "dataset_20250106_123500_def456",
  "port": "COM3",
  "baud_rate": 9600,
  "collection_duration_ms": 5000,
  "prediction_result": null,
  "confidence": null,
  "raw_response": null,
  "status": "error",
  "error_message": "Connection timeout after 3 retries",
  "created_at": "2025-01-06T12:35:00Z",
  "updated_at": "2025-01-06T12:35:15Z"
}
```

## Testing

### 1. Test with Real Serial Device
1. Connect your serial device
2. Let it collect data
3. Wait for prediction API call
4. Check predictions page - should see new record

### 2. Test with Manual Insert
1. Go to predictions page
2. Click "➕ Add Test" button
3. Should see test record appear immediately

### 3. Check Terminal Logs
When a prediction is made, you should see:
```
[api_client] Calling prediction API for dataset dataset_xxx
[database] Inserted pending prediction with id: 1
[api_client] Prediction successful: 0.95
[database] Updated prediction to success
```

## Troubleshooting

### No predictions appearing after API call
1. Check terminal for database errors
2. Verify database file exists in app data directory
3. Check that `db_state` is properly managed in Tauri

### Predictions stuck in "pending" status
- API call may have failed silently
- Check network connectivity
- Verify API endpoint is correct in .env file

### Database errors
- Check file permissions on app data directory
- Verify SQLite is properly initialized
- Check for disk space issues

## Future Enhancements

Potential improvements:
1. **Batch operations** - Insert multiple predictions at once
2. **Data retention** - Auto-delete old predictions after X days
3. **Export functionality** - Export predictions to CSV/JSON
4. **Statistics** - Calculate success rate, average confidence, etc.
5. **Notifications** - Alert on failed predictions
6. **Retry mechanism** - Automatically retry failed predictions

