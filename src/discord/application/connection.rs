use std::env;

pub struct Connection {
    pub client: reqwest::Client,
}

impl Connection {
    pub async fn register_commands(
        &self
    ) -> Result<(), reqwest::Error> {

        /*
        let response = self.client
            .post(format!("https://discord.com/api/v10/applications/{}/commands", 1059938872370397286))
            .header(reqwest::header::AUTHORIZATION, format!("Bot {}", env::var("DISCORD_TOKEN").expect("Discord token not found");))
            .json(command_data)
            .send()
            .await?;
         */

        Ok(())
    }
}