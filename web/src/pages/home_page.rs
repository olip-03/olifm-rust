use crate::page::Page as PageType;
use std::collections::HashMap;

pub fn home_page() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "Welcome to oli.fm".to_string());

    let render = |p: &PageType| {
        let title = p
            .params
            .get("title")
            .map(|s| s.as_str())
            .unwrap_or("Untitled");
        format!("<h1>{}</h1>", title)
    };

    PageType::new("Home", params, render)
}
