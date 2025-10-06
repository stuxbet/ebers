mod api_client;

use api_client::{create_prediction_request, PredictionApiClient, PredictionResponse};
use serde::Serialize;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

#[cfg(desktop)]
use tauri_plugin_serialplugin::{commands, desktop_api};
fn try_load_dotenv() {
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

#[derive(Serialize, Clone)]
struct SerialStatus {
    connected: bool,
    port: Option<String>,
}

#[derive(Serialize, Clone)]
struct PredictionLoading {
    loading: bool,
    dataset_id: Option<String>,
}

#[derive(Serialize, Clone)]
struct PredictionError {
    error: String,
    dataset_id: Option<String>,
}

#[cfg(desktop)]
#[tauri::command]
async fn start_serial(app: AppHandle) -> Result<(), String> {
    // Load .env if present and read config
    try_load_dotenv();
    let port = std::env::var("SERIAL_PORT").unwrap_or_else(|_| "COM3".to_string());
    let baud: u32 = std::env::var("SERIAL_BAUD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(115_200);
    let api_endpoint = std::env::var("PREDICTION_API_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8000/api/predict".to_string());

    // Create API client
    let api_client = PredictionApiClient::new(api_endpoint);

    // Spawn a background task that will auto-connect, read, and handle hot-plug
    tauri::async_runtime::spawn({
        let app = app.clone();
        let target_port = port.clone();
        let baud_rate = baud;
        async move {
            let mut line_buffer = String::new();

            let mut buffer = String::new();
            let mut last_data_at: Option<Instant> = None;
            let mut data_start_at: Option<Instant> = None;
            let idle_gap = Duration::from_millis(2000);
            let mut is_open = false;

            loop {
                if !is_open {
                    // Try to open the target port; keep retrying until connected
                    match commands::open(
                        app.clone(),
                        app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
                        target_port.clone(),
                        baud,
                        None,
                        None,
                        None,
                        None,
                        Some(50),
                    ) {
                        Ok(_) => {
                            println!("[serial] opened {} @ {} baud", target_port, baud);
                            is_open = true;
                            // reset dataset timing on (re)connect
                            last_data_at = None;
                            // notify frontend of connection status
                            let _ = app.emit(
                                "serial:status",
                                &SerialStatus {
                                    connected: true,
                                    port: Some(target_port.clone()),
                                },
                            );
                        }
                        Err(e) => {
                            // Not connected yet; wait and retry
                            println!("[serial] waiting for {}: {}", target_port, e);
                            sleep(Duration::from_millis(500));
                            continue;
                        }
                    }
                }

                // When open, read with a short timeout and process data/idle flush
                match commands::read(
                    app.clone(),
                    app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
                    target_port.clone(),
                    Some(50),
                    Some(1024),
                ) {
                    Ok(chunk) => {
                        if !chunk.is_empty() {
                            // append to csv dataset
                            buffer.push_str(&chunk);
                            let now = Instant::now();
                            last_data_at = Some(now);
                            if data_start_at.is_none() {
                                data_start_at = Some(now);
                            }

                            // Accumulate and emit only complete lines to the frontend
                            line_buffer.push_str(&chunk);
                            loop {
                                if let Some(pos) = line_buffer.find('\n') {
                                    let mut line: String = line_buffer.drain(..=pos).collect();
                                    if line.ends_with('\n') {
                                        line.pop();
                                    }
                                    if line.ends_with('\r') {
                                        line.pop();
                                    }
                                    // Log and emit full line (prevents chunk boundary artifacts)
                                    println!("[serial {}] {}", target_port, line);
                                    let _ = app.emit("serial:data", &line);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // timeout or no data; check for idle gap end-of-dataset
                        if let Some(t) = last_data_at {
                            if t.elapsed() >= idle_gap && !buffer.is_empty() {
                                let collection_duration_ms = data_start_at
                                    .map(|start| start.elapsed().as_millis() as u64)
                                    .unwrap_or(0);

                                println!(
                                    "[serial {}] csv dataset complete ({} bytes)",
                                    target_port,
                                    buffer.as_bytes().len()
                                );

                                // Call prediction API
                                let csv_data = buffer.clone();
                                let app_clone = app.clone();
                                let port_clone = target_port.clone();
                                let api_client_clone = api_client.clone();

                                tauri::async_runtime::spawn(async move {
                                    // Emit loading state
                                    let _ = app_clone.emit(
                                        "serial:prediction_loading",
                                        &PredictionLoading {
                                            loading: true,
                                            dataset_id: None,
                                        },
                                    );

                                    // Create prediction request
                                    match create_prediction_request(
                                        &csv_data,
                                        port_clone.clone(),
                                        baud_rate,
                                        collection_duration_ms,
                                    ) {
                                        Ok(request) => {
                                            let dataset_id = request.dataset_id.clone();
                                            println!("[api_client] Calling prediction API for dataset {}", dataset_id);

                                            // Call API with retry logic
                                            match api_client_clone.predict(request).await {
                                                Ok(response) => {
                                                    println!(
                                                        "[api_client] Prediction successful: {}",
                                                        response.probability
                                                    );
                                                    let _ = app_clone.emit(
                                                        "serial:prediction_result",
                                                        &response,
                                                    );
                                                }
                                                Err(err) => {
                                                    println!(
                                                        "[api_client] Prediction failed: {}",
                                                        err
                                                    );
                                                    let _ = app_clone.emit(
                                                        "serial:prediction_error",
                                                        &PredictionError {
                                                            error: err,
                                                            dataset_id: Some(dataset_id),
                                                        },
                                                    );
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("[api_client] Failed to create prediction request: {}", err);
                                            let _ = app_clone.emit(
                                                "serial:prediction_error",
                                                &PredictionError {
                                                    error: format!(
                                                        "Failed to parse CSV data: {}",
                                                        err
                                                    ),
                                                    dataset_id: None,
                                                },
                                            );
                                        }
                                    }
                                });

                                buffer.clear();
                                last_data_at = None;
                                data_start_at = None;
                            }
                        }

                        // also verify port is still present; if not, mark disconnected
                        if let Ok(ports) = commands::available_ports(
                            app.clone(),
                            app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
                        ) {
                            let present = ports.contains_key(&target_port);
                            if !present {
                                // Treat disconnect as end-of-dataset too
                                if !buffer.is_empty() {
                                    let collection_duration_ms = data_start_at
                                        .map(|start| start.elapsed().as_millis() as u64)
                                        .unwrap_or(0);

                                    println!(
                                        "[serial {}] csv dataset complete on disconnect ({} bytes)",
                                        target_port,
                                        buffer.as_bytes().len()
                                    );

                                    // Call prediction API
                                    let csv_data = buffer.clone();
                                    let app_clone = app.clone();
                                    let port_clone = target_port.clone();
                                    let api_client_clone = api_client.clone();

                                    tauri::async_runtime::spawn(async move {
                                        // Emit loading state
                                        let _ = app_clone.emit(
                                            "serial:prediction_loading",
                                            &PredictionLoading {
                                                loading: true,
                                                dataset_id: None,
                                            },
                                        );

                                        // Create prediction request
                                        match create_prediction_request(
                                            &csv_data,
                                            port_clone.clone(),
                                            baud_rate,
                                            collection_duration_ms,
                                        ) {
                                            Ok(request) => {
                                                let dataset_id = request.dataset_id.clone();
                                                println!("[api_client] Calling prediction API for dataset {}", dataset_id);

                                                // Call API with retry logic
                                                match api_client_clone.predict(request).await {
                                                    Ok(response) => {
                                                        println!("[api_client] Prediction successful: {}", response.probability);
                                                        let _ = app_clone.emit(
                                                            "serial:prediction_result",
                                                            &response,
                                                        );
                                                    }
                                                    Err(err) => {
                                                        println!(
                                                            "[api_client] Prediction failed: {}",
                                                            err
                                                        );
                                                        let _ = app_clone.emit(
                                                            "serial:prediction_error",
                                                            &PredictionError {
                                                                error: err,
                                                                dataset_id: Some(dataset_id),
                                                            },
                                                        );
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                println!("[api_client] Failed to create prediction request: {}", err);
                                                let _ = app_clone.emit(
                                                    "serial:prediction_error",
                                                    &PredictionError {
                                                        error: format!(
                                                            "Failed to parse CSV data: {}",
                                                            err
                                                        ),
                                                        dataset_id: None,
                                                    },
                                                );
                                            }
                                        }
                                    });

                                    buffer.clear();
                                    last_data_at = None;
                                    data_start_at = None;
                                }
                                println!("[serial] device on {} disconnected", target_port);
                                // notify frontend of disconnection status
                                let _ = app.emit(
                                    "serial:status",
                                    &SerialStatus {
                                        connected: false,
                                        port: Some(target_port.clone()),
                                    },
                                );
                                is_open = false;
                                // back off briefly before attempting to reconnect
                                sleep(Duration::from_millis(500));
                            }
                        }
                    }
                }
            }
        }
    });
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_serialplugin::init())
            .invoke_handler(tauri::generate_handler![start_serial]);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
