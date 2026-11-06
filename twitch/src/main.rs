use log::LevelFilter;

use crate::bot::Bot;

mod bot;
mod config;
mod gateway;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let bot = Bot::new().await.expect("Fail create bot");
    let task = bot.handle_messages();
    task.await.unwrap();
}
