pub mod post_create;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::post_create::PostCreate;

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    #[graphql(name = "postCreate")]
    async fn post_create(&self, ctx: &Context<'_>, content: String) -> Result<PostCreate> {
        PostCreate::exec(ctx, content).await
    }
}
