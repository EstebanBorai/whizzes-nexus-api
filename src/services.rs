use std::sync::Arc;

use crate::database::Database;
use crate::modules::user::UserService;

pub struct Services {
    pub user: Arc<UserService>,
}

impl Services {
    pub fn new(database: Database) -> Self {
        let database = Arc::new(database);
        let user = Arc::new(UserService::new(Arc::clone(&database)));

        Self { user }
    }
}
