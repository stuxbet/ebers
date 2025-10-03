use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn ResultsPage(
    latest_serial: ReadSignal<String>,
    on_navigate_to_home: WriteSignal<Page>,
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


            </div>
        </div>
    }
}
