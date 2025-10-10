use leptos::prelude::*;
use leptos::task::spawn_local;

mod components;
mod pages;
mod serial;

use components::ProfileMenu;
use pages::{
    DetectionsPage, HomePage, PatientFormPage, ResultsPage, SettingsPage, TestReadingPage,
    TestResultsPage, TestStartPage,
};
use serial::initialize_serial;

#[derive(Clone, PartialEq)]
pub enum Page {
    TestStart,
    PatientForm,
    TestReading,
    TestResults,
    Home,
    Results,
    History, // Renamed from Detections
    Settings,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(Page::TestStart);
    let (latest_serial, set_latest_serial) = signal(String::new());
    let (connected, set_connected) = signal(false);

    // Detection state
    let (detection_loading, set_detection_loading) = signal(false);
    let (detection_result, set_detection_result) = signal(None::<serial::DetectionData>);
    let (detection_error, set_detection_error) = signal(None::<String>);

    // Initialize serial communication
    spawn_local(async move {
        initialize_serial(
            set_latest_serial,
            set_connected,
            set_detection_loading,
            set_detection_result,
            set_detection_error,
        )
        .await;
    });

    view! {
        <main class="container">
            // Profile menu in top right corner
            <div class="profile-menu-wrapper">
                <ProfileMenu on_navigate=set_current_page />
            </div>

            {move || match current_page.get() {
                Page::TestStart => view! {
                    <TestStartPage
                        on_navigate=set_current_page
                    />
                }.into_any(),
                Page::PatientForm => view! {
                    <PatientFormPage
                        on_navigate=set_current_page
                    />
                }.into_any(),
                Page::TestReading => view! {
                    <TestReadingPage
                        connected=connected
                        on_navigate=set_current_page
                        detection_loading=detection_loading
                        detection_result=detection_result
                        detection_error=detection_error
                    />
                }.into_any(),
                Page::TestResults => view! {
                    <TestResultsPage
                        on_navigate=set_current_page
                        detection_result=detection_result
                    />
                }.into_any(),
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
                        detection_loading=detection_loading
                        detection_result=detection_result
                        detection_error=detection_error
                    />
                }.into_any(),
                Page::History => view! {
                    <DetectionsPage
                        on_navigate_to_home=set_current_page
                    />
                }.into_any(),
                Page::Settings => view! {
                    <SettingsPage
                        on_navigate_to_home=set_current_page
                    />
                }.into_any(),
            }}
        </main>
    }
}
