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
const CAPTION_LENGTH_WARN: usize = 1000;

pub async fn get_video_info(bvid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let video_url = format!(
        "https://api.bilibili.com/x/web-interface/view?bvid={}",
        bvid
    );
    let video_resp: serde_json::Value = client.get(&video_url).send().await?.json().await?;

    let data = &video_resp["data"];
    let title = data["title"].as_str().unwrap_or_default();
    let desc = data["desc"].as_str().unwrap_or_default();
    let owner_name = data["owner"]["name"].as_str().unwrap_or_default();
    let owner_mid = data["owner"]["mid"].as_i64().unwrap_or_default();

    // 获取置顶评论
    let reply_url = format!(
        "https://api.bilibili.com/x/v2/reply/main?oid={}&type=1&ps=1&sort=2",
        data["aid"].as_i64().unwrap_or_default()
    );
    let reply_resp: serde_json::Value = client.get(&reply_url).send().await?.json().await?;

    let mut output = String::new();

    output.push_str(&format!(
        "🎬 [{}](https://www.bilibili.com/video/{})\n",
        title, bvid
    ));

    output.push_str(&format!(
        "[@{}](https://space.bilibili.com/{})\n",
        owner_name, owner_mid
    ));

    let check_point = output.clone();
    if !desc.is_empty() {
        output.push_str(&format!("📝 {}\n", desc));
        if output.len() > CAPTION_LENGTH_WARN {
            return Ok(check_point);
        }
    }

    // // let data = &reply_resp["data"]["upper"];
    // println!("res :{}", &reply_resp);
    // println!("{}",&reply_resp["data"]["top"]["upper"]["member"]["uname"].as_str().unwrap_or_default());
    let top_reply = &reply_resp["data"]["top"]["upper"];
    if !top_reply.is_null() {
        output.push_str("〰〰〰〰〰〰〰〰〰〰\n");
        let replier_name = top_reply["member"]["uname"].as_str().unwrap_or_default();
        let replier_mid = top_reply["member"]["mid"].as_i64().unwrap_or_default();
        let reply_content = top_reply["content"]["message"].as_str().unwrap_or_default();

        output.push_str(&format!(
            "📌> [@{}](https://space.bilibili.com/{})\n",
            replier_name, replier_mid
        ));
        output.push_str(&format!("{}\n", reply_content));
    }

    if output.len() > CAPTION_LENGTH_WARN {
        return Ok(check_point);
    }

    Ok(output)
}

// #[derive(Debug, Deserialize)]
// struct VideoResponse {
//     data: VideoData,
// }
//
// #[derive(Debug, Deserialize)]
// struct VideoData {
//     title: String,
//     desc: String,
//     aid: u64,
// }
//
// #[derive(Debug, Deserialize)]
// struct CommentResponse {
//     data: CommentData,
// }
//
// #[derive(Debug, Deserialize)]
// struct CommentData {
//     hots: Vec<Comment>,
// }
//
// #[derive(Debug, Deserialize)]
// struct Comment {
//     content: CommentContent,
// }
//
// #[derive(Debug, Deserialize)]
// struct CommentContent {
//     message: String,
// }
//
// pub async fn get_bili_info(bv_id: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let client = Client::new();
//
//     let video_info_url = format!(
//         "https://api.bilibili.com/x/web-interface/view?bvid={}",
//         bv_id
//     );
//     let video_resp = client
//         .get(&video_info_url)
//         .send()
//         .await?
//         .json::<VideoResponse>()
//         .await?;
//
//     let title = &video_resp.data.title;
//     let desc = &video_resp.data.desc;
//
//     let comment_url = format!(
//         "https://api.bilibili.com/x/v2/reply?type=1&oid={}",
//         video_resp.data.aid
//     );
//     let comment_resp = client
//         .get(&comment_url)
//         .send()
//         .await?
//         .json::<CommentResponse>()
//         .await?;
//
//     let top_comment = comment_resp
//         .data
//         .hots
//         .get(0)
//         .map(|c| c.content.message.clone())
//         .unwrap_or("".to_string());
//     let mut caption = format!("🎬 [{}](https://www.bilibili.com/video/{}\n", title, bv_id);
//     let desc_cap = format!("📝 简介：{}\n", desc);
//     if top_comment.is_empty() {
//         return ok(caption);
//     }
//     let top_comment_cap = if top_comment.len() > 0 {
//         format!("〰〰〰〰〰〰〰〰〰〰\n📌 ：{}\n", top_comment)
//     } else {
//         "".to_string()
//     };
//     if (caption + &desc_cap).len() > 1024 {
//         return ok(caption);
//     }
//     caption.push_str(&desc_cap);
//     // Step 3: 构建 Telegram Caption
//     let caption = format!(
//         "🎬 {}\n\n📝 简介：{}\n📌 置顶评论：{}\n🔗 https://www.bilibili.com/video/{}",
//         title, desc, top_comment, bv_id
//     );
//
//     println!("{}", caption);
//     Ok(())
// }
