use super::*;
use leptos::{prelude::*, task::spawn_local};
use serde_wasm_bindgen::{from_value, to_value};
use tracing::{error, info};

stylance::import_crate_style!(style, "src/app.module.css");

async fn fetch_variables() -> Result<ProjectsAndVariables, String> {
    let result = invoke("fetch_variables", JsValue::null()).await;
    match result {
        Ok(config) => Ok(from_value(config).expect("value should be a valid object")),
        Err(e) => Err(e.as_string().expect("value should be a valid string")),
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = signal(Page::Variables);
    let on_refresh = move || {
        info!("TODO: refresh");
        spawn_local(async move {
            info!("{:?}", fetch_variables().await);
        });
    };
    view! {
        <main class="h-screen flex flex-col">
            <NavBar page=page set_page=set_page on_refresh=on_refresh />
            <Show when=move || page.get() == Page::Variables>
                <Variables />
            </Show>
            <Show when=move || page.get() == Page::Settings>
                <Settings set_page=set_page />
            </Show>
        </main>
    }
}
