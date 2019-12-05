use yansi::Paint;
use std::env;

use msgbot;

fn main() {
    let token = env::var("TELEGRAM_BOT_KEY").expect("TELEGRAM_BOT_KEY not found in env");
    let mut bot = msgbot::TeleBot::new(token.as_str());
    msgbot::telebot::start(bot);
}
