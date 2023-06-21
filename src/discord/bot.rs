use dotenv::dotenv;
use octocrab::Octocrab;
use poise::serenity_prelude::GatewayIntents;
use std::{env, time::Duration};

use super::{
    application::handler::Handler,
    command::{commits_command, pullrequests_command, repository_command, status_command},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub handler: Handler,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

pub async fn build_discord() {
    env_logger::init();
    dotenv().ok();

    let discord_token = env::var("DISCORD_TOKEN").expect("Discord token not found");
    let github_token = env::var("GITHUB_TOKEN").expect("GitHub token not found");

    let options = poise::FrameworkOptions {
        commands: vec![
            pullrequests_command::pullrequests(),
            repository_command::repository(),
            status_command::status(),
            commits_command::commits(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            additional_prefixes: vec![poise::Prefix::Literal("prefix_literal")],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(discord_token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    handler: Handler {
                        octocrab: Octocrab::builder()
                            .personal_token(github_token)
                            .build()
                            .expect("Failed to build Octocrab client"),
                    },
                })
            })
        })
        .options(options)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .run()
        .await
        .unwrap();
}
