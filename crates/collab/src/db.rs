use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use serde::Serialize;
pub use sqlx::postgres::PgPoolOptions as DbOptions;
use sqlx::{types::Uuid, FromRow};
use time::OffsetDateTime;

#[async_trait]
pub trait Db: Send + Sync {
    async fn create_user(&self, github_login: &str, admin: bool) -> Result<UserId>;
    async fn get_all_users(&self) -> Result<Vec<User>>;
    async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>>;
    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>>;
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>>;
    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>>;
    async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()>;
    async fn destroy_user(&self, id: UserId) -> Result<()>;

    async fn get_contacts(&self, id: UserId) -> Result<Contacts>;
    async fn send_contact_request(&self, requester_id: UserId, responder_id: UserId) -> Result<()>;
    async fn dismiss_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
    ) -> Result<()>;
    async fn respond_to_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> Result<()>;

    async fn create_access_token_hash(
        &self,
        user_id: UserId,
        access_token_hash: &str,
        max_access_token_count: usize,
    ) -> Result<()>;
    async fn get_access_token_hashes(&self, user_id: UserId) -> Result<Vec<String>>;
    #[cfg(any(test, feature = "seed-support"))]

    async fn find_org_by_slug(&self, slug: &str) -> Result<Option<Org>>;
    #[cfg(any(test, feature = "seed-support"))]
    async fn create_org(&self, name: &str, slug: &str) -> Result<OrgId>;
    #[cfg(any(test, feature = "seed-support"))]
    async fn add_org_member(&self, org_id: OrgId, user_id: UserId, is_admin: bool) -> Result<()>;
    #[cfg(any(test, feature = "seed-support"))]
    async fn create_org_channel(&self, org_id: OrgId, name: &str) -> Result<ChannelId>;
    #[cfg(any(test, feature = "seed-support"))]

    async fn get_org_channels(&self, org_id: OrgId) -> Result<Vec<Channel>>;
    async fn get_accessible_channels(&self, user_id: UserId) -> Result<Vec<Channel>>;
    async fn can_user_access_channel(&self, user_id: UserId, channel_id: ChannelId)
        -> Result<bool>;
    #[cfg(any(test, feature = "seed-support"))]
    async fn add_channel_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        is_admin: bool,
    ) -> Result<()>;
    async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        sender_id: UserId,
        body: &str,
        timestamp: OffsetDateTime,
        nonce: u128,
    ) -> Result<MessageId>;
    async fn get_channel_messages(
        &self,
        channel_id: ChannelId,
        count: usize,
        before_id: Option<MessageId>,
    ) -> Result<Vec<ChannelMessage>>;
    #[cfg(test)]
    async fn teardown(&self, url: &str);
}

pub struct PostgresDb {
    pool: sqlx::PgPool,
}

impl PostgresDb {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self> {
        let pool = DbOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await
            .context("failed to connect to postgres database")?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Db for PostgresDb {
    // users

    async fn create_user(&self, github_login: &str, admin: bool) -> Result<UserId> {
        let query = "
            INSERT INTO users (github_login, admin)
            VALUES ($1, $2)
            ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
            RETURNING id
        ";
        Ok(sqlx::query_scalar(query)
            .bind(github_login)
            .bind(admin)
            .fetch_one(&self.pool)
            .await
            .map(UserId)?)
    }

    async fn get_all_users(&self) -> Result<Vec<User>> {
        let query = "SELECT * FROM users ORDER BY github_login ASC";
        Ok(sqlx::query_as(query).fetch_all(&self.pool).await?)
    }

    async fn fuzzy_search_users(&self, name_query: &str, limit: u32) -> Result<Vec<User>> {
        let like_string = fuzzy_like_string(name_query);
        let query = "
            SELECT users.*
            FROM users
            WHERE github_login like $1
            ORDER BY github_login <-> $2
            LIMIT $3
        ";
        Ok(sqlx::query_as(query)
            .bind(like_string)
            .bind(name_query)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?)
    }

    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
        let users = self.get_users_by_ids(vec![id]).await?;
        Ok(users.into_iter().next())
    }

    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        let ids = ids.into_iter().map(|id| id.0).collect::<Vec<_>>();
        let query = "
            SELECT users.*
            FROM users
            WHERE users.id = ANY ($1)
        ";
        Ok(sqlx::query_as(query)
            .bind(&ids)
            .fetch_all(&self.pool)
            .await?)
    }

    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE github_login = $1 LIMIT 1";
        Ok(sqlx::query_as(query)
            .bind(github_login)
            .fetch_optional(&self.pool)
            .await?)
    }

    async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        let query = "UPDATE users SET admin = $1 WHERE id = $2";
        Ok(sqlx::query(query)
            .bind(is_admin)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .map(drop)?)
    }

    async fn destroy_user(&self, id: UserId) -> Result<()> {
        let query = "DELETE FROM access_tokens WHERE user_id = $1;";
        sqlx::query(query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .map(drop)?;
        let query = "DELETE FROM users WHERE id = $1;";
        Ok(sqlx::query(query)
            .bind(id.0)
            .execute(&self.pool)
            .await
            .map(drop)?)
    }

    // contacts

    async fn get_contacts(&self, user_id: UserId) -> Result<Contacts> {
        let query = "
            SELECT user_id_a, user_id_b, a_to_b, accepted, should_notify
            FROM contacts
            WHERE user_id_a = $1 OR user_id_b = $1;
        ";

        let mut rows = sqlx::query_as::<_, (UserId, UserId, bool, bool, bool)>(query)
            .bind(user_id)
            .fetch(&self.pool);

        let mut current = Vec::new();
        let mut outgoing_requests = Vec::new();
        let mut incoming_requests = Vec::new();
        while let Some(row) = rows.next().await {
            let (user_id_a, user_id_b, a_to_b, accepted, should_notify) = row?;

            if user_id_a == user_id {
                if accepted {
                    current.push(user_id_b);
                } else if a_to_b {
                    outgoing_requests.push(user_id_b);
                } else {
                    incoming_requests.push(IncomingContactRequest {
                        requesting_user_id: user_id_b,
                        should_notify,
                    });
                }
            } else {
                if accepted {
                    current.push(user_id_a);
                } else if a_to_b {
                    incoming_requests.push(IncomingContactRequest {
                        requesting_user_id: user_id_a,
                        should_notify,
                    });
                } else {
                    outgoing_requests.push(user_id_a);
                }
            }
        }

        Ok(Contacts {
            current,
            outgoing_requests,
            incoming_requests,
        })
    }

    async fn send_contact_request(&self, sender_id: UserId, receiver_id: UserId) -> Result<()> {
        let (id_a, id_b, a_to_b) = if sender_id < receiver_id {
            (sender_id, receiver_id, true)
        } else {
            (receiver_id, sender_id, false)
        };
        let query = "
            INSERT into contacts (user_id_a, user_id_b, a_to_b, accepted, should_notify)
            VALUES ($1, $2, $3, 'f', 't')
            ON CONFLICT (user_id_a, user_id_b) DO UPDATE
            SET
                accepted = 't'
            WHERE
                NOT contacts.accepted AND
                ((contacts.a_to_b = excluded.a_to_b AND contacts.user_id_a = excluded.user_id_b) OR
                (contacts.a_to_b != excluded.a_to_b AND contacts.user_id_a = excluded.user_id_a));
        ";
        let result = sqlx::query(query)
            .bind(id_a.0)
            .bind(id_b.0)
            .bind(a_to_b)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 1 {
            Ok(())
        } else {
            Err(anyhow!("contact already requested"))
        }
    }

    async fn respond_to_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> Result<()> {
        let (id_a, id_b, a_to_b) = if responder_id < requester_id {
            (responder_id, requester_id, false)
        } else {
            (requester_id, responder_id, true)
        };
        let result = if accept {
            let query = "
                UPDATE contacts
                SET accepted = 't', should_notify = 'f'
                WHERE user_id_a = $1 AND user_id_b = $2 AND a_to_b = $3;
            ";
            sqlx::query(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .bind(a_to_b)
                .execute(&self.pool)
                .await?
        } else {
            let query = "
                DELETE FROM contacts
                WHERE user_id_a = $1 AND user_id_b = $2 AND a_to_b = $3 AND NOT accepted;
            ";
            sqlx::query(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .bind(a_to_b)
                .execute(&self.pool)
                .await?
        };
        if result.rows_affected() == 1 {
            Ok(())
        } else {
            Err(anyhow!("no such contact request"))
        }
    }

    async fn dismiss_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
    ) -> Result<()> {
        let (id_a, id_b, a_to_b) = if responder_id < requester_id {
            (responder_id, requester_id, false)
        } else {
            (requester_id, responder_id, true)
        };

        let query = "
            UPDATE contacts
            SET should_notify = 'f'
            WHERE user_id_a = $1 AND user_id_b = $2 AND a_to_b = $3;
        ";

        let result = sqlx::query(query)
            .bind(id_a.0)
            .bind(id_b.0)
            .bind(a_to_b)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(anyhow!("no such contact request"))?;
        }

        Ok(())
    }

    // access tokens

    async fn create_access_token_hash(
        &self,
        user_id: UserId,
        access_token_hash: &str,
        max_access_token_count: usize,
    ) -> Result<()> {
        let insert_query = "
            INSERT INTO access_tokens (user_id, hash)
            VALUES ($1, $2);
        ";
        let cleanup_query = "
            DELETE FROM access_tokens
            WHERE id IN (
                SELECT id from access_tokens
                WHERE user_id = $1
                ORDER BY id DESC
                OFFSET $3
            )
        ";

        let mut tx = self.pool.begin().await?;
        sqlx::query(insert_query)
            .bind(user_id.0)
            .bind(access_token_hash)
            .execute(&mut tx)
            .await?;
        sqlx::query(cleanup_query)
            .bind(user_id.0)
            .bind(access_token_hash)
            .bind(max_access_token_count as u32)
            .execute(&mut tx)
            .await?;
        Ok(tx.commit().await?)
    }

    async fn get_access_token_hashes(&self, user_id: UserId) -> Result<Vec<String>> {
        let query = "
            SELECT hash
            FROM access_tokens
            WHERE user_id = $1
            ORDER BY id DESC
        ";
        Ok(sqlx::query_scalar(query)
            .bind(user_id.0)
            .fetch_all(&self.pool)
            .await?)
    }

    // orgs

    #[allow(unused)] // Help rust-analyzer
    #[cfg(any(test, feature = "seed-support"))]
    async fn find_org_by_slug(&self, slug: &str) -> Result<Option<Org>> {
        let query = "
            SELECT *
            FROM orgs
            WHERE slug = $1
        ";
        Ok(sqlx::query_as(query)
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?)
    }

    #[cfg(any(test, feature = "seed-support"))]
    async fn create_org(&self, name: &str, slug: &str) -> Result<OrgId> {
        let query = "
            INSERT INTO orgs (name, slug)
            VALUES ($1, $2)
            RETURNING id
        ";
        Ok(sqlx::query_scalar(query)
            .bind(name)
            .bind(slug)
            .fetch_one(&self.pool)
            .await
            .map(OrgId)?)
    }

    #[cfg(any(test, feature = "seed-support"))]
    async fn add_org_member(&self, org_id: OrgId, user_id: UserId, is_admin: bool) -> Result<()> {
        let query = "
            INSERT INTO org_memberships (org_id, user_id, admin)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
        ";
        Ok(sqlx::query(query)
            .bind(org_id.0)
            .bind(user_id.0)
            .bind(is_admin)
            .execute(&self.pool)
            .await
            .map(drop)?)
    }

    // channels

    #[cfg(any(test, feature = "seed-support"))]
    async fn create_org_channel(&self, org_id: OrgId, name: &str) -> Result<ChannelId> {
        let query = "
            INSERT INTO channels (owner_id, owner_is_user, name)
            VALUES ($1, false, $2)
            RETURNING id
        ";
        Ok(sqlx::query_scalar(query)
            .bind(org_id.0)
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map(ChannelId)?)
    }

    #[allow(unused)] // Help rust-analyzer
    #[cfg(any(test, feature = "seed-support"))]
    async fn get_org_channels(&self, org_id: OrgId) -> Result<Vec<Channel>> {
        let query = "
            SELECT *
            FROM channels
            WHERE
                channels.owner_is_user = false AND
                channels.owner_id = $1
        ";
        Ok(sqlx::query_as(query)
            .bind(org_id.0)
            .fetch_all(&self.pool)
            .await?)
    }

    async fn get_accessible_channels(&self, user_id: UserId) -> Result<Vec<Channel>> {
        let query = "
            SELECT
                channels.*
            FROM
                channel_memberships, channels
            WHERE
                channel_memberships.user_id = $1 AND
                channel_memberships.channel_id = channels.id
        ";
        Ok(sqlx::query_as(query)
            .bind(user_id.0)
            .fetch_all(&self.pool)
            .await?)
    }

    async fn can_user_access_channel(
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
        Ok(sqlx::query_scalar::<_, i32>(query)
            .bind(user_id.0)
            .bind(channel_id.0)
            .fetch_optional(&self.pool)
            .await
            .map(|e| e.is_some())?)
    }

    #[cfg(any(test, feature = "seed-support"))]
    async fn add_channel_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        is_admin: bool,
    ) -> Result<()> {
        let query = "
            INSERT INTO channel_memberships (channel_id, user_id, admin)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
        ";
        Ok(sqlx::query(query)
            .bind(channel_id.0)
            .bind(user_id.0)
            .bind(is_admin)
            .execute(&self.pool)
            .await
            .map(drop)?)
    }

    // messages

    async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        sender_id: UserId,
        body: &str,
        timestamp: OffsetDateTime,
        nonce: u128,
    ) -> Result<MessageId> {
        let query = "
            INSERT INTO channel_messages (channel_id, sender_id, body, sent_at, nonce)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (nonce) DO UPDATE SET nonce = excluded.nonce
            RETURNING id
        ";
        Ok(sqlx::query_scalar(query)
            .bind(channel_id.0)
            .bind(sender_id.0)
            .bind(body)
            .bind(timestamp)
            .bind(Uuid::from_u128(nonce))
            .fetch_one(&self.pool)
            .await
            .map(MessageId)?)
    }

    async fn get_channel_messages(
        &self,
        channel_id: ChannelId,
        count: usize,
        before_id: Option<MessageId>,
    ) -> Result<Vec<ChannelMessage>> {
        let query = r#"
            SELECT * FROM (
                SELECT
                    id, channel_id, sender_id, body, sent_at AT TIME ZONE 'UTC' as sent_at, nonce
                FROM
                    channel_messages
                WHERE
                    channel_id = $1 AND
                    id < $2
                ORDER BY id DESC
                LIMIT $3
            ) as recent_messages
            ORDER BY id ASC
        "#;
        Ok(sqlx::query_as(query)
            .bind(channel_id.0)
            .bind(before_id.unwrap_or(MessageId::MAX))
            .bind(count as i64)
            .fetch_all(&self.pool)
            .await?)
    }

    #[cfg(test)]
    async fn teardown(&self, url: &str) {
        use util::ResultExt;

        let query = "
            SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname = current_database() AND pid <> pg_backend_pid();
        ";
        sqlx::query(query).execute(&self.pool).await.log_err();
        self.pool.close().await;
        <sqlx::Postgres as sqlx::migrate::MigrateDatabase>::drop_database(url)
            .await
            .log_err();
    }
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type, Serialize,
        )]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[allow(unused)]
            pub const MAX: Self = Self(i32::MAX);

            #[allow(unused)]
            pub fn from_proto(value: u64) -> Self {
                Self(value as i32)
            }

            #[allow(unused)]
            pub fn to_proto(&self) -> u64 {
                self.0 as u64
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

id_type!(UserId);
#[derive(Clone, Debug, FromRow, Serialize, PartialEq)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub admin: bool,
}

id_type!(OrgId);
#[derive(FromRow)]
pub struct Org {
    pub id: OrgId,
    pub name: String,
    pub slug: String,
}

id_type!(ChannelId);
#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub owner_id: i32,
    pub owner_is_user: bool,
}

id_type!(MessageId);
#[derive(Clone, Debug, FromRow)]
pub struct ChannelMessage {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub sender_id: UserId,
    pub body: String,
    pub sent_at: OffsetDateTime,
    pub nonce: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contacts {
    pub current: Vec<UserId>,
    pub incoming_requests: Vec<IncomingContactRequest>,
    pub outgoing_requests: Vec<UserId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingContactRequest {
    pub requesting_user_id: UserId,
    pub should_notify: bool,
}

fn fuzzy_like_string(string: &str) -> String {
    let mut result = String::with_capacity(string.len() * 2 + 1);
    for c in string.chars() {
        if c.is_alphanumeric() {
            result.push('%');
            result.push(c);
        }
    }
    result.push('%');
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use anyhow::anyhow;
    use collections::BTreeMap;
    use gpui::executor::Background;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use rand::prelude::*;
    use sqlx::{
        migrate::{MigrateDatabase, Migrator},
        Postgres,
    };
    use std::{path::Path, sync::Arc};
    use util::post_inc;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_users_by_ids() {
        for test_db in [
            TestDb::postgres().await,
            TestDb::fake(Arc::new(gpui::executor::Background::new())),
        ] {
            let db = test_db.db();

            let user = db.create_user("user", false).await.unwrap();
            let friend1 = db.create_user("friend-1", false).await.unwrap();
            let friend2 = db.create_user("friend-2", false).await.unwrap();
            let friend3 = db.create_user("friend-3", false).await.unwrap();

            assert_eq!(
                db.get_users_by_ids(vec![user, friend1, friend2, friend3])
                    .await
                    .unwrap(),
                vec![
                    User {
                        id: user,
                        github_login: "user".to_string(),
                        admin: false,
                    },
                    User {
                        id: friend1,
                        github_login: "friend-1".to_string(),
                        admin: false,
                    },
                    User {
                        id: friend2,
                        github_login: "friend-2".to_string(),
                        admin: false,
                    },
                    User {
                        id: friend3,
                        github_login: "friend-3".to_string(),
                        admin: false,
                    }
                ]
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_recent_channel_messages() {
        for test_db in [
            TestDb::postgres().await,
            TestDb::fake(Arc::new(gpui::executor::Background::new())),
        ] {
            let db = test_db.db();
            let user = db.create_user("user", false).await.unwrap();
            let org = db.create_org("org", "org").await.unwrap();
            let channel = db.create_org_channel(org, "channel").await.unwrap();
            for i in 0..10 {
                db.create_channel_message(
                    channel,
                    user,
                    &i.to_string(),
                    OffsetDateTime::now_utc(),
                    i,
                )
                .await
                .unwrap();
            }

            let messages = db.get_channel_messages(channel, 5, None).await.unwrap();
            assert_eq!(
                messages.iter().map(|m| &m.body).collect::<Vec<_>>(),
                ["5", "6", "7", "8", "9"]
            );

            let prev_messages = db
                .get_channel_messages(channel, 4, Some(messages[0].id))
                .await
                .unwrap();
            assert_eq!(
                prev_messages.iter().map(|m| &m.body).collect::<Vec<_>>(),
                ["1", "2", "3", "4"]
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_channel_message_nonces() {
        for test_db in [
            TestDb::postgres().await,
            TestDb::fake(Arc::new(gpui::executor::Background::new())),
        ] {
            let db = test_db.db();
            let user = db.create_user("user", false).await.unwrap();
            let org = db.create_org("org", "org").await.unwrap();
            let channel = db.create_org_channel(org, "channel").await.unwrap();

            let msg1_id = db
                .create_channel_message(channel, user, "1", OffsetDateTime::now_utc(), 1)
                .await
                .unwrap();
            let msg2_id = db
                .create_channel_message(channel, user, "2", OffsetDateTime::now_utc(), 2)
                .await
                .unwrap();
            let msg3_id = db
                .create_channel_message(channel, user, "3", OffsetDateTime::now_utc(), 1)
                .await
                .unwrap();
            let msg4_id = db
                .create_channel_message(channel, user, "4", OffsetDateTime::now_utc(), 2)
                .await
                .unwrap();

            assert_ne!(msg1_id, msg2_id);
            assert_eq!(msg1_id, msg3_id);
            assert_eq!(msg2_id, msg4_id);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_access_tokens() {
        let test_db = TestDb::postgres().await;
        let db = test_db.db();
        let user = db.create_user("the-user", false).await.unwrap();

        db.create_access_token_hash(user, "h1", 3).await.unwrap();
        db.create_access_token_hash(user, "h2", 3).await.unwrap();
        assert_eq!(
            db.get_access_token_hashes(user).await.unwrap(),
            &["h2".to_string(), "h1".to_string()]
        );

        db.create_access_token_hash(user, "h3", 3).await.unwrap();
        assert_eq!(
            db.get_access_token_hashes(user).await.unwrap(),
            &["h3".to_string(), "h2".to_string(), "h1".to_string(),]
        );

        db.create_access_token_hash(user, "h4", 3).await.unwrap();
        assert_eq!(
            db.get_access_token_hashes(user).await.unwrap(),
            &["h4".to_string(), "h3".to_string(), "h2".to_string(),]
        );

        db.create_access_token_hash(user, "h5", 3).await.unwrap();
        assert_eq!(
            db.get_access_token_hashes(user).await.unwrap(),
            &["h5".to_string(), "h4".to_string(), "h3".to_string()]
        );
    }

    #[test]
    fn test_fuzzy_like_string() {
        assert_eq!(fuzzy_like_string("abcd"), "%a%b%c%d%");
        assert_eq!(fuzzy_like_string("x y"), "%x%y%");
        assert_eq!(fuzzy_like_string(" z  "), "%z%");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_fuzzy_search_users() {
        let test_db = TestDb::postgres().await;
        let db = test_db.db();
        for github_login in [
            "california",
            "colorado",
            "oregon",
            "washington",
            "florida",
            "delaware",
            "rhode-island",
        ] {
            db.create_user(github_login, false).await.unwrap();
        }

        assert_eq!(
            fuzzy_search_user_names(db, "clr").await,
            &["colorado", "california"]
        );
        assert_eq!(
            fuzzy_search_user_names(db, "ro").await,
            &["rhode-island", "colorado", "oregon"],
        );

        async fn fuzzy_search_user_names(db: &Arc<dyn Db>, query: &str) -> Vec<String> {
            db.fuzzy_search_users(query, 10)
                .await
                .unwrap()
                .into_iter()
                .map(|user| user.github_login)
                .collect::<Vec<_>>()
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_add_contacts() {
        for test_db in [
            TestDb::postgres().await,
            TestDb::fake(Arc::new(gpui::executor::Background::new())),
        ] {
            let db = test_db.db();

            let user_1 = db.create_user("user1", false).await.unwrap();
            let user_2 = db.create_user("user2", false).await.unwrap();
            let user_3 = db.create_user("user3", false).await.unwrap();

            // User starts with no contacts
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                Contacts {
                    current: vec![],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );

            // User requests a contact. Both users see the pending request.
            db.send_contact_request(user_1, user_2).await.unwrap();
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                Contacts {
                    current: vec![],
                    outgoing_requests: vec![user_2],
                    incoming_requests: vec![],
                },
            );
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                Contacts {
                    current: vec![],
                    outgoing_requests: vec![],
                    incoming_requests: vec![IncomingContactRequest {
                        requesting_user_id: user_1,
                        should_notify: true
                    }],
                },
            );

            // User 2 dismisses the contact request notification without accepting or rejecting.
            // We shouldn't notify them again.
            db.dismiss_contact_request(user_1, user_2)
                .await
                .unwrap_err();
            db.dismiss_contact_request(user_2, user_1).await.unwrap();
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                Contacts {
                    current: vec![],
                    outgoing_requests: vec![],
                    incoming_requests: vec![IncomingContactRequest {
                        requesting_user_id: user_1,
                        should_notify: false
                    }],
                },
            );

            // User can't accept their own contact request
            db.respond_to_contact_request(user_1, user_2, true)
                .await
                .unwrap_err();

            // User accepts a contact request. Both users see the contact.
            db.respond_to_contact_request(user_2, user_1, true)
                .await
                .unwrap();
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                Contacts {
                    current: vec![user_2],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                Contacts {
                    current: vec![user_1],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );

            // Users cannot re-request existing contacts.
            db.send_contact_request(user_1, user_2).await.unwrap_err();
            db.send_contact_request(user_2, user_1).await.unwrap_err();

            // Users send each other concurrent contact requests and
            // see that they are immediately accepted.
            db.send_contact_request(user_1, user_3).await.unwrap();
            db.send_contact_request(user_3, user_1).await.unwrap();
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                Contacts {
                    current: vec![user_2, user_3],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );
            assert_eq!(
                db.get_contacts(user_3).await.unwrap(),
                Contacts {
                    current: vec![user_1],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );

            // User declines a contact request. Both users see that it is gone.
            db.send_contact_request(user_2, user_3).await.unwrap();
            db.respond_to_contact_request(user_3, user_2, false)
                .await
                .unwrap();
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                Contacts {
                    current: vec![user_1],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );
            assert_eq!(
                db.get_contacts(user_3).await.unwrap(),
                Contacts {
                    current: vec![user_1],
                    outgoing_requests: vec![],
                    incoming_requests: vec![],
                },
            );
        }
    }

    pub struct TestDb {
        pub db: Option<Arc<dyn Db>>,
        pub url: String,
    }

    impl TestDb {
        pub async fn postgres() -> Self {
            lazy_static! {
                static ref LOCK: Mutex<()> = Mutex::new(());
            }

            let _guard = LOCK.lock();
            let mut rng = StdRng::from_entropy();
            let name = format!("zed-test-{}", rng.gen::<u128>());
            let url = format!("postgres://postgres@localhost/{}", name);
            let migrations_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"));
            Postgres::create_database(&url)
                .await
                .expect("failed to create test db");
            let db = PostgresDb::new(&url, 5).await.unwrap();
            let migrator = Migrator::new(migrations_path).await.unwrap();
            migrator.run(&db.pool).await.unwrap();
            Self {
                db: Some(Arc::new(db)),
                url,
            }
        }

        pub fn fake(background: Arc<Background>) -> Self {
            Self {
                db: Some(Arc::new(FakeDb::new(background))),
                url: Default::default(),
            }
        }

        pub fn db(&self) -> &Arc<dyn Db> {
            self.db.as_ref().unwrap()
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if let Some(db) = self.db.take() {
                futures::executor::block_on(db.teardown(&self.url));
            }
        }
    }

    pub struct FakeDb {
        background: Arc<Background>,
        users: Mutex<BTreeMap<UserId, User>>,
        next_user_id: Mutex<i32>,
        orgs: Mutex<BTreeMap<OrgId, Org>>,
        next_org_id: Mutex<i32>,
        org_memberships: Mutex<BTreeMap<(OrgId, UserId), bool>>,
        channels: Mutex<BTreeMap<ChannelId, Channel>>,
        next_channel_id: Mutex<i32>,
        channel_memberships: Mutex<BTreeMap<(ChannelId, UserId), bool>>,
        channel_messages: Mutex<BTreeMap<MessageId, ChannelMessage>>,
        next_channel_message_id: Mutex<i32>,
        contacts: Mutex<Vec<FakeContact>>,
    }

    struct FakeContact {
        requester_id: UserId,
        responder_id: UserId,
        accepted: bool,
        should_notify: bool,
    }

    impl FakeDb {
        pub fn new(background: Arc<Background>) -> Self {
            Self {
                background,
                users: Default::default(),
                next_user_id: Mutex::new(1),
                orgs: Default::default(),
                next_org_id: Mutex::new(1),
                org_memberships: Default::default(),
                channels: Default::default(),
                next_channel_id: Mutex::new(1),
                channel_memberships: Default::default(),
                channel_messages: Default::default(),
                next_channel_message_id: Mutex::new(1),
                contacts: Default::default(),
            }
        }
    }

    #[async_trait]
    impl Db for FakeDb {
        async fn create_user(&self, github_login: &str, admin: bool) -> Result<UserId> {
            self.background.simulate_random_delay().await;

            let mut users = self.users.lock();
            if let Some(user) = users
                .values()
                .find(|user| user.github_login == github_login)
            {
                Ok(user.id)
            } else {
                let user_id = UserId(post_inc(&mut *self.next_user_id.lock()));
                users.insert(
                    user_id,
                    User {
                        id: user_id,
                        github_login: github_login.to_string(),
                        admin,
                    },
                );
                Ok(user_id)
            }
        }

        async fn get_all_users(&self) -> Result<Vec<User>> {
            unimplemented!()
        }

        async fn fuzzy_search_users(&self, _: &str, _: u32) -> Result<Vec<User>> {
            unimplemented!()
        }

        async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
            Ok(self.get_users_by_ids(vec![id]).await?.into_iter().next())
        }

        async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
            self.background.simulate_random_delay().await;
            let users = self.users.lock();
            Ok(ids.iter().filter_map(|id| users.get(id).cloned()).collect())
        }

        async fn get_user_by_github_login(&self, _github_login: &str) -> Result<Option<User>> {
            unimplemented!()
        }

        async fn set_user_is_admin(&self, _id: UserId, _is_admin: bool) -> Result<()> {
            unimplemented!()
        }

        async fn destroy_user(&self, _id: UserId) -> Result<()> {
            unimplemented!()
        }

        async fn get_contacts(&self, id: UserId) -> Result<Contacts> {
            self.background.simulate_random_delay().await;
            let mut current = Vec::new();
            let mut requests_sent = Vec::new();
            let mut requests_received = Vec::new();
            for contact in self.contacts.lock().iter() {
                if contact.requester_id == id {
                    if contact.accepted {
                        current.push(contact.responder_id);
                    } else {
                        requests_sent.push(contact.responder_id);
                    }
                } else if contact.responder_id == id {
                    if contact.accepted {
                        current.push(contact.requester_id);
                    } else {
                        requests_received.push(IncomingContactRequest {
                            requesting_user_id: contact.requester_id,
                            should_notify: contact.should_notify,
                        });
                    }
                }
            }
            Ok(Contacts {
                current,
                outgoing_requests,
                incoming_requests,
            })
        }

        async fn send_contact_request(
            &self,
            requester_id: UserId,
            responder_id: UserId,
        ) -> Result<()> {
            let mut contacts = self.contacts.lock();
            for contact in contacts.iter_mut() {
                if contact.requester_id == requester_id && contact.responder_id == responder_id {
                    if contact.accepted {
                        Err(anyhow!("contact already exists"))?;
                    } else {
                        Err(anyhow!("contact already requested"))?;
                    }
                }
                if contact.responder_id == requester_id && contact.requester_id == responder_id {
                    if contact.accepted {
                        Err(anyhow!("contact already exists"))?;
                    } else {
                        contact.accepted = true;
                        return Ok(());
                    }
                }
            }
            contacts.push(FakeContact {
                requester_id,
                responder_id,
                accepted: false,
                should_notify: true,
            });
            Ok(())
        }

        async fn dismiss_contact_request(
            &self,
            responder_id: UserId,
            requester_id: UserId,
        ) -> Result<()> {
            let mut contacts = self.contacts.lock();
            for contact in contacts.iter_mut() {
                if contact.requester_id == requester_id && contact.responder_id == responder_id {
                    if contact.accepted {
                        return Err(anyhow!("contact already confirmed"));
                    }
                    contact.should_notify = false;
                    return Ok(());
                }
            }
            Err(anyhow!("no such contact request"))
        }

        async fn respond_to_contact_request(
            &self,
            responder_id: UserId,
            requester_id: UserId,
            accept: bool,
        ) -> Result<()> {
            let mut contacts = self.contacts.lock();
            for (ix, contact) in contacts.iter_mut().enumerate() {
                if contact.requester_id == requester_id && contact.responder_id == responder_id {
                    if contact.accepted {
                        return Err(anyhow!("contact already confirmed"));
                    }
                    if accept {
                        contact.accepted = true;
                    } else {
                        contacts.remove(ix);
                    }
                    return Ok(());
                }
            }
            Err(anyhow!("no such contact request"))
        }

        async fn create_access_token_hash(
            &self,
            _user_id: UserId,
            _access_token_hash: &str,
            _max_access_token_count: usize,
        ) -> Result<()> {
            unimplemented!()
        }

        async fn get_access_token_hashes(&self, _user_id: UserId) -> Result<Vec<String>> {
            unimplemented!()
        }

        async fn find_org_by_slug(&self, _slug: &str) -> Result<Option<Org>> {
            unimplemented!()
        }

        async fn create_org(&self, name: &str, slug: &str) -> Result<OrgId> {
            self.background.simulate_random_delay().await;
            let mut orgs = self.orgs.lock();
            if orgs.values().any(|org| org.slug == slug) {
                Err(anyhow!("org already exists"))
            } else {
                let org_id = OrgId(post_inc(&mut *self.next_org_id.lock()));
                orgs.insert(
                    org_id,
                    Org {
                        id: org_id,
                        name: name.to_string(),
                        slug: slug.to_string(),
                    },
                );
                Ok(org_id)
            }
        }

        async fn add_org_member(
            &self,
            org_id: OrgId,
            user_id: UserId,
            is_admin: bool,
        ) -> Result<()> {
            self.background.simulate_random_delay().await;
            if !self.orgs.lock().contains_key(&org_id) {
                return Err(anyhow!("org does not exist"));
            }
            if !self.users.lock().contains_key(&user_id) {
                return Err(anyhow!("user does not exist"));
            }

            self.org_memberships
                .lock()
                .entry((org_id, user_id))
                .or_insert(is_admin);
            Ok(())
        }

        async fn create_org_channel(&self, org_id: OrgId, name: &str) -> Result<ChannelId> {
            self.background.simulate_random_delay().await;
            if !self.orgs.lock().contains_key(&org_id) {
                return Err(anyhow!("org does not exist"));
            }

            let mut channels = self.channels.lock();
            let channel_id = ChannelId(post_inc(&mut *self.next_channel_id.lock()));
            channels.insert(
                channel_id,
                Channel {
                    id: channel_id,
                    name: name.to_string(),
                    owner_id: org_id.0,
                    owner_is_user: false,
                },
            );
            Ok(channel_id)
        }

        async fn get_org_channels(&self, org_id: OrgId) -> Result<Vec<Channel>> {
            self.background.simulate_random_delay().await;
            Ok(self
                .channels
                .lock()
                .values()
                .filter(|channel| !channel.owner_is_user && channel.owner_id == org_id.0)
                .cloned()
                .collect())
        }

        async fn get_accessible_channels(&self, user_id: UserId) -> Result<Vec<Channel>> {
            self.background.simulate_random_delay().await;
            let channels = self.channels.lock();
            let memberships = self.channel_memberships.lock();
            Ok(channels
                .values()
                .filter(|channel| memberships.contains_key(&(channel.id, user_id)))
                .cloned()
                .collect())
        }

        async fn can_user_access_channel(
            &self,
            user_id: UserId,
            channel_id: ChannelId,
        ) -> Result<bool> {
            self.background.simulate_random_delay().await;
            Ok(self
                .channel_memberships
                .lock()
                .contains_key(&(channel_id, user_id)))
        }

        async fn add_channel_member(
            &self,
            channel_id: ChannelId,
            user_id: UserId,
            is_admin: bool,
        ) -> Result<()> {
            self.background.simulate_random_delay().await;
            if !self.channels.lock().contains_key(&channel_id) {
                return Err(anyhow!("channel does not exist"));
            }
            if !self.users.lock().contains_key(&user_id) {
                return Err(anyhow!("user does not exist"));
            }

            self.channel_memberships
                .lock()
                .entry((channel_id, user_id))
                .or_insert(is_admin);
            Ok(())
        }

        async fn create_channel_message(
            &self,
            channel_id: ChannelId,
            sender_id: UserId,
            body: &str,
            timestamp: OffsetDateTime,
            nonce: u128,
        ) -> Result<MessageId> {
            self.background.simulate_random_delay().await;
            if !self.channels.lock().contains_key(&channel_id) {
                return Err(anyhow!("channel does not exist"));
            }
            if !self.users.lock().contains_key(&sender_id) {
                return Err(anyhow!("user does not exist"));
            }

            let mut messages = self.channel_messages.lock();
            if let Some(message) = messages
                .values()
                .find(|message| message.nonce.as_u128() == nonce)
            {
                Ok(message.id)
            } else {
                let message_id = MessageId(post_inc(&mut *self.next_channel_message_id.lock()));
                messages.insert(
                    message_id,
                    ChannelMessage {
                        id: message_id,
                        channel_id,
                        sender_id,
                        body: body.to_string(),
                        sent_at: timestamp,
                        nonce: Uuid::from_u128(nonce),
                    },
                );
                Ok(message_id)
            }
        }

        async fn get_channel_messages(
            &self,
            channel_id: ChannelId,
            count: usize,
            before_id: Option<MessageId>,
        ) -> Result<Vec<ChannelMessage>> {
            let mut messages = self
                .channel_messages
                .lock()
                .values()
                .rev()
                .filter(|message| {
                    message.channel_id == channel_id
                        && message.id < before_id.unwrap_or(MessageId::MAX)
                })
                .take(count)
                .cloned()
                .collect::<Vec<_>>();
            messages.sort_unstable_by_key(|message| message.id);
            Ok(messages)
        }

        async fn teardown(&self, _: &str) {}
    }
}
