use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, SimpleObject};
use std::sync::Arc;

use crate::error::Result;
use crate::graphql::loaders::UserLoader;
use crate::graphql::relay::{self, RelayConnection};
use crate::modules::post::graphql::{Post, PostError};
use crate::services::Services;

#[derive(SimpleObject)]
pub struct Feed {
    feed: Option<RelayConnection<Post>>,
    error: Option<PostError>,
}

impl Feed {
    pub async fn exec(
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Self> {
        let services = ctx.data::<Arc<Services>>().unwrap();

        match services.post.find_public_posts(first).await {
            Ok(posts) => {
                let user_loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
                let users = user_loader
                    .load_many(posts.iter().map(|p| p.user_id))
                    .await
                    .unwrap();
                let posts = posts.iter().map(|p| Post {
                    id: p.id,
                    content: p.content.clone(),
                    user: users.get(&p.user_id).unwrap().to_owned(),
                    scope: p.scope,
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                });
                let posts_connection =
                    relay::query(posts, relay::Params::new(after, before, first, last), 10).await?;
                Ok(Feed {
                    feed: Some(posts_connection),
                    error: None,
                })
            }
            Err(err) => {
                let post_error = PostError::try_from(err)?;

                Ok(Feed {
                    feed: None,
                    error: Some(post_error),
                })
            }
        }
    }
}
