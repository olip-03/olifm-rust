use crate::error::GithubError;
use crate::models::{GithubRepo, GithubUser, RepoContent};

/// User-Agent string for API requests
const USER_AGENT: &str = "olifm-rust/1.0";

/// Generic function to fetch JSON from a URL
pub async fn get_json(url: &str) -> Result<String, GithubError> {
    let result = surf::get(url).recv_string().await?;
    Ok(result)
}

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

    /// Fetch a GitHub user by username
    pub async fn get_user(&self, username: &str) -> Result<GithubUser, GithubError> {
        if username.trim().is_empty() {
            return Err(GithubError::InvalidInput(
                "Username cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/users/{}", self.base_url, username);
        let mut response = surf::get(&url).header("User-Agent", USER_AGENT).await?;

        match response.status() {
            surf::StatusCode::NotFound => Err(GithubError::NotFound),
            surf::StatusCode::Forbidden => Err(GithubError::RateLimited),
            _ => {
                let user: GithubUser = response.body_json().await?;
                Ok(user)
            }
        }
    }

    /// Fetch a GitHub repository by owner and repo name
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<GithubRepo, GithubError> {
        if owner.trim().is_empty() || repo.trim().is_empty() {
            return Err(GithubError::InvalidInput(
                "Owner and repo cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        let mut response = surf::get(&url).header("User-Agent", USER_AGENT).await?;

        match response.status() {
            surf::StatusCode::NotFound => Err(GithubError::NotFound),
            surf::StatusCode::Forbidden => Err(GithubError::RateLimited),
            _ => {
                let repo: GithubRepo = response.body_json().await?;
                Ok(repo)
            }
        }
    }

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

        let url = format!("{}/repos/{}/{}/{}", self.base_url, owner, repo, path);
        let mut response = surf::get(&url).header("User-Agent", USER_AGENT).await?;

        match response.status() {
            surf::StatusCode::NotFound => Err(GithubError::NotFound),
            surf::StatusCode::Forbidden => Err(GithubError::RateLimited),
            _ => {
                // convert response to Vector of RepoContent and return
                let content: Vec<RepoContent> = response.body_json().await?;
                Ok(content)
            }
        }
    }

    /// Fetch raw JSON for a user (for backwards compatibility)
    pub async fn get_user_json(&self, username: &str) -> Result<String, GithubError> {
        let url = format!("{}/users/{}", self.base_url, username);
        get_json(&url).await
    }

    /// Fetch raw JSON for a repository (for backwards compatibility)
    pub async fn get_repo_json(&self, owner: &str, repo: &str) -> Result<String, GithubError> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        get_json(&url).await
    }
}
