use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentPath {
    pub name: String,
    pub path: String,
    pub size: u32,
    #[serde(rename = "type")]
    pub content_type: String,
}
