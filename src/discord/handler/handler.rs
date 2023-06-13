use std::sync::Arc;

use serenity::{
    async_trait,
    model::prelude::{ChannelId, Message, Ready},
    prelude::{Context, EventHandler, Mutex},
};

pub struct Handler {
    pub desired_channel_id: Arc<Mutex<Option<ChannelId>>>,
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
        let user = msg.author;
        let prefix = "!";

        if !msg.content.starts_with(prefix) {
            if !user.bot {
                let desired_channel_id = {
                    let desired_channel_id = self.desired_channel_id.lock().await;
                    *desired_channel_id
                };

                if let Some(channel_id) = desired_channel_id {
                    if msg.channel_id == channel_id {
                        println!("Mensaje recibido en #development: {:?}", msg.content);
                    }
                }
            }
        } else {
            println!("Comando recibido en #development: {:?}", msg.content);
        }
    }
}
