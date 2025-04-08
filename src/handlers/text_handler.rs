use crate::handlers::b23_handler::b23_text_handler;
use teloxide::prelude::Message;
use teloxide::types::*;
use teloxide::{Bot, RequestError};

pub async fn plain_text_handler(bot: Bot, msg: Message, me: Me) -> Result<(), RequestError> {
    if msg.text().is_none() {
        return Ok(());
    }
    b23_text_handler(bot, msg).await;
    Ok(())
}
