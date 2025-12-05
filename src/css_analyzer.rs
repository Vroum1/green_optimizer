use scraper::{Html, Selector};
use regex::Regex;
use crate::url_resolver::resolve_url;


pub struct CssAnalysis {
    pub total_selectors: usize,
    pub used_selectors: usize,
    pub unused_selectors: Vec<String>,
    pub total_bytes: usize,
    pub estimated_waste: usize,
}

pub async fn analyze_css(document: &Html, base_url: &str) -> CssAnalysis {
    let mut total_selectors = 0;
    let mut used_selectors = 0;
    let mut unused_selectors: Vec<String> = Vec::new();
    let mut total_bytes = 0;
    
    // Get all CSS links
    let css_selector = Selector::parse("link[rel='stylesheet']").unwrap();
    
    for element in document.select(&css_selector) {
        if let Some(href) = element.value().attr("href") {
            let css_url = resolve_url(base_url, href);
            
            if let Ok(css_content) = fetch_css(&css_url).await {
                total_bytes += css_content.len();
                
                // Extract and check selectors
                let selectors = extract_css_selectors(&css_content);
                
                for selector_str in selectors {
                    total_selectors += 1;
                    
                    if is_selector_used(document, &selector_str) {
                        used_selectors += 1;
                    } else {
                        unused_selectors.push(selector_str);
                    }
                }
            }
        }
    }
    
    // Also check inline <style> tags
    let style_selector = Selector::parse("style").unwrap();
    for element in document.select(&style_selector) {
        let css_content = element.inner_html();
        total_bytes += css_content.len();
        
        let selectors = extract_css_selectors(&css_content);
        
        for selector_str in selectors {
            total_selectors += 1;
            
            if is_selector_used(document, &selector_str) {
                used_selectors += 1;
            } else {
                unused_selectors.push(selector_str);
            }
        }
    }
    
    // Estimate wasted bytes (rough approximation)
    let waste_ratio = if total_selectors > 0 {
        unused_selectors.len() as f64 / total_selectors as f64
    } else {
        0.0
    };
    let estimated_waste = (total_bytes as f64 * waste_ratio) as usize;
    
    CssAnalysis {
        total_selectors,
        used_selectors,
        unused_selectors,
        total_bytes,
        estimated_waste,
    }
}

fn extract_css_selectors(css: &str) -> Vec<String> {
    let mut selectors = Vec::new();
    
    // Remove comments
    let comment_re = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let css_clean = comment_re.replace_all(css, "");
    
    // Remove @media, @keyframes, @font-face blocks content but keep selectors inside @media
    let media_re = Regex::new(r"@media[^{]+\{([\s\S]*?)\}\s*\}").unwrap();
    let mut all_css = css_clean.to_string();
    
    // Extract selectors from @media blocks
    for cap in media_re.captures_iter(&css_clean) {
        if let Some(inner) = cap.get(1) {
            all_css.push_str(inner.as_str());
        }
    }
    
    // Remove @keyframes, @font-face entirely
    let at_rules_re = Regex::new(r"@(keyframes|font-face|import|charset)[^{]*\{[^}]*\}").unwrap();
    let css_no_at = at_rules_re.replace_all(&all_css, "");
    
    // Extract selectors (everything before {)
    let selector_re = Regex::new(r"([^{}]+)\{[^}]*\}").unwrap();
    
    for cap in selector_re.captures_iter(&css_no_at) {
        if let Some(selector_match) = cap.get(1) {
            let selector_str = selector_match.as_str().trim();
            
            // Split multiple selectors (separated by comma)
            for s in selector_str.split(',') {
                let s = s.trim();
                if !s.is_empty() && !s.starts_with('@') {
                    selectors.push(s.to_string());
                }
            }
        }
    }
    
    selectors
}

fn is_selector_used(document: &Html, selector_str: &str) -> bool {
    // Clean up the selector for scraper
    let clean_selector = clean_selector_for_parsing(selector_str);
    
    if clean_selector.is_empty() {
        return true; // Assume used if we can't parse
    }
    
    // Try to parse and match the selector
    match Selector::parse(&clean_selector) {
        Ok(selector) => document.select(&selector).next().is_some(),
        Err(_) => true, // Assume used if we can't parse (pseudo-elements, etc.)
    }
}

fn clean_selector_for_parsing(selector: &str) -> String {
    let mut s = selector.to_string();
    
    // Remove pseudo-elements and pseudo-classes that scraper can't handle
    let pseudo_re = Regex::new(r"::(before|after|first-line|first-letter|selection|placeholder)").unwrap();
    s = pseudo_re.replace_all(&s, "").to_string();
    
    let pseudo_class_re = Regex::new(r":(hover|active|focus|visited|first-child|last-child|nth-child\([^)]+\)|not\([^)]+\)|disabled|enabled|checked|empty|root|target)").unwrap();
    s = pseudo_class_re.replace_all(&s, "").to_string();
    
    // Remove any remaining invalid characters
    s = s.trim().to_string();
    
    s
}

async fn fetch_css(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}
