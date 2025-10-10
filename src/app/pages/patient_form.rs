use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn PatientFormPage(
    on_navigate: WriteSignal<Page>,
) -> impl IntoView {
    // Form state
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (date_of_birth, set_date_of_birth) = signal(String::new());
    let (patient_id_number, set_patient_id_number) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (test_type, set_test_type) = signal(String::from("COVID-19"));

    // Handle form submission
    let on_submit = move |_| {
        // TODO: Create patient and test records in database
        // For now, just navigate to test reading page
        on_navigate.set(Page::TestReading);
    };

    let on_cancel = move |_| {
        on_navigate.set(Page::TestStart);
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
                                <option value="COVID-19">"COVID-19"</option>
                                <option value="Influenza A/B">"Influenza A/B"</option>
                                <option value="Strep A">"Strep A"</option>
                                <option value="RSV">"RSV (Respiratory Syncytial Virus)"</option>
                                <option value="Malaria">"Malaria"</option>
                                <option value="HIV">"HIV"</option>
                                <option value="Hepatitis">"Hepatitis"</option>
                                <option value="Other">"Other"</option>
                            </select>
                        </div>
                    </div>

                    // Action Buttons
                    <div style="display: flex; gap: 1rem; justify-content: flex-end; padding-top: 1rem; border-top: 1px solid var(--color-border-light);">
                        <button
                            type="button"
                            class="button"
                            on:click=on_cancel
                            style="padding: 0.75rem 1.5rem; background-color: var(--color-bg-tertiary); color: var(--color-text-primary);"
                        >
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="button"
                            style="padding: 0.75rem 2rem; background-color: var(--color-accent-primary); color: white; font-weight: 500;"
                        >
                            "Start Test →"
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

