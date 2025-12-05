use crate::url_resolver::{resolve_url, is_remote_url};
use std::fs;

pub async fn extract_ressources(
    selector: scraper::Selector,
    document: &scraper::Html,
    url: &str,
    attr_name: &str,
    images_urls: &mut Vec<String>,
    resource_type: &str,
    total_size: &mut usize
) -> usize {
    let mut request_count = 0;
    
    for element in document.select(&selector) {
        if let Some(src) = element.value().attr(attr_name) {
            if resource_type == "Image" {
                images_urls.push(resolve_url(url, src));
            }
            let resource_url = resolve_url(url, src);
            
            let size_result = if is_remote_url(&resource_url) {
                get_remote_resource_size(&resource_url).await
            } else {
                get_local_resource_size(&resource_url)
            };
            
            if let Ok(size) = size_result {
                *total_size += size;
                request_count += 1;
                println!("{}: {} - {} bytes", resource_type, src, size);
            } else {
                println!("{}: {} - ⚠️  Failed to fetch", resource_type, src);
            }
        }
    }
    
    request_count
}

async fn get_remote_resource_size(url: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.len())
}

fn get_local_resource_size(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read(path)?;
    Ok(content.len())
}