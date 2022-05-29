use async_graphql::dataloader::Loader;
use async_graphql::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::database::Database;
use crate::error::Error;
use crate::modules::user::{User, UsersTableRow};

pub struct UserLoader {
    database: Arc<Database>,
}

impl UserLoader {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UserLoader {
    type Value = User;
    type Error = Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let query = format!(
            "SELECT * FROM users WHERE id IN ({})",
            keys.iter()
                .map(|uuid| { format!("'{}'::uuid", uuid) })
                .join(",")
        );
        let result: Vec<UsersTableRow> = sqlx::query_as(&query)
            .fetch_all(&self.database.conn_pool)
            .await?;
        let mut res: HashMap<Uuid, User> = HashMap::new();

        result.into_iter().for_each(|u: UsersTableRow| {
            res.insert(u.id, User::from(u));
        });

        Ok(res)
    }
}
