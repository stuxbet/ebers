use crate::app::Page;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePatientArgs {
    patient_data: CreatePatientRequest,
}

#[derive(Serialize, Deserialize)]
struct CreatePatientRequest {
    first_name: String,
    last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patient_id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Patient {
    uuid: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateTestArgs {
    test_data: CreateTestRequest,
}

#[derive(Serialize, Deserialize)]
struct CreateTestRequest {
    patient_uuid: String,
    test_type: String,
    device_id: Option<String>,
    firmware_version: Option<String>,
}

#[component]
pub fn PatientFormPage(
    on_navigate: WriteSignal<Page>,
    set_current_test_uuid: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Form state
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (date_of_birth, set_date_of_birth) = signal(String::new());
    let (patient_id_number, set_patient_id_number) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (test_type, set_test_type) = signal(String::from("covid-19"));

    let (submitting, set_submitting) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Handle form submission
    let on_submit = move |_| {
        let first = first_name.get();
        let last = last_name.get();
        let dob = date_of_birth.get();
        let pid = patient_id_number.get();
        let em = email.get();
        let ph = phone.get();
        let nt = notes.get();
        let tt = test_type.get();

        leptos::task::spawn_local(async move {
            use leptos::web_sys::console;

            set_submitting.set(true);
            set_error.set(None);

            // Create patient
            let patient_request = CreatePatientRequest {
                first_name: first.clone(),
                last_name: last.clone(),
                date_of_birth: if dob.is_empty() {
                    None
                } else {
                    Some(dob.clone())
                },
                patient_id_number: if pid.is_empty() {
                    None
                } else {
                    Some(pid.clone())
                },
                email: if em.is_empty() {
                    None
                } else {
                    Some(em.clone())
                },
                phone: if ph.is_empty() {
                    None
                } else {
                    Some(ph.clone())
                },
                notes: if nt.is_empty() {
                    None
                } else {
                    Some(nt.clone())
                },
            };

            console::log_1(&JsValue::from_str("Creating patient..."));

            let args = CreatePatientArgs {
                patient_data: patient_request,
            };

            match invoke(
                "create_patient",
                serde_wasm_bindgen::to_value(&args).unwrap(),
            )
            .await
            {
                Ok(patient_result) => {
                    console::log_1(&JsValue::from_str("Patient created successfully"));

                    // Parse patient to get UUID
                    match serde_wasm_bindgen::from_value::<Patient>(patient_result) {
                        Ok(patient) => {
                            console::log_1(&JsValue::from_str(&format!(
                                "Patient UUID: {}",
                                patient.uuid
                            )));

                            // Create test
                            let test_request = CreateTestRequest {
                                patient_uuid: patient.uuid,
                                test_type: tt.clone(),
                                device_id: None,
                                firmware_version: None,
                            };

                            console::log_1(&JsValue::from_str("Creating test..."));

                            let test_args = CreateTestArgs {
                                test_data: test_request,
                            };

                            match invoke(
                                "create_test",
                                serde_wasm_bindgen::to_value(&test_args).unwrap(),
                            )
                            .await
                            {
                                Ok(result) => {
                                    // Extract the test UUID from the result
                                    if let Ok(test_uuid) =
                                        js_sys::Reflect::get(&result, &JsValue::from_str("uuid"))
                                    {
                                        // Try as string first, fallback to JSON stringify for Uuid objects
                                        let uuid_str = if let Some(s) = test_uuid.as_string() {
                                            Some(s)
                                        } else {
                                            js_sys::JSON::stringify(&test_uuid)
                                                .ok()
                                                .and_then(|s| s.as_string())
                                                .map(|s| s.trim_matches('"').to_string())
                                        };

                                        if let Some(uuid_str) = uuid_str {
                                            set_current_test_uuid.set(Some(uuid_str));
                                        }
                                    }

                                    set_submitting.set(false);
                                    on_navigate.set(Page::TestReading);
                                }
                                Err(e) => {
                                    let error_msg = format!("Failed to create test: {:?}", e);
                                    console::log_1(&JsValue::from_str(&error_msg));
                                    set_error.set(Some(error_msg));
                                    set_submitting.set(false);
                                }
                            }
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to parse patient: {:?}", e);
                            console::log_1(&JsValue::from_str(&error_msg));
                            set_error.set(Some(error_msg));
                            set_submitting.set(false);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to create patient: {:?}", e);
                    console::log_1(&JsValue::from_str(&error_msg));
                    set_error.set(Some(error_msg));
                    set_submitting.set(false);
                }
            }
        });
    };

    let on_cancel = move |_| {
        on_navigate.set(Page::Landing);
    };

    view! {
        <div class="animate-fade-in">
            // Header
            <header style="margin-bottom: 2rem;">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1rem;">
                    <button
                        class="button"
                        on:click=on_cancel
                        style="padding: 0.5rem 1rem;"
                    >
                        "← Cancel"
                    </button>
                    <h1 style="margin: 0; font-size: 2rem; font-weight: 300;">
                        "New Test - Patient Information"
                    </h1>
                    <div style="width: 100px;"></div> // Spacer for centering
                </div>
                <p style="text-align: center; color: var(--color-text-secondary); margin: 0;">
                    "Enter patient details and select test type"
                </p>
            </header>

            // Form Card
            <div class="card" style="max-width: 800px; margin: 0 auto;">
                <form on:submit=move |e| {
                    e.prevent_default();
                    on_submit(());
                }>
                    // Patient Information Section
                    <div style="margin-bottom: 2rem;">
                        <h2 style="font-size: 1.25rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary); border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.5rem;">
                            "Patient Information"
                        </h2>

                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;">
                            // First Name
                            <div class="form-group">
                                <label class="form-label">
                                    "First Name "
                                    <span style="color: var(--color-error);">"*"</span>
                                </label>
                                <input
                                    type="text"
                                    class="form-input"
                                    placeholder="Enter first name"
                                    required
                                    prop:value=move || first_name.get()
                                    on:input=move |e| set_first_name.set(event_target_value(&e))
                                />
                            </div>

                            // Last Name
                            <div class="form-group">
                                <label class="form-label">
                                    "Last Name "
                                    <span style="color: var(--color-error);">"*"</span>
                                </label>
                                <input
                                    type="text"
                                    class="form-input"
                                    placeholder="Enter last name"
                                    required
                                    prop:value=move || last_name.get()
                                    on:input=move |e| set_last_name.set(event_target_value(&e))
                                />
                            </div>

                            // Date of Birth
                            <div class="form-group">
                                <label class="form-label">
                                    "Date of Birth"
                                </label>
                                <input
                                    type="date"
                                    class="form-input"
                                    prop:value=move || date_of_birth.get()
                                    on:input=move |e| set_date_of_birth.set(event_target_value(&e))
                                />
                            </div>

                            // Patient ID Number
                            <div class="form-group">
                                <label class="form-label">
                                    "Patient ID Number"
                                </label>
                                <input
                                    type="text"
                                    class="form-input"
                                    placeholder="Optional ID number"
                                    prop:value=move || patient_id_number.get()
                                    on:input=move |e| set_patient_id_number.set(event_target_value(&e))
                                />
                            </div>

                            // Email
                            <div class="form-group">
                                <label class="form-label">
                                    "Email"
                                </label>
                                <input
                                    type="email"
                                    class="form-input"
                                    placeholder="patient@example.com"
                                    prop:value=move || email.get()
                                    on:input=move |e| set_email.set(event_target_value(&e))
                                />
                            </div>

                            // Phone
                            <div class="form-group">
                                <label class="form-label">
                                    "Phone"
                                </label>
                                <input
                                    type="tel"
                                    class="form-input"
                                    placeholder="(555) 123-4567"
                                    prop:value=move || phone.get()
                                    on:input=move |e| set_phone.set(event_target_value(&e))
                                />
                            </div>
                        </div>

                        // Notes (full width)
                        <div class="form-group" style="margin-top: 1.5rem;">
                            <label class="form-label">
                                "Notes"
                            </label>
                            <textarea
                                class="form-input"
                                placeholder="Additional notes or observations..."
                                rows="3"
                                prop:value=move || notes.get()
                                on:input=move |e| set_notes.set(event_target_value(&e))
                                style="resize: vertical; font-family: inherit;"
                            ></textarea>
                        </div>
                    </div>

                    // Test Type Section
                    <div style="margin-bottom: 2rem;">
                        <h2 style="font-size: 1.25rem; font-weight: 500; margin-bottom: 1.5rem; color: var(--color-text-secondary); border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.5rem;">
                            "Test Information"
                        </h2>

                        <div class="form-group">
                            <label class="form-label">
                                "Test Type "
                                <span style="color: var(--color-error);">"*"</span>
                            </label>
                            <select
                                class="form-input"
                                required
                                prop:value=move || test_type.get()
                                on:change=move |e| set_test_type.set(event_target_value(&e))
                            >
                                <option value="covid-19">"COVID-19"</option>
                                <option value="influenza-ab">"Influenza A/B"</option>
                                <option value="strep-a">"Strep A"</option>
                                <option value="rsv">"RSV (Respiratory Syncytial Virus)"</option>
                                <option value="malaria">"Malaria"</option>
                                <option value="hiv">"HIV"</option>
                                <option value="hepatitis">"Hepatitis"</option>
                            </select>
                        </div>
                    </div>

                    // Error Message
                    {move || error.get().map(|err| view! {
                        <div style="padding: 1rem; background-color: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.3); border-radius: 8px; color: rgb(239, 68, 68); margin-bottom: 1rem;">
                            {err}
                        </div>
                    })}

                    // Action Buttons
                    <div style="display: flex; gap: 1rem; justify-content: flex-end; padding-top: 1rem; border-top: 1px solid var(--color-border-light);">
                        <button
                            type="button"
                            class="button"
                            on:click=on_cancel
                            disabled=move || submitting.get()
                            style="padding: 0.75rem 1.5rem; background-color: var(--color-bg-tertiary); color: var(--color-text-primary);"
                        >
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="button"
                            disabled=move || submitting.get()
                            style="padding: 0.75rem 2rem; background-color: var(--color-accent-primary); color: white; font-weight: 500;"
                        >
                            {move || if submitting.get() {
                                "Creating..."
                            } else {
                                "Start Test →"
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
