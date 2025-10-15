use crate::app::Page;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn ScientificViewPage(on_navigate: WriteSignal<Page>) -> impl IntoView {
    // Control panel signals
    let (selected_test_type, set_selected_test_type) = signal("All".to_string());
    let (selected_time_range, set_selected_time_range) = signal("Last 30 Days".to_string());
    let (selected_result_filter, set_selected_result_filter) = signal("All Results".to_string());
    let (analysis_running, set_analysis_running) = signal(false);

    let on_back_to_home = move |_| {
        on_navigate.set(Page::Landing);
    };

    let on_start_analysis = move |_: leptos::ev::MouseEvent| {
        set_analysis_running.set(true);
        // Simulate analysis process
        spawn_local(async move {
            // Simulate some processing time with a simple delay
            let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::NULL);
            wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();

            // Simple delay simulation
            for _ in 0..1000000 {
                // Busy wait to simulate processing
            }

            set_analysis_running.set(false);
        });
    };

    view! {
        <div class="container" style="max-width: 1400px; margin: 0 auto; padding: 2rem;">
            // Header with back button
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 2rem;">
                <div style="display: flex; align-items: center; gap: 1rem;">
                    <button
                        class="button"
                        on:click=on_back_to_home
                        style="padding: 0.5rem 1rem; font-size: 0.875rem;"
                    >
                        "← Back to Home"
                    </button>
                    <h1 style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary); margin: 0;">
                        "Scientific Analysis Dashboard"
                    </h1>
                </div>
            </div>

            // Main layout: Left panel + Center content
            <div style="display: grid; grid-template-columns: 300px 1fr; gap: 2rem; margin-bottom: 2rem;">
                // Left Control Panel
                <div class="card" style="padding: 1.5rem; height: fit-content;">
                    <h3 style="font-size: 1.125rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary);">
                        "Analysis Controls"
                    </h3>

                    // Test Type Filter
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                            "Test Type"
                        </label>
                        <select
                            style="width: 100%; padding: 0.75rem; border: 1px solid var(--color-border-light); border-radius: 4px; background-color: var(--color-bg-primary); color: var(--color-text-primary); font-size: 0.875rem;"
                            on:change=move |ev| {
                                if let Some(target) = ev.target() {
                                    if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                        set_selected_test_type.set(select.value());
                                    }
                                }
                            }
                        >
                            <option value="All">"All Types"</option>
                            <option value="PCR">"PCR Test"</option>
                            <option value="Antigen">"Antigen Test"</option>
                            <option value="Antibody">"Antibody Test"</option>
                            <option value="Molecular">"Molecular Test"</option>
                        </select>
                    </div>

                    // Time Range Filter
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                            "Time Range"
                        </label>
                        <select
                            style="width: 100%; padding: 0.75rem; border: 1px solid var(--color-border-light); border-radius: 4px; background-color: var(--color-bg-primary); color: var(--color-text-primary); font-size: 0.875rem;"
                            on:change=move |ev| {
                                if let Some(target) = ev.target() {
                                    if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                        set_selected_time_range.set(select.value());
                                    }
                                }
                            }
                        >
                            <option value="Last 7 Days">"Last 7 Days"</option>
                            <option value="Last 30 Days" selected>"Last 30 Days"</option>
                            <option value="Last 90 Days">"Last 90 Days"</option>
                            <option value="Last 6 Months">"Last 6 Months"</option>
                            <option value="Last Year">"Last Year"</option>
                            <option value="All Time">"All Time"</option>
                        </select>
                    </div>

                    // Result Filter
                    <div style="margin-bottom: 1.5rem;">
                        <label style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                            "Result Filter"
                        </label>
                        <select
                            style="width: 100%; padding: 0.75rem; border: 1px solid var(--color-border-light); border-radius: 4px; background-color: var(--color-bg-primary); color: var(--color-text-primary); font-size: 0.875rem;"
                            on:change=move |ev| {
                                if let Some(target) = ev.target() {
                                    if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                        set_selected_result_filter.set(select.value());
                                    }
                                }
                            }
                        >
                            <option value="All Results" selected>"All Results"</option>
                            <option value="Positive Only">"Positive Only"</option>
                            <option value="Negative Only">"Negative Only"</option>
                            <option value="Inconclusive Only">"Inconclusive Only"</option>
                            <option value="Pending Only">"Pending Only"</option>
                        </select>
                    </div>

                    // Analysis Type
                    <div style="margin-bottom: 2rem;">
                        <label style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--color-text-secondary); margin-bottom: 0.5rem;">
                            "Analysis Type"
                        </label>
                        <select
                            style="width: 100%; padding: 0.75rem; border: 1px solid var(--color-border-light); border-radius: 4px; background-color: var(--color-bg-primary); color: var(--color-text-primary); font-size: 0.875rem;"
                        >
                            <option value="trend">"Trend Analysis"</option>
                            <option value="distribution">"Result Distribution"</option>
                            <option value="accuracy">"Accuracy Metrics"</option>
                            <option value="temporal">"Temporal Patterns"</option>
                        </select>
                    </div>

                    // Start Analysis Button
                    <button
                        class="button primary"
                        on:click=on_start_analysis
                        disabled=move || analysis_running.get()
                        style="width: 100%; padding: 1rem; font-size: 1rem; font-weight: 500;"
                    >
                        {move || if analysis_running.get() { "Running Analysis..." } else { "Start Analysis" }}
                    </button>

                    // Current Selections Display
                    <div style="margin-top: 1.5rem; padding: 1rem; background-color: var(--color-bg-secondary); border-radius: 4px; border: 1px solid var(--color-border-light);">
                        <h4 style="font-size: 0.75rem; font-weight: 500; color: var(--color-text-secondary); margin-bottom: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em;">
                            "Current Selection"
                        </h4>
                        <div style="font-size: 0.75rem; color: var(--color-text-primary); line-height: 1.5;">
                            <div style="margin-bottom: 0.25rem;">
                                <strong>"Type: "</strong> {move || selected_test_type.get()}
                            </div>
                            <div style="margin-bottom: 0.25rem;">
                                <strong>"Range: "</strong> {move || selected_time_range.get()}
                            </div>
                            <div>
                                <strong>"Filter: "</strong> {move || selected_result_filter.get()}
                            </div>
                        </div>
                    </div>
                </div>

                // Center Content Area
                <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                    // Graph Area
                    <div class="card" style="padding: 2rem; min-height: 400px;">
                        <h3 style="font-size: 1.125rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary);">
                            "Data Visualization"
                        </h3>

                        {move || {
                            if analysis_running.get() {
                                view! {
                                    <div style="display: flex; align-items: center; justify-content: center; height: 300px; color: var(--color-text-secondary);">
                                        <div style="text-align: center;">
                                            <div style="width: 40px; height: 40px; border: 3px solid var(--color-border-light); border-top: 3px solid var(--color-primary); border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 1rem auto;"></div>
                                            <p style="margin: 0; font-size: 0.9375rem;">
                                                "Analyzing data..."
                                            </p>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="display: flex; align-items: center; justify-content: center; height: 300px; border: 2px dashed var(--color-border-light); border-radius: 8px; color: var(--color-text-secondary);">
                                        <div style="text-align: center;">
                                            <div style="font-size: 3rem; margin-bottom: 1rem; opacity: 0.3;">
                                                "📊"
                                            </div>
                                            <p style="margin: 0; font-size: 1rem; font-weight: 500;">
                                                "Interactive Graph Area"
                                            </p>
                                            <p style="margin: 0.5rem 0 0 0; font-size: 0.875rem;">
                                                "Click 'Start Analysis' to generate visualizations"
                                            </p>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>


        </div>
    }
}
