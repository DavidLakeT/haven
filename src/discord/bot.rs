use super::application::handler::Handler;
use crate::discord::group::general_group::GENERAL_GROUP;
use dotenv::dotenv;
use octocrab::Octocrab;
use serenity::{
    framework::standard::StandardFramework,
    prelude::GatewayIntents,
    prelude::*,
};
use std::env;

pub async fn build_discord() {
    env_logger::init();
    dotenv().ok();

    let discord_token = env::var("DISCORD_TOKEN").expect("Discord token not found");
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

    let shards = 1;

    {
        let mut data = client.data.write().await;
        data.insert::<Handler>(handler.clone());
    }

    if let Err(why) = client.start_shards(shards).await {
        println!("An error occurred while running the client: {why:?}");
    }
}
