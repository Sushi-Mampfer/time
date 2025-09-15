use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="w-full h-full">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <Stylesheet id="leptos" href="/pkg/time.css" />
                <MetaTags />
            </head>
            <body class="w-full, h-full bg-zinc-800 text-neutral-200 font-[helvetica] fill-neutral-200">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="You shouldn't see this" />
        <Router>
            <main class="w-full h-full">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("clock") view=ClockPage />
                    <Route path=StaticSegment("share") view=SharePage />
                    <Route path=path!("share/:time") view=SharedPage />
                </Routes>
            </main>
        </Router>
    }
}
