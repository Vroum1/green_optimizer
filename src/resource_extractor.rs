use crate::url_resolver::resolve_url;

pub async fn extract_ressources(
    selector: scraper::Selector,
    document: &scraper::Html,
    url: &str,
    attr_name: &str,
    resource_type: &str,
    total_size: &mut usize
) -> usize {
    let mut request_count = 0;
    
    for element in document.select(&selector) {
        if let Some(src) = element.value().attr(attr_name) {
            let resource_url = resolve_url(url, src);
            if let Ok(size) = get_resource_size(&resource_url).await {
                *total_size += size;
                request_count += 1;
                println!("{}: {} - {} bytes", resource_type, src, size);
            }
        }
    }
    
    request_count
}

async fn get_resource_size(url: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.len())
}

