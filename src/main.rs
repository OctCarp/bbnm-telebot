mod commands;
mod downloader;
mod handlers;
mod parsers;
mod services;
mod utils;

use crate::commands::commands_enum::SimpleCommand;
use crate::commands::simple_commands_handler::simple_commands_handler;

use crate::downloader::bili_down::down_bv;
use crate::handlers::text_handler::plain_text_handler;
use teloxide::prelude::*;

async fn run() {
    pretty_env_logger::init();
    log::info!("Starting b23nm Bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        // You can use branching to define multiple ways in which an update will be handled. If the
        // first branch fails, an update will be passed to the second branch, and so on.
        .branch(
            dptree::entry()
                // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                .filter_command::<SimpleCommand>()
                // If a command parsing fails, this handler will not be executed.
                .endpoint(simple_commands_handler),
        )
        .branch(dptree::entry().endpoint(plain_text_handler));

    Dispatcher::builder(bot, handler)
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        // If no handler succeeded to handle an update, this closure will be called.
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[tokio::main]
async fn main() {
    run().await;

    // simp_test().await;
}

async fn simp_test() {
    down_bv("BV1kFd7YAEcy".to_string()).await;

    // download_bili_video("https://www.bilibili.com/video/BV1Dx411j7XV/").await;

    // down_bv("BV1Dx411j7XV".to_string()).await;
    // let s=get_video_info("BV1Dx411j7XV").await.unwrap();
    // println!("{:?}", s);
}
