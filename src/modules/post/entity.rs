use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::user::User;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub author: User,
    pub scope: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
