use leptos::prelude::*;

#[component]
pub fn PlatformGrid() -> impl IntoView {
    let platforms = vec![
        ("🔌", "Serial USB"),
        ("📡", "Auto-Detect"),
        ("💾", "CSV Export"),
        ("⚡", "Real-time"),
    ];

    view! {
        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 1rem; margin: 2rem 0;">
            {platforms.into_iter().enumerate().map(|(i, (icon, name))| {
                view! {
                    <div
                        class="platform-card animate-fade-in"
                        style=format!("animation-delay: {}ms", i * 100)
                    >
                        <div class="platform-icon" style="font-size: 2.5rem;">
                            {icon}
                        </div>
                        <div class="platform-name">{name}</div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

