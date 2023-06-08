use crate::{webapi::controller::catchers::{
    bad_request, default, forbidden, internal_error, not_found, status, unauthorized,
}, discord::bot::init_bot};
use rocket::{catchers, routes, Build, Rocket, fairing::{Fairing, Info, Kind}, tokio};

pub struct DiscordBot;

#[rocket::async_trait]
impl Fairing for DiscordBot {
    fn info(&self) -> Info {
        Info {
            name: "Bot Initialization",
            kind: Kind::Liftoff,
        }
    }
}

pub fn build_server() -> Rocket<Build> {
    rocket::build()
        .attach(DiscordBot)
        .mount("/", routes![status]).register(
            "/",
            catchers![
                bad_request,
                unauthorized,
                forbidden,
                not_found,
                internal_error,
                default
            ],
        )
}
