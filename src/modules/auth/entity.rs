use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Tokens {
    pub access_token: String,
}
