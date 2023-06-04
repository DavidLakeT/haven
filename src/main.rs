use haven::utils::server::build_server;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    build_server()
}
