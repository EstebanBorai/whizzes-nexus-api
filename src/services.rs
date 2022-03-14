use std::sync::Arc;

use crate::config::Config;
use crate::database::Database;
use crate::modules::auth::AuthService;
use crate::modules::user::{UserRepository, UserService};

pub struct Services {
    pub auth: Arc<AuthService>,
    pub user: Arc<UserService>,
}

impl Services {
    pub fn new(config: &Config, database: Database) -> Self {
        let database = Arc::new(database);
        let user_repository = Arc::new(UserRepository::new(Arc::clone(&database)));
        let user_service = Arc::new(UserService::new(Arc::clone(&user_repository)));
        let auth_service = Arc::new(AuthService::new(config, Arc::clone(&user_service)));

        Self {
            auth: auth_service,
            user: user_service,
        }
    }
}
