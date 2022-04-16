use async_graphql::{Context, SimpleObject};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::Result;
use crate::modules::post::graphql::PostError;
use crate::modules::post::Post;
use crate::routes::AuthToken;
use crate::services::Services;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct PostCreate {
    post: Option<Post>,
    error: Option<PostError>,
}

impl PostCreate {
    pub async fn exec(ctx: &Context<'_>, content: String) -> Result<PostCreate> {
        let auth = ctx.data::<AuthToken>().unwrap();
        let services = ctx.data::<Arc<Services>>().unwrap();
        let user = services.auth.whoami(auth.token().unwrap()).await?;
        let result = services.post.create(user, content.as_str()).await;

        match result {
            Ok(post) => Ok(PostCreate {
                post: Some(post),
                error: None,
            }),
            Err(err) => {
                let post_error = PostError::try_from(err)?;

                Ok(PostCreate {
                    post: None,
                    error: Some(post_error),
                })
            }
        }
    }
}
