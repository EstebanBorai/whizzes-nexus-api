use rocket::http::Method;
use rocket::response::{Responder, Response};
use std::collections::HashSet;

const ACCESS_CONTROL_ALLOW_CREDENTIALS: &str = "Access-Control-Allow-Credentials";
const ACCESS_CONTROL_ALLOW_HEADERS: &str = "Access-Control-Allow-Headers";
const ACCESS_CONTROL_ALLOW_METHODS: &str = "Access-Control-Allow-Methods";
const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";

pub struct Cors<R> {
    responder: R,
    allow_origin: &'static str,
    allow_credentials: bool,
    #[allow(dead_code)]
    allow_headers: HashSet<&'static str>,
    #[allow(dead_code)]
    allow_methods: HashSet<Method>,
}

impl<'r, R: Responder<'r, 'r>> Cors<R> {
    pub fn origin(responder: R, origin: &'static str) -> Self {
        Cors {
            responder,
            allow_origin: origin,
            allow_credentials: true,
            allow_methods: HashSet::default(),
            allow_headers: HashSet::default(),
        }
    }
}

impl<'r, R: Responder<'r, 'static>> Responder<'r, 'r> for Cors<R> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        let mut response = Response::build_from(self.responder.respond_to(request)?)
            .raw_header(ACCESS_CONTROL_ALLOW_ORIGIN, self.allow_origin)
            .finalize();

        if self.allow_credentials {
            response.set_raw_header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true");
        }

        response.set_raw_header(ACCESS_CONTROL_ALLOW_HEADERS, "*");
        response.set_raw_header(ACCESS_CONTROL_ALLOW_METHODS, "*");

        Ok(response)
    }
}

pub type CorsPreflight = Cors<()>;

impl CorsPreflight {
    pub fn preflight(origin: &'static str) -> CorsPreflight {
        Cors::origin((), origin)
    }
}
