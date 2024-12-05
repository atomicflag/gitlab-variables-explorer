use super::*;
use leptos::{prelude::*, task::spawn_local};
use serde_wasm_bindgen::{from_value, to_value};
use tracing::error;

stylance::import_crate_style!(style, "src/settings.module.css");

#[derive(Serialize, Debug)]
struct WriteConfigArgs {
    config: Config,
}

async fn write_config(config: Config) -> Result<(), String> {
    let result = invoke(
        "write_config",
        to_value(&WriteConfigArgs { config }).expect("config serialization should succeed"),
    )
    .await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.as_string().expect("value should be a valid string")),
    }
}

async fn read_config() -> Result<Config, String> {
    let result = invoke("read_config", JsValue::null()).await;
    match result {
        Ok(config) => Ok(from_value(config).expect("value should be a valid config")),
        Err(e) => Err(e.as_string().expect("value should be a valid string")),
    }
}

#[component]
pub fn Settings(set_page: WriteSignal<Page>) -> impl IntoView {
    let (host, set_host) = signal(String::new());
    let (token, set_token) = signal(String::new());
    spawn_local(async move {
        let result = read_config().await;
        match result {
            Ok(config) => {
                set_host.set(config.host);
                set_token.set(config.token);
            }
            Err(e) => error!("Error reading configuration file: {}", e),
        };
    });
    let on_save = move |_| {
        spawn_local(async move {
            let result = write_config(Config {
                host: host.get_untracked(),
                token: token.get_untracked(),
            })
            .await;
            if let Err(e) = result {
                error!("Error saving the configuration file: {}", e);
            };
            set_page.set(Page::Variables);
        });
    };

    let on_cancel = move |_| {
        set_page.set(Page::Variables);
    };
    view! {
        <div class="px-6 w-full max-w-xl mx-auto">
            <div class="mt-5 grid grid-cols-1 gap-x-6 gap-y-8">
                <div class="col-span-full">
                    <label class=style::label>Host</label>
                    <div class="mt-2">
                        <input
                            type="text"
                            placeholder="gitlab.com"
                            class=style::input_field
                            prop:value=host
                            on:input=move |ev| set_host.set(event_target_value(&ev))
                        />
                    </div>
                </div>
                <div class="col-span-full">
                    <label class=style::label>Token</label>
                    <div class="mt-2">
                        <input
                            type="password"
                            class=style::input_field
                            prop:value=token
                            on:input=move |ev| set_token.set(event_target_value(&ev))
                        />
                    </div>
                </div>
            </div>
            <div class="mt-6 flex items-center gap-3 justify-end">
                <button class=style::button on:click=on_cancel>
                    Cancel
                </button>
                <button on:click=on_save class=style::save_button>
                    Save
                </button>
            </div>
        </div>
    }
}
