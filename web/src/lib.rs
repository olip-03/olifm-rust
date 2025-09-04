use wasm_bindgen::prelude::*;
use web_sys::{Element, window};

mod router;
use router::Router;

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

fn get_app_container() -> Element {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    if let Some(app) = document.get_element_by_id("app") {
        app
    } else {
        let body = document.body().expect("document should have a body");
        let app = document
            .create_element("div")
            .expect("Failed to create app div");
        app.set_id("app");
        body.append_child(&app)
            .expect("Failed to append app container");
        app
    }
}

pub fn clear_content() {
    let app = get_app_container();
    app.set_inner_html("");
}

pub fn render_home() {
    clear_content();
    let app = get_app_container();

    let content = "<h1>Welcome</h1>\
        <p>This site's a little bare right now... We'll get back to you later</p>\
        <nav>\
            <a href=\"#/documents\">Documents</a> |\
            <a href=\"#/about\">About</a>\
        </nav>";

    app.set_inner_html(content);
    console_log!("Rendered home page");
}

pub fn render_documents() {
    clear_content();
    let app = get_app_container();

    let content = "<h1>Documents</h1>\
        <p>Documents page - functionality coming soon!</p>\
        <nav><a href=\"#/\">← Back to Home</a></nav>";

    app.set_inner_html(content);
    console_log!("Rendered documents page");
}

pub fn render_about() {
    clear_content();
    let app = get_app_container();

    let content = "<h1>About OliFM</h1>\
        <p>OliFM is a modern, web-based file manager built with:</p>\
        <ul>\
            <li>Rust for the backend logic</li>\
            <li>WebAssembly for running in the browser</li>\
            <li>Client-side routing</li>\
        </ul>\
        <nav><a href=\"#/\">← Back to Home</a></nav>";

    app.set_inner_html(content);
    console_log!("Rendered about page");
}

pub fn render_404() {
    clear_content();
    let app = get_app_container();

    let content = "<h1>404 - Page Not Found</h1>\
        <p>The page you're looking for doesn't exist.</p>\
        <nav><a href=\"#/\">← Go Home</a></nav>";

    app.set_inner_html(content);
    console_log!("Rendered 404 page");
}

// Called when the WASM module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("WASM module loaded successfully!");

    // Initialize router
    Router::init();

    console_log!("Router initialized!");
}
