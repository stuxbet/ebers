use leptos::prelude::*;
use leptos::task::spawn_local;

mod components;
mod pages;
mod serial;

use pages::{HomePage, PredictionsPage, ResultsPage};
use serial::initialize_serial;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Results,
    Predictions,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(Page::Home);
    let (latest_serial, set_latest_serial) = signal(String::new());
    let (connected, set_connected) = signal(false);

    // Prediction state
    let (prediction_loading, set_prediction_loading) = signal(false);
    let (prediction_result, set_prediction_result) = signal(None::<serial::PredictionData>);
    let (prediction_error, set_prediction_error) = signal(None::<String>);

    // Initialize serial communication
    spawn_local(async move {
        initialize_serial(
            set_latest_serial,
            set_connected,
            set_prediction_loading,
            set_prediction_result,
            set_prediction_error,
        )
        .await;
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
                        prediction_loading=prediction_loading
                        prediction_result=prediction_result
                        prediction_error=prediction_error
                    />
                }.into_any(),
                Page::Predictions => view! {
                    <PredictionsPage
                        on_navigate_to_home=set_current_page
                    />
                }.into_any(),
            }}
        </main>
    }
}
