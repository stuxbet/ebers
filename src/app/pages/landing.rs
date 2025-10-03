use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn HomePage(
    connected: ReadSignal<bool>,
    on_navigate_to_results: WriteSignal<Page>,
) -> impl IntoView {
    view! {
        <h1>"Welcome Foothold Labs!"</h1>

        <div class="row">
            <a href="https://footholdlabs.com/" target="_blank">
                <img src="https://media.licdn.com/dms/image/v2/C4E0BAQGTcebEYz_Hvg/company-logo_200_200/company-logo_200_200/0/1630569533433/foothold_labs_logo?e=1761782400&v=beta&t=6psnH45OQow8ZyMB9rjFBets4mI8M9KG5C7c8NEYnJs" class="logo tauri" alt="Foothold Labs logo"/>
            </a>
        </div>

        <div class="device-status">
            <h2>"Device Status"</h2>
            <p class="status-text">
                <strong>"Device: "</strong>
                <span class={move || if connected.get() { "status-connected" } else { "status-disconnected" }}>
                    { move || if connected.get() { "Connected" } else { "Disconnected" } }
                </span>
            </p>

            <Show when=move || connected.get()>
                <button
                    class="read-results-btn"
                    on:click=move |_| on_navigate_to_results.set(Page::Results)
                >
                    "Read Results"
                </button>
            </Show>

            <Show when=move || !connected.get()>
                <p class="connection-message">"Please connect a device to read results."</p>
            </Show>
        </div>
    }
}
