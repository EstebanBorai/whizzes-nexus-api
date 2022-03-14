use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Token {
    pub token: String,
}
