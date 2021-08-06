use serde::Serialize;
use sqlx::{FromRow, Result};

pub use async_sqlx_session::PostgresSessionStore as SessionStore;
pub use sqlx::postgres::PgPoolOptions as DbOptions;

pub struct Db(pub sqlx::PgPool);

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    id: i32,
    pub github_login: String,
    pub admin: bool,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Signup {
    id: i32,
    pub github_login: String,
    pub email_address: String,
    pub about: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Channel {
    id: i32,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct ChannelMessage {
    id: i32,
    sender_id: i32,
    body: String,
    sent_at: i64,
}

#[derive(Clone, Copy)]
pub struct UserId(pub i32);

#[derive(Clone, Copy)]
pub struct OrgId(pub i32);

#[derive(Clone, Copy)]
pub struct ChannelId(pub i32);

#[derive(Clone, Copy)]
pub struct SignupId(pub i32);

#[derive(Clone, Copy)]
pub struct MessageId(pub i32);

impl Db {
    // signups

    pub async fn create_signup(
        &self,
        github_login: &str,
        email_address: &str,
        about: &str,
    ) -> Result<SignupId> {
        let query = "
            INSERT INTO signups (github_login, email_address, about)
            VALUES ($1, $2, $3)
            RETURNING id
        ";
        sqlx::query_scalar(query)
            .bind(github_login)
            .bind(email_address)
            .bind(about)
            .fetch_one(&self.0)
            .await
            .map(SignupId)
    }

    pub async fn get_all_signups(&self) -> Result<Vec<Signup>> {
        let query = "SELECT * FROM users ORDER BY github_login ASC";
        sqlx::query_as(query).fetch_all(&self.0).await
    }

    pub async fn delete_signup(&self, id: SignupId) -> Result<()> {
        let query = "DELETE FROM signups WHERE id = $1";
        sqlx::query(query)
            .bind(id.0)
            .execute(&self.0)
            .await
            .map(drop)
    }

    // users

    pub async fn create_user(&self, github_login: &str, admin: bool) -> Result<UserId> {
        let query = "
            INSERT INTO users (github_login, admin)
            VALUES ($1, $2)
            RETURNING id
        ";
        sqlx::query_scalar(query)
            .bind(github_login)
            .bind(admin)
            .fetch_one(&self.0)
            .await
            .map(UserId)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let query = "SELECT * FROM users ORDER BY github_login ASC";
        sqlx::query_as(query).fetch_all(&self.0).await
    }

    pub async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE github_login = $1 LIMIT 1";
        sqlx::query_as(query)
            .bind(github_login)
            .fetch_optional(&self.0)
            .await
    }

    pub async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        let query = "UPDATE users SET admin = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(is_admin)
            .bind(id.0)
            .execute(&self.0)
            .await
            .map(drop)
    }

    pub async fn delete_user(&self, id: UserId) -> Result<()> {
        let query = "DELETE FROM users WHERE id = $1;";
        sqlx::query(query)
            .bind(id.0)
            .execute(&self.0)
            .await
            .map(drop)
    }

    // access tokens

    pub async fn create_access_token_hash(
        &self,
        user_id: UserId,
        access_token_hash: String,
    ) -> Result<()> {
        let query = "
            INSERT INTO access_tokens (user_id, hash)
            VALUES ($1, $2)
        ";
        sqlx::query(query)
            .bind(user_id.0 as i32)
            .bind(access_token_hash)
            .execute(&self.0)
            .await
            .map(drop)
    }

    pub async fn get_access_token_hashes(&self, user_id: UserId) -> Result<Vec<String>> {
        let query = "SELECT hash FROM access_tokens WHERE user_id = $1";
        sqlx::query_scalar::<_, String>(query)
            .bind(user_id.0 as i32)
            .fetch_all(&self.0)
            .await
    }

    // orgs

    #[cfg(test)]
    pub async fn create_org(&self, name: &str, slug: &str) -> Result<OrgId> {
        let query = "
            INSERT INTO orgs (name, slug)
            VALUES ($1, $2)
            RETURNING id
        ";
        sqlx::query_scalar(query)
            .bind(name)
            .bind(slug)
            .fetch_one(&self.0)
            .await
            .map(OrgId)
    }

    #[cfg(test)]
    pub async fn add_org_member(&self, org_id: OrgId, user_id: UserId) -> Result<()> {
        let query = "
            INSERT INTO org_memberships (org_id, user_id)
            VALUES ($1, $2)
        ";
        sqlx::query(query)
            .bind(org_id.0)
            .bind(user_id.0)
            .execute(&self.0)
            .await
            .map(drop)
    }

    // channels

    #[cfg(test)]
    pub async fn create_org_channel(&self, org_id: OrgId, name: &str) -> Result<ChannelId> {
        let query = "
            INSERT INTO channels (owner_id, owner_is_user, name)
            VALUES ($1, false, $2)
            RETURNING id
        ";
        sqlx::query_scalar(query)
            .bind(org_id.0)
            .bind(name)
            .fetch_one(&self.0)
            .await
            .map(ChannelId)
    }

    pub async fn get_channels_for_user(&self, user_id: UserId) -> Result<Vec<Channel>> {
        let query = "
            SELECT
                channels.id, channels.name
            FROM
                channel_memberships, channels
            WHERE
                channel_memberships.user_id = $1 AND
                channel_memberships.channel_id = channels.id
        ";
        sqlx::query_as(query)
            .bind(user_id.0)
            .fetch_all(&self.0)
            .await
    }

    pub async fn can_user_access_channel(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> Result<bool> {
        let query = "
            SELECT id
            FROM channel_memberships
            WHERE user_id = $1 AND channel_id = $2
            LIMIT 1
        ";
        sqlx::query_scalar::<_, i32>(query)
            .bind(user_id.0)
            .bind(channel_id.0)
            .fetch_optional(&self.0)
            .await
            .map(|e| e.is_some())
    }

    #[cfg(test)]
    pub async fn add_channel_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        is_admin: bool,
    ) -> Result<()> {
        let query = "
            INSERT INTO channel_memberships (channel_id, user_id, admin)
            VALUES ($1, $2, $3)
        ";
        sqlx::query(query)
            .bind(channel_id.0)
            .bind(user_id.0)
            .bind(is_admin)
            .execute(&self.0)
            .await
            .map(drop)
    }

    // messages

    pub async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        sender_id: UserId,
        body: &str,
    ) -> Result<MessageId> {
        let query = "
            INSERT INTO channel_messages (channel_id, sender_id, body, sent_at)
            VALUES ($1, $2, $3, NOW()::timestamp)
            RETURNING id
        ";
        sqlx::query_scalar(query)
            .bind(channel_id.0)
            .bind(sender_id.0)
            .bind(body)
            .fetch_one(&self.0)
            .await
            .map(MessageId)
    }

    pub async fn get_recent_channel_messages(
        &self,
        channel_id: ChannelId,
        count: usize,
    ) -> Result<Vec<ChannelMessage>> {
        let query = "
            SELECT id, sender_id, body, sent_at
            FROM channel_messages
            WHERE channel_id = $1
            LIMIT $2
        ";
        sqlx::query_as(query)
            .bind(channel_id.0)
            .bind(count as i64)
            .fetch_all(&self.0)
            .await
    }
}

impl std::ops::Deref for Db {
    type Target = sqlx::PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Channel {
    pub fn id(&self) -> ChannelId {
        ChannelId(self.id)
    }
}

impl User {
    pub fn id(&self) -> UserId {
        UserId(self.id)
    }
}
