use gitlab_variables_explorer_ui::App;
use leptos::*;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

fn main() {
    fmt()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .without_time()
        .with_ansi(false)
        .init();
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
