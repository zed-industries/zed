use std::sync::Arc;

use async_trait::async_trait;
use rpc::proto;

use crate::Result;
use crate::db::{Channel, Database, UserId};
use crate::entities::User;

#[cfg(feature = "test-support")]
pub use self::fake_user_service::*;

#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>>;

    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>>;

    async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>>;

    // NOTE: This method is only tangentially related to users, but we're putting it on the `UserService` to avoid
    // introducing a separate service.
    //
    // We're also using the `proto::ChannelMember` representation in the return type, as we don't yet have a domain
    // representation of a channel member (and doesn't seem necessary to introduce one, at this point).
    async fn search_channel_members(
        &self,
        channel: &Channel,
        query: &str,
        limit: u32,
    ) -> Result<(Vec<proto::ChannelMember>, Vec<User>)>;

    #[cfg(feature = "test-support")]
    fn as_fake(&self) -> Arc<FakeUserService> {
        panic!("called as_fake on a real `UserService`");
    }
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

    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        let user = self.database.get_user_by_github_login(github_login).await?;

        Ok(user.map(User::from))
    }

    async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>> {
        let users = self.database.fuzzy_search_users(query, limit).await?;

        Ok(users.into_iter().map(User::from).collect())
    }

    async fn search_channel_members(
        &self,
        channel: &Channel,
        query: &str,
        limit: u32,
    ) -> Result<(Vec<proto::ChannelMember>, Vec<User>)> {
        let (members, users) = self
            .database
            .get_channel_participant_details(channel, query, limit as u64)
            .await?;

        Ok((
            members
                .into_iter()
                .map(proto::ChannelMember::from)
                .collect(),
            users.into_iter().map(User::from).collect(),
        ))
    }
}

#[cfg(feature = "test-support")]
mod fake_user_service {
    use std::sync::Weak;

    use collections::HashMap;
    use tokio::sync::Mutex;

    use super::*;

    #[derive(Debug)]
    pub struct NewUserParams {
        pub github_login: String,
        pub github_user_id: i32,
    }

    pub struct FakeUserService {
        this: Weak<Self>,
        state: Arc<Mutex<FakeUserServiceState>>,
        database: Arc<Database>,
    }

    struct FakeUserServiceState {
        next_user_id: UserId,
        users: HashMap<UserId, User>,
    }

    impl Default for FakeUserServiceState {
        fn default() -> Self {
            Self {
                next_user_id: UserId(1),
                users: HashMap::default(),
            }
        }
    }

    impl FakeUserService {
        pub fn new(database: Arc<Database>) -> Arc<Self> {
            Arc::new_cyclic(|this| Self {
                this: this.clone(),
                state: Arc::new(Mutex::default()),
                database,
            })
        }

        pub async fn create_user(
            &self,
            email_address: &str,
            name: Option<&str>,
            admin: bool,
            params: NewUserParams,
        ) -> UserId {
            let mut state = self.state.lock().await;

            let user_id = state.next_user_id;
            let _ = email_address;
            state.users.insert(
                user_id,
                User {
                    id: user_id,
                    github_login: params.github_login,
                    github_user_id: params.github_user_id,
                    name: name.map(|name| name.to_string()),
                    admin,
                    connected_once: false,
                },
            );

            state.next_user_id = UserId(state.next_user_id.0 + 1);

            user_id
        }

        pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
            let state = self.state.lock().await;

            let user = state.users.get(&id).cloned();

            Ok(user)
        }
    }

    #[async_trait]
    impl UserService for FakeUserService {
        async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
            let state = self.state.lock().await;

            let users = state
                .users
                .values()
                .filter(|user| ids.contains(&user.id))
                .cloned()
                .collect();

            Ok(users)
        }

        async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
            let state = self.state.lock().await;

            let user = state
                .users
                .values()
                .find(|user| user.github_login == github_login)
                .cloned();

            Ok(user)
        }

        async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>> {
            let _ = query;
            let _ = limit;
            unimplemented!("not currently exercised by any tests")
        }

        async fn search_channel_members(
            &self,
            channel: &Channel,
            query: &str,
            limit: u32,
        ) -> Result<(Vec<proto::ChannelMember>, Vec<User>)> {
            let state = self.state.lock().await;

            let users = state
                .users
                .values()
                .filter(|user| user.github_login.contains(query))
                .take(limit as usize)
                .cloned()
                .collect::<Vec<_>>();

            let members = self
                .database
                .get_channel_memberships_for_user_ids(
                    channel,
                    users.iter().map(|user| user.id).collect(),
                )
                .await?;

            Ok((
                members
                    .into_iter()
                    .map(proto::ChannelMember::from)
                    .collect(),
                users,
            ))
        }

        #[cfg(feature = "test-support")]
        fn as_fake(&self) -> Arc<FakeUserService> {
            self.this.upgrade().unwrap()
        }
    }
}
