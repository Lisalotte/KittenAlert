use teloxide::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    env::set_var("TELOXIDE_TOKEN", "1671683413:AAEbPdktygghJI0HNE3aGy2FC67nmUoq75U");

    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;

    // Receive incoming updates using long polling
    // bot.get_updates();
}