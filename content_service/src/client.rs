use crate::error::ContentServiceError;
use crate::models::{Img, JsonEntry};
use gloo_net::http::Request;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

// User-Agent string for API requests
const USER_AGENT: &str = "olifm-rust/1.0";

// The in-memory map exposed publicly
#[derive(Debug, Clone)]
pub struct ContentServiceClient {
    base_url: String,
    pub files: HashMap<String, JsonEntry>,
}

impl ContentServiceClient {
    pub fn directory_structure_url(&self) -> String {
        format!("{}/directory_structure.json", self.base_url)
    }

    pub fn new() -> Self {
        Self {
            base_url: "https://oli.fm".to_string(),
            files: HashMap::new(),
        }
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            files: HashMap::new(),
        }
    }

    /// Get the base URL being used
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Internal: fetch and parse the directory structure from the static JSON
    async fn fetch_directory_structure(&self) -> Result<Vec<JsonEntry>, ContentServiceError> {
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

        let items: Vec<JsonEntry> =
            serde_json::from_str(&text).map_err(ContentServiceError::ParseError)?;
        Ok(items)
    }

    pub async fn get_content(
        &mut self,
        path: String,
        filter: Option<String>,
    ) -> Result<Vec<JsonEntry>, ContentServiceError> {
        // Fetch directory structure JSON
        let items = self.fetch_directory_structure().await?;

        // Update internal files map
        for item in &items {
            self.files.insert(item.path.clone(), item.clone());
        }

        // Filter items
        let mut filtered_items: Vec<JsonEntry> = items
            .into_iter()
            .filter(|item| {
                // Check path prefix
                if !item.path.starts_with(&path) {
                    return false;
                }

                // Check optional type filter
                if let Some(ref f) = filter {
                    if item.entry_type != *f {
                        return false;
                    }
                }
                true
            })
            .collect();

        // Sort by date (newest first)
        filtered_items.sort_by(|a, b| {
            match (&a.date, &b.date) {
                (Some(date_a), Some(date_b)) => {
                    // Sort in descending order (newest first)
                    date_b.cmp(date_a)
                }
                (Some(_), None) => {
                    // Items with dates come before items without dates
                    std::cmp::Ordering::Less
                }
                (None, Some(_)) => {
                    // Items without dates come after items with dates
                    std::cmp::Ordering::Greater
                }
                (None, None) => {
                    // If both have no date, sort by name
                    a.name.cmp(&b.name)
                }
            }
        });

        Ok(filtered_items)
    }

    /// Get all images from files that contain them (sorted by date)
    pub fn get_all_images(&self) -> Vec<&Img> {
        let mut files_with_images: Vec<&JsonEntry> = self
            .files
            .values()
            .filter(|file| !file.images.is_empty())
            .collect();

        // Sort files by date first
        files_with_images.sort_by(|a, b| match (&a.date, &b.date) {
            (Some(date_a), Some(date_b)) => date_b.cmp(date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.name.cmp(&b.name),
        });

        files_with_images
            .into_iter()
            .flat_map(|file| &file.images)
            .collect()
    }

    /// Get images from a specific file
    pub fn get_file_images(&self, file_path: &str) -> Option<&Vec<Img>> {
        self.files.get(file_path).map(|file| &file.images)
    }

    pub async fn get_document(&self, path: &str) -> Result<String, ContentServiceError> {
        if path.trim().is_empty() {
            return Err(ContentServiceError::InvalidInput(
                "Path cannot be empty".to_string(),
            ));
        }

        // Construct the full URL for the document
        let document_url = if path.starts_with("http://") || path.starts_with("https://") {
            // If path is already a full URL, use it as-is
            path.to_string()
        } else {
            // Otherwise, construct URL with base_url
            let clean_path = path.trim_start_matches('/');
            format!("{}/{}", self.base_url, clean_path)
        };

        // Make the HTTP request
        let resp = Request::get(&document_url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        if !resp.ok() {
            return Err(ContentServiceError::NetworkError(format!(
                "HTTP error {}: Failed to fetch document from {}",
                resp.status(),
                document_url
            )));
        }

        // Get the response text (markdown content)
        let markdown_content = resp.text().await.map_err(|e| {
            ContentServiceError::NetworkError(format!("Failed to read document content: {:?}", e))
        })?;

        Ok(markdown_content)
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

    /// Fetch directory content by path with callback (results sorted by date)
    /// Added parameter: filter to apply on the loaded directory structure
    pub fn get_content<F>(&self, path: String, filter: Option<String>, callback: F)
    where
        F: FnOnce(Result<Vec<JsonEntry>, ContentServiceError>) + 'static,
    {
        let mut inner = self.inner.clone();

        spawn_local(async move {
            let res = inner.get_content(path, filter).await;
            callback(res);
        });
    }

    /// Get all images from the loaded files (sorted by file date)
    pub fn get_all_images(&self) -> Vec<&Img> {
        self.inner.get_all_images()
    }

    /// Get images from a specific file
    pub fn get_file_images(&self, file_path: &str) -> Option<&Vec<Img>> {
        self.inner.get_file_images(file_path)
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
