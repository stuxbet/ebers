use crate::app::Page;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Detection record from the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRecord {
    pub id: Option<i64>,
    pub uuid: String,
    pub port: String,
    pub baud_rate: i32,
    pub collection_duration_ms: i64,
    pub detection_result: Option<String>,
    pub confidence: Option<f64>,
    pub raw_response: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn DetectionsPage(on_navigate_to_home: WriteSignal<Page>) -> impl IntoView {
    let (detections, set_detections) = signal(Vec::<DetectionRecord>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (filter_status, set_filter_status) = signal(String::from("all"));

    // Function to load detections
    let load_detections = move || {
        leptos::task::spawn_local(async move {
            use leptos::web_sys::console;

            set_loading.set(true);
            set_error.set(None);

            console::log_1(&JsValue::from_str("Fetching detections from database..."));

            match invoke("get_all_detections", JsValue::NULL).await {
                Ok(result) => {
                    console::log_1(&JsValue::from_str("Received result from backend"));
                    console::log_1(&result);

                    match serde_wasm_bindgen::from_value::<Vec<DetectionRecord>>(result) {
                        Ok(detects) => {
                            console::log_1(&JsValue::from_str(&format!(
                                "Successfully parsed {} detections",
                                detects.len()
                            )));
                            set_detections.set(detects);
                            set_loading.set(false);
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to parse detections: {:?}", e);
                            console::log_1(&JsValue::from_str(&error_msg));
                            set_error.set(Some(error_msg));
                            set_loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to load detections: {:?}", e);
                    console::log_1(&JsValue::from_str(&error_msg));
                    set_error.set(Some(error_msg));
                    set_loading.set(false);
                }
            }
        });
    };

    // Load detections on mount
    Effect::new(move || {
        load_detections();
    });

    // Handler to insert test data
    let insert_test = move |_| {
        leptos::task::spawn_local(async move {
            use leptos::web_sys::console;

            console::log_1(&JsValue::from_str("Inserting test detection..."));

            match invoke("insert_test_detection", JsValue::NULL).await {
                Ok(result) => {
                    console::log_1(&JsValue::from_str("Test detection inserted"));
                    console::log_1(&result);
                    // Reload detections
                    load_detections();
                }
                Err(e) => {
                    console::log_1(&JsValue::from_str(&format!(
                        "Failed to insert test: {:?}",
                        e
                    )));
                }
            }
        });
    };

    // Filtered detections based on status
    let filtered_detections = move || {
        let filter = filter_status.get();
        let all_detects = detections.get();

        if filter == "all" {
            all_detects
        } else {
            all_detects
                .into_iter()
                .filter(|d| d.status == filter)
                .collect()
        }
    };

    view! {
        <div class="animate-fade-in">
            // Header with back button
            <header style="margin-bottom: 2rem;">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1rem;">
                    <button
                        class="button"
                        on:click=move |_| on_navigate_to_home.set(Page::TestStart)
                        style="padding: 0.5rem 1rem;"
                    >
                        "← Back to Home"
                    </button>
                    <h1 style="margin: 0; font-size: 2rem; font-weight: 300;">
                        "Test History"
                    </h1>
                    <div style="display: flex; gap: 0.5rem;">
                        <button
                            class="button"
                            on:click=move |_| load_detections()
                            style="padding: 0.5rem 1rem;"
                        >
                            "🔄 Refresh"
                        </button>
                        <button
                            class="button"
                            on:click=insert_test
                            style="padding: 0.5rem 1rem; background-color: var(--color-success);"
                        >
                            "➕ Add Test"
                        </button>
                    </div>
                </div>

                // Filter buttons
                <div style="display: flex; gap: 0.5rem; justify-content: center; flex-wrap: wrap;">
                    <button
                        class=move || if filter_status.get() == "all" { "button button-primary" } else { "button" }
                        on:click=move |_| set_filter_status.set("all".to_string())
                    >
                        "All"
                    </button>
                    <button
                        class=move || if filter_status.get() == "success" { "button button-primary" } else { "button" }
                        on:click=move |_| set_filter_status.set("success".to_string())
                    >
                        "Success"
                    </button>
                    <button
                        class=move || if filter_status.get() == "pending" { "button button-primary" } else { "button" }
                        on:click=move |_| set_filter_status.set("pending".to_string())
                    >
                        "Pending"
                    </button>
                    <button
                        class=move || if filter_status.get() == "error" { "button button-primary" } else { "button" }
                        on:click=move |_| set_filter_status.set("error".to_string())
                    >
                        "Error"
                    </button>
                </div>
            </header>

            // Loading state
            {move || if loading.get() {
                view! {
                    <div style="text-align: center; padding: 3rem;">
                        <p>"Loading detections..."</p>
                    </div>
                }.into_any()
            } else if let Some(err) = error.get() {
                // Error state
                view! {
                    <div class="card" style="background-color: var(--color-error-bg); border-color: var(--color-error); padding: 1.5rem;">
                        <h3 style="color: var(--color-error); margin-top: 0;">"Error"</h3>
                        <p>{err}</p>
                    </div>
                }.into_any()
            } else {
                // Detections list
                let detects = filtered_detections();
                if detects.is_empty() {
                    view! {
                        <div style="text-align: center; padding: 3rem;">
                            <p style="color: var(--color-text-secondary);">
                                "No detections found"
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="display: flex; flex-direction: column; gap: 1rem;">
                            <div style="color: var(--color-text-secondary); font-size: 0.875rem;">
                                {format!("Showing {} detection(s)", detects.len())}
                            </div>
                            {detects.into_iter().map(|detect| {
                                view! {
                                    <DetectionCard detection=detect />
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn DetectionCard(detection: DetectionRecord) -> impl IntoView {
    let status_color = match detection.status.as_str() {
        "success" => "var(--color-success)",
        "error" => "var(--color-error)",
        "pending" => "var(--color-warning)",
        _ => "var(--color-text-secondary)",
    };

    let status_bg = match detection.status.as_str() {
        "success" => "var(--color-success-bg)",
        "error" => "var(--color-error-bg)",
        "pending" => "var(--color-warning-bg)",
        _ => "var(--color-bg-secondary)",
    };

    view! {
        <div class="card" style="padding: 1.5rem;">
            <div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;">
                <div>
                    <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 0.5rem;">
                        <span
                            style=format!(
                                "display: inline-block; padding: 0.25rem 0.75rem; border-radius: 12px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; background-color: {}; color: {};",
                                status_bg, status_color
                            )
                        >
                            {detection.status.clone()}
                        </span>
                        <span style="color: var(--color-text-secondary); font-size: 0.875rem;">
                            {format_timestamp(&detection.created_at)}
                        </span>
                    </div>
                    <div style="font-family: monospace; font-size: 0.75rem; color: var(--color-text-secondary);">
                        {format!("ID: {}", detection.uuid)}
                    </div>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 1rem;">
                <div>
                    <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                        "Port"
                    </div>
                    <div style="font-weight: 500; color: var(--color-text-primary);">
                        {detection.port.clone()}
                    </div>
                </div>
                <div>
                    <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                        "Baud Rate"
                    </div>
                    <div style="font-weight: 500; color: var(--color-text-primary);">
                        {format!("{}", detection.baud_rate)}
                    </div>
                </div>
                <div>
                    <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                        "Duration"
                    </div>
                    <div style="font-weight: 500; color: var(--color-text-primary);">
                        {format!("{}ms", detection.collection_duration_ms)}
                    </div>
                </div>
                {detection.confidence.map(|conf| {
                    view! {
                        <div>
                            <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Confidence"
                            </div>
                            <div style="font-weight: 500; color: var(--color-text-primary);">
                                {format!("{:.2}%", conf * 100.0)}
                            </div>
                        </div>
                    }
                })}
            </div>

            {detection.detection_result.as_ref().map(|result| {
                view! {
                    <div style="margin-bottom: 1rem;">
                        <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                            "Detection Result"
                        </div>
                        <div style="padding: 0.75rem; background-color: var(--color-bg-secondary); border-radius: 6px; font-family: monospace; font-size: 0.875rem; color: var(--color-text-primary);">
                            {result.clone()}
                        </div>
                    </div>
                }
            })}

            {detection.error_message.as_ref().map(|error| {
                view! {
                    <div>
                        <div style="font-size: 0.75rem; color: var(--color-error); margin-bottom: 0.25rem;">
                            "Error Message"
                        </div>
                        <div style="padding: 0.75rem; background-color: var(--color-error-bg); border: 1px solid var(--color-error); border-radius: 6px; font-size: 0.875rem; color: var(--color-error);">
                            {error.clone()}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

/// Format timestamp to a more readable format
fn format_timestamp(timestamp: &str) -> String {
    // Try to parse and format the timestamp
    // For now, just return the timestamp as-is
    // You can enhance this with chrono or js_sys::Date if needed
    timestamp.to_string()
}
