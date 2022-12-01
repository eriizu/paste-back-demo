use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::Body;
use rocket::{http, Request, Response};

pub struct CorsFairing;

#[rocket::async_trait]
impl Fairing for CorsFairing {
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(http::Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(http::Header::new("Access-Control-Expose-Headers", "*"));
        response.set_header(http::Header::new("Access-Control-Allow-Headers", "*"));

        if response.status() == http::Status::NotFound && request.method() == http::Method::Options
        {
            response.set_status(http::Status::NoContent);
            response.remove_header("content-type");
            let body = response.body_mut();
            *body = Body::default();
        }
    }

    fn info(&self) -> Info {
        Info {
            name: "Custom CORS Fairing",
            kind: Kind::Response,
        }
    }
}
