use reqwest::Client;

pub async fn validate_url(url: &str) -> bool {
    let client = Client::new();
    match client.head(url).send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}
