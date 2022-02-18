use std::sync::Arc;

use crate::database::Database;
use crate::modules::user::{UserRepository, UserService};

pub struct Services {
    pub user: Arc<UserService>,
}

impl Services {
    pub fn new(database: Database) -> Self {
        let database = Arc::new(database);
        let user_repository = Arc::new(UserRepository::new(Arc::clone(&database)));
        let user_service = Arc::new(UserService::new(Arc::clone(&user_repository)));

        Self { user: user_service }
    }
}
