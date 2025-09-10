use crate::get_base_url;
use content_service::{ContentServiceClient, ContentServiceError, JsonEntry};
use futures::lock::Mutex; // This is the async-compatible Mutex
use std::sync::{Arc, LazyLock}; // Remove Mutex from here
static GLOBAL_CONTENT_CLIENT: LazyLock<Arc<Mutex<ContentServiceClient>>> = LazyLock::new(|| {
    let url = get_base_url!();
    Arc::new(Mutex::new(ContentServiceClient::with_base_url(url)))
});

pub async fn get_global_content(
    path: String,
    filter: Option<String>,
) -> Result<Vec<JsonEntry>, ContentServiceError> {
    let client = GLOBAL_CONTENT_CLIENT.clone();
    let mut client_ref = client.lock().await; // Now async-compatible
    client_ref.get_content(path, filter).await
}

pub async fn get_global_document(path: &str) -> Result<String, ContentServiceError> {
    let client = GLOBAL_CONTENT_CLIENT.clone();
    let mut client_ref = client.lock().await; // Now async-compatible
    client_ref.get_document(path).await
}

// Keep this for backward compatibility if needed
pub fn global_content_service() -> Arc<Mutex<ContentServiceClient>> {
    GLOBAL_CONTENT_CLIENT.clone()
}
