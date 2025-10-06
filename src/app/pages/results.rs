use crate::app::serial::PredictionData;
use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn ResultsPage(
    latest_serial: ReadSignal<String>,
    on_navigate_to_home: WriteSignal<Page>,
    prediction_loading: ReadSignal<bool>,
    prediction_result: ReadSignal<Option<PredictionData>>,
    prediction_error: ReadSignal<Option<String>>,
) -> impl IntoView {
    let has_data = move || !latest_serial.get().is_empty();

    view! {
        <div class="animate-fade-in">
            <div class="results-header">
                <div>
                    <h1 style="margin: 0; font-size: 2rem; font-weight: 300; color: var(--color-text-primary);">
                        "Data Results"
                    </h1>
                    <p style="margin: 0.5rem 0 0 0; color: var(--color-text-secondary); font-size: 0.875rem;">
                        "Real-time serial data stream"
                    </p>
                </div>
                <button
                    class="home-btn"
                    on:click=move |_| on_navigate_to_home.set(Page::Home)
                >
                    "← Home"
                </button>
            </div>

            <div class="results-content">
                // Live Data Stream Card
                <div class="card card-elevated animate-fade-in" style="animation-delay: 100ms;">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                        <div>
                            <div class="asset-header">"LIVE DATA STREAM"</div>
                        </div>
                        <Show
                            when=has_data
                            fallback=|| view! {
                                <span class="badge secondary">
                                    "Idle"
                                </span>
                            }
                        >
                            <span class="badge" style="background-color: var(--color-success);">
                                <span class="animate-pulse">"●"</span>
                                " Streaming"
                            </span>
                        </Show>
                    </div>

                    <div class="serial-data">
                        <pre class="serial-output">
                            {move || {
                                let data = latest_serial.get();
                                if data.is_empty() {
                                    "Waiting for data...\n\nConnect a device to start receiving serial data.".to_string()
                                } else {
                                    data
                                }
                            }}
                        </pre>
                    </div>

                    <div class="asset-info" style="margin-top: 1.5rem;">
                        <div class="asset-info-row">
                            <span class="asset-label">"Data Points"</span>
                            <span class="asset-value">
                                {move || {
                                    let data = latest_serial.get();
                                    if data.is_empty() {
                                        "0".to_string()
                                    } else {
                                        data.lines().count().to_string()
                                    }
                                }}
                            </span>
                        </div>
                        <div class="asset-info-row">
                            <span class="asset-label">"Status"</span>
                            <span class={move || if has_data() { "status-connected" } else { "status-disconnected" }}>
                                {move || if has_data() { "Active" } else { "Idle" }}
                            </span>
                        </div>
                    </div>
                </div>

                // Info Cards Grid
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-top: 2rem;">
                    <div class="card animate-fade-in" style="animation-delay: 200ms;">
                        <div class="asset-header">"PROCESSING"</div>
                        <p style="color: var(--color-text-secondary); font-size: 0.875rem; margin: 1rem 0;">
                            "Data is automatically aggregated and processed in the backend."
                        </p>
                        <div class="badge secondary" style="font-size: 0.75rem;">
                            "Auto-aggregation"
                        </div>
                    </div>

                    <div class="card animate-fade-in" style="animation-delay: 300ms;">
                        <div class="asset-header">"EXPORT"</div>
                        <p style="color: var(--color-text-secondary); font-size: 0.875rem; margin: 1rem 0;">
                            "CSV files are generated after idle period detection."
                        </p>
                        <div class="badge secondary" style="font-size: 0.75rem;">
                            "CSV Ready"
                        </div>
                    </div>

                </div>

                // Prediction Results Card
                <div class="card card-elevated animate-fade-in" style="margin-top: 2rem; animation-delay: 400ms;">
                    <div class="asset-header" style="margin-bottom: 1.5rem;">"AI PREDICTION"</div>

                    <Show
                        when=move || prediction_loading.get()
                        fallback=move || view! {
                            <Show
                                when=move || prediction_result.get().is_some()
                                fallback=move || view! {
                                    <Show
                                        when=move || prediction_error.get().is_some()
                                        fallback=|| view! {
                                            <div style="text-align: center; padding: 2rem; color: var(--color-text-secondary);">
                                                <p>"Waiting for data to be processed..."</p>
                                                <p style="font-size: 0.875rem; margin-top: 0.5rem;">
                                                    "Predictions will appear here after data collection is complete."
                                                </p>
                                            </div>
                                        }
                                    >
                                        {move || {
                                            let error = prediction_error.get().unwrap_or_default();
                                            view! {
                                                <div style="padding: 1.5rem; background-color: rgba(239, 68, 68, 0.1); border-radius: 8px; border: 1px solid rgba(239, 68, 68, 0.3);">
                                                    <div style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 0.5rem;">
                                                        <span style="font-size: 1.5rem;">"⚠️"</span>
                                                        <span style="font-weight: 600; color: #ef4444;">"Prediction Error"</span>
                                                    </div>
                                                    <p style="color: var(--color-text-secondary); margin: 0;">
                                                        {error}
                                                    </p>
                                                </div>
                                            }
                                        }}
                                    </Show>
                                }
                            >
                                {move || {
                                    let result = prediction_result.get().unwrap();
                                    let probability_percent = (result.probability * 100.0).round();
                                    let confidence_display = result.confidence
                                        .map(|c| format!("{:.1}%", c * 100.0))
                                        .unwrap_or_else(|| "N/A".to_string());

                                    view! {
                                        <div style="text-align: center;">
                                            <div style="margin-bottom: 2rem;">
                                                <div style="font-size: 4rem; font-weight: 700; color: var(--color-text-primary); line-height: 1;">
                                                    {format!("{:.0}%", probability_percent)}
                                                </div>
                                                <div style="color: var(--color-text-secondary); font-size: 0.875rem; margin-top: 0.5rem;">
                                                    "Prediction Probability"
                                                </div>
                                            </div>

                                            <div class="asset-info">
                                                <div class="asset-info-row">
                                                    <span class="asset-label">"Confidence"</span>
                                                    <span class="asset-value">{confidence_display}</span>
                                                </div>
                                                <div class="asset-info-row">
                                                    <span class="asset-label">"Dataset ID"</span>
                                                    <span class="asset-value" style="font-family: monospace; font-size: 0.75rem;">
                                                        {result.dataset_id.chars().take(8).collect::<String>()}
                                                    </span>
                                                </div>
                                                <div class="asset-info-row">
                                                    <span class="asset-label">"Processed At"</span>
                                                    <span class="asset-value" style="font-size: 0.75rem;">
                                                        {result.processed_at.chars().take(19).collect::<String>()}
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }}
                            </Show>
                        }
                    >
                        <div style="text-align: center; padding: 2rem;">
                            <div class="animate-pulse" style="font-size: 3rem; margin-bottom: 1rem;">
                                "⏳"
                            </div>
                            <p style="color: var(--color-text-secondary); font-weight: 500;">
                                "Processing prediction..."
                            </p>
                            <p style="color: var(--color-text-secondary); font-size: 0.875rem; margin-top: 0.5rem;">
                                "This may take a moment. The API will retry up to 3 times if needed."
                            </p>
                        </div>
                    </Show>
                </div>

            </div>
        </div>
    }
}
