use minify_html::{Cfg, minify};
use std::fs;
use std::path::Path;

pub fn minify_html_content(html: &str, url: &str) {
    let cfg = Cfg { ..Cfg::default() };
    let minified = minify(html.as_bytes(), &cfg);
    
    // Create output path next to original file
    let original_path = Path::new(&url);
    let file_stem = original_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let extension = original_path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("html");
    let parent_dir = original_path.parent()
        .unwrap_or(Path::new("."));
    
    let output_path = parent_dir.join(format!("{}.{}", file_stem, extension));
    fs::remove_file(original_path).expect(&format!("Failed to delete {} file", original_path.display()));

    std::fs::write(&output_path, minified).expect("Failed to write minified file");
    
    // Show size comparison
    let original_size = html.len();
    let minified_size = std::fs::metadata(&output_path)
        .map(|m| m.len() as usize)
        .unwrap_or(0);
    let saved = original_size - minified_size;
    let saved_percent = (saved as f64 / original_size as f64) * 100.0;
    
    println!("✅ Minified file saved as {}", output_path.display());
    println!("   Original: {} bytes", original_size);
    println!("   Minified: {} bytes", minified_size);
    println!("   Saved: {} bytes ({:.1}%)", saved, saved_percent);
}

pub fn change_html_image_urls(file_path: &str, converted_urls: &Vec<String>) {
    // Read the current file content
    let html = fs::read_to_string(file_path)
        .expect(&format!("Failed to read file: {}", file_path));
    
    let mut modified_html = html.clone();
    
    for converted_url in converted_urls {
        // Get just the filenames for replacement
        let (original_filename, new_filename) = get_filenames(converted_url);
        
        modified_html = modified_html.replace(&original_filename, &new_filename);
    }
        
    // Write the modified content back to the file
    fs::write(file_path, &modified_html)
        .expect(&format!("Failed to write to file: {}", file_path));
    
    println!("✅ Updated {} image URLs in {}", converted_urls.len(), file_path);
}

fn get_filenames(webp_path: &str) -> (String, String) {
    let path = Path::new(webp_path);
    
    // Get the new filename (e.g., "orNoir.webp")
    let new_filename = path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(webp_path)
        .to_string();
    
    // Get the base name without extension (e.g., "orNoir")
    let base_name = path.file_stem()
        .and_then(|f| f.to_str())
        .unwrap_or("");
    
    // Try to find the original extension
    let extensions = [".jpeg", ".jpg", ".png", ".gif", ".bmp", ".tiff"];
    let parent_dir = path.parent().unwrap_or(Path::new("."));
    
    for ext in extensions {
        let potential_path = parent_dir.join(format!("{}{}", base_name, ext));
        let original_filename = format!("{}{}", base_name, ext);
        
        // Check if this was the original file (now deleted) or just try common extensions
        // Since original is deleted, we just try each extension
        if webp_path.contains(base_name) {
            // Return the first match that makes sense
            return (original_filename, new_filename);
        }
    }
    
    // Default fallback: assume .jpeg
    (format!("{}.jpeg", base_name), new_filename)
}