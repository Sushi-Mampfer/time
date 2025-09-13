use crate::components::Clock;
use leptos::prelude::*;

#[component]
pub fn ClockPage() -> impl IntoView {
    view! {
        <Clock title=true />
    }
}
