pub mod bionic;
pub mod text;

use std::env;

use teloxide::dptree::endpoint;
use teloxide::prelude::*;
use teloxide::types::*;

use crate::bionic::bionify;
use crate::text::escape_markdown_v2;

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
            let escaped = escape_markdown_v2(&query.query);
            let bionic = bionify(&escaped);
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
            // While constructing them from the struct itself is possible, it is preferred
            // to use the builder pattern if you wish to add more
            // information to your result. Please refer to the documentation
            // for more detailed information about each field. https://docs.rs/teloxide/latest/teloxide/types/struct.InlineQueryResultArticle.html

            let results = vec![
                InlineQueryResult::Article(result_article),
                // InlineQueryResult::Article(ddg_search),
            ];

            // Send it off! One thing to note -- the ID we use here must be of the query
            // we're responding to.
            let response = bot.answer_inline_query(&query.id, results).send().await;
            if let Err(err) = response {
                log::error!("Error in handler: {:?}", err);
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
