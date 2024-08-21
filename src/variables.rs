use super::*;
use leptos::*;

stylance::import_crate_style!(style, "src/variables.module.css");

#[component]
pub fn ProjectList(width: ReadSignal<i32>) -> impl IntoView {
    view! {
        <div class=style::scrollable_list style:width=move || format!("{}px", width.get())>
            {(1..=25).map(|i| view! { <div class=style::item>"Project "{i}</div> }).collect_view()}
        </div>
    }
}

#[component]
pub fn VariableList() -> impl IntoView {
    view! {
        <div class=style::scrollable_list>
            {(1..=6).map(|i| view! { <div class=style::item>"Variable "{i}</div> }).collect_view()}
        </div>
    }
}

#[component]
pub fn VariableEdit() -> impl IntoView {
    view! { <textarea class=style::variable_edit contentEditable=true style:height="300px" /> }
}

#[component]
pub fn Variables() -> impl IntoView {
    let (projects_width, set_projects_width) = create_signal(300);
    view! {
        <div class="px-3 flex flex-col grow overflow-hidden">
            <div class="flex gap-3">
                <div class=style::label style:width=move || format!("{}px", projects_width.get())>
                    Projects
                </div>
                <div class=style::label>Variables</div>
            </div>
            <div class="flex gap-3 grow overflow-hidden">
                <ProjectList width=projects_width />
                <VariableList />
            </div>
            <VariableEdit />
        </div>
    }
}
