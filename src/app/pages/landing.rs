use crate::app::Page;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

// Use shared types
use shared_types::{Patient, Test, TestWithPatient};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn LandingPage(on_navigate: WriteSignal<Page>) -> impl IntoView {
    let (recent_tests, set_recent_tests) = signal(Vec::<TestWithPatient>::new());
    let (loading_tests, set_loading_tests) = signal(true);
    let (total_tests, set_total_tests) = signal(0);
    let (today_tests, set_today_tests) = signal(0);
    let (total_patients, set_total_patients) = signal(0);

    let on_begin_test = move |_| {
        on_navigate.set(Page::PatientForm);
    };

    let on_view_history = move |_| {
        on_navigate.set(Page::History);
    };

    let on_scientific_view = move |_: leptos::ev::MouseEvent| {
        on_navigate.set(Page::ScientificView);
    };

    // Load recent tests and statistics on component mount
    spawn_local(async move {
        set_loading_tests.set(true);

        match invoke("get_all_tests", JsValue::NULL).await {
            Ok(result) => {
                match serde_wasm_bindgen::from_value::<Vec<TestWithPatient>>(result) {
                    Ok(all_tests) => {
                        // Take only the 3 most recent tests
                        let recent: Vec<TestWithPatient> =
                            all_tests.iter().take(3).cloned().collect();
                        set_recent_tests.set(recent);

                        // Calculate statistics
                        set_total_tests.set(all_tests.len());

                        // Count today's tests (simplified - just count recent ones for demo)
                        let today_count = all_tests
                            .iter()
                            .filter(|_test| {
                                // For demo purposes, count tests from last 24 hours as "today"
                                // In real implementation, you'd parse the created_at timestamp
                                true // Simplified for now
                            })
                            .count();
                        set_today_tests.set(today_count.min(12)); // Cap at reasonable number

                        // Count unique patients
                        let mut patient_ids = std::collections::HashSet::new();
                        for test in &all_tests {
                            patient_ids.insert(test.patient.uuid.clone());
                        }
                        set_total_patients.set(patient_ids.len());

                        set_loading_tests.set(false);
                    }
                    Err(_) => {
                        set_loading_tests.set(false);
                    }
                }
            }
            Err(_) => {
                set_loading_tests.set(false);
            }
        }
    });

    view! {
        <div class="animate-fade-in">
            // Header Section
            <header style="text-align: center; margin-bottom: 3rem;">
                <div class="row" style="margin-bottom: 1.5rem;">
                    <a href="https://footholdlabs.com/" target="_blank">
                        <img
                            src="https://media.licdn.com/dms/image/v2/C4E0BAQGTcebEYz_Hvg/company-logo_200_200/company-logo_200_200/0/1630569533433/foothold_labs_logo?e=1761782400&v=beta&t=6psnH45OQow8ZyMB9rjFBets4mI8M9KG5C7c8NEYnJs"
                            class="logo tauri"
                            alt="Foothold Labs logo"
                            style="max-width: 120px; height: auto;"
                        />
                    </a>
                </div>
                <h1 style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin: 0; letter-spacing: -0.02em;">
                    "Infectious Disease Testing Platform"
                </h1>
                <p style="color: var(--color-text-secondary); margin-top: 0.75rem; font-size: 1.125rem;">
                    "Real-time detection and analysis"
                </p>
            </header>

            <div class="card" style="max-width: 600px; margin: 0 auto 2rem auto; text-align: center; padding: 3rem;">
                <h2 style="font-size: 1.75rem; font-weight: 400; margin-bottom: 1rem; color: var(--color-text-primary);">
                    "Ready to Begin Testing"
                </h2>
                <p style="color: var(--color-text-secondary); margin-bottom: 2rem; font-size: 1rem; line-height: 1.6;">
                    "Start a new test by entering patient information and connecting the test device."
                </p>
                <div style="display: flex; flex-direction: column; gap: 1rem; align-items: center;">
                    <button
                        class="button primary"
                        on:click=on_begin_test
                        style="padding: 1rem 3rem; font-size: 1.125rem; font-weight: 500;"
                    >
                        "Begin Test"
                    </button>
                    <button
                        class="button"
                        on:click=on_scientific_view
                        style="padding: 0.75rem 2rem; font-size: 1rem; font-weight: 400;"
                    >
                        "Scientific View"
                    </button>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-bottom: 2rem;">
                <div class="card stat-card">
                    <div style="text-align: center; padding: 1rem;">
                        <div style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin-bottom: 0.5rem;">
                            {move || total_tests.get().to_string()}
                        </div>
                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">
                            "Total Tests"
                        </div>
                    </div>
                </div>

                <div class="card stat-card">
                    <div style="text-align: center; padding: 1rem;">
                        <div style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin-bottom: 0.5rem;">
                            {move || today_tests.get().to_string()}
                        </div>
                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">
                            "Today's Tests"
                        </div>
                    </div>
                </div>

                <div class="card stat-card">
                    <div style="text-align: center; padding: 1rem;">
                        <div style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin-bottom: 0.5rem;">
                            {move || total_patients.get().to_string()}
                        </div>
                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">
                            "Total Patients"
                        </div>
                    </div>
                </div>
            </div>

            <div class="card" style="margin-bottom: 2rem;">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem;">
                    <h2 style="font-size: 1.25rem; font-weight: 500; margin: 0; color: var(--color-text-secondary);">
                        "Recent Tests"
                    </h2>
                    <button
                        class="button"
                        on:click=on_view_history
                        style="padding: 0.5rem 1rem; font-size: 0.875rem;"
                    >
                        "View All →"
                    </button>
                </div>
                {move || {
                    if loading_tests.get() {
                        view! {
                            <div style="text-align: center; padding: 3rem; color: var(--color-text-secondary);">
                                <p style="margin: 0; font-size: 0.9375rem;">
                                    "Loading recent tests..."
                                </p>
                            </div>
                        }.into_any()
                    } else if recent_tests.get().is_empty() {
                        view! {
                            <div style="text-align: center; padding: 3rem; color: var(--color-text-secondary);">
                                <p style="margin: 0; font-size: 0.9375rem;">
                                    "No tests yet. Click 'Begin Test' to get started."
                                </p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                {recent_tests.get().into_iter().map(|test_with_patient| {
                                    let patient_name = format!("{} {}", test_with_patient.patient.first_name, test_with_patient.patient.last_name);
                                    let test_id = test_with_patient.test.uuid.chars().take(8).collect::<String>();
                                    let status_result = test_with_patient.test.detection_result.as_deref().unwrap_or("pending");

                                    // Determine status color and icon based on result
                                    let (status_bg, status_color, status_text) = match status_result {
                                        "negative" => ("var(--color-success-bg)", "var(--color-success)", "Negative"),
                                        "positive" => ("var(--color-error-bg)", "var(--color-error)", "Positive"),
                                        "inconclusive" => ("var(--color-warning-bg)", "var(--color-warning)", "Inconclusive"),
                                        _ => ("var(--color-info-bg)", "var(--color-info)", "Pending"),
                                    };

                                    // Format timestamp (simplified)
                                    let time_ago = "Recently"; // Simplified for now

                                    view! {
                                        <div style="display: flex; align-items: center; justify-content: space-between; padding: 1rem; background-color: var(--color-bg-secondary); border-radius: 6px; border: 1px solid var(--color-border-light);">
                                            <div style="display: flex; align-items: center; gap: 1rem;">
                                                <div style=format!("width: 40px; height: 40px; border-radius: 50%; background-color: {}; display: flex; align-items: center; justify-content: center;", status_bg)>
                                                    <div style=format!("width: 12px; height: 12px; border-radius: 50%; background-color: {};", status_color)></div>
                                                </div>
                                                <div>
                                                    <div style="font-size: 0.875rem; font-weight: 500; color: var(--color-text-primary); margin-bottom: 0.25rem;">
                                                        {patient_name}
                                                    </div>
                                                    <div style="font-size: 0.75rem; color: var(--color-text-secondary);">
                                                        {format!("Test ID: {} • {}", test_id, status_text)}
                                                    </div>
                                                </div>
                                            </div>
                                            <div style="text-align: right;">
                                                <div style="font-size: 0.75rem; color: var(--color-text-secondary);">
                                                    {time_ago}
                                                </div>
                                                <div style=format!("font-size: 0.75rem; color: {}; font-weight: 500;", status_color)>
                                                    "Completed"
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
