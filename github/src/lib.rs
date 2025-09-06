//! GitHub API client library
//!
//! This library provides a simple interface to interact with the GitHub API.

pub mod client;
pub mod error;
pub mod models;
pub mod utils;

// Re-export commonly used types for convenience
pub use client::{GithubClient, GithubClientCallback};
pub use error::GithubError;
pub use models::{GithubRepo, GithubUser, RepoContent};
pub use utils::console_log;
