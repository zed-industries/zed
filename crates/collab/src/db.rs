use std::{ops::Range, time::Duration};

use crate::{Error, Result};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use axum::http::StatusCode;
use collections::HashMap;
use futures::StreamExt;
use nanoid::nanoid;
use serde::Serialize;
pub use sqlx::postgres::PgPoolOptions as DbOptions;
use sqlx::{types::Uuid, FromRow, QueryBuilder, Row};
use time::OffsetDateTime;

#[async_trait]
pub trait Db: Send + Sync {
    async fn create_user(
        &self,
        github_login: &str,
        email_address: Option<&str>,
        admin: bool,
    ) -> Result<UserId>;
    async fn get_all_users(&self, page: u32, limit: u32) -> Result<Vec<User>>;
    async fn create_users(&self, users: Vec<(String, String, usize)>) -> Result<Vec<UserId>>;
    async fn fuzzy_search_users(&self, query: &str, limit: u32) -> Result<Vec<User>>;
    async fn get_user_by_id(&self, id: UserId) -> Result<Option<User>>;
    async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<User>>;
    async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>>;
    async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()>;
    async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()>;
    async fn destroy_user(&self, id: UserId) -> Result<()>;

    async fn set_invite_count(&self, id: UserId, count: u32) -> Result<()>;
    async fn get_invite_code_for_user(&self, id: UserId) -> Result<Option<(String, u32)>>;
    async fn get_user_for_invite_code(&self, code: &str) -> Result<User>;
    async fn redeem_invite_code(
        &self,
        code: &str,
        login: &str,
        email_address: Option<&str>,
    ) -> Result<UserId>;

    /// Registers a new project for the given user.
    async fn register_project(&self, host_user_id: UserId) -> Result<ProjectId>;

    /// Unregisters a project for the given project id.
    async fn unregister_project(&self, project_id: ProjectId) -> Result<()>;

    /// Create a new project for the given user.
    async fn update_worktree_extensions(
        &self,
        project_id: ProjectId,
        worktree_id: u64,
        extensions: HashMap<String, usize>,
    ) -> Result<()>;

    /// Record which users have been active in which projects during
    /// a given period of time.
    async fn record_project_activity(
        &self,
        time_period: Range<OffsetDateTime>,
        active_projects: &[(UserId, ProjectId)],
    ) -> Result<()>;

    /// Get the users that have been most active during the given time period,
    /// along with the amount of time they have been active in each project.
    async fn summarize_project_activity(
        &self,
        time_period: Range<OffsetDateTime>,
        max_user_count: usize,
    ) -> Result<Vec<UserActivitySummary>>;

    async fn get_contacts(&self, id: UserId) -> Result<Vec<Contact>>;
    async fn has_contact(&self, user_id_a: UserId, user_id_b: UserId) -> Result<bool>;
    async fn send_contact_request(&self, requester_id: UserId, responder_id: UserId) -> Result<()>;
    async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<()>;
    async fn dismiss_contact_notification(
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
    #[cfg(test)]
    fn as_fake<'a>(&'a self) -> Option<&'a tests::FakeDb>;
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

    async fn create_user(
        &self,
        github_login: &str,
        email_address: Option<&str>,
        admin: bool,
    ) -> Result<UserId> {
        let query = "
            INSERT INTO users (github_login, email_address, admin)
            VALUES ($1, $2, $3)
            ON CONFLICT (github_login) DO UPDATE SET github_login = excluded.github_login
            RETURNING id
        ";
        Ok(sqlx::query_scalar(query)
            .bind(github_login)
            .bind(email_address)
            .bind(admin)
            .fetch_one(&self.pool)
            .await
            .map(UserId)?)
    }

    async fn get_all_users(&self, page: u32, limit: u32) -> Result<Vec<User>> {
        let query = "SELECT * FROM users ORDER BY github_login ASC LIMIT $1 OFFSET $2";
        Ok(sqlx::query_as(query)
            .bind(limit)
            .bind(page * limit)
            .fetch_all(&self.pool)
            .await?)
    }

    async fn create_users(&self, users: Vec<(String, String, usize)>) -> Result<Vec<UserId>> {
        let mut query = QueryBuilder::new(
            "INSERT INTO users (github_login, email_address, admin, invite_code, invite_count)",
        );
        query.push_values(
            users,
            |mut query, (github_login, email_address, invite_count)| {
                query
                    .push_bind(github_login)
                    .push_bind(email_address)
                    .push_bind(false)
                    .push_bind(nanoid!(16))
                    .push_bind(invite_count as u32);
            },
        );
        query.push(
            "
            ON CONFLICT (github_login) DO UPDATE SET
                github_login = excluded.github_login,
                invite_count = excluded.invite_count,
                invite_code = CASE WHEN users.invite_code IS NULL
                                   THEN excluded.invite_code
                                   ELSE users.invite_code
                              END
            RETURNING id
            ",
        );

        let rows = query.build().fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.try_get::<UserId, _>(0).ok())
            .collect())
    }

    async fn fuzzy_search_users(&self, name_query: &str, limit: u32) -> Result<Vec<User>> {
        let like_string = fuzzy_like_string(name_query);
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

    async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()> {
        let query = "UPDATE users SET connected_once = $1 WHERE id = $2";
        Ok(sqlx::query(query)
            .bind(connected_once)
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

    // invite codes

    async fn set_invite_count(&self, id: UserId, count: u32) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        if count > 0 {
            sqlx::query(
                "
                UPDATE users
                SET invite_code = $1
                WHERE id = $2 AND invite_code IS NULL
            ",
            )
            .bind(nanoid!(16))
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
        .bind(count)
        .bind(id)
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn get_invite_code_for_user(&self, id: UserId) -> Result<Option<(String, u32)>> {
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
    }

    async fn get_user_for_invite_code(&self, code: &str) -> Result<User> {
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
    }

    async fn redeem_invite_code(
        &self,
        code: &str,
        login: &str,
        email_address: Option<&str>,
    ) -> Result<UserId> {
        let mut tx = self.pool.begin().await?;

        let inviter_id: Option<UserId> = sqlx::query_scalar(
            "
                UPDATE users
                SET invite_count = invite_count - 1
                WHERE
                    invite_code = $1 AND
                    invite_count > 0
                RETURNING id
            ",
        )
        .bind(code)
        .fetch_optional(&mut tx)
        .await?;

        let inviter_id = match inviter_id {
            Some(inviter_id) => inviter_id,
            None => {
                if sqlx::query_scalar::<_, i32>("SELECT 1 FROM users WHERE invite_code = $1")
                    .bind(code)
                    .fetch_optional(&mut tx)
                    .await?
                    .is_some()
                {
                    Err(Error::Http(
                        StatusCode::UNAUTHORIZED,
                        "no invites remaining".to_string(),
                    ))?
                } else {
                    Err(Error::Http(
                        StatusCode::NOT_FOUND,
                        "invite code not found".to_string(),
                    ))?
                }
            }
        };

        let invitee_id = sqlx::query_scalar(
            "
                INSERT INTO users
                    (github_login, email_address, admin, inviter_id)
                VALUES
                    ($1, $2, 'f', $3)
                RETURNING id
            ",
        )
        .bind(login)
        .bind(email_address)
        .bind(inviter_id)
        .fetch_one(&mut tx)
        .await
        .map(UserId)?;

        sqlx::query(
            "
                INSERT INTO contacts
                    (user_id_a, user_id_b, a_to_b, should_notify, accepted)
                VALUES
                    ($1, $2, 't', 't', 't')
            ",
        )
        .bind(inviter_id)
        .bind(invitee_id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(invitee_id)
    }

    // projects

    async fn register_project(&self, host_user_id: UserId) -> Result<ProjectId> {
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
    }

    async fn unregister_project(&self, project_id: ProjectId) -> Result<()> {
        sqlx::query(
            "
            UPDATE projects
            SET unregistered = 't'
            WHERE project_id = $1
            ",
        )
        .bind(project_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_worktree_extensions(
        &self,
        project_id: ProjectId,
        worktree_id: u64,
        extensions: HashMap<String, usize>,
    ) -> Result<()> {
        let mut query = QueryBuilder::new(
            "INSERT INTO worktree_extensions (project_id, worktree_id, extension, count)",
        );
        query.push_values(extensions, |mut query, (extension, count)| {
            query
                .push_bind(project_id)
                .push_bind(worktree_id as i32)
                .push_bind(extension)
                .push_bind(count as u32);
        });
        query.push(
            "
            ON CONFLICT (project_id, worktree_id, extension) DO UPDATE SET
            count = excluded.count
            ",
        );
        query.build().execute(&self.pool).await?;

        Ok(())
    }

    async fn record_project_activity(
        &self,
        time_period: Range<OffsetDateTime>,
        projects: &[(UserId, ProjectId)],
    ) -> Result<()> {
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
    }

    async fn summarize_project_activity(
        &self,
        time_period: Range<OffsetDateTime>,
        max_user_count: usize,
    ) -> Result<Vec<UserActivitySummary>> {
        let query = "
            WITH
                project_durations AS (
                    SELECT user_id, project_id, SUM(duration_millis) AS project_duration
                    FROM project_activity_periods
                    WHERE $1 <= ended_at AND ended_at <= $2
                    GROUP BY user_id, project_id
                ),
                user_durations AS (
                    SELECT user_id, SUM(project_duration) as total_duration
                    FROM project_durations
                    GROUP BY user_id
                    ORDER BY total_duration DESC
                    LIMIT $3
                )
            SELECT user_durations.user_id, users.github_login, project_id, project_duration
            FROM user_durations, project_durations, users
            WHERE
                user_durations.user_id = project_durations.user_id AND
                user_durations.user_id = users.id
            ORDER BY user_id ASC, project_duration DESC
        ";

        let mut rows = sqlx::query_as::<_, (UserId, String, ProjectId, i64)>(query)
            .bind(time_period.start)
            .bind(time_period.end)
            .bind(max_user_count as i32)
            .fetch(&self.pool);

        let mut result = Vec::<UserActivitySummary>::new();
        while let Some(row) = rows.next().await {
            let (user_id, github_login, project_id, duration_millis) = row?;
            let project_id = project_id;
            let duration = Duration::from_millis(duration_millis as u64);
            if let Some(last_summary) = result.last_mut() {
                if last_summary.id == user_id {
                    last_summary.project_activity.push((project_id, duration));
                    continue;
                }
            }
            result.push(UserActivitySummary {
                id: user_id,
                project_activity: vec![(project_id, duration)],
                github_login,
            });
        }

        Ok(result)
    }

    // contacts

    async fn get_contacts(&self, user_id: UserId) -> Result<Vec<Contact>> {
        let query = "
            SELECT user_id_a, user_id_b, a_to_b, accepted, should_notify
            FROM contacts
            WHERE user_id_a = $1 OR user_id_b = $1;
        ";

        let mut rows = sqlx::query_as::<_, (UserId, UserId, bool, bool, bool)>(query)
            .bind(user_id)
            .fetch(&self.pool);

        let mut contacts = vec![Contact::Accepted {
            user_id,
            should_notify: false,
        }];
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
            } else {
                if accepted {
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
        }

        contacts.sort_unstable_by_key(|contact| contact.user_id());

        Ok(contacts)
    }

    async fn has_contact(&self, user_id_1: UserId, user_id_2: UserId) -> Result<bool> {
        let (id_a, id_b) = if user_id_1 < user_id_2 {
            (user_id_1, user_id_2)
        } else {
            (user_id_2, user_id_1)
        };

        let query = "
            SELECT 1 FROM contacts
            WHERE user_id_a = $1 AND user_id_b = $2 AND accepted = 't'
            LIMIT 1
        ";
        Ok(sqlx::query_scalar::<_, i32>(query)
            .bind(id_a.0)
            .bind(id_b.0)
            .fetch_optional(&self.pool)
            .await?
            .is_some())
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
                accepted = 't',
                should_notify = 'f'
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
    }

    async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<()> {
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
    }

    async fn dismiss_contact_notification(
        &self,
        user_id: UserId,
        contact_user_id: UserId,
    ) -> Result<()> {
        let (id_a, id_b, a_to_b) = if user_id < contact_user_id {
            (user_id, contact_user_id, true)
        } else {
            (contact_user_id, user_id, false)
        };

        let query = "
            UPDATE contacts
            SET should_notify = 'f'
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
                SET accepted = 't', should_notify = 't'
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

    #[cfg(test)]
    fn as_fake(&self) -> Option<&tests::FakeDb> {
        None
    }
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type, Serialize,
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
#[derive(Clone, Debug, Default, FromRow, Serialize, PartialEq)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
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
    pub project_activity: Vec<(ProjectId, Duration)>,
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

            let user = db.create_user("user", None, false).await.unwrap();
            let friend1 = db.create_user("friend-1", None, false).await.unwrap();
            let friend2 = db.create_user("friend-2", None, false).await.unwrap();
            let friend3 = db.create_user("friend-3", None, false).await.unwrap();

            assert_eq!(
                db.get_users_by_ids(vec![user, friend1, friend2, friend3])
                    .await
                    .unwrap(),
                vec![
                    User {
                        id: user,
                        github_login: "user".to_string(),
                        admin: false,
                        ..Default::default()
                    },
                    User {
                        id: friend1,
                        github_login: "friend-1".to_string(),
                        admin: false,
                        ..Default::default()
                    },
                    User {
                        id: friend2,
                        github_login: "friend-2".to_string(),
                        admin: false,
                        ..Default::default()
                    },
                    User {
                        id: friend3,
                        github_login: "friend-3".to_string(),
                        admin: false,
                        ..Default::default()
                    }
                ]
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_users() {
        let db = TestDb::postgres().await;
        let db = db.db();

        // Create the first batch of users, ensuring invite counts are assigned
        // correctly and the respective invite codes are unique.
        let user_ids_batch_1 = db
            .create_users(vec![
                ("user1".to_string(), "hi@user1.com".to_string(), 5),
                ("user2".to_string(), "hi@user2.com".to_string(), 4),
                ("user3".to_string(), "hi@user3.com".to_string(), 3),
            ])
            .await
            .unwrap();
        assert_eq!(user_ids_batch_1.len(), 3);

        let users = db.get_users_by_ids(user_ids_batch_1.clone()).await.unwrap();
        assert_eq!(users.len(), 3);
        assert_eq!(users[0].github_login, "user1");
        assert_eq!(users[0].email_address.as_deref(), Some("hi@user1.com"));
        assert_eq!(users[0].invite_count, 5);
        assert_eq!(users[1].github_login, "user2");
        assert_eq!(users[1].email_address.as_deref(), Some("hi@user2.com"));
        assert_eq!(users[1].invite_count, 4);
        assert_eq!(users[2].github_login, "user3");
        assert_eq!(users[2].email_address.as_deref(), Some("hi@user3.com"));
        assert_eq!(users[2].invite_count, 3);

        let invite_code_1 = users[0].invite_code.clone().unwrap();
        let invite_code_2 = users[1].invite_code.clone().unwrap();
        let invite_code_3 = users[2].invite_code.clone().unwrap();
        assert_ne!(invite_code_1, invite_code_2);
        assert_ne!(invite_code_1, invite_code_3);
        assert_ne!(invite_code_2, invite_code_3);

        // Create the second batch of users and include a user that is already in the database, ensuring
        // the invite count for the existing user is updated without changing their invite code.
        let user_ids_batch_2 = db
            .create_users(vec![
                ("user2".to_string(), "hi@user2.com".to_string(), 10),
                ("user4".to_string(), "hi@user4.com".to_string(), 2),
            ])
            .await
            .unwrap();
        assert_eq!(user_ids_batch_2.len(), 2);
        assert_eq!(user_ids_batch_2[0], user_ids_batch_1[1]);

        let users = db.get_users_by_ids(user_ids_batch_2).await.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].github_login, "user2");
        assert_eq!(users[0].email_address.as_deref(), Some("hi@user2.com"));
        assert_eq!(users[0].invite_count, 10);
        assert_eq!(users[0].invite_code, Some(invite_code_2.clone()));
        assert_eq!(users[1].github_login, "user4");
        assert_eq!(users[1].email_address.as_deref(), Some("hi@user4.com"));
        assert_eq!(users[1].invite_count, 2);

        let invite_code_4 = users[1].invite_code.clone().unwrap();
        assert_ne!(invite_code_4, invite_code_1);
        assert_ne!(invite_code_4, invite_code_2);
        assert_ne!(invite_code_4, invite_code_3);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_project_activity() {
        let test_db = TestDb::postgres().await;
        let db = test_db.db();

        let user_1 = db.create_user("user_1", None, false).await.unwrap();
        let user_2 = db.create_user("user_2", None, false).await.unwrap();
        let user_3 = db.create_user("user_3", None, false).await.unwrap();
        let project_1 = db.register_project(user_1).await.unwrap();
        let project_2 = db.register_project(user_2).await.unwrap();
        let t0 = OffsetDateTime::now_utc() - Duration::from_secs(60 * 60);

        // User 2 opens a project
        let t1 = t0 + Duration::from_secs(10);
        db.record_project_activity(t0..t1, &[(user_2, project_2)])
            .await
            .unwrap();

        let t2 = t1 + Duration::from_secs(10);
        db.record_project_activity(t1..t2, &[(user_2, project_2)])
            .await
            .unwrap();

        // User 1 joins the project
        let t3 = t2 + Duration::from_secs(10);
        db.record_project_activity(t2..t3, &[(user_2, project_2), (user_1, project_2)])
            .await
            .unwrap();

        // User 1 opens another project
        let t4 = t3 + Duration::from_secs(10);
        db.record_project_activity(
            t3..t4,
            &[
                (user_2, project_2),
                (user_1, project_2),
                (user_1, project_1),
            ],
        )
        .await
        .unwrap();

        // User 3 joins that project
        let t5 = t4 + Duration::from_secs(10);
        db.record_project_activity(
            t4..t5,
            &[
                (user_2, project_2),
                (user_1, project_2),
                (user_1, project_1),
                (user_3, project_1),
            ],
        )
        .await
        .unwrap();

        // User 2 leaves
        let t6 = t5 + Duration::from_secs(5);
        db.record_project_activity(t5..t6, &[(user_1, project_1), (user_3, project_1)])
            .await
            .unwrap();

        let summary = db.summarize_project_activity(t0..t6, 10).await.unwrap();
        assert_eq!(
            summary,
            &[
                UserActivitySummary {
                    id: user_1,
                    github_login: "user_1".to_string(),
                    project_activity: vec![
                        (project_2, Duration::from_secs(30)),
                        (project_1, Duration::from_secs(25))
                    ]
                },
                UserActivitySummary {
                    id: user_2,
                    github_login: "user_2".to_string(),
                    project_activity: vec![(project_2, Duration::from_secs(50))]
                },
                UserActivitySummary {
                    id: user_3,
                    github_login: "user_3".to_string(),
                    project_activity: vec![(project_1, Duration::from_secs(15))]
                },
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_recent_channel_messages() {
        for test_db in [
            TestDb::postgres().await,
            TestDb::fake(Arc::new(gpui::executor::Background::new())),
        ] {
            let db = test_db.db();
            let user = db.create_user("user", None, false).await.unwrap();
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
            let user = db.create_user("user", None, false).await.unwrap();
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
        let user = db.create_user("the-user", None, false).await.unwrap();

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
            "California",
            "colorado",
            "oregon",
            "washington",
            "florida",
            "delaware",
            "rhode-island",
        ] {
            db.create_user(github_login, None, false).await.unwrap();
        }

        assert_eq!(
            fuzzy_search_user_names(db, "clr").await,
            &["colorado", "California"]
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

            let user_1 = db.create_user("user1", None, false).await.unwrap();
            let user_2 = db.create_user("user2", None, false).await.unwrap();
            let user_3 = db.create_user("user3", None, false).await.unwrap();

            // User starts with no contacts
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                vec![Contact::Accepted {
                    user_id: user_1,
                    should_notify: false
                }],
            );

            // User requests a contact. Both users see the pending request.
            db.send_contact_request(user_1, user_2).await.unwrap();
            assert!(!db.has_contact(user_1, user_2).await.unwrap());
            assert!(!db.has_contact(user_2, user_1).await.unwrap());
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Outgoing { user_id: user_2 }
                ],
            );
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                &[
                    Contact::Incoming {
                        user_id: user_1,
                        should_notify: true
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: false
                    },
                ]
            );

            // User 2 dismisses the contact request notification without accepting or rejecting.
            // We shouldn't notify them again.
            db.dismiss_contact_notification(user_1, user_2)
                .await
                .unwrap_err();
            db.dismiss_contact_notification(user_2, user_1)
                .await
                .unwrap();
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                &[
                    Contact::Incoming {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: false
                    },
                ]
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
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: true
                    }
                ],
            );
            assert!(db.has_contact(user_1, user_2).await.unwrap());
            assert!(db.has_contact(user_2, user_1).await.unwrap());
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false,
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: false,
                    },
                ]
            );

            // Users cannot re-request existing contacts.
            db.send_contact_request(user_1, user_2).await.unwrap_err();
            db.send_contact_request(user_2, user_1).await.unwrap_err();

            // Users can't dismiss notifications of them accepting other users' requests.
            db.dismiss_contact_notification(user_2, user_1)
                .await
                .unwrap_err();
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: true,
                    },
                ]
            );

            // Users can dismiss notifications of other users accepting their requests.
            db.dismiss_contact_notification(user_1, user_2)
                .await
                .unwrap();
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: false,
                    },
                ]
            );

            // Users send each other concurrent contact requests and
            // see that they are immediately accepted.
            db.send_contact_request(user_1, user_3).await.unwrap();
            db.send_contact_request(user_3, user_1).await.unwrap();
            assert_eq!(
                db.get_contacts(user_1).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: false,
                    },
                    Contact::Accepted {
                        user_id: user_3,
                        should_notify: false
                    },
                ]
            );
            assert_eq!(
                db.get_contacts(user_3).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_3,
                        should_notify: false
                    }
                ],
            );

            // User declines a contact request. Both users see that it is gone.
            db.send_contact_request(user_2, user_3).await.unwrap();
            db.respond_to_contact_request(user_3, user_2, false)
                .await
                .unwrap();
            assert!(!db.has_contact(user_2, user_3).await.unwrap());
            assert!(!db.has_contact(user_3, user_2).await.unwrap());
            assert_eq!(
                db.get_contacts(user_2).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_2,
                        should_notify: false
                    }
                ]
            );
            assert_eq!(
                db.get_contacts(user_3).await.unwrap(),
                &[
                    Contact::Accepted {
                        user_id: user_1,
                        should_notify: false
                    },
                    Contact::Accepted {
                        user_id: user_3,
                        should_notify: false
                    }
                ],
            );
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_invite_codes() {
        let postgres = TestDb::postgres().await;
        let db = postgres.db();
        let user1 = db.create_user("user-1", None, false).await.unwrap();

        // Initially, user 1 has no invite code
        assert_eq!(db.get_invite_code_for_user(user1).await.unwrap(), None);

        // Setting invite count to 0 when no code is assigned does not assign a new code
        db.set_invite_count(user1, 0).await.unwrap();
        assert!(db.get_invite_code_for_user(user1).await.unwrap().is_none());

        // User 1 creates an invite code that can be used twice.
        db.set_invite_count(user1, 2).await.unwrap();
        let (invite_code, invite_count) =
            db.get_invite_code_for_user(user1).await.unwrap().unwrap();
        assert_eq!(invite_count, 2);

        // User 2 redeems the invite code and becomes a contact of user 1.
        let user2 = db
            .redeem_invite_code(&invite_code, "user-2", None)
            .await
            .unwrap();
        let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
        assert_eq!(invite_count, 1);
        assert_eq!(
            db.get_contacts(user1).await.unwrap(),
            [
                Contact::Accepted {
                    user_id: user1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user2,
                    should_notify: true
                }
            ]
        );
        assert_eq!(
            db.get_contacts(user2).await.unwrap(),
            [
                Contact::Accepted {
                    user_id: user1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user2,
                    should_notify: false
                }
            ]
        );

        // User 3 redeems the invite code and becomes a contact of user 1.
        let user3 = db
            .redeem_invite_code(&invite_code, "user-3", None)
            .await
            .unwrap();
        let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
        assert_eq!(invite_count, 0);
        assert_eq!(
            db.get_contacts(user1).await.unwrap(),
            [
                Contact::Accepted {
                    user_id: user1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user2,
                    should_notify: true
                },
                Contact::Accepted {
                    user_id: user3,
                    should_notify: true
                }
            ]
        );
        assert_eq!(
            db.get_contacts(user3).await.unwrap(),
            [
                Contact::Accepted {
                    user_id: user1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user3,
                    should_notify: false
                },
            ]
        );

        // Trying to reedem the code for the third time results in an error.
        db.redeem_invite_code(&invite_code, "user-4", None)
            .await
            .unwrap_err();

        // Invite count can be updated after the code has been created.
        db.set_invite_count(user1, 2).await.unwrap();
        let (latest_code, invite_count) =
            db.get_invite_code_for_user(user1).await.unwrap().unwrap();
        assert_eq!(latest_code, invite_code); // Invite code doesn't change when we increment above 0
        assert_eq!(invite_count, 2);

        // User 4 can now redeem the invite code and becomes a contact of user 1.
        let user4 = db
            .redeem_invite_code(&invite_code, "user-4", None)
            .await
            .unwrap();
        let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
        assert_eq!(invite_count, 1);
        assert_eq!(
            db.get_contacts(user1).await.unwrap(),
            [
                Contact::Accepted {
                    user_id: user1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user2,
                    should_notify: true
                },
                Contact::Accepted {
                    user_id: user3,
                    should_notify: true
                },
                Contact::Accepted {
                    user_id: user4,
                    should_notify: true
                }
            ]
        );
        assert_eq!(
            db.get_contacts(user4).await.unwrap(),
            [
                Contact::Accepted {
                    user_id: user1,
                    should_notify: false
                },
                Contact::Accepted {
                    user_id: user4,
                    should_notify: false
                },
            ]
        );

        // An existing user cannot redeem invite codes.
        db.redeem_invite_code(&invite_code, "user-2", None)
            .await
            .unwrap_err();
        let (_, invite_count) = db.get_invite_code_for_user(user1).await.unwrap().unwrap();
        assert_eq!(invite_count, 1);
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
        pub users: Mutex<BTreeMap<UserId, User>>,
        pub projects: Mutex<BTreeMap<ProjectId, Project>>,
        pub worktree_extensions: Mutex<BTreeMap<(ProjectId, u64, String), usize>>,
        pub orgs: Mutex<BTreeMap<OrgId, Org>>,
        pub org_memberships: Mutex<BTreeMap<(OrgId, UserId), bool>>,
        pub channels: Mutex<BTreeMap<ChannelId, Channel>>,
        pub channel_memberships: Mutex<BTreeMap<(ChannelId, UserId), bool>>,
        pub channel_messages: Mutex<BTreeMap<MessageId, ChannelMessage>>,
        pub contacts: Mutex<Vec<FakeContact>>,
        next_channel_message_id: Mutex<i32>,
        next_user_id: Mutex<i32>,
        next_org_id: Mutex<i32>,
        next_channel_id: Mutex<i32>,
        next_project_id: Mutex<i32>,
    }

    #[derive(Debug)]
    pub struct FakeContact {
        pub requester_id: UserId,
        pub responder_id: UserId,
        pub accepted: bool,
        pub should_notify: bool,
    }

    impl FakeDb {
        pub fn new(background: Arc<Background>) -> Self {
            Self {
                background,
                users: Default::default(),
                next_user_id: Mutex::new(1),
                projects: Default::default(),
                worktree_extensions: Default::default(),
                next_project_id: Mutex::new(1),
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
        async fn create_user(
            &self,
            github_login: &str,
            email_address: Option<&str>,
            admin: bool,
        ) -> Result<UserId> {
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
                        email_address: email_address.map(str::to_string),
                        admin,
                        invite_code: None,
                        invite_count: 0,
                        connected_once: false,
                    },
                );
                Ok(user_id)
            }
        }

        async fn get_all_users(&self, _page: u32, _limit: u32) -> Result<Vec<User>> {
            unimplemented!()
        }

        async fn create_users(&self, _users: Vec<(String, String, usize)>) -> Result<Vec<UserId>> {
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

        async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
            Ok(self
                .users
                .lock()
                .values()
                .find(|user| user.github_login == github_login)
                .cloned())
        }

        async fn set_user_is_admin(&self, _id: UserId, _is_admin: bool) -> Result<()> {
            unimplemented!()
        }

        async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()> {
            self.background.simulate_random_delay().await;
            let mut users = self.users.lock();
            let mut user = users
                .get_mut(&id)
                .ok_or_else(|| anyhow!("user not found"))?;
            user.connected_once = connected_once;
            Ok(())
        }

        async fn destroy_user(&self, _id: UserId) -> Result<()> {
            unimplemented!()
        }

        // invite codes

        async fn set_invite_count(&self, _id: UserId, _count: u32) -> Result<()> {
            unimplemented!()
        }

        async fn get_invite_code_for_user(&self, _id: UserId) -> Result<Option<(String, u32)>> {
            Ok(None)
        }

        async fn get_user_for_invite_code(&self, _code: &str) -> Result<User> {
            unimplemented!()
        }

        async fn redeem_invite_code(
            &self,
            _code: &str,
            _login: &str,
            _email_address: Option<&str>,
        ) -> Result<UserId> {
            unimplemented!()
        }

        // projects

        async fn register_project(&self, host_user_id: UserId) -> Result<ProjectId> {
            self.background.simulate_random_delay().await;
            if !self.users.lock().contains_key(&host_user_id) {
                Err(anyhow!("no such user"))?;
            }

            let project_id = ProjectId(post_inc(&mut *self.next_project_id.lock()));
            self.projects.lock().insert(
                project_id,
                Project {
                    id: project_id,
                    host_user_id,
                    unregistered: false,
                },
            );
            Ok(project_id)
        }

        async fn unregister_project(&self, project_id: ProjectId) -> Result<()> {
            self.projects
                .lock()
                .get_mut(&project_id)
                .ok_or_else(|| anyhow!("no such project"))?
                .unregistered = true;
            Ok(())
        }

        async fn update_worktree_extensions(
            &self,
            project_id: ProjectId,
            worktree_id: u64,
            extensions: HashMap<String, usize>,
        ) -> Result<()> {
            self.background.simulate_random_delay().await;
            if !self.projects.lock().contains_key(&project_id) {
                Err(anyhow!("no such project"))?;
            }

            for (extension, count) in extensions {
                self.worktree_extensions
                    .lock()
                    .insert((project_id, worktree_id, extension), count);
            }

            Ok(())
        }

        async fn record_project_activity(
            &self,
            _period: Range<OffsetDateTime>,
            _active_projects: &[(UserId, ProjectId)],
        ) -> Result<()> {
            unimplemented!()
        }

        async fn summarize_project_activity(
            &self,
            _period: Range<OffsetDateTime>,
            _limit: usize,
        ) -> Result<Vec<UserActivitySummary>> {
            unimplemented!()
        }

        // contacts

        async fn get_contacts(&self, id: UserId) -> Result<Vec<Contact>> {
            self.background.simulate_random_delay().await;
            let mut contacts = vec![Contact::Accepted {
                user_id: id,
                should_notify: false,
            }];

            for contact in self.contacts.lock().iter() {
                if contact.requester_id == id {
                    if contact.accepted {
                        contacts.push(Contact::Accepted {
                            user_id: contact.responder_id,
                            should_notify: contact.should_notify,
                        });
                    } else {
                        contacts.push(Contact::Outgoing {
                            user_id: contact.responder_id,
                        });
                    }
                } else if contact.responder_id == id {
                    if contact.accepted {
                        contacts.push(Contact::Accepted {
                            user_id: contact.requester_id,
                            should_notify: false,
                        });
                    } else {
                        contacts.push(Contact::Incoming {
                            user_id: contact.requester_id,
                            should_notify: contact.should_notify,
                        });
                    }
                }
            }

            contacts.sort_unstable_by_key(|contact| contact.user_id());
            Ok(contacts)
        }

        async fn has_contact(&self, user_id_a: UserId, user_id_b: UserId) -> Result<bool> {
            self.background.simulate_random_delay().await;
            Ok(self.contacts.lock().iter().any(|contact| {
                contact.accepted
                    && ((contact.requester_id == user_id_a && contact.responder_id == user_id_b)
                        || (contact.requester_id == user_id_b && contact.responder_id == user_id_a))
            }))
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
                        contact.should_notify = false;
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

        async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<()> {
            self.contacts.lock().retain(|contact| {
                !(contact.requester_id == requester_id && contact.responder_id == responder_id)
            });
            Ok(())
        }

        async fn dismiss_contact_notification(
            &self,
            user_id: UserId,
            contact_user_id: UserId,
        ) -> Result<()> {
            let mut contacts = self.contacts.lock();
            for contact in contacts.iter_mut() {
                if contact.requester_id == contact_user_id
                    && contact.responder_id == user_id
                    && !contact.accepted
                {
                    contact.should_notify = false;
                    return Ok(());
                }
                if contact.requester_id == user_id
                    && contact.responder_id == contact_user_id
                    && contact.accepted
                {
                    contact.should_notify = false;
                    return Ok(());
                }
            }
            Err(anyhow!("no such notification"))?
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
                        Err(anyhow!("contact already confirmed"))?;
                    }
                    if accept {
                        contact.accepted = true;
                        contact.should_notify = true;
                    } else {
                        contacts.remove(ix);
                    }
                    return Ok(());
                }
            }
            Err(anyhow!("no such contact request"))?
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
                Err(anyhow!("org already exists"))?
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
                Err(anyhow!("org does not exist"))?;
            }
            if !self.users.lock().contains_key(&user_id) {
                Err(anyhow!("user does not exist"))?;
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
                Err(anyhow!("org does not exist"))?;
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
                Err(anyhow!("channel does not exist"))?;
            }
            if !self.users.lock().contains_key(&user_id) {
                Err(anyhow!("user does not exist"))?;
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
                Err(anyhow!("channel does not exist"))?;
            }
            if !self.users.lock().contains_key(&sender_id) {
                Err(anyhow!("user does not exist"))?;
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

        #[cfg(test)]
        fn as_fake(&self) -> Option<&FakeDb> {
            Some(self)
        }
    }
}
