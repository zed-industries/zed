use anyhow::Context;
use async_std::task::{block_on, yield_now};
use serde::Serialize;
use sqlx::{FromRow, Result};
use time::OffsetDateTime;

pub use async_sqlx_session::PostgresSessionStore as SessionStore;
pub use sqlx::postgres::PgPoolOptions as DbOptions;

macro_rules! test_support {
    ($self:ident, { $($token:tt)* }) => {{
        let body = async {
            $($token)*
        };
        if $self.test_mode {
            yield_now().await;
            block_on(body)
        } else {
            body.await
        }
    }};
}

#[derive(Clone)]
pub struct Db {
    pool: sqlx::PgPool,
    test_mode: bool,
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub admin: bool,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Signup {
    pub id: SignupId,
    pub github_login: String,
    pub email_address: String,
    pub about: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct ChannelMessage {
    pub id: MessageId,
    pub sender_id: UserId,
    pub body: String,
    pub sent_at: OffsetDateTime,
}

impl Db {
    pub async fn new(url: &str, max_connections: u32) -> tide::Result<Self> {
        let pool = DbOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await
            .context("failed to connect to postgres database")?;
        Ok(Self {
            pool,
            test_mode: false,
        })
    }

    // signups

    pub async fn create_signup(
        &self,
        github_login: &str,
        email_address: &str,
        about: &str,
    ) -> Result<SignupId> {
        test_support!(self, {
            let query = "
                INSERT INTO signups (github_login, email_address, about)
                VALUES ($1, $2, $3)
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(github_login)
                .bind(email_address)
                .bind(about)
                .fetch_one(&self.pool)
                .await
                .map(SignupId)
        })
    }

    pub async fn get_all_signups(&self) -> Result<Vec<Signup>> {
        test_support!(self, {
            let query = "SELECT * FROM users ORDER BY github_login ASC";
            sqlx::query_as(query).fetch_all(&self.pool).await
        })
    }

    pub async fn delete_signup(&self, id: SignupId) -> Result<()> {
        test_support!(self, {
            let query = "DELETE FROM signups WHERE id = $1";
            sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)
        })
    }

    // users

    pub async fn create_user(&self, github_login: &str, admin: bool) -> Result<UserId> {
        test_support!(self, {
            let query = "
                INSERT INTO users (github_login, admin)
                VALUES ($1, $2)
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(github_login)
                .bind(admin)
                .fetch_one(&self.pool)
                .await
                .map(UserId)
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        test_support!(self, {
            let query = "SELECT * FROM users ORDER BY github_login ASC";
            sqlx::query_as(query).fetch_all(&self.pool).await
        })
    }

    pub async fn get_users_by_ids(
        &self,
        requester_id: UserId,
        ids: impl Iterator<Item = UserId>,
    ) -> Result<Vec<User>> {
        test_support!(self, {
            // Only return users that are in a common channel with the requesting user.
            let query = "
                SELECT users.*
                FROM
                    users, channel_memberships
                WHERE
                    users.id = ANY ($1) AND
                    channel_memberships.user_id = users.id AND
                    channel_memberships.channel_id IN (
                        SELECT channel_id
                        FROM channel_memberships
                        WHERE channel_memberships.user_id = $2
                    )
            ";

            sqlx::query_as(query)
                .bind(&ids.map(|id| id.0).collect::<Vec<_>>())
                .bind(requester_id)
                .fetch_all(&self.pool)
                .await
        })
    }

    pub async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        test_support!(self, {
            let query = "SELECT * FROM users WHERE github_login = $1 LIMIT 1";
            sqlx::query_as(query)
                .bind(github_login)
                .fetch_optional(&self.pool)
                .await
        })
    }

    pub async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        test_support!(self, {
            let query = "UPDATE users SET admin = $1 WHERE id = $2";
            sqlx::query(query)
                .bind(is_admin)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)
        })
    }

    pub async fn delete_user(&self, id: UserId) -> Result<()> {
        test_support!(self, {
            let query = "DELETE FROM users WHERE id = $1;";
            sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)
        })
    }

    // access tokens

    pub async fn create_access_token_hash(
        &self,
        user_id: UserId,
        access_token_hash: String,
    ) -> Result<()> {
        test_support!(self, {
            let query = "
            INSERT INTO access_tokens (user_id, hash)
            VALUES ($1, $2)
        ";
            sqlx::query(query)
                .bind(user_id.0)
                .bind(access_token_hash)
                .execute(&self.pool)
                .await
                .map(drop)
        })
    }

    pub async fn get_access_token_hashes(&self, user_id: UserId) -> Result<Vec<String>> {
        test_support!(self, {
            let query = "SELECT hash FROM access_tokens WHERE user_id = $1";
            sqlx::query_scalar(query)
                .bind(user_id.0)
                .fetch_all(&self.pool)
                .await
        })
    }

    // orgs

    #[cfg(test)]
    pub async fn create_org(&self, name: &str, slug: &str) -> Result<OrgId> {
        test_support!(self, {
            let query = "
                INSERT INTO orgs (name, slug)
                VALUES ($1, $2)
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(name)
                .bind(slug)
                .fetch_one(&self.pool)
                .await
                .map(OrgId)
        })
    }

    #[cfg(test)]
    pub async fn add_org_member(
        &self,
        org_id: OrgId,
        user_id: UserId,
        is_admin: bool,
    ) -> Result<()> {
        test_support!(self, {
            let query = "
                INSERT INTO org_memberships (org_id, user_id, admin)
                VALUES ($1, $2, $3)
            ";
            sqlx::query(query)
                .bind(org_id.0)
                .bind(user_id.0)
                .bind(is_admin)
                .execute(&self.pool)
                .await
                .map(drop)
        })
    }

    // channels

    #[cfg(test)]
    pub async fn create_org_channel(&self, org_id: OrgId, name: &str) -> Result<ChannelId> {
        test_support!(self, {
            let query = "
                INSERT INTO channels (owner_id, owner_is_user, name)
                VALUES ($1, false, $2)
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(org_id.0)
                .bind(name)
                .fetch_one(&self.pool)
                .await
                .map(ChannelId)
        })
    }

    pub async fn get_channels_for_user(&self, user_id: UserId) -> Result<Vec<Channel>> {
        test_support!(self, {
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
                .fetch_all(&self.pool)
                .await
        })
    }

    pub async fn can_user_access_channel(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> Result<bool> {
        test_support!(self, {
            let query = "
                SELECT id
                FROM channel_memberships
                WHERE user_id = $1 AND channel_id = $2
                LIMIT 1
            ";
            sqlx::query_scalar::<_, i32>(query)
                .bind(user_id.0)
                .bind(channel_id.0)
                .fetch_optional(&self.pool)
                .await
                .map(|e| e.is_some())
        })
    }

    #[cfg(test)]
    pub async fn add_channel_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        is_admin: bool,
    ) -> Result<()> {
        test_support!(self, {
            let query = "
                INSERT INTO channel_memberships (channel_id, user_id, admin)
                VALUES ($1, $2, $3)
            ";
            sqlx::query(query)
                .bind(channel_id.0)
                .bind(user_id.0)
                .bind(is_admin)
                .execute(&self.pool)
                .await
                .map(drop)
        })
    }

    // messages

    pub async fn create_channel_message(
        &self,
        channel_id: ChannelId,
        sender_id: UserId,
        body: &str,
        timestamp: OffsetDateTime,
    ) -> Result<MessageId> {
        test_support!(self, {
            let query = "
                INSERT INTO channel_messages (channel_id, sender_id, body, sent_at)
                VALUES ($1, $2, $3, $4)
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(channel_id.0)
                .bind(sender_id.0)
                .bind(body)
                .bind(timestamp)
                .fetch_one(&self.pool)
                .await
                .map(MessageId)
        })
    }

    pub async fn get_recent_channel_messages(
        &self,
        channel_id: ChannelId,
        count: usize,
        before_id: Option<MessageId>,
    ) -> Result<Vec<ChannelMessage>> {
        test_support!(self, {
            let query = r#"
                SELECT * FROM (
                    SELECT
                        id, sender_id, body, sent_at AT TIME ZONE 'UTC' as sent_at
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
            sqlx::query_as(query)
                .bind(channel_id.0)
                .bind(before_id.unwrap_or(MessageId::MAX))
                .bind(count as i64)
                .fetch_all(&self.pool)
                .await
        })
    }
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type, Serialize)]
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
    };
}

id_type!(UserId);
id_type!(OrgId);
id_type!(ChannelId);
id_type!(SignupId);
id_type!(MessageId);

#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::prelude::*;
    use sqlx::{
        migrate::{MigrateDatabase, Migrator},
        Postgres,
    };
    use std::path::Path;

    pub struct TestDb {
        pub db: Db,
        pub name: String,
        pub url: String,
    }

    impl TestDb {
        pub fn new() -> Self {
            // Enable tests to run in parallel by serializing the creation of each test database.
            lazy_static::lazy_static! {
                static ref DB_CREATION: std::sync::Mutex<()> = std::sync::Mutex::new(());
            }

            let mut rng = StdRng::from_entropy();
            let name = format!("zed-test-{}", rng.gen::<u128>());
            let url = format!("postgres://postgres@localhost/{}", name);
            let migrations_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"));
            let db = block_on(async {
                {
                    let _lock = DB_CREATION.lock();
                    Postgres::create_database(&url)
                        .await
                        .expect("failed to create test db");
                }
                let mut db = Db::new(&url, 5).await.unwrap();
                db.test_mode = true;
                let migrator = Migrator::new(migrations_path).await.unwrap();
                migrator.run(&db.pool).await.unwrap();
                db
            });

            Self { db, name, url }
        }

        pub fn db(&self) -> &Db {
            &self.db
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            block_on(async {
                let query = "
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE pg_stat_activity.datname = '{}' AND pid <> pg_backend_pid();
                ";
                sqlx::query(query)
                    .bind(&self.name)
                    .execute(&self.db.pool)
                    .await
                    .unwrap();
                self.db.pool.close().await;
                Postgres::drop_database(&self.url).await.unwrap();
            });
        }
    }

    #[gpui::test]
    async fn test_recent_channel_messages() {
        let test_db = TestDb::new();
        let db = test_db.db();
        let user = db.create_user("user", false).await.unwrap();
        let org = db.create_org("org", "org").await.unwrap();
        let channel = db.create_org_channel(org, "channel").await.unwrap();
        for i in 0..10 {
            db.create_channel_message(channel, user, &i.to_string(), OffsetDateTime::now_utc())
                .await
                .unwrap();
        }

        let messages = db
            .get_recent_channel_messages(channel, 5, None)
            .await
            .unwrap();
        assert_eq!(
            messages.iter().map(|m| &m.body).collect::<Vec<_>>(),
            ["5", "6", "7", "8", "9"]
        );

        let prev_messages = db
            .get_recent_channel_messages(channel, 4, Some(messages[0].id))
            .await
            .unwrap();
        assert_eq!(
            prev_messages.iter().map(|m| &m.body).collect::<Vec<_>>(),
            ["1", "2", "3", "4"]
        );
    }
}
