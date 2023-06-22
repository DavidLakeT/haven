use crate::discord::bot::{Context, Error};
use log::error;

/// Lists last commits from a Git repository.
#[poise::command(slash_command)]
pub async fn commits(
    ctx: Context<'_>,
    #[description = "Github username of the repository owner"] repository_owner: String,
    #[description = "Name of the Github repository"] repository_name: String,
) -> Result<(), Error> {
    let handler = &ctx.data().handler;

    match handler
        .get_repository_commits(repository_owner.clone(), repository_name)
        .await
    {
        Ok(commits) => {
            if commits.is_empty() {
                ctx.send(|m| m.content("No commits found.")).await?;
                return Ok(());
            }

            ctx.send(|m| {
                m.embed(|e| {
                    let commits = commits.into_iter().take(5);
                    for (idx, commit) in commits.enumerate() {
                        e.field(format!("Commit #{}", idx + 1), commit.commit.message, false);
                    }

                    e.color(0x0099FF).title("Last commits info").image(format!(
                        "https://avatars.githubusercontent.com/{repository_owner}"
                    ))
                })
            })
            .await?;
        }
        Err(_) => match ctx
            .channel_id()
            .say(&ctx.serenity_context().http, "Error: Not found")
            .await
        {
            Ok(_) => {}
            Err(serenity_err) => {
                error!("There was an error: {serenity_err:?}");
            }
        },
    }

    Ok(())
}
