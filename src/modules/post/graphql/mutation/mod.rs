pub mod post_create;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::post_create::{PostCreate, PostCreateInput};

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    #[graphql(name = "postCreate")]
    async fn post_create(&self, ctx: &Context<'_>, input: PostCreateInput) -> Result<PostCreate> {
        PostCreate::exec(ctx, input).await
    }
}
