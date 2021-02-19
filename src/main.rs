extern crate timer;
extern crate chrono;

use teloxide::prelude::*;
use std::env;
use tokio::time;
use std::time::{Duration, Instant, SystemTime};

use std::thread;
use std::sync::{Arc, Mutex};

async fn webrequest() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.rust-lang.org").await?;

    log::info!("Status: {}", res.status());
    let body = res.text().await?;
    log::info!("Body:\n\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() {
    env::set_var("TELOXIDE_TOKEN", "1671683413:AAEbPdktygghJI0HNE3aGy2FC67nmUoq75U");

    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();

    tokio::spawn( async move {
        let mut interval = time::interval(Duration::from_millis(2000));
        for _i in 0..5 {
            interval.tick().await;
            webrequest().await;
        }
    });

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;

}