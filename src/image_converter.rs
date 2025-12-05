use std::path::Path;
use std::fs;

pub fn convert_images_to_webp(images_urls: &Vec<String>) -> Vec<String>{
    use image::io::Reader as ImageReader;
    use webp::Encoder;
    use colored::*;
    
    println!("\n{}", "🖼️  Converting images to WebP...".cyan().bold());
    
    let mut converted = 0;
    let mut total_saved: i64 = 0;
    let mut converted_urls: Vec<String> = Vec::new();
    
    for image_path in images_urls {
        // Skip remote URLs and non-image files
        if image_path.starts_with("http") {
            println!("  ⏭️  Skipping remote image: {}", image_path);
            continue;
        }
        
        // Check if file exists and is a supported format
        let path = Path::new(image_path);
        if !path.exists() {
            println!("  ⚠️  File not found: {}", image_path);
            continue;
        }
        
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Skip if already WebP or unsupported format
        if extension == "webp" {
            println!("  ⏭️  Already WebP: {}", image_path);
            continue;
        }
        
        if !["jpg", "jpeg", "png", "gif", "bmp", "tiff"].contains(&extension.as_str()) {
            println!("  ⏭️  Unsupported format: {}", image_path);
            continue;
        }
        
        // Get original file size
        let original_size = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        // Load and convert the image
        match ImageReader::open(path) {
            Ok(reader) => {
                match reader.decode() {
                    Ok(img) => {
                        let rgba = img.to_rgba8();
                        let (width, height) = rgba.dimensions();
                        
                        // Create WebP encoder (quality 80 is a good balance)
                        let encoder = Encoder::from_rgba(&rgba, width, height);
                        let webp_data = encoder.encode(80.0);
                        
                        // Create output path
                        let output_path = path.with_extension("webp");
                        
                        // Write WebP file
                        match std::fs::write(&output_path, &*webp_data) {
                            Ok(_) => {
                                let new_size = webp_data.len() as u64;
                                let saved = original_size as i64 - new_size as i64;
                                total_saved += saved;
                                converted += 1;
                                
                                let saved_str = if saved > 0 {
                                    format!("saved {} bytes", saved).green()
                                } else {
                                    format!("increased {} bytes", -saved).red()
                                };

                                
                                
                                println!("  ✅ {} -> {} ({})", 
                                    image_path, 
                                    output_path.display(),
                                    saved_str);

                                fs::remove_file(image_path).expect(&format!("Failed to delete {} file", image_path));
                                converted_urls.push(output_path.to_string_lossy().to_string());
                            },
                            Err(e) => {
                                println!("  ❌ Failed to write {}: {}", output_path.display(), e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("  ❌ Failed to decode {}: {}", image_path, e);
                    }
                }
            },
            Err(e) => {
                println!("  ❌ Failed to open {}: {}", image_path, e);
            }
        }
    }
    
    // Summary
    println!("\n{}", "═══════════════════════════════════════".cyan());
    println!("📊 Conversion summary:");
    println!("   Images converted: {}", converted.to_string().green().bold());
    
    if total_saved > 0 {
        println!("   Total space saved: {} bytes ({:.2} KB)", 
            total_saved.to_string().green().bold(),
            total_saved as f64 / 1024.0);
    } else if total_saved < 0 {
        println!("   Total space increased: {} bytes", 
            (-total_saved).to_string().red().bold());
    }
    println!("{}", "═══════════════════════════════════════".cyan());

    return converted_urls;
}

