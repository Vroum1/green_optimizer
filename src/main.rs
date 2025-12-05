use std::env;
use std::io::{stdin,stdout,Write};
use regex::Regex;

mod resource_extractor;
use resource_extractor::extract_ressources;

mod output;
use output::print_result;
use output::print_css_analysis;

mod url_resolver;
use url_resolver::is_local_path;

mod css_analyzer;

mod image_converter;
use image_converter::convert_images_to_webp;

mod html_minifier;
use html_minifier::minify_html_content;

#[tokio::main]
async fn main() {
    let html: String;
    let url: String;
    let mut local: bool = false;
    let mut images_urls: Vec<String> = Vec::new();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let input = &args[1];
        
        if is_local_path(input) {
            // Local file mode
            local = true;
            println!("📁 Analyzing local file: {}", input);
            html = std::fs::read_to_string(input).expect("Failed to read the file");
            url = input.to_string();
        } else {
            // URL passed as argument
            println!("🌐 Analyzing URL: {}", input);
            url = input.to_string();
            let response = reqwest::get(&url).await.unwrap();
            html = response.text().await.unwrap();
        }
    } else {
        // Interactive mode - ask user for URL
        let url_input = get_user_url().await;
        url = url_input;
        let response = reqwest::get(&url).await.unwrap();
        html = response.text().await.unwrap();
    }
    
    // Calculate HTML size in bytes
    let html_size = html.len();
    println!("HTML size: {} bytes ({:.2} KB)", html_size, html_size as f64 / 1024.0);
    
    // Parse HTML and find all resources
    let document = scraper::Html::parse_document(&html);
    
    let mut total_size = html_size;
    let mut total_requests = 1;
    
    // Extract CSS files
    let css_selector = scraper::Selector::parse("link[rel='stylesheet']").unwrap();
    let css_count = extract_ressources(css_selector, &document, &url, "href",&mut images_urls, "CSS",  &mut total_size).await;
    total_requests += css_count;

    // Extract JS files
    let js_selector = scraper::Selector::parse("script[src]").unwrap();
    let js_count = extract_ressources(js_selector, &document, &url, "src", &mut images_urls, "JS",  &mut total_size).await;
    total_requests += js_count;

    // Extract images
    let img_selector = scraper::Selector::parse("img[src]").unwrap();
    let img_count = extract_ressources(img_selector, &document, &url, "src", &mut images_urls, "Image", &mut total_size).await;
    total_requests += img_count;

    // Extract fonts
    let font_selector = scraper::Selector::parse("link[rel='preload'][as='font'], link[href$='.woff'], link[href$='.woff2']").unwrap();
    let font_count = extract_ressources(font_selector, &document, &url, "href", &mut images_urls, "Font", &mut total_size).await;
    total_requests += font_count;

    let css_analysis = css_analyzer::analyze_css(&document, &url).await;
    print_result(total_requests, css_count, js_count, img_count, font_count, total_size);
    print_css_analysis(&css_analysis);

    if local {
        println!("Do you wish to convert images to webp format? (It would save space) (y/n): ");
        let mut choice=String::new();
        stdin().read_line(&mut choice).expect("Failed to read input");
        if choice.trim().to_lowercase() == "y" {
            convert_images_to_webp(&images_urls);
        }

        println!("Do you wish to minify the local file? (y/n): ");
        let mut choice=String::new();
        stdin().read_line(&mut choice).expect("Failed to read input");
        if choice.trim().to_lowercase() == "y" {
            minify_html_content(&html, &url);
        }

    }
}

async fn get_user_url() -> String {
    let mut url_input=String::new();
    print!("Please enter a URL: ");
    let _=stdout().flush();
    stdin().read_line(&mut url_input).expect("Did not enter a correct string");
    if let Some('\n')=url_input.chars().next_back() {
        url_input.pop();
    }
    if let Some('\r')=url_input.chars().next_back() {
        url_input.pop();
    }
    println!("You typed: {}",url_input);
    let re = Regex::new(r"^https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&/=]*)$").unwrap();

        if !re.is_match(&url_input) {
        eprintln!("Invalid URL format!");
        std::process::exit(1);
    }
    return url_input;
}
