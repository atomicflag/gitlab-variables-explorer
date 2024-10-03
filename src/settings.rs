use super::*;
use ev::MouseEvent;
use leptos::*;
use serde_wasm_bindgen::{from_value, to_value};
use tracing::info;
use web_sys::HtmlOListElement;

stylance::import_crate_style!(style, "src/settings.module.css");

#[derive(Serialize, Debug)]
pub struct WriteConfigArgs {
    config: Config,
}

#[component]
pub fn Settings(set_page: WriteSignal<Page>) -> impl IntoView {
    let write_config = create_action(|config: &Config| {
        invoke(
            "write_config",
            to_value(&WriteConfigArgs {
                config: config.clone(),
            })
            .unwrap(),
        )
    });
    let (host, set_host) = create_signal(String::new());
    let (token, set_token) = create_signal(String::new());
    spawn_local(async move {
        let config: Config = from_value(invoke("read_config", JsValue::null()).await).unwrap();
        set_host.set(config.host);
        set_token.set(config.token);
    });
    let on_save = move |_: MouseEvent| {
        write_config.dispatch(Config {
            host: host.get_untracked(),
            token: token.get_untracked(),
        });
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
                            value=host
                        />
                    </div>
                </div>
                <div class="col-span-full">
                    <label class=style::label>Token</label>
                    <div class="mt-2">
                        <input type="password" class=style::input_field value=token />
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
