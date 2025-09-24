use crate::console_log;
use crate::page::Page as PageType;
use crate::pages::{
    page_about, page_document, page_home, page_not_found, page_pictures, page_sounds,
};
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
        let mut path = path;
        let mut tags = String::new();
        if let Some(query) = path.split_once('?') {
            path = query.0;
        }
        // set_params.insert("tags".to_string(), tags);

        // match page
        let mut page = match path {
            "/" | "/home" => page_home::page_home(),
            "/about" | "resume" => page_about::page_about(),
            "/pictures" => page_pictures::page_pictures(),
            "/sounds" => page_sounds::page_sounds(),
            _ => {
                // Check for ? for query parameters
                if let Some(query) = path.split_once('?') {
                    console_log!("Query parameters: {:?}", query);
                    page_not_found::page_not_found()
                } else {
                    // Check for wildcard routes
                    if let Some(_) = Self::extract_wildcard(path, "/blog/") {
                        page_document::page_document(&path)
                    } else if let Some(_) = Self::extract_wildcard(path, "/pictures/") {
                        page_document::page_document(&path)
                    } else if let Some(_) = Self::extract_wildcard(path, "/resume/") {
                        page_document::page_document(&path)
                    } else {
                        page_not_found::page_not_found()
                    }
                }
            }
        };
        Self::render(page);
    }

    fn extract_params(path: &mut str) -> Vec<String> {
        let mut params = Vec::new();
        let mut parts = path.split('/');
        while let Some(part) = parts.next() {
            if part.starts_with(':') {
                params.push(part[1..].to_string());
            }
        }
        params
    }

    fn extract_wildcard(path: &str, prefix: &str) -> Option<String> {
        if path.starts_with(prefix) && path.len() > prefix.len() {
            Some(path[prefix.len()..].to_string())
        } else {
            None
        }
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
