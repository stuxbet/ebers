use leptos::prelude::*;
use leptos::task::spawn_local;

mod components;
mod pages;
mod serial;

use pages::{HomePage, ResultsPage};
use serial::initialize_serial;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Results,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(Page::Home);
    let (latest_serial, set_latest_serial) = signal(String::new());
    let (connected, set_connected) = signal(false);

    // Initialize serial communication
    spawn_local(async move {
        initialize_serial(set_latest_serial, set_connected).await;
    });

    view! {
        <main class="container">
            {move || match current_page.get() {
                Page::Home => view! {
                    <HomePage
                        connected=connected
                        on_navigate_to_results=set_current_page
                    />
                }.into_any(),
                Page::Results => view! {
                    <ResultsPage
                        latest_serial=latest_serial
                        on_navigate_to_home=set_current_page
                    />
                }.into_any(),
            }}
        </main>
    }
}
