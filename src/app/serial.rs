use js_sys::Function;
use leptos::prelude::*;
use leptos::web_sys::console;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Function) -> JsValue;
}

/// Initialize serial communication and set up event listeners
/// 
/// This function:
/// - Invokes the backend `start_serial` command
/// - Sets up a listener for `serial:data` events to receive serial data
/// - Sets up a listener for `serial:status` events to track connection status
pub async fn initialize_serial(
    set_latest_serial: WriteSignal<String>,
    set_connected: WriteSignal<bool>,
) {
    // Start the serial communication in the backend
    let _ = invoke("start_serial", JsValue::NULL).await;

    // Set up serial data event handler
    setup_serial_data_listener(set_latest_serial).await;
    
    // Set up connection status event handler
    setup_status_listener(set_connected).await;
}

/// Set up listener for serial data events
async fn setup_serial_data_listener(set_latest_serial: WriteSignal<String>) {
    let update_latest_serial = set_latest_serial;
    let serial_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
        if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
            if let Some(s) = payload.as_string() {
                update_latest_serial.set(s.clone());
                console::log_1(&JsValue::from_str(&format!("serial:data: {}", s)));
            } else {
                console::log_1(&payload);
            }
        } else {
            console::log_1(&JsValue::from_str("serial:data: <no payload>"));
        }
    }) as Box<dyn FnMut(JsValue)>);

    let _unlisten = listen("serial:data", serial_event_handler.as_ref().unchecked_ref()).await;
    serial_event_handler.forget();
}

/// Set up listener for connection status events
async fn setup_status_listener(set_connected: WriteSignal<bool>) {
    let update_connected = set_connected;
    let status_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
        if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
            // Try payload.connected boolean first
            if let Ok(val) = js_sys::Reflect::get(&payload, &JsValue::from_str("connected")) {
                if let Some(b) = val.as_bool() {
                    update_connected.set(b);
                    console::log_1(&JsValue::from_str(&format!(
                        "serial:status connected={}",
                        b
                    )));
                }
            } else if let Some(b) = payload.as_bool() {
                // Or a bare boolean payload
                update_connected.set(b);
                console::log_1(&JsValue::from_str(&format!(
                    "serial:status connected={} (bool)",
                    b
                )));
            } else {
                console::log_1(&payload);
            }
        } else {
            console::log_1(&JsValue::from_str("serial:status: <no payload>"));
        }
    }) as Box<dyn FnMut(JsValue)>);
    
    let _unlisten_status = listen(
        "serial:status",
        status_event_handler.as_ref().unchecked_ref(),
    )
    .await;
    status_event_handler.forget();
}
