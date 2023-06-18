use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::discord::application::handler::Handler;

#[command]
async fn commits(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
        .get_repository_commits(repository_author.clone(), repository_name)
        .await
    {
        Ok(commits) => {
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");
                    return Ok(());
                }
            };

            let mut embed = CreateEmbed::default();
            embed.color(0x0099FF).title("Commits info").image(format!(
                "https://avatars.githubusercontent.com/{repository_author}"
            ));

            let commits = commits.into_iter().take(5);
            for (idx, commit) in commits.enumerate() {
                embed.field(format!("Commit #{}", idx + 1), commit.commit.message, false);
            }

            let _ = channel
                .guild()
                .unwrap()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await;
        }
        Err(_) => {
            let _ = msg.channel_id.say(&ctx.http, "Error").await;
        }
    }

    Ok(())
}
