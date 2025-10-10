use crate::app::Page;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn SettingsPage(on_navigate_to_home: WriteSignal<Page>) -> impl IntoView {
    // State for port selection
    let (available_ports, set_available_ports) = signal(Vec::<String>::new());
    let (current_port, set_current_port) = signal(String::new());
    let (selected_port, set_selected_port) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(None::<String>);

    // Load available ports and current port on mount
    spawn_local(async move {
        // Get available ports
        match invoke("list_serial_ports", JsValue::NULL).await {
            Ok(ports_value) => {
                if let Ok(ports_vec) = serde_wasm_bindgen::from_value::<Vec<String>>(ports_value) {
                    set_available_ports.set(ports_vec);
                }
            }
            Err(e) => {
                leptos::logging::log!("Failed to list ports: {:?}", e);
            }
        }

        // Get current port
        match invoke("get_current_port", JsValue::NULL).await {
            Ok(port_value) => {
                if let Some(port_str) = port_value.as_string() {
                    set_current_port.set(port_str.clone());
                    set_selected_port.set(port_str);
                }
            }
            Err(e) => {
                leptos::logging::log!("Failed to get current port: {:?}", e);
            }
        }
    });

    // Handler to refresh port list
    let refresh_ports = move |_| {
        spawn_local(async move {
            match invoke("list_serial_ports", JsValue::NULL).await {
                Ok(ports_value) => {
                    if let Ok(ports_vec) =
                        serde_wasm_bindgen::from_value::<Vec<String>>(ports_value)
                    {
                        set_available_ports.set(ports_vec);
                        set_success.set(Some("Port list refreshed".to_string()));
                        // Clear success message after 3 seconds
                        set_timeout(
                            move || {
                                set_success.set(None);
                            },
                            std::time::Duration::from_secs(3),
                        );
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to refresh ports: {:?}", e)));
                }
            }
        });
    };

    // Handler to apply port change
    let apply_port_change = move |_| {
        let port = selected_port.get();
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            set_success.set(None);

            // Create a simple JS object for the port argument
            let args = js_sys::Object::new();
            js_sys::Reflect::set(&args, &JsValue::from_str("port"), &JsValue::from_str(&port))
                .unwrap();

            match invoke("set_serial_port", args.into()).await {
                Ok(_) => {
                    set_current_port.set(port.clone());
                    set_success.set(Some(format!(
                        "Port changed to {}. Please restart the application for changes to take effect.",
                        port
                    )));
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to change port: {:?}", e)));
                }
            }
            set_loading.set(false);
        });
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

                        // COM Port Selection
                        <div class="settings-item" style="flex-direction: column; align-items: stretch; gap: 1rem;">
                            <div style="display: flex; justify-content: space-between; align-items: center;">
                                <div>
                                    <div style="font-weight: 500; color: var(--color-text-primary);">
                                        "Serial Port"
                                    </div>
                                    <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                                        "Select the COM port for serial communication"
                                    </div>
                                </div>
                                <button
                                    class="button"
                                    on:click=refresh_ports
                                    style="padding: 0.5rem 0.75rem; font-size: 0.875rem;"
                                >
                                    "🔄 Refresh"
                                </button>
                            </div>

                            <div style="display: flex; gap: 0.75rem; align-items: center;">
                                <select
                                    class="port-selector"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_selected_port.set(value);
                                    }
                                    prop:value=move || selected_port.get()
                                    style="flex: 1; padding: 0.625rem; border: 1px solid var(--color-border-medium); border-radius: 6px; background: var(--color-surface); font-family: inherit; font-size: 0.875rem;"
                                >
                                    {move || {
                                        let ports = available_ports.get();
                                        if ports.is_empty() {
                                            let empty_value = String::new();
                                            let empty_text = "No ports available".to_string();
                                            vec![view! {
                                                <option value=empty_value>{empty_text}</option>
                                            }]
                                        } else {
                                            ports.into_iter().map(|port| {
                                                let port_value = port.clone();
                                                let port_text = port;
                                                view! {
                                                    <option value=port_value>{port_text}</option>
                                                }
                                            }).collect::<Vec<_>>()
                                        }
                                    }}
                                </select>

                                <button
                                    class="button primary"
                                    on:click=apply_port_change
                                    disabled=move || loading.get() || selected_port.get() == current_port.get() || selected_port.get().is_empty()
                                    style="padding: 0.625rem 1.25rem; font-size: 0.875rem; white-space: nowrap;"
                                >
                                    {move || if loading.get() { "Applying..." } else { "Apply" }}
                                </button>
                            </div>

                            {move || {
                                if let Some(msg) = success.get() {
                                    view! {
                                        <div style="padding: 0.75rem; background: #d4edda; border: 1px solid #c3e6cb; border-radius: 6px; color: #155724; font-size: 0.875rem;">
                                            {msg}
                                        </div>
                                    }.into_any()
                                } else if let Some(err) = error.get() {
                                    view! {
                                        <div style="padding: 0.75rem; background: #f8d7da; border: 1px solid #f5c6cb; border-radius: 6px; color: #721c24; font-size: 0.875rem;">
                                            {err}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}

                            {move || {
                                let current = current_port.get();
                                if !current.is_empty() {
                                    view! {
                                        <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                                            "Current port: " <strong>{current}</strong>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
                        </div>

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
