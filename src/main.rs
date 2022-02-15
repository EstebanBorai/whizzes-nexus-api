mod config;
mod routes;

use std::env;

use self::config::Config;
use self::routes::index;

#[rocket::launch]
async fn rocket() -> _ {
    if cfg!(not(debug_assertions)) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let config = Config::new();

    rocket::custom(&config.server_config).mount("/", rocket::routes![index])
}
