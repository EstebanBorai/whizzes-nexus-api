use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

const ACCESS_CONTROL_ALLOW_CREDENTIALS: &str = "Access-Control-Allow-Credentials";
const ACCESS_CONTROL_ALLOW_HEADERS: &str = "Access-Control-Allow-Headers";
const ACCESS_CONTROL_ALLOW_METHODS: &str = "Access-Control-Allow-Methods";
const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN, "*"));
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_CREDENTIALS, "*"));
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_HEADERS, "*"));
        response.set_header(Header::new(ACCESS_CONTROL_ALLOW_METHODS, "*"));
    }
}
