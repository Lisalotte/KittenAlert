use timer;
use chrono;

use teloxide::prelude::*;
use std::env;
use tokio::time;
use std::time::Duration;
use sled::Db;

mod kitties;

async fn webrequest() -> Result<String, reqwest::Error> {
    let url = "https://www.marktplaats.nl/l/dieren-en-toebehoren/katten-en-kittens-overige-katten/#searchInTitleAndDescription:true";
    let res = reqwest::get(url).await?;

    log::info!("Status: {}", res.status());
    let body = res.text().await?;
    log::info!("Body:\n\n{}", body);

    Ok(body)
}
async fn show(kitty: kitties::Kitty) {
    todo!()
}

async fn kittenloop(bot: teloxide::Bot) {
    let id = 249861073;
    let db = Db::open("kitty_database")
        .unwrap()
        .open_tree("shown")
        .unwrap();
    bot.send_message(id, "Hello World").send().await.unwrap();
    
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        let page = webrequest().await.unwrap();
        let list = kitties::from_page(page);
        for kitty in list {
            if !kitty.seen(&db) {
                db.insert(kitty.as_bytes(), "").unwrap();
            } else {
                show(kitty).await;
            }
        }
        interval.tick().await;
    }
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