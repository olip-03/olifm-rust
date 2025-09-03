use reqwest;
use serde::{Deserialize, Serialize};

pub async fn get_json(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await?;
    Ok(result)
}

// GitHub API response structures
#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug)]
pub enum GithubError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    NotFound,
    RateLimited,
}

impl From<reqwest::Error> for GithubError {
    fn from(error: reqwest::Error) -> Self {
        GithubError::RequestError(error)
    }
}

impl From<serde_json::Error> for GithubError {
    fn from(error: serde_json::Error) -> Self {
        GithubError::ParseError(error)
    }
}

// GitHub Service struct
pub struct GithubService {
    base_url: String,
    client: reqwest::Client,
}

impl GithubService {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("olifm-rust/1.0")
            .build()
            .unwrap();

        GithubService {
            base_url: "https://api.github.com".to_string(),
            client,
        }
    }

    pub fn get_url(&self) -> &str {
        &self.base_url
    }

    pub async fn get_user(&self, username: &str) -> Result<GithubUser, GithubError> {
        let url = format!("{}/users/{}", self.base_url, username);
        let response = self.client.get(&url).send().await?;

        if response.status() == 404 {
            return Err(GithubError::NotFound);
        }

        let user: GithubUser = response.json().await?;
        Ok(user)
    }

    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<GithubRepo, GithubError> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        let response = self.client.get(&url).send().await?;

        if response.status() == 404 {
            return Err(GithubError::NotFound);
        }

        let repo: GithubRepo = response.json().await?;
        Ok(repo)
    }

    pub async fn get_user_repos(&self, username: &str) -> Result<Vec<GithubRepo>, GithubError> {
        let url = format!("{}/users/{}/repos", self.base_url, username);
        let response = self.client.get(&url).send().await?;

        if response.status() == 404 {
            return Err(GithubError::NotFound);
        }

        let repos: Vec<GithubRepo> = response.json().await?;
        Ok(repos)
    }

    // Raw JSON methods for backwards compatibility
    pub async fn get_user_json(&self, username: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/users/{}", self.base_url, username);
        get_json(&url).await
    }

    pub async fn get_repo_json(&self, owner: &str, repo: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        get_json(&url).await
    }
}

// Factory function to create a GitHub service instance
pub fn github_service() -> GithubService {
    GithubService::new()
}
