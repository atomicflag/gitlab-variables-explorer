pub mod app;
mod navbar;
mod settings;
mod variables;

pub use app::*;
use navbar::*;
use settings::*;
use variables::*;

use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct Config {
    host: String,
    token: String,
}

#[derive(Debug, Default)]
struct Context {
    config: Config
}

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Variables,
    Settings,
}
