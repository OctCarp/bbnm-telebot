use once_cell::sync::Lazy;
use regex::Regex;

use crate::services::http_bili::resolve_b23_short_url;
use crate::services::http_common::validate_url;

// Regex patterns for AV/BV and B23 URLs
pub static BV_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(?P<id>BV[0-9A-Za-z]{10})\b").unwrap());

pub static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(https?://)?(www\.)?bilibili\.com/video/(?P<id>BV[0-9A-Za-z]{10})").unwrap()
});

pub static AV_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(?P<id>av\d+)\b").unwrap());

pub static B23_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(https?://)?(?P<url>b23\.tv/\S+)").unwrap());

/// Build the full URL and validate it
async fn build_and_validate_url(id: &str) -> Option<String> {
    let url = format!("https://www.bilibili.com/video/{}", id);
    if validate_url(&url).await {
        Some(url)
    } else {
        None
    }
}

/// Extract AV/BV URL and resolve it to the full video URL
pub async fn extract_avbv_url(text: &str) -> Option<String> {
    for regex in [&*BV_REGEX, &*URL_REGEX, &*AV_REGEX] {
        if let Some(caps) = regex.captures(text) {
            if let Some(id) = caps.name("id") {
                if id.as_str().starts_with("av") {
                    let av_id = id.as_str().trim_start_matches("av").parse::<i64>().unwrap();
                    let bv_id = bilirust::utils::av_to_bv(av_id);
                    return build_and_validate_url(&bv_id).await;
                }
                return build_and_validate_url(id.as_str()).await;
            }
        }
    }
    None
}

/// Extract B23 URL and resolve it to the full video URL
pub async fn extract_b23_url(text: &str) -> Option<String> {
    if let Some(caps) = B23_REGEX.captures(text) {
        if let Some(url_match) = caps.name("url") {
            let short_url = format!("https://{}", url_match.as_str());
            if let Some(real_url) = resolve_b23_short_url(&short_url).await {
                if let Some(id_caps) = BV_REGEX.captures(&real_url) {
                    if let Some(id) = id_caps.name("id") {
                        return Some(format!("https://www.bilibili.com/video/{}", id.as_str()));
                    }
                }
            }
        }
    }
    None
}

pub fn parse_url_to_bv(url: &str) -> Option<String> {
    URL_REGEX
        .captures(url)
        .and_then(|caps| caps.name("id"))
        .map(|id| id.as_str().to_string())
}
