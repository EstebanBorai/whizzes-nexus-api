use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::database::Database;
use crate::error::Result;
use crate::modules::user::User;

use super::entity::Post;

pub struct PostsTableRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub scope: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "posts"]
pub struct InsertPostTableRow {
    pub content: String,
    pub scope: String,
    pub user_id: Option<Uuid>,
}

pub struct PostRepository {
    database: Arc<Database>,
}

impl PostRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn insert(&self, user: User, dto: InsertPostTableRow) -> Result<Post> {
        let conn = self.database.conn_pool.get()?;
        let dto = InsertPostTableRow {
            content: dto.content.to_string(),
            scope: dto.scope.to_string(),
            user_id: Some(user.id.clone()),
        };
        let row = diesel::insert_into(posts::table)
            .values(dto)
            .get_result::<PostsTableRow>(&conn)?;
        let post = Post {
            id: row.id,
            content: row.content,
            scope: row.scope,
            author: user,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        Ok(post)
    }

    pub async fn find_by_author(&self, author_id: &Uuid) -> Result<Vec<PostsTableRow>> {
        let conn = self.database.conn_pool.get()?;
        let posts = posts::table
            .filter(posts::user_id.eq(author_id))
            .load::<PostsTableRow>(&conn)?;

        Ok(posts)
    }
}
