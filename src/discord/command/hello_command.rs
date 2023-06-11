use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
    utils::MessageBuilder,
};

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return Ok(());
        }
    };

    let response = MessageBuilder::new()
        .quote_rest()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push(" used the 'hello' command in the ")
        .mention(&channel)
        .push(" channel")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
