extern crate timer;
extern crate chrono;

use teloxide::prelude::*;
use std::env;
use tokio::time;
use std::time::Duration;

struct WebPage {
    body: String,
}

async fn webrequest() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://www.rust-lang.org").await?;

    log::info!("Status: {}", res.status());
    let body = res.text().await?;
    //log::info!("Body:\n\n{}", body);

    Ok(body)
}

async fn webrequest_loop() {

    let mut info = String::new();
    let mut web = String::new();
    let mut webpage = WebPage { body: String::new() };

    tokio::spawn( async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            web = String::new();
            interval.tick().await;
            webpage.body = webrequest().await.unwrap();
            info = web.clone();
            return info;
        }
    });
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    env::set_var("TELOXIDE_TOKEN", "1671683413:AAEbPdktygghJI0HNE3aGy2FC67nmUoq75U");

    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();
    let mut info = String::new();
    let mut web = String::new();
    let mut webpage = WebPage { body: String::new() };

    tokio::spawn( async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            web = String::new();
            interval.tick().await;
            webpage.body = webrequest().await.unwrap();
            info = web.clone();
            return info;
        }
    });

    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        //message.answer_str(info).await?;
        ResponseResult::<()>::Ok(())
    })
    .await;

    //teloxide::repl(bot, |message| async move {
        //let info = web;
        //message.answer(info).send().await?;
    //    ResponseResult::<()>::Ok(())
    //})
    //.await;

}