extern crate timer;
extern crate chrono;

use teloxide::prelude::*;
use std::env;

use std::thread;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    env::set_var("TELOXIDE_TOKEN", "1671683413:AAEbPdktygghJI0HNE3aGy2FC67nmUoq75U");

    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();
    
    let timer = timer::Timer::new();
    // Number of times the callback has been called.
    let count = Arc::new(Mutex::new(0));

    // Start repeating. Each callback increases `count`.
    let guard = {
        let count = count.clone();
        timer.schedule_repeating(chrono::Duration::seconds(5), move || {
            *count.lock().unwrap() += 1;
            let count_result = *count.lock().unwrap();
            log::info!("The timer was called {} times", count_result);
        })
    };

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;

    // Now drop the guard. This should stop the timer.
    drop(guard);

    // Receive incoming updates using long polling
    // bot.get_updates();
}