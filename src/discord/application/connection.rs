use poise::serenity_prelude as serenity;
use serenity::model::prelude::command::{Command, CommandOptionType};

pub struct Connection<'a> {
    pub client: &'a serenity::Client,
}

impl Connection<'_> {
    pub async fn register_global_commands(&self) -> Result<(), serenity::Error> {
        match Command::create_global_application_command(
            &self.client.cache_and_http.http,
            |command| {
                command
                    .name("repository")
                    .description("Shows information about a Git repository")
                    .create_option(|option| {
                        option
                            .name("repository_owner")
                            .description("Github username of the repository owner")
                            .kind(CommandOptionType::String)
                            .required(true)
                            .create_sub_option(|sub_option| {
                                sub_option
                                    .name("repository_name")
                                    .description("Name of the Github repository")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                            })
                    })
            },
        )
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
