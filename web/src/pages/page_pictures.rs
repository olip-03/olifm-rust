use crate::get_app;
use crate::get_base_url;
use crate::image::get_base64_from_blurhash;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::Style;
use crate::pages::macros::load_readme;
use crate::render_site;
use content_service::JsonEntry;
use std::collections::HashMap;
pub fn page_pictures() -> PageType {
    let mut params = HashMap::new();
    let render = |p: &PageType| "".to_string();

    let on_after_render = || {
        render_site!("pictures", Style::Photo);
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}

pub fn page_pictures_card_html(item: JsonEntry) -> String {
    let base = get_base_url!().to_string();
    let mut html = String::new();
    let first_img = item.images.first().unwrap();
    let blurhash = &first_img.blurhash;
    let base64 = get_base64_from_blurhash(&blurhash);

    let img_url = format!("{}/content{}", base, first_img.path);
    let img_style = format!("aspect-ratio: {};", first_img.aspect_ratio);

    let wrapper_style = format!("aspect-ratio: {};", first_img.aspect_ratio);

    let img_blur = format!(
        "<img class=\"photo-card-blur\" src=\"data:image/bmp;base64,{}\" alt=\"blurred image\" />",
        base64
    );
    let img_main = format!(
        "<img class=\"photo-card-img\" src=\"{}\" alt=\"{}\" />",
        img_url, item.name
    );

    html.push_str(&format!(
        "<div class=\"photo-card\">
            <div class=\"photo-card-img-wrap\" style=\"{}\">
                {}
                {}
            </div>
            <strong>{}</strong>
        </div>",
        wrapper_style, img_blur, img_main, item.name
    ));
    html
}
