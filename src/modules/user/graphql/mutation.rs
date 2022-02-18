use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::modules::user::{User, UserCreateDto};
use crate::services::Services;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(name = "userCreate")]
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateDto) -> Result<User> {
        let services = ctx.data::<Arc<Services>>().unwrap();
        let user = services.user.create(input).await?;

        Ok(user)
    }
}
