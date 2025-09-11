use crate::get_app;
use crate::get_base_url;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::Style;
use crate::pages::macros::load_readme;
use crate::render_site;
use crate::setup_article_observer;
use content_service::JsonEntry;
use pulldown_cmark::{Parser, html};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub fn page_home() -> PageType {
    // todo: loading spinner
    let mut params = HashMap::new();
    let render = |p: &PageType| "loading contents...".to_string();

    let on_after_render = || {
        render_site!("blog", Style::Card);
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}

pub fn page_home_card_html(item: JsonEntry) -> String {
    let card_id = format!("card-{}", item.name.replace(" ", "-").to_lowercase());
    let mut html = String::new();
    html.push_str(&format!(
        "<div class=\"base-card article-card\"
              data-card-id=\"{}\"
              data-card-name=\"{}\"
              data-card-path=\"{}\"
              onclick=\"on_article_card_click('{}', '{}')\">
            <strong>{}</strong> - {} ({})
            <div id=\"content{}\">
                Just a test for now. See how things look
            </div>
        </div>",
        card_id,
        item.name,
        item.path,
        item.name.replace("'", "\\'"),
        item.path.replace("'", "\\'"),
        item.name,
        item.entry_type,
        item.size,
        item.path
    ));
    html
}
