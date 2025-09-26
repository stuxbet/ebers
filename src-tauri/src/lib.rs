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
                            //TODO: remove in future. just for testing
                            let _ = app.emit("serial:data", &chunk);
                            println!(
                                "[serial {}] {} ({} bytes)",
                                port,
                                chunk,
                                chunk.as_bytes().len()
                            );
                        }
                    } // push to frontend
                    Err(_) => { /* timeout or port empty; loop again */ }
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
