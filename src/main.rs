use blurhash::encode;
use image::{GenericImageView, ImageReader};
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Serializable JSON entry used for output (name included for all types)
#[derive(Serialize)]
struct JsonEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
    size: u64,
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    images: Vec<Img>,
}

#[derive(Serialize, Clone)]
struct Img {
    blurhash: String,
    aspect_ratio: String,
    name: String,
    path: String,
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

fn build_directory_structure(base: &Path) -> std::io::Result<Vec<JsonEntry>> {
    let mut entries: Vec<JsonEntry> = Vec::new();
    let images: Vec<Img> = build_img_structure(base).expect("Could not encode images");

    if base.is_dir() {
        let mut dirs = vec![base.to_path_buf()];
        while let Some(dir) = dirs.pop() {
            for entry_res in fs::read_dir(&dir)? {
                let entry = entry_res?;
                let meta = entry.metadata()?;
                let path = entry.path();

                if meta.is_dir() {
                    // Just add directories to processing queue, don't create entries
                    dirs.push(path);
                } else if meta.is_file() {
                    // Only process files
                    let relative_path = path.strip_prefix(base).unwrap_or(&path);
                    let path_str = format!("/{}", relative_path.display());
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    // Find images referenced in this file
                    let file_images = find_images(path.to_str().unwrap_or(""), images.clone());

                    entries.push(JsonEntry {
                        path: path_str,
                        entry_type: "file".to_string(),
                        size: meta.len(),
                        name,
                        images: file_images,
                    });
                }
            }
        }
    } else if base.is_file() {
        // Handle single file case
        let meta = fs::metadata(base)?;
        let path_str = format!("./{}", base.display());
        let name = base
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        // Find images referenced in this file
        let file_images = find_images(base.to_str().unwrap_or(""), images.clone());

        entries.push(JsonEntry {
            path: path_str,
            entry_type: "file".to_string(),
            size: meta.len(),
            name,
            images: file_images,
        });
    }

    Ok(entries)
}

/// Recursively traverse a directory and collect all image files with their metadata
fn build_img_structure(base: &Path) -> std::io::Result<Vec<Img>> {
    let mut images: Vec<Img> = Vec::new();

    if base.is_dir() {
        let mut dirs = vec![base.to_path_buf()];
        while let Some(dir) = dirs.pop() {
            for entry_res in fs::read_dir(&dir)? {
                let entry = entry_res?;
                let meta = entry.metadata()?;
                let path = entry.path();

                if meta.is_dir() {
                    // Add directories to processing queue
                    dirs.push(path);
                } else if meta.is_file() && is_image_file(&path) {
                    // Process image files
                    let relative_path = path.strip_prefix(base).unwrap_or(&path);
                    let path_str = format!("/{}", relative_path.display());
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    // Get blurhash and aspect ratio
                    if let Some((blurhash, aspect_ratio)) = encode_blurhash_and_aspect(&path) {
                        images.push(Img {
                            blurhash,
                            aspect_ratio,
                            name,
                            path: path_str,
                        });
                    }
                    // Skip images that couldn't be processed
                }
            }
        }
    } else if base.is_file() && is_image_file(base) {
        // Handle single image file case
        let path_str = format!("./{}", base.display());
        let name = base
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        if let Some((blurhash, aspect_ratio)) = encode_blurhash_and_aspect(base) {
            images.push(Img {
                blurhash,
                aspect_ratio,
                name,
                path: path_str,
            });
        }
    }

    Ok(images)
}

fn find_images(path: &str, img_store: Vec<Img>) -> Vec<Img> {
    let mut images: Vec<Img> = Vec::new();

    // Create a lookup map by image name for O(1) access
    let img_map: HashMap<String, &Img> = img_store
        .iter()
        .map(|img| (img.name.clone(), img))
        .collect();

    // Read the file content
    if let Ok(content) = fs::read_to_string(path) {
        // Regex to match ![[filename]] pattern
        let re = Regex::new(r"!\[\[([^\]]+)\]\]").unwrap();

        // Find all matches and collect unique image names
        let mut found_names: std::collections::HashSet<String> = std::collections::HashSet::new();

        for capture in re.captures_iter(&content) {
            if let Some(filename) = capture.get(1) {
                let filename_str = filename.as_str().to_string();
                if found_names.insert(filename_str.clone()) {
                    // Only process if we haven't seen this filename before
                    if let Some(img) = img_map.get(&filename_str) {
                        images.push((*img).clone());
                    }
                }
            }
        }
    }

    images
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
