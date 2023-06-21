use crate::discord::bot::{Context, Error};

/// Shows message if bot is working correctly.
#[poise::command(slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|m| m.content("Hello, i'm here!")).await?;
    Ok(())
}
