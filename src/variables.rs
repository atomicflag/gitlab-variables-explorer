use super::*;
use leptos::html::{Div, Textarea};
use leptos::prelude::*;
use leptos_use::core::Position;
use leptos_use::{
    use_draggable_with_options, use_element_bounding, UseDraggableOptions, UseDraggableReturn,
    UseElementBoundingReturn,
};
use reactive_stores::StoreField;
use thaw::*;

stylance::import_crate_style!(style, "src/variables.module.css");

#[component]
pub fn ProjectList(#[prop(into)] width: Signal<i32>) -> impl IntoView {
    let state = expect_context::<Store<ProjectsAndVariables>>();
    view! {
        <div style:width=move || format!("calc({}px - 0.75rem)", width.get())>
            <For each=move || state.projects() key=|row| row.read().id.clone() let:project>
                <div>{project.read().name.clone()}</div>
            </For>
        </div>
    }
}

#[component]
pub fn VariableList() -> impl IntoView {
    view! {
        <div class="grow">
            {(1..=25).map(|i| view! { <div>"Variable "{i}</div> }).collect_view()}
        </div>
    }
}

#[component]
pub fn VariableEdit(#[prop(into)] height: Signal<i32>) -> impl IntoView {
    view! { <textarea style:height=move || format!("calc({}px - 1.5rem)", height.get()) /> }
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
    view! {
        <Flex class="grow p-2 overflow-hidden">
            <Flex vertical=true class="min-w-[256px] overflow-y-auto" gap=FlexGap::Size(0)>
                <a href="#" class="px-2 py-1 border-l-2 hover:bg-neutral-700">
                    <Flex vertical=true align=FlexAlign::Start gap=FlexGap::Size(0)>
                        <div class="text-xs text-gray-400">path / to / project</div>
                        <div>"Project 0"</div>
                    </Flex>
                </a>
                {(1..=25)
                    .map(|i| {
                        view! {
                            <a
                                href="#"
                                class="px-2 py-1 border-l-2 hover:bg-neutral-700 border-transparent"
                            >
                                <Flex vertical=true align=FlexAlign::Start gap=FlexGap::Size(0)>
                                    <div class="text-xs text-gray-400">path / to / project</div>
                                    <div>"Project "{i}</div>
                                </Flex>
                            </a>
                        }
                    })
                    .collect_view()}
            </Flex>
            <Flex vertical=true class="grow overflow-y-auto">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell resizable=true min_width=100.0>
                                "Name"
                            </TableHeaderCell>
                            <TableHeaderCell>"Value"</TableHeaderCell>
                            <TableHeaderCell attr:width="100"></TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {(1..=25)
                            .map(|i| {
                                view! {
                                    <TableRow class=style::row>
                                        <TableCell>"GITLAB_VARIABLE_"{i}</TableCell>
                                        <TableCell>"2"</TableCell>
                                        <TableCell class=style::buttons>
                                            <Button
                                                appearance=ButtonAppearance::Transparent
                                                icon=icondata::TbPencil
                                            />
                                            <Button
                                                appearance=ButtonAppearance::Transparent
                                                icon=icondata::TbTrash
                                                class="text-red-700"
                                            />
                                        </TableCell>
                                    </TableRow>
                                }
                            })
                            .collect_view()}
                    </TableBody>
                </Table>
            </Flex>
        </Flex>
    }
}
