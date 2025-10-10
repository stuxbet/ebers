use tauri_plugin_sql::{Migration, MigrationKind};

/// Get all database migrations
pub fn get_migrations() -> Vec<Migration> {
    vec![
        // Migration 1: Initial schema with settings table
        Migration {
            version: 1,
            description: "create_settings_table",
            sql: "CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                  );",
            kind: MigrationKind::Up,
        },
        // Migration 2: Patient and test schema
        Migration {
            version: 2,
            description: "create_patient_test_schema",
            sql: "
                -- Drop old detections table if it exists
                DROP TABLE IF EXISTS detections;
                
                -- Create patients table
                CREATE TABLE IF NOT EXISTS patients (
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
                );

                -- Create tests table
                CREATE TABLE IF NOT EXISTS tests (
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
                );

                -- Create indexes for patients
                CREATE INDEX IF NOT EXISTS idx_patients_uuid ON patients(uuid);
                CREATE INDEX IF NOT EXISTS idx_patients_created_at ON patients(created_at);

                -- Create indexes for tests
                CREATE INDEX IF NOT EXISTS idx_tests_uuid ON tests(uuid);
                CREATE INDEX IF NOT EXISTS idx_tests_patient_id ON tests(patient_id);
                CREATE INDEX IF NOT EXISTS idx_tests_status ON tests(status);
                CREATE INDEX IF NOT EXISTS idx_tests_created_at ON tests(created_at);
            ",
            kind: MigrationKind::Up,
        },
    ]
}

