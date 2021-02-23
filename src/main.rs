extern crate timer;
extern crate chrono;

use teloxide::prelude::*;
use std::env;
use tokio::time;
use std::time::Duration;

async fn webrequest() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.rust-lang.org").await?;

    log::info!("Status: {}", res.status());
    let body = res.text().await?;
    log::info!("Body:\n\n{}", body);

    Ok(())
}

async fn kittenloop() {
    let mut interval = time::interval(Duration::from_millis(2000));
    loop {
        interval.tick().await;
        webrequest().await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    env::set_var("TELOXIDE_TOKEN", "1671683413:AAEbPdktygghJI0HNE3aGy2FC67nmUoq75U");

    teloxide::enable_logging!();
    log::info!("Starting kitty...");

    let bot = Bot::from_env();

    let check_kitties = kittenloop();
    let awnser_message = teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    });
    
    tokio::join!(check_kitties, awnser_message);
}