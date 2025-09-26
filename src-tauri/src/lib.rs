use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_serialplugin::{commands, desktop_api};

#[tauri::command]
async fn start_serial(app: AppHandle) -> Result<(), String> {
    let port = "COM3".to_string(); // adjust for your device
    let serial = app.state::<desktop_api::SerialPort<tauri::Wry>>();

    println!("[serial] opening {} @ {} baud", port, 115_200);
    let open_res = commands::open(
        app.clone(),
        serial.clone(),
        port.clone(),
        115_200,
        None,
        None,
        None,
        None,
        Some(50),
    );
    match open_res {
        Ok(_) => println!("[serial] opened {}", port),
        Err(e) => {
            println!("[serial] failed to open {}: {}", port, e);
            return Err(e.to_string());
        }
    };
    tauri::async_runtime::spawn({
        let app = app.clone();
        let port = port.clone();
        async move {
            let mut buffer = String::new();
            let mut last_data_at: Option<Instant> = None;
            let idle_gap = Duration::from_millis(2000);

            loop {
                match commands::read(
                    app.clone(),
                    app.state::<desktop_api::SerialPort<tauri::Wry>>().clone(),
                    port.clone(),
                    Some(50),
                    Some(1024),
                ) {
                    Ok(chunk) => {
                        if !chunk.is_empty() {
                            // append to csv dataset
                            buffer.push_str(&chunk);
                            last_data_at = Some(Instant::now());

                            // TODO: remove in future if not needed
                            let _ = app.emit("serial:data", &chunk);
                            println!(
                                "[serial {}] {} ({} bytes)",
                                port,
                                chunk,
                                chunk.as_bytes().len()
                            );
                        }
                    }
                    Err(_) => {
                        // timeout or no data; check for idle gap end-of-dataset
                        if let Some(t) = last_data_at {
                            if t.elapsed() >= idle_gap && !buffer.is_empty() {
                                println!(
                                    "[serial {}] csv dataset complete ({} bytes):\n{}",
                                    port,
                                    buffer.as_bytes().len(),
                                    buffer
                                );
                                buffer.clear();
                                last_data_at = None;
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
