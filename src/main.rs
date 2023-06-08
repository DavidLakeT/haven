use dotenv::dotenv;
use haven::utils::server::build_server;
use rocket::launch;

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    dotenv().ok();
    build_server()
}
