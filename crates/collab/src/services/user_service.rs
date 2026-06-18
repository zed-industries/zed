use anyhow::{Context as _, anyhow};
use async_trait::async_trait;
use cloud_api_types::internal_api::{
    self, FuzzySearchChannelMembersByGithubLoginBody,
    FuzzySearchChannelMembersByGithubLoginResponse, FuzzySearchUsersBody, FuzzySearchUsersResponse,
    LookUpUserByGithubLoginBody, LookUpUserByGithubLoginResponse, LookUpUsersByLegacyIdBody,
    LookUpUsersByLegacyIdResponse,
};
use reqwest::RequestBuilder;
use rpc::proto;
use serde::de::DeserializeOwned;

use crate::Result;
use crate::db::{Channel, UserId};
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
    fn as_fake(&self) -> std::sync::Arc<FakeUserService> {
        panic!("called as_fake on a real `UserService`");
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
        let response_body: FuzzySearchUsersResponse = self
            .send_request(
                self.http_client
                    .post(format!(
                        "{}/internal/users/fuzzy_search",
                        &self.zed_cloud_url
                    ))
                    .json(&FuzzySearchUsersBody {
                        query: query.to_string(),
                        limit,
                    }),
            )
            .await?;

        Ok(response_body.users.into_iter().map(User::from).collect())
    }

    async fn search_channel_members(
        &self,
        channel: &Channel,
        query: &str,
        limit: u32,
    ) -> Result<(Vec<proto::ChannelMember>, Vec<User>)> {
        let response_body: FuzzySearchChannelMembersByGithubLoginResponse = self
            .send_request(
                self.http_client
                    .post(format!(
                        "{}/internal/channel_members/fuzzy_search_by_github_login",
                        &self.zed_cloud_url
                    ))
                    .json(&FuzzySearchChannelMembersByGithubLoginBody {
                        channel_id: channel.root_id().0,
                        query: query.to_string(),
                        limit,
                    }),
            )
            .await?;

        let members = response_body
            .channel_members
            .into_iter()
            .map(channel_member_to_proto)
            .collect::<Vec<_>>();
        let users = response_body
            .users
            .into_iter()
            .map(User::from)
            .collect::<Vec<_>>();

        Ok((members, users))
    }
}

fn channel_member_to_proto(member: internal_api::ChannelMember) -> proto::ChannelMember {
    let kind = match member.kind {
        internal_api::ChannelMemberKind::Member => proto::channel_member::Kind::Member,
        internal_api::ChannelMemberKind::Invitee => proto::channel_member::Kind::Invitee,
    };
    let role = match member.role {
        internal_api::ChannelMemberRole::Admin => proto::ChannelRole::Admin,
        internal_api::ChannelMemberRole::Member => proto::ChannelRole::Member,
        internal_api::ChannelMemberRole::Talker => proto::ChannelRole::Talker,
        internal_api::ChannelMemberRole::Guest => proto::ChannelRole::Guest,
        internal_api::ChannelMemberRole::Banned => proto::ChannelRole::Banned,
    };

    proto::ChannelMember {
        user_id: UserId(member.legacy_user_id).to_proto(),
        kind: kind.into(),
        role: role.into(),
    }
}

impl From<internal_api::User> for User {
    fn from(user: internal_api::User) -> Self {
        Self {
            id: UserId(user.legacy_user_id),
            avatar_url: user.avatar_url,
            github_login: user.github_login,
            name: user.name,
            admin: user.admin,
            connected_once: user.connected_once,
        }
    }
}

#[cfg(feature = "test-support")]
mod fake_user_service {
    use std::sync::{Arc, Weak};

    use collections::HashMap;
    use tokio::sync::Mutex;

    use crate::db::Database;

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
                    avatar_url: format!("https://github.com/{}.png?size=128", params.github_login),
                    github_login: params.github_login,
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
