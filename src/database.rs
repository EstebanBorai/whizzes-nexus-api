use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::config::Config;

pub type PgConnPool = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub conn_pool: PgConnPool,
}

impl Database {
    pub fn new(config: &Config) -> Self {
        let conn_pool = Self::make_connection_pool(config);

        Self { conn_pool }
    }

    fn make_connection_pool(config: &Config) -> PgConnPool {
        let manager = ConnectionManager::<PgConnection>::new::<&String>(&config.database_url);

        Pool::new(manager).expect("Failed to initialize database connection pool")
    }
}
