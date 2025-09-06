use crate::page::Page as PageType;
use crate::pages::{about_page, home_page, not_found_page};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Simple function-based router that avoids static mut issues
pub struct Router;

impl Router {
    pub fn init() {
        // Set up event listener for hash changes
        Self::setup_hash_listener();
        // Handle initial route
        Self::handle_current_route();
    }

    pub fn navigate_to(path: &str) {
        if let Some(window) = window() {
            let location = window.location();
            let _ = location.set_hash(&format!("#{}", path));
        }
    }

    fn handle_route(path: &str) {
        log(&format!("Routing to: {}", path));
        let page = match path {
            "/" | "/home" => home_page::home_page(),
            "/about" => about_page::about_page(),
            _ => not_found_page::not_found_page(),
        };

        Self::render(page);
    }

    fn render(page: PageType) {
        let window = window().expect("Cannot render: no global 'window' exists!");
        let document = window
            .document()
            .expect("Cannot render: window should have a document!");
        if let Some(app) = document.get_element_by_id("app") {
            let content = page.to_html();
            app.set_inner_html(&content);
        } else {
            log(&format!("Could not render content for page {}", page.name));
        }
    }

    fn handle_current_route() {
        if let Some(window) = window() {
            let location = window.location();
            if let Ok(hash) = location.hash() {
                let path = if hash.starts_with('#') {
                    &hash[1..]
                } else {
                    &hash
                };

                let route = if path.is_empty() { "/" } else { path };
                Self::handle_route(route);
            } else {
                Self::handle_route("/");
            }
        }
    }

    fn setup_hash_listener() {
        if let Some(window) = window() {
            let closure = Closure::wrap(Box::new(move |_event: Event| {
                Self::handle_current_route();
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
                .expect("should register hashchange listener");

            closure.forget();
        }
    }
}
