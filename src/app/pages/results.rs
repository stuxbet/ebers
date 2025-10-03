use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn ResultsPage(
    latest_serial: ReadSignal<String>,
    on_navigate_to_home: WriteSignal<Page>,
) -> impl IntoView {
    view! {
        <div class="results-header">
            <h1>"Results"</h1>
            <button
                class="home-btn"
                on:click=move |_| on_navigate_to_home.set(Page::Home)
            >
                "← Home"
            </button>
        </div>

        <div class="results-content">
            <h2>"Serial Data"</h2>
            <div class="serial-data">
                <p><strong>"Latest Data:"</strong></p>
                <pre class="serial-output">{ move || latest_serial.get() }</pre>
            </div>

            <div class="results-info">
                <p>"CSV data aggregation and processing will be implemented in the backend."</p>
            </div>
        </div>
    }
}
