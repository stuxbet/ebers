use js_sys::Function;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::console;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{prelude::*, JsCast};

mod pages;
use pages::{HomePage, ResultsPage};

#[derive(Clone, PartialEq)]
pub enum Page {
    Home,
    Results,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Function) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(Page::Home);
    let (latest_serial, set_latest_serial) = signal(String::new());
    let (connected, set_connected) = signal(false);

    spawn_local(async move {
        let _ = invoke("start_serial", JsValue::NULL).await;

        let update_latest_serial = set_latest_serial;
        let serial_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
                if let Some(s) = payload.as_string() {
                    update_latest_serial.set(s.clone());
                    console::log_1(&JsValue::from_str(&format!("serial:data: {}", s)));
                } else {
                    console::log_1(&payload);
                }
            } else {
                console::log_1(&JsValue::from_str("serial:data: <no payload>"));
            }
        }) as Box<dyn FnMut(JsValue)>);

        let update_connected = set_connected;
        let status_event_handler = Closure::wrap(Box::new(move |event: JsValue| {
            if let Ok(payload) = js_sys::Reflect::get(&event, &JsValue::from_str("payload")) {
                // Try payload.connected boolean first
                if let Ok(val) = js_sys::Reflect::get(&payload, &JsValue::from_str("connected")) {
                    if let Some(b) = val.as_bool() {
                        update_connected.set(b);
                        console::log_1(&JsValue::from_str(&format!(
                            "serial:status connected={}",
                            b
                        )));
                    }
                } else if let Some(b) = payload.as_bool() {
                    // Or a bare boolean payload
                    update_connected.set(b);
                    console::log_1(&JsValue::from_str(&format!(
                        "serial:status connected={} (bool)",
                        b
                    )));
                } else {
                    console::log_1(&payload);
                }
            } else {
                console::log_1(&JsValue::from_str("serial:status: <no payload>"));
            }
        }) as Box<dyn FnMut(JsValue)>);
        let _unlisten_status = listen(
            "serial:status",
            status_event_handler.as_ref().unchecked_ref(),
        )
        .await;
        status_event_handler.forget();

        let _unlisten = listen("serial:data", serial_event_handler.as_ref().unchecked_ref()).await;
        serial_event_handler.forget();
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
