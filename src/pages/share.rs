use crate::components::Clock;
use chrono::Local;
use leptos::{logging::log, prelude::*};

#[component]
pub fn SharePage() -> impl IntoView {
    let now = Local::now();

    let hours = RwSignal::new(now.format("%H").to_string());
    let minutes = RwSignal::new(now.format("%M").to_string());

    view! {
        <Clock title=true />
        <div>
            <input type="number" bind:value=hours />
            <input type="number" bind:value=minutes />
        </div>
    }
}

#[component]
pub fn SharedPage() -> impl IntoView {
    view! {}
}
