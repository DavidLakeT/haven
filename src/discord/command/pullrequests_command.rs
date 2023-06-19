use crate::discord::application::handler::Handler;
use log::error;
use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[description = "Lists last pull requests from a Git repository."]
async fn pullrequests(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    match handler
        .get_repository_pull_requests(repository_author.clone(), repository_name)
        .await
    {
        Ok(pull_requests) => {
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");
                    return Ok(());
                }
            };

            let mut embed = CreateEmbed::default();
            embed
                .color(0x0099FF)
                .title("Last pull requests info")
                .image(format!(
                    "https://avatars.githubusercontent.com/{repository_author}"
                ));

            let pull_requests = pull_requests.into_iter().take(5);
            for (idx, pull_request) in pull_requests.enumerate() {
                embed.field(
                    format!("Pull request #{}", idx + 1),
                    pull_request.title.unwrap(),
                    false,
                );
            }

            let _ = channel
                .guild()
                .unwrap()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await;
        }
        Err(_) => match msg.channel_id.say(&ctx.http, "Error: Not found").await {
            Ok(_) => {}
            Err(serenity_err) => {
                error!("There was an error: {serenity_err:?}");
            }
        },
    }

    Ok(())
}
