use super::*;
use leptos::{prelude::*, task::spawn_local};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;
use tracing::{error, info};

stylance::import_crate_style!(style, "src/app.module.css");

#[derive(Clone, Serialize, Deserialize, Debug)]
struct FetchVariablesResult {
    pub projects: Vec<Project>,
    pub variables: Vec<BoundVariable>,
}

async fn fetch_variables() -> Result<FetchVariablesResult, String> {
    let result = invoke("fetch_variables", JsValue::null()).await;
    match result {
        Ok(config) => Ok(from_value(config).expect("value should be a valid object")),
        Err(e) => Err(e.as_string().expect("value should be a valid string")),
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = signal(Page::Variables);
    let state = Store::new(ProjectsAndVariables::default());
    provide_context(state.clone());
    let on_refresh = move || {
        spawn_local(async move {
            state.is_refreshing().set(true);
            let result = fetch_variables().await;
            match result {
                Ok(value) => {
                    state.variables().set(value.variables);
                    state.projects().set(value.projects);
                }
                Err(e) => error!("Error fetching variables: {}", e),
            };
            state.is_refreshing().set(false);
        });
    };
    let theme = RwSignal::new(Theme::dark());
    view! {
        <ConfigProvider theme class="flex flex-col h-screen">
            <NavBar page set_page on_refresh />
            <Show when=move || page.get() == Page::Variables>
                <Variables />
            </Show>
            <Show when=move || page.get() == Page::Settings>
                <Settings set_page />
            </Show>
        </ConfigProvider>
    }
}
