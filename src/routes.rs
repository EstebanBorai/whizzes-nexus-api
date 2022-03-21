use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::response::content;
use rocket::State;

use crate::graphql::Schema;
use crate::responders::cors::{Cors, CorsPreflight};

#[rocket::options("/<_..>")]
pub fn cors_preflight() -> CorsPreflight {
    Cors::preflight("http://localhost:3000")
}

#[rocket::get("/graphql")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<Schema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(schema: &State<Schema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}
