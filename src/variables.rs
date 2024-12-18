use super::*;
use leptos::html::{Div, Textarea};
use leptos::prelude::*;
use leptos_use::core::Position;
use leptos_use::{
    use_draggable_with_options, use_element_bounding, UseDraggableOptions, UseDraggableReturn,
    UseElementBoundingReturn,
};
use reactive_stores::StoreField;

stylance::import_crate_style!(style, "src/variables.module.css");

#[component]
pub fn ProjectList(#[prop(into)] width: Signal<i32>) -> impl IntoView {
    let state = expect_context::<Store<ProjectsAndVariables>>();
    view! {
        <div
            class=style::scrollable_list
            style:width=move || format!("calc({}px - 0.75rem)", width.get())
        >
            <For
                each=move || state.projects()
                key=|row| row.read().id.clone()
                children=|child| {
                    let value = child.reader().expect("value should exist");
                    view! { <div class=style::item>{value.name.clone()}</div> }
                }
            />
        </div>
    }
}

#[component]
pub fn VariableList() -> impl IntoView {
    view! {
        <div class=style::scrollable_list.to_owned()
            + " grow">
            {(1..=25).map(|i| view! { <div class=style::item>"Variable "{i}</div> }).collect_view()}
        </div>
    }
}

#[component]
pub fn VariableEdit(#[prop(into)] height: Signal<i32>) -> impl IntoView {
    view! {
        <textarea
            class=style::variable_edit
            style:height=move || format!("calc({}px - 1.5rem)", height.get())
        />
    }
}

fn horizontal_resize(initial_width: i32) -> (NodeRef<Div>, Signal<i32>) {
    let node_ref: NodeRef<Div> = NodeRef::new();
    let UseDraggableReturn { x, .. } = use_draggable_with_options(
        node_ref,
        UseDraggableOptions::default()
            .prevent_default(true)
            .initial_value(Position {
                x: initial_width as f64,
                y: 0.0,
            }),
    );
    (node_ref, Signal::derive(move || x.get() as i32))
}

fn vertical_resize(initial_height: i32) -> (NodeRef<Div>, Signal<i32>) {
    let node_ref: NodeRef<Div> = NodeRef::new();
    let UseDraggableReturn { y, .. } = use_draggable_with_options(
        node_ref,
        UseDraggableOptions::default()
            .prevent_default(true)
            .initial_value(Position {
                x: 0.0,
                y: initial_height as f64,
            }),
    );
    (node_ref, Signal::derive(move || y.get() as i32))
}

fn window_height() -> i32 {
    leptos::leptos_dom::helpers::window()
        .inner_height()
        .expect("window should exist")
        .as_f64()
        .expect("window height should be f64") as i32
}

fn element_top() -> (NodeRef<Div>, Signal<i32>) {
    let node_ref: NodeRef<Div> = NodeRef::new();
    let UseElementBoundingReturn { top, .. } = use_element_bounding(node_ref);
    (node_ref, Signal::derive(move || top.get() as i32))
}

#[component]
pub fn Variables() -> impl IntoView {
    let (h_resize_ref, h_resize_width) = horizontal_resize(300);
    let (v_resize_ref, v_resize_height) = vertical_resize(window_height() - 200);
    let (container_ref, container_top) = element_top();
    let edit_height = Signal::derive(move || *container_top.read() - *v_resize_height.read());
    view! {
        <div class="px-3 flex flex-col grow overflow-hidden">
            <div class="flex grow gap-3">
                <div
                    class=style::label
                    style:width=move || format!("calc({}px - 0.75rem)", h_resize_width.get())
                >
                    Projects
                </div>
                <div class=style::label>Variables</div>
            </div>
            <div class="flex grow overflow-hidden">
                <ProjectList width=h_resize_width />
                <div node_ref=h_resize_ref class=style::h_resize />
                <VariableList />
            </div>
            <div node_ref=v_resize_ref class=style::v_resize />
            <VariableEdit height=edit_height />
            <div class="flex grow justify-end gap-3 pb-3" node_ref=container_ref>
                <button class=style::button>Delete</button>
                <button class=style::button>Revert</button>
                <button class=style::button_accent>Save</button>
            </div>
        </div>
    }
}
