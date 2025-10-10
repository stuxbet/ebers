use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn TestReadingPage(
    connected: ReadSignal<bool>,
    on_navigate: WriteSignal<Page>,
    detection_loading: ReadSignal<bool>,
    detection_result: ReadSignal<Option<crate::app::serial::DetectionData>>,
    detection_error: ReadSignal<Option<String>>,
) -> impl IntoView {
    // Auto-navigate to results when detection is complete
    Effect::new(move || {
        if detection_result.get().is_some() {
            // Detection completed successfully
            on_navigate.set(Page::TestResults);
        }
    });

    let on_cancel = move |_| {
        // TODO: Cancel the test and clean up
        on_navigate.set(Page::TestStart);
    };

    view! {
        <div class="animate-fade-in" style="display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 80vh;">
            // Main Loading Card
            <div class="card" style="max-width: 600px; text-align: center; padding: 3rem;">
                // Status Icon/Animation
                <div style="margin-bottom: 2rem;">
                    {move || {
                        if detection_error.get().is_some() {
                            view! {
                                <div style="font-size: 4rem; color: var(--color-error);">
                                    "⚠️"
                                </div>
                            }.into_any()
                        } else if detection_loading.get() {
                            view! {
                                <div style="font-size: 4rem; color: var(--color-accent-primary);">
                                    "🔬"
                                </div>
                            }.into_any()
                        } else if connected.get() {
                            view! {
                                <div class="spinner-large"></div>
                            }.into_any()
                        } else {
                            view! {
                                <div style="font-size: 4rem; color: var(--color-text-tertiary);">
                                    "🔌"
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                // Status Title
                <h1 style="font-size: 2rem; font-weight: 300; margin-bottom: 1rem; color: var(--color-text-primary);">
                    {move || {
                        if detection_error.get().is_some() {
                            "Test Error"
                        } else if detection_loading.get() {
                            "Analyzing Data..."
                        } else if connected.get() {
                            "Reading Device Data..."
                        } else {
                            "Waiting for Device..."
                        }
                    }}
                </h1>

                // Status Description
                <p style="color: var(--color-text-secondary); font-size: 1.125rem; margin-bottom: 2rem; line-height: 1.6;">
                    {move || {
                        if let Some(error) = detection_error.get() {
                            error
                        } else if detection_loading.get() {
                            "Sending data to detection API and processing results...".to_string()
                        } else if connected.get() {
                            "Collecting serial data from the device. Please wait...".to_string()
                        } else {
                            "Please connect the test device to begin data collection.".to_string()
                        }
                    }}
                </p>

                // Progress Steps
                <div style="margin: 2rem 0;">
                    <div style="display: flex; flex-direction: column; gap: 1rem; text-align: left; max-width: 400px; margin: 0 auto;">
                        // Step 1: Device Connection
                        <div style="display: flex; align-items: center; gap: 1rem;">
                            <div style=move || format!(
                                "width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 500; {}",
                                if connected.get() {
                                    "background-color: var(--color-success); color: white;"
                                } else {
                                    "background-color: var(--color-bg-tertiary); color: var(--color-text-secondary);"
                                }
                            )>
                                {move || if connected.get() { "✓" } else { "1" }}
                            </div>
                            <span style=move || format!(
                                "{}",
                                if connected.get() {
                                    "color: var(--color-text-primary); font-weight: 500;"
                                } else {
                                    "color: var(--color-text-secondary);"
                                }
                            )>
                                "Device Connected"
                            </span>
                        </div>

                        // Step 2: Data Collection
                        <div style="display: flex; align-items: center; gap: 1rem;">
                            <div style=move || format!(
                                "width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 500; {}",
                                if detection_loading.get() || detection_result.get().is_some() {
                                    "background-color: var(--color-success); color: white;"
                                } else if connected.get() {
                                    "background-color: var(--color-accent-primary); color: white;"
                                } else {
                                    "background-color: var(--color-bg-tertiary); color: var(--color-text-secondary);"
                                }
                            )>
                                {move || {
                                    if detection_loading.get() || detection_result.get().is_some() {
                                        "✓"
                                    } else {
                                        "2"
                                    }
                                }}
                            </div>
                            <span style=move || format!(
                                "{}",
                                if detection_loading.get() || detection_result.get().is_some() {
                                    "color: var(--color-text-primary); font-weight: 500;"
                                } else if connected.get() {
                                    "color: var(--color-text-primary);"
                                } else {
                                    "color: var(--color-text-secondary);"
                                }
                            )>
                                "Data Collection Complete"
                            </span>
                        </div>

                        // Step 3: Analysis
                        <div style="display: flex; align-items: center; gap: 1rem;">
                            <div style=move || format!(
                                "width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 500; {}",
                                if detection_result.get().is_some() {
                                    "background-color: var(--color-success); color: white;"
                                } else if detection_loading.get() {
                                    "background-color: var(--color-accent-primary); color: white;"
                                } else {
                                    "background-color: var(--color-bg-tertiary); color: var(--color-text-secondary);"
                                }
                            )>
                                {move || {
                                    if detection_result.get().is_some() {
                                        "✓"
                                    } else {
                                        "3"
                                    }
                                }}
                            </div>
                            <span style=move || format!(
                                "{}",
                                if detection_result.get().is_some() {
                                    "color: var(--color-text-primary); font-weight: 500;"
                                } else if detection_loading.get() {
                                    "color: var(--color-text-primary);"
                                } else {
                                    "color: var(--color-text-secondary);"
                                }
                            )>
                                "Analysis Complete"
                            </span>
                        </div>
                    </div>
                </div>

                // Loading Spinner (when actively processing)
                {move || {
                    if (connected.get() && !detection_loading.get() && detection_result.get().is_none() && detection_error.get().is_none()) 
                        || detection_loading.get() {
                        view! {
                            <div style="margin: 2rem 0;">
                                <div class="spinner"></div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}

                // Cancel Button
                <div style="margin-top: 2rem; padding-top: 2rem; border-top: 1px solid var(--color-border-light);">
                    <button
                        class="button"
                        on:click=on_cancel
                        style="padding: 0.75rem 1.5rem; background-color: var(--color-bg-tertiary); color: var(--color-text-primary);"
                    >
                        {move || {
                            if detection_error.get().is_some() {
                                "← Back to Home"
                            } else {
                                "Cancel Test"
                            }
                        }}
                    </button>
                </div>
            </div>

            // Device Status Footer
            <div style="margin-top: 2rem; text-align: center; color: var(--color-text-tertiary); font-size: 0.875rem;">
                {move || {
                    if connected.get() {
                        view! {
                            <div style="display: flex; align-items: center; gap: 0.5rem; justify-content: center;">
                                <div style="width: 8px; height: 8px; border-radius: 50%; background-color: var(--color-success);"></div>
                                <span>"Device connected and streaming data"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div style="display: flex; align-items: center; gap: 0.5rem; justify-content: center;">
                                <div style="width: 8px; height: 8px; border-radius: 50%; background-color: var(--color-text-tertiary);"></div>
                                <span>"Waiting for device connection..."</span>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

