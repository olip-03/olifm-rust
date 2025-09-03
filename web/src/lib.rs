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

// Called when the WASM module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("WASM module loaded successfully!");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Create a simple greeting paragraph
    let greeting = document
        .create_element("p")
        .expect("Failed to create element");
    greeting.set_text_content(Some("Under Construction! Check back soon :)"));
    greeting
        .set_attribute("style", "color: red; font-size: 20px; margin: 10px;")
        .expect("Failed to set style");

    body.append_child(&greeting)
        .expect("Failed to append child");

    console_log!("DOM element added successfully!");
}
