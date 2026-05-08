use std::sync::Arc;

use anyhow::{Context as _, anyhow};
use async_trait::async_trait;
use cloud_api_types::internal_api::{
    self, LookUpUserByGithubLoginBody, LookUpUserByGithubLoginResponse, LookUpUsersByLegacyIdBody,
    LookUpUsersByLegacyIdResponse,
};
use reqwest::RequestBuilder;
use rpc::proto;
use serde::de::DeserializeOwned;

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

/// A [`UserService`] implementation for transitioning from reading from the database to reading from Cloud.
pub struct TransitionalUserService {
    cloud_user_service: CloudUserService,
    database_user_service: DatabaseUserService,
}

impl TransitionalUserService {
    pub fn new(
        cloud_user_service: CloudUserService,
        database_user_service: DatabaseUserService,
    ) -> Self {
        Self {
            cloud_user_service,
            database_user_service,
        }
    }
}

#[async_trait]
impl UserService for TransitionalUserService {
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        self.cloud_user_service.get_users_by_ids(ids).await
    }

    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        self.cloud_user_service
            .get_user_by_github_login(github_login)
            .await
    }

    async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>> {
        self.database_user_service
            .fuzzy_search_users(query, limit)
            .await
    }

    async fn search_channel_members(
        &self,
        channel: &Channel,
        query: &str,
        limit: u32,
    ) -> Result<(Vec<proto::ChannelMember>, Vec<User>)> {
        self.database_user_service
            .search_channel_members(channel, query, limit)
            .await
    }
}

/// A [`UserService`] implementation backed by Cloud.
pub struct CloudUserService {
    http_client: reqwest::Client,
    zed_cloud_url: String,
    internal_api_key: String,
}

impl CloudUserService {
    pub fn new(
        http_client: reqwest::Client,
        zed_cloud_url: String,
        internal_api_key: String,
    ) -> Self {
        Self {
            http_client,
            zed_cloud_url,
            internal_api_key,
        }
    }

    async fn send_request<T: DeserializeOwned + 'static>(
        &self,
        request: RequestBuilder,
    ) -> Result<T> {
        let request = request
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", &self.internal_api_key),
            )
            .build()
            .context("failed to build request")?;

        let response = self
            .http_client
            .execute(request)
            .await
            .context("failed to send request to Cloud")?;

        let status = response.status();
        match response.error_for_status() {
            Ok(response) => {
                let response_body: T = response
                    .json()
                    .await
                    .context("failed to parse response body")?;

                Ok(response_body)
            }
            Err(_err) => Err(anyhow!("request to Cloud failed with status {status}",))?,
        }
    }
}

#[async_trait]
impl UserService for CloudUserService {
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        let response_body: LookUpUsersByLegacyIdResponse = self
            .send_request(
                self.http_client
                    .post(format!(
                        "{}/internal/users/look_up_by_legacy_id",
                        &self.zed_cloud_url
                    ))
                    .json(&LookUpUsersByLegacyIdBody {
                        legacy_user_ids: ids.into_iter().map(|id| id.0).collect(),
                    }),
            )
            .await?;

        Ok(response_body.users.into_iter().map(User::from).collect())
    }

    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        let response_body: LookUpUserByGithubLoginResponse = self
            .send_request(
                self.http_client
                    .post(format!(
                        "{}/internal/users/look_up_by_github_login",
                        &self.zed_cloud_url
                    ))
                    .json(&LookUpUserByGithubLoginBody {
                        github_login: github_login.to_string(),
                    }),
            )
            .await?;

        Ok(response_body.user.map(User::from))
    }

    async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>> {
        let _ = query;
        let _ = limit;

        unimplemented!("not yet implemented in Cloud")
    }

    async fn search_channel_members(
        &self,
        channel: &Channel,
        query: &str,
        limit: u32,
    ) -> Result<(Vec<proto::ChannelMember>, Vec<User>)> {
        let _ = channel;
        let _ = query;
        let _ = limit;

        unimplemented!("not yet implemented in Cloud")
    }
}

impl From<internal_api::User> for User {
    fn from(user: internal_api::User) -> Self {
        Self {
            id: UserId(user.legacy_user_id),
            github_login: user.github_login,
            github_user_id: user.github_user_id,
            name: user.name,
            admin: user.admin,
            connected_once: user.connected_once,
        }
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
