use crate::get_app;
use crate::page::Page as PageType;
use std::collections::HashMap;
pub fn page_about() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "About OliFM".to_string());
    params.insert(
        "description".to_string(),
        "Blogging site written in Rust, running on Github.".to_string(),
    );

    let render = |p: &PageType| {
        let title = p.params.get("title").map(|s| s.as_str()).unwrap_or("About");
        let description = p
            .params
            .get("description")
            .map(|s| s.as_str())
            .unwrap_or("");

        format!(
            "<h1>{}</h1>\
            <p>{}</p>\
            <p>I've built this site with:</p>\
            <ul>\
                <li>Rust for the backend logic</li>\
                <li>Client-side routing</li>\
                <li>WebAssembly running the show</li>\
            </ul>\
            <nav><a href=\"#/\">← Back to Home</a></nav>",
            title, description
        )
    };

    PageType::new("About", params, render)
}
