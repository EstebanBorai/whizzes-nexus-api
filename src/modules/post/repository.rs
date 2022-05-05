use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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

#[derive(Debug, Deserialize, Serialize)]
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

    pub async fn find_by_author(&self, author_id: &Uuid) -> Result<Vec<PostsTableRow>> {
        let result: Vec<PostsTableRow> = sqlx::query_as("SELECT * FROM posts WHERE user_id = $1")
            .bind(author_id)
            .fetch_all(&self.database.conn_pool)
            .await?;
        let posts = result
            .into_iter()
            .map(|row| PostsTableRow {
                id: row.id,
                user_id: row.user_id,
                content: row.content,
                scope: row.scope,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect::<Vec<PostsTableRow>>();
        Ok(posts)
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

        Ok(Post {
            id: result.id,
            content: result.content,
            scope: result.scope,
            author: user,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
}
