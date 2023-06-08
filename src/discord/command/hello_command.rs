use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    let embed = "hola";

    if let Err(why) = msg.channel_id.say(&ctx.http, &embed).await {
        println!("Error al enviar el mensaje: {:?}", why);
    }

    Ok(())
}
