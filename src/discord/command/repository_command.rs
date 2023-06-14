use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::{Channel, Message},
    prelude::Context,
};

use crate::discord::handler::handler::Handler;

#[command]
async fn repository(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let handler = data.get::<Handler>().unwrap();

    if let Ok(repository_name) = args.single::<String>() {
        match handler.get_repository_info(repository_name).await {
            Ok(repository) => {
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
                            e.color(0x0099FF)
                                .title("Repository info")
                                .field("Name", repository.name, false)
                                .field("Stars", repository.stargazers_count.unwrap(), false)
                                .thumbnail("http://www.elnuestrabelendeumbria.edu.co/imagenes/juandavid.png")
                        })
                    })
                    .await;
            }
            Err(_) => match msg.channel_id.say(&ctx.http, "Error").await {
                _ => {}
            },
        }
    }

    Ok(())
}
