use std::sync::Arc;

use crate::error::Result;
use crate::modules::post::graphql::post_create::PostCreateInput;
use crate::modules::user::User;

use super::{InsertPostTableRow, Post, PostRepository};

pub struct PostService {
    repository: Arc<PostRepository>,
}

impl PostService {
    pub fn new(repository: Arc<PostRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, user: User, payload: PostCreateInput) -> Result<Post> {
        let inserted = self
            .repository
            .insert(
                user,
                InsertPostTableRow {
                    content: payload.content,
                    scope: payload.scope.to_lowercase(),
                    user_id: None,
                },
            )
            .await?;

        Ok(inserted)
    }

    pub async fn find_by_author(&self, user: User) -> Result<Vec<Post>> {
        let posts = self.repository.find_by_author(&user.id).await?;
        let posts: Vec<Post> = posts
            .into_iter()
            .map(|post| Post {
                id: post.id,
                content: post.content,
                scope: post.scope,
                author: user.clone(),
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .collect();

        Ok(posts)
    }
}
