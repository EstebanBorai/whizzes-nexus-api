use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::request::{FromRequest, Outcome};
use rocket::response::content;
use rocket::{Request, State};

use crate::graphql::Schema;
use crate::responders::cors::{Cors, CorsPreflight};

#[derive(Debug)]
pub struct AuthToken {
    token: Option<String>,
}

impl AuthToken {
    pub fn new(token: &str) -> Self {
        Self {
            token: Some(token.to_string()),
        }
    }

    pub fn empty() -> Self {
        Self { token: None }
    }

    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header = request.headers().get_one("Authorization");

        if let Some(auth_header_value) = authorization_header {
            let auth_token = AuthToken::new(auth_header_value);

            return Outcome::Success(auth_token);
        }

        Outcome::Success(AuthToken::empty())
    }
}

#[rocket::options("/<_..>")]
pub fn cors_preflight() -> CorsPreflight {
    Cors::preflight("http://localhost:3000")
}

#[rocket::get("/graphql")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<Schema>,
    request: GraphQLRequest,
    auth: AuthToken,
) -> GraphQLResponse {
    request.data(auth).execute(schema).await
}
