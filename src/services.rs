use std::sync::Arc;

use crate::config::Config;
use crate::database::Database;
use crate::modules::auth::AuthService;
// use crate::modules::post::{PostRepository, PostService};
use crate::modules::user::{UserRepository, UserService};

pub struct Services {
    pub auth: Arc<AuthService>,
    // pub post: Arc<PostService>,
    pub user: Arc<UserService>,
}

impl Services {
    pub fn new(config: &Config, database: Database) -> Self {
        let database = Arc::new(database);
        let user_repository = Arc::new(UserRepository::new(Arc::clone(&database)));
        let user_service = Arc::new(UserService::new(Arc::clone(&user_repository)));
        // let post_repository = Arc::new(PostRepository::new(Arc::clone(&database)));
        // let post_service = Arc::new(PostService::new(post_repository));
        let auth_service = Arc::new(AuthService::new(config, Arc::clone(&user_service)));

        Self {
            auth: auth_service,
            // post: post_service,
            user: user_service,
        }
    }
}
