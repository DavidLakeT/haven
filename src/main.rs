use dotenv::dotenv;
use haven::{discord::bot::init_bot, utils::server::build_server};

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    init_bot().await;
    build_server()
}
