use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use blurhash::encode;
use image::{GenericImageView, ImageReader};
use serde::Serialize;

// Import the existing models for reuse

/// Serializable JSON entry used for output (name included for all types)
#[derive(Serialize)]
struct JsonEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
    size: u64,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    blurhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
}

/// Determine if a path points to an image based on extension
fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "bmp" | "tiff" | "gif"
        )
    } else {
        false
    }
}

/// Encode a blurhash and aspect ratio for an image path
fn encode_blurhash_and_aspect(path: &Path) -> Option<(String, String)> {
    let reader = ImageReader::open(path).ok()?;
    let img = reader.decode().ok()?;
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8();
    let blurhash = encode(4, 3, width, height, rgba.as_raw()).ok()?;
    if height == 0 {
        return None;
    }
    let aspect_ratio = format!("{}/{}", width, height);
    Some((blurhash, aspect_ratio))
}

/// Recursively traverse a directory and emit JsonEntry records using the models as sources
fn build_directory_structure(base: &Path) -> std::io::Result<Vec<JsonEntry>> {
    let mut entries: Vec<JsonEntry> = Vec::new();

    if base.is_dir() {
        let mut dirs = vec![base.to_path_buf()];
        while let Some(dir) = dirs.pop() {
            for entry_res in fs::read_dir(&dir)? {
                let entry = entry_res?;
                let meta = entry.metadata()?;
                let path = entry.path();
                let relative_path = path.strip_prefix(base).unwrap_or(&path);
                let path_str = format!("./{}", relative_path.display());

                // derive a name from the path's file/directory name
                let name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                if meta.is_dir() {
                    // Directory entry
                    entries.push(JsonEntry {
                        path: path_str.clone(),
                        entry_type: "directory".to_string(),
                        size: meta.len(),
                        name: name.clone(),
                        blurhash: None,
                        aspect_ratio: None,
                    });
                    dirs.push(path);
                } else if meta.is_file() {
                    if is_image_file(&path) {
                        if let Some((bh, ar)) = encode_blurhash_and_aspect(&path) {
                            entries.push(JsonEntry {
                                path: path_str,
                                entry_type: "image".to_string(),
                                size: meta.len(),
                                name: name.clone(),
                                blurhash: Some(bh),
                                aspect_ratio: Some(ar),
                            });
                        } else {
                            // Image but blurhash failed; store as a plain file
                            entries.push(JsonEntry {
                                path: path_str,
                                entry_type: "file".to_string(),
                                size: meta.len(),
                                name: name.clone(),
                                blurhash: None,
                                aspect_ratio: None,
                            });
                        }
                    } else {
                        entries.push(JsonEntry {
                            path: path_str,
                            entry_type: "file".to_string(),
                            size: meta.len(),
                            name: name.clone(),
                            blurhash: None,
                            aspect_ratio: None,
                        });
                    }
                }
            }
        }
    } else {
        // Base is a file
        let meta = fs::metadata(base)?;
        let path_str = format!("./{}", base.display());
        let name = base
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        if meta.is_dir() {
            entries.push(JsonEntry {
                path: path_str,
                entry_type: "directory".to_string(),
                size: meta.len(),
                name,
                blurhash: None,
                aspect_ratio: None,
            });
        } else if meta.is_file() {
            if is_image_file(base) {
                if let Some((bh, ar)) = encode_blurhash_and_aspect(base) {
                    entries.push(JsonEntry {
                        path: path_str,
                        entry_type: "image".to_string(),
                        size: meta.len(),
                        name,
                        blurhash: Some(bh),
                        aspect_ratio: Some(ar),
                    });
                } else {
                    entries.push(JsonEntry {
                        path: path_str,
                        entry_type: "file".to_string(),
                        size: meta.len(),
                        name,
                        blurhash: None,
                        aspect_ratio: None,
                    });
                }
            } else {
                entries.push(JsonEntry {
                    path: path_str,
                    entry_type: "file".to_string(),
                    size: meta.len(),
                    name,
                    blurhash: None,
                    aspect_ratio: None,
                });
            }
        }
    }

    Ok(entries)
}

fn print_usage(program_name: &str) {
    eprintln!(
        "Usage: {} --content <content_dir> --out <output_dir>",
        program_name
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut content_dir: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--content" => {
                if i + 1 < args.len() {
                    content_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    print_usage(&args[0]);
                    return;
                }
            }
            "--out" => {
                if i + 1 < args.len() {
                    output_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    print_usage(&args[0]);
                    return;
                }
            }
            _ => {
                print_usage(&args[0]);
                return;
            }
        }
    }

    let content_dir = match content_dir {
        Some(p) => p,
        None => {
            eprintln!("Error: --content argument is required.");
            print_usage(&args[0]);
            return;
        }
    };

    let output_dir = match output_dir {
        Some(p) => p,
        None => {
            eprintln!("Error: --out argument is required.");
            print_usage(&args[0]);
            return;
        }
    };

    if !content_dir.exists() {
        eprintln!(
            "Error: content directory '{}' does not exist.",
            content_dir.display()
        );
        return;
    }

    if !output_dir.exists() {
        if let Err(e) = fs::create_dir_all(&output_dir) {
            eprintln!(
                "Error: failed to create output directory '{}': {}",
                output_dir.display(),
                e
            );
            return;
        }
    }

    match build_directory_structure(&content_dir) {
        Ok(entries) => {
            let json = match serde_json::to_string_pretty(&entries) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("Error serializing directory structure to JSON: {}", e);
                    return;
                }
            };

            let output_path = output_dir.join("directory_structure.json");
            match File::create(&output_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json.as_bytes()) {
                        eprintln!("Error writing to '{}': {}", output_path.display(), e);
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("Error creating file '{}': {}", output_path.display(), e);
                    return;
                }
            }

            println!("Directory structure saved to '{}'.", output_path.display());
        }
        Err(e) => {
            eprintln!("Error building directory structure: {}", e);
        }
    }
}
