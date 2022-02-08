use anyhow::Context;
use async_std::task::{block_on, yield_now};
use serde::Serialize;
use sqlx::{types::Uuid, FromRow, Result};
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
        wants_releases: bool,
        wants_updates: bool,
        wants_community: bool,
    ) -> Result<SignupId> {
        test_support!(self, {
            let query = "
                INSERT INTO signups (
                    github_login,
                    email_address,
                    about,
                    wants_releases,
                    wants_updates,
                    wants_community
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(github_login)
                .bind(email_address)
                .bind(about)
                .bind(wants_releases)
                .bind(wants_updates)
                .bind(wants_community)
                .fetch_one(&self.pool)
                .await
                .map(SignupId)
        })
    }

    pub async fn get_all_signups(&self) -> Result<Vec<Signup>> {
        test_support!(self, {
            let query = "SELECT * FROM signups ORDER BY github_login ASC";
            sqlx::query_as(query).fetch_all(&self.pool).await
        })
    }

    pub async fn destroy_signup(&self, id: SignupId) -> Result<()> {
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
                ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
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

    pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
        let users = self.get_users_by_ids([id]).await?;
        Ok(users.into_iter().next())
    }

    pub async fn get_users_by_ids(
        &self,
        ids: impl IntoIterator<Item = UserId>,
    ) -> Result<Vec<User>> {
        let ids = ids.into_iter().map(|id| id.0).collect::<Vec<_>>();
        test_support!(self, {
            let query = "
                SELECT users.*
                FROM users
                WHERE users.id = ANY ($1)
            ";

            sqlx::query_as(query).bind(&ids).fetch_all(&self.pool).await
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

    pub async fn destroy_user(&self, id: UserId) -> Result<()> {
        test_support!(self, {
            let query = "DELETE FROM access_tokens WHERE user_id = $1;";
            sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?;
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
        access_token_hash: &str,
        max_access_token_count: usize,
    ) -> Result<()> {
        test_support!(self, {
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
            tx.commit().await
        })
    }

    pub async fn get_access_token_hashes(&self, user_id: UserId) -> Result<Vec<String>> {
        test_support!(self, {
            let query = "
                SELECT hash
                FROM access_tokens
                WHERE user_id = $1
                ORDER BY id DESC
            ";
            sqlx::query_scalar(query)
                .bind(user_id.0)
                .fetch_all(&self.pool)
                .await
        })
    }

    // orgs

    #[allow(unused)] // Help rust-analyzer
    #[cfg(any(test, feature = "seed-support"))]
    pub async fn find_org_by_slug(&self, slug: &str) -> Result<Option<Org>> {
        test_support!(self, {
            let query = "
                SELECT *
                FROM orgs
                WHERE slug = $1
            ";
            sqlx::query_as(query)
                .bind(slug)
                .fetch_optional(&self.pool)
                .await
        })
    }

    #[cfg(any(test, feature = "seed-support"))]
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

    #[cfg(any(test, feature = "seed-support"))]
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
                ON CONFLICT DO NOTHING
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

    #[cfg(any(test, feature = "seed-support"))]
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

    #[allow(unused)] // Help rust-analyzer
    #[cfg(any(test, feature = "seed-support"))]
    pub async fn get_org_channels(&self, org_id: OrgId) -> Result<Vec<Channel>> {
        test_support!(self, {
            let query = "
                SELECT *
                FROM channels
                WHERE
                    channels.owner_is_user = false AND
                    channels.owner_id = $1
            ";
            sqlx::query_as(query)
                .bind(org_id.0)
                .fetch_all(&self.pool)
                .await
        })
    }

    pub async fn get_accessible_channels(&self, user_id: UserId) -> Result<Vec<Channel>> {
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

    #[cfg(any(test, feature = "seed-support"))]
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
                ON CONFLICT DO NOTHING
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
        nonce: u128,
    ) -> Result<MessageId> {
        test_support!(self, {
            let query = "
                INSERT INTO channel_messages (channel_id, sender_id, body, sent_at, nonce)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (nonce) DO UPDATE SET nonce = excluded.nonce
                RETURNING id
            ";
            sqlx::query_scalar(query)
                .bind(channel_id.0)
                .bind(sender_id.0)
                .bind(body)
                .bind(timestamp)
                .bind(Uuid::from_u128(nonce))
                .fetch_one(&self.pool)
                .await
                .map(MessageId)
        })
    }

    pub async fn get_channel_messages(
        &self,
        channel_id: ChannelId,
        count: usize,
        before_id: Option<MessageId>,
    ) -> Result<Vec<ChannelMessage>> {
        test_support!(self, {
            let query = r#"
                SELECT * FROM (
                    SELECT
                        id, sender_id, body, sent_at AT TIME ZONE 'UTC' as sent_at, nonce
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
    };
}

id_type!(UserId);
#[derive(Debug, FromRow, Serialize, PartialEq)]
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

id_type!(SignupId);
#[derive(Debug, FromRow, Serialize)]
pub struct Signup {
    pub id: SignupId,
    pub github_login: String,
    pub email_address: String,
    pub about: String,
    pub wants_releases: Option<bool>,
    pub wants_updates: Option<bool>,
    pub wants_community: Option<bool>,
}

id_type!(ChannelId);
#[derive(Debug, FromRow, Serialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
}

id_type!(MessageId);
#[derive(Debug, FromRow)]
pub struct ChannelMessage {
    pub id: MessageId,
    pub sender_id: UserId,
    pub body: String,
    pub sent_at: OffsetDateTime,
    pub nonce: Uuid,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use rand::prelude::*;
    use sqlx::{
        migrate::{MigrateDatabase, Migrator},
        Postgres,
    };
    use std::{
        mem,
        path::Path,
        sync::atomic::{AtomicUsize, Ordering::SeqCst},
    };
    use util::ResultExt as _;

    pub struct TestDb {
        pub db: Option<Db>,
        pub name: String,
        pub url: String,
    }

    lazy_static! {
        static ref DB_POOL: Mutex<Vec<TestDb>> = Default::default();
        static ref DB_COUNT: AtomicUsize = Default::default();
    }

    impl TestDb {
        pub fn new() -> Self {
            DB_COUNT.fetch_add(1, SeqCst);
            let mut pool = DB_POOL.lock();
            if let Some(db) = pool.pop() {
                db.truncate();
                db
            } else {
                let mut rng = StdRng::from_entropy();
                let name = format!("zed-test-{}", rng.gen::<u128>());
                let url = format!("postgres://postgres@localhost/{}", name);
                let migrations_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations"));
                let db = block_on(async {
                    Postgres::create_database(&url)
                        .await
                        .expect("failed to create test db");
                    let mut db = Db::new(&url, 5).await.unwrap();
                    db.test_mode = true;
                    let migrator = Migrator::new(migrations_path).await.unwrap();
                    migrator.run(&db.pool).await.unwrap();
                    db
                });

                Self {
                    db: Some(db),
                    name,
                    url,
                }
            }
        }

        pub fn db(&self) -> &Db {
            self.db.as_ref().unwrap()
        }

        fn truncate(&self) {
            block_on(async {
                let query = "
                    SELECT tablename FROM pg_tables
                    WHERE schemaname = 'public';
                ";
                let table_names = sqlx::query_scalar::<_, String>(query)
                    .fetch_all(&self.db().pool)
                    .await
                    .unwrap();
                sqlx::query(&format!(
                    "TRUNCATE TABLE {} RESTART IDENTITY",
                    table_names.join(", ")
                ))
                .execute(&self.db().pool)
                .await
                .unwrap();
            })
        }

        async fn teardown(mut self) -> Result<()> {
            let db = self.db.take().unwrap();
            let query = "
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}' AND pid <> pg_backend_pid();
            ";
            sqlx::query(query)
                .bind(&self.name)
                .execute(&db.pool)
                .await?;
            db.pool.close().await;
            Postgres::drop_database(&self.url).await?;
            Ok(())
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if let Some(db) = self.db.take() {
                DB_POOL.lock().push(TestDb {
                    db: Some(db),
                    name: mem::take(&mut self.name),
                    url: mem::take(&mut self.url),
                });
                if DB_COUNT.fetch_sub(1, SeqCst) == 1 {
                    block_on(async move {
                        let mut pool = DB_POOL.lock();
                        for db in pool.drain(..) {
                            db.teardown().await.log_err();
                        }
                    });
                }
            }
        }
    }

    #[gpui::test]
    async fn test_get_users_by_ids() {
        let test_db = TestDb::new();
        let db = test_db.db();

        let user = db.create_user("user", false).await.unwrap();
        let friend1 = db.create_user("friend-1", false).await.unwrap();
        let friend2 = db.create_user("friend-2", false).await.unwrap();
        let friend3 = db.create_user("friend-3", false).await.unwrap();

        assert_eq!(
            db.get_users_by_ids([user, friend1, friend2, friend3])
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

    #[gpui::test]
    async fn test_recent_channel_messages() {
        let test_db = TestDb::new();
        let db = test_db.db();
        let user = db.create_user("user", false).await.unwrap();
        let org = db.create_org("org", "org").await.unwrap();
        let channel = db.create_org_channel(org, "channel").await.unwrap();
        for i in 0..10 {
            db.create_channel_message(channel, user, &i.to_string(), OffsetDateTime::now_utc(), i)
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

    #[gpui::test]
    async fn test_channel_message_nonces() {
        let test_db = TestDb::new();
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

    #[gpui::test]
    async fn test_create_access_tokens() {
        let test_db = TestDb::new();
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
}
