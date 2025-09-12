use crate::console_log;
use crate::get_base_url;
use crate::image::get_base64_from_blurhash;
use crate::log;

use content_service::{ContentServiceClient, ContentServiceError, Img, JsonEntry};
use futures::lock::Mutex;
use std::sync::{Arc, LazyLock};

static GLOBAL_CONTENT_CLIENT: LazyLock<Arc<Mutex<ContentServiceClient>>> = LazyLock::new(|| {
    let url = get_base_url!();
    Arc::new(Mutex::new(ContentServiceClient::with_base_url(url)))
});

pub async fn get_global_content(
    path: String,
    filter: Option<String>,
) -> Result<Vec<JsonEntry>, ContentServiceError> {
    let client = GLOBAL_CONTENT_CLIENT.clone();
    let mut client_ref = client.lock().await;
    client_ref.get_content(path, filter).await
}

pub async fn get_global_tags(path: String) -> Result<Vec<String>, ContentServiceError> {
    let client = GLOBAL_CONTENT_CLIENT.clone();
    let mut client_ref = client.lock().await;
    client_ref.get_tags(path).await
}

pub async fn get_global_document(path: &str) -> Result<String, ContentServiceError> {
    let client = GLOBAL_CONTENT_CLIENT.clone();
    let mut client_ref = client.lock().await;
    client_ref.get_document(path).await
}

pub fn global_content_service() -> Arc<Mutex<ContentServiceClient>> {
    GLOBAL_CONTENT_CLIENT.clone()
}

pub fn strip_frontmatter(content: &str) -> &str {
    let trimmed = content.trim_start();
    if trimmed.starts_with("---") {
        if let Some(end_pos) = trimmed[3..].find("---") {
            let after_frontmatter = &trimmed[end_pos + 6..];
            return after_frontmatter.trim_start();
        }
    }
    content
}

pub fn replace_images(content: &str, images: &[Img]) -> String {
    let mut result = content.to_string();

    for img in images {
        let pattern = format!("![[{}]]", img.name);
        let img_url = format!("{}/content{}", get_base_url!(), img.path);

        let base64 = get_base64_from_blurhash(&img.blurhash);

        let html_img = format!(
            r#"<div class="article-image-wrap" style="aspect-ratio: {};">
                <img class="article-image-blur" src="data:image/bmp;base64,{}" alt="blurred image" />
                <img class="photo-card-img article-image" src="{}" alt="{}" loading="lazy" onload="this.style.opacity=1" />
            </div>"#,
            img.aspect_ratio, base64, img_url, img.name
        );

        result = result.replace(&pattern, &html_img);
    }

    result
}

pub async fn get_entry_by_path(path: &str) -> Option<JsonEntry> {
    match get_global_content("".to_string(), None).await {
        Ok(content) => content
            .into_iter()
            .find(|item| item.path.to_lowercase() == path.to_lowercase()),
        Err(e) => {
            console_log!("Failed to load content: {:?}", e);
            None
        }
    }
}

pub fn parse_debug_sequence(debug_str: &str) -> String {
    use regex::Regex;

    let re = Regex::new(r#"String\("([^"]*)"\)"#).unwrap();
    let items: Vec<&str> = re
        .captures_iter(debug_str)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect();

    if items.is_empty() {
        debug_str.to_string()
    } else {
        items.join(" • ")
    }
}
