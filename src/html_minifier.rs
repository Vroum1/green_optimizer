use minify_html::{Cfg, minify};

pub fn minify_html_content(html: &str, url: &str) {
        use std::path::Path;
    
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
    
    let output_path = parent_dir.join(format!("{}.min.{}", file_stem, extension));
    
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
