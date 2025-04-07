use crate::parsers::bili_parser::{extract_avbv_url, extract_b23_url};
use crate::utils::users::get_user_str;
use reqwest::get;
use teloxide::prelude::{Message, Requester};
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::*;
use teloxide::{Bot, RequestError};

pub async fn plain_text_handler(bot: Bot, msg: Message, me: Me) -> Result<(), RequestError> {
    if msg.text().is_none() {
        return Ok(());
    }
    b23_text_handler(bot, msg).await;
    Ok(())
}

async fn b23_text_handler(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let text = msg.text().unwrap().to_string();
    if let Some(url) = extract_avbv_url(&text).await {
        bot.send_message(msg.chat.id, format!("Bili 链接： {}", url))
            .reply_to(msg.id)
            .await?;

        return Ok(());
    } else if let Some(url) = extract_b23_url(&text).await {
        bot.delete(&msg).await?;

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
