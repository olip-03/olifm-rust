use crate::error::ContentServiceError;
use crate::models::{DirectoryEntry, DirectoryItem, FileEntry, ImageEntry};
use gloo_net::http::Request;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

// User-Agent string for API requests
const USER_AGENT: &str = "olifm-rust/1.0";

// The in-memory maps exposed publicly
#[derive(Debug, Clone)]
pub struct ContentServiceClient {
    base_url: String,
    pub directories: HashMap<String, DirectoryEntry>,
    pub files: HashMap<String, FileEntry>,
    pub images: HashMap<String, ImageEntry>,
}

impl ContentServiceClient {
    pub fn directory_structure_url(&self) -> String {
        format!("{}/directory_structure.json", self.base_url)
    }

    pub fn new() -> Self {
        Self {
            base_url: "https://oli.fm".to_string(),
            directories: HashMap::new(),
            files: HashMap::new(),
            images: HashMap::new(),
        }
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            directories: HashMap::new(),
            files: HashMap::new(),
            images: HashMap::new(),
        }
    }

    /// Get the base URL being used
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Internal: fetch and parse the directory structure from the static JSON
    async fn fetch_directory_structure(&self) -> Result<Vec<DirectoryItem>, ContentServiceError> {
        let url = self.directory_structure_url();
        let resp = Request::get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        if !resp.ok() {
            return Err(ContentServiceError::NetworkError(format!(
                "HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp.text().await.map_err(|e| {
            ContentServiceError::NetworkError(format!("Failed to read response text: {:?}", e))
        })?;

        let items: Vec<DirectoryItem> =
            serde_json::from_str(&text).map_err(ContentServiceError::ParseError)?;
        Ok(items)
    }

    pub async fn get_content(
        &mut self,
        path: String,
        filter: Option<String>,
    ) -> Result<Vec<DirectoryItem>, ContentServiceError> {
        // Fetch directory structure JSON
        let items = self.fetch_directory_structure().await?;

        // Filter items
        let filtered_items: Vec<DirectoryItem> = items
            .into_iter()
            .filter(|item| {
                // Check path prefix
                let item_path = match item {
                    DirectoryItem::Directory { path, .. } => path,
                    DirectoryItem::File { path, .. } => path,
                    DirectoryItem::Image { path, .. } => path,
                };
                if !item_path.starts_with(&path) {
                    return false;
                }

                // Check optional type filter
                if let Some(ref f) = filter {
                    let item_type = match item {
                        DirectoryItem::Directory { entry_type, .. } => entry_type.as_str(),
                        DirectoryItem::File { entry_type, .. } => entry_type.as_str(),
                        DirectoryItem::Image { entry_type, .. } => entry_type.as_str(),
                    };
                    if item_type != f.as_str() {
                        return false;
                    }
                }
                true
            })
            .collect();

        Ok(filtered_items)
    }

    pub async fn get_document(&self, path: &str) -> Result<String, ContentServiceError> {
        if path.trim().is_empty() {
            return Err(ContentServiceError::InvalidInput(
                "Path cannot be empty".to_string(),
            ));
        }
        // For compatibility, fetch the static JSON as document (optional)
        let _ = self.fetch_directory_structure().await?;
        // Not returning the full content here; in a real implementation you might fetch
        // and return specific docs. For now, provide a simple placeholder.
        Ok(String::new())
    }
}

/// Callback-based content service API client for WASM compatibility
#[derive(Debug, Clone)]
pub struct ContentServiceClientCallback {
    inner: ContentServiceClient,
}

impl Default for ContentServiceClientCallback {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentServiceClientCallback {
    /// Create a new callback-based content service client with the default API URL
    pub fn new() -> Self {
        Self {
            inner: ContentServiceClient::new(),
        }
    }

    /// Create a new callback-based content service client with a custom base URL
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            inner: ContentServiceClient::with_base_url(base_url),
        }
    }

    /// Get the base URL being used
    pub fn base_url(&self) -> &str {
        self.inner.base_url()
    }

    /// Fetch directory content by owner, repo name, and path with callback
    /// Added parameter: filter to apply on the loaded directory structure
    pub fn get_content<F>(&self, path: String, filter: Option<String>, callback: F)
    where
        F: FnOnce(Result<Vec<DirectoryItem>, ContentServiceError>) + 'static,
    {
        let mut inner = self.inner.clone();

        spawn_local(async move {
            let res = inner.get_content(path, filter).await;
            callback(res);
        });
    }

    pub fn get_document<F>(&self, path: &str, callback: F)
    where
        F: FnOnce(Result<String, ContentServiceError>) + 'static,
    {
        let inner = self.inner.clone();
        let doc_path = path.to_string();
        spawn_local(async move {
            let result = inner.get_document(&doc_path).await;
            callback(result);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ContentServiceClient::new();
        assert_eq!(client.base_url(), "https://oli.fm");
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let client = ContentServiceClient::with_base_url("https://custom.contentservice.com");
        assert_eq!(client.base_url(), "https://custom.contentservice.com");
    }
}
