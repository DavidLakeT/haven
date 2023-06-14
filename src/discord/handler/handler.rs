use octocrab::{models::Repository, Octocrab};
use serenity::{
    async_trait,
    http::Http,
    model::prelude::{ChannelId, Ready},
    prelude::{Context, EventHandler, TypeMapKey},
};

#[derive(Clone)]
pub struct Handler {
    pub octocrab: Octocrab,
}

impl TypeMapKey for Handler {
    type Value = Handler;
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

impl Handler {
    pub async fn get_repository_info(
        &self,
        http: &Http,
        channel_id: ChannelId,
    ) -> Result<Repository, octocrab::Error> {
        self.octocrab.repos("DavidLakeT", "haven").get().await
    }
}
