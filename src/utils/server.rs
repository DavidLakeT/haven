use rocket::http::Status;
use rocket::Request;
use rocket::{catch, catchers, get, routes, Build, Rocket};

#[get("/status")]
pub async fn status() -> &'static str {
    "Ok"
}

#[catch(400)]
fn bad_request(req: &Request) -> String {
    format!("Bad request '{}'", req.uri())
}

#[catch(401)]
fn unauthorized(req: &Request) -> String {
    format!("Unauthorized access '{}'", req.uri())
}

#[catch(403)]
fn forbidden(req: &Request) -> String {
    format!("Forbidden access '{}'", req.uri())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Resource not found '{}'", req.uri())
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal server error"
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("Unknown error: {} ({})", status, req.uri())
}

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
