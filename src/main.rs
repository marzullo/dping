pub mod http;

use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use once_cell::sync::OnceCell;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

use serenity::model::prelude::{Ready, UserId};

use serenity::async_trait;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tokio::sync::mpsc::Receiver;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        let http = &_ctx.http;

        let mut queue = RECEIVER.get().unwrap().lock().await;

        let user_id = UserId(str::parse(&env::var("USER_ID").unwrap()).unwrap());

        loop {
            let message = queue.recv().await;

            let channel = user_id.create_dm_channel(http).await;

            let message = MessageBuilder::new()
                .mention(&user_id)
                .push(" ".to_owned() + &message.unwrap())
                .build();

            let res = channel
                .unwrap()
                .send_message(http, |m| m.content(message))
                .await;

            println!("{:?}", res);
        }
    }
}

static RECEIVER: OnceCell<Arc<Mutex<Receiver<String>>>> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();

    let message_queue = tokio::sync::mpsc::channel::<String>(10);

    RECEIVER.set(Arc::new(Mutex::new(message_queue.1))).unwrap();

    let discord_thread = tokio::spawn(async move {
        // Login with a bot token from the environment
        let token = env::var("DISCORD_TOKEN").expect("token");
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

        let framework = StandardFramework::new()
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP);

        let mut client = Client::builder(token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await
            .expect("Error creating client");

        // start listening for events by starting a single shard
        if let Err(why) = client.start().await {
            println!("An error occurred while running the client: {:?}", why);
        }
    });

    let web_thread = tokio::spawn(http::main_web(message_queue.0));

    discord_thread.await;
    web_thread.await;
}
