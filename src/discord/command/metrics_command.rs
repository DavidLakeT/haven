use serenity::prelude::Mentionable;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn metrics(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = match args.single::<serenity::model::id::UserId>() {
        Ok(user) => user,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Por favor, menciona a un usuario válido.")
                .await?;
            return Ok(());
        }
    };

    let user = match user_id.to_user(&ctx).await {
        Ok(user) => user,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "No se pudo encontrar al usuario.")
                .await?;
            return Ok(());
        }
    };

    let metrics_message = format!(
        "Métricas del usuario {}:\n\n\
        Nombre de usuario: {}\n\
        Apodo en el servidor: {}\n",
        user.mention(),
        user.name,
        user.nick_in(&ctx.http, msg.guild_id.unwrap())
            .await
            .unwrap_or_else(|| String::from("N/A")),
    );

    msg.channel_id.say(&ctx.http, metrics_message).await?;

    Ok(())
}
