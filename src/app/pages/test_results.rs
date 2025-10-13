use crate::app::Page;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::console;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct CompleteTestData {
    test_uuid: String,
    detection_result: String,
    confidence: f64,
    raw_response: String,
}

#[derive(Serialize)]
struct CompleteTestArgs {
    data: CompleteTestData,
}

#[component]
pub fn TestResultsPage(
    on_navigate: WriteSignal<Page>,
    detection_result: ReadSignal<Option<crate::app::serial::DetectionData>>,
    current_test_uuid: ReadSignal<Option<String>>,
) -> impl IntoView {
    // Save results when detection completes
    Effect::new(move || {
        if let (Some(result), Some(test_uuid)) = (detection_result.get(), current_test_uuid.get()) {
            // Determine detection result based on probability
            let detection_result_str = if result.probability >= 0.7 {
                "positive"
            } else if result.probability >= 0.3 {
                "inconclusive"
            } else {
                "negative"
            };

            let test_uuid_clone = test_uuid.clone();
            let result_clone = result.clone();

            spawn_local(async move {
                let complete_data = CompleteTestData {
                    test_uuid: test_uuid_clone.clone(),
                    detection_result: detection_result_str.to_string(),
                    confidence: result_clone.confidence.unwrap_or(result_clone.probability),
                    raw_response: format!(
                        r#"{{"probability": {}, "confidence": {}, "dataset_id": "{}", "processed_at": "{}"}}"#,
                        result_clone.probability,
                        result_clone.confidence.unwrap_or(result_clone.probability),
                        result_clone.dataset_id,
                        result_clone.processed_at
                    ),
                };

                let args = CompleteTestArgs {
                    data: complete_data,
                };

                if let Err(e) = invoke(
                    "complete_test",
                    serde_wasm_bindgen::to_value(&args).unwrap(),
                )
                .await
                {
                    console::log_1(&JsValue::from_str(&format!(
                        "Failed to save test results: {:?}",
                        e
                    )));
                }
            });
        }
    });

    let on_new_test = move |_| {
        on_navigate.set(Page::Landing);
    };

    let on_view_history = move |_| {
        on_navigate.set(Page::History);
    };

    view! {
        <div class="animate-fade-in">
            // Header
            <header style="margin-bottom: 2rem;">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1rem;">
                    <h1 style="margin: 0; font-size: 2rem; font-weight: 300;">
                        "Test Results"
                    </h1>
                    <div style="display: flex; gap: 0.5rem;">
                        <button
                            class="button"
                            on:click=on_view_history
                            style="padding: 0.5rem 1rem;"
                        >
                            "View History"
                        </button>
                        <button
                            class="button"
                            on:click=on_new_test
                            style="padding: 0.5rem 1rem; background-color: var(--color-accent-primary); color: white;"
                        >
                            "New Test"
                        </button>
                    </div>
                </div>
            </header>

            // Results Content
            <div style="display: grid; gap: 2rem; max-width: 1200px; margin: 0 auto;">
                // Patient Information Card (Placeholder - will be populated from DB)
                <div class="card">
                    <h2 style="font-size: 1.25rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary); border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.5rem;">
                        "Patient Information"
                    </h2>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;">
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Patient Name"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-text-primary); font-weight: 500;">
                                "John Doe" // TODO: Get from patient record
                            </div>
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Date of Birth"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-text-primary);">
                                "1990-01-01" // TODO: Get from patient record
                            </div>
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Patient ID"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-text-primary);">
                                "P-12345" // TODO: Get from patient record
                            </div>
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Test Type"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-text-primary); font-weight: 500;">
                                "COVID-19" // TODO: Get from test record
                            </div>
                        </div>
                    </div>
                </div>

                // Detection Results Card
                <div class="card">
                    <h2 style="font-size: 1.25rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary); border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.5rem;">
                        "Detection Results"
                    </h2>

                    {move || {
                        if let Some(result) = detection_result.get() {
                            view! {
                                <div>
                                    <div style=format!(
                                        "padding: 1.5rem; border-radius: 8px; margin-bottom: 1.5rem; text-align: center; {}",
                                        if result.probability >= 0.7 {
                                            "background-color: #fee2e2; border: 2px solid #dc2626;"
                                        } else if result.probability >= 0.3 {
                                            "background-color: #fef3c7; border: 2px solid #f59e0b;"
                                        } else {
                                            "background-color: #dcfce7; border: 2px solid #16a34a;"
                                        }
                                    )>
                                        <div style="margin-bottom: 0.5rem;">
                                            <div style=format!(
                                                "width: 64px; height: 64px; border-radius: 50%; margin: 0 auto; display: flex; align-items: center; justify-content: center; {}",
                                                if result.probability >= 0.7 {
                                                    "background-color: #dc2626;"
                                                } else if result.probability >= 0.3 {
                                                    "background-color: #f59e0b;"
                                                } else {
                                                    "background-color: #16a34a;"
                                                }
                                            )>
                                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" style="color: white;">
                                                    {if result.probability >= 0.7 {
                                                        view! { <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM13 17H11V15H13V17ZM13 13H11V7H13V13Z" fill="currentColor"/> }
                                                    } else if result.probability >= 0.3 {
                                                        view! { <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM13 17H11V15H13V17ZM13 13H11V7H13V13Z" fill="currentColor"/> }
                                                    } else {
                                                        view! { <path d="M9 16.2L4.8 12L3.4 13.4L9 19L21 7L19.6 5.6L9 16.2Z" fill="currentColor"/> }
                                                    }}
                                                </svg>
                                            </div>
                                        </div>
                                        <div style=format!(
                                            "font-size: 1.5rem; font-weight: 600; margin-bottom: 0.5rem; {}",
                                            if result.probability >= 0.7 {
                                                "color: #991b1b;"
                                            } else if result.probability >= 0.3 {
                                                "color: #92400e;"
                                            } else {
                                                "color: #166534;"
                                            }
                                        )>
                                            {if result.probability >= 0.7 {
                                                "POSITIVE DETECTION"
                                            } else if result.probability >= 0.3 {
                                                "INCONCLUSIVE"
                                            } else {
                                                "NEGATIVE"
                                            }}
                                        </div>
                                        <div style=format!(
                                            "font-size: 1rem; font-weight: 500; {}",
                                            if result.probability >= 0.7 {
                                                "color: #7f1d1d;"
                                            } else if result.probability >= 0.3 {
                                                "color: #78350f;"
                                            } else {
                                                "color: #14532d;"
                                            }
                                        )>
                                            {format!("Confidence: {:.1}%", result.probability * 100.0)}
                                        </div>
                                    </div>

                                    // Detailed Metrics
                                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.5rem;">
                                        <div class="stat-card">
                                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                                                "Detection Probability"
                                            </div>
                                            <div style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary);">
                                                {format!("{:.1}%", result.probability * 100.0)}
                                            </div>
                                        </div>

                                        <div class="stat-card">
                                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                                                "Confidence Score"
                                            </div>
                                            <div style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary);">
                                                {result.confidence.map(|c| format!("{:.1}%", c * 100.0)).unwrap_or_else(|| "N/A".to_string())}
                                            </div>
                                        </div>

                                        <div class="stat-card">
                                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                                                "Dataset ID"
                                            </div>
                                            <div style="font-size: 0.875rem; font-weight: 400; color: var(--color-text-primary); font-family: monospace; word-break: break-all;">
                                                {result.dataset_id.clone()}
                                            </div>
                                        </div>
                                    </div>

                                    // Processed At
                                    <div style="margin-top: 1.5rem; padding: 1rem; background-color: var(--color-bg-secondary); border-radius: 8px;">
                                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                            "Processed At"
                                        </div>
                                        <div style="font-size: 0.875rem; color: var(--color-text-primary); font-family: monospace;">
                                            {result.processed_at.clone()}
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div style="text-align: center; padding: 3rem; color: var(--color-text-secondary);">
                                    <p style="font-size: 0.9375rem;">"No detection results available"</p>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                // Test Metadata Card
                <div class="card">
                    <h2 style="font-size: 1.25rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary); border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.5rem;">
                        "Test Metadata"
                    </h2>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.5rem;">
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Test Date"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-text-primary);">
                                {move || {
                                    if let Some(result) = detection_result.get() {
                                        result.processed_at.clone()
                                    } else {
                                        "N/A".to_string()
                                    }
                                }}
                            </div>
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Status"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-success); font-weight: 500;">
                                "Completed"
                            </div>
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Device Port"
                            </div>
                            <div style="font-size: 1rem; color: var(--color-text-primary); font-family: monospace;">
                                "COM3" // TODO: Get from test record
                            </div>
                        </div>
                    </div>
                </div>

                // Action Buttons
                <div style="display: flex; gap: 1rem; justify-content: center; padding-top: 1rem;">
                    <button
                        class="button"
                        on:click=on_view_history
                        style="padding: 0.75rem 1.5rem;"
                    >
                        "View All Tests"
                    </button>
                    <button
                        class="button"
                        on:click=on_new_test
                        style="padding: 0.75rem 2rem; background-color: var(--color-accent-primary); color: white; font-weight: 500;"
                    >
                        "Home"
                    </button>
                </div>
            </div>
        </div>
    }
}
