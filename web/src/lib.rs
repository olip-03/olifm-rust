use shared::{add, getJson, indirect_fn_access, pub_func};
use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("WASM module loaded successfully!");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Get all documents from the Content Service
    let json = getJson();

    // Create a simple greeting paragraph
    let result = add(10, 20);
    let txt = format!("Number is {result}");

    let greeting = document
        .create_element("p")
        .expect("Failed to create element");
    greeting.set_text_content(Some(&txt));
    greeting
        .set_attribute("style", "color: red; font-size: 20px; margin: 10px;")
        .expect("Failed to set style");

    body.append_child(&greeting)
        .expect("Failed to append child");

    console_log!("DOM element added successfully!");
}
