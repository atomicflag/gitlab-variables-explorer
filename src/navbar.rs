use super::*;
use icondata as i;
use leptos::prelude::*;
use leptos_icons::*;
use thaw::*;

stylance::import_crate_style!(style, "src/navbar.module.css");

#[component]
pub fn NavBar(
    page: ReadSignal<Page>,
    set_page: WriteSignal<Page>,
    #[prop(into)] on_refresh: Callback<()>,
) -> impl IntoView {
    let (search_string, set_search_string) = signal(String::new());
    // let toggle_settings = move |_| match page.get() {
    //     Page::Settings => set_page.set(Page::Variables),
    //     _ => set_page.set(Page::Settings),
    // };
    let state = expect_context::<Store<ProjectsAndVariables>>();
    view! {
        <Flex class="p-2" style="background-color: var(--colorNeutralBackground6)">
            <Input placeholder="Search" class="grow" />
            <Button appearance=ButtonAppearance::Subtle icon=icondata::TbReload />
            <Button appearance=ButtonAppearance::Subtle icon=icondata::TbSettings />
        </Flex>
    }
}
