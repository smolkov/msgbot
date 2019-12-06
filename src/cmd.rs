use std::env;
use std::time::{Duration, Instant};

use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};
use tokio::timer::delay;

use std::include_str;
use super::state;

const HELP: &str = include_str!("asset/help/help.md");
const GET: &str  = include_str!("asset/help/get.md");
const SET: &str  = include_str!("asset/help/set.md");
const RUN: &str  = include_str!("asset/help/run.md");
const HOLD: &str = include_str!("asset/help/hold.md");



pub struct Folge {
    handler: Box<dyn Fn(&Api, &Message) -> Message>,
}



fn get_messege_text(message:&Message) -> String {
    let text = match message.kind {
        MessageKind::Text { ref data, .. } => String::from(data.as_str()),
        _ => String::from(""),

    };
    text
}

// pub use self::telebot::TeleBot;


async fn help(api: Api, message: Message) -> Result<(), Error> {

    let mut reply = message.text_reply(HELP);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    // let mut reply = message.text_reply("<b>Bold HTML message</b>");
    // api.send(reply.parse_mode(ParseMode::Html)).await?;
    Ok(())
}

async fn state(api: Api, message: Message) -> Result<(), Error> {

    let mut reply = message.text_reply(HELP);
    api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    let text = get_messege_text(&message);
    let info = state::systeminfo(&text).await;
    // let mut reply = message.text_reply("<b>Bold HTML message</b>");
    // api.send(reply.parse_mode(ParseMode::Html)).await?;
    Ok(())
}

fn set_parameter(path:&str,value:&str) -> String {
    format!("Ein parameter {} value {} wurde erfogreich geandert {}",path,"<simulate>",value)
}
fn get_parameter(path:&str) -> String {
    format!("{}","<empty>")
}
async fn set(api: Api, message: Message) -> Result<(), Error> {
    // let Vec<v> = text.split();
    // println!("API:{:?}",api);
    println!("SET:{:?}",&message);
    let msg = get_messege_text(&message);
    let text:Vec<&str> = msg.splitn(3, ' ').collect();
    match text.len() {
        2 => api.send(message.text_reply("Irgentwie klappt nich zu wenige parameter. Hier ist ein Beispiel: /set root.class.parname value")).await?,
        3 => api.send(message.text_reply(set_parameter(text[1],text[2]))).await?,
        _ => api.send( message.text_reply(SET).parse_mode(ParseMode::Markdown)).await?,  
    };
   Ok(())
}

async fn get(api: Api, message: Message) -> Result<(), Error> {
    println!("SET:{:?}",&message);
    let msg = get_messege_text(&message);
    let text:Vec<&str> = msg.splitn(2, ' ').collect();
    match text.len() {
        1 => api.send(message.text_reply("So kann ich nicht suchen.Es fellt parameter name. Hier ist ein Beispiel: /get root.class.parname")).await?,
        2 => api.send(message.text_reply(get_parameter(text[1]))).await?,
        _ => api.send( message.text_reply(GET).parse_mode(ParseMode::Markdown)).await?,  
    };
   Ok(())
}

async fn run(api: Api, message: Message) -> Result<(), Error> {
    println!("SET:{:?}",&message);
    let msg = get_messege_text(&message);
    let text:Vec<&str> = msg.splitn(2, ' ').collect();
    match text.len() {
        1 => api.send(message.text_reply("So kann ich nicht suchen.Es fellt parameter name. Hier ist ein Beispiel: /run ufo.irrigation")).await?,
        2 => api.send(message.text_reply(get_parameter(text[1]))).await?,
        _ => api.send(message.text_reply(RUN).parse_mode(ParseMode::Markdown)).await?,  
    };
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
    println!("CHAR:{:?}",&message);
    api.send(message.text_reply("Das verstehe ich nicht!")).await?;

    
    // let mut reply = message1.text_reply(HELP);
    // api.send(reply.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

/// ### 🔬 `@larpwa_bot`
/// ```
/// help  - usage information
/// status - status
/// set   - change parameter */set pwa.m1.name neue method name*
/// get   - parameter value and description */get pwa.m1.name* return method name
/// run   - command */run irrigatron.action.gp1.start*
/// mod   - mod */mod interval* for interval run
/// stop  - all runned comman and 🔄 to ready
/// ```
pub async fn message_handler(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/help"   => help(api, message).await?,
            "/state"  => state(api, message).await?,
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
