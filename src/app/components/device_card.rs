use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn DeviceStatusCard(
    connected: ReadSignal<bool>,
    on_navigate_to_results: WriteSignal<Page>,
) -> impl IntoView {
    view! {
        <div class="card card-elevated animate-fade-in" style="max-width: 400px; margin: 2rem auto;">
            <div class="asset-header">"DEVICE STATUS"</div>

            <div class="asset-info">
                <div class="asset-info-row">
                    <span class="asset-label">"Connection"</span>
                    <span class={move || if connected.get() { "status-connected" } else { "status-disconnected" }}>
                        {move || if connected.get() { "Connected" } else { "Disconnected" }}
                    </span>
                </div>
                <div class="asset-info-row">
                    <span class="asset-label">"Port"</span>
                    <span class="asset-value">{move || if connected.get() { "Auto-detected" } else { "—" }}</span>
                </div>
                <div class="asset-info-row">
                    <span class="asset-label">"Protocol"</span>
                    <span class="asset-value">{move || if connected.get() { "Serial USB" } else { "—" }}</span>
                </div>
                <div class="asset-info-row">
                    <span class="asset-label">"Status"</span>
                    <span class="asset-value">
                        {move || if connected.get() { "Ready" } else { "Waiting..." }}
                    </span>
                </div>
            </div>

            <Show when=move || connected.get()>
                <button
                    class="primary read-results-btn"
                    on:click=move |_| on_navigate_to_results.set(Page::Results)
                    style="width: 100%; margin-top: 1.5rem;"
                >
                    "Read Results →"
                </button>
            </Show>

            <Show when=move || !connected.get()>
                <div class="connection-message" style="margin-top: 1.5rem; text-align: center; padding: 1rem; background-color: rgba(125, 211, 192, 0.1); border-radius: 8px; border-left: 4px solid var(--color-accent-primary);">
                    "⚡ Waiting for device connection..."
                </div>
            </Show>
        </div>
    }
}

