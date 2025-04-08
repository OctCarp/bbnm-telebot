use crate::downloader::bili_down::down_vu;
use crate::parsers::bili_parser::{extract_avbv_url, extract_b23_url, parse_url_to_bv};
use crate::services::http_bili::get_video_info;
use crate::utils::files::sanitize_filename;
use crate::utils::users::get_user_str;

use anyhow::Context;
use bilirust::{FNVAL_MP4, VIDEO_QUALITY_720P};
use teloxide::Bot;
use teloxide::payloads::SendVideoSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::InputFile;
use teloxide::types::ParseMode;

pub(crate) async fn b23_text_handler(bot: Bot, msg: Message) -> anyhow::Result<()> {
    let text = msg.text().unwrap().to_string();
    if let Some(url) = extract_avbv_url(&text).await {
        bot.send_message(msg.chat.id, format!("Bili 链接： {}", url))
            .reply_to(msg.id)
            .await?;
        download_url_and_send(&bot, &msg, &url).await?;

        return Ok(());
    } else if let Some(url) = extract_b23_url(&text).await {
        bot.delete(&msg).await?;
        download_url_and_send(&bot, &msg, &url).await?;
        bot.send_message(
            msg.chat.id,
            format!(
                "{} 分享了 B23 链接： {}",
                get_user_str(msg.from.clone()),
                url
            ),
        )
        .await?;

        return Ok(());
    }

    Ok(())
}

async fn download_url_and_send(bot: &Bot, msg: &Message, url: &str) -> anyhow::Result<String> {
    let bv = match parse_url_to_bv(url) {
        Some(bv) => bv,
        None => {
            bot.send_message(msg.chat.id, "解析链接异常").await?;
            return Ok(String::from(""));
        }
    };

    let client = bilirust::Client::new();
    log::info!("Match BV number: {}", bv);
    let info = client
        .bv_info(bv.clone())
        .await
        .context("Get video info failed")?;

    log::info!("Video title: {}", info.title);

    let format = FNVAL_MP4;
    let download_urls = client
        .bv_download_url(bv.clone(), info.cid, format, VIDEO_QUALITY_720P)
        .await
        .context("Get resource url failed")?;
    let filename = format!("./temp/bv/{}.mp4", sanitize_filename(&bv));

    down_vu(&download_urls, &filename)
        .await
        .context("Download failed")?;

    bot.send_video(msg.chat.id, InputFile::file(filename.clone()))
        .caption(get_video_info(&bv).await.unwrap())
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to(msg.id)
        .await?;

    Ok(filename)
}
