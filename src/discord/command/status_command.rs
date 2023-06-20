use log::error;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[description = "Shows message if bot is working correctly."]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.reply(&ctx.http, "Hello, i'm here!").await {
        Ok(_) => {}
        Err(serenity_err) => {
            error!(
                "Received status command and there was an error while responding: {}",
                serenity_err
            );
        }
    };

    Ok(())
}
