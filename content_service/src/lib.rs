//! GitHub API client library
//!
//! This library provides a simple interface to interact with the GitHub API.

pub mod client;
pub mod error;
pub mod models;
pub mod utils;

// Re-export commonly used types for convenience
pub use client::{ContentServiceClient, ContentServiceClientCallback};
pub use error::ContentServiceError;
pub use models::{DirectoryEntry, DirectoryItem, FileEntry, ImageEntry};
pub use utils::console_log;
