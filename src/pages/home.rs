use crate::components::Clock;
use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Clock title=true />
    }
}
