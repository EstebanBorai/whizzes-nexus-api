pub mod relay;

use async_graphql::{EmptySubscription, MergedObject};

use crate::modules::auth::graphql::AuthMutation;
// use crate::modules::post::graphql::{PostMutation, PostQuery};
use crate::modules::user::graphql::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(/*pub PostQuery*/ pub UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(pub AuthMutation, /*pub PostMutation*/ pub UserMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
