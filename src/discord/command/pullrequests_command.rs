use crate::discord::bot::{Context, Error};
use log::error;

/// Lists last pull requests from a Git repository.
#[poise::command(slash_command)]
pub async fn pullrequests(
    ctx: Context<'_>,
    #[description = "Github username of the repository owner"] repository_owner: String,
    #[description = "Name of the Github repository"] repository_name: String,
) -> Result<(), Error> {
    let handler = &ctx.data().handler;

    match handler
        .get_repository_pull_requests(repository_owner.clone(), repository_name)
        .await
    {
        Ok(pull_requests) => {
            if pull_requests.is_empty() {
                ctx.send(|m| m.content("No pull requests found.")).await?;
                return Ok(());
            }

            ctx.send(|m| {
                m.embed(|e| {
                    let pull_requests = pull_requests.into_iter().take(5);
                    for (idx, pull_request) in pull_requests.enumerate() {
                        e.field(
                            format!("Pull request #{}", idx + 1),
                            pull_request.title.unwrap(),
                            false,
                        );
                    }

                    e.color(0x0099FF)
                        .title("Last pull requests info")
                        .image(format!(
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
