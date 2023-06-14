use dotenv::dotenv;
use haven::discord::bot::build_discord;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    build_discord().await;
}
