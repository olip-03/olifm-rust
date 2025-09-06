use crate::get_app;
use crate::page::Page as PageType;
use std::collections::HashMap;

pub fn page_not_found() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "404 - Page Not Found".to_string());
    params.insert(
        "message".to_string(),
        "The page you're looking for doesn't exist.".to_string(),
    );

    let render = |p: &PageType| {
        let title = p.params.get("title").map(|s| s.as_str()).unwrap_or("404");
        let message = p
            .params
            .get("message")
            .map(|s| s.as_str())
            .unwrap_or("Page not found");

        format!(
            "<h1>{}</h1>\
            <p>{}</p>\
            <nav><a href=\"#/\">← Go Home</a></nav>",
            title, message
        )
    };

    PageType::new("NotFound", params, render)
}
