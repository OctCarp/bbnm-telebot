use lazy_static::lazy_static;
use regex::Regex;

use crate::services::http_bili::resolve_b23_short_url;
use crate::services::http_common::validate_url;

lazy_static! {
    pub static ref BV_REGEX: Regex = Regex::new(r"\b(BV[0-9A-Za-z]{10})\b").unwrap();
    pub static ref URL_REGEX: Regex =
        Regex::new(r"(https?://)?(www\.)?bilibili\.com/video/(BV[0-9A-Za-z]{10})").unwrap();
    pub static ref B23_REGEX: Regex = Regex::new(r"(https?://)?(b23\.tv/\S+)").unwrap();
    pub static ref AV_REGEX: Regex = Regex::new(r"\b(av\d+)\b").unwrap();
}

/// Extract Bilibili URL from a given text.
pub async fn extract_avbv_url(text: &str) -> Option<String> {
    async fn build_and_validate_url(id: &str) -> Option<String> {
        let url = format!("https://www.bilibili.com/video/{}", id);
        if validate_url(&url).await {
            Some(url)
        } else {
            None
        }
    }

    if let Some(caps) = BV_REGEX.captures(text) {
        return build_and_validate_url(&caps[1]).await;
    } else if let Some(caps) = URL_REGEX.captures(text) {
        return build_and_validate_url(&caps[3]).await;
    } else if let Some(caps) = AV_REGEX.captures(text) {
        return build_and_validate_url(&caps[1]).await;
    }

    None
}

pub async fn extract_b23_url(text: &str) -> Option<String> {
    // check if the text contains a B23 URL
    if let Some(caps) = B23_REGEX.captures(text) {
        let b23 = &caps[2];
        let url = format!("https://{}", b23);
        if let Some(real_url) = resolve_b23_short_url(&url).await {
            if let Some(url_cap) = URL_REGEX.captures(&real_url) {
                return Some(format!("https://b23.tv/{}", &url_cap[3]));
            }
        }
    }

    None
}
