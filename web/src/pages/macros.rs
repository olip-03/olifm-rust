use crate::console_log;
use crate::get_base_url;
use crate::global_content_service;
use crate::log;
use content_service::ContentServiceError;
use content_service::models::DirectoryItem;
use futures::join;
use gloo_net::Error;
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

        wasm_bindgen_futures::spawn_local(async move {
            let base = get_base_url!().to_string();
            let doc_url = format!(
                "{}/content/{}/readme.md",
                base, content_path
            );
            match crate::pages::macros::get_page_content(&content_path, &doc_url).await {
                Ok((mut repo_content, document)) => {
                    let mut html = String::new();

                    load_readme(&mut repo_content, &mut html, &document);

                    let div_class = format!("{}-container", &content_path);
                    html.push_str(&format!("<div class=\"{}\">", div_class));

                    for item in repo_content {
                        match item {
                            DirectoryItem::File { name, entry_type, size, .. } => {}
                            DirectoryItem::Image { name, entry_type, size, blurhash, aspect_ratio, .. } => {}
                            DirectoryItem::Directory { name, entry_type, size, path, .. } => {
                                if path.starts_with("/pictures") {
                                    // Render with image-card style if path begins with content_path
                                    html.push_str(&format!(
                                        "<div class=\"image-card\">
                                            <strong>{}</strong> - {} ({})
                                        </div>",
                                        name, entry_type, size
                                    ));
                                } else {
                                    // Fall back to article-card style (or any other)
                                    html.push_str(&format!(
                                        "<div class=\"article-card\"><strong>{}</strong> - {} ({})</div>",
                                        name, entry_type, size
                                    ));
                                }
                            }
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
) -> Result<(Vec<DirectoryItem>, String), ContentServiceError> {
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

pub fn load_readme(content: &mut Vec<DirectoryItem>, html: &mut String, document: &String) {
    html.push_str("<div class=\"page-title\">");
    if let Some(pos) = content.iter().position(|item| match item {
        DirectoryItem::File { name, .. }
        | DirectoryItem::Directory { name, .. }
        | DirectoryItem::Image { name, .. } => name.to_lowercase() == "readme.md",
    }) {
        let readme = content.remove(pos);
        html.push_str(&document);
    } else {
        console_log!("No readme found");
        html.push_str(&document);
    }
    html.push_str("</div>");
}

async fn fetch_directory_structure() -> Result<Vec<DirectoryItem>, String> {
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

    let items: Vec<DirectoryItem> =
        serde_json::from_str(&text).map_err(|e| format!("Parse error: {}", e))?;
    Ok(items)
}
