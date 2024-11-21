use super::*;
use leptos::*;
use std::{rc::Rc,cell::RefCell};
use tracing::{info,error};
use serde_wasm_bindgen::{from_value, to_value};

stylance::import_crate_style!(style, "src/app.module.css");

async fn read_config() -> Result<Config, String> {
    let result = invoke("read_config", JsValue::null()).await;
    match result {
        Ok(config) => Ok(from_value(config).expect("value should be a valid config")),
        Err(e) => Err(e.as_string().expect("value should be a valid string")),
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = create_signal(Page::Variables);
    let context = Rc::new(RefCell::new(Context::default()));
    provide_context(context.clone());
    spawn_local(async move {
        let result = read_config().await;
        match result {
            Ok(config) => {
                let mut context = context.borrow_mut();
                context.config.host = config.host;
                context.config.token = config.token;
            }
            Err(e) => error!("Error reading configuration file: {}", e),
        };
    });
    let on_refresh = move |_| {
        info!("TODO: refresh");
    };
    view! {
        <main class="h-screen flex flex-col">
            <NavBar page=page set_page=set_page on_refresh=on_refresh/>
            <Show when=move || page.get() == Page::Variables>
                <Variables />
            </Show>
            <Show when=move || page.get() == Page::Settings>
                <Settings set_page=set_page />
            </Show>
        </main>
    }
}
