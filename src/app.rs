use leptos::prelude::*;
use leptos::task::spawn_local;

mod components;
mod pages;
mod serial;

use components::ProfileMenu;
use pages::{
    DetectionsPage, LandingPage, PatientFormPage, SettingsPage, TestReadingPage, TestResultsPage,
};
use serial::initialize_serial;

#[derive(Clone, PartialEq)]
pub enum Page {
    Landing,
    PatientForm,
    TestReading,
    TestResults,
    History,
    Settings,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(Page::Landing);
    let (_latest_serial, set_latest_serial) = signal(String::new());
    let (connected, set_connected) = signal(false);

    // Detection state
    let (detection_loading, set_detection_loading) = signal(false);
    let (detection_result, set_detection_result) = signal(None::<serial::DetectionData>);
    let (detection_error, set_detection_error) = signal(None::<String>);

    // Current test UUID (set when creating a test, used to save results)
    let (current_test_uuid, set_current_test_uuid) = signal(None::<String>);

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
                Page::Landing => view! {
                    <LandingPage
                        on_navigate=set_current_page
                    />
                }.into_any(),
                Page::PatientForm => view! {
                    <PatientFormPage
                        on_navigate=set_current_page
                        set_current_test_uuid=set_current_test_uuid
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
                        current_test_uuid=current_test_uuid
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
