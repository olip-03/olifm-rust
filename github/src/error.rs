use std::fmt;

/// GitHub API specific errors
#[derive(Debug)]
pub enum GithubError {
    /// HTTP request failed
    RequestError(gloo_net::Error),
    /// JSON parsing failed
    ParseError(serde_json::Error),
    /// Resource not found (404)
    NotFound,
    /// Rate limit exceeded
    RateLimited,
    /// Invalid input provided
    InvalidInput(String),
    /// Network error
    NetworkError(String),
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GithubError::RequestError(err) => write!(f, "Request error: {}", err),
            GithubError::ParseError(err) => write!(f, "Parse error: {}", err),
            GithubError::NotFound => write!(f, "Resource not found"),
            GithubError::RateLimited => write!(f, "Rate limit exceeded"),
            GithubError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            GithubError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for GithubError {}

impl From<gloo_net::Error> for GithubError {
    fn from(error: gloo_net::Error) -> Self {
        GithubError::RequestError(error)
    }
}

impl From<serde_json::Error> for GithubError {
    fn from(error: serde_json::Error) -> Self {
        GithubError::ParseError(error)
    }
}
