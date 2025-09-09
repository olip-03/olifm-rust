use crate::console_log;
use crate::get_base_url;
use crate::global_content_service;
use crate::log;
use content_service::ContentServiceError;
use content_service::{Img, JsonEntry};
use futures::join;
use gloo_net::Error;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub enum Style {
    Card,
    Photo,
    Music,
}

#[macro_export]
macro_rules! render_site {
    ($path:expr, $style:expr) => {{
        let content_path = $path.to_string();
        let style = $style;

        wasm_bindgen_futures::spawn_local(async move {
            let base = get_base_url!().to_string();
            let doc_url = format!("{}/content/{}/readme.md", base, content_path);
            match crate::pages::macros::get_page_content(&content_path, &doc_url).await {
                Ok((mut repo_content, document)) => {
                    let mut html = String::new();

                    load_readme(&mut repo_content, &mut html, &document);

                    let div_class = format!("{}-container", &content_path);
                    html.push_str(&format!("<div class=\"{}\">", div_class));

                    for item in repo_content {
                        match style {
                            // TODO: Custom music card implementation
                            Style::Card | Style::Music => html
                                .push_str(&crate::pages::page_sounds::page_sounds_card_html(item)),
                            Style::Photo => html.push_str(
                                &crate::pages::page_pictures::page_pictures_card_html(item),
                            ),
                        }
                    }
                    html.push_str("</div>");

                    get_app!().set_inner_html(&html);
                }
                Err(e) => {
                    crate::console_log!("Error fetching directory data: {:?}", e);
                    get_app!().set_inner_html(&format!(
                        "<p style=\"color: red;\">Error loading content: {}</p>",
                        e
                    ));
                }
            }
        });
    }};
}

pub async fn get_page_content(
    _path: &str,
    doc_url: &str,
) -> Result<(Vec<JsonEntry>, String), ContentServiceError> {
    console_log!("Fetching content from {}", _path);
    let client = global_content_service();

    let path = format!("/{}", _path);
    let items = client
        .clone()
        .get_content(path, Some("file".to_string()))
        .await?;

    let doc_resp = gloo_net::http::Request::get(doc_url)
        .header("User-Agent", "olifm-rust/1.0")
        .send()
        .await
        .map_err(|e| {
            ContentServiceError::NetworkError(format!("Failed to read response text: {}", e))
        })?;

    if !doc_resp.ok() {
        return Err(ContentServiceError::NetworkError(format!(
            "HTTP error: {}",
            doc_resp.status()
        )));
    }

    let document = doc_resp.text().await.map_err(|e| {
        ContentServiceError::NetworkError(format!("Failed to read response text: {}", e))
    })?;

    Ok((items, document))
}

pub fn load_readme(content: &mut Vec<JsonEntry>, html: &mut String, document: &String) {
    html.push_str("<div class=\"page-title\">");
    if let Some(pos) = content
        .iter()
        .position(|item| item.name.to_lowercase() == "readme.md")
    {
        let readme = content.remove(pos);
        html.push_str(&document);
    } else {
        console_log!("No readme found");
        html.push_str(&document);
    }
    html.push_str("</div>");
}

async fn fetch_directory_structure() -> Result<Vec<JsonEntry>, String> {
    let site_url = get_base_url!().to_string();
    let url = format!("{}/directory_structure.json", site_url);
    let resp = gloo_net::http::Request::get(&url)
        .header("User-Agent", "olifm-rust/1.0")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let text = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {:?}", e))?;

    let items: Vec<JsonEntry> =
        serde_json::from_str(&text).map_err(|e| format!("Parse error: {}", e))?;
    Ok(items)
}
