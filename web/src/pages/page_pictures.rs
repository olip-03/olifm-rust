use crate::get_app;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::Style;
use crate::render_site;
use github::GithubClientCallback;
use std::collections::HashMap;

pub fn page_pictures() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "Welcome to oli.fm".to_string());

    let render = |p: &PageType| {
        let title = p
            .params
            .get("title")
            .map(|s| s.as_str())
            .unwrap_or("Untitled");
        title.to_string()
    };

    let on_after_render = || {
        render_site!("olip-03", "oli-fm-content", "pictures", Style::Photo);
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}
