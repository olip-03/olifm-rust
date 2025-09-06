use wasm_bindgen::prelude::*;
use web_sys::{Element, window};

mod page;
mod pages;
mod router;

use router::Router;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

#[macro_export]
macro_rules! get_app {
    () => {{
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        document
            .get_element_by_id("app")
            .expect("document should have element with id 'app'")
    }};
}

#[macro_export]
macro_rules! clear_app {
    () => {{
        let app = get_app!();
        app.set_inner_html("");
    }};
}

#[wasm_bindgen(start)]
pub fn main() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    init_shell(document);
    Router::init();
}

fn init_shell(document: web_sys::Document) {
    let body = document.body().expect("document should have a body");

    // remove loading
    if let Some(element) = document.get_element_by_id("loading") {
        body.remove_child(&element)
            .expect("Failed to remove element");
    }

    // append image
    let img = document
        .create_element("img")
        .expect("Failed to create img element");
    img.set_attribute("src", "assets/radio.mkv0001-0250.gif");
    img.set_attribute("class", "logo");
    body.append_child(&img);

    // append nav
    let nav = document
        .create_element("div")
        .expect("Failed to create nav div");
    nav.set_attribute("class", "nav");
    init_nav(&document, &nav);
    body.append_child(&nav);

    // create app container
    let app = document
        .create_element("div")
        .expect("Failed to create app div");
    app.set_id("app");
    body.append_child(&app)
        .expect("Failed to append app container");
}

fn init_nav(document: &web_sys::Document, nav: &web_sys::Element) {
    let home = create_button(&document, "Home", "");
    nav.append_child(&home);

    let pictures = create_button(&document, "Pictures", "pictures");
    nav.append_child(&pictures);

    let sounds = create_button(&document, "Sounds", "sounds");
    nav.append_child(&sounds);

    let about = create_button(&document, "About", "about");
    nav.append_child(&about);
}

fn create_button(document: &web_sys::Document, name: &str, page: &str) -> Element {
    let btn = document
        .create_element("a")
        .expect("Failed to create img element");
    btn.set_inner_html(name);
    let href_value = format!("#/{}", page.to_string());
    btn.set_attribute("href", href_value.as_str())
        .expect("Failed to set href attribute");
    btn
}
