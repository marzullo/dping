use std::env;

use serenity::framework::standard::macros::group;
use serenity::async_trait;

use serenity::model::prelude::{Ready, UserId};
use serenity::prelude::{EventHandler, Context};
use serenity::utils::MessageBuilder;

use crate::RECEIVER;

#[group]
pub struct General;

pub struct Handler;

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