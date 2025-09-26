use js_sys::Function;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::console;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Function) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    //this is just for displaying current serial data. will not be in use in production. in future just the csv will be sent from the back end for display purposes only.
    let (latest_serial, set_latest_serial) = signal(String::new());
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
        let _unlisten = listen("serial:data", serial_event_handler.as_ref().unchecked_ref()).await;
        serial_event_handler.forget();
    });

    view! {
        <main class="container">
            <h1>"Welcome Foothold Labs!"</h1>

            <div class="row">
                <a href="https://footholdlabs.com/" target="_blank">
                    <img src="https://media.licdn.com/dms/image/v2/C4E0BAQGTcebEYz_Hvg/company-logo_200_200/company-logo_200_200/0/1630569533433/foothold_labs_logo?e=1761782400&v=beta&t=6psnH45OQow8ZyMB9rjFBets4mI8M9KG5C7c8NEYnJs" class="logo tauri" alt="Foothold Labs logo"/>
                </a>

            </div>

            <p><strong>Latest serial:</strong> { move || latest_serial.get() }</p>

        </main>
    }
}
