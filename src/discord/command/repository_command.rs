use serenity::{
    framework::standard::{macros::command, CommandResult, Args},
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn repository(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}