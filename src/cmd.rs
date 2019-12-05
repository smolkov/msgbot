use std::env;
use std::time::{Duration, Instant};

use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};
use tokio::timer::delay;

use std::include_str;
// pub mod telebot;

const HELP: &str = include_str!("asset/help/help.md");
const GET: &str  = include_str!("asset/help/get.md");
const SET: &str  = include_str!("asset/help/set.md");
const RUN: &str  = include_str!("asset/help/run.md");
const HOLD: &str = include_str!("asset/help/hold.md");




// pub use self::telebot::TeleBot;


async fn help(api: Api, message: Message) -> Result<(), Error> {


    let mut reply = message.text_reply(HELP);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    // let mut reply = message.text_reply("<b>Bold HTML message</b>");
    // api.send(reply.parse_mode(ParseMode::Html)).await?;
    Ok(())
}

async fn status(api: Api, message: Message) -> Result<(), Error> {

    let mut reply = message.text_reply(HELP);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    // let mut reply = message.text_reply("<b>Bold HTML message</b>");
    // api.send(reply.parse_mode(ParseMode::Html)).await?;
    Ok(())
}

async fn set(api: Api, message: Message) -> Result<(), Error> {

    let mut reply = message.text_reply(SET);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

async fn get(api: Api, message: Message) -> Result<(), Error> {

    let mut reply = message.text_reply(GET);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

async fn run(api: Api, message: Message) -> Result<(), Error> {
    let message1 = api.send(message.text_reply("Round 1")).await?;
    let when = Instant::now() + Duration::from_secs(1);
    delay(when).await;
    let message2 = api.send(message1.edit_text("Round 2")).await?;
    let when = Instant::now() + Duration::from_secs(1);
    delay(when).await;
    let mut reply = message.text_reply(RUN);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

async fn hold(api: Api, message: Message) -> Result<(), Error> {

    let mut reply = message.text_reply(HOLD);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

async fn report(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply("Hey, ich brauche mehr daten um ein Reporn zu erstellen?")).await?;

    // let mut reply = message1.text_reply(HELP);
    // api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}


async fn chat(api: Api, message: Message) -> Result<(), Error> {

    api.send(message.text_reply("Hey, einfach plaudern oder hast du was konretes vor?")).await?;

    
    // let mut reply = message1.text_reply(HELP);
    // api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

/// ### 🔬 `@larpwa_bot`
/// ```
/// help - 📖 usage information
/// state - 🎛 status
/// set - 🔧 change parameter */set pwa.m1.name neue method name*
/// get - 🔧 parameter value and description */get pwa.m1.name* return method name
/// run - 📢 command */run irrigatron.action.gp1.start*
/// mod - 🔄 mod */mod interval* for interval run
/// stop - ⛔ all runned comman and 🔄 to ready
/// ```
pub async fn message_handler(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/help"   => help(api, message).await?,
            "/status" => status(api, message).await?,
            "/set"    => set(api, message).await?,
            "/get"    => get(api, message).await?,
            "/run"    => run(api, message).await?,
            "/hold"   => hold(api, message).await?,
            "/report" => report(api,message).await?,
            _ => chat(api, message).await?,
        },
        _ => (),
    };

    Ok(())
}
