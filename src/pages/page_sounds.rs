use crate::get_app;
use crate::get_base_url;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::load_readme;
use crate::pages::macros::Style;
use crate::render_site;
use crate::setup_article_observer;
use pulldown_cmark::{html, Parser};
use std::collections::HashMap;

pub fn page_sounds() -> PageType {
    let params = HashMap::new();
    let render = |_: &PageType| "".to_string();

    let on_after_render = || {
        render_site!("sounds", Style::Music);
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}
