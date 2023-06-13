use dotenv::dotenv;
use haven::{discord::bot::build_discord, github::server::get_repository};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    build_discord().await;
}