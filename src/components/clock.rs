use chrono::Local;
use gloo_timers::callback::{Interval, Timeout};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Clock(
    #[prop(default = false)] title: bool,
    #[prop(default = "16".to_string())] size: String,
    #[prop(default = "19".to_string())] height: String,
) -> impl IntoView {
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
        <div class="w-full h-full flex items-center justify-center"><p class="leading-none" style={format!("height: {}rem; font-size: {}rem", height, size)}>{time}</p></div>
    }
}
