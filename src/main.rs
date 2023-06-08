use dotenv::dotenv;
use haven::{discord::bot::init_bot, utils::server::build_server};
use rocket::tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let server_task = tokio::spawn(async {
        build_server().launch().await.map_err(|e| {
            eprintln!("Rocket error: {:?}", e);
        })
    });

    let bot_task = tokio::spawn(async {
        init_bot().await;
    });

    let _ = tokio::try_join!(server_task, bot_task)?;

    Ok(())
}
