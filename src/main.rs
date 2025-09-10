use blurhash::encode;
use chrono::{DateTime, Utc};
use content_service::models::{Img, JsonEntry};
use image::{GenericImageView, ImageReader};
use regex::Regex;
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
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
                    dirs.push(path);
                } else if meta.is_file() && !is_image_file(&path) {
                    let relative_path = path.strip_prefix(base).unwrap_or(&path);
                    let path_str = format!("/{}", relative_path.display());
                    let mut name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    let file_images = find_images(path.to_str().unwrap_or(""), images.clone());

                    // Extract frontmatter metadata
                    let metadata = extract_frontmatter(path.to_str().unwrap_or(""));
                    if metadata.contains_key("name") {
                        name = metadata["name"].clone();
                    }
                    let date = if let Some(metadata_date) = metadata.get("date") {
                        Some(metadata_date.clone())
                    } else {
                        meta.modified()
                            .ok()
                            .and_then(|time| system_time_to_iso8601(time))
                    };

                    entries.push(JsonEntry {
                        path: path_str,
                        entry_type: "file".to_string(),
                        size: meta.len(),
                        name,
                        date,
                        images: file_images,
                        metadata,
                    });
                }
            }
        }
    } else if base.is_file() && !is_image_file(base) {
        let meta = fs::metadata(base)?;
        let path_str = format!("./{}", base.display());
        let mut name = base
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let file_images = find_images(base.to_str().unwrap_or(""), images.clone());
        let metadata = extract_frontmatter(base.to_str().unwrap_or(""));
        if metadata.contains_key("name") {
            name = metadata["name"].clone();
        }
        let date = if let Some(metadata_date) = metadata.get("date") {
            Some(metadata_date.clone())
        } else {
            meta.modified()
                .ok()
                .and_then(|time| system_time_to_iso8601(time))
        };

        entries.push(JsonEntry {
            path: path_str,
            entry_type: "file".to_string(),
            size: meta.len(),
            name,
            date,
            images: file_images,
            metadata,
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

fn extract_frontmatter(file_path: &str) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read file {}: {}", file_path, e);
            return metadata;
        }
    };

    // More flexible regex that handles multiline YAML frontmatter
    // (?s) enables DOTALL mode so . matches newlines
    let re = match Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*") {
        Ok(re) => re,
        Err(e) => {
            println!("Regex error: {}", e);
            return metadata;
        }
    };

    if let Some(captures) = re.captures(&content) {
        if let Some(yaml_content) = captures.get(1) {
            let yaml_str = yaml_content.as_str().trim();

            println!("Found frontmatter in {}: \n{}", file_path, yaml_str);

            // Parse YAML using serde_yaml
            match serde_yaml::from_str::<YamlValue>(yaml_str) {
                Ok(yaml_value) => {
                    println!("Parsed YAML: {:?}", yaml_value);

                    if let YamlValue::Mapping(map) = yaml_value {
                        for (key, value) in map {
                            if let YamlValue::String(k) = key {
                                let value_str = match value {
                                    YamlValue::String(s) => s,
                                    YamlValue::Number(n) => n.to_string(),
                                    YamlValue::Bool(b) => b.to_string(),
                                    YamlValue::Null => "null".to_string(),
                                    _ => format!("{:?}", value),
                                };
                                println!("Adding metadata: {} = {}", k, value_str);
                                metadata.insert(k, value_str);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to parse YAML in {}: {}", file_path, e);

                    // Fallback to simple line-by-line parsing
                    for line in yaml_str.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }

                        if let Some((key, value)) = line.split_once(':') {
                            let key = key.trim().to_string();
                            let value = value
                                .trim()
                                .trim_matches('"')
                                .trim_matches('\'')
                                .to_string();
                            println!("Fallback parsing: {} = {}", key, value);
                            metadata.insert(key, value);
                        }
                    }
                }
            }
        }
    } else {
        println!("No frontmatter found in {}", file_path);
    }

    metadata
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

fn system_time_to_iso8601(time: SystemTime) -> Option<String> {
    let datetime: DateTime<Utc> = time.into();
    Some(datetime.to_rfc3339())
}
