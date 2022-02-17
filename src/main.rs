mod config;
mod routes;

use dotenv::dotenv;
use std::env;

use self::config::Config;
use self::routes::index;

#[rocket::launch]
async fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "1");

    if cfg!(debug_assertions) {
        dotenv().expect("No \".env\" file found. Copy the current \".env.sample\" file into a \".env\" file and run the server again.");
    }

    let config = Config::new();

    rocket::custom(&config.server_config).mount("/", rocket::routes![index])
}
