use crate::discord::bot::{Context, Error};
use log::error;

/// Shows information about a Git repository.
#[poise::command(slash_command)]
pub async fn repository(
    ctx: Context<'_>,
    #[description = "Github username of the repository owner"] repository_owner: String,
    #[description = "Name of the Github repository"] repository_name: String,
) -> Result<(), Error> {
    let handler = &ctx.data().handler;

    match handler
        .get_repository_info(repository_owner.clone(), repository_name)
        .await
    {
        Ok(repository) => {
            ctx.send(|m| {
                m.embed(|e| {
                    e.color(0x0099FF)
                        .title("Click here to view repository")
                        .field("Name", repository.full_name.unwrap(), true)
                        .field("Stars", repository.stargazers_count.unwrap(), true)
                        .field("Forks", repository.forks_count.unwrap(), true)
                        .field("Description", repository.description.unwrap(), true)
                        .image(format!(
                            "https://avatars.githubusercontent.com/{repository_owner}"
                        ))
                        .url(repository.html_url.unwrap())
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
                error!("There was an error: {}", serenity_err);
            }
        },
    }

    Ok(())
}
