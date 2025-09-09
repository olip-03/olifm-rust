use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonEntry {
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub size: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>, // e.g., "2025-09-09T08:00:00Z"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Img>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Img {
    pub blurhash: String,
    pub aspect_ratio: String,
    pub name: String,
    pub path: String,
}
