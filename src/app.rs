use super::*;
use leptos::*;

use serde_wasm_bindgen::{from_value, to_value};
// use serde::{Deserialize, Serialize};
//
stylance::import_crate_style!(style, "src/app.module.css");

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = create_signal(Page::Variables);

    // let on_click = move |_| {
    //     spawn_local(async move {
    //         let msg = invoke("greet", to_value(&GreetArgs{name:"asdf"}).unwrap()).await;
    //         set_text.set(from_value(msg).unwrap());
    //     });
    // };
    // <input class="" type="text" placeholder="Search" autocomplete="off" />
    // <p>{text}</p>
    // <button on:click=on_click>"callback"</button>

    view! {
        <main class="h-screen flex flex-col">
            <NavBar page=page set_page=set_page />
            <Show when=move || page.get() == Page::Variables>
                <Variables />
            </Show>
            <Show when=move || page.get() == Page::Settings>
                <Settings set_page=set_page />
            </Show>
        </main>
    }
}
