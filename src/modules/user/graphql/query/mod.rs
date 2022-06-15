pub mod me;
pub mod users;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::me::Me;
use self::users::{Users, UsersFilter};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(name = "me")]
    async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        Me::exec(ctx).await
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        filter: Option<UsersFilter>,
    ) -> Result<Users> {
        Users::exec(ctx, after, before, first, last, filter).await
    }
}
