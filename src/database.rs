use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};

use crate::config::Config;

pub struct Database {
    pub conn_pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(config: &Config) -> Self {
        let conn_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .expect("Failed to establish a Database Connection");

        Self { conn_pool }
    }
}
