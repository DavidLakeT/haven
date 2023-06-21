use serenity::model::prelude::command::{Command, CommandOptionType};

pub struct Connection {
    pub client: reqwest::Client,
}

impl Connection {
    pub async fn register_commands(
        &self,
        client: serenity::Client
    ) -> Result<(), reqwest::Error> {
        let _ = Command::create_global_application_command(&client.cache_and_http.http, 
            |command| {
                command
                    .name("repository")
                    .description("Shows information about a Git repository")
                    .create_option(|option| {
                        option
                            .name("repository owner")
                            .description("Github username of the repository owner")
                            .kind(CommandOptionType::String)
                            .required(true)
                            .create_sub_option(|sub_option| {
                                sub_option
                                    .name("repository name")
                                    .description("Name of the Github repository")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                            })
                    })
        })
        .await;

        Ok(())
    }
}