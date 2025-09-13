use crate::components::Clock;
use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn SharePage() -> impl IntoView {
    let now = Local::now();

    let hours = RwSignal::new(now.format("%H").to_string());
    let minutes = RwSignal::new(now.format("%M").to_string());

    Effect::new(move || {
        let m = minutes.get().parse::<i32>().unwrap();
        if m >= 60 {
            hours.update(|h| *h = (h.parse::<i32>().unwrap() + m / 60).to_string());
            minutes.set((m % 60).to_string());
        } else if m < 0 {
            minutes.set("0".to_string());
        }
    });
    Effect::new(move || {
        let h = hours.get().parse::<i32>().unwrap();
        if h >= 24 {
            hours.set((h % 24).to_string());
        } else if h < 0 {
            hours.set("0".to_string());
        }
    });

    view! {
        <Clock title=true />
        <div>
            <button class="w-3 h-1" on:click=move |_| { hours.update(|h| *h = (h.parse::<i32>().unwrap() + 1).to_string());}><svg xmlns="http://www.w3.org/2000/svg" shape-rendering="geometricPrecision" text-rendering="geometricPrecision" image-rendering="optimizeQuality" fill-rule="evenodd" clip-rule="evenodd" viewBox="0 0 512 319.24"><path d="m5.9 270.28 43.07 43.07c7.86 7.86 20.73 7.84 28.56 0l178.48-178.48L434.5 313.35c7.86 7.86 20.74 7.82 28.56 0l43.07-43.07c7.83-7.84 7.83-20.72 0-28.56L313.72 49.32l-.36-.37-43.07-43.07c-7.83-7.82-20.7-7.86-28.56 0l-43.07 43.07-.36.37L5.9 241.72c-7.87 7.86-7.87 20.7 0 28.56z"/></svg></button>
            <input type="number" bind:value=hours />
            <button class="w-3 h-1" on:click=move |_| { hours.update(|h| *h = (h.parse::<i32>().unwrap() - 1).to_string());}><svg xmlns="http://www.w3.org/2000/svg" shape-rendering="geometricPrecision" text-rendering="geometricPrecision" image-rendering="optimizeQuality" fill-rule="evenodd" clip-rule="evenodd" viewBox="0 0 512.02 319.26"><path d="M5.9 48.96 48.97 5.89c7.86-7.86 20.73-7.84 28.56 0l178.48 178.48L434.5 5.89c7.86-7.86 20.74-7.82 28.56 0l43.07 43.07c7.83 7.84 7.83 20.72 0 28.56l-192.41 192.4-.36.37-43.07 43.07c-7.83 7.82-20.7 7.86-28.56 0l-43.07-43.07-.36-.37L5.9 77.52c-7.87-7.86-7.87-20.7 0-28.56z"/></svg></button>
            <button class="w-3 h-1" on:click=move |_| { minutes.update(|m| *m = (m.parse::<i32>().unwrap() + 1).to_string());}><svg xmlns="http://www.w3.org/2000/svg" shape-rendering="geometricPrecision" text-rendering="geometricPrecision" image-rendering="optimizeQuality" fill-rule="evenodd" clip-rule="evenodd" viewBox="0 0 512 319.24"><path d="m5.9 270.28 43.07 43.07c7.86 7.86 20.73 7.84 28.56 0l178.48-178.48L434.5 313.35c7.86 7.86 20.74 7.82 28.56 0l43.07-43.07c7.83-7.84 7.83-20.72 0-28.56L313.72 49.32l-.36-.37-43.07-43.07c-7.83-7.82-20.7-7.86-28.56 0l-43.07 43.07-.36.37L5.9 241.72c-7.87 7.86-7.87 20.7 0 28.56z"/></svg></button>
            <input type="number" bind:value=minutes />
            <button class="w-3 h-1" on:click=move |_| { minutes.update(|m| *m = (m.parse::<i32>().unwrap() - 1).to_string());}><svg xmlns="http://www.w3.org/2000/svg" shape-rendering="geometricPrecision" text-rendering="geometricPrecision" image-rendering="optimizeQuality" fill-rule="evenodd" clip-rule="evenodd" viewBox="0 0 512.02 319.26"><path d="M5.9 48.96 48.97 5.89c7.86-7.86 20.73-7.84 28.56 0l178.48 178.48L434.5 5.89c7.86-7.86 20.74-7.82 28.56 0l43.07 43.07c7.83 7.84 7.83 20.72 0 28.56l-192.41 192.4-.36.37-43.07 43.07c-7.83 7.82-20.7 7.86-28.56 0l-43.07-43.07-.36-.37L5.9 77.52c-7.87-7.86-7.87-20.7 0-28.56z"/></svg></button>
        </div>
    }
}

#[component]
pub fn SharedPage() -> impl IntoView {
    view! {}
}
