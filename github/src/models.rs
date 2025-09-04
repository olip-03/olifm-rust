use serde::{Deserialize, Serialize};

/// Represents a GitHub user from the API
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub created_at: String,
}

/// Represents a GitHub repository from the API
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubRepo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub clone_url: String,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RepoContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: Option<String>,
    #[serde(rename = "type")]
    pub content_type: String,
}

impl GithubUser {
    /// Returns the display name, falling back to login if name is None
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.login)
    }
}
