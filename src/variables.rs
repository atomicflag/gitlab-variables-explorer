use super::*;
use leptos::html::{Div, Textarea};
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{
    use_draggable_with_options, use_element_bounding, UseDraggableOptions, UseDraggableReturn,
    UseElementBoundingReturn,
};

stylance::import_crate_style!(style, "src/variables.module.css");

#[component]
pub fn ProjectList(#[prop(into)] width: Signal<i32>) -> impl IntoView {
    view! {
        <div
            class=style::scrollable_list
            style:width=move || format!("calc({}px - 0.75rem)", width.get())
        >
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
pub fn VariableEdit(#[prop(into)] height: Signal<i32>) -> impl IntoView {
    view! {
        <textarea
            class=style::variable_edit
            contentEditable=true
            style:height=move || format!("calc({}px - 1.5rem)", height.get())
        />
    }
}

fn horizontal_resize(initial_width: i32) -> (NodeRef<Div>, Signal<i32>) {
    let node_ref = create_node_ref::<Div>();
    let UseDraggableReturn { x, .. } = use_draggable_with_options(
        node_ref,
        UseDraggableOptions::default()
            .prevent_default(true)
            .initial_value(Position {
                x: initial_width as f64,
                y: 0.0,
            }),
    );
    (node_ref, (move || x.get() as i32).into())
}

fn vertical_resize(initial_height: i32) -> (NodeRef<Div>, Signal<i32>) {
    let node_ref = create_node_ref::<Div>();
    let UseDraggableReturn { y, .. } = use_draggable_with_options(
        node_ref,
        UseDraggableOptions::default()
            .prevent_default(true)
            .initial_value(Position {
                x: 0.0,
                y: initial_height as f64,
            }),
    );
    (node_ref, (move || y.get() as i32).into())
}

fn window_height() -> i32 {
    leptos::window()
        .inner_height()
        .expect("window should exist")
        .as_f64()
        .expect("window height should be f64") as i32
}

fn element_bottom() -> (NodeRef<Div>, Signal<i32>) {
    let node_ref = create_node_ref::<Div>();
    let UseElementBoundingReturn { bottom, .. } = use_element_bounding(node_ref);
    (node_ref, (move || bottom.get() as i32).into())
}

#[component]
pub fn Variables() -> impl IntoView {
    let (h_resize_ref, h_resize_width) = horizontal_resize(300);
    let (v_resize_ref, v_resize_height) = vertical_resize(window_height() - 200);
    let (container_ref, container_bottom) = element_bottom();
    let edit_height = move || container_bottom.get() - v_resize_height.get();
    view! {
        <div class="px-3 flex flex-col grow overflow-hidden" node_ref=container_ref>
            <div class="flex gap-3">
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
        </div>
    }
}
