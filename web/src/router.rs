use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Event, window};

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
        match path {
            "/" | "/home" => crate::render_home(),
            "/documents" => crate::render_documents(),
            "/about" => crate::render_about(),
            _ => crate::render_404(),
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
