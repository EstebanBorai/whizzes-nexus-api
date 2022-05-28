pub mod feed;
pub mod posts;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::feed::Feed;
use self::posts::Posts;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    #[graphql(name = "feed")]
    async fn feed(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Feed> {
        Feed::exec(ctx, after, before, first, last).await
    }

    #[graphql(name = "posts")]
    async fn posts(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Posts> {
        Posts::exec(ctx, after, before, first, last).await
    }
}
