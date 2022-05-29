use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::{self, Uuid};

use crate::database::Database;
use crate::error::Result;
use crate::modules::user::User;

use super::entity::Post;
use super::Scope;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct PostsTableRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub scope: Scope,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PostsTableRow> for Post {
    fn from(dto: PostsTableRow) -> Self {
        Self {
            id: dto.id,
            user_id: dto.user_id,
            content: dto.content,
            scope: dto.scope,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertPostTableRow {
    pub content: String,
    pub scope: Scope,
    pub user_id: Option<Uuid>,
}

pub struct PostRepository {
    database: Arc<Database>,
}

impl PostRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn find_by_author(&self, user_id: &Uuid) -> Result<Vec<Post>> {
        let result: Vec<PostsTableRow> = sqlx::query_as("SELECT * FROM posts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.database.conn_pool)
            .await?;
        let posts = result
            .into_iter()
            .map(|row| Post {
                id: row.id,
                user_id: row.user_id,
                content: row.content,
                scope: row.scope,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect::<Vec<Post>>();

        Ok(posts)
    }

    pub async fn insert(&self, user: User, dto: InsertPostTableRow) -> Result<Post> {
        let result: PostsTableRow = sqlx::query_as(
            r#"
            INSERT INTO posts (
                content,
                scope,
                user_id
            ) VALUES (
                $1,
                $2::scope,
                $3
            ) RETURNING *"#,
        )
        .bind(dto.content)
        .bind(dto.scope)
        .bind(user.id)
        .fetch_one(&self.database.conn_pool)
        .await?;

        Ok(Post {
            id: result.id,
            content: result.content,
            scope: result.scope,
            user_id: user.id,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    pub async fn find_public_posts(&self) -> Result<Vec<Post>> {
        let result: Vec<PostsTableRow> =
            sqlx::query_as("SELECT * FROM posts WHERE scope = 'public' LIMIT 20")
                .fetch_all(&self.database.conn_pool)
                .await?;
        let posts = result
            .into_iter()
            .map(|row| Post {
                id: row.id,
                user_id: row.user_id,
                content: row.content,
                scope: row.scope,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect::<Vec<Post>>();

        Ok(posts)
    }
}
