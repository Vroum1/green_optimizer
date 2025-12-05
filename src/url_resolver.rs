pub fn resolve_url(base: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") {
        relative.to_string()
    } else if relative.starts_with("//") {
        format!("https:{}", relative)
    } else if relative.starts_with('/') {
        let base_url = url::Url::parse(base).unwrap();
        format!("{}://{}{}", base_url.scheme(), base_url.host_str().unwrap(), relative)
    } else {
        let base_url = url::Url::parse(base).unwrap();
        base_url.join(relative).unwrap().to_string()
    }
}