use serde::Deserialize;

/// Structures saved in public maps
#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub path: String,
    pub entry_type: String,
    pub size: u64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub entry_type: String,
    pub size: u64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ImageEntry {
    pub path: String,
    pub entry_type: String,
    pub size: u64,
    pub blurhash: String,
    pub aspect_ratio: String,
    pub name: String,
}

// Raw shapes matching the directory_structure.json (via untagged enum)
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DirectoryItem {
    Directory {
        path: String,
        #[serde(rename = "type")]
        entry_type: String,
        size: u64,
        name: String,
    },
    File {
        path: String,
        #[serde(rename = "type")]
        entry_type: String,
        size: u64,
        name: String,
    },
    Image {
        path: String,
        #[serde(rename = "type")]
        entry_type: String,
        size: u64,
        blurhash: String,
        aspect_ratio: String,
        name: String,
    },
}
