use crate::app::Page;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Use shared types
use shared_types::{Patient, Test, TestWithPatient};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn DetectionsPage(on_navigate_to_home: WriteSignal<Page>) -> impl IntoView {
    let (tests, set_tests) = signal(Vec::<TestWithPatient>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (filter_status, set_filter_status) = signal(String::from("all"));

    // Function to load tests
    let load_tests = move || {
        leptos::task::spawn_local(async move {
            use leptos::web_sys::console;

            set_loading.set(true);
            set_error.set(None);

            console::log_1(&JsValue::from_str("Fetching tests from database..."));

            match invoke("get_all_tests", JsValue::NULL).await {
                Ok(result) => {
                    console::log_1(&JsValue::from_str("Received result from backend"));
                    console::log_1(&result);

                    match serde_wasm_bindgen::from_value::<Vec<TestWithPatient>>(result) {
                        Ok(test_list) => {
                            console::log_1(&JsValue::from_str(&format!(
                                "Successfully parsed {} tests",
                                test_list.len()
                            )));
                            set_tests.set(test_list);
                            set_loading.set(false);
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to parse tests: {:?}", e);
                            console::log_1(&JsValue::from_str(&error_msg));
                            set_error.set(Some(error_msg));
                            set_loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to load tests: {:?}", e);
                    console::log_1(&JsValue::from_str(&error_msg));
                    set_error.set(Some(error_msg));
                    set_loading.set(false);
                }
            }
        });
    };

    // Load tests on mount
    Effect::new(move || {
        load_tests();
    });

    // Filtered tests based on status
    let filtered_tests = move || {
        let filter = filter_status.get();
        let all_tests = tests.get();

        if filter == "all" {
            all_tests
        } else {
            all_tests
                .into_iter()
                .filter(|t| t.test.status == filter)
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
                        on:click=move |_| on_navigate_to_home.set(Page::Landing)
                        style="padding: 0.5rem 1rem;"
                    >
                        "← Back to Home"
                    </button>
                    <h1 style="margin: 0; font-size: 2rem; font-weight: 300;">
                        "Test History"
                    </h1>
                    <button
                        class="button"
                        on:click=move |_| load_tests()
                        style="padding: 0.5rem 1rem;"
                    >
                        "Refresh"
                    </button>
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
                        class=move || if filter_status.get() == "completed" { "button button-primary" } else { "button" }
                        on:click=move |_| set_filter_status.set("completed".to_string())
                    >
                        "Completed"
                    </button>
                    <button
                        class=move || if filter_status.get() == "in_progress" { "button button-primary" } else { "button" }
                        on:click=move |_| set_filter_status.set("in_progress".to_string())
                    >
                        "In Progress"
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
                        <p>"Loading tests..."</p>
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
                // Tests list
                let test_list = filtered_tests();
                if test_list.is_empty() {
                    view! {
                        <div style="text-align: center; padding: 3rem;">
                            <p style="color: var(--color-text-secondary);">
                                "No tests found"
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="display: flex; flex-direction: column; gap: 1rem;">
                            <div style="color: var(--color-text-secondary); font-size: 0.875rem;">
                                {format!("Showing {} test(s)", test_list.len())}
                            </div>
                            {test_list.into_iter().map(|test_with_patient| {
                                view! {
                                    <TestCard test_with_patient=test_with_patient />
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
fn TestCard(test_with_patient: TestWithPatient) -> impl IntoView {
    let test = test_with_patient.test;
    let patient = test_with_patient.patient;

    let status_color = match test.status.as_str() {
        "completed" => "var(--color-success)",
        "error" => "var(--color-error)",
        "in_progress" => "var(--color-info)",
        "pending" => "var(--color-warning)",
        _ => "var(--color-text-secondary)",
    };

    let status_bg = match test.status.as_str() {
        "completed" => "var(--color-success-bg)",
        "error" => "var(--color-error-bg)",
        "in_progress" => "var(--color-info-bg)",
        "pending" => "var(--color-warning-bg)",
        _ => "var(--color-bg-secondary)",
    };

    let result_color = match test.detection_result.as_deref() {
        Some("positive") => "var(--color-error)",
        Some("negative") => "var(--color-success)",
        Some("inconclusive") => "var(--color-warning)",
        _ => "var(--color-text-secondary)",
    };

    let result_bg = match test.detection_result.as_deref() {
        Some("positive") => "var(--color-error-bg)",
        Some("negative") => "var(--color-success-bg)",
        Some("inconclusive") => "var(--color-warning-bg)",
        _ => "var(--color-bg-secondary)",
    };

    view! {
        <div class="card" style="padding: 1.5rem;">
            // Header with patient info and status
            <div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;">
                <div style="flex: 1;">
                    <h3 style="margin: 0 0 0.5rem 0; font-size: 1.25rem; font-weight: 500;">
                        {format!("{} {}", patient.first_name, patient.last_name)}
                    </h3>
                    {patient.patient_id_number.as_ref().map(|id| {
                        view! {
                            <div style="color: var(--color-text-secondary); font-size: 0.875rem; margin-bottom: 0.25rem;">
                                {format!("Patient ID: {}", id)}
                            </div>
                        }
                    })}
                    <div style="font-family: monospace; font-size: 0.75rem; color: var(--color-text-secondary);">
                        {format!("Test ID: {}", test.uuid)}
                    </div>
                </div>
                <div style="display: flex; flex-direction: column; align-items: flex-end; gap: 0.5rem;">
                    <span
                        style=format!(
                            "display: inline-block; padding: 0.25rem 0.75rem; border-radius: 12px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; background-color: {}; color: {};",
                            status_bg, status_color
                        )
                    >
                        {test.status.clone()}
                    </span>
                    <span style="color: var(--color-text-secondary); font-size: 0.875rem;">
                        {format_timestamp(&test.created_at)}
                    </span>
                </div>
            </div>

            // Test information grid
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 1rem;">
                <div>
                    <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                        "Test Type"
                    </div>
                    <div style="font-weight: 500; color: var(--color-text-primary); text-transform: capitalize;">
                        {test.test_type.clone()}
                    </div>
                </div>
                {patient.date_of_birth.as_ref().map(|dob| {
                    view! {
                        <div>
                            <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Date of Birth"
                            </div>
                            <div style="font-weight: 500; color: var(--color-text-primary);">
                                {dob.clone()}
                            </div>
                        </div>
                    }
                })}
                {test.device_id.as_ref().map(|device| {
                    view! {
                        <div>
                            <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Device ID"
                            </div>
                            <div style="font-weight: 500; color: var(--color-text-primary);">
                                {device.clone()}
                            </div>
                        </div>
                    }
                })}
                {test.firmware_version.as_ref().map(|fw| {
                    view! {
                        <div>
                            <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Firmware"
                            </div>
                            <div style="font-weight: 500; color: var(--color-text-primary);">
                                {fw.clone()}
                            </div>
                        </div>
                    }
                })}
                {test.confidence.map(|conf| {
                    view! {
                        <div>
                            <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Confidence"
                            </div>
                            <div style="font-weight: 500; color: var(--color-text-primary);">
                                {format!("{:.1}%", conf * 100.0)}
                            </div>
                        </div>
                    }
                })}
            </div>

            // Detection result (if available)
            {test.detection_result.as_ref().map(|result| {
                view! {
                    <div style="margin-bottom: 1rem;">
                        <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                            "Detection Result"
                        </div>
                        <div
                            style=format!(
                                "padding: 1rem; border-radius: 8px; font-weight: 600; text-transform: uppercase; text-align: center; background-color: {}; color: {};",
                                result_bg, result_color
                            )
                        >
                            {result.clone()}
                        </div>
                    </div>
                }
            })}

            // Raw response (if available)
            {test.raw_response.as_ref().map(|response| {
                view! {
                    <div style="margin-bottom: 1rem;">
                        <div style="font-size: 0.75rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                            "Raw Response"
                        </div>
                        <div style="padding: 0.75rem; background-color: var(--color-bg-secondary); border-radius: 6px; font-family: monospace; font-size: 0.875rem; color: var(--color-text-primary); max-height: 200px; overflow-y: auto;">
                            {response.clone()}
                        </div>
                    </div>
                }
            })}

            // Error message (if any)
            {test.error_message.as_ref().map(|error| {
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
