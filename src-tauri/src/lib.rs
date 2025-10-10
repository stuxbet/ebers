mod commands;
mod db_operations;
mod detection_client;
mod migrations;
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
                    .add_migrations("sqlite:ebers.db", migrations::get_migrations())
                    .build(),
            )
            .setup(|app| {
                // Connect to the database that tauri-plugin-sql created and migrated
                tauri::async_runtime::block_on(async {
                    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
                    use std::str::FromStr;

                    let app_data_dir = app
                        .path()
                        .app_data_dir()
                        .expect("Failed to get app data dir");

                    let db_path = app_data_dir.join("ebers.db");

                    println!("📁 Database path: {}", db_path.display());

                    // Connect to the same database that tauri-plugin-sql manages
                    let options =
                        SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))
                            .expect("Failed to create SQLite options")
                            .create_if_missing(true);

                    let pool = SqlitePoolOptions::new()
                        .max_connections(5)
                        .connect_with(options)
                        .await
                        .expect("Failed to connect to database");

                    println!("✅ Connected to database pool for Rust commands");

                    app.manage(tokio::sync::Mutex::new(pool));
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                serial_handler::start_serial,
                serial_handler::list_serial_ports,
                serial_handler::get_current_port,
                serial_handler::set_serial_port,
                commands::save_setting,
                commands::get_setting,
                commands::create_patient,
                commands::get_patient_by_uuid,
                commands::get_all_patients,
                commands::create_test,
                commands::get_test_by_uuid,
                commands::get_all_tests,
                commands::update_test_status,
                commands::complete_test
            ]);
    }

    #[cfg(not(desktop))]
    {
        builder = builder.invoke_handler(tauri::generate_handler![
            commands::get_all_patients,
            commands::get_all_tests
        ]);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
