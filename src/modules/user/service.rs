use diesel::prelude::*;
use std::sync::Arc;

use crate::database::Database;
use crate::error::Result;
use crate::schema::users;

use super::{User, UserTableRow};

pub struct UserService {
    database: Arc<Database>,
}

impl UserService {
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            database: Arc::clone(&database),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        let conn = self.database.conn_pool.get()?;
        let users = users::table.load::<UserTableRow>(&conn)?;
        let users = users.into_iter().map(User::from).collect::<Vec<User>>();

        Ok(users)
    }
}
