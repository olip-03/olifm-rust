//! GitHub API client library
//!
//! This library provides a simple interface to interact with the GitHub API.

pub mod client;
pub mod error;
pub mod models;

// Re-export commonly used types for convenience
pub use client::{GithubClient, get_json};
pub use error::GithubError;
pub use models::{GithubRepo, GithubUser};

// Re-export the main types for convenience

/// Convenience function to create a new GitHub client
///
/// This is a factory function that creates a new `GithubClient` with default settings.
pub fn github_service() -> GithubClient {
    GithubClient::new()
}
