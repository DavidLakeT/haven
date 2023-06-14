use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::discord::handler::handler::Handler;

#[command]
async fn repository(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let handler = data.get::<Handler>().unwrap();

    handler
        .get_repository_info(&ctx.http, msg.channel_id)
        .await?;

    Ok(())
}
