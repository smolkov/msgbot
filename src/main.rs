use std::env;
use std::time::{Duration, Instant};

use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};
use tokio::timer::delay;

use msgbot::cmd;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
           cmd::message_handler(api.clone(), message).await?;
        }
    }
    Ok(())
}
