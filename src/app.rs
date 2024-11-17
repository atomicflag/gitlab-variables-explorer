use super::*;
use leptos::*;

use tracing::info;

stylance::import_crate_style!(style, "src/app.module.css");

#[component]
pub fn App() -> impl IntoView {
    let (page, set_page) = create_signal(Page::Variables);
    let on_refresh = move |_| {
        info!("TODO: refresh");
    };
    // provide context???
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
