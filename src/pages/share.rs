use crate::components::Clock;
use chrono::{Datelike, Local, NaiveTime, Utc};
use leptos::{logging::log, prelude::*};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

#[component]
pub fn SharePage() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();

    let now = Local::now();

    let hours = RwSignal::new(now.format("%k").to_string().trim().to_string());
    let minutes = RwSignal::new(now.format("%M").to_string());

    Effect::new(move || {
        let m = minutes.get().parse::<i32>().unwrap();
        if m >= 60 {
            hours.update(|h| *h = (h.parse::<i32>().unwrap() + m / 60).to_string());
            minutes.set((m % 60).to_string());
        } else if m < 0 {
            minutes.set("0".to_string());
            hours.update(|h| *h = (h.parse::<i32>().unwrap() - 1).to_string());
        }
    });
    Effect::new(move || {
        let h = hours.get().parse::<i32>().unwrap();
        if h >= 24 {
            hours.set((h % 24).to_string());
        } else if h < 0 {
            hours.set("23".to_string());
        }
    });

    let save = move |_| {
        let time_local = Local::now()
            .with_time(
                NaiveTime::from_hms_opt(
                    hours.get().parse::<u32>().unwrap(),
                    minutes.get().parse::<u32>().unwrap(),
                    0,
                )
                .unwrap(),
            )
            .unwrap();
        let day_local = time_local.day();
        let time_utc = time_local.to_utc();
        let day_utc = time_utc.day();
        let offset = if day_local + 1 == day_utc {
            "+1"
        } else if day_local - 1 == day_utc {
            "-1"
        } else {
            ""
        };
        navigate(
            &format!("/share/{}{}", time_utc.format("%H:%M"), offset),
            Default::default(),
        );
    };

    view! {
        <div class="w-full h-full grid content-center justify-center gap-[3rem] grid-rows-[7%_auto_auto_auto_1fr]">
            <div></div>
            <Clock title=true height="6".to_string() size="5".to_string() />
            <div class="w-full h-full grid content-center justify-center grid-rows-[repeat(3, auto)] grid-cols-[repeat(3, auto)]">
                <button
                    class="col-start-1 col-end-2 row-start-1 row-end-2 w-[9rem] justify-self-end hover:fill-amber-400"
                    on:click=move |_| {
                        hours.update(|h| *h = (h.parse::<i32>().unwrap() + 1).to_string());
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        shape-rendering="geometricPrecision"
                        text-rendering="geometricPrecision"
                        image-rendering="optimizeQuality"
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        viewBox="0 0 512 319.24"
                    >
                        <path d="m5.9 270.28 43.07 43.07c7.86 7.86 20.73 7.84 28.56 0l178.48-178.48L434.5 313.35c7.86 7.86 20.74 7.82 28.56 0l43.07-43.07c7.83-7.84 7.83-20.72 0-28.56L313.72 49.32l-.36-.37-43.07-43.07c-7.83-7.82-20.7-7.86-28.56 0l-43.07 43.07-.36.37L5.9 241.72c-7.87 7.86-7.87 20.7 0 28.56z" />
                    </svg>
                </button>
                <input type="number" bind:value=hours class="col-start-1 col-end-2 row-start-2 row-end-3 text-right text-[8rem] focus:outline-none w-full" />
                <button
                    class="col-start-1 col-end-2 row-start-3 row-end-4 w-[9rem] justify-self-end hover:fill-amber-400"
                    on:click=move |_| {
                        hours.update(|h| *h = (h.parse::<i32>().unwrap() - 1).to_string());
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        shape-rendering="geometricPrecision"
                        text-rendering="geometricPrecision"
                        image-rendering="optimizeQuality"
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        viewBox="0 0 512.02 319.26"
                    >
                        <path d="M5.9 48.96 48.97 5.89c7.86-7.86 20.73-7.84 28.56 0l178.48 178.48L434.5 5.89c7.86-7.86 20.74-7.82 28.56 0l43.07 43.07c7.83 7.84 7.83 20.72 0 28.56l-192.41 192.4-.36.37-43.07 43.07c-7.83 7.82-20.7 7.86-28.56 0l-43.07-43.07-.36-.37L5.9 77.52c-7.87-7.86-7.87-20.7 0-28.56z" />
                    </svg>
                </button>
                <p class="row-start-2 row-end-3 col-start-2 col-end-3 p-0.5 text-[8rem]">:</p>
                <button
                    class="col-start-3 col-end-4 row-start-1 row-end-2 w-[9rem] hover:fill-amber-400"
                    on:click=move |_| {
                        minutes.update(|m| *m = (m.parse::<i32>().unwrap() + 1).to_string());
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        shape-rendering="geometricPrecision"
                        text-rendering="geometricPrecision"
                        image-rendering="optimizeQuality"
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        viewBox="0 0 512 319.24"
                    >
                        <path d="m5.9 270.28 43.07 43.07c7.86 7.86 20.73 7.84 28.56 0l178.48-178.48L434.5 313.35c7.86 7.86 20.74 7.82 28.56 0l43.07-43.07c7.83-7.84 7.83-20.72 0-28.56L313.72 49.32l-.36-.37-43.07-43.07c-7.83-7.82-20.7-7.86-28.56 0l-43.07 43.07-.36.37L5.9 241.72c-7.87 7.86-7.87 20.7 0 28.56z" />
                    </svg>
                </button>
                <input type="number" bind:value=minutes class="col-start-3 col-end-4 row-start-2 row-end-3 text-[8rem] focus:outline-none w-full" />
                <button
                    class="col-start-3 col-end-4 row-start-3 row-end-4 w-[9rem] hover:fill-amber-400"
                    on:click=move |_| {
                        minutes.update(|m| *m = (m.parse::<i32>().unwrap() - 1).to_string());
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        shape-rendering="geometricPrecision"
                        text-rendering="geometricPrecision"
                        image-rendering="optimizeQuality"
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        viewBox="0 0 512.02 319.26"
                    >
                        <path d="M5.9 48.96 48.97 5.89c7.86-7.86 20.73-7.84 28.56 0l178.48 178.48L434.5 5.89c7.86-7.86 20.74-7.82 28.56 0l43.07 43.07c7.83 7.84 7.83 20.72 0 28.56l-192.41 192.4-.36.37-43.07 43.07c-7.83 7.82-20.7 7.86-28.56 0l-43.07-43.07-.36-.37L5.9 77.52c-7.87-7.86-7.87-20.7 0-28.56z" />
                    </svg>
                </button>
            </div>
            <button on:click=save class="w-[20rem] text-center text-[4rem] leading-16 bg-zinc-700 p-1 rounded-[4rem] hover:bg-zinc-600 justify-self-center">Save</button>
        </div>
    }
}

#[component]
pub fn SharedPage() -> impl IntoView {
    let params = use_params_map();
    let time = match params.get_untracked().get("time") {
        Some(t) => t,
        _ => "00:00".to_string(),
    };

    let (mut hours, mut minutes, mut days_offset) = if time.len() == 5 {
        (
            time[..2].parse::<i32>().unwrap_or_else(|_| 0),
            time[3..].parse::<i32>().unwrap_or_else(|_| 0),
            0,
        )
    } else if time.len() == 7 {
        (
            time[..2].parse::<i32>().unwrap_or_else(|_| 0),
            time[3..5].parse::<i32>().unwrap_or_else(|_| 0),
            match &time[5..] {
                "+1" => 1,
                "-1" => -1,
                _ => 0,
            },
        )
    } else {
        (0, 0, 0)
    };

    let offset = Local::now().offset().local_minus_utc();
    minutes += offset / 60;
    hours += minutes / 60;
    minutes %= 60;
    days_offset += hours / 24;
    hours %= 24;

    view! {
        <Title text=format!("{:02}:{:02}", hours, minutes) />
        <div class="w-full h-full flex items-center justify-center flex-col">
            <p class="leading-none h-[19rem] text-[16rem]">{format!("{:02}:{:02}", hours, minutes)}</p>
            <p class="text-[6rem]">
                {
                    if days_offset < 0 {
                        "A day before"
                    } else if days_offset > 0 {
                        "A day after"
                    } else {
                        ""
                    }
                }
            </p>
        </div>
    }
}
