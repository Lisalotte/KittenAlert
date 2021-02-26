use teloxide::prelude::*;
use teloxide::{types, BotBuilder};
use std::env;
use tokio::time;
use std::time::Duration;
use sled::Db;

mod kitties;

async fn webrequest() -> Result<String, reqwest::Error> {
    let url = "https://www.marktplaats.nl/l/dieren-en-toebehoren/katten-en-kittens-overige-katten/#searchInTitleAndDescription:true";
    let res = reqwest::get(url).await?;

    //log::info!("Status: {}", res.status());
    let body = res.text().await?;

    Ok(body)
}
async fn show(kitty: kitties::Kitty, bot: teloxide::Bot) {
    let id = 249861073;
    let message = format!(
        "<b>{}</b>\n{}\n<a href='{}'>Linkje</a>", 
        kitty.title.as_str(), 
        kitty.date.as_str(), 
        kitty.url.as_str()
    );
    bot.send_message(id, message).send().await.unwrap();
}

async fn kittenloop(bot: teloxide::Bot) {
    let id = 249861073;
    let db = Db::open("kitty_database")
        .unwrap()
        .open_tree("shown")
        .unwrap();
    bot.send_message(id, "Hello World").send().await.unwrap();
    
    let mut interval = time::interval(Duration::from_secs(120));
    loop {
        let page = webrequest().await.unwrap();
        let list = kitties::from_page(page);
        for kitty in list {
            if !kitty.seen(&db) {
                db.insert(kitty.title.as_bytes(), kitty.as_bytes()).unwrap();
                show(kitty, bot.clone()).await;
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

    let bot = BotBuilder::new()
        .parse_mode(types::ParseMode::HTML)
        .build();

    let check_kitties = kittenloop(bot.clone());
    let answer_message = teloxide::repl(bot, |message| async move {
        dbg!(&message);
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    });
    
    tokio::join!(check_kitties, answer_message);
}