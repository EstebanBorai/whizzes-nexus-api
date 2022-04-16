use async_graphql::{Context, SimpleObject};
use std::sync::Arc;

use crate::error::Result;
use crate::graphql::relay::{self, RelayConnection};
use crate::modules::post::graphql::PostError;
use crate::modules::post::Post;
use crate::routes::AuthToken;
use crate::services::Services;

#[derive(SimpleObject)]
pub struct Posts {
    posts: Option<RelayConnection<Post>>,
    error: Option<PostError>,
}

impl Posts {
    pub async fn exec(
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Self> {
        let auth = ctx.data::<AuthToken>().unwrap();
        let services = ctx.data::<Arc<Services>>().unwrap();
        let user = services.auth.whoami(auth.token().unwrap()).await?;
        let result = services.post.find_by_author(user).await;

        match result {
            Ok(posts) => {
                let posts_connection = relay::query(
                    posts.into_iter(),
                    relay::Params::new(after, before, first, last),
                    10,
                )
                .await?;
                Ok(Posts {
                    posts: Some(posts_connection),
                    error: None,
                })
            }
            Err(err) => {
                let post_error = PostError::try_from(err)?;

                Ok(Posts {
                    posts: None,
                    error: Some(post_error),
                })
            }
        }
    }
}
