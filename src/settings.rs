use super::*;
use leptos::*;

stylance::import_crate_style!(style, "src/settings.module.css");

#[component]
pub fn Settings(set_page: WriteSignal<Page>) -> impl IntoView {
    view! {
        <div class="px-6 w-full max-w-xl mx-auto">
            <div class="mt-5 grid grid-cols-1 gap-x-6 gap-y-8">
                <div class="col-span-full">
                    <label class=style::label>Host</label>
                    <div class="mt-2">
                        <input type="text" placeholder="gitlab.com" class=style::input_field />
                    </div>
                </div>
                <div class="col-span-full">
                    <label class=style::label>Token</label>
                    <div class="mt-2">
                        <input type="password" class=style::input_field />
                    </div>
                </div>
            </div>
            <div class="mt-6 flex items-center justify-end">
                <button on:click=move |_| set_page.set(Page::Variables) class=style::save_button>
                    Save
                </button>
            </div>
        </div>
    }
}
