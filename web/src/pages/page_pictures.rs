use crate::console_log;
use crate::content::parse_debug_sequence;
use crate::get_app;
use crate::get_base_url;
use crate::image::get_base64_from_blurhash;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::{Style, get_page_tags, load_readme};
use crate::render_site;
use crate::setup_article_observer;
use content_service::JsonEntry;
use pulldown_cmark::{Parser, html};
use std::collections::HashMap;

pub fn page_pictures() -> PageType {
    let params = HashMap::new();
    let render = |_: &PageType| "".to_string();

    let on_after_render = || {
        render_site!("pictures", Style::Photo);
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}

pub fn page_pictures_card_html(item: JsonEntry) -> String {
    let base = get_base_url!().to_string();

    let mut html = String::new();
    if item.images.is_empty() {
        console_log!("No images available, returning empty HTML");
        return html;
    }

    let first_img = item.images.first().unwrap();
    let blurhash = &first_img.blurhash;

    let base64 = get_base64_from_blurhash(&blurhash);

    let img_url = format!("{}/content{}", base, first_img.path);
    let wrapper_style = format!("aspect-ratio: {};", first_img.aspect_ratio);

    let img_blur = format!(
        "<img class=\"photo-card-blur\" src=\"data:image/bmp;base64,{}\" alt=\"blurred image\" />",
        base64
    );
    let img_main = format!(
        "<img class=\"photo-card-img\" src=\"{}\" alt=\"{}\" loading=\"lazy\" onload=\"this.style.opacity=1\"/>",
        img_url, item.name
    );

    let card_id = format!("card-{}", item.name.replace(" ", "-").to_lowercase());
    let mut item_date = String::new();
    if let Some(date) = item.metadata.get("date") {
        item_date = date.clone();
    }

    html.push_str(&format!(
        "<div class=\"base-card photo-card\" data-card-id=\"{}\" data-card-name=\"{}\" data-card-path=\"{}\" onclick=\"on_article_card_click('{}')\" style=\"cursor: pointer;\">
            <div class=\"photo-card-img-wrap\" style=\"{}\">
                {}
                {}
            </div>
            <div class=\"text-inline\">
                <strong>{}</strong> • {}
            </div>",
        card_id,
        item.name,
        item.path,
        item.path,
        wrapper_style,
        img_blur,
        img_main,
        item.name,
        item_date
    ));

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

    html.push_str("</div>");
    html
}
