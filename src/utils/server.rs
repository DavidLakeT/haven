use crate::webapi::controller::catchers::{
    bad_request, default, forbidden, internal_error, not_found, status, unauthorized,
};
use rocket::{catchers, routes, Build, Rocket};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Header, Method, Status},
    Request, Response,
};

pub struct CorsPolicy;

#[rocket::async_trait]
impl Fairing for CorsPolicy {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new(
            "Access-Control-Expose-Headers",
            "*, Authorization, Content-Type, User",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(0, std::io::Cursor::new(""));
            response.set_status(Status::Ok);
        }
    }
}

pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CorsPolicy)
        .mount("/", routes![status])
        .register(
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
