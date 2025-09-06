use crate::error::GithubError;
use crate::models::{GithubRepo, GithubUser, RepoContent};
use gloo_net::http::Request;
use serde_json;
use wasm_bindgen_futures::spawn_local;

/// User-Agent string for API requests
const USER_AGENT: &str = "olifm-rust/1.0";

/// GitHub API client
#[derive(Debug, Clone)]
pub struct GithubClient {
    base_url: String,
}

impl Default for GithubClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GithubClient {
    /// Create a new GitHub client with the default API URL
    pub fn new() -> Self {
        Self {
            base_url: "https://api.github.com".to_string(),
        }
    }

    /// Create a new GitHub client with a custom base URL (useful for GitHub Enterprise)
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
    async fn get_json(&self, url: &str) -> Result<String, GithubError> {
        let response = Request::get(url)
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        if !response.ok() {
            return match response.status() {
                404 => Err(GithubError::NotFound),
                403 => Err(GithubError::RateLimited),
                _ => Err(GithubError::NetworkError(format!(
                    "HTTP error: {}",
                    response.status()
                ))),
            };
        }

        let text = response.text().await.map_err(|e| {
            GithubError::NetworkError(format!("Failed to read response text: {:?}", e))
        })?;

        Ok(text)
    }

    /// Fetch a GitHub user by username
    pub async fn get_user(&self, username: &str) -> Result<GithubUser, GithubError> {
        if username.trim().is_empty() {
            return Err(GithubError::InvalidInput(
                "Username cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/users/{}", self.base_url, username);
        let json_str = self.get_json(&url).await?;

        let user: GithubUser =
            serde_json::from_str(&json_str).map_err(|e| GithubError::ParseError(e))?;

        Ok(user)
    }

    /// Fetch a GitHub repository by owner and repo name
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<GithubRepo, GithubError> {
        if owner.trim().is_empty() || repo.trim().is_empty() {
            return Err(GithubError::InvalidInput(
                "Owner and repo cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        let json_str = self.get_json(&url).await?;

        let repo: GithubRepo =
            serde_json::from_str(&json_str).map_err(|e| GithubError::ParseError(e))?;

        Ok(repo)
    }

    /// Fetch repository content by owner, repo name, and path
    pub async fn get_repo_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Vec<RepoContent>, GithubError> {
        if owner.trim().is_empty() || repo.trim().is_empty() {
            return Err(GithubError::InvalidInput(
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

        // GitHub API can return either a single object or an array
        // Try to parse as array first, then as single object
        let content: Vec<RepoContent> = match serde_json::from_str::<Vec<RepoContent>>(&json_str) {
            Ok(content_array) => content_array,
            Err(_) => {
                // Try parsing as single object
                let single_content: RepoContent =
                    serde_json::from_str(&json_str).map_err(|e| GithubError::ParseError(e))?;
                vec![single_content]
            }
        };

        Ok(content)
    }

    /// Fetch raw JSON for a user (for backwards compatibility)
    pub async fn get_user_json(&self, username: &str) -> Result<String, GithubError> {
        if username.trim().is_empty() {
            return Err(GithubError::InvalidInput(
                "Username cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/users/{}", self.base_url, username);
        self.get_json(&url).await
    }

    /// Fetch raw JSON for a repository (for backwards compatibility)
    pub async fn get_repo_json(&self, owner: &str, repo: &str) -> Result<String, GithubError> {
        if owner.trim().is_empty() || repo.trim().is_empty() {
            return Err(GithubError::InvalidInput(
                "Owner and repo cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        self.get_json(&url).await
    }
}

/// Callback-based GitHub API client for WASM compatibility
#[derive(Debug, Clone)]
pub struct GithubClientCallback {
    inner: GithubClient,
}

impl Default for GithubClientCallback {
    fn default() -> Self {
        Self::new()
    }
}

impl GithubClientCallback {
    /// Create a new callback-based GitHub client with the default API URL
    pub fn new() -> Self {
        Self {
            inner: GithubClient::new(),
        }
    }

    /// Create a new callback-based GitHub client with a custom base URL
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            inner: GithubClient::with_base_url(base_url),
        }
    }

    /// Get the base URL being used
    pub fn base_url(&self) -> &str {
        self.inner.base_url()
    }

    /// Fetch a GitHub user by username with callback
    pub fn get_user<F>(&self, username: &str, callback: F)
    where
        F: FnOnce(Result<GithubUser, GithubError>) + 'static,
    {
        let inner = self.inner.clone();
        let username = username.to_string();

        spawn_local(async move {
            let result = inner.get_user(&username).await;
            callback(result);
        });
    }

    /// Fetch a GitHub repository by owner and repo name with callback
    pub fn get_repo<F>(&self, owner: &str, repo: &str, callback: F)
    where
        F: FnOnce(Result<GithubRepo, GithubError>) + 'static,
    {
        let inner = self.inner.clone();
        let owner = owner.to_string();
        let repo = repo.to_string();

        spawn_local(async move {
            let result = inner.get_repo(&owner, &repo).await;
            callback(result);
        });
    }

    /// Fetch repository content by owner, repo name, and path with callback
    pub fn get_repo_content<F>(&self, owner: &str, repo: &str, path: &str, callback: F)
    where
        F: FnOnce(Result<Vec<RepoContent>, GithubError>) + 'static,
    {
        let inner = self.inner.clone();
        let owner = owner.to_string();
        let repo = repo.to_string();
        let path = path.to_string();

        spawn_local(async move {
            let result = inner.get_repo_content(&owner, &repo, &path).await;
            callback(result);
        });
    }

    /// Fetch raw JSON for a user with callback
    pub fn get_user_json<F>(&self, username: &str, callback: F)
    where
        F: FnOnce(Result<String, GithubError>) + 'static,
    {
        let inner = self.inner.clone();
        let username = username.to_string();

        spawn_local(async move {
            let result = inner.get_user_json(&username).await;
            callback(result);
        });
    }

    /// Fetch raw JSON for a repository with callback
    pub fn get_repo_json<F>(&self, owner: &str, repo: &str, callback: F)
    where
        F: FnOnce(Result<String, GithubError>) + 'static,
    {
        let inner = self.inner.clone();
        let owner = owner.to_string();
        let repo = repo.to_string();

        spawn_local(async move {
            let result = inner.get_repo_json(&owner, &repo).await;
            callback(result);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GithubClient::new();
        assert_eq!(client.base_url(), "https://api.github.com");
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let client = GithubClient::with_base_url("https://api.github.enterprise.com");
        assert_eq!(client.base_url(), "https://api.github.enterprise.com");
    }
}
