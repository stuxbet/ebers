mod commands;
mod detection_client;
mod models;
mod serial_handler;

use tauri::Manager;

pub fn try_load_dotenv() {
    // Load only from repo root .env (relative to src-tauri)
    if let Ok(content) = std::fs::read_to_string("../.env") {
        for raw in content.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let mut val = v.trim();
                if let Some(stripped) = val.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    val = stripped;
                } else if let Some(stripped) =
                    val.strip_prefix('\'').and_then(|s| s.strip_suffix('\''))
                {
                    val = stripped;
                }
                std::env::set_var(key, val);
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_serialplugin::init())
            .plugin(
                tauri_plugin_sql::Builder::default()
                    .add_migrations(
                        "sqlite:ebers.db",
                        vec![
                            // Migration 1: Create initial tables
                            tauri_plugin_sql::Migration {
                                version: 1,
                                description: "create_initial_tables",
                                sql: "CREATE TABLE IF NOT EXISTS detections (
                                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                                    uuid TEXT NOT NULL UNIQUE,
                                    port TEXT NOT NULL,
                                    baud_rate INTEGER NOT NULL,
                                    collection_duration_ms INTEGER NOT NULL,
                                    detection_result TEXT,
                                    confidence REAL,
                                    raw_response TEXT,
                                    status TEXT NOT NULL,
                                    error_message TEXT,
                                    created_at TEXT NOT NULL,
                                    updated_at TEXT NOT NULL
                                );
                                CREATE INDEX IF NOT EXISTS idx_detections_uuid ON detections(uuid);
                                CREATE INDEX IF NOT EXISTS idx_detections_created_at ON detections(created_at);
                                CREATE INDEX IF NOT EXISTS idx_detections_status ON detections(status);",
                                kind: tauri_plugin_sql::MigrationKind::Up,
                            },
                        ],
                    )
                    .build(),
            )
            .setup(|app| {
                // Initialize the database pool
                tauri::async_runtime::block_on(async {
                    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
                    use std::str::FromStr;

                    let db_path = app.path().app_data_dir()
                        .expect("Failed to get app data dir")
                        .join("ebers.db");

                    // Create parent directory if it doesn't exist
                    if let Some(parent) = db_path.parent() {
                        std::fs::create_dir_all(parent).expect("Failed to create app data directory");
                    }

                    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))
                        .expect("Failed to create SQLite options")
                        .create_if_missing(true);

                    let pool = SqlitePoolOptions::new()
                        .max_connections(5)
                        .connect_with(options)
                        .await
                        .expect("Failed to connect to database");

                    // Run migrations
                    sqlx::query(
                        "CREATE TABLE IF NOT EXISTS detections (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            uuid TEXT NOT NULL UNIQUE,
                            port TEXT NOT NULL,
                            baud_rate INTEGER NOT NULL,
                            collection_duration_ms INTEGER NOT NULL,
                            detection_result TEXT,
                            confidence REAL,
                            raw_response TEXT,
                            status TEXT NOT NULL,
                            error_message TEXT,
                            created_at TEXT NOT NULL,
                            updated_at TEXT NOT NULL
                        );",
                    )
                    .execute(&pool)
                    .await
                    .expect("Failed to create detections table");

                    sqlx::query("CREATE INDEX IF NOT EXISTS idx_detections_uuid ON detections(uuid);")
                        .execute(&pool)
                        .await
                        .expect("Failed to create uuid index");

                    sqlx::query("CREATE INDEX IF NOT EXISTS idx_detections_created_at ON detections(created_at);")
                        .execute(&pool)
                        .await
                        .expect("Failed to create created_at index");

                    sqlx::query("CREATE INDEX IF NOT EXISTS idx_detections_status ON detections(status);")
                        .execute(&pool)
                        .await
                        .expect("Failed to create status index");

                    app.manage(tokio::sync::Mutex::new(pool));
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                serial_handler::start_serial,
                commands::get_all_detections,
                commands::get_detection_by_uuid,
                commands::get_detections_by_status,
                commands::insert_test_detection
            ]);
    }

    #[cfg(not(desktop))]
    {
        builder = builder.invoke_handler(tauri::generate_handler![
            commands::get_all_detections,
            commands::get_detection_by_uuid,
            commands::get_detections_by_status
        ]);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
