use crate::components::Clock;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ClockPage() -> impl IntoView {
    view! {
        <A href="/" {..} class="w-full h-full block"><Clock title=true /></A>
    }
}
