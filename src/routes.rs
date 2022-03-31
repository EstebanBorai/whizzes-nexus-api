use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::request::{FromRequest, Outcome};
use rocket::response::content;
use rocket::{Request, State};

use crate::graphql::Schema;
use crate::responders::cors::{Cors, CorsPreflight};

#[derive(Debug)]
pub struct AuthToken {
    token: String,
}

impl AuthToken {
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header_value) = request.headers().get_one("Authorization") {
            return Outcome::Success(AuthToken {
                token: auth_header_value.into(),
            });
        }

        Outcome::Forward(())
    }
}

#[rocket::options("/<_..>")]
pub fn cors_preflight() -> CorsPreflight {
    Cors::preflight("http://localhost:3000")
}

#[rocket::post("/graphql")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<Schema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<Schema>,
    request: GraphQLRequest,
    auth: AuthToken,
) -> GraphQLResponse {
    request.data(auth).execute(schema).await
}
