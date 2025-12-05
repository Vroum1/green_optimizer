use colored::*;
use crate::css_analyzer;

pub fn print_result(total_requests: usize ,css_count: usize ,js_count: usize ,img_count: usize ,font_count: usize ,total_size: usize) {

    let total_size_kb = total_size as f64 / 1024.0;
    let total_size_mb = total_size as f64 / (1024.0 * 1024.0);

    let mut weight_text = format!("{} bytes ({:.2} KB, {:.2} MB)", total_size, total_size_kb, total_size_mb);
    let colored_weight = if total_size_mb < 2.0 {
        weight_text = weight_text + "✓  Lightweight";
        weight_text.green()
    } else if total_size_mb < 4.0 {
        weight_text = weight_text + "!  Kinda heavy";
        weight_text.yellow()
    } else {
        weight_text = weight_text + "✗  Heavy";
        weight_text.red()
    };
    println!("\n{}", "========== SUMMARY ==========".bold().cyan());
    println!("Total requests: {}", total_requests.to_string().bold());
    println!("  - HTML: {}", "1".white());
    println!("  - CSS: {}", css_count.to_string().blue());
    println!("  - JS: {}", js_count.to_string().yellow());
    println!("  - Images: {}", img_count.to_string().magenta());
    println!("  - Fonts: {}", font_count.to_string().cyan());
    println!("\nTotal page weight: {}", colored_weight.bold());
    println!("{}", "=============================".bold().cyan());
    println!("\n{}", "This data is computed based on the initial HTML request and the page may be heavier, since we do not wait for all resources (css, js) to load.".italic());
}

pub fn print_css_analysis(analysis: &css_analyzer::CssAnalysis) {
    use colored::*;
    
    println!("\n{}", "========== CSS ANALYSIS ==========".bold().cyan());
    println!("Total selectors: {}", analysis.total_selectors.to_string().white());
    println!("Used selectors: {}", analysis.used_selectors.to_string().green());
    println!("Unused selectors: {}", analysis.unused_selectors.len().to_string().red());
    
    let usage_percent = if analysis.total_selectors > 0 {
        (analysis.used_selectors as f64 / analysis.total_selectors as f64) * 100.0
    } else {
        100.0
    };
    
    let usage_colored = if usage_percent > 80.0 {
        format!("{:.1}%", usage_percent).green()
    } else if usage_percent > 50.0 {
        format!("{:.1}%", usage_percent).yellow()
    } else {
        format!("{:.1}%", usage_percent).red()
    };
    
    println!("CSS usage: {}", usage_colored.bold());
    println!("Estimated waste: {} bytes ({:.2} KB)", 
             analysis.estimated_waste.to_string().red(),
             analysis.estimated_waste as f64 / 1024.0);
    
    // Show first 10 unused selectors
    if !analysis.unused_selectors.is_empty() {
        println!("\n{}", "Top unused selectors:".yellow());
        for (i, selector) in analysis.unused_selectors.iter().take(10).enumerate() {
            println!("  {}. {}", i + 1, selector.bright_black());
        }
        if analysis.unused_selectors.len() > 10 {
            println!("  ... and {} more", analysis.unused_selectors.len() - 10);
        }
    }
    
    println!("{}", "==================================".bold().cyan());
}