pub mod relay;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject};

use crate::modules::user::graphql::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(pub UserQuery);

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;
