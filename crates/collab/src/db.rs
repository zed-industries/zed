mod access_token;
mod contact;
mod project;
mod project_collaborator;
mod room;
mod room_participant;
mod signup;
#[cfg(test)]
mod tests;
mod user;
mod worktree;

use crate::{Error, Result};
use anyhow::anyhow;
use collections::{BTreeMap, HashMap, HashSet};
pub use contact::Contact;
use dashmap::DashMap;
use futures::StreamExt;
use hyper::StatusCode;
use rpc::{proto, ConnectionId};
pub use sea_orm::ConnectOptions;
use sea_orm::{
    entity::prelude::*, ActiveValue, ConnectionTrait, DatabaseBackend, DatabaseConnection,
    DatabaseTransaction, DbErr, FromQueryResult, IntoActiveModel, JoinType, QueryOrder,
    QuerySelect, Statement, TransactionTrait,
};
use sea_query::{Alias, Expr, OnConflict, Query};
use serde::{Deserialize, Serialize};
pub use signup::{Invite, NewSignup, WaitlistSummary};
use sqlx::migrate::{Migrate, Migration, MigrationSource};
use sqlx::Connection;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::time::Duration;
use std::{future::Future, marker::PhantomData, rc::Rc, sync::Arc};
use tokio::sync::{Mutex, OwnedMutexGuard};
pub use user::Model as User;

pub struct Database {
    options: ConnectOptions,
    pool: DatabaseConnection,
    rooms: DashMap<RoomId, Arc<Mutex<()>>>,
    #[cfg(test)]
    background: Option<std::sync::Arc<gpui::executor::Background>>,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

impl Database {
    pub async fn new(options: ConnectOptions) -> Result<Self> {
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options).await?,
            rooms: DashMap::with_capacity(16384),
            #[cfg(test)]
            background: None,
            #[cfg(test)]
            runtime: None,
        })
    }

    pub async fn migrate(
        &self,
        migrations_path: &Path,
        ignore_checksum_mismatch: bool,
    ) -> anyhow::Result<Vec<(Migration, Duration)>> {
        let migrations = MigrationSource::resolve(migrations_path)
            .await
            .map_err(|err| anyhow!("failed to load migrations: {err:?}"))?;

        let mut connection = sqlx::AnyConnection::connect(self.options.get_url()).await?;

        connection.ensure_migrations_table().await?;
        let applied_migrations: HashMap<_, _> = connection
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
                    let elapsed = connection.apply(&migration).await?;
                    new_migrations.push((migration, elapsed));
                }
            }
        }

        Ok(new_migrations)
    }

    // users

    pub async fn create_user(
        &self,
        email_address: &str,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        self.transact(|tx| async {
            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(Some(email_address.into())),
                github_login: ActiveValue::set(params.github_login.clone()),
                github_user_id: ActiveValue::set(Some(params.github_user_id)),
                admin: ActiveValue::set(admin),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubLogin)
                    .update_column(user::Column::GithubLogin)
                    .to_owned(),
            )
            .exec_with_returning(&tx)
            .await?;

            tx.commit().await?;

            Ok(NewUserResult {
                user_id: user.id,
                metrics_id: user.metrics_id.to_string(),
                signup_device_id: None,
                inviting_user_id: None,
            })
        })
        .await
    }

    pub async fn get_user_by_id(&self, id: UserId) -> Result<Option<user::Model>> {
        self.transact(|tx| async move { Ok(user::Entity::find_by_id(id).one(&tx).await?) })
            .await
    }

    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<user::Model>> {
        self.transact(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Id.is_in(ids.iter().copied()))
                .all(&tx)
                .await?)
        })
        .await
    }

    pub async fn get_user_by_github_account(
        &self,
        github_login: &str,
        github_user_id: Option<i32>,
    ) -> Result<Option<User>> {
        self.transact(|tx| async {
            let tx = tx;
            if let Some(github_user_id) = github_user_id {
                if let Some(user_by_github_user_id) = user::Entity::find()
                    .filter(user::Column::GithubUserId.eq(github_user_id))
                    .one(&tx)
                    .await?
                {
                    let mut user_by_github_user_id = user_by_github_user_id.into_active_model();
                    user_by_github_user_id.github_login = ActiveValue::set(github_login.into());
                    Ok(Some(user_by_github_user_id.update(&tx).await?))
                } else if let Some(user_by_github_login) = user::Entity::find()
                    .filter(user::Column::GithubLogin.eq(github_login))
                    .one(&tx)
                    .await?
                {
                    let mut user_by_github_login = user_by_github_login.into_active_model();
                    user_by_github_login.github_user_id = ActiveValue::set(Some(github_user_id));
                    Ok(Some(user_by_github_login.update(&tx).await?))
                } else {
                    Ok(None)
                }
            } else {
                Ok(user::Entity::find()
                    .filter(user::Column::GithubLogin.eq(github_login))
                    .one(&tx)
                    .await?)
            }
        })
        .await
    }

    pub async fn get_all_users(&self, page: u32, limit: u32) -> Result<Vec<User>> {
        self.transact(|tx| async move {
            Ok(user::Entity::find()
                .order_by_asc(user::Column::GithubLogin)
                .limit(limit as u64)
                .offset(page as u64 * limit as u64)
                .all(&tx)
                .await?)
        })
        .await
    }

    pub async fn get_users_with_no_invites(
        &self,
        invited_by_another_user: bool,
    ) -> Result<Vec<User>> {
        self.transact(|tx| async move {
            Ok(user::Entity::find()
                .filter(
                    user::Column::InviteCount
                        .eq(0)
                        .and(if invited_by_another_user {
                            user::Column::InviterId.is_not_null()
                        } else {
                            user::Column::InviterId.is_null()
                        }),
                )
                .all(&tx)
                .await?)
        })
        .await
    }

    pub async fn get_user_metrics_id(&self, id: UserId) -> Result<String> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            MetricsId,
        }

        self.transact(|tx| async move {
            let metrics_id: Uuid = user::Entity::find_by_id(id)
                .select_only()
                .column(user::Column::MetricsId)
                .into_values::<_, QueryAs>()
                .one(&tx)
                .await?
                .ok_or_else(|| anyhow!("could not find user"))?;
            Ok(metrics_id.to_string())
        })
        .await
    }

    pub async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        self.transact(|tx| async move {
            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .col_expr(user::Column::Admin, is_admin.into())
                .exec(&tx)
                .await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()> {
        self.transact(|tx| async move {
            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .col_expr(user::Column::ConnectedOnce, connected_once.into())
                .exec(&tx)
                .await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn destroy_user(&self, id: UserId) -> Result<()> {
        self.transact(|tx| async move {
            access_token::Entity::delete_many()
                .filter(access_token::Column::UserId.eq(id))
                .exec(&tx)
                .await?;
            user::Entity::delete_by_id(id).exec(&tx).await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    // contacts

    pub async fn get_contacts(&self, user_id: UserId) -> Result<Vec<Contact>> {
        #[derive(Debug, FromQueryResult)]
        struct ContactWithUserBusyStatuses {
            user_id_a: UserId,
            user_id_b: UserId,
            a_to_b: bool,
            accepted: bool,
            should_notify: bool,
            user_a_busy: bool,
            user_b_busy: bool,
        }

        self.transact(|tx| async move {
            let user_a_participant = Alias::new("user_a_participant");
            let user_b_participant = Alias::new("user_b_participant");
            let mut db_contacts = contact::Entity::find()
                .column_as(
                    Expr::tbl(user_a_participant.clone(), room_participant::Column::Id)
                        .is_not_null(),
                    "user_a_busy",
                )
                .column_as(
                    Expr::tbl(user_b_participant.clone(), room_participant::Column::Id)
                        .is_not_null(),
                    "user_b_busy",
                )
                .filter(
                    contact::Column::UserIdA
                        .eq(user_id)
                        .or(contact::Column::UserIdB.eq(user_id)),
                )
                .join_as(
                    JoinType::LeftJoin,
                    contact::Relation::UserARoomParticipant.def(),
                    user_a_participant,
                )
                .join_as(
                    JoinType::LeftJoin,
                    contact::Relation::UserBRoomParticipant.def(),
                    user_b_participant,
                )
                .into_model::<ContactWithUserBusyStatuses>()
                .stream(&tx)
                .await?;

            let mut contacts = Vec::new();
            while let Some(db_contact) = db_contacts.next().await {
                let db_contact = db_contact?;
                if db_contact.user_id_a == user_id {
                    if db_contact.accepted {
                        contacts.push(Contact::Accepted {
                            user_id: db_contact.user_id_b,
                            should_notify: db_contact.should_notify && db_contact.a_to_b,
                            busy: db_contact.user_b_busy,
                        });
                    } else if db_contact.a_to_b {
                        contacts.push(Contact::Outgoing {
                            user_id: db_contact.user_id_b,
                        })
                    } else {
                        contacts.push(Contact::Incoming {
                            user_id: db_contact.user_id_b,
                            should_notify: db_contact.should_notify,
                        });
                    }
                } else if db_contact.accepted {
                    contacts.push(Contact::Accepted {
                        user_id: db_contact.user_id_a,
                        should_notify: db_contact.should_notify && !db_contact.a_to_b,
                        busy: db_contact.user_a_busy,
                    });
                } else if db_contact.a_to_b {
                    contacts.push(Contact::Incoming {
                        user_id: db_contact.user_id_a,
                        should_notify: db_contact.should_notify,
                    });
                } else {
                    contacts.push(Contact::Outgoing {
                        user_id: db_contact.user_id_a,
                    });
                }
            }

            contacts.sort_unstable_by_key(|contact| contact.user_id());

            Ok(contacts)
        })
        .await
    }

    pub async fn is_user_busy(&self, user_id: UserId) -> Result<bool> {
        self.transact(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(room_participant::Column::UserId.eq(user_id))
                .one(&tx)
                .await?;
            Ok(participant.is_some())
        })
        .await
    }

    pub async fn has_contact(&self, user_id_1: UserId, user_id_2: UserId) -> Result<bool> {
        self.transact(|tx| async move {
            let (id_a, id_b) = if user_id_1 < user_id_2 {
                (user_id_1, user_id_2)
            } else {
                (user_id_2, user_id_1)
            };

            Ok(contact::Entity::find()
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b))
                        .and(contact::Column::Accepted.eq(true)),
                )
                .one(&tx)
                .await?
                .is_some())
        })
        .await
    }

    pub async fn send_contact_request(&self, sender_id: UserId, receiver_id: UserId) -> Result<()> {
        self.transact(|tx| async move {
            let (id_a, id_b, a_to_b) = if sender_id < receiver_id {
                (sender_id, receiver_id, true)
            } else {
                (receiver_id, sender_id, false)
            };

            let rows_affected = contact::Entity::insert(contact::ActiveModel {
                user_id_a: ActiveValue::set(id_a),
                user_id_b: ActiveValue::set(id_b),
                a_to_b: ActiveValue::set(a_to_b),
                accepted: ActiveValue::set(false),
                should_notify: ActiveValue::set(true),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::columns([contact::Column::UserIdA, contact::Column::UserIdB])
                    .values([
                        (contact::Column::Accepted, true.into()),
                        (contact::Column::ShouldNotify, false.into()),
                    ])
                    .action_and_where(
                        contact::Column::Accepted.eq(false).and(
                            contact::Column::AToB
                                .eq(a_to_b)
                                .and(contact::Column::UserIdA.eq(id_b))
                                .or(contact::Column::AToB
                                    .ne(a_to_b)
                                    .and(contact::Column::UserIdA.eq(id_a))),
                        ),
                    )
                    .to_owned(),
            )
            .exec_without_returning(&tx)
            .await?;

            if rows_affected == 1 {
                tx.commit().await?;
                Ok(())
            } else {
                Err(anyhow!("contact already requested"))?
            }
        })
        .await
    }

    pub async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<()> {
        self.transact(|tx| async move {
            let (id_a, id_b) = if responder_id < requester_id {
                (responder_id, requester_id)
            } else {
                (requester_id, responder_id)
            };

            let result = contact::Entity::delete_many()
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b)),
                )
                .exec(&tx)
                .await?;

            if result.rows_affected == 1 {
                tx.commit().await?;
                Ok(())
            } else {
                Err(anyhow!("no such contact"))?
            }
        })
        .await
    }

    pub async fn dismiss_contact_notification(
        &self,
        user_id: UserId,
        contact_user_id: UserId,
    ) -> Result<()> {
        self.transact(|tx| async move {
            let (id_a, id_b, a_to_b) = if user_id < contact_user_id {
                (user_id, contact_user_id, true)
            } else {
                (contact_user_id, user_id, false)
            };

            let result = contact::Entity::update_many()
                .set(contact::ActiveModel {
                    should_notify: ActiveValue::set(false),
                    ..Default::default()
                })
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b))
                        .and(
                            contact::Column::AToB
                                .eq(a_to_b)
                                .and(contact::Column::Accepted.eq(true))
                                .or(contact::Column::AToB
                                    .ne(a_to_b)
                                    .and(contact::Column::Accepted.eq(false))),
                        ),
                )
                .exec(&tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("no such contact request"))?
            } else {
                tx.commit().await?;
                Ok(())
            }
        })
        .await
    }

    pub async fn respond_to_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> Result<()> {
        self.transact(|tx| async move {
            let (id_a, id_b, a_to_b) = if responder_id < requester_id {
                (responder_id, requester_id, false)
            } else {
                (requester_id, responder_id, true)
            };
            let rows_affected = if accept {
                let result = contact::Entity::update_many()
                    .set(contact::ActiveModel {
                        accepted: ActiveValue::set(true),
                        should_notify: ActiveValue::set(true),
                        ..Default::default()
                    })
                    .filter(
                        contact::Column::UserIdA
                            .eq(id_a)
                            .and(contact::Column::UserIdB.eq(id_b))
                            .and(contact::Column::AToB.eq(a_to_b)),
                    )
                    .exec(&tx)
                    .await?;
                result.rows_affected
            } else {
                let result = contact::Entity::delete_many()
                    .filter(
                        contact::Column::UserIdA
                            .eq(id_a)
                            .and(contact::Column::UserIdB.eq(id_b))
                            .and(contact::Column::AToB.eq(a_to_b))
                            .and(contact::Column::Accepted.eq(false)),
                    )
                    .exec(&tx)
                    .await?;

                result.rows_affected
            };

            if rows_affected == 1 {
                tx.commit().await?;
                Ok(())
            } else {
                Err(anyhow!("no such contact request"))?
            }
        })
        .await
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

    pub async fn fuzzy_search_users(&self, name_query: &str, limit: u32) -> Result<Vec<User>> {
        self.transact(|tx| async {
            let tx = tx;
            let like_string = Self::fuzzy_like_string(name_query);
            let query = "
                SELECT users.*
                FROM users
                WHERE github_login ILIKE $1
                ORDER BY github_login <-> $2
                LIMIT $3
            ";

            Ok(user::Entity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    self.pool.get_database_backend(),
                    query.into(),
                    vec![like_string.into(), name_query.into(), limit.into()],
                ))
                .all(&tx)
                .await?)
        })
        .await
    }

    // signups

    pub async fn create_signup(&self, signup: NewSignup) -> Result<()> {
        self.transact(|tx| async {
            signup::ActiveModel {
                email_address: ActiveValue::set(signup.email_address.clone()),
                email_confirmation_code: ActiveValue::set(random_email_confirmation_code()),
                email_confirmation_sent: ActiveValue::set(false),
                platform_mac: ActiveValue::set(signup.platform_mac),
                platform_windows: ActiveValue::set(signup.platform_windows),
                platform_linux: ActiveValue::set(signup.platform_linux),
                platform_unknown: ActiveValue::set(false),
                editor_features: ActiveValue::set(Some(signup.editor_features.clone())),
                programming_languages: ActiveValue::set(Some(signup.programming_languages.clone())),
                device_id: ActiveValue::set(signup.device_id.clone()),
                ..Default::default()
            }
            .insert(&tx)
            .await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn get_waitlist_summary(&self) -> Result<WaitlistSummary> {
        self.transact(|tx| async move {
            let query = "
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
            ";
            Ok(
                WaitlistSummary::find_by_statement(Statement::from_sql_and_values(
                    self.pool.get_database_backend(),
                    query.into(),
                    vec![],
                ))
                .one(&tx)
                .await?
                .ok_or_else(|| anyhow!("invalid result"))?,
            )
        })
        .await
    }

    pub async fn record_sent_invites(&self, invites: &[Invite]) -> Result<()> {
        let emails = invites
            .iter()
            .map(|s| s.email_address.as_str())
            .collect::<Vec<_>>();
        self.transact(|tx| async {
            signup::Entity::update_many()
                .filter(signup::Column::EmailAddress.is_in(emails.iter().copied()))
                .col_expr(signup::Column::EmailConfirmationSent, true.into())
                .exec(&tx)
                .await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn get_unsent_invites(&self, count: usize) -> Result<Vec<Invite>> {
        self.transact(|tx| async move {
            Ok(signup::Entity::find()
                .select_only()
                .column(signup::Column::EmailAddress)
                .column(signup::Column::EmailConfirmationCode)
                .filter(
                    signup::Column::EmailConfirmationSent.eq(false).and(
                        signup::Column::PlatformMac
                            .eq(true)
                            .or(signup::Column::PlatformUnknown.eq(true)),
                    ),
                )
                .limit(count as u64)
                .into_model()
                .all(&tx)
                .await?)
        })
        .await
    }

    // invite codes

    pub async fn create_invite_from_code(
        &self,
        code: &str,
        email_address: &str,
        device_id: Option<&str>,
    ) -> Result<Invite> {
        self.transact(|tx| async move {
            let existing_user = user::Entity::find()
                .filter(user::Column::EmailAddress.eq(email_address))
                .one(&tx)
                .await?;

            if existing_user.is_some() {
                Err(anyhow!("email address is already in use"))?;
            }

            let inviter = match user::Entity::find()
                .filter(user::Column::InviteCode.eq(code))
                .one(&tx)
                .await?
            {
                Some(inviter) => inviter,
                None => {
                    return Err(Error::Http(
                        StatusCode::NOT_FOUND,
                        "invite code not found".to_string(),
                    ))?
                }
            };

            if inviter.invite_count == 0 {
                Err(Error::Http(
                    StatusCode::UNAUTHORIZED,
                    "no invites remaining".to_string(),
                ))?;
            }

            let signup = signup::Entity::insert(signup::ActiveModel {
                email_address: ActiveValue::set(email_address.into()),
                email_confirmation_code: ActiveValue::set(random_email_confirmation_code()),
                email_confirmation_sent: ActiveValue::set(false),
                inviting_user_id: ActiveValue::set(Some(inviter.id)),
                platform_linux: ActiveValue::set(false),
                platform_mac: ActiveValue::set(false),
                platform_windows: ActiveValue::set(false),
                platform_unknown: ActiveValue::set(true),
                device_id: ActiveValue::set(device_id.map(|device_id| device_id.into())),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(signup::Column::EmailAddress)
                    .update_column(signup::Column::InvitingUserId)
                    .to_owned(),
            )
            .exec_with_returning(&tx)
            .await?;
            tx.commit().await?;

            Ok(Invite {
                email_address: signup.email_address,
                email_confirmation_code: signup.email_confirmation_code,
            })
        })
        .await
    }

    pub async fn create_user_from_invite(
        &self,
        invite: &Invite,
        user: NewUserParams,
    ) -> Result<Option<NewUserResult>> {
        self.transact(|tx| async {
            let tx = tx;
            let signup = signup::Entity::find()
                .filter(
                    signup::Column::EmailAddress
                        .eq(invite.email_address.as_str())
                        .and(
                            signup::Column::EmailConfirmationCode
                                .eq(invite.email_confirmation_code.as_str()),
                        ),
                )
                .one(&tx)
                .await?
                .ok_or_else(|| Error::Http(StatusCode::NOT_FOUND, "no such invite".to_string()))?;

            if signup.user_id.is_some() {
                return Ok(None);
            }

            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(Some(invite.email_address.clone())),
                github_login: ActiveValue::set(user.github_login.clone()),
                github_user_id: ActiveValue::set(Some(user.github_user_id)),
                admin: ActiveValue::set(false),
                invite_count: ActiveValue::set(user.invite_count),
                invite_code: ActiveValue::set(Some(random_invite_code())),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubLogin)
                    .update_columns([
                        user::Column::EmailAddress,
                        user::Column::GithubUserId,
                        user::Column::Admin,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&tx)
            .await?;

            let mut signup = signup.into_active_model();
            signup.user_id = ActiveValue::set(Some(user.id));
            let signup = signup.update(&tx).await?;

            if let Some(inviting_user_id) = signup.inviting_user_id {
                let result = user::Entity::update_many()
                    .filter(
                        user::Column::Id
                            .eq(inviting_user_id)
                            .and(user::Column::InviteCount.gt(0)),
                    )
                    .col_expr(
                        user::Column::InviteCount,
                        Expr::col(user::Column::InviteCount).sub(1),
                    )
                    .exec(&tx)
                    .await?;

                if result.rows_affected == 0 {
                    Err(Error::Http(
                        StatusCode::UNAUTHORIZED,
                        "no invites remaining".to_string(),
                    ))?;
                }

                contact::Entity::insert(contact::ActiveModel {
                    user_id_a: ActiveValue::set(inviting_user_id),
                    user_id_b: ActiveValue::set(user.id),
                    a_to_b: ActiveValue::set(true),
                    should_notify: ActiveValue::set(true),
                    accepted: ActiveValue::set(true),
                    ..Default::default()
                })
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
            }

            tx.commit().await?;
            Ok(Some(NewUserResult {
                user_id: user.id,
                metrics_id: user.metrics_id.to_string(),
                inviting_user_id: signup.inviting_user_id,
                signup_device_id: signup.device_id,
            }))
        })
        .await
    }

    pub async fn set_invite_count_for_user(&self, id: UserId, count: u32) -> Result<()> {
        self.transact(|tx| async move {
            if count > 0 {
                user::Entity::update_many()
                    .filter(
                        user::Column::Id
                            .eq(id)
                            .and(user::Column::InviteCode.is_null()),
                    )
                    .col_expr(user::Column::InviteCode, random_invite_code().into())
                    .exec(&tx)
                    .await?;
            }

            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .col_expr(user::Column::InviteCount, count.into())
                .exec(&tx)
                .await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn get_invite_code_for_user(&self, id: UserId) -> Result<Option<(String, u32)>> {
        self.transact(|tx| async move {
            match user::Entity::find_by_id(id).one(&tx).await? {
                Some(user) if user.invite_code.is_some() => {
                    Ok(Some((user.invite_code.unwrap(), user.invite_count as u32)))
                }
                _ => Ok(None),
            }
        })
        .await
    }

    pub async fn get_user_for_invite_code(&self, code: &str) -> Result<User> {
        self.transact(|tx| async move {
            user::Entity::find()
                .filter(user::Column::InviteCode.eq(code))
                .one(&tx)
                .await?
                .ok_or_else(|| {
                    Error::Http(
                        StatusCode::NOT_FOUND,
                        "that invite code does not exist".to_string(),
                    )
                })
        })
        .await
    }

    // rooms

    pub async fn incoming_call_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Option<proto::IncomingCall>> {
        self.transact(|tx| async move {
            let pending_participant = room_participant::Entity::find()
                .filter(
                    room_participant::Column::UserId
                        .eq(user_id)
                        .and(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&tx)
                .await?;

            if let Some(pending_participant) = pending_participant {
                let room = self.get_room(pending_participant.room_id, &tx).await?;
                Ok(Self::build_incoming_call(&room, user_id))
            } else {
                Ok(None)
            }
        })
        .await
    }

    pub async fn create_room(
        &self,
        user_id: UserId,
        connection_id: ConnectionId,
        live_kit_room: &str,
    ) -> Result<RoomGuard<proto::Room>> {
        self.transact(|tx| async move {
            let room = room::ActiveModel {
                live_kit_room: ActiveValue::set(live_kit_room.into()),
                ..Default::default()
            }
            .insert(&tx)
            .await?;
            let room_id = room.id;

            room_participant::ActiveModel {
                room_id: ActiveValue::set(room_id),
                user_id: ActiveValue::set(user_id),
                answering_connection_id: ActiveValue::set(Some(connection_id.0 as i32)),
                calling_user_id: ActiveValue::set(user_id),
                calling_connection_id: ActiveValue::set(connection_id.0 as i32),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            self.commit_room_transaction(room_id, tx, room).await
        })
        .await
    }

    pub async fn call(
        &self,
        room_id: RoomId,
        calling_user_id: UserId,
        calling_connection_id: ConnectionId,
        called_user_id: UserId,
        initial_project_id: Option<ProjectId>,
    ) -> Result<RoomGuard<(proto::Room, proto::IncomingCall)>> {
        self.transact(|tx| async move {
            room_participant::ActiveModel {
                room_id: ActiveValue::set(room_id),
                user_id: ActiveValue::set(called_user_id),
                calling_user_id: ActiveValue::set(calling_user_id),
                calling_connection_id: ActiveValue::set(calling_connection_id.0 as i32),
                initial_project_id: ActiveValue::set(initial_project_id),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            let incoming_call = Self::build_incoming_call(&room, called_user_id)
                .ok_or_else(|| anyhow!("failed to build incoming call"))?;
            self.commit_room_transaction(room_id, tx, (room, incoming_call))
                .await
        })
        .await
    }

    pub async fn call_failed(
        &self,
        room_id: RoomId,
        called_user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.transact(|tx| async move {
            room_participant::Entity::delete_many()
                .filter(
                    room_participant::Column::RoomId
                        .eq(room_id)
                        .and(room_participant::Column::UserId.eq(called_user_id)),
                )
                .exec(&tx)
                .await?;
            let room = self.get_room(room_id, &tx).await?;
            self.commit_room_transaction(room_id, tx, room).await
        })
        .await
    }

    pub async fn decline_call(
        &self,
        expected_room_id: Option<RoomId>,
        user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.transact(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    room_participant::Column::UserId
                        .eq(user_id)
                        .and(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&tx)
                .await?
                .ok_or_else(|| anyhow!("could not decline call"))?;
            let room_id = participant.room_id;

            if expected_room_id.map_or(false, |expected_room_id| expected_room_id != room_id) {
                return Err(anyhow!("declining call on unexpected room"))?;
            }

            room_participant::Entity::delete(participant.into_active_model())
                .exec(&tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            self.commit_room_transaction(room_id, tx, room).await
        })
        .await
    }

    pub async fn cancel_call(
        &self,
        expected_room_id: Option<RoomId>,
        calling_connection_id: ConnectionId,
        called_user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.transact(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    room_participant::Column::UserId
                        .eq(called_user_id)
                        .and(
                            room_participant::Column::CallingConnectionId
                                .eq(calling_connection_id.0 as i32),
                        )
                        .and(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&tx)
                .await?
                .ok_or_else(|| anyhow!("could not cancel call"))?;
            let room_id = participant.room_id;
            if expected_room_id.map_or(false, |expected_room_id| expected_room_id != room_id) {
                return Err(anyhow!("canceling call on unexpected room"))?;
            }

            room_participant::Entity::delete(participant.into_active_model())
                .exec(&tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            self.commit_room_transaction(room_id, tx, room).await
        })
        .await
    }

    pub async fn join_room(
        &self,
        room_id: RoomId,
        user_id: UserId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.transact(|tx| async move {
            let result = room_participant::Entity::update_many()
                .filter(
                    room_participant::Column::RoomId
                        .eq(room_id)
                        .and(room_participant::Column::UserId.eq(user_id))
                        .and(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .col_expr(
                    room_participant::Column::AnsweringConnectionId,
                    connection_id.0.into(),
                )
                .exec(&tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("room does not exist or was already joined"))?
            } else {
                let room = self.get_room(room_id, &tx).await?;
                self.commit_room_transaction(room_id, tx, room).await
            }
        })
        .await
    }

    pub async fn leave_room(
        &self,
        connection_id: ConnectionId,
    ) -> Result<Option<RoomGuard<LeftRoom>>> {
        self.transact(|tx| async move {
            let leaving_participant = room_participant::Entity::find()
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.0))
                .one(&tx)
                .await?;

            if let Some(leaving_participant) = leaving_participant {
                // Leave room.
                let room_id = leaving_participant.room_id;
                room_participant::Entity::delete_by_id(leaving_participant.id)
                    .exec(&tx)
                    .await?;

                // Cancel pending calls initiated by the leaving user.
                let called_participants = room_participant::Entity::find()
                    .filter(
                        room_participant::Column::CallingConnectionId
                            .eq(connection_id.0)
                            .and(room_participant::Column::AnsweringConnectionId.is_null()),
                    )
                    .all(&tx)
                    .await?;
                room_participant::Entity::delete_many()
                    .filter(
                        room_participant::Column::Id
                            .is_in(called_participants.iter().map(|participant| participant.id)),
                    )
                    .exec(&tx)
                    .await?;
                let canceled_calls_to_user_ids = called_participants
                    .into_iter()
                    .map(|participant| participant.user_id)
                    .collect();

                // Detect left projects.
                #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
                enum QueryProjectIds {
                    ProjectId,
                }
                let project_ids: Vec<ProjectId> = project_collaborator::Entity::find()
                    .select_only()
                    .column_as(
                        project_collaborator::Column::ProjectId,
                        QueryProjectIds::ProjectId,
                    )
                    .filter(project_collaborator::Column::ConnectionId.eq(connection_id.0))
                    .into_values::<_, QueryProjectIds>()
                    .all(&tx)
                    .await?;
                let mut left_projects = HashMap::default();
                let mut collaborators = project_collaborator::Entity::find()
                    .filter(project_collaborator::Column::ProjectId.is_in(project_ids))
                    .stream(&tx)
                    .await?;
                while let Some(collaborator) = collaborators.next().await {
                    let collaborator = collaborator?;
                    let left_project =
                        left_projects
                            .entry(collaborator.project_id)
                            .or_insert(LeftProject {
                                id: collaborator.project_id,
                                host_user_id: Default::default(),
                                connection_ids: Default::default(),
                                host_connection_id: Default::default(),
                            });

                    let collaborator_connection_id =
                        ConnectionId(collaborator.connection_id as u32);
                    if collaborator_connection_id != connection_id {
                        left_project.connection_ids.push(collaborator_connection_id);
                    }

                    if collaborator.is_host {
                        left_project.host_user_id = collaborator.user_id;
                        left_project.host_connection_id =
                            ConnectionId(collaborator.connection_id as u32);
                    }
                }
                drop(collaborators);

                // Leave projects.
                project_collaborator::Entity::delete_many()
                    .filter(project_collaborator::Column::ConnectionId.eq(connection_id.0))
                    .exec(&tx)
                    .await?;

                // Unshare projects.
                project::Entity::delete_many()
                    .filter(
                        project::Column::RoomId
                            .eq(room_id)
                            .and(project::Column::HostConnectionId.eq(connection_id.0)),
                    )
                    .exec(&tx)
                    .await?;

                let room = self.get_room(room_id, &tx).await?;
                Ok(Some(
                    self.commit_room_transaction(
                        room_id,
                        tx,
                        LeftRoom {
                            room,
                            left_projects,
                            canceled_calls_to_user_ids,
                        },
                    )
                    .await?,
                ))
            } else {
                Ok(None)
            }
        })
        .await
    }

    pub async fn update_room_participant_location(
        &self,
        room_id: RoomId,
        connection_id: ConnectionId,
        location: proto::ParticipantLocation,
    ) -> Result<RoomGuard<proto::Room>> {
        self.transact(|tx| async {
            let mut tx = tx;
            let location_kind;
            let location_project_id;
            match location
                .variant
                .as_ref()
                .ok_or_else(|| anyhow!("invalid location"))?
            {
                proto::participant_location::Variant::SharedProject(project) => {
                    location_kind = 0;
                    location_project_id = Some(ProjectId::from_proto(project.id));
                }
                proto::participant_location::Variant::UnsharedProject(_) => {
                    location_kind = 1;
                    location_project_id = None;
                }
                proto::participant_location::Variant::External(_) => {
                    location_kind = 2;
                    location_project_id = None;
                }
            }

            let result = room_participant::Entity::update_many()
                .filter(
                    room_participant::Column::RoomId
                        .eq(room_id)
                        .and(room_participant::Column::AnsweringConnectionId.eq(connection_id.0)),
                )
                .set(room_participant::ActiveModel {
                    location_kind: ActiveValue::set(Some(location_kind)),
                    location_project_id: ActiveValue::set(location_project_id),
                    ..Default::default()
                })
                .exec(&tx)
                .await?;

            if result.rows_affected == 1 {
                let room = self.get_room(room_id, &mut tx).await?;
                self.commit_room_transaction(room_id, tx, room).await
            } else {
                Err(anyhow!("could not update room participant location"))?
            }
        })
        .await
    }

    async fn get_guest_connection_ids(
        &self,
        project_id: ProjectId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ConnectionId>> {
        todo!()
        // let mut guest_connection_ids = Vec::new();
        // let mut db_guest_connection_ids = sqlx::query_scalar::<_, i32>(
        //     "
        //     SELECT connection_id
        //     FROM project_collaborators
        //     WHERE project_id = $1 AND is_host = FALSE
        //     ",
        // )
        // .bind(project_id)
        // .fetch(tx);
        // while let Some(connection_id) = db_guest_connection_ids.next().await {
        //     guest_connection_ids.push(ConnectionId(connection_id? as u32));
        // }
        // Ok(guest_connection_ids)
    }

    fn build_incoming_call(
        room: &proto::Room,
        called_user_id: UserId,
    ) -> Option<proto::IncomingCall> {
        let pending_participant = room
            .pending_participants
            .iter()
            .find(|participant| participant.user_id == called_user_id.to_proto())?;

        Some(proto::IncomingCall {
            room_id: room.id,
            calling_user_id: pending_participant.calling_user_id,
            participant_user_ids: room
                .participants
                .iter()
                .map(|participant| participant.user_id)
                .collect(),
            initial_project: room.participants.iter().find_map(|participant| {
                let initial_project_id = pending_participant.initial_project_id?;
                participant
                    .projects
                    .iter()
                    .find(|project| project.id == initial_project_id)
                    .cloned()
            }),
        })
    }

    async fn get_room(&self, room_id: RoomId, tx: &DatabaseTransaction) -> Result<proto::Room> {
        let db_room = room::Entity::find_by_id(room_id)
            .one(tx)
            .await?
            .ok_or_else(|| anyhow!("could not find room"))?;

        let mut db_participants = db_room
            .find_related(room_participant::Entity)
            .stream(tx)
            .await?;
        let mut participants = HashMap::default();
        let mut pending_participants = Vec::new();
        while let Some(db_participant) = db_participants.next().await {
            let db_participant = db_participant?;
            if let Some(answering_connection_id) = db_participant.answering_connection_id {
                let location = match (
                    db_participant.location_kind,
                    db_participant.location_project_id,
                ) {
                    (Some(0), Some(project_id)) => {
                        Some(proto::participant_location::Variant::SharedProject(
                            proto::participant_location::SharedProject {
                                id: project_id.to_proto(),
                            },
                        ))
                    }
                    (Some(1), _) => Some(proto::participant_location::Variant::UnsharedProject(
                        Default::default(),
                    )),
                    _ => Some(proto::participant_location::Variant::External(
                        Default::default(),
                    )),
                };
                participants.insert(
                    answering_connection_id,
                    proto::Participant {
                        user_id: db_participant.user_id.to_proto(),
                        peer_id: answering_connection_id as u32,
                        projects: Default::default(),
                        location: Some(proto::ParticipantLocation { variant: location }),
                    },
                );
            } else {
                pending_participants.push(proto::PendingParticipant {
                    user_id: db_participant.user_id.to_proto(),
                    calling_user_id: db_participant.calling_user_id.to_proto(),
                    initial_project_id: db_participant.initial_project_id.map(|id| id.to_proto()),
                });
            }
        }
        drop(db_participants);

        let mut db_projects = db_room
            .find_related(project::Entity)
            .find_with_related(worktree::Entity)
            .stream(tx)
            .await?;

        while let Some(row) = db_projects.next().await {
            let (db_project, db_worktree) = row?;
            if let Some(participant) = participants.get_mut(&db_project.host_connection_id) {
                let project = if let Some(project) = participant
                    .projects
                    .iter_mut()
                    .find(|project| project.id == db_project.id.to_proto())
                {
                    project
                } else {
                    participant.projects.push(proto::ParticipantProject {
                        id: db_project.id.to_proto(),
                        worktree_root_names: Default::default(),
                    });
                    participant.projects.last_mut().unwrap()
                };

                if let Some(db_worktree) = db_worktree {
                    project.worktree_root_names.push(db_worktree.root_name);
                }
            }
        }

        Ok(proto::Room {
            id: db_room.id.to_proto(),
            live_kit_room: db_room.live_kit_room,
            participants: participants.into_values().collect(),
            pending_participants,
        })
    }

    async fn commit_room_transaction<T>(
        &self,
        room_id: RoomId,
        tx: DatabaseTransaction,
        data: T,
    ) -> Result<RoomGuard<T>> {
        let lock = self.rooms.entry(room_id).or_default().clone();
        let _guard = lock.lock_owned().await;
        tx.commit().await?;
        Ok(RoomGuard {
            data,
            _guard,
            _not_send: PhantomData,
        })
    }

    // projects

    pub async fn project_count_excluding_admins(&self) -> Result<usize> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            Count,
        }

        self.transact(|tx| async move {
            Ok(project::Entity::find()
                .select_only()
                .column_as(project::Column::Id.count(), QueryAs::Count)
                .inner_join(user::Entity)
                .filter(user::Column::Admin.eq(false))
                .into_values::<_, QueryAs>()
                .one(&tx)
                .await?
                .unwrap_or(0) as usize)
        })
        .await
    }

    pub async fn share_project(
        &self,
        room_id: RoomId,
        connection_id: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(ProjectId, proto::Room)>> {
        self.transact(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(room_participant::Column::AnsweringConnectionId.eq(connection_id.0))
                .one(&tx)
                .await?
                .ok_or_else(|| anyhow!("could not find participant"))?;
            if participant.room_id != room_id {
                return Err(anyhow!("shared project on unexpected room"))?;
            }

            let project = project::ActiveModel {
                room_id: ActiveValue::set(participant.room_id),
                host_user_id: ActiveValue::set(participant.user_id),
                host_connection_id: ActiveValue::set(connection_id.0 as i32),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            worktree::Entity::insert_many(worktrees.iter().map(|worktree| worktree::ActiveModel {
                id: ActiveValue::set(worktree.id as i32),
                project_id: ActiveValue::set(project.id),
                abs_path: ActiveValue::set(worktree.abs_path.clone()),
                root_name: ActiveValue::set(worktree.root_name.clone()),
                visible: ActiveValue::set(worktree.visible),
                scan_id: ActiveValue::set(0),
                is_complete: ActiveValue::set(false),
            }))
            .exec(&tx)
            .await?;

            project_collaborator::ActiveModel {
                project_id: ActiveValue::set(project.id),
                connection_id: ActiveValue::set(connection_id.0 as i32),
                user_id: ActiveValue::set(participant.user_id),
                replica_id: ActiveValue::set(ReplicaId(0)),
                is_host: ActiveValue::set(true),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            self.commit_room_transaction(room_id, tx, (project.id, room))
                .await
        })
        .await
    }

    pub async fn unshare_project(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<(proto::Room, Vec<ConnectionId>)>> {
        self.transact(|tx| async move {
            todo!()
            // let guest_connection_ids = self.get_guest_connection_ids(project_id, &mut tx).await?;
            // let room_id: RoomId = sqlx::query_scalar(
            //     "
            //     DELETE FROM projects
            //     WHERE id = $1 AND host_connection_id = $2
            //     RETURNING room_id
            //     ",
            // )
            // .bind(project_id)
            // .bind(connection_id.0 as i32)
            // .fetch_one(&mut tx)
            // .await?;
            // let room = self.get_room(room_id, &mut tx).await?;
            // self.commit_room_transaction(room_id, tx, (room, guest_connection_ids))
            //     .await
        })
        .await
    }

    pub async fn update_project(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(proto::Room, Vec<ConnectionId>)>> {
        self.transact(|tx| async move {
            todo!()
            // let room_id: RoomId = sqlx::query_scalar(
            //     "
            //     SELECT room_id
            //     FROM projects
            //     WHERE id = $1 AND host_connection_id = $2
            //     ",
            // )
            // .bind(project_id)
            // .bind(connection_id.0 as i32)
            // .fetch_one(&mut tx)
            // .await?;

            // if !worktrees.is_empty() {
            //     let mut params = "(?, ?, ?, ?, ?, ?, ?),".repeat(worktrees.len());
            //     params.pop();
            //     let query = format!(
            //         "
            //         INSERT INTO worktrees (
            //         project_id,
            //         id,
            //         root_name,
            //         abs_path,
            //         visible,
            //         scan_id,
            //         is_complete
            //         )
            //         VALUES {params}
            //         ON CONFLICT (project_id, id) DO UPDATE SET root_name = excluded.root_name
            //         "
            //     );

            //     let mut query = sqlx::query(&query);
            //     for worktree in worktrees {
            //         query = query
            //             .bind(project_id)
            //             .bind(worktree.id as i32)
            //             .bind(&worktree.root_name)
            //             .bind(&worktree.abs_path)
            //             .bind(worktree.visible)
            //             .bind(0)
            //             .bind(false)
            //     }
            //     query.execute(&mut tx).await?;
            // }

            // let mut params = "?,".repeat(worktrees.len());
            // if !worktrees.is_empty() {
            //     params.pop();
            // }
            // let query = format!(
            //     "
            //     DELETE FROM worktrees
            //     WHERE project_id = ? AND id NOT IN ({params})
            //     ",
            // );

            // let mut query = sqlx::query(&query).bind(project_id);
            // for worktree in worktrees {
            //     query = query.bind(WorktreeId(worktree.id as i32));
            // }
            // query.execute(&mut tx).await?;

            // let guest_connection_ids = self.get_guest_connection_ids(project_id, &mut tx).await?;
            // let room = self.get_room(room_id, &mut tx).await?;
            // self.commit_room_transaction(room_id, tx, (room, guest_connection_ids))
            //     .await
        })
        .await
    }

    pub async fn update_worktree(
        &self,
        update: &proto::UpdateWorktree,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        self.transact(|tx| async move {
            todo!()
            // let project_id = ProjectId::from_proto(update.project_id);
            // let worktree_id = WorktreeId::from_proto(update.worktree_id);

            // // Ensure the update comes from the host.
            // let room_id: RoomId = sqlx::query_scalar(
            //     "
            //     SELECT room_id
            //     FROM projects
            //     WHERE id = $1 AND host_connection_id = $2
            //     ",
            // )
            // .bind(project_id)
            // .bind(connection_id.0 as i32)
            // .fetch_one(&mut tx)
            // .await?;

            // // Update metadata.
            // sqlx::query(
            //     "
            //     UPDATE worktrees
            //     SET
            //     root_name = $1,
            //     scan_id = $2,
            //     is_complete = $3,
            //     abs_path = $4
            //     WHERE project_id = $5 AND id = $6
            //     RETURNING 1
            //     ",
            // )
            // .bind(&update.root_name)
            // .bind(update.scan_id as i64)
            // .bind(update.is_last_update)
            // .bind(&update.abs_path)
            // .bind(project_id)
            // .bind(worktree_id)
            // .fetch_one(&mut tx)
            // .await?;

            // if !update.updated_entries.is_empty() {
            //     let mut params =
            //         "(?, ?, ?, ?, ?, ?, ?, ?, ?, ?),".repeat(update.updated_entries.len());
            //     params.pop();

            //     let query = format!(
            //         "
            //         INSERT INTO worktree_entries (
            //         project_id,
            //         worktree_id,
            //         id,
            //         is_dir,
            //         path,
            //         inode,
            //         mtime_seconds,
            //         mtime_nanos,
            //         is_symlink,
            //         is_ignored
            //         )
            //         VALUES {params}
            //         ON CONFLICT (project_id, worktree_id, id) DO UPDATE SET
            //         is_dir = excluded.is_dir,
            //         path = excluded.path,
            //         inode = excluded.inode,
            //         mtime_seconds = excluded.mtime_seconds,
            //         mtime_nanos = excluded.mtime_nanos,
            //         is_symlink = excluded.is_symlink,
            //         is_ignored = excluded.is_ignored
            //         "
            //     );
            //     let mut query = sqlx::query(&query);
            //     for entry in &update.updated_entries {
            //         let mtime = entry.mtime.clone().unwrap_or_default();
            //         query = query
            //             .bind(project_id)
            //             .bind(worktree_id)
            //             .bind(entry.id as i64)
            //             .bind(entry.is_dir)
            //             .bind(&entry.path)
            //             .bind(entry.inode as i64)
            //             .bind(mtime.seconds as i64)
            //             .bind(mtime.nanos as i32)
            //             .bind(entry.is_symlink)
            //             .bind(entry.is_ignored);
            //     }
            //     query.execute(&mut tx).await?;
            // }

            // if !update.removed_entries.is_empty() {
            //     let mut params = "?,".repeat(update.removed_entries.len());
            //     params.pop();
            //     let query = format!(
            //         "
            //         DELETE FROM worktree_entries
            //         WHERE project_id = ? AND worktree_id = ? AND id IN ({params})
            //         "
            //     );

            //     let mut query = sqlx::query(&query).bind(project_id).bind(worktree_id);
            //     for entry_id in &update.removed_entries {
            //         query = query.bind(*entry_id as i64);
            //     }
            //     query.execute(&mut tx).await?;
            // }

            // let connection_ids = self.get_guest_connection_ids(project_id, &mut tx).await?;
            // self.commit_room_transaction(room_id, tx, connection_ids)
            //     .await
        })
        .await
    }

    pub async fn update_diagnostic_summary(
        &self,
        update: &proto::UpdateDiagnosticSummary,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        self.transact(|tx| async {
            todo!()
            // let project_id = ProjectId::from_proto(update.project_id);
            // let worktree_id = WorktreeId::from_proto(update.worktree_id);
            // let summary = update
            //     .summary
            //     .as_ref()
            //     .ok_or_else(|| anyhow!("invalid summary"))?;

            // // Ensure the update comes from the host.
            // let room_id: RoomId = sqlx::query_scalar(
            //     "
            //     SELECT room_id
            //     FROM projects
            //     WHERE id = $1 AND host_connection_id = $2
            //     ",
            // )
            // .bind(project_id)
            // .bind(connection_id.0 as i32)
            // .fetch_one(&mut tx)
            // .await?;

            // // Update summary.
            // sqlx::query(
            //     "
            //     INSERT INTO worktree_diagnostic_summaries (
            //     project_id,
            //     worktree_id,
            //     path,
            //     language_server_id,
            //     error_count,
            //     warning_count
            //     )
            //     VALUES ($1, $2, $3, $4, $5, $6)
            //     ON CONFLICT (project_id, worktree_id, path) DO UPDATE SET
            //     language_server_id = excluded.language_server_id,
            //     error_count = excluded.error_count,
            //     warning_count = excluded.warning_count
            //     ",
            // )
            // .bind(project_id)
            // .bind(worktree_id)
            // .bind(&summary.path)
            // .bind(summary.language_server_id as i64)
            // .bind(summary.error_count as i32)
            // .bind(summary.warning_count as i32)
            // .execute(&mut tx)
            // .await?;

            // let connection_ids = self.get_guest_connection_ids(project_id, &mut tx).await?;
            // self.commit_room_transaction(room_id, tx, connection_ids)
            //     .await
        })
        .await
    }

    pub async fn start_language_server(
        &self,
        update: &proto::StartLanguageServer,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        self.transact(|tx| async {
            todo!()
            // let project_id = ProjectId::from_proto(update.project_id);
            // let server = update
            //     .server
            //     .as_ref()
            //     .ok_or_else(|| anyhow!("invalid language server"))?;

            // // Ensure the update comes from the host.
            // let room_id: RoomId = sqlx::query_scalar(
            //     "
            //     SELECT room_id
            //     FROM projects
            //     WHERE id = $1 AND host_connection_id = $2
            //     ",
            // )
            // .bind(project_id)
            // .bind(connection_id.0 as i32)
            // .fetch_one(&mut tx)
            // .await?;

            // // Add the newly-started language server.
            // sqlx::query(
            //     "
            //     INSERT INTO language_servers (project_id, id, name)
            //     VALUES ($1, $2, $3)
            //     ON CONFLICT (project_id, id) DO UPDATE SET
            //     name = excluded.name
            //     ",
            // )
            // .bind(project_id)
            // .bind(server.id as i64)
            // .bind(&server.name)
            // .execute(&mut tx)
            // .await?;

            // let connection_ids = self.get_guest_connection_ids(project_id, &mut tx).await?;
            // self.commit_room_transaction(room_id, tx, connection_ids)
            //     .await
        })
        .await
    }

    pub async fn join_project(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<(Project, ReplicaId)>> {
        self.transact(|tx| async move {
            todo!()
            // let (room_id, user_id) = sqlx::query_as::<_, (RoomId, UserId)>(
            //     "
            //     SELECT room_id, user_id
            //     FROM room_participants
            //     WHERE answering_connection_id = $1
            //     ",
            // )
            // .bind(connection_id.0 as i32)
            // .fetch_one(&mut tx)
            // .await?;

            // // Ensure project id was shared on this room.
            // sqlx::query(
            //     "
            //     SELECT 1
            //     FROM projects
            //     WHERE id = $1 AND room_id = $2
            //     ",
            // )
            // .bind(project_id)
            // .bind(room_id)
            // .fetch_one(&mut tx)
            // .await?;

            // let mut collaborators = sqlx::query_as::<_, ProjectCollaborator>(
            //     "
            //     SELECT *
            //     FROM project_collaborators
            //     WHERE project_id = $1
            //     ",
            // )
            // .bind(project_id)
            // .fetch_all(&mut tx)
            // .await?;
            // let replica_ids = collaborators
            //     .iter()
            //     .map(|c| c.replica_id)
            //     .collect::<HashSet<_>>();
            // let mut replica_id = ReplicaId(1);
            // while replica_ids.contains(&replica_id) {
            //     replica_id.0 += 1;
            // }
            // let new_collaborator = ProjectCollaborator {
            //     project_id,
            //     connection_id: connection_id.0 as i32,
            //     user_id,
            //     replica_id,
            //     is_host: false,
            // };

            // sqlx::query(
            //     "
            //     INSERT INTO project_collaborators (
            //     project_id,
            //     connection_id,
            //     user_id,
            //     replica_id,
            //     is_host
            //     )
            //     VALUES ($1, $2, $3, $4, $5)
            //     ",
            // )
            // .bind(new_collaborator.project_id)
            // .bind(new_collaborator.connection_id)
            // .bind(new_collaborator.user_id)
            // .bind(new_collaborator.replica_id)
            // .bind(new_collaborator.is_host)
            // .execute(&mut tx)
            // .await?;
            // collaborators.push(new_collaborator);

            // let worktree_rows = sqlx::query_as::<_, WorktreeRow>(
            //     "
            //     SELECT *
            //     FROM worktrees
            //     WHERE project_id = $1
            //     ",
            // )
            // .bind(project_id)
            // .fetch_all(&mut tx)
            // .await?;
            // let mut worktrees = worktree_rows
            //     .into_iter()
            //     .map(|worktree_row| {
            //         (
            //             worktree_row.id,
            //             Worktree {
            //                 id: worktree_row.id,
            //                 abs_path: worktree_row.abs_path,
            //                 root_name: worktree_row.root_name,
            //                 visible: worktree_row.visible,
            //                 entries: Default::default(),
            //                 diagnostic_summaries: Default::default(),
            //                 scan_id: worktree_row.scan_id as u64,
            //                 is_complete: worktree_row.is_complete,
            //             },
            //         )
            //     })
            //     .collect::<BTreeMap<_, _>>();

            // // Populate worktree entries.
            // {
            //     let mut entries = sqlx::query_as::<_, WorktreeEntry>(
            //         "
            //         SELECT *
            //         FROM worktree_entries
            //         WHERE project_id = $1
            //         ",
            //     )
            //     .bind(project_id)
            //     .fetch(&mut tx);
            //     while let Some(entry) = entries.next().await {
            //         let entry = entry?;
            //         if let Some(worktree) = worktrees.get_mut(&entry.worktree_id) {
            //             worktree.entries.push(proto::Entry {
            //                 id: entry.id as u64,
            //                 is_dir: entry.is_dir,
            //                 path: entry.path,
            //                 inode: entry.inode as u64,
            //                 mtime: Some(proto::Timestamp {
            //                     seconds: entry.mtime_seconds as u64,
            //                     nanos: entry.mtime_nanos as u32,
            //                 }),
            //                 is_symlink: entry.is_symlink,
            //                 is_ignored: entry.is_ignored,
            //             });
            //         }
            //     }
            // }

            // // Populate worktree diagnostic summaries.
            // {
            //     let mut summaries = sqlx::query_as::<_, WorktreeDiagnosticSummary>(
            //         "
            //         SELECT *
            //         FROM worktree_diagnostic_summaries
            //         WHERE project_id = $1
            //         ",
            //     )
            //     .bind(project_id)
            //     .fetch(&mut tx);
            //     while let Some(summary) = summaries.next().await {
            //         let summary = summary?;
            //         if let Some(worktree) = worktrees.get_mut(&summary.worktree_id) {
            //             worktree
            //                 .diagnostic_summaries
            //                 .push(proto::DiagnosticSummary {
            //                     path: summary.path,
            //                     language_server_id: summary.language_server_id as u64,
            //                     error_count: summary.error_count as u32,
            //                     warning_count: summary.warning_count as u32,
            //                 });
            //         }
            //     }
            // }

            // // Populate language servers.
            // let language_servers = sqlx::query_as::<_, LanguageServer>(
            //     "
            //     SELECT *
            //     FROM language_servers
            //     WHERE project_id = $1
            //     ",
            // )
            // .bind(project_id)
            // .fetch_all(&mut tx)
            // .await?;

            // self.commit_room_transaction(
            //     room_id,
            //     tx,
            //     (
            //         Project {
            //             collaborators,
            //             worktrees,
            //             language_servers: language_servers
            //                 .into_iter()
            //                 .map(|language_server| proto::LanguageServer {
            //                     id: language_server.id.to_proto(),
            //                     name: language_server.name,
            //                 })
            //                 .collect(),
            //         },
            //         replica_id as ReplicaId,
            //     ),
            // )
            // .await
        })
        .await
    }

    pub async fn leave_project(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<LeftProject>> {
        self.transact(|tx| async move {
            todo!()
            // let result = sqlx::query(
            //     "
            //     DELETE FROM project_collaborators
            //     WHERE project_id = $1 AND connection_id = $2
            //     ",
            // )
            // .bind(project_id)
            // .bind(connection_id.0 as i32)
            // .execute(&mut tx)
            // .await?;

            // if result.rows_affected() == 0 {
            //     Err(anyhow!("not a collaborator on this project"))?;
            // }

            // let connection_ids = sqlx::query_scalar::<_, i32>(
            //     "
            //     SELECT connection_id
            //     FROM project_collaborators
            //     WHERE project_id = $1
            //     ",
            // )
            // .bind(project_id)
            // .fetch_all(&mut tx)
            // .await?
            // .into_iter()
            // .map(|id| ConnectionId(id as u32))
            // .collect();

            // let (room_id, host_user_id, host_connection_id) =
            //     sqlx::query_as::<_, (RoomId, i32, i32)>(
            //         "
            //         SELECT room_id, host_user_id, host_connection_id
            //         FROM projects
            //         WHERE id = $1
            //         ",
            //     )
            //     .bind(project_id)
            //     .fetch_one(&mut tx)
            //     .await?;

            // self.commit_room_transaction(
            //     room_id,
            //     tx,
            //     LeftProject {
            //         id: project_id,
            //         host_user_id: UserId(host_user_id),
            //         host_connection_id: ConnectionId(host_connection_id as u32),
            //         connection_ids,
            //     },
            // )
            // .await
        })
        .await
    }

    pub async fn project_collaborators(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<Vec<project_collaborator::Model>> {
        self.transact(|tx| async move {
            todo!()
            // let collaborators = sqlx::query_as::<_, ProjectCollaborator>(
            //     "
            //     SELECT *
            //     FROM project_collaborators
            //     WHERE project_id = $1
            //     ",
            // )
            // .bind(project_id)
            // .fetch_all(&mut tx)
            // .await?;

            // if collaborators
            //     .iter()
            //     .any(|collaborator| collaborator.connection_id == connection_id.0 as i32)
            // {
            //     Ok(collaborators)
            // } else {
            //     Err(anyhow!("no such project"))?
            // }
        })
        .await
    }

    pub async fn project_connection_ids(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<HashSet<ConnectionId>> {
        self.transact(|tx| async move {
            todo!()
            // let connection_ids = sqlx::query_scalar::<_, i32>(
            //     "
            //     SELECT connection_id
            //     FROM project_collaborators
            //     WHERE project_id = $1
            //     ",
            // )
            // .bind(project_id)
            // .fetch_all(&mut tx)
            // .await?;

            // if connection_ids.contains(&(connection_id.0 as i32)) {
            //     Ok(connection_ids
            //         .into_iter()
            //         .map(|connection_id| ConnectionId(connection_id as u32))
            //         .collect())
            // } else {
            //     Err(anyhow!("no such project"))?
            // }
        })
        .await
    }

    // access tokens

    pub async fn create_access_token_hash(
        &self,
        user_id: UserId,
        access_token_hash: &str,
        max_access_token_count: usize,
    ) -> Result<()> {
        self.transact(|tx| async {
            let tx = tx;

            access_token::ActiveModel {
                user_id: ActiveValue::set(user_id),
                hash: ActiveValue::set(access_token_hash.into()),
                ..Default::default()
            }
            .insert(&tx)
            .await?;

            access_token::Entity::delete_many()
                .filter(
                    access_token::Column::Id.in_subquery(
                        Query::select()
                            .column(access_token::Column::Id)
                            .from(access_token::Entity)
                            .and_where(access_token::Column::UserId.eq(user_id))
                            .order_by(access_token::Column::Id, sea_orm::Order::Desc)
                            .limit(10000)
                            .offset(max_access_token_count as u64)
                            .to_owned(),
                    ),
                )
                .exec(&tx)
                .await?;
            tx.commit().await?;
            Ok(())
        })
        .await
    }

    pub async fn get_access_token_hashes(&self, user_id: UserId) -> Result<Vec<String>> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            Hash,
        }

        self.transact(|tx| async move {
            Ok(access_token::Entity::find()
                .select_only()
                .column(access_token::Column::Hash)
                .filter(access_token::Column::UserId.eq(user_id))
                .order_by_desc(access_token::Column::Id)
                .into_values::<_, QueryAs>()
                .all(&tx)
                .await?)
        })
        .await
    }

    async fn transact<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: Send + Fn(DatabaseTransaction) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let body = async {
            loop {
                let tx = self.pool.begin().await?;

                // In Postgres, serializable transactions are opt-in
                if let DatabaseBackend::Postgres = self.pool.get_database_backend() {
                    tx.execute(Statement::from_string(
                        DatabaseBackend::Postgres,
                        "SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;".into(),
                    ))
                    .await?;
                }

                match f(tx).await {
                    Ok(result) => return Ok(result),
                    Err(error) => match error {
                        Error::Database2(
                            DbErr::Exec(sea_orm::RuntimeErr::SqlxError(error))
                            | DbErr::Query(sea_orm::RuntimeErr::SqlxError(error)),
                        ) if error
                            .as_database_error()
                            .and_then(|error| error.code())
                            .as_deref()
                            == Some("40001") =>
                        {
                            // Retry (don't break the loop)
                        }
                        error @ _ => return Err(error),
                    },
                }
            }
        };

        #[cfg(test)]
        {
            if let Some(background) = self.background.as_ref() {
                background.simulate_random_delay().await;
            }

            self.runtime.as_ref().unwrap().block_on(body)
        }

        #[cfg(not(test))]
        {
            body.await
        }
    }
}

pub struct RoomGuard<T> {
    data: T,
    _guard: OwnedMutexGuard<()>,
    _not_send: PhantomData<Rc<()>>,
}

impl<T> Deref for RoomGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for RoomGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
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

        impl From<$name> for sea_query::Value {
            fn from(value: $name) -> Self {
                sea_query::Value::Int(Some(value.0))
            }
        }

        impl sea_orm::TryGetable for $name {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                Ok(Self(i32::try_get(res, pre, col)?))
            }
        }

        impl sea_query::ValueType for $name {
            fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
                match v {
                    Value::TinyInt(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::SmallInt(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::Int(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::BigInt(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::TinyUnsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::SmallUnsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::Unsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    Value::BigUnsigned(Some(int)) => {
                        Ok(Self(int.try_into().map_err(|_| sea_query::ValueTypeErr)?))
                    }
                    _ => Err(sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name).into()
            }

            fn array_type() -> sea_query::ArrayType {
                sea_query::ArrayType::Int
            }

            fn column_type() -> sea_query::ColumnType {
                sea_query::ColumnType::Integer(None)
            }
        }

        impl sea_orm::TryFromU64 for $name {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                Ok(Self(n.try_into().map_err(|_| {
                    DbErr::ConvertFromU64(concat!(
                        "error converting ",
                        stringify!($name),
                        " to u64"
                    ))
                })?))
            }
        }

        impl sea_query::Nullable for $name {
            fn null() -> Value {
                Value::Int(None)
            }
        }
    };
}

id_type!(AccessTokenId);
id_type!(ContactId);
id_type!(RoomId);
id_type!(RoomParticipantId);
id_type!(ProjectId);
id_type!(ProjectCollaboratorId);
id_type!(ReplicaId);
id_type!(SignupId);
id_type!(UserId);
id_type!(WorktreeId);

pub struct LeftRoom {
    pub room: proto::Room,
    pub left_projects: HashMap<ProjectId, LeftProject>,
    pub canceled_calls_to_user_ids: Vec<UserId>,
}

pub struct Project {
    pub collaborators: Vec<project_collaborator::Model>,
    pub worktrees: BTreeMap<WorktreeId, Worktree>,
    pub language_servers: Vec<proto::LanguageServer>,
}

pub struct LeftProject {
    pub id: ProjectId,
    pub host_user_id: UserId,
    pub host_connection_id: ConnectionId,
    pub connection_ids: Vec<ConnectionId>,
}

pub struct Worktree {
    pub id: WorktreeId,
    pub abs_path: String,
    pub root_name: String,
    pub visible: bool,
    pub entries: Vec<proto::Entry>,
    pub diagnostic_summaries: Vec<proto::DiagnosticSummary>,
    pub scan_id: u64,
    pub is_complete: bool,
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
    use sea_orm::ConnectionTrait;
    use sqlx::migrate::MigrateDatabase;
    use std::sync::Arc;

    pub struct TestDb {
        pub db: Option<Arc<Database>>,
        pub connection: Option<sqlx::AnyConnection>,
    }

    impl TestDb {
        pub fn sqlite(background: Arc<Background>) -> Self {
            let url = format!("sqlite::memory:");
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();

            let mut db = runtime.block_on(async {
                let mut options = ConnectOptions::new(url);
                options.max_connections(5);
                let db = Database::new(options).await.unwrap();
                let sql = include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/migrations.sqlite/20221109000000_test_schema.sql"
                ));
                db.pool
                    .execute(sea_orm::Statement::from_string(
                        db.pool.get_database_backend(),
                        sql.into(),
                    ))
                    .await
                    .unwrap();
                db
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
                connection: None,
            }
        }

        pub fn postgres(background: Arc<Background>) -> Self {
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
                let mut options = ConnectOptions::new(url);
                options
                    .max_connections(5)
                    .idle_timeout(Duration::from_secs(0));
                let db = Database::new(options).await.unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
                db.migrate(Path::new(migrations_path), false).await.unwrap();
                db
            });

            db.background = Some(background);
            db.runtime = Some(runtime);

            Self {
                db: Some(Arc::new(db)),
                connection: None,
            }
        }

        pub fn db(&self) -> &Arc<Database> {
            self.db.as_ref().unwrap()
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            let db = self.db.take().unwrap();
            if let DatabaseBackend::Postgres = db.pool.get_database_backend() {
                db.runtime.as_ref().unwrap().block_on(async {
                    use util::ResultExt;
                    let query = "
                        SELECT pg_terminate_backend(pg_stat_activity.pid)
                        FROM pg_stat_activity
                        WHERE
                            pg_stat_activity.datname = current_database() AND
                            pid <> pg_backend_pid();
                    ";
                    db.pool
                        .execute(sea_orm::Statement::from_string(
                            db.pool.get_database_backend(),
                            query.into(),
                        ))
                        .await
                        .log_err();
                    sqlx::Postgres::drop_database(db.options.get_url())
                        .await
                        .log_err();
                })
            }
        }
    }
}
