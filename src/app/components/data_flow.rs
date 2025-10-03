use leptos::prelude::*;

#[component]
pub fn DataFlowDiagram() -> impl IntoView {
    let steps = vec![
        "Connect",
        "Auto-Detect",
        "Stream Data",
        "Process",
        "Aggregate",
        "Export CSV",
    ];

    view! {
        <div class="flow-container animate-fade-in">
            <h2 style="text-align: center; margin-bottom: 2rem; color: var(--color-text-secondary); font-size: 1.25rem; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">
                "Data Pipeline"
            </h2>
            <div class="flow-items">
                {steps.into_iter().enumerate().map(|(i, step)| {
                    view! {
                        <span
                            class="badge animate-fade-in"
                            style=format!("animation-delay: {}ms", i * 100)
                        >
                            {step}
                        </span>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

