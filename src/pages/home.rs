use crate::components::Clock;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full h-full grid content-center justify-center">
            <A href="/clock" {..} class="hover:text-amber-400"><Clock title=true /></A>
            <A href="/share" {..} class="w-full text-center text-[4rem] leading-16 bg-zinc-700 p-1 rounded-[4rem] hover:bg-zinc-600">Share</A>
        </div>
    }
}
