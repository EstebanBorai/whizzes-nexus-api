mod catchers;
mod config;
mod database;
mod error;
mod fairings;
mod graphql;
mod modules;
mod responders;
mod routes;
mod services;

use async_graphql::dataloader::DataLoader;
use async_graphql::EmptySubscription;
use dotenv::dotenv;
use rocket::routes;
use std::env;
use std::sync::Arc;

use self::config::Config;
use self::database::Database;
use self::graphql::loaders::UserLoader;
use self::graphql::{Mutation, Query, Schema};
use self::routes::{cors_preflight, graphql_playground, graphql_request};
use self::services::Services;

#[rocket::launch]
async fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "1");

    if cfg!(debug_assertions) {
        dotenv().expect("No \".env\" file found. Copy the current \".env.sample\" file into a \".env\" file and run the server again.");
    }

    let config = Config::new();
    let database = Database::new(&config).await;
    let database = Arc::new(database);
    let services = Services::new(&config, Arc::clone(&database));
    let services = Arc::new(services);
    let graphql_schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::clone(&services))
        .data(DataLoader::new(
            UserLoader::new(Arc::clone(&database)),
            rocket::tokio::spawn,
        ))
        .finish();

    rocket::custom(&config.server_config)
        .attach(fairings::cors::Cors)
        .manage(Arc::clone(&services))
        .manage(graphql_schema)
        .mount(
            "/",
            routes![cors_preflight, graphql_playground, graphql_request],
        )
        .register("/", rocket::catchers![catchers::not_found])
}
