use teloxide::utils::command::BotCommands;

/// Simple commands
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum SimpleCommand {
    /// Shows start message.
    #[command(description = "Start start message.")]
    Start,
    /// Shows help message.
    #[command(description = "Show Help message.")]
    Help,
    /// Shows your user ID.
    #[command(description = "Get your user ID.")]
    MyId,
}

// /// Maintainer commands
// #[derive(BotCommands, Clone)]
// #[command(rename_rule = "lowercase")]
// pub enum MaintainerCommands {
//     /// Generate a number within range
//     #[command(parse_with = "split")]
//     Rand { from: u64, to: u64 },
// }
//
// /// Group commands
// #[derive(BotCommands, Clone)]
// #[command(rename_rule = "lowercase")]
// pub enum GroupCommand {
//     /// Repeats a message
//     Repeat { text: String },
// }
