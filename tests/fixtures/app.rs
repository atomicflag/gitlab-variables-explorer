use gitlab_variables_explorer_ui::*;
use leptos::{leptos_dom::helpers::document, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn app_view() {
    let test_wrapper = {
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _ = document.body().unwrap().append_child(&test_wrapper);
        test_wrapper
    };
    mount_to(test_wrapper.unchecked_into(), || view! { <App /> }).forget();
}

pub fn find_text() -> Option<Element> {
    document().query_selector("main p").unwrap()
}
