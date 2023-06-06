use haven::{utils::server::build_server, discord::bot::init_bot};
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    init_bot().await;
    build_server()
}
