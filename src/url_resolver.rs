use std::path::Path;

pub fn resolve_url(base: &str, relative: &str) -> String {
    // Check if it's already an absolute URL
    if relative.starts_with("http://") || relative.starts_with("https://") {
        relative.to_string()
    } else if relative.starts_with("//") {
        format!("https:{}", relative)
    } else if is_local_path(base) {
        // Handle local file paths
        resolve_local_path(base, relative)
    } else if relative.starts_with('/') {
        // Absolute path on remote server
        let base_url = url::Url::parse(base).unwrap();
        format!("{}://{}{}", base_url.scheme(), base_url.host_str().unwrap(), relative)
    } else {
        // Relative path on remote server
        let base_url = url::Url::parse(base).unwrap();
        base_url.join(relative).unwrap().to_string()
    }
}

pub fn is_local_path(path: &str) -> bool {
    path.starts_with('/') || 
    path.starts_with("./") || 
    path.starts_with("../") ||
    path.starts_with("file://") ||
    Path::new(path).exists()
}

fn resolve_local_path(base: &str, relative: &str) -> String {
    // Remove file:// prefix if present
    let base_clean = base.strip_prefix("file://").unwrap_or(base);
    
    let base_path = Path::new(base_clean);
    
    // Get the directory containing the base file
    let base_dir = if base_path.is_file() {
        base_path.parent().unwrap_or(Path::new("."))
    } else {
        base_path
    };
    
    // Handle different relative path formats
    let resolved = if relative.starts_with('/') {
        // Absolute local path
        Path::new(relative).to_path_buf()
    } else {
        // Relative to base directory
        base_dir.join(relative)
    };
    
    // Normalize the path
    match resolved.canonicalize() {
        Ok(canonical) => canonical.to_string_lossy().to_string(),
        Err(_) => resolved.to_string_lossy().to_string(),
    }
}

/// Check if a resource path is local or remote
pub fn is_remote_url(path: &str) -> bool {
    path.starts_with("http://") || path.starts_with("https://") || path.starts_with("//")
}