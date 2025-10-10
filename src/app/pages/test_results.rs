use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn TestResultsPage(
    on_navigate: WriteSignal<Page>,
    detection_result: ReadSignal<Option<crate::app::serial::DetectionData>>,
) -> impl IntoView {
    let on_new_test = move |_| {
        on_navigate.set(Page::TestStart);
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
                            "📊 View History"
                        </button>
                        <button
                            class="button"
                            on:click=on_new_test
                            style="padding: 0.5rem 1rem; background-color: var(--color-accent-primary); color: white;"
                        >
                            "➕ New Test"
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
                                    // Result Status Banner
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
                                        <div style="font-size: 3rem; margin-bottom: 0.5rem;">
                                            {if result.probability >= 0.7 {
                                                "🔴"
                                            } else if result.probability >= 0.3 {
                                                "🟡"
                                            } else {
                                                "🟢"
                                            }}
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
                                    <div style="font-size: 3rem; margin-bottom: 1rem;">"📊"</div>
                                    <p>"No detection results available"</p>
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
                        "📊 View All Tests"
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
