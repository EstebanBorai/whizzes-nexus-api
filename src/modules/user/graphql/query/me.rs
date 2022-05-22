use async_graphql::{Context, SimpleObject};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::Result;
use crate::modules::user::graphql::types::UserError;
use crate::modules::user::User;
use crate::routes::AuthToken;
use crate::services::Services;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Me {
    me: Option<User>,
    error: Option<UserError>,
}

impl Me {
    pub async fn exec(ctx: &Context<'_>) -> Result<Self> {
        let auth = ctx.data_unchecked::<AuthToken>();
        let services = ctx.data_unchecked::<Arc<Services>>();
        let token = auth.token()?;
        let result = services.auth.whoami(token).await;

        match result {
            Ok(user) => Ok(Me {
                me: Some(user),
                error: None,
            }),
            Err(err) => {
                let user_error = UserError::try_from(err)?;

                Ok(Me {
                    me: None,
                    error: Some(user_error),
                })
            }
        }
    }
}
