use std::{env, sync::Arc};

use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{
        channel::Message,
        prelude::{ChannelId, Ready},
    },
    prelude::GatewayIntents,
    prelude::*,
};

#[group]
#[commands(ping)]
struct General;

struct Handler {
    desired_channel_id: Arc<Mutex<Option<ChannelId>>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(guild_id) = ready.guilds.get(0).map(|guild| guild.id) {
            if let Some(guild) = ready.guilds.iter().find(|g| g.id == guild_id) {
                if let Ok(channels) = guild.id.channels(&ctx.http).await {
                    if let Some((channel_id, _)) =
                        channels.iter().find(|(_, c)| c.name == "development")
                    {
                        let channel_id = channel_id;
                        let mut desired_channel_id = self.desired_channel_id.lock().await;
                        *desired_channel_id = Some(*channel_id);
                    }
                }
            }
        }
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        let desired_channel_id = {
            let desired_channel_id = self.desired_channel_id.lock().await;
            *desired_channel_id
        };

        if let Some(channel_id) = desired_channel_id {
            if msg.channel_id == channel_id {
                println!("Mensaje recibido en el canal deseado: {:?}", msg.content);
            }
        }
    }
}

pub async fn init_bot() {
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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
