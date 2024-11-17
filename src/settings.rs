use super::*;
use leptos::*;
use serde_wasm_bindgen::{from_value, to_value};
use tracing::error;

stylance::import_crate_style!(style, "src/settings.module.css");

#[derive(Serialize, Debug)]
pub struct WriteConfigArgs {
    config: Config,
}

#[component]
pub fn Settings(set_page: WriteSignal<Page>) -> impl IntoView {
    let (host, set_host) = create_signal(String::new());
    let (token, set_token) = create_signal(String::new());
    spawn_local(async move {
        let result = invoke("read_config", JsValue::null()).await;
        match result {
            Ok(config) => {
                let config: Config = from_value(config).expect("value should be a valid config");
                set_host.set(config.host);
                set_token.set(config.token);
            }
            Err(e) => {
                error!(
                    "Error reading configuration file: {}",
                    e.as_string().expect("value should be a valid string")
                );
            }
        };
    });
    let on_save = move |_| {
        spawn_local(async move {
            let result = invoke(
                "write_config",
                to_value(&WriteConfigArgs {
                    config: Config {
                        host: host.get_untracked(),
                        token: token.get_untracked(),
                    },
                })
                .expect("config serialization should succeed"),
            )
            .await;
            if let Err(e) = result {
                error!(
                    "Error saving the configuration file: {}",
                    e.as_string().expect("value should be a valid string")
                );
            };
            set_page.set(Page::Variables);
        });
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
            <div class="mt-6 flex items-center justify-end">
                <button on:click=on_save class=style::save_button>
                    Save
                </button>
            </div>
        </div>
    }
}
