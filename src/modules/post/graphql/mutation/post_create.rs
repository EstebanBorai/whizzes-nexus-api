use async_graphql::{Context, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::Result;
use crate::modules::post::graphql::PostError;
use crate::modules::post::{Post, Scope};
use crate::routes::AuthToken;
use crate::services::Services;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct PostCreate {
    post: Option<Post>,
    error: Option<PostError>,
}

#[derive(Deserialize, Serialize, InputObject)]
#[graphql(input_name = "PostCreateInput")]
pub struct PostCreateInput {
    pub content: String,
    pub scope: Scope,
}

impl PostCreate {
    pub async fn exec(ctx: &Context<'_>, input: PostCreateInput) -> Result<PostCreate> {
        let auth = ctx.data_unchecked::<AuthToken>();
        let services = ctx.data::<Arc<Services>>().unwrap();
        let token = auth.token()?;
        let user = services.auth.whoami(token).await?;
        let result = services.post.create(user, input).await;

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
