use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;
use std::str::FromStr;

/// Initialize the database pool and run setup queries
pub async fn initialize_database(app_data_dir: PathBuf) -> Result<SqlitePool, String> {
    let db_path = app_data_dir.join("ebers.db");

    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))
        .map_err(|e| format!("Failed to create SQLite options: {}", e))?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Run initialization queries
    run_initialization_queries(&pool).await?;

    Ok(pool)
}

/// Run database initialization queries
/// Note: These are run in addition to migrations for backwards compatibility
async fn run_initialization_queries(pool: &SqlitePool) -> Result<(), String> {
    // Drop old detections table if it exists
    sqlx::query("DROP TABLE IF EXISTS detections;")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to drop old detections table: {}", e))?;

    // Create settings table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create settings table: {}", e))?;

    // Create patients table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS patients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            date_of_birth TEXT,
            patient_id_number TEXT,
            email TEXT,
            phone TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create patients table: {}", e))?;

    // Create tests table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT NOT NULL UNIQUE,
            patient_id INTEGER NOT NULL,
            test_type TEXT NOT NULL,
            device_id TEXT,
            firmware_version TEXT,
            detection_result TEXT,
            confidence REAL,
            raw_response TEXT,
            status TEXT NOT NULL,
            error_message TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            completed_at TEXT,
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        );",
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create tests table: {}", e))?;

    // Create indexes
    create_indexes(pool).await?;

    Ok(())
}

/// Create database indexes
async fn create_indexes(pool: &SqlitePool) -> Result<(), String> {
    // Patients indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_patients_uuid ON patients(uuid);")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create patients uuid index: {}", e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_patients_created_at ON patients(created_at);")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create patients created_at index: {}", e))?;

    // Tests indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tests_uuid ON tests(uuid);")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create tests uuid index: {}", e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tests_patient_id ON tests(patient_id);")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create tests patient_id index: {}", e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tests_status ON tests(status);")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create tests status index: {}", e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tests_created_at ON tests(created_at);")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create tests created_at index: {}", e))?;

    Ok(())
}

