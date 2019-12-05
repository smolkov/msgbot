use std::env;
use std::io;
use bot_framework::{BotWrapper, BotHandler};
use telegram_bot::prelude::*;
use telegram_bot::types::*;
use telegram_bot::Api;




pub struct TeleBot {
    hello: String ,
    key: String,
    github: String,

}

impl TeleBot {
    pub fn new(key:&str) -> TeleBot {
        TeleBot {
            hello:"Hello".to_owned(),
            key:key.to_owned(),
            github: "empty".to_owned(),
        }
    }
}

impl BotHandler for TeleBot {
    fn inline_query(&self, api: &Api, query: InlineQuery) {
        let input_text_message_content = InputTextMessageContent {
            message_text: query.query.clone(),
            parse_mode: Some(telegram_bot::ParseMode::Markdown),
            disable_web_page_preview: true,
        };
        println!("inline query");
        let mut article = InlineQueryResultArticle::new(
            format!("{}", query.from.id),
            format!("Hello, User!"),
            input_text_message_content,
        );
        article.description(format!("This is an inline query result article"));

        let mut ans = query.answer(vec![]);
        ans.add_inline_result(article);
        api.spawn(ans);
    }
}


pub fn start(telebot:TeleBot) -> io::Result<()> {
    // let token = env::var("TELEGRAM_BOT_KEY").expect("TELEGRAM_BOT_KEY not found in env");
    let token = telebot.key.to_owned();
    let mut bot = BotWrapper::new_with_handler(token,telebot);
    bot.command("hello".into(), |api, msg| {
        api.spawn(msg.text_reply(format!("Hello, {}!", &msg.from.first_name)));
    });
    bot.command("set".into(),|api,msg| {
        log::info!("set parameter {} = {}","empty","10");
    });
    bot.run();
    Ok(())
}