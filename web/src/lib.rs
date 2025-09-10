use crate::content::global_content_service;
use crate::content::{get_global_content, get_global_document};
use crate::router::Router;
use content_service::ContentServiceClient;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, window}; // Add this import

pub mod content;
pub mod image;
pub mod page;

mod pages;
mod router;

#[wasm_bindgen(start)]
pub fn main() {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    init_shell(document);
    Router::init();
}

#[wasm_bindgen]
pub fn on_article_card_visible(card_id: &str, card_name: &str, card_path: &str) {
    let card_id = card_id.to_string();
    let card_name = card_name.to_string();
    let card_path = card_path.to_string();

    let url = format!("{}/content{}", get_base_url!(), card_path);

    spawn_local(async move {
        console_log!(
            "Article card '{}' with ID '{}' is now visible!",
            card_name,
            card_id
        );

        match { get_global_document(&url).await } {
            Ok(document) => {
                console_log!("{}", document);
            }
            Err(e) => {
                console_log!("Failed to load content for '{}': {:?}", card_name, e);
            }
        }
    });
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
    app.set_attribute("class", "app");

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setupArticleObserver)]
    fn setup_article_observer();
}

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

#[macro_export]
macro_rules! get_document {
    () => {{
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        document
    }};
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

#[macro_export]
macro_rules! get_base_url {
    () => {{
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        // origin is the base URL of the site, e.g. "https://example.com"
        location
            .origin()
            .expect("Couldn't get site base url")
            .to_string()
    }};
}
