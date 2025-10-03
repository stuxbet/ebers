use leptos::prelude::*;

#[component]
pub fn StatsDashboard(connected: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 1.5rem; margin: 2rem 0;">
            <div class="card animate-fade-in" style="text-align: center;">
                <div class="asset-label" style="margin-bottom: 0.5rem;">"Connection"</div>
                <div class="health-score" style="font-size: 2.5rem; margin: 0.5rem 0;">
                    {move || if connected.get() { "Active" } else { "Idle" }}
                </div>
                <div class="asset-label">"status"</div>
            </div>

            <div class="card animate-fade-in" style="text-align: center; animation-delay: 100ms;">
                <div class="asset-label" style="margin-bottom: 0.5rem;">"Data Rate"</div>
                <div class="health-score" style="font-size: 2.5rem; margin: 0.5rem 0;">
                    {move || if connected.get() { "9600" } else { "—" }}
                </div>
                <div class="asset-label">"baud"</div>
            </div>

            <div class="card animate-fade-in" style="text-align: center; animation-delay: 200ms;">
                <div class="asset-label" style="margin-bottom: 0.5rem;">"Uptime"</div>
                <div class="health-score" style="font-size: 2.5rem; margin: 0.5rem 0;">
                    {move || if connected.get() { "Live" } else { "—" }}
                </div>
                <div class="asset-label">"session"</div>
            </div>

            <div class="card animate-fade-in" style="text-align: center; animation-delay: 300ms;">
                <div class="asset-label" style="margin-bottom: 0.5rem;">"Protocol"</div>
                <div class="health-score" style="font-size: 2.5rem; margin: 0.5rem 0;">
                    {move || if connected.get() { "USB" } else { "—" }}
                </div>
                <div class="asset-label">"type"</div>
            </div>
        </div>
    }
}
