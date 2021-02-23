use timer;
use chrono;

use teloxide::prelude::*;
use std::env;
use tokio::time;
use std::time::Duration;
use sled::Db;

async fn webrequest() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.rust-lang.org").await?;

    log::info!("Status: {}", res.status());
    let body = res.text().await?;
    log::info!("Body:\n\n{}", body);

    Ok(())
}

async fn kittenloop(bot: teloxide::Bot) {
    let id = 249861073;
    let kitty_db = Db::open("kitty_database").unwrap();
    bot.send_message(id, "Hello World").send().await.unwrap();
    let mut interval = time::interval(Duration::from_secs(10));
    /*loop {
        interval.tick().await;
        webrequest().await.unwrap();
    }*/
}

#[tokio::main]
async fn main() {
    env::set_var("TELOXIDE_TOKEN", "1671683413:AAEbPdktygghJI0HNE3aGy2FC67nmUoq75U");

    teloxide::enable_logging!();
    log::info!("Starting kitty...");

    let bot = Bot::from_env();

    let check_kitties = kittenloop(bot.clone());
    let answer_message = teloxide::repl(bot, |message| async move {
        dbg!(&message);
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    });
    
    tokio::join!(check_kitties, answer_message);
}