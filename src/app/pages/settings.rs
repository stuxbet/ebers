use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn SettingsPage(on_navigate_to_home: WriteSignal<Page>) -> impl IntoView {
    view! {
        <div class="animate-fade-in">
            // Header with back button
            <header style="margin-bottom: 2rem;">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1rem;">
                    <button
                        class="button"
                        on:click=move |_| on_navigate_to_home.set(Page::Home)
                        style="padding: 0.5rem 1rem;"
                    >
                        "← Back to Home"
                    </button>
                    <h1 style="margin: 0; font-size: 2rem; font-weight: 300;">
                        "Settings"
                    </h1>
                    <div style="width: 100px;"></div> // Spacer for centering
                </div>
            </header>

            // Settings Content
            <div class="card" style="max-width: 800px; margin: 0 auto;">
                <div style="padding: 1rem;">
                    <h2 style="font-size: 1.5rem; font-weight: 400; margin-bottom: 1.5rem; color: var(--color-text-primary);">
                        "Application Settings"
                    </h2>

                    // Settings sections
                    <div class="settings-section">
                        <h3 style="font-size: 1.125rem; font-weight: 500; margin-bottom: 1rem; color: var(--color-text-secondary);">
                            "General"
                        </h3>
                        <div class="settings-item">
                            <div>
                                <div style="font-weight: 500; color: var(--color-text-primary);">
                                    "Application Version"
                                </div>
                                <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                                    "v0.1.0"
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="settings-divider"></div>

                    <div class="settings-section">
                        <h3 style="font-size: 1.125rem; font-weight: 500; margin-bottom: 1rem; color: var(--color-text-secondary);">
                            "Serial Connection"
                        </h3>
                        <div class="settings-item">
                            <div>
                                <div style="font-weight: 500; color: var(--color-text-primary);">
                                    "Auto-reconnect"
                                </div>
                                <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                                    "Automatically reconnect to devices when disconnected"
                                </div>
                            </div>
                            <div class="toggle-placeholder" style="color: var(--color-success);">
                                "✓ Enabled"
                            </div>
                        </div>
                    </div>

                    <div class="settings-divider"></div>

                    <div class="settings-section">
                        <h3 style="font-size: 1.125rem; font-weight: 500; margin-bottom: 1rem; color: var(--color-text-secondary);">
                            "Data Export"
                        </h3>
                        <div class="settings-item">
                            <div>
                                <div style="font-weight: 500; color: var(--color-text-primary);">
                                    "CSV Format"
                                </div>
                                <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                                    "Export data in CSV format after idle period"
                                </div>
                            </div>
                            <div class="toggle-placeholder" style="color: var(--color-success);">
                                "✓ Enabled"
                            </div>
                        </div>
                    </div>

                    <div class="settings-divider"></div>

                    <div class="settings-section">
                        <h3 style="font-size: 1.125rem; font-weight: 500; margin-bottom: 1rem; color: var(--color-text-secondary);">
                            "About"
                        </h3>
                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); line-height: 1.6;">
                            <p>"Serial Data Platform - Real-time data acquisition and analysis"</p>
                            <p style="margin-top: 0.5rem;">"Built with Tauri and Leptos"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

