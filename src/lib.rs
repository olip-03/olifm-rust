use crate::content::{get_global_document, strip_frontmatter};
use crate::router::Router;
use pulldown_cmark::{Parser, html};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, window};

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
pub fn on_article_card_visible(card_name: &str, card_path: &str) {
    // let card_id = card_id.to_string();
    let card_name = card_name.to_string();
    let card_path = card_path.to_string();

    let url = format!("{}/content{}", get_base_url!(), card_path);

    spawn_local(async move {
        match { get_global_document(&url).await } {
            Ok(markdown_content) => {
                let fixed_content = strip_frontmatter(&markdown_content);

                let parser = Parser::new(&fixed_content);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                let id = format!("content{}", &card_path);
                match get_document!().get_element_by_id(&id) {
                    Some(element) => {
                        element.set_inner_html(&html_output);
                    }
                    None => {
                        // do nun
                    }
                }
            }
            Err(e) => {
                console_log!("Failed to load content for '{}': {:?}", card_name, e);
            }
        }
    });
}

#[wasm_bindgen]
pub fn on_article_card_click(card_path: &str) {
    Router::navigate_to(&card_path);
}

#[wasm_bindgen]
pub fn on_tag_click(tag: &str) {
    console_log!("Tag clicked: {}", tag);
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
    img.set_attribute("src", "assets/radio.mkv0001-0250.gif")
        .expect("Failed to add source to img");
    img.set_attribute("class", "logo")
        .expect("Failed to add class to img");
    body.append_child(&img).expect("Failed to add img");

    // append nav
    let nav = document
        .create_element("div")
        .expect("Failed to create nav div");
    nav.set_attribute("class", "nav")
        .expect("Failed to add class to nav");
    init_nav(&document, &nav);
    body.append_child(&nav).expect("Failed to add navigation");

    // create app container
    let app = document
        .create_element("div")
        .expect("Failed to create app div");
    app.set_id("app");
    app.set_attribute("class", "app")
        .expect("Failed to set class attribute for app");

    body.append_child(&app)
        .expect("Failed to append app container");
}

fn init_nav(document: &web_sys::Document, nav: &web_sys::Element) {
    let home = create_button(&document, "Home", "");
    nav.append_child(&home)
        .expect("Failed to append home button");

    let pictures = create_button(&document, "Pictures", "pictures");
    nav.append_child(&pictures)
        .expect("Failed to append pictures button");

    let sounds = create_button(&document, "Sounds", "sounds");
    nav.append_child(&sounds)
        .expect("Failed to append sounds button");

    let about = create_button(&document, "About", "about");
    nav.append_child(&about)
        .expect("Failed to append about button");
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

#[macro_export]
macro_rules! get_full_url {
    () => {{
        let window = web_sys::window().expect("no global window exists");
        let location = window.location();
        // `href` returns the full URL (origin + path + query + fragment)
        location.href().expect("Couldn't get full URL").to_string()
    }};
}
