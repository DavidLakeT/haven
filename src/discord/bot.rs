use super::handler::handler::Handler;
use crate::discord::command::hello_command::HELLO_COMMAND;
use crate::discord::command::metrics_command::METRICS_COMMAND;
use crate::discord::command::ping_command::PING_COMMAND;
use crate::discord::command::repository_command::REPOSITORY_COMMAND;
use crate::discord::command::commits_command::COMMITS_COMMAND;
use octocrab::Octocrab;
use serenity::{
    framework::standard::{macros::group, StandardFramework},
    prelude::GatewayIntents,
    prelude::*,
};
use std::env;

#[group]
#[commands(ping, metrics, hello, repository, commits)]
struct General;

pub async fn build_discord() {
    let discord_token = env::var("DISCORD_TOKEN").expect("token");
    let github_token = env::var("GITHUB_TOKEN").expect("GitHub token not found");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP);

    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .expect("Failed to build Octocrab client");

    let handler = Handler { octocrab };

    let mut client = Client::builder(discord_token, intents)
        .event_handler(handler.clone())
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Handler>(handler.clone());
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
