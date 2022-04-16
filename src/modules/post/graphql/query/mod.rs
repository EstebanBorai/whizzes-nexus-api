pub mod posts;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::posts::Posts;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
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
