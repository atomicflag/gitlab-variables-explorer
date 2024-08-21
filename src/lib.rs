pub mod app;
pub mod navbar;
pub mod settings;
pub mod variables;

pub use app::*;
pub use navbar::*;
pub use settings::*;
pub use variables::*;

use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Page {
    Variables,
    Settings,
}
