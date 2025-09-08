use crate::error::ContentServiceError;
use crate::models::ContentPath;
use gloo_net::http::Request;
use serde_json;
use wasm_bindgen_futures::spawn_local;
/// User-Agent string for API requests
const USER_AGENT: &str = "olifm-rust/1.0";

/// Content service API client
#[derive(Debug, Clone)]
pub struct ContentServiceClient {
    base_url: String,
}

impl Default for ContentServiceClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentServiceClient {
    /// Create a new content service client with the default API URL
    pub fn new() -> Self {
        Self {
            base_url: "https://oli.fm".to_string(),
        }
    }

    /// Create a new content service client with a custom base URL (useful for enterprise/self-hosted)
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Get the base URL being used
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Generic function to fetch JSON from a URL
    async fn get_json(&self, url: &str) -> Result<String, ContentServiceError> {
        let response = Request::get(url)
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        if !response.ok() {
            return match response.status() {
                404 => Err(ContentServiceError::NotFound),
                403 => Err(ContentServiceError::RateLimited),
                _ => Err(ContentServiceError::NetworkError(format!(
                    "HTTP error: {}",
                    response.status()
                ))),
            };
        }

        let text = response.text().await.map_err(|e| {
            ContentServiceError::NetworkError(format!("Failed to read response text: {:?}", e))
        })?;

        Ok(text)
    }

    /// Fetch repository content by owner, repo name, and path
    pub async fn get_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Vec<ContentPath>, ContentServiceError> {
        if owner.trim().is_empty() || repo.trim().is_empty() {
            return Err(ContentServiceError::InvalidInput(
                "Owner and repo cannot be empty".to_string(),
            ));
        }

        let url = if path.is_empty() {
            format!("{}/repos/{}/{}/contents", self.base_url, owner, repo)
        } else {
            format!(
                "{}/repos/{}/{}/contents/{}",
                self.base_url, owner, repo, path
            )
        };

        let json_str = self.get_json(&url).await?;

        // Content service API can return either a single object or an array
        // Try to parse as array first, then as single object
        let content: Vec<ContentPath> = match serde_json::from_str::<Vec<ContentPath>>(&json_str) {
            Ok(content_array) => content_array,
            Err(_) => {
                // Try parsing as single object
                let single_content: ContentPath =
                    serde_json::from_str(&json_str).map_err(ContentServiceError::ParseError)?;
                vec![single_content]
            }
        };

        Ok(content)
    }

    pub async fn get_document(&self, path: &str) -> Result<String, ContentServiceError> {
        if path.trim().is_empty() {
            return Err(ContentServiceError::InvalidInput(
                "Path cannot be empty".to_string(),
            ));
        }
        let doc_string = self.get_json(path).await?;
        Ok(doc_string)
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

    /// Fetch repository content by owner, repo name, and path with callback
    pub fn get_content<F>(&self, owner: &str, repo: &str, path: &str, callback: F)
    where
        F: FnOnce(Result<Vec<ContentPath>, ContentServiceError>) + 'static,
    {
        let inner = self.inner.clone();
        let owner = owner.to_string();
        let repo = repo.to_string();
        let path = path.to_string();

        spawn_local(async move {
            let result = inner.get_content(&owner, &repo, &path).await;
            callback(result);
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
