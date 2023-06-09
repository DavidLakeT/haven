use super::handler::handler::Handler;
use crate::discord::command::hello_command::HELLO_COMMAND;
use crate::discord::command::metrics_command::METRICS_COMMAND;
use crate::discord::command::ping_command::PING_COMMAND;
use serenity::{
    framework::standard::{macros::group, StandardFramework},
    model::prelude::ChannelId,
    prelude::GatewayIntents,
    prelude::*,
};
use std::{env, sync::Arc};

#[group]
#[commands(ping, metrics, hello)]
struct General;

pub async fn build_discord() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP);

    let desired_channel_id: Arc<Mutex<Option<ChannelId>>> = Arc::new(Mutex::new(None));
    let handler = Handler {
        desired_channel_id: desired_channel_id.clone(),
    };

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
