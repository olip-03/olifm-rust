use crate::get_app;
use crate::get_base_url;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::Style;
use crate::pages::macros::load_readme;
use crate::render_site;
use content_service::JsonEntry;
use std::collections::HashMap;

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
    let mut html = String::new();
    html.push_str(&format!(
        "<div class=\"article-card\">
            <strong>{}</strong> - {} ({})
        </div>",
        item.name, item.entry_type, item.size
    ));
    html
}
