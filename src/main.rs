mod config;
mod database;
mod domain;
mod error;
mod routes;
mod schema;
mod services;

use dotenv::dotenv;
use std::env;

use self::config::Config;
use self::database::Database;
use self::routes::index;
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

    let users = services.user.find_all().await.unwrap();

    println!("{:?}", users);

    rocket::custom(&config.server_config).mount("/", rocket::routes![index])
}
