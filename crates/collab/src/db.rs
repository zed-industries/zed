use crate::{Error, Result};
use anyhow::anyhow;
use axum::http::StatusCode;
use collections::HashMap;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::{Migrate as _, Migration, MigrationSource},
    types::Uuid,
    FromRow,
};
use std::{path::Path, time::Duration};
use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(test)]
pub type DefaultDb = Db<sqlx::Sqlite>;

#[cfg(not(test))]
pub type DefaultDb = Db<sqlx::Postgres>;

pub struct Db<D: sqlx::Database> {
    pool: sqlx::Pool<D>,
    #[cfg(test)]
    background: Option<std::sync::Arc<gpui::executor::Background>>,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

macro_rules! test_support {
    ($self:ident, { $($token:tt)* }) => {{
        let body = async {
            $($token)*
        };

        if cfg!(test) {
            #[cfg(not(test))]
            unreachable!();

            #[cfg(test)]
            if let Some(background) = $self.background.as_ref() {
                background.simulate_random_delay().await;
            }

            #[cfg(test)]
            $self.runtime.as_ref().unwrap().block_on(body)
        } else {
            body.await
        }
    }};
}

pub trait RowsAffected {
    fn rows_affected(&self) -> u64;
}

#[cfg(test)]
impl RowsAffected for sqlx::sqlite::SqliteQueryResult {
    fn rows_affected(&self) -> u64 {
        self.rows_affected()
    }
}

impl RowsAffected for sqlx::postgres::PgQueryResult {
    fn rows_affected(&self) -> u64 {
        self.rows_affected()
    }
}

#[cfg(test)]
impl Db<sqlx::Sqlite> {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self> {
        use std::str::FromStr as _;
        let options = sqlx::sqlite::SqliteConnectOptions::from_str(url)
            .unwrap()
            .create_if_missing(true)
            .shared_cache(true);
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .min_connections(2)
            .max_connections(max_connections)
            .connect_with(options)
            .await?;
        Ok(Self {
            pool,
            background: None,
            runtime: None,
        })
    }

    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        test_support!(self, {
            let query = "
                SELECT users.*
                FROM users
                WHERE users.id IN (SELECT value from json_each($1))
            ";
            Ok(sqlx::query_as(query)
                .bind(&serde_json::json!(ids))
                .fetch_all(&self.pool)
                .await?)
        })
    }

    pub async fn get_user_metrics_id(&self, id: UserId) -> Result<String> {
        test_support!(self, {
            let query = "
                SELECT metrics_id
                FROM users
                WHERE id = $1
            ";
            Ok(sqlx::query_scalar(query)
                .bind(id)
                .fetch_one(&self.pool)
                .await?)
        })
    }

    pub async fn create_user(
        &self,
        email_address: &str,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        test_support!(self, {
            let query = "
                INSERT INTO users (email_address, github_login, github_user_id, admin, metrics_id)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
                RETURNING id, metrics_id
            ";

            let (user_id, metrics_id): (UserId, String) = sqlx::query_as(query)
                .bind(email_address)
                .bind(params.github_login)
                .bind(params.github_user_id)
                .bind(admin)
                .bind(Uuid::new_v4().to_string())
                .fetch_one(&self.pool)
                .await?;
            Ok(NewUserResult {
                user_id,
                metrics_id,
                signup_device_id: None,
                inviting_user_id: None,
            })
        })
    }

    pub async fn fuzzy_search_users(&self, _name_query: &str, _limit: u32) -> Result<Vec<User>> {
        unimplemented!()
    }

    pub async fn create_user_from_invite(
        &self,
        _invite: &Invite,
        _user: NewUserParams,
    ) -> Result<Option<NewUserResult>> {
        unimplemented!()
    }

    pub async fn create_signup(&self, _signup: &Signup) -> Result<()> {
        unimplemented!()
    }

    pub async fn create_invite_from_code(
        &self,
        _code: &str,
        _email_address: &str,
        _device_id: Option<&str>,
    ) -> Result<Invite> {
        unimplemented!()
    }

    pub async fn record_sent_invites(&self, _invites: &[Invite]) -> Result<()> {
        unimplemented!()
    }
}

impl Db<sqlx::Postgres> {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;
        Ok(Self {
            pool,
            #[cfg(test)]
            background: None,
            #[cfg(test)]
            runtime: None,
        })
    }

    #[cfg(test)]
    pub fn teardown(&self, url: &str) {
        self.runtime.as_ref().unwrap().block_on(async {
            use util::ResultExt;
            let query = "
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = current_database() AND pid <> pg_backend_pid();
            ";
            sqlx::query(query).execute(&self.pool).await.log_err();
            self.pool.close().await;
            <sqlx::Sqlite as sqlx::migrate::MigrateDatabase>::drop_database(url)
                .await
                .log_err();
        })
    }

    pub async fn fuzzy_search_users(&self, name_query: &str, limit: u32) -> Result<Vec<User>> {
        test_support!(self, {
            let like_string = Self::fuzzy_like_string(name_query);
            let query = "
                SELECT users.*
                FROM users
                WHERE github_login ILIKE $1
                ORDER BY github_login <-> $2
                LIMIT $3
            ";
            Ok(sqlx::query_as(query)
                .bind(like_string)
                .bind(name_query)
                .bind(limit as i32)
                .fetch_all(&self.pool)
                .await?)
        })
    }

    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>> {
        test_support!(self, {
            let query = "
                SELECT users.*
                FROM users
                WHERE users.id = ANY ($1)
            ";
            Ok(sqlx::query_as(query)
                .bind(&ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .fetch_all(&self.pool)
                .await?)
        })
    }

    pub async fn get_user_metrics_id(&self, id: UserId) -> Result<String> {
        test_support!(self, {
            let query = "
                SELECT metrics_id::text
                FROM users
                WHERE id = $1
            ";
            Ok(sqlx::query_scalar(query)
                .bind(id)
                .fetch_one(&self.pool)
                .await?)
        })
    }

    pub async fn create_user(
        &self,
        email_address: &str,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        test_support!(self, {
            let query = "
                INSERT INTO users (email_address, github_login, github_user_id, admin)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
                RETURNING id, metrics_id::text
            ";

            let (user_id, metrics_id): (UserId, String) = sqlx::query_as(query)
                .bind(email_address)
                .bind(params.github_login)
                .bind(params.github_user_id)
                .bind(admin)
                .fetch_one(&self.pool)
                .await?;
            Ok(NewUserResult {
                user_id,
                metrics_id,
                signup_device_id: None,
                inviting_user_id: None,
            })
        })
    }

    pub async fn create_user_from_invite(
        &self,
        invite: &Invite,
        user: NewUserParams,
    ) -> Result<Option<NewUserResult>> {
        test_support!(self, {
            let mut tx = self.pool.begin().await?;

            let (signup_id, existing_user_id, inviting_user_id, signup_device_id): (
                i32,
                Option<UserId>,
                Option<UserId>,
                Option<String>,
            ) = sqlx::query_as(
                "
                SELECT id, user_id, inviting_user_id, device_id
                FROM signups
                WHERE
                    email_address = $1 AND
                    email_confirmation_code = $2
                ",
            )
            .bind(&invite.email_address)
            .bind(&invite.email_confirmation_code)
            .fetch_optional(&mut tx)
            .await?
            .ok_or_else(|| Error::Http(StatusCode::NOT_FOUND, "no such invite".to_string()))?;

            if existing_user_id.is_some() {
                return Ok(None);
            }

            let (user_id, metrics_id): (UserId, String) = sqlx::query_as(
                "
                INSERT INTO users
                (email_address, github_login, github_user_id, admin, invite_count, invite_code)
                VALUES
                ($1, $2, $3, FALSE, $4, $5)
                ON CONFLICT (github_login) DO UPDATE SET
                    email_address = excluded.email_address,
                    github_user_id = excluded.github_user_id,
                    admin = excluded.admin
                RETURNING id, metrics_id::text
                ",
            )
            .bind(&invite.email_address)
            .bind(&user.github_login)
            .bind(&user.github_user_id)
            .bind(&user.invite_count)
            .bind(random_invite_code())
            .fetch_one(&mut tx)
            .await?;

            sqlx::query(
                "
                UPDATE signups
                SET user_id = $1
                WHERE id = $2
                ",
            )
            .bind(&user_id)
            .bind(&signup_id)
            .execute(&mut tx)
            .await?;

            if let Some(inviting_user_id) = inviting_user_id {
                let (user_id_a, user_id_b, a_to_b) = if inviting_user_id < user_id {
                    (inviting_user_id, user_id, true)
                } else {
                    (user_id, inviting_user_id, false)
                };

                sqlx::query(
                    "
                    INSERT INTO contacts
                        (user_id_a, user_id_b, a_to_b, should_notify, accepted)
                    VALUES
                        ($1, $2, $3, TRUE, TRUE)
                    ON CONFLICT DO NOTHING
                    ",
                )
                .bind(user_id_a)
                .bind(user_id_b)
                .bind(a_to_b)
                .execute(&mut tx)
                .await?;
            }

            tx.commit().await?;
            Ok(Some(NewUserResult {
                user_id,
                metrics_id,
                inviting_user_id,
                signup_device_id,
            }))
        })
    }

    pub async fn create_signup(&self, signup: &Signup) -> Result<()> {
        test_support!(self, {
            sqlx::query(
                "
                INSERT INTO signups
                (
                    email_address,
                    email_confirmation_code,
                    email_confirmation_sent,
                    platform_linux,
                    platform_mac,
                    platform_windows,
                    platform_unknown,
                    editor_features,
                    programming_languages,
                    device_id,
                    added_to_mailing_list
                )
                VALUES
                    ($1, $2, FALSE, $3, $4, $5, FALSE, $6, $7, $8, $9)
                ON CONFLICT (email_address) DO UPDATE SET
                    email_address = excluded.email_address
                RETURNING id
                ",
            )
            .bind(&signup.email_address)
            .bind(&random_email_confirmation_code())
            .bind(&signup.platform_linux)
            .bind(&signup.platform_mac)
            .bind(&signup.platform_windows)
            .bind(&signup.editor_features)
            .bind(&signup.programming_languages)
            .bind(&signup.device_id)
            .bind(&signup.added_to_mailing_list)
            .execute(&self.pool)
            .await?;
            Ok(())
        })
    }

    pub async fn create_invite_from_code(
        &self,
        code: &str,
        email_address: &str,
        device_id: Option<&str>,
    ) -> Result<Invite> {
        test_support!(self, {
            let mut tx = self.pool.begin().await?;

            let existing_user: Option<UserId> = sqlx::query_scalar(
                "
                SELECT id
                FROM users
                WHERE email_address = $1
                ",
            )
            .bind(email_address)
            .fetch_optional(&mut tx)
            .await?;
            if existing_user.is_some() {
                Err(anyhow!("email address is already in use"))?;
            }

            let inviting_user_id_with_invites: Option<UserId> = sqlx::query_scalar(
                "
                UPDATE users
                SET invite_count = invite_count - 1
                WHERE invite_code = $1 AND invite_count > 0
                RETURNING id
                ",
            )
            .bind(code)
            .fetch_optional(&mut tx)
            .await?;

            let Some(inviter_id) = inviting_user_id_with_invites else {
                return Err(Error::Http(
                    StatusCode::UNAUTHORIZED,
                    "unable to find an invite code with invites remaining".to_string(),
                ));
            };

            let email_confirmation_code: String = sqlx::query_scalar(
                "
                INSERT INTO signups
                (
                    email_address,
                    email_confirmation_code,
                    email_confirmation_sent,
                    inviting_user_id,
                    platform_linux,
                    platform_mac,
                    platform_windows,
                    platform_unknown,
                    device_id
                )
                VALUES
                    ($1, $2, FALSE, $3, FALSE, FALSE, FALSE, TRUE, $4)
                ON CONFLICT (email_address)
                DO UPDATE SET
                    inviting_user_id = excluded.inviting_user_id
                RETURNING email_confirmation_code
                ",
            )
            .bind(&email_address)
            .bind(&random_email_confirmation_code())
            .bind(&inviter_id)
            .bind(&device_id)
            .fetch_one(&mut tx)
            .await?;

            tx.commit().await?;

            Ok(Invite {
                email_address: email_address.into(),
                email_confirmation_code,
            })
        })
    }

    pub async fn record_sent_invites(&self, invites: &[Invite]) -> Result<()> {
        test_support!(self, {
            let emails = invites
                .iter()
                .map(|s| s.email_address.as_str())
                .collect::<Vec<_>>();
            sqlx::query(
                "
                UPDATE signups
                SET email_confirmation_sent = TRUE
                WHERE email_address = ANY ($1)
                ",
            )
            .bind(&emails)
            .execute(&self.pool)
            .await?;
            Ok(())
        })
    }
}

impl<D> Db<D>
where
    D: sqlx::Database + sqlx::migrate::MigrateDatabase,
    D::Connection: sqlx::migrate::Migrate,
    for<'a> <D as sqlx::database::HasArguments<'a>>::Arguments: sqlx::IntoArguments<'a, D>,
    for<'a> &'a mut D::Connection: sqlx::Executor<'a, Database = D>,
    for<'a, 'b> &'b mut sqlx::Transaction<'a, D>: sqlx::Executor<'b, Database = D>,
    D::QueryResult: RowsAffected,
    String: sqlx::Type<D>,
    i32: sqlx::Type<D>,
    i64: sqlx::Type<D>,
    bool: sqlx::Type<D>,
    str: sqlx::Type<D>,
    Uuid: sqlx::Type<D>,
    sqlx::types::Json<serde_json::Value>: sqlx::Type<D>,
    OffsetDateTime: sqlx::Type<D>,
    PrimitiveDateTime: sqlx::Type<D>,
    usize: sqlx::ColumnIndex<D::Row>,
    for<'a> &'a str: sqlx::ColumnIndex<D::Row>,
    for<'a> &'a str: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> String: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> Option<String>: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> Option<&'a str>: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> i32: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> i64: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> bool: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> Uuid: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> sqlx::types::JsonValue: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> OffsetDateTime: sqlx::Encode<'a, D> + sqlx::Decode<'a, D>,
    for<'a> PrimitiveDateTime: sqlx::Decode<'a, D> + sqlx::Decode<'a, D>,
{
    pub async fn migrate(
        &self,
        migrations_path: &Path,
        ignore_checksum_mismatch: bool,
    ) -> anyhow::Result<Vec<(Migration, Duration)>> {
        let migrations = MigrationSource::resolve(migrations_path)
            .await
            .map_err(|err| anyhow!("failed to load migrations: {err:?}"))?;

        let mut conn = self.pool.acquire().await?;

        conn.ensure_migrations_table().await?;
        let applied_migrations: HashMap<_, _> = conn
            .list_applied_migrations()
            .await?
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        let mut new_migrations = Vec::new();
        for migration in migrations {
            match applied_migrations.get(&migration.version) {
                Some(applied_migration) => {
                    if migration.checksum != applied_migration.checksum && !ignore_checksum_mismatch
                    {
                        Err(anyhow!(
                            "checksum mismatch for applied migration {}",
                            migration.description
                        ))?;
                    }
                }
                None => {
                    let elapsed = conn.apply(&migration).await?;
                    new_migrations.push((migration, elapsed));
                }
            }
        }

        Ok(new_migrations)
    }

    pub fn fuzzy_like_string(string: &str) -> String {
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

    // users

    pub async fn get_all_users(&self, page: u32, limit: u32) -> Result<Vec<User>> {
        test_support!(self, {
            let query = "SELECT * FROM users ORDER BY github_login ASC LIMIT $1 OFFSET $2";
            Ok(sqlx::query_as(query)
                .bind(limit as i32)
                .bind((page * limit) as i32)
                .fetch_all(&self.pool)
                .await?)
        })
    }

    pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>> {
        test_support!(self, {
            let query = "
                SELECT users.*
                FROM users
                WHERE id = $1
                LIMIT 1
            ";
            Ok(sqlx::query_as(query)
                .bind(&id)
                .fetch_optional(&self.pool)
                .await?)
        })
    }

    pub async fn get_users_with_no_invites(
        &self,
        invited_by_another_user: bool,
    ) -> Result<Vec<User>> {
        test_support!(self, {
            let query = format!(
                "
                SELECT users.*
                FROM users
                WHERE invite_count = 0
                AND inviter_id IS{} NULL
                ",
                if invited_by_another_user { " NOT" } else { "" }
            );

            Ok(sqlx::query_as(&query).fetch_all(&self.pool).await?)
        })
    }

    pub async fn get_user_by_github_account(
        &self,
        github_login: &str,
        github_user_id: Option<i32>,
    ) -> Result<Option<User>> {
        test_support!(self, {
            if let Some(github_user_id) = github_user_id {
                let mut user = sqlx::query_as::<_, User>(
                    "
                    UPDATE users
                    SET github_login = $1
                    WHERE github_user_id = $2
                    RETURNING *
                    ",
                )
                .bind(github_login)
                .bind(github_user_id)
                .fetch_optional(&self.pool)
                .await?;

                if user.is_none() {
                    user = sqlx::query_as::<_, User>(
                        "
                        UPDATE users
                        SET github_user_id = $1
                        WHERE github_login = $2
                        RETURNING *
                        ",
                    )
                    .bind(github_user_id)
                    .bind(github_login)
                    .fetch_optional(&self.pool)
                    .await?;
                }

                Ok(user)
            } else {
                let user = sqlx::query_as(
                    "
                    SELECT * FROM users
                    WHERE github_login = $1
                    LIMIT 1
                    ",
                )
                .bind(github_login)
                .fetch_optional(&self.pool)
                .await?;
                Ok(user)
            }
        })
    }

    pub async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        test_support!(self, {
            let query = "UPDATE users SET admin = $1 WHERE id = $2";
            Ok(sqlx::query(query)
                .bind(is_admin)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?)
        })
    }

    pub async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()> {
        test_support!(self, {
            let query = "UPDATE users SET connected_once = $1 WHERE id = $2";
            Ok(sqlx::query(query)
                .bind(connected_once)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?)
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
            Ok(sqlx::query(query)
                .bind(id.0)
                .execute(&self.pool)
                .await
                .map(drop)?)
        })
    }

    // signups

    pub async fn get_waitlist_summary(&self) -> Result<WaitlistSummary> {
        test_support!(self, {
            Ok(sqlx::query_as(
                "
                SELECT
                    COUNT(*) as count,
                    COALESCE(SUM(CASE WHEN platform_linux THEN 1 ELSE 0 END), 0) as linux_count,
                    COALESCE(SUM(CASE WHEN platform_mac THEN 1 ELSE 0 END), 0) as mac_count,
                    COALESCE(SUM(CASE WHEN platform_windows THEN 1 ELSE 0 END), 0) as windows_count,
                    COALESCE(SUM(CASE WHEN platform_unknown THEN 1 ELSE 0 END), 0) as unknown_count
                FROM (
                    SELECT *
                    FROM signups
                    WHERE
                        NOT email_confirmation_sent
                ) AS unsent
                ",
            )
            .fetch_one(&self.pool)
            .await?)
        })
    }

    pub async fn get_unsent_invites(&self, count: usize) -> Result<Vec<Invite>> {
        test_support!(self, {
            Ok(sqlx::query_as(
                "
                SELECT
                    email_address, email_confirmation_code
                FROM signups
                WHERE
                    NOT email_confirmation_sent AND
                    (platform_mac OR platform_unknown)
                ORDER BY
                    created_at
                LIMIT $1
                ",
            )
            .bind(count as i32)
            .fetch_all(&self.pool)
            .await?)
        })
    }

    // invite codes

    pub async fn set_invite_count_for_user(&self, id: UserId, count: u32) -> Result<()> {
        test_support!(self, {
            let mut tx = self.pool.begin().await?;
            if count > 0 {
                sqlx::query(
                    "
                    UPDATE users
                    SET invite_code = $1
                    WHERE id = $2 AND invite_code IS NULL
                ",
                )
                .bind(random_invite_code())
                .bind(id)
                .execute(&mut tx)
                .await?;
            }

            sqlx::query(
                "
                UPDATE users
                SET invite_count = $1
                WHERE id = $2
                ",
            )
            .bind(count as i32)
            .bind(id)
            .execute(&mut tx)
            .await?;
            tx.commit().await?;
            Ok(())
        })
    }

    pub async fn get_invite_code_for_user(&self, id: UserId) -> Result<Option<(String, u32)>> {
        test_support!(self, {
            let result: Option<(String, i32)> = sqlx::query_as(
                "
                    SELECT invite_code, invite_count
                    FROM users
                    WHERE id = $1 AND invite_code IS NOT NULL 
                ",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
            if let Some((code, count)) = result {
                Ok(Some((code, count.try_into().map_err(anyhow::Error::new)?)))
            } else {
                Ok(None)
            }
        })
    }

    pub async fn get_user_for_invite_code(&self, code: &str) -> Result<User> {
        test_support!(self, {
            sqlx::query_as(
                "
                    SELECT *
                    FROM users
                    WHERE invite_code = $1
                ",
            )
            .bind(code)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| {
                Error::Http(
                    StatusCode::NOT_FOUND,
                    "that invite code does not exist".to_string(),
                )
            })
        })
    }

    // projects

    /// Registers a new project for the given user.
    pub async fn register_project(&self, host_user_id: UserId) -> Result<ProjectId> {
        test_support!(self, {
            Ok(sqlx::query_scalar(
                "
                INSERT INTO projects(host_user_id)
                VALUES ($1)
                RETURNING id
                ",
            )
            .bind(host_user_id)
            .fetch_one(&self.pool)
            .await
            .map(ProjectId)?)
        })
    }

    /// Unregisters a project for the given project id.
    pub async fn unregister_project(&self, project_id: ProjectId) -> Result<()> {
        test_support!(self, {
            sqlx::query(
                "
                UPDATE projects
                SET unregistered = TRUE
                WHERE id = $1
                ",
            )
            .bind(project_id)
            .execute(&self.pool)
            .await?;
            Ok(())
        })
    }

    // contacts

    pub async fn get_contacts(&self, user_id: UserId) -> Result<Vec<Contact>> {
        test_support!(self, {
            let query = "
                SELECT user_id_a, user_id_b, a_to_b, accepted, should_notify
                FROM contacts
                WHERE user_id_a = $1 OR user_id_b = $1;
            ";

            let mut rows = sqlx::query_as::<_, (UserId, UserId, bool, bool, bool)>(query)
                .bind(user_id)
                .fetch(&self.pool);

            let mut contacts = Vec::new();
            while let Some(row) = rows.next().await {
                let (user_id_a, user_id_b, a_to_b, accepted, should_notify) = row?;

                if user_id_a == user_id {
                    if accepted {
                        contacts.push(Contact::Accepted {
                            user_id: user_id_b,
                            should_notify: should_notify && a_to_b,
                        });
                    } else if a_to_b {
                        contacts.push(Contact::Outgoing { user_id: user_id_b })
                    } else {
                        contacts.push(Contact::Incoming {
                            user_id: user_id_b,
                            should_notify,
                        });
                    }
                } else if accepted {
                    contacts.push(Contact::Accepted {
                        user_id: user_id_a,
                        should_notify: should_notify && !a_to_b,
                    });
                } else if a_to_b {
                    contacts.push(Contact::Incoming {
                        user_id: user_id_a,
                        should_notify,
                    });
                } else {
                    contacts.push(Contact::Outgoing { user_id: user_id_a });
                }
            }

            contacts.sort_unstable_by_key(|contact| contact.user_id());

            Ok(contacts)
        })
    }

    pub async fn has_contact(&self, user_id_1: UserId, user_id_2: UserId) -> Result<bool> {
        test_support!(self, {
            let (id_a, id_b) = if user_id_1 < user_id_2 {
                (user_id_1, user_id_2)
            } else {
                (user_id_2, user_id_1)
            };

            let query = "
                SELECT 1 FROM contacts
                WHERE user_id_a = $1 AND user_id_b = $2 AND accepted = TRUE
                LIMIT 1
            ";
            Ok(sqlx::query_scalar::<_, i32>(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .fetch_optional(&self.pool)
                .await?
                .is_some())
        })
    }

    pub async fn send_contact_request(&self, sender_id: UserId, receiver_id: UserId) -> Result<()> {
        test_support!(self, {
            let (id_a, id_b, a_to_b) = if sender_id < receiver_id {
                (sender_id, receiver_id, true)
            } else {
                (receiver_id, sender_id, false)
            };
            let query = "
                INSERT into contacts (user_id_a, user_id_b, a_to_b, accepted, should_notify)
                VALUES ($1, $2, $3, FALSE, TRUE)
                ON CONFLICT (user_id_a, user_id_b) DO UPDATE
                SET
                    accepted = TRUE,
                    should_notify = FALSE
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
                Err(anyhow!("contact already requested"))?
            }
        })
    }

    pub async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<()> {
        test_support!(self, {
            let (id_a, id_b) = if responder_id < requester_id {
                (responder_id, requester_id)
            } else {
                (requester_id, responder_id)
            };
            let query = "
                DELETE FROM contacts
                WHERE user_id_a = $1 AND user_id_b = $2;
            ";
            let result = sqlx::query(query)
                .bind(id_a.0)
                .bind(id_b.0)
                .execute(&self.pool)
                .await?;

            if result.rows_affected() == 1 {
                Ok(())
            } else {
                Err(anyhow!("no such contact"))?
            }
        })
    }

    pub async fn dismiss_contact_notification(
        &self,
        user_id: UserId,
        contact_user_id: UserId,
    ) -> Result<()> {
        test_support!(self, {
            let (id_a, id_b, a_to_b) = if user_id < contact_user_id {
                (user_id, contact_user_id, true)
            } else {
                (contact_user_id, user_id, false)
            };

            let query = "
                UPDATE contacts
                SET should_notify = FALSE
                WHERE
                    user_id_a = $1 AND user_id_b = $2 AND
                    (
                        (a_to_b = $3 AND accepted) OR
                        (a_to_b != $3 AND NOT accepted)
                    );
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
        })
    }

    pub async fn respond_to_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> Result<()> {
        test_support!(self, {
            let (id_a, id_b, a_to_b) = if responder_id < requester_id {
                (responder_id, requester_id, false)
            } else {
                (requester_id, responder_id, true)
            };
            let result = if accept {
                let query = "
                    UPDATE contacts
                    SET accepted = TRUE, should_notify = TRUE
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
                Err(anyhow!("no such contact request"))?
            }
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
                    LIMIT 10000
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
                .bind(max_access_token_count as i32)
                .execute(&mut tx)
                .await?;
            Ok(tx.commit().await?)
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
            Ok(sqlx::query_scalar(query)
                .bind(user_id.0)
                .fetch_all(&self.pool)
                .await?)
        })
    }
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            sqlx::Type,
            Serialize,
            Deserialize,
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
            pub fn to_proto(self) -> u64 {
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
#[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub github_user_id: Option<i32>,
    pub email_address: Option<String>,
    pub admin: bool,
    pub invite_code: Option<String>,
    pub invite_count: i32,
    pub connected_once: bool,
}

id_type!(ProjectId);
#[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
pub struct Project {
    pub id: ProjectId,
    pub host_user_id: UserId,
    pub unregistered: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contact {
    Accepted {
        user_id: UserId,
        should_notify: bool,
    },
    Outgoing {
        user_id: UserId,
    },
    Incoming {
        user_id: UserId,
        should_notify: bool,
    },
}

impl Contact {
    pub fn user_id(&self) -> UserId {
        match self {
            Contact::Accepted { user_id, .. } => *user_id,
            Contact::Outgoing { user_id } => *user_id,
            Contact::Incoming { user_id, .. } => *user_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncomingContactRequest {
    pub requester_id: UserId,
    pub should_notify: bool,
}

#[derive(Clone, Deserialize, Default)]
pub struct Signup {
    pub email_address: String,
    pub platform_mac: bool,
    pub platform_windows: bool,
    pub platform_linux: bool,
    pub editor_features: Vec<String>,
    pub programming_languages: Vec<String>,
    pub device_id: Option<String>,
    pub added_to_mailing_list: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromRow)]
pub struct WaitlistSummary {
    #[sqlx(default)]
    pub count: i64,
    #[sqlx(default)]
    pub linux_count: i64,
    #[sqlx(default)]
    pub mac_count: i64,
    #[sqlx(default)]
    pub windows_count: i64,
    #[sqlx(default)]
    pub unknown_count: i64,
}

#[derive(Clone, FromRow, PartialEq, Debug, Serialize, Deserialize)]
pub struct Invite {
    pub email_address: String,
    pub email_confirmation_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserParams {
    pub github_login: String,
    pub github_user_id: i32,
    pub invite_count: i32,
}

#[derive(Debug)]
pub struct NewUserResult {
    pub user_id: UserId,
    pub metrics_id: String,
    pub inviting_user_id: Option<UserId>,
    pub signup_device_id: Option<String>,
}

fn random_invite_code() -> String {
    nanoid::nanoid!(16)
}

fn random_email_confirmation_code() -> String {
    nanoid::nanoid!(64)
}

#[cfg(test)]
pub use test::*;

#[cfg(test)]
mod test {
    use super::*;
    use gpui::executor::Background;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use rand::prelude::*;
    use sqlx::migrate::MigrateDatabase;
    use std::sync::Arc;

    pub struct SqliteTestDb {
        pub db: Option<Arc<Db<sqlx::Sqlite>>>,
        pub conn: sqlx::sqlite::SqliteConnection,
    }

    pub struct PostgresTestDb {
        pub db: Option<Arc<Db<sqlx::Postgres>>>,
        pub url: String,
    }

    impl SqliteTestDb {
        pub fn new(background: Arc<Background>) -> Self {
            let mut rng = StdRng::from_entropy();
            let url = format!("file:zed-test-{}?mode=memory", rng.gen::<u128>());
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();

            let (mut db, conn) = runtime.block_on(async {
                let db = Db::<sqlx::Sqlite>::new(&url, 5).await.unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations.sqlite");
                db.migrate(migrations_path.as_ref(), false).await.unwrap();
                let conn = db.pool.acquire().await.unwrap().detach();
                (db, conn)
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
                conn,
            }
        }

        pub fn db(&self) -> &Arc<Db<sqlx::Sqlite>> {
            self.db.as_ref().unwrap()
        }
    }

    impl PostgresTestDb {
        pub fn new(background: Arc<Background>) -> Self {
            lazy_static! {
                static ref LOCK: Mutex<()> = Mutex::new(());
            }

            let _guard = LOCK.lock();
            let mut rng = StdRng::from_entropy();
            let url = format!(
                "postgres://postgres@localhost/zed-test-{}",
                rng.gen::<u128>()
            );
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();

            let mut db = runtime.block_on(async {
                sqlx::Postgres::create_database(&url)
                    .await
                    .expect("failed to create test db");
                let db = Db::<sqlx::Postgres>::new(&url, 5).await.unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
                db.migrate(Path::new(migrations_path), false).await.unwrap();
                db
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
                url,
            }
        }

        pub fn db(&self) -> &Arc<Db<sqlx::Postgres>> {
            self.db.as_ref().unwrap()
        }
    }

    impl Drop for PostgresTestDb {
        fn drop(&mut self) {
            let db = self.db.take().unwrap();
            db.teardown(&self.url);
        }
    }
}
