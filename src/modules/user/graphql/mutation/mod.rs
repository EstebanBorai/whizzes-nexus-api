pub mod account_register;

use async_graphql::{Context, Object};

use crate::error::Result;

use self::account_register::{AccountRegister, AccountRegisterInput};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(name = "accountRegister")]
    async fn account_register(
        &self,
        ctx: &Context<'_>,
        input: AccountRegisterInput,
    ) -> Result<AccountRegister> {
        account_register::exec(ctx, input).await
    }
}
