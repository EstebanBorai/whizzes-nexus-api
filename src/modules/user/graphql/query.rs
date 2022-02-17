use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::graphql::relay;
use crate::modules::user::User;
use crate::services::Services;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> relay::ConnectionResult<User> {
        let services = ctx.data::<Arc<Services>>().unwrap();
        let users = services.user.find_all().await?;

        relay::query(
            users.into_iter(),
            relay::Params::new(after, before, first, last),
            10,
        )
        .await
    }
}
