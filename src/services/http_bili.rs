pub async fn resolve_b23_short_url(url: &str) -> Option<String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .ok()?;

    let res = client.get(url).send().await.ok()?;
    if let Some(loc) = res.headers().get("Location") {
        let real_url = loc.to_str().ok()?.to_string();
        return Some(real_url);
    }

    None
}
