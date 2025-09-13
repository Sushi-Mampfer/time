use chrono::Local;
use gloo_timers::callback::{Interval, Timeout};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Clock(#[prop(default = false)] title: bool) -> impl IntoView {
    let (time, set_time) = signal(Local::now().format("%H:%M:%S").to_string());
    #[cfg(target_arch = "wasm32")]
    Interval::new(1000, move || {
        set_time.set(Local::now().format("%H:%M:%S").to_string());
    })
    .forget();

    view! {
        <Show when=move || title>
            <Title text=time />
        </ Show>
        <p>{time}</p>
    }
}
