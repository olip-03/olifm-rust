use crate::console_log;
use crate::content::parse_debug_sequence;
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

pub fn page_home() -> PageType {
    // todo: loading spinner
    let params = HashMap::new();
    let render = |_: &PageType| "loading contents...".to_string();

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
              onclick=\"on_article_card_click('{}')\">",
        card_id,
        item.name,
        item.path,
        item.path.replace("'", "\\'")
    ));

    // Title section
    html.push_str(&format!("<div><strong>{}</strong>", item.name));

    // Add date if available
    if let Some(date) = item.metadata.get("date") {
        html.push_str(&format!(" • {}", date));
    }

    html.push_str("</div>");

    // Metadata table
    let mut rows = String::new();

    if let Some(medium) = item.metadata.get("medium") {
        rows.push_str(&format!(
            "<tr>
                <td class=\"list-cell\">
                    <img class=\"list-image\" src=\"img/photo_camera.svg\" alt=\"Camera Icon\">
                    <p>{}</p>
                </td>
            </tr>",
            medium
        ));
    }

    if let Some(tags) = item.metadata.get("tags") {
        let formatted_tags = parse_debug_sequence(tags);
        rows.push_str(&format!(
            "<tr>
                <td class=\"list-cell\">
                    <img class=\"list-image\" src=\"img/tag.svg\" alt=\"Tag Icon\">
                    <p>{}</p>
                </td>
            </tr>",
            formatted_tags
        ));
    }

    if !rows.is_empty() {
        html.push_str(&format!("<table class=\"metadata\">{}</table>", rows));
    }

    html.push_str(&format!(
        "<div id=\"content{}\">
        </div>
    </div>",
        item.path
    ));

    html
}
