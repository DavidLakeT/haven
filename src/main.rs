use haven::discord::bot::build_discord;

#[tokio::main]
async fn main() {
    build_discord().await;
}
