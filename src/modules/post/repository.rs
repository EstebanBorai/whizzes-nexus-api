use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::poll::Poll;
use sqlx::{FromRow, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::database::Database;
use crate::error::Result;
use crate::modules::user::User;

use super::entity::Post;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct PostsTableRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub scope: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PostsTableRow> for Post {
    fn from(row: PostsTableRow) -> Self {
        Post {
            id: row.id,
            user_id: row.user_id,
            content: row.content,
            scope: row.scope,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Insertable, Queryable, Serialize)]
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
        let result: PostsTableRow = sqlx::query_as(
            r#"
                INSERT INTO posts (
                    content,
                    scope,
                    user_id,
                ) VALUES (
                    $1,
                    $2,
                    $3,
                ) RETURNING *"#,
        )
        .bind(dto.content)
        .bind(dto.scope)
        .bind(dto.user_id)
        .fetch_one(&self.database.conn_pool)
        .await?;

        Ok(Post::from(result))
    }

    pub async fn find_by_author(&self, author_id: &Uuid) -> Result<Vec<PostsTableRow>> {
        let conn = self.database.conn_pool.get()?;
        let posts = posts::table
            .filter(posts::user_id.eq(author_id))
            .load::<PostsTableRow>(&conn)?;

        Ok(posts)
    }
}
