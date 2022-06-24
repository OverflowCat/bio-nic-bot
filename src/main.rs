pub mod bionic;
pub mod text;
pub mod zh;

use std::env;

use teloxide::dptree::endpoint;
use teloxide::prelude::*;
use teloxide::types::*;

use crate::bionic::bionify;
use crate::zh::tokenize;
use crate::text::{escape_all_markdown_v2, escape_markdown_v2};

fn gen_res(text: &str) -> Vec<InlineQueryResult> {
    /*
    let escaped: String;
    if query.from.username == Some("xxxxx".to_string()) {
        println!("USERNAME: {}", query.from.username.as_ref().unwrap());
        escaped = escape_all_markdown_v2(&query.query);
    } else {
        escaped = escape_markdown_v2(&query.query);
    }
    */
    let bionic = bionify(&text);
    let result_article = InlineQueryResultArticle::new(
        // Each item needs a unique ID, as well as the response container for the
        // items. These can be whatever, as long as they don't conflict.
        "01".to_string(),
        // What the user will actually see
        "Bionic writing",
        // What message will be sent when clicked/tapped
        InputMessageContent::Text(
            InputMessageContentText::new(&bionic).parse_mode(ParseMode::MarkdownV2),
        ),
    )
    .thumb_url(
        "http://telegra.ph/file/7cae60e9c68a995866fb3.png"
            .parse()
            .unwrap(),
    )
    .description("Send a bionic reading message");
    
    let tokenized = tokenize(&text);
    let bionified: Vec<String> = tokenized.iter().map(|x| bionify(x)).collect();
    let bionified = bionified.join("");
    let result_article_zh = InlineQueryResultArticle::new(
        "zh",
        "Jieba",
        InputMessageContent::Text(
            InputMessageContentText::new(bionified).parse_mode(ParseMode::MarkdownV2),
        ),
    ).description("Send with jieba");

    vec![
        InlineQueryResult::Article(result_article),
        InlineQueryResult::Article(result_article_zh)
    ]
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    let token = env::var("BIO_NIC_BOT_TOKEN").expect("BIO_NIC_BOT_TOKEN not set");
    let bot = Bot::new(token).auto_send();
    let inline_handler = Update::filter_inline_query().branch(endpoint(
        |query: InlineQuery, bot: AutoSend<Bot>| async move {
            if query.query.is_empty() {
                return respond(());
            }
            println!("Text: {}", query.query);
            let escaped = escape_markdown_v2(&query.query);
            let results = gen_res(&escaped);
            let response = bot.answer_inline_query(&query.id, results).send().await;
            if let Err(err) = response {
                log::error!("Error first try: {:?}", err);
                let escaped = escape_all_markdown_v2(&query.query);
                let results = gen_res(&escaped);
                let response = bot.answer_inline_query(&query.id, results).send().await;
                if let Err(err) = response {
                    log::error!("Error second try: {:?}", err);
                }
            }
            respond(())
        },
    ));

    // let message_handler = Update::filter_message().branch(endpoint(message_handler));

    let handler = dptree::entry()
        // .branch(message_handler)
        .branch(inline_handler);
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
