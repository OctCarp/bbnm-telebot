use teloxide::{RequestError, prelude::*, types::Me, utils::command::BotCommands};

use crate::commands::commands_enum::*;

pub async fn simple_commands_handler(
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), RequestError> {
    let text = match cmd {
        SimpleCommand::Start => "Welcome! BB23 nm Bot is running. ".to_string(),
        SimpleCommand::Help => {
            format!(
                "Hello! I am {}. Here are the commands you can use:\n{}",
                me.user.first_name,
                SimpleCommand::descriptions()
            )
        }
        SimpleCommand::MyId => {
            format!("Your id is: {}", msg.from.unwrap().id)
        }
    };

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}
