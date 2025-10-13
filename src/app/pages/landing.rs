use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn LandingPage(on_navigate: WriteSignal<Page>) -> impl IntoView {
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

            <div class="card" style="max-width: 600px; margin: 0 auto 2rem auto; text-align: center; padding: 3rem;">
                <h2 style="font-size: 1.75rem; font-weight: 400; margin-bottom: 1rem; color: var(--color-text-primary);">
                    "Ready to Begin Testing"
                </h2>
                <p style="color: var(--color-text-secondary); margin-bottom: 2rem; font-size: 1rem; line-height: 1.6;">
                    "Start a new test by entering patient information and connecting the test device."
                </p>
                <button
                    class="button primary"
                    on:click=on_begin_test
                    style="padding: 1rem 3rem; font-size: 1.125rem; font-weight: 500;"
                >
                    "Begin Test"
                </button>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-bottom: 2rem;">
                <div class="card stat-card">
                    <div style="text-align: center; padding: 1rem;">
                        <div style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin-bottom: 0.5rem;">
                            "0"
                        </div>
                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">
                            "Total Tests"
                        </div>
                    </div>
                </div>

                <div class="card stat-card">
                    <div style="text-align: center; padding: 1rem;">
                        <div style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin-bottom: 0.5rem;">
                            "0"
                        </div>
                        <div style="font-size: 0.875rem; color: var(--color-text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">
                            "Today's Tests"
                        </div>
                    </div>
                </div>

                <div class="card stat-card">
                    <div style="text-align: center; padding: 1rem;">
                        <div style="font-size: 2.5rem; font-weight: 300; color: var(--color-text-primary); margin-bottom: 0.5rem;">
                            "0"
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
                <div style="text-align: center; padding: 3rem; color: var(--color-text-secondary);">
                    <p style="margin: 0; font-size: 0.9375rem;">
                        "No tests yet. Click 'Begin Test' to get started."
                    </p>
                </div>
            </div>
        </div>
    }
}
