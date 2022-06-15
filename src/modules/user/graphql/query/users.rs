use async_graphql::{Context, InputObject, SimpleObject};
use std::sync::Arc;

use crate::error::Result;
use crate::graphql::relay::{self, RelayConnection};
use crate::modules::user::graphql::types::UserError;
use crate::modules::user::User;
use crate::routes::AuthToken;
use crate::services::Services;

#[derive(SimpleObject)]
pub struct Users {
    users: Option<RelayConnection<User>>,
    error: Option<UserError>,
}

#[derive(InputObject)]
pub struct UsersFilter {
    pub username: Option<String>,
}

impl Users {
    pub async fn exec(
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        filter: Option<UsersFilter>,
    ) -> Result<Users> {
        let auth = ctx.data_unchecked::<AuthToken>();
        let services = ctx.data_unchecked::<Arc<Services>>();
        let token = auth.token()?;

        services.auth.whoami(token).await?;

        if let Some(filter) = filter {
            if let Some(username) = filter.username {
                return match services.user.find_by_username(username.as_str()).await {
                    Ok(find_by_username) => {
                        // The result is wrapped into a `Vec<User>` to keep
                        // responses consistent.
                        let res = if let Some(user) = find_by_username {
                            vec![user]
                        } else {
                            Vec::default()
                        };

                        let users_connection = relay::query(
                            res.into_iter(),
                            relay::Params::new(after, before, first, last),
                            10,
                        )
                        .await?;
                        Ok(Users {
                            users: Some(users_connection),
                            error: None,
                        })
                    }
                    Err(err) => {
                        let user_error = UserError::try_from(err)?;

                        Ok(Users {
                            users: None,
                            error: Some(user_error),
                        })
                    }
                };
            }
        }

        match services.user.find_all().await {
            Ok(users) => {
                let users_connection = relay::query(
                    users.into_iter(),
                    relay::Params::new(after, before, first, last),
                    10,
                )
                .await?;
                Ok(Users {
                    users: Some(users_connection),
                    error: None,
                })
            }
            Err(err) => {
                let user_error = UserError::try_from(err)?;

                Ok(Users {
                    users: None,
                    error: Some(user_error),
                })
            }
        }
    }
}
