use crate::app::Page;
use leptos::prelude::*;

#[component]
pub fn ProfileMenu(on_navigate: WriteSignal<Page>) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Close dropdown when clicking outside
    let toggle_dropdown = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    let navigate_to_settings = move |_| {
        on_navigate.set(Page::Settings);
        set_is_open.set(false);
    };

    view! {
        <div class="profile-menu-container">
            <button
                class="profile-button"
                on:click=toggle_dropdown
                aria-label="Profile menu"
            >
                // Default profile icon (user silhouette)
                <svg
                    width="32"
                    height="32"
                    viewBox="0 0 32 32"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <circle
                        cx="16"
                        cy="16"
                        r="15"
                        fill="var(--color-bg-tertiary)"
                        stroke="var(--color-border-medium)"
                        stroke-width="2"
                    />
                    <circle
                        cx="16"
                        cy="12"
                        r="5"
                        fill="var(--color-text-secondary)"
                    />
                    <path
                        d="M6 26C6 21 10 18 16 18C22 18 26 21 26 26"
                        fill="var(--color-text-secondary)"
                    />
                </svg>
            </button>

            {move || {
                if is_open.get() {
                    view! {
                        <div class="profile-dropdown">
                            <button
                                class="dropdown-item"
                                on:click=navigate_to_settings
                            >
                                <span>"Settings"</span>
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
