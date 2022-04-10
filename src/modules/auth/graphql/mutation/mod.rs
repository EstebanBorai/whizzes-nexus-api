pub mod token_create;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::token_create::TokenCreate;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    #[graphql(name = "tokenCreate")]
    pub async fn token_create(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<TokenCreate> {
        TokenCreate::exec(ctx, username, password).await
    }
}
