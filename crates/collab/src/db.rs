use crate::{Error, Result};
use anyhow::anyhow;
use axum::http::StatusCode;
use collections::HashMap;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::{Migrate as _, Migration, MigrationSource},
    types::Uuid,
    FromRow, QueryBuilder,
};
use std::{cmp, ops::Range, path::Path, time::Duration};
use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(test)]
pub type DefaultDb = Db<sqlx::Sqlite>;

#[cfg(not(test))]
pub type DefaultDb = Db<sqlx::Postgres>;

pub struct Db<D: sqlx::Database> {
    pool: sqlx::Pool<D>,
    #[cfg(test)]
    background: Option<std::sync::Arc<gpui::executor::Background>>,
}

macro_rules! test_support {
    ($self:ident, { $($token:tt)* }) => {{
        let body = async {
            $($token)*
        };

        if cfg!(test) {
            #[cfg(test)]
            if let Some(background) = $self.background.as_ref() {
                background.simulate_random_delay().await;
            }
            tokio::runtime::Builder::new_current_thread().enable_io().enable_time().build().unwrap().block_on(body)
        } else {
            body.await
        }
    }};
}

pub trait RowsAffected {
    fn rows_affected(&self) -> u64;
}

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

impl Db<sqlx::Sqlite> {
    #[cfg(test)]
    pub async fn new(url: &str, max_connections: u32) -> Result<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;
        Ok(Self {
            pool,
            background: None,
        })
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
                -- ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
                RETURNING id, 'the-metrics-id'
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

    pub async fn create_signup(&self, signup: Signup) -> Result<()> {
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
                    device_id
                )
                VALUES
                    ($1, $2, FALSE, $3, $4, $5, FALSE, $6)
                RETURNING id
                ",
            )
            .bind(&signup.email_address)
            .bind(&random_email_confirmation_code())
            .bind(&signup.platform_linux)
            .bind(&signup.platform_mac)
            .bind(&signup.platform_windows)
            // .bind(&signup.editor_features)
            // .bind(&signup.programming_languages)
            .bind(&signup.device_id)
            .execute(&self.pool)
            .await?;
            Ok(())
        })
    }

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
                LIMIT $1
                ",
            )
            .bind(count as i32)
            .fetch_all(&self.pool)
            .await?)
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
                WHERE email_address IN (SELECT value from json_each($1))
                ",
            )
            .bind(&serde_json::json!(emails))
            .execute(&self.pool)
            .await?;
            Ok(())
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
                let id: Option<UserId> = sqlx::query_scalar(
                    "
                    UPDATE users
                    SET invite_count = invite_count - 1
                    WHERE id = $1 AND invite_count > 0
                    RETURNING id
                    ",
                )
                .bind(&inviting_user_id)
                .fetch_optional(&mut tx)
                .await?;

                if id.is_none() {
                    Err(Error::Http(
                        StatusCode::UNAUTHORIZED,
                        "no invites remaining".to_string(),
                    ))?;
                }

                sqlx::query(
                    "
                    INSERT INTO contacts
                        (user_id_a, user_id_b, a_to_b, should_notify, accepted)
                    VALUES
                        ($1, $2, TRUE, TRUE, TRUE)
                    ON CONFLICT DO NOTHING
                    ",
                )
                .bind(inviting_user_id)
                .bind(user_id)
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

            let row: Option<(UserId, i32)> = sqlx::query_as(
                "
                SELECT id, invite_count
                FROM users
                WHERE invite_code = $1
                ",
            )
            .bind(code)
            .fetch_optional(&mut tx)
            .await?;

            let (inviter_id, invite_count) = match row {
                Some(row) => row,
                None => Err(Error::Http(
                    StatusCode::NOT_FOUND,
                    "invite code not found".to_string(),
                ))?,
            };

            if invite_count == 0 {
                Err(Error::Http(
                    StatusCode::UNAUTHORIZED,
                    "no invites remaining".to_string(),
                ))?;
            }

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

    /// Update file counts by extension for the given project and worktree.
    pub async fn update_worktree_extensions(
        &self,
        project_id: ProjectId,
        worktree_id: u64,
        extensions: HashMap<String, u32>,
    ) -> Result<()> {
        test_support!(self, {
            if extensions.is_empty() {
                return Ok(());
            }

            let mut query = QueryBuilder::new(
                "INSERT INTO worktree_extensions (project_id, worktree_id, extension, count)",
            );
            query.push_values(extensions, |mut query, (extension, count)| {
                query
                    .push_bind(project_id)
                    .push_bind(worktree_id as i32)
                    .push_bind(extension)
                    .push_bind(count as i32);
            });
            query.push(
                "
                ON CONFLICT (project_id, worktree_id, extension) DO UPDATE SET
                count = excluded.count
                ",
            );
            // query.build().execute(&self.pool).await?;

            Ok(())
        })
    }

    /// Get the file counts on the given project keyed by their worktree and extension.
    pub async fn get_project_extensions(
        &self,
        project_id: ProjectId,
    ) -> Result<HashMap<u64, HashMap<String, usize>>> {
        test_support!(self, {
            #[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
            struct WorktreeExtension {
                worktree_id: i32,
                extension: String,
                count: i32,
            }

            let query = "
                SELECT worktree_id, extension, count
                FROM worktree_extensions
                WHERE project_id = $1
            ";
            let counts = sqlx::query_as::<_, WorktreeExtension>(query)
                .bind(&project_id)
                .fetch_all(&self.pool)
                .await?;

            let mut extension_counts = HashMap::default();
            for count in counts {
                extension_counts
                    .entry(count.worktree_id as u64)
                    .or_insert_with(HashMap::default)
                    .insert(count.extension, count.count as usize);
            }
            Ok(extension_counts)
        })
    }

    /// Record which users have been active in which projects during
    /// a given period of time.
    pub async fn record_user_activity(
        &self,
        time_period: Range<OffsetDateTime>,
        projects: &[(UserId, ProjectId)],
    ) -> Result<()> {
        test_support!(self, {
            let query = "
                INSERT INTO project_activity_periods
                (ended_at, duration_millis, user_id, project_id)
                VALUES
                ($1, $2, $3, $4);
            ";

            let mut tx = self.pool.begin().await?;
            let duration_millis =
                ((time_period.end - time_period.start).as_seconds_f64() * 1000.0) as i32;
            for (user_id, project_id) in projects {
                sqlx::query(query)
                    .bind(time_period.end)
                    .bind(duration_millis)
                    .bind(user_id)
                    .bind(project_id)
                    .execute(&mut tx)
                    .await?;
            }
            tx.commit().await?;

            Ok(())
        })
    }

    /// Get the number of users who have been active in the given
    /// time period for at least the given time duration.
    pub async fn get_active_user_count(
        &self,
        time_period: Range<OffsetDateTime>,
        min_duration: Duration,
        only_collaborative: bool,
    ) -> Result<usize> {
        test_support!(self, {
            let mut with_clause = String::new();
            with_clause.push_str("WITH\n");
            with_clause.push_str(
                "
                project_durations AS (
                    SELECT user_id, project_id, SUM(duration_millis) AS project_duration
                    FROM project_activity_periods
                    WHERE $1 < ended_at AND ended_at <= $2
                    GROUP BY user_id, project_id
                ),
                ",
            );
            with_clause.push_str(
                "
                project_collaborators as (
                    SELECT project_id, COUNT(DISTINCT user_id) as max_collaborators
                    FROM project_durations
                    GROUP BY project_id
                ),
                ",
            );

            if only_collaborative {
                with_clause.push_str(
                    "
                    user_durations AS (
                        SELECT user_id, SUM(project_duration) as total_duration
                        FROM project_durations, project_collaborators
                        WHERE
                            project_durations.project_id = project_collaborators.project_id AND
                            max_collaborators > 1
                        GROUP BY user_id
                        ORDER BY total_duration DESC
                        LIMIT $3
                    )
                    ",
                );
            } else {
                with_clause.push_str(
                    "
                    user_durations AS (
                        SELECT user_id, SUM(project_duration) as total_duration
                        FROM project_durations
                        GROUP BY user_id
                        ORDER BY total_duration DESC
                        LIMIT $3
                    )
                    ",
                );
            }

            let query = format!(
                "
                {with_clause}
                SELECT count(user_durations.user_id)
                FROM user_durations
                WHERE user_durations.total_duration >= $3
                "
            );

            let count: i64 = sqlx::query_scalar(&query)
                .bind(time_period.start)
                .bind(time_period.end)
                .bind(min_duration.as_millis() as i64)
                .fetch_one(&self.pool)
                .await?;
            Ok(count as usize)
        })
    }

    /// Get the users that have been most active during the given time period,
    /// along with the amount of time they have been active in each project.
    pub async fn get_top_users_activity_summary(
        &self,
        time_period: Range<OffsetDateTime>,
        max_user_count: usize,
    ) -> Result<Vec<UserActivitySummary>> {
        test_support!(self, {
            let query = "
                WITH
                    project_durations AS (
                        SELECT user_id, project_id, SUM(duration_millis) AS project_duration
                        FROM project_activity_periods
                        WHERE $1 < ended_at AND ended_at <= $2
                        GROUP BY user_id, project_id
                    ),
                    user_durations AS (
                        SELECT user_id, SUM(project_duration) as total_duration
                        FROM project_durations
                        GROUP BY user_id
                        ORDER BY total_duration DESC
                        LIMIT $3
                    ),
                    project_collaborators as (
                        SELECT project_id, COUNT(DISTINCT user_id) as max_collaborators
                        FROM project_durations
                        GROUP BY project_id
                    )
                SELECT user_durations.user_id, users.github_login, project_durations.project_id, project_duration, max_collaborators
                FROM user_durations, project_durations, project_collaborators, users
                WHERE
                    user_durations.user_id = project_durations.user_id AND
                    user_durations.user_id = users.id AND
                    project_durations.project_id = project_collaborators.project_id
                ORDER BY total_duration DESC, user_id ASC, project_id ASC
            ";

            let mut rows = sqlx::query_as::<_, (UserId, String, ProjectId, i64, i64)>(query)
                .bind(time_period.start)
                .bind(time_period.end)
                .bind(max_user_count as i32)
                .fetch(&self.pool);

            let mut result = Vec::<UserActivitySummary>::new();
            while let Some(row) = rows.next().await {
                let (user_id, github_login, project_id, duration_millis, project_collaborators) =
                    row?;
                let project_id = project_id;
                let duration = Duration::from_millis(duration_millis as u64);
                let project_activity = ProjectActivitySummary {
                    id: project_id,
                    duration,
                    max_collaborators: project_collaborators as usize,
                };
                if let Some(last_summary) = result.last_mut() {
                    if last_summary.id == user_id {
                        last_summary.project_activity.push(project_activity);
                        continue;
                    }
                }
                result.push(UserActivitySummary {
                    id: user_id,
                    project_activity: vec![project_activity],
                    github_login,
                });
            }

            Ok(result)
        })
    }

    /// Get the project activity for the given user and time period.
    pub async fn get_user_activity_timeline(
        &self,
        time_period: Range<OffsetDateTime>,
        user_id: UserId,
    ) -> Result<Vec<UserActivityPeriod>> {
        test_support!(self, {
            const COALESCE_THRESHOLD: Duration = Duration::from_secs(30);

            let query = "
                SELECT
                    project_activity_periods.ended_at,
                    project_activity_periods.duration_millis,
                    project_activity_periods.project_id,
                    worktree_extensions.extension,
                    worktree_extensions.count
                FROM project_activity_periods
                LEFT OUTER JOIN
                    worktree_extensions
                ON
                    project_activity_periods.project_id = worktree_extensions.project_id
                WHERE
                    project_activity_periods.user_id = $1 AND
                    $2 < project_activity_periods.ended_at AND
                    project_activity_periods.ended_at <= $3
                ORDER BY project_activity_periods.id ASC
            ";

            let mut rows = sqlx::query_as::<
                _,
                (
                    PrimitiveDateTime,
                    i32,
                    ProjectId,
                    Option<String>,
                    Option<i32>,
                ),
            >(query)
            .bind(user_id)
            .bind(time_period.start)
            .bind(time_period.end)
            .fetch(&self.pool);

            let mut time_periods: HashMap<ProjectId, Vec<UserActivityPeriod>> = Default::default();
            while let Some(row) = rows.next().await {
                let (ended_at, duration_millis, project_id, extension, extension_count) = row?;
                let ended_at = ended_at.assume_utc();
                let duration = Duration::from_millis(duration_millis as u64);
                let started_at = ended_at - duration;
                let project_time_periods = time_periods.entry(project_id).or_default();

                if let Some(prev_duration) = project_time_periods.last_mut() {
                    if started_at <= prev_duration.end + COALESCE_THRESHOLD
                        && ended_at >= prev_duration.start
                    {
                        prev_duration.end = cmp::max(prev_duration.end, ended_at);
                    } else {
                        project_time_periods.push(UserActivityPeriod {
                            project_id,
                            start: started_at,
                            end: ended_at,
                            extensions: Default::default(),
                        });
                    }
                } else {
                    project_time_periods.push(UserActivityPeriod {
                        project_id,
                        start: started_at,
                        end: ended_at,
                        extensions: Default::default(),
                    });
                }

                if let Some((extension, extension_count)) = extension.zip(extension_count) {
                    project_time_periods
                        .last_mut()
                        .unwrap()
                        .extensions
                        .insert(extension, extension_count as usize);
                }
            }

            let mut durations = time_periods.into_values().flatten().collect::<Vec<_>>();
            durations.sort_unstable_by_key(|duration| duration.start);
            Ok(durations)
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
            Ok(sqlx::query_as(query)
                .bind(slug)
                .fetch_optional(&self.pool)
                .await?)
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
            Ok(sqlx::query_scalar(query)
                .bind(name)
                .bind(slug)
                .fetch_one(&self.pool)
                .await
                .map(OrgId)?)
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
            Ok(sqlx::query(query)
                .bind(org_id.0)
                .bind(user_id.0)
                .bind(is_admin)
                .execute(&self.pool)
                .await
                .map(drop)?)
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
            Ok(sqlx::query_scalar(query)
                .bind(org_id.0)
                .bind(name)
                .fetch_one(&self.pool)
                .await
                .map(ChannelId)?)
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
            Ok(sqlx::query_as(query)
                .bind(org_id.0)
                .fetch_all(&self.pool)
                .await?)
        })
    }

    pub async fn get_accessible_channels(&self, user_id: UserId) -> Result<Vec<Channel>> {
        test_support!(self, {
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
            Ok(sqlx::query_scalar::<_, i32>(query)
                .bind(user_id.0)
                .bind(channel_id.0)
                .fetch_optional(&self.pool)
                .await
                .map(|e| e.is_some())?)
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
            Ok(sqlx::query(query)
                .bind(channel_id.0)
                .bind(user_id.0)
                .bind(is_admin)
                .execute(&self.pool)
                .await
                .map(drop)?)
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
            Ok(sqlx::query_scalar(query)
                .bind(channel_id.0)
                .bind(sender_id.0)
                .bind(body)
                .bind(timestamp)
                .bind(Uuid::from_u128(nonce))
                .fetch_one(&self.pool)
                .await
                .map(MessageId)?)
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserActivitySummary {
    pub id: UserId,
    pub github_login: String,
    pub project_activity: Vec<ProjectActivitySummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectActivitySummary {
    pub id: ProjectId,
    pub duration: Duration,
    pub max_collaborators: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserActivityPeriod {
    pub project_id: ProjectId,
    #[serde(with = "time::serde::iso8601")]
    pub start: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end: OffsetDateTime,
    pub extensions: HashMap<String, usize>,
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

#[derive(Clone, Deserialize)]
pub struct Signup {
    pub email_address: String,
    pub platform_mac: bool,
    pub platform_windows: bool,
    pub platform_linux: bool,
    pub editor_features: Vec<String>,
    pub programming_languages: Vec<String>,
    pub device_id: Option<String>,
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

#[derive(FromRow, PartialEq, Debug, Serialize, Deserialize)]
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
    use rand::prelude::*;
    use sqlx::migrate::MigrateDatabase;
    use std::sync::Arc;

    pub struct TestDb {
        pub db: Option<Arc<DefaultDb>>,
        pub url: String,
    }

    impl TestDb {
        pub async fn new(background: Arc<Background>) -> Self {
            let mut rng = StdRng::from_entropy();
            let url = format!("/tmp/zed-test-{}", rng.gen::<u128>());
            sqlx::Sqlite::create_database(&url).await.unwrap();
            let mut db = DefaultDb::new(&url, 5).await.unwrap();
            db.background = Some(background);
            let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations.sqlite");
            db.migrate(Path::new(migrations_path), false).await.unwrap();
            Self {
                db: Some(Arc::new(db)),
                url,
            }
        }

        pub fn db(&self) -> &Arc<DefaultDb> {
            self.db.as_ref().unwrap()
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            std::fs::remove_file(&self.url).ok();
        }
    }
}
