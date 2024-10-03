use super::*;
use icondata as i;
use leptos::*;
use leptos_icons::*;

stylance::import_crate_style!(style, "src/navbar.module.css");

#[component]
pub fn SearchBar(
    search_string: ReadSignal<String>,
    set_search_string: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="relative w-full flex flex-grow">
            <input
                type="text"
                placeholder="Search"
                class=style::search_bar
                on:input=move |ev| { set_search_string.set(event_target_value(&ev)) }
                prop:value=search_string
            />
            <Show when=move || search_string.with(|v| !v.is_empty())>
                <button
                    class="absolute right-2 inset-y-0"
                    on:click=move |_| set_search_string.set(String::new())
                >
                    <Icon width="1.5rem" height="1.5rem" icon=i::TbX />
                </button>
            </Show>
        </div>
    }
}

#[component]
pub fn NavBar(page: ReadSignal<Page>, set_page: WriteSignal<Page>) -> impl IntoView {
    let (search_string, set_search_string) = create_signal(String::new());
    let button_style = move || match page.get() {
        Page::Settings => style::button_active,
        _ => style::button,
    };
    let toggle_settings = move |_| match page.get() {
        Page::Settings => set_page.set(Page::Variables),
        _ => set_page.set(Page::Settings),
    };
    view! {
        <nav>
            <div class="mx-auto px-3">
                <div class="flex h-16 items-center justify-between">
                    <div class="flex items-center space-x-2 flex-grow">
                        <Show when=move || page.get() == Page::Variables>
                            <SearchBar
                                search_string=search_string
                                set_search_string=set_search_string
                            />
                            <a href="#" class=style::button>
                                <Icon width="1.5rem" height="1.5rem" icon=i::TbReload />
                            </a>
                        </Show>
                        <Show when=move || page.get() == Page::Settings>
                            <div class="block w-full flex-grow font-bold">Settings</div>
                        </Show>
                        <a href="#" class=button_style on:click=toggle_settings>
                            <Icon width="1.5rem" height="1.5rem" icon=i::TbSettings />
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
