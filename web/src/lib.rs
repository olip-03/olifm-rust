use js_sys::Promise;
use shared::github_service;
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

    // Create a simple greeting element
    let greeting = document
        .create_element("p")
        .expect("Failed to create element");
    greeting.set_text_content(Some("Hello from Rust and WebAssembly!"));
    greeting
        .set_attribute("style", "color: red; font-size: 20px; margin: 10px;")
        .expect("Failed to set style");

    body.append_child(&greeting)
        .expect("Failed to append child");

    // Create GitHub service info element
    let info = document
        .create_element("p")
        .expect("Failed to create element");
    info.set_text_content(Some(
        "GitHub Service initialized (async calls not supported in WASM start function)",
    ));
    info.set_attribute("style", "color: blue; font-size: 16px; margin: 10px;")
        .expect("Failed to set style");

    body.append_child(&info).expect("Failed to append child");

    console_log!("DOM elements added successfully!");
}

// WASM-compatible GitHub service functions that return JavaScript promises
#[wasm_bindgen]
pub fn get_github_user(username: &str) -> Promise {
    let username = username.to_string();
    wasm_bindgen_futures::future_to_promise(async move {
        let github = github_service();
        match github.get_user(&username).await {
            Ok(user) => {
                let result =
                    js_sys::JSON::stringify(&serde_wasm_bindgen::to_value(&user).unwrap()).unwrap();
                Ok(result.into())
            }
            Err(_) => {
                let error = JsValue::from_str("Failed to fetch GitHub user");
                Err(error)
            }
        }
    })
}

#[wasm_bindgen]
pub fn get_github_repo(owner: &str, repo: &str) -> Promise {
    let owner = owner.to_string();
    let repo = repo.to_string();
    wasm_bindgen_futures::future_to_promise(async move {
        let github = github_service();
        match github.get_repo(&owner, &repo).await {
            Ok(repo_data) => {
                let result =
                    js_sys::JSON::stringify(&serde_wasm_bindgen::to_value(&repo_data).unwrap())
                        .unwrap();
                Ok(result.into())
            }
            Err(_) => {
                let error = JsValue::from_str("Failed to fetch GitHub repository");
                Err(error)
            }
        }
    })
}

#[wasm_bindgen]
pub fn get_github_api_url() -> String {
    let github = github_service();
    github.get_url().to_string()
}
