use js_sys::Function;
use leptos::prelude::*;
use leptos::web_sys::console;
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{prelude::*, JsCast};

/// Prediction data received from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionData {
    pub probability: f64,
    pub confidence: Option<f64>,
    pub dataset_id: String,
    pub processed_at: String,
}

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
/// - Sets up listeners for prediction events (loading, result, error)
pub async fn initialize_serial(
    set_latest_serial: WriteSignal<String>,
    set_connected: WriteSignal<bool>,
    set_prediction_loading: WriteSignal<bool>,
    set_prediction_result: WriteSignal<Option<PredictionData>>,
    set_prediction_error: WriteSignal<Option<String>>,
) {
    // Start the serial communication in the backend
    let _ = invoke("start_serial", JsValue::NULL).await;

    // Set up serial data event handler
    setup_serial_data_listener(set_latest_serial).await;

    // Set up connection status event handler
    setup_status_listener(set_connected).await;

    // Set up prediction event handlers
    setup_prediction_loading_listener(set_prediction_loading).await;
    setup_prediction_result_listener(set_prediction_result, set_prediction_loading).await;
    setup_prediction_error_listener(set_prediction_error, set_prediction_loading).await;
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

/// Set up listener for prediction loading events
async fn setup_prediction_loading_listener(set_prediction_loading: WriteSignal<bool>) {
    let update_loading = set_prediction_loading;
    let loading_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
        if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
            if let Ok(loading_val) = js_sys::Reflect::get(&payload, &JsValue::from_str("loading")) {
                if let Some(loading) = loading_val.as_bool() {
                    update_loading.set(loading);
                    console::log_1(&JsValue::from_str(&format!(
                        "serial:prediction_loading: {}",
                        loading
                    )));
                }
            }
        }
    }) as Box<dyn FnMut(JsValue)>);

    let _unlisten = listen(
        "serial:prediction_loading",
        loading_event_handler.as_ref().unchecked_ref(),
    )
    .await;
    loading_event_handler.forget();
}

/// Set up listener for prediction result events
async fn setup_prediction_result_listener(
    set_prediction_result: WriteSignal<Option<PredictionData>>,
    set_prediction_loading: WriteSignal<bool>,
) {
    let update_result = set_prediction_result;
    let update_loading = set_prediction_loading;
    let result_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
        if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
            // Try to deserialize the payload into PredictionData
            match serde_wasm_bindgen::from_value::<PredictionData>(payload.clone()) {
                Ok(prediction) => {
                    console::log_1(&JsValue::from_str(&format!(
                        "serial:prediction_result: probability={}",
                        prediction.probability
                    )));
                    update_result.set(Some(prediction));
                    update_loading.set(false);
                }
                Err(e) => {
                    console::log_1(&JsValue::from_str(&format!(
                        "Failed to parse prediction result: {:?}",
                        e
                    )));
                }
            }
        }
    }) as Box<dyn FnMut(JsValue)>);

    let _unlisten = listen(
        "serial:prediction_result",
        result_event_handler.as_ref().unchecked_ref(),
    )
    .await;
    result_event_handler.forget();
}

/// Set up listener for prediction error events
async fn setup_prediction_error_listener(
    set_prediction_error: WriteSignal<Option<String>>,
    set_prediction_loading: WriteSignal<bool>,
) {
    let update_error = set_prediction_error;
    let update_loading = set_prediction_loading;
    let error_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
        if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
            if let Ok(error_val) = js_sys::Reflect::get(&payload, &JsValue::from_str("error")) {
                if let Some(error_msg) = error_val.as_string() {
                    console::log_1(&JsValue::from_str(&format!(
                        "serial:prediction_error: {}",
                        error_msg
                    )));
                    update_error.set(Some(error_msg));
                    update_loading.set(false);
                }
            }
        }
    }) as Box<dyn FnMut(JsValue)>);

    let _unlisten = listen(
        "serial:prediction_error",
        error_event_handler.as_ref().unchecked_ref(),
    )
    .await;
    error_event_handler.forget();
}
