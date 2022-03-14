use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::modules::auth::Token;
use crate::services::Services;
use crate::Result;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    #[graphql(name = "tokenCreate")]
    async fn token_create(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<Token> {
        let services = ctx.data::<Arc<Services>>().unwrap();

        services.auth.create_token(username, password).await
    }
}
