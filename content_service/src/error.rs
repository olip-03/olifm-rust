use std::fmt;

/// Content service specific errors
#[derive(Debug)]
pub enum ContentServiceError {
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

impl fmt::Display for ContentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentServiceError::RequestError(err) => write!(f, "Request error: {}", err),
            ContentServiceError::ParseError(err) => write!(f, "Parse error: {}", err),
            ContentServiceError::NotFound => write!(f, "Resource not found"),
            ContentServiceError::RateLimited => write!(f, "Rate limit exceeded"),
            ContentServiceError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ContentServiceError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for ContentServiceError {}

impl From<gloo_net::Error> for ContentServiceError {
    fn from(error: gloo_net::Error) -> Self {
        ContentServiceError::RequestError(error)
    }
}

impl From<serde_json::Error> for ContentServiceError {
    fn from(error: serde_json::Error) -> Self {
        ContentServiceError::ParseError(error)
    }
}
