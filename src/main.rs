mod config;
mod database;
mod error;
mod graphql;
mod modules;
mod routes;
mod schema;
mod services;

use async_graphql::EmptySubscription;
use dotenv::dotenv;
use rocket::routes;
use std::env;
use std::sync::Arc;

use self::config::Config;
use self::database::Database;
use self::graphql::{Mutation, Query, Schema};
use self::routes::{graphql_playground, graphql_query, graphql_request};
use self::services::Services;

#[macro_use]
extern crate diesel;

#[rocket::launch]
async fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "1");

    if cfg!(debug_assertions) {
        dotenv().expect("No \".env\" file found. Copy the current \".env.sample\" file into a \".env\" file and run the server again.");
    }

    let config = Config::new();
    let database = Database::new(&config);
    let services = Services::new(database);
    let services = Arc::new(services);
    let graphql_schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::clone(&services))
        .finish();

    rocket::custom(&config.server_config)
        .manage(Arc::clone(&services))
        .manage(graphql_schema)
        .mount(
            "/",
            routes![graphql_playground, graphql_query, graphql_request],
        )
}
