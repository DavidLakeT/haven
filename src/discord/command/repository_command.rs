use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::discord::handler::handler::Handler;

#[command]
#[description = "Shows information about a Git repository."]
async fn repository(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let handler = data.get::<Handler>().unwrap();

    let repository_author = match args.single::<String>() {
        Ok(author) => author,
        Err(_) => {
            msg.reply(ctx, "An argument is required to run this command.")
                .await?;
            return Ok(());
        }
    };

    let repository_name = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            msg.reply(ctx, "An argument is required to run this command.")
                .await?;
            return Ok(());
        }
    };

    match handler.get_repository_info(repository_author.clone(), repository_name).await {
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
                            .title("Click here to view repository")
                            //.author(|a| a.name("Nombre del autor").icon_url("https://i.pinimg.com/474x/fa/10/cd/fa10cdaaf6d4303fcd96217a69a6b078.jpg"))
                            .field("Name", repository.full_name.unwrap(), true)
                            .field("Stars", repository.stargazers_count.unwrap(), true)
                            .field("Forks", repository.forks_count.unwrap(), true)
                            .field("Description", repository.description.unwrap(), true)
                            .image(format!("https://avatars.githubusercontent.com/{}", repository_author))
                            .url(repository.html_url.unwrap())
                        })
                })
                .await;
        }
        Err(_) => match msg.channel_id.say(&ctx.http, "Error").await {
            _ => {}
        },
    }

    Ok(())
}
