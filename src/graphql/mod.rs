pub mod relay;

use async_graphql::{EmptySubscription, MergedObject};

use crate::modules::user::graphql::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(pub UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(pub UserMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
