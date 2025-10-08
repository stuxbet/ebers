use crate::detection_client::{create_detection_request, DetectionApiClient};
use crate::models::{Database, DbState, DetectionRecord};
use serde::Serialize;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

#[cfg(desktop)]
use tauri_plugin_serialplugin::{commands, desktop_api};

#[derive(Serialize, Clone)]
pub struct SerialStatus {
    pub connected: bool,
    pub port: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct DetectionLoading {
    pub loading: bool,
    pub dataset_id: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct DetectionError {
    pub error: String,
    pub dataset_id: Option<String>,
}

/// Configuration for serial connection
#[derive(Clone)]
struct SerialConfig {
    port: String,
    baud_rate: u32,
    api_endpoint: String,
}

/// State for managing serial data collection
struct SerialDataState {
    line_buffer: String,
    csv_buffer: String,
    last_data_at: Option<Instant>,
    data_start_at: Option<Instant>,
    idle_gap: Duration,
}

impl SerialDataState {
    fn new() -> Self {
        Self {
            line_buffer: String::new(),
            csv_buffer: String::new(),
            last_data_at: None,
            data_start_at: None,
            idle_gap: Duration::from_millis(2000),
        }
    }

    fn reset_timing(&mut self) {
        self.last_data_at = None;
        self.data_start_at = None;
    }

    fn clear_buffers(&mut self) {
        self.csv_buffer.clear();
        self.reset_timing();
    }

    fn is_idle(&self) -> bool {
        if let Some(t) = self.last_data_at {
            t.elapsed() >= self.idle_gap && !self.csv_buffer.is_empty()
        } else {
            false
        }
    }

    fn get_collection_duration_ms(&self) -> u64 {
        self.data_start_at
            .map(|start| start.elapsed().as_millis() as u64)
            .unwrap_or(0)
    }
}

/// Load configuration from environment variables
fn load_serial_config() -> SerialConfig {
    crate::try_load_dotenv();
    SerialConfig {
        port: std::env::var("SERIAL_PORT").unwrap_or_else(|_| "COM3".to_string()),
        baud_rate: std::env::var("SERIAL_BAUD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(115_200),
        api_endpoint: std::env::var("DETECTION_API_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:8000/api/detect".to_string()),
    }
}

/// Attempt to open serial port connection
fn try_open_serial_port(app: &AppHandle, port: &str, baud_rate: u32) -> Result<(), String> {
    commands::open(
        app.clone(),
        app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
        port.to_string(),
        baud_rate,
        None,
        None,
        None,
        None,
        Some(50),
    )
    .map_err(|e| e.to_string())
}

/// Emit connection status to frontend
fn emit_connection_status(app: &AppHandle, connected: bool, port: &str) {
    let _ = app.emit(
        "serial:status",
        &SerialStatus {
            connected,
            port: Some(port.to_string()),
        },
    );
}

/// Handle successful serial port connection
fn handle_connection_success(
    app: &AppHandle,
    port: &str,
    baud_rate: u32,
    data_state: &mut SerialDataState,
) {
    println!("[serial] opened {} @ {} baud", port, baud_rate);
    data_state.reset_timing();
    emit_connection_status(app, true, port);
}

/// Process incoming serial data chunk
fn process_serial_data_chunk(
    app: &AppHandle,
    chunk: &str,
    port: &str,
    data_state: &mut SerialDataState,
) {
    if chunk.is_empty() {
        return;
    }

    // Append to CSV dataset
    data_state.csv_buffer.push_str(chunk);
    let now = Instant::now();
    data_state.last_data_at = Some(now);
    if data_state.data_start_at.is_none() {
        data_state.data_start_at = Some(now);
    }

    // Process complete lines for frontend emission
    data_state.line_buffer.push_str(chunk);

    loop {
        if let Some(pos) = data_state.line_buffer.find('\n') {
            let mut line: String = data_state.line_buffer.drain(..=pos).collect();
            if line.ends_with('\n') {
                line.pop();
            }
            if line.ends_with('\r') {
                line.pop();
            }
            // Log and emit full line (prevents chunk boundary artifacts)
            println!("[serial {}] {}", port, line);
            let _ = app.emit("serial:data", &line);
        } else {
            break;
        }
    }
}

/// Handle detection API call for completed dataset
async fn handle_detection_api_call(
    app: AppHandle,
    csv_data: String,
    port: String,
    baud_rate: u32,
    collection_duration_ms: u64,
    api_client: DetectionApiClient,
) {
    use chrono::Utc;

    // Emit loading state
    let _ = app.emit(
        "serial:detection_loading",
        &DetectionLoading {
            loading: true,
            dataset_id: None,
        },
    );

    // Create detection request
    match create_detection_request(&csv_data, port.clone(), baud_rate, collection_duration_ms) {
        Ok(request) => {
            let dataset_id = request.dataset_id.clone();
            println!(
                "[api_client] Calling detection API for dataset {}",
                dataset_id
            );

            // Create a pending detection record
            let mut detection_record = DetectionRecord {
                id: None,
                uuid: dataset_id.clone(),
                port: port.clone(),
                baud_rate: baud_rate as i32,
                collection_duration_ms: collection_duration_ms as i64,
                detection_result: None,
                confidence: None,
                raw_response: None,
                status: "pending".to_string(),
                error_message: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            };

            // Get database pool from app state
            let db_state = app.state::<DbState>();
            let pool = db_state.lock().await;

            // Insert pending record
            match Database::insert_detection(&*pool, &detection_record).await {
                Ok(id) => {
                    println!("[database] Inserted pending detection with id: {}", id);
                    detection_record.id = Some(id);
                }
                Err(e) => {
                    println!("[database] Failed to insert pending detection: {}", e);
                }
            }

            // Call API with retry logic
            match api_client.detect(request).await {
                Ok(response) => {
                    println!(
                        "[api_client] Detection successful: {}",
                        response.probability
                    );

                    // Update detection record with success
                    detection_record.detection_result =
                        Some(format!("Probability: {}", response.probability));
                    detection_record.confidence = response.confidence;
                    detection_record.raw_response = serde_json::to_string(&response).ok();
                    detection_record.status = "success".to_string();
                    detection_record.updated_at = Utc::now().to_rfc3339();

                    // Update in database
                    match Database::update_detection(&*pool, &detection_record).await {
                        Ok(_) => println!("[database] Updated detection to success"),
                        Err(e) => println!("[database] Failed to update detection: {}", e),
                    }

                    let _ = app.emit("serial:detection_result", &response);
                }
                Err(err) => {
                    println!("[api_client] Detection failed: {}", err);

                    // Update detection record with error
                    detection_record.status = "error".to_string();
                    detection_record.error_message = Some(err.clone());
                    detection_record.updated_at = Utc::now().to_rfc3339();

                    // Update in database
                    match Database::update_detection(&*pool, &detection_record).await {
                        Ok(_) => println!("[database] Updated detection to error"),
                        Err(e) => println!("[database] Failed to update detection: {}", e),
                    }

                    let _ = app.emit(
                        "serial:detection_error",
                        &DetectionError {
                            error: err,
                            dataset_id: Some(dataset_id),
                        },
                    );
                }
            }
        }
        Err(err) => {
            println!("[api_client] Failed to create detection request: {}", err);
            let _ = app.emit(
                "serial:detection_error",
                &DetectionError {
                    error: format!("Failed to parse CSV data: {}", err),
                    dataset_id: None,
                },
            );
        }
    }
}

/// Process completed dataset (either from idle timeout or disconnect)
fn process_completed_dataset(
    app: &AppHandle,
    data_state: &mut SerialDataState,
    port: &str,
    baud_rate: u32,
    api_client: &DetectionApiClient,
    reason: &str,
) {
    if data_state.csv_buffer.is_empty() {
        return;
    }

    let collection_duration_ms = data_state.get_collection_duration_ms();
    println!(
        "[serial {}] csv dataset complete {} ({} bytes)",
        port,
        reason,
        data_state.csv_buffer.as_bytes().len()
    );

    // Spawn API call task
    let csv_data = data_state.csv_buffer.clone();
    let app_clone = app.clone();
    let port_clone = port.to_string();
    let api_client_clone = api_client.clone();

    tauri::async_runtime::spawn(async move {
        handle_detection_api_call(
            app_clone,
            csv_data,
            port_clone,
            baud_rate,
            collection_duration_ms,
            api_client_clone,
        )
        .await;
    });

    data_state.clear_buffers();
}

/// Check if serial port is still available
fn is_port_available(app: &AppHandle, port: &str) -> bool {
    if let Ok(ports) = commands::available_ports(
        app.clone(),
        app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
    ) {
        ports.contains_key(port)
    } else {
        false
    }
}

/// Handle serial port disconnection
fn handle_port_disconnection(
    app: &AppHandle,
    data_state: &mut SerialDataState,
    port: &str,
    baud_rate: u32,
    api_client: &DetectionApiClient,
) {
    // Process any remaining data as completed dataset
    process_completed_dataset(
        app,
        data_state,
        port,
        baud_rate,
        api_client,
        "on disconnect",
    );

    println!("[serial] device on {} disconnected", port);
    emit_connection_status(app, false, port);
}

/// Read data from serial port
fn read_serial_data(app: &AppHandle, port: &str) -> Result<String, String> {
    commands::read(
        app.clone(),
        app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
        port.to_string(),
        Some(50),
        Some(1024),
    )
    .map_err(|e| e.to_string())
}

/// Main serial monitoring loop
async fn run_serial_monitor_loop(
    app: AppHandle,
    config: SerialConfig,
    api_client: DetectionApiClient,
) {
    let mut data_state = SerialDataState::new();
    let mut is_open = false;

    loop {
        if !is_open {
            // Try to open the target port; keep retrying until connected
            match try_open_serial_port(&app, &config.port, config.baud_rate) {
                Ok(_) => {
                    handle_connection_success(
                        &app,
                        &config.port,
                        config.baud_rate,
                        &mut data_state,
                    );
                    is_open = true;
                }
                Err(e) => {
                    // Not connected yet; wait and retry
                    println!("[serial] waiting for {}: {}", config.port, e);
                    sleep(Duration::from_millis(500));
                    continue;
                }
            }
        }

        // When open, read with a short timeout and process data/idle flush
        match read_serial_data(&app, &config.port) {
            Ok(chunk) => {
                process_serial_data_chunk(&app, &chunk, &config.port, &mut data_state);
            }
            Err(_) => {
                // Timeout or no data; check for idle gap end-of-dataset
                if data_state.is_idle() {
                    process_completed_dataset(
                        &app,
                        &mut data_state,
                        &config.port,
                        config.baud_rate,
                        &api_client,
                        "(idle timeout)",
                    );
                }

                // Also verify port is still present; if not, mark disconnected
                if !is_port_available(&app, &config.port) {
                    handle_port_disconnection(
                        &app,
                        &mut data_state,
                        &config.port,
                        config.baud_rate,
                        &api_client,
                    );
                    is_open = false;
                    // Back off briefly before attempting to reconnect
                    sleep(Duration::from_millis(500));
                }
            }
        }

        // Yield to the async runtime to allow other tasks (like API calls) to run
        // This prevents the serial loop from starving other async tasks
        tokio::task::yield_now().await;
    }
}

#[cfg(desktop)]
#[tauri::command]
pub async fn start_serial(app: AppHandle) -> Result<(), String> {
    let config = load_serial_config();
    let api_client = DetectionApiClient::new(config.api_endpoint.clone());

    // Spawn a background task that will auto-connect, read, and handle hot-plug
    tauri::async_runtime::spawn(run_serial_monitor_loop(app, config, api_client));

    Ok(())
}
