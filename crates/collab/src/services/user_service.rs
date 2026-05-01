use std::sync::Arc;

use async_trait::async_trait;

use crate::Result;
use crate::db::{Database, UserId};
use crate::entities::User;

#[cfg(feature = "test-support")]
pub use self::fake_user_service::*;

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>>;
}

/// A [`UserService`] implementation backed by the database.
pub struct DatabaseUserService {
    database: Arc<Database>,
}

impl DatabaseUserService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl UserService for DatabaseUserService {
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        let users = self.database.get_users_by_ids(ids).await?;

        Ok(users.into_iter().map(User::from).collect())
    }
}

#[cfg(feature = "test-support")]
mod fake_user_service {
    use super::*;

    pub struct FakeUserService {}

    impl FakeUserService {
        pub fn new() -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl UserService for FakeUserService {
        async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
            let _ = ids;
            todo!()
        }
    }
}
