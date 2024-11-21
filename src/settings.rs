use super::*;
use leptos::*;
use serde_wasm_bindgen::{from_value, to_value};
use std::{cell::RefCell, rc::Rc};
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

#[component]
pub fn Settings(set_page: WriteSignal<Page>) -> impl IntoView {
    let context = use_context::<Rc<RefCell<Context>>>().expect("context should exist");
    let (host, set_host) = create_signal(context.borrow().config.host.clone());
    let (token, set_token) = create_signal(context.borrow().config.token.clone());

    let on_save = move |_| {
        let context = context.clone();
        spawn_local(async move {
            let result = write_config(Config {
                host: host.get_untracked(),
                token: token.get_untracked(),
            })
            .await;
            if let Err(e) = result {
                error!("Error saving the configuration file: {}", e);
            };
            let mut context = context.borrow_mut();
            context.config.host = host.get_untracked();
            context.config.token = token.get_untracked();
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
            <div class="mt-6 flex items-center gap-3 justify-end">
                <button class=style::button>Cancel</button>
                <button on:click=on_save class=style::save_button>
                    Save
                </button>
            </div>
        </div>
    }
}
