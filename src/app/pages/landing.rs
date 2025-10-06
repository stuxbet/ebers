use crate::app::components::{DeviceStatusCard, StatsDashboard};
use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn HomePage(
    connected: ReadSignal<bool>,
    on_navigate_to_results: WriteSignal<Page>,
) -> impl IntoView {
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
                    "Serial Data Platform"
                </h1>
                <p style="color: var(--color-text-secondary); margin-top: 0.75rem; font-size: 1.125rem;">
                    "Real-time data acquisition and analysis"
                </p>
            </header>



            // Device Status Card
            <DeviceStatusCard
                connected=connected
                on_navigate_to_results=on_navigate_to_results
            />

            // Stats Dashboard
            <StatsDashboard connected=connected />

            // Predictions History Button
            <div style="margin-top: 2rem; text-align: center;">
                <button
                    class="button"
                    on:click=move |_| on_navigate_to_results.set(Page::Predictions)
                    style="padding: 0.75rem 1.5rem; font-size: 1rem;"
                >
                    "📊 View Prediction History"
                </button>
            </div>

            // Footer Info
            <div class="results-info" style="margin-top: 3rem; text-align: center;">
                <p>"Powered by Vertec Colossus "</p>
            </div>
        </div>
    }
}
