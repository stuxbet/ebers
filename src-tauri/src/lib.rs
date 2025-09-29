use serde::Serialize;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};
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

#[tauri::command]
async fn start_serial(app: AppHandle) -> Result<(), String> {
    // Load .env if present and read config
    try_load_dotenv();
    let port = std::env::var("SERIAL_PORT").unwrap_or_else(|_| "COM3".to_string());
    let baud: u32 = std::env::var("SERIAL_BAUD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(115_200);

    // Spawn a background task that will auto-connect, read, and handle hot-plug
    tauri::async_runtime::spawn({
        let app = app.clone();
        let target_port = port.clone();
        async move {
            let mut line_buffer = String::new();

            let mut buffer = String::new();
            let mut last_data_at: Option<Instant> = None;
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
                            last_data_at = Some(Instant::now());

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
                                println!(
                                    "[serial {}] csv dataset complete ({} bytes):\n{}",
                                    target_port,
                                    buffer.as_bytes().len(),
                                    buffer
                                );
                                buffer.clear();
                                last_data_at = None;
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
                                    println!(
                                        "[serial {}] csv dataset complete ({} bytes):\n{}",
                                        target_port,
                                        buffer.as_bytes().len(),
                                        buffer
                                    );
                                    buffer.clear();
                                    last_data_at = None;
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
    tauri::Builder::default()
        .plugin(tauri_plugin_serialplugin::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_serial])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
