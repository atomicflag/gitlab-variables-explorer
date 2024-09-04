mod fixtures;

use fixtures::app as ui;
use gitlab_variables_explorer_ui::*;
use leptos::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn dummy() {
    ui::app_view();

    let Some(text) = ui::find_text() else {
        panic!("Text not found");
    };
    // let runtime = create_runtime();

    assert_eq!(text.text_content().unwrap(), "hello");

    // runtime.dispose();
}
