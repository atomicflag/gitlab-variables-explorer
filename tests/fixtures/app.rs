use gitlab_variables_explorer_ui::*;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

fn test_wrapper() -> Element {
    let document = leptos::document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);
    test_wrapper
}

pub fn app_view() {
    let test_wrapper = test_wrapper();
    mount_to(test_wrapper.clone().unchecked_into(), || view! { <App /> });
}

pub fn find_text() -> Option<Element> {
    leptos::document().query_selector("main p").unwrap()
}
