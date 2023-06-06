use rocket::http::Status;
use rocket::{catch, get, Request};

#[get("/status")]
pub async fn status() -> &'static str {
    "Ok"
}

#[catch(400)]
pub fn bad_request(req: &Request) -> String {
    format!("Bad request '{}'", req.uri())
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> String {
    format!("Unauthorized access '{}'", req.uri())
}

#[catch(403)]
pub fn forbidden(req: &Request) -> String {
    format!("Forbidden access '{}'", req.uri())
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Resource not found '{}'", req.uri())
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Internal server error"
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("Unknown error: {} ({})", status, req.uri())
}
