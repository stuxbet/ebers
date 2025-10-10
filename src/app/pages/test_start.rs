use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn TestStartPage(
    on_navigate: WriteSignal<Page>,
) -> impl IntoView {
    let on_begin_test = move |_| {
        on_navigate.set(Page::PatientForm);
    };

    let on_view_history = move |_| {
        on_navigate.set(Page::History);
    };

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

            // Main Action Card
            <div class="card" style="max-width: 600px; margin: 0 auto 2rem auto; text-align: center; padding: 3rem;">
                <div style="font-size: 4rem; margin-bottom: 1.5rem;">
                    "🔬"
                </div>
                <h2 style="font-size: 1.75rem; font-weight: 400; margin-bottom: 1rem; color: var(--color-text-primary);">
                    "Ready to Begin Testing"
                </h2>
                <p style="color: var(--color-text-secondary); margin-bottom: 2rem; font-size: 1rem; line-height: 1.6;">
                    "Start a new test by entering patient information and connecting the test device."
                </p>
                <button
                    class="button"
                    on:click=on_begin_test
                    style="padding: 1rem 3rem; font-size: 1.125rem; background-color: var(--color-accent-primary); color: white; font-weight: 500; box-shadow: var(--shadow-md);"
                >
                    "Begin Test →"
                </button>
            </div>

            // Quick Stats Dashboard
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-bottom: 2rem;">
                // Total Tests Card
                <div class="card stat-card">
                    <div style="display: flex; align-items: center; gap: 1rem;">
                        <div style="font-size: 2.5rem;">
                            "📊"
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Total Tests"
                            </div>
                            <div style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary);">
                                "0" // TODO: Get from database
                            </div>
                        </div>
                    </div>
                </div>

                // Today's Tests Card
                <div class="card stat-card">
                    <div style="display: flex; align-items: center; gap: 1rem;">
                        <div style="font-size: 2.5rem;">
                            "📅"
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Today's Tests"
                            </div>
                            <div style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary);">
                                "0" // TODO: Get from database
                            </div>
                        </div>
                    </div>
                </div>

                // Patients Card
                <div class="card stat-card">
                    <div style="display: flex; align-items: center; gap: 1rem;">
                        <div style="font-size: 2.5rem;">
                            "👥"
                        </div>
                        <div>
                            <div style="font-size: 0.875rem; color: var(--color-text-secondary); margin-bottom: 0.25rem;">
                                "Total Patients"
                            </div>
                            <div style="font-size: 2rem; font-weight: 300; color: var(--color-text-primary);">
                                "0" // TODO: Get from database
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Recent Tests Section
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

                // Placeholder for recent tests
                <div style="text-align: center; padding: 3rem; color: var(--color-text-secondary);">
                    <div style="font-size: 3rem; margin-bottom: 1rem;">
                        "📋"
                    </div>
                    <p style="margin: 0;">
                        "No tests yet. Click 'Begin Test' to get started."
                    </p>
                </div>
            </div>

            // Quick Actions
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem;">
                <button
                    class="card"
                    on:click=on_view_history
                    style="cursor: pointer; text-align: left; padding: 1.5rem; border: 1px solid var(--color-border-light); background: var(--color-surface); transition: all 0.2s ease;"
                >
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">
                        "📊"
                    </div>
                    <div style="font-size: 1rem; font-weight: 500; margin-bottom: 0.25rem; color: var(--color-text-primary);">
                        "View Test History"
                    </div>
                    <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                        "Browse all previous tests and results"
                    </div>
                </button>

                <button
                    class="card"
                    on:click=move |_| on_navigate.set(Page::Settings)
                    style="cursor: pointer; text-align: left; padding: 1.5rem; border: 1px solid var(--color-border-light); background: var(--color-surface); transition: all 0.2s ease;"
                >
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">
                        "⚙️"
                    </div>
                    <div style="font-size: 1rem; font-weight: 500; margin-bottom: 0.25rem; color: var(--color-text-primary);">
                        "Settings"
                    </div>
                    <div style="font-size: 0.875rem; color: var(--color-text-secondary);">
                        "Configure device and application settings"
                    </div>
                </button>
            </div>
        </div>
    }
}

