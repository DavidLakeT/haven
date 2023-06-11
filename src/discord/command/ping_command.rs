use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return Ok(());
        }
    };

    let _ = channel
        .guild()
        .unwrap()
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.color(0x0099FF).title("Comandos de BotBoy").field(
                    "Menú help",
                    format!(
                        "Tenemos {} categorías y {} comandos para explorar.",
                        "0",
                        "0"
                    ),
                    false,
                )
            })
        })
        .await;

    Ok(())
}
