use crate::controller::catchers::{
    bad_request, default, forbidden, internal_error, not_found, status, unauthorized,
};
use rocket::{catchers, routes, Build, Rocket};

pub fn build_server() -> Rocket<Build> {
    rocket::build().mount("/", routes![status]).register(
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
