use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::request::{FromRequest, Outcome};
use rocket::response::content;
use rocket::{Request, State};

use crate::error::{Error, Result};
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

    pub fn from_header(value: &str) -> Self {
        let parts = value.split(' ').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Self::empty();
        }

        if let Some(scheme) = parts.get(0) {
            let token = *parts.get(1).unwrap();

            if (*scheme) == "JWT" && !token.is_empty() {
                return Self::new(token);
            }
        }

        Self::empty()
    }

    pub fn empty() -> Self {
        Self { token: None }
    }

    pub fn token(&self) -> Result<String> {
        if let Some(token) = &self.token {
            return Ok(token.to_string());
        }

        Err(Error::unauthorized())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header = request.headers().get_one("Authorization");

        if let Some(auth_header_value) = authorization_header {
            let auth_token = AuthToken::from_header(auth_header_value);

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

#[cfg(test)]
mod tests {
    use super::AuthToken;

    #[test]
    fn token_from_auth_header() {
        let auth_header = "JWT MyCoolToken";
        let auth_token = AuthToken::from_header(auth_header);

        assert_eq!(auth_token.token, Some(String::from("MyCoolToken")));
    }

    #[test]
    fn token_is_none_on_invalid_scheme() {
        let auth_header = "Bearer MyCoolToken";
        let auth_token = AuthToken::from_header(auth_header);

        assert_eq!(auth_token.token, None);
    }

    #[test]
    fn token_is_none_on_invalid_header_value() {
        let auth_header = "MyCoolToken";
        let auth_token = AuthToken::from_header(auth_header);

        assert_eq!(auth_token.token, None);
    }

    #[test]
    fn token_is_none_on_empty_string() {
        let auth_header = "";
        let auth_token = AuthToken::from_header(auth_header);

        assert_eq!(auth_token.token, None);
    }
}
