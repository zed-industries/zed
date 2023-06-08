mod access_token;
mod contact;
mod follower;
mod language_server;
mod project;
mod project_collaborator;
mod room;
mod room_participant;
mod server;
mod signup;
#[cfg(test)]
mod tests;
mod user;
mod worktree;
mod worktree_diagnostic_summary;
mod worktree_entry;
mod worktree_repository;
mod worktree_repository_statuses;
mod worktree_settings_file;

use crate::executor::Executor;
use crate::{Error, Result};
use anyhow::anyhow;
use collections::{BTreeMap, HashMap, HashSet};
pub use contact::Contact;
use dashmap::DashMap;
use futures::StreamExt;
use hyper::StatusCode;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use rpc::{proto, ConnectionId};
use sea_orm::Condition;
pub use sea_orm::ConnectOptions;
use sea_orm::{
    entity::prelude::*, ActiveValue, ConnectionTrait, DatabaseConnection, DatabaseTransaction,
    DbErr, FromQueryResult, IntoActiveModel, IsolationLevel, JoinType, QueryOrder, QuerySelect,
    Statement, TransactionTrait,
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
    rng: Mutex<StdRng>,
    executor: Executor,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

impl Database {
    pub async fn new(options: ConnectOptions, executor: Executor) -> Result<Self> {
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options).await?,
            rooms: DashMap::with_capacity(16384),
            rng: Mutex::new(StdRng::seed_from_u64(0)),
            executor,
            #[cfg(test)]
            runtime: None,
        })
    }

    #[cfg(test)]
    pub fn reset(&self) {
        self.rooms.clear();
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

    pub async fn create_server(&self, environment: &str) -> Result<ServerId> {
        self.transaction(|tx| async move {
            let server = server::ActiveModel {
                environment: ActiveValue::set(environment.into()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            Ok(server.id)
        })
        .await
    }

    pub async fn stale_room_ids(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<Vec<RoomId>> {
        self.transaction(|tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryAs {
                RoomId,
            }

            let stale_server_epochs = self
                .stale_server_ids(environment, new_server_id, &tx)
                .await?;
            Ok(room_participant::Entity::find()
                .select_only()
                .column(room_participant::Column::RoomId)
                .distinct()
                .filter(
                    room_participant::Column::AnsweringConnectionServerId
                        .is_in(stale_server_epochs),
                )
                .into_values::<_, QueryAs>()
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn refresh_room(
        &self,
        room_id: RoomId,
        new_server_id: ServerId,
    ) -> Result<RoomGuard<RefreshedRoom>> {
        self.room_transaction(room_id, |tx| async move {
            let stale_participant_filter = Condition::all()
                .add(room_participant::Column::RoomId.eq(room_id))
                .add(room_participant::Column::AnsweringConnectionId.is_not_null())
                .add(room_participant::Column::AnsweringConnectionServerId.ne(new_server_id));

            let stale_participant_user_ids = room_participant::Entity::find()
                .filter(stale_participant_filter.clone())
                .all(&*tx)
                .await?
                .into_iter()
                .map(|participant| participant.user_id)
                .collect::<Vec<_>>();

            // Delete participants who failed to reconnect and cancel their calls.
            let mut canceled_calls_to_user_ids = Vec::new();
            room_participant::Entity::delete_many()
                .filter(stale_participant_filter)
                .exec(&*tx)
                .await?;
            let called_participants = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::CallingUserId
                                .is_in(stale_participant_user_ids.iter().copied()),
                        )
                        .add(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .all(&*tx)
                .await?;
            room_participant::Entity::delete_many()
                .filter(
                    room_participant::Column::Id
                        .is_in(called_participants.iter().map(|participant| participant.id)),
                )
                .exec(&*tx)
                .await?;
            canceled_calls_to_user_ids.extend(
                called_participants
                    .into_iter()
                    .map(|participant| participant.user_id),
            );

            let room = self.get_room(room_id, &tx).await?;
            // Delete the room if it becomes empty.
            if room.participants.is_empty() {
                project::Entity::delete_many()
                    .filter(project::Column::RoomId.eq(room_id))
                    .exec(&*tx)
                    .await?;
                room::Entity::delete_by_id(room_id).exec(&*tx).await?;
            }

            Ok(RefreshedRoom {
                room,
                stale_participant_user_ids,
                canceled_calls_to_user_ids,
            })
        })
        .await
    }

    pub async fn delete_stale_servers(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            server::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(server::Column::Environment.eq(environment))
                        .add(server::Column::Id.ne(new_server_id)),
                )
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    async fn stale_server_ids(
        &self,
        environment: &str,
        new_server_id: ServerId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ServerId>> {
        let stale_servers = server::Entity::find()
            .filter(
                Condition::all()
                    .add(server::Column::Environment.eq(environment))
                    .add(server::Column::Id.ne(new_server_id)),
            )
            .all(&*tx)
            .await?;
        Ok(stale_servers.into_iter().map(|server| server.id).collect())
    }

    // users

    pub async fn create_user(
        &self,
        email_address: &str,
        admin: bool,
        params: NewUserParams,
    ) -> Result<NewUserResult> {
        self.transaction(|tx| async {
            let tx = tx;
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
            .exec_with_returning(&*tx)
            .await?;

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
        self.transaction(|tx| async move { Ok(user::Entity::find_by_id(id).one(&*tx).await?) })
            .await
    }

    pub async fn get_users_by_ids(&self, ids: Vec<UserId>) -> Result<Vec<user::Model>> {
        self.transaction(|tx| async {
            let tx = tx;
            Ok(user::Entity::find()
                .filter(user::Column::Id.is_in(ids.iter().copied()))
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn get_user_by_github_login(&self, github_login: &str) -> Result<Option<User>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .filter(user::Column::GithubLogin.eq(github_login))
                .one(&*tx)
                .await?)
        })
        .await
    }

    pub async fn get_or_create_user_by_github_account(
        &self,
        github_login: &str,
        github_user_id: Option<i32>,
        github_email: Option<&str>,
    ) -> Result<Option<User>> {
        self.transaction(|tx| async move {
            let tx = &*tx;
            if let Some(github_user_id) = github_user_id {
                if let Some(user_by_github_user_id) = user::Entity::find()
                    .filter(user::Column::GithubUserId.eq(github_user_id))
                    .one(tx)
                    .await?
                {
                    let mut user_by_github_user_id = user_by_github_user_id.into_active_model();
                    user_by_github_user_id.github_login = ActiveValue::set(github_login.into());
                    Ok(Some(user_by_github_user_id.update(tx).await?))
                } else if let Some(user_by_github_login) = user::Entity::find()
                    .filter(user::Column::GithubLogin.eq(github_login))
                    .one(tx)
                    .await?
                {
                    let mut user_by_github_login = user_by_github_login.into_active_model();
                    user_by_github_login.github_user_id = ActiveValue::set(Some(github_user_id));
                    Ok(Some(user_by_github_login.update(tx).await?))
                } else {
                    let user = user::Entity::insert(user::ActiveModel {
                        email_address: ActiveValue::set(github_email.map(|email| email.into())),
                        github_login: ActiveValue::set(github_login.into()),
                        github_user_id: ActiveValue::set(Some(github_user_id)),
                        admin: ActiveValue::set(false),
                        invite_count: ActiveValue::set(0),
                        invite_code: ActiveValue::set(None),
                        metrics_id: ActiveValue::set(Uuid::new_v4()),
                        ..Default::default()
                    })
                    .exec_with_returning(&*tx)
                    .await?;
                    Ok(Some(user))
                }
            } else {
                Ok(user::Entity::find()
                    .filter(user::Column::GithubLogin.eq(github_login))
                    .one(tx)
                    .await?)
            }
        })
        .await
    }

    pub async fn get_all_users(&self, page: u32, limit: u32) -> Result<Vec<User>> {
        self.transaction(|tx| async move {
            Ok(user::Entity::find()
                .order_by_asc(user::Column::GithubLogin)
                .limit(limit as u64)
                .offset(page as u64 * limit as u64)
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn get_users_with_no_invites(
        &self,
        invited_by_another_user: bool,
    ) -> Result<Vec<User>> {
        self.transaction(|tx| async move {
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
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn get_user_metrics_id(&self, id: UserId) -> Result<String> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            MetricsId,
        }

        self.transaction(|tx| async move {
            let metrics_id: Uuid = user::Entity::find_by_id(id)
                .select_only()
                .column(user::Column::MetricsId)
                .into_values::<_, QueryAs>()
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("could not find user"))?;
            Ok(metrics_id.to_string())
        })
        .await
    }

    pub async fn set_user_is_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        self.transaction(|tx| async move {
            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .set(user::ActiveModel {
                    admin: ActiveValue::set(is_admin),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    pub async fn set_user_connected_once(&self, id: UserId, connected_once: bool) -> Result<()> {
        self.transaction(|tx| async move {
            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .set(user::ActiveModel {
                    connected_once: ActiveValue::set(connected_once),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    pub async fn destroy_user(&self, id: UserId) -> Result<()> {
        self.transaction(|tx| async move {
            access_token::Entity::delete_many()
                .filter(access_token::Column::UserId.eq(id))
                .exec(&*tx)
                .await?;
            user::Entity::delete_by_id(id).exec(&*tx).await?;
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

        self.transaction(|tx| async move {
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
                .stream(&*tx)
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
        self.transaction(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(room_participant::Column::UserId.eq(user_id))
                .one(&*tx)
                .await?;
            Ok(participant.is_some())
        })
        .await
    }

    pub async fn has_contact(&self, user_id_1: UserId, user_id_2: UserId) -> Result<bool> {
        self.transaction(|tx| async move {
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
                .one(&*tx)
                .await?
                .is_some())
        })
        .await
    }

    pub async fn send_contact_request(&self, sender_id: UserId, receiver_id: UserId) -> Result<()> {
        self.transaction(|tx| async move {
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
            .exec_without_returning(&*tx)
            .await?;

            if rows_affected == 1 {
                Ok(())
            } else {
                Err(anyhow!("contact already requested"))?
            }
        })
        .await
    }

    /// Returns a bool indicating whether the removed contact had originally accepted or not
    ///
    /// Deletes the contact identified by the requester and responder ids, and then returns
    /// whether the deleted contact had originally accepted or was a pending contact request.
    ///
    /// # Arguments
    ///
    /// * `requester_id` - The user that initiates this request
    /// * `responder_id` - The user that will be removed
    pub async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<bool> {
        self.transaction(|tx| async move {
            let (id_a, id_b) = if responder_id < requester_id {
                (responder_id, requester_id)
            } else {
                (requester_id, responder_id)
            };

            let contact = contact::Entity::find()
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such contact"))?;

            contact::Entity::delete_by_id(contact.id).exec(&*tx).await?;
            Ok(contact.accepted)
        })
        .await
    }

    pub async fn dismiss_contact_notification(
        &self,
        user_id: UserId,
        contact_user_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
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
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("no such contact request"))?
            } else {
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
        self.transaction(|tx| async move {
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
                    .exec(&*tx)
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
                    .exec(&*tx)
                    .await?;

                result.rows_affected
            };

            if rows_affected == 1 {
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
        self.transaction(|tx| async {
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
                .all(&*tx)
                .await?)
        })
        .await
    }

    // signups

    pub async fn create_signup(&self, signup: &NewSignup) -> Result<()> {
        self.transaction(|tx| async move {
            signup::Entity::insert(signup::ActiveModel {
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
                added_to_mailing_list: ActiveValue::set(signup.added_to_mailing_list),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(signup::Column::EmailAddress)
                    .update_columns([
                        signup::Column::PlatformMac,
                        signup::Column::PlatformWindows,
                        signup::Column::PlatformLinux,
                        signup::Column::EditorFeatures,
                        signup::Column::ProgrammingLanguages,
                        signup::Column::DeviceId,
                        signup::Column::AddedToMailingList,
                    ])
                    .to_owned(),
            )
            .exec(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_signup(&self, email_address: &str) -> Result<signup::Model> {
        self.transaction(|tx| async move {
            let signup = signup::Entity::find()
                .filter(signup::Column::EmailAddress.eq(email_address))
                .one(&*tx)
                .await?
                .ok_or_else(|| {
                    anyhow!("signup with email address {} doesn't exist", email_address)
                })?;

            Ok(signup)
        })
        .await
    }

    pub async fn get_waitlist_summary(&self) -> Result<WaitlistSummary> {
        self.transaction(|tx| async move {
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
                .one(&*tx)
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
        self.transaction(|tx| async {
            let tx = tx;
            signup::Entity::update_many()
                .filter(signup::Column::EmailAddress.is_in(emails.iter().copied()))
                .set(signup::ActiveModel {
                    email_confirmation_sent: ActiveValue::set(true),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_unsent_invites(&self, count: usize) -> Result<Vec<Invite>> {
        self.transaction(|tx| async move {
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
                .order_by_asc(signup::Column::CreatedAt)
                .limit(count as u64)
                .into_model()
                .all(&*tx)
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
        added_to_mailing_list: bool,
    ) -> Result<Invite> {
        self.transaction(|tx| async move {
            let existing_user = user::Entity::find()
                .filter(user::Column::EmailAddress.eq(email_address))
                .one(&*tx)
                .await?;

            if existing_user.is_some() {
                Err(anyhow!("email address is already in use"))?;
            }

            let inviting_user_with_invites = match user::Entity::find()
                .filter(
                    user::Column::InviteCode
                        .eq(code)
                        .and(user::Column::InviteCount.gt(0)),
                )
                .one(&*tx)
                .await?
            {
                Some(inviting_user) => inviting_user,
                None => {
                    return Err(Error::Http(
                        StatusCode::UNAUTHORIZED,
                        "unable to find an invite code with invites remaining".to_string(),
                    ))?
                }
            };
            user::Entity::update_many()
                .filter(
                    user::Column::Id
                        .eq(inviting_user_with_invites.id)
                        .and(user::Column::InviteCount.gt(0)),
                )
                .col_expr(
                    user::Column::InviteCount,
                    Expr::col(user::Column::InviteCount).sub(1),
                )
                .exec(&*tx)
                .await?;

            let signup = signup::Entity::insert(signup::ActiveModel {
                email_address: ActiveValue::set(email_address.into()),
                email_confirmation_code: ActiveValue::set(random_email_confirmation_code()),
                email_confirmation_sent: ActiveValue::set(false),
                inviting_user_id: ActiveValue::set(Some(inviting_user_with_invites.id)),
                platform_linux: ActiveValue::set(false),
                platform_mac: ActiveValue::set(false),
                platform_windows: ActiveValue::set(false),
                platform_unknown: ActiveValue::set(true),
                device_id: ActiveValue::set(device_id.map(|device_id| device_id.into())),
                added_to_mailing_list: ActiveValue::set(added_to_mailing_list),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(signup::Column::EmailAddress)
                    .update_column(signup::Column::InvitingUserId)
                    .to_owned(),
            )
            .exec_with_returning(&*tx)
            .await?;

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
        self.transaction(|tx| async {
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
                .one(&*tx)
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
            .exec_with_returning(&*tx)
            .await?;

            let mut signup = signup.into_active_model();
            signup.user_id = ActiveValue::set(Some(user.id));
            let signup = signup.update(&*tx).await?;

            if let Some(inviting_user_id) = signup.inviting_user_id {
                let (user_id_a, user_id_b, a_to_b) = if inviting_user_id < user.id {
                    (inviting_user_id, user.id, true)
                } else {
                    (user.id, inviting_user_id, false)
                };

                contact::Entity::insert(contact::ActiveModel {
                    user_id_a: ActiveValue::set(user_id_a),
                    user_id_b: ActiveValue::set(user_id_b),
                    a_to_b: ActiveValue::set(a_to_b),
                    should_notify: ActiveValue::set(true),
                    accepted: ActiveValue::set(true),
                    ..Default::default()
                })
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&*tx)
                .await?;
            }

            Ok(Some(NewUserResult {
                user_id: user.id,
                metrics_id: user.metrics_id.to_string(),
                inviting_user_id: signup.inviting_user_id,
                signup_device_id: signup.device_id,
            }))
        })
        .await
    }

    pub async fn set_invite_count_for_user(&self, id: UserId, count: i32) -> Result<()> {
        self.transaction(|tx| async move {
            if count > 0 {
                user::Entity::update_many()
                    .filter(
                        user::Column::Id
                            .eq(id)
                            .and(user::Column::InviteCode.is_null()),
                    )
                    .set(user::ActiveModel {
                        invite_code: ActiveValue::set(Some(random_invite_code())),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?;
            }

            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .set(user::ActiveModel {
                    invite_count: ActiveValue::set(count),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_invite_code_for_user(&self, id: UserId) -> Result<Option<(String, i32)>> {
        self.transaction(|tx| async move {
            match user::Entity::find_by_id(id).one(&*tx).await? {
                Some(user) if user.invite_code.is_some() => {
                    Ok(Some((user.invite_code.unwrap(), user.invite_count)))
                }
                _ => Ok(None),
            }
        })
        .await
    }

    pub async fn get_user_for_invite_code(&self, code: &str) -> Result<User> {
        self.transaction(|tx| async move {
            user::Entity::find()
                .filter(user::Column::InviteCode.eq(code))
                .one(&*tx)
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
        self.transaction(|tx| async move {
            let pending_participant = room_participant::Entity::find()
                .filter(
                    room_participant::Column::UserId
                        .eq(user_id)
                        .and(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&*tx)
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
        connection: ConnectionId,
        live_kit_room: &str,
    ) -> Result<proto::Room> {
        self.transaction(|tx| async move {
            let room = room::ActiveModel {
                live_kit_room: ActiveValue::set(live_kit_room.into()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            room_participant::ActiveModel {
                room_id: ActiveValue::set(room.id),
                user_id: ActiveValue::set(user_id),
                answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
                answering_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                answering_connection_lost: ActiveValue::set(false),
                calling_user_id: ActiveValue::set(user_id),
                calling_connection_id: ActiveValue::set(connection.id as i32),
                calling_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room.id, &tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn call(
        &self,
        room_id: RoomId,
        calling_user_id: UserId,
        calling_connection: ConnectionId,
        called_user_id: UserId,
        initial_project_id: Option<ProjectId>,
    ) -> Result<RoomGuard<(proto::Room, proto::IncomingCall)>> {
        self.room_transaction(room_id, |tx| async move {
            room_participant::ActiveModel {
                room_id: ActiveValue::set(room_id),
                user_id: ActiveValue::set(called_user_id),
                answering_connection_lost: ActiveValue::set(false),
                calling_user_id: ActiveValue::set(calling_user_id),
                calling_connection_id: ActiveValue::set(calling_connection.id as i32),
                calling_connection_server_id: ActiveValue::set(Some(ServerId(
                    calling_connection.owner_id as i32,
                ))),
                initial_project_id: ActiveValue::set(initial_project_id),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            let incoming_call = Self::build_incoming_call(&room, called_user_id)
                .ok_or_else(|| anyhow!("failed to build incoming call"))?;
            Ok((room, incoming_call))
        })
        .await
    }

    pub async fn call_failed(
        &self,
        room_id: RoomId,
        called_user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            room_participant::Entity::delete_many()
                .filter(
                    room_participant::Column::RoomId
                        .eq(room_id)
                        .and(room_participant::Column::UserId.eq(called_user_id)),
                )
                .exec(&*tx)
                .await?;
            let room = self.get_room(room_id, &tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn decline_call(
        &self,
        expected_room_id: Option<RoomId>,
        user_id: UserId,
    ) -> Result<Option<RoomGuard<proto::Room>>> {
        self.optional_room_transaction(|tx| async move {
            let mut filter = Condition::all()
                .add(room_participant::Column::UserId.eq(user_id))
                .add(room_participant::Column::AnsweringConnectionId.is_null());
            if let Some(room_id) = expected_room_id {
                filter = filter.add(room_participant::Column::RoomId.eq(room_id));
            }
            let participant = room_participant::Entity::find()
                .filter(filter)
                .one(&*tx)
                .await?;

            let participant = if let Some(participant) = participant {
                participant
            } else if expected_room_id.is_some() {
                return Err(anyhow!("could not find call to decline"))?;
            } else {
                return Ok(None);
            };

            let room_id = participant.room_id;
            room_participant::Entity::delete(participant.into_active_model())
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok(Some((room_id, room)))
        })
        .await
    }

    pub async fn cancel_call(
        &self,
        room_id: RoomId,
        calling_connection: ConnectionId,
        called_user_id: UserId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::UserId.eq(called_user_id))
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(
                            room_participant::Column::CallingConnectionId
                                .eq(calling_connection.id as i32),
                        )
                        .add(
                            room_participant::Column::CallingConnectionServerId
                                .eq(calling_connection.owner_id as i32),
                        )
                        .add(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no call to cancel"))?;

            room_participant::Entity::delete(participant.into_active_model())
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn join_room(
        &self,
        room_id: RoomId,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async move {
            let result = room_participant::Entity::update_many()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(room_participant::Column::UserId.eq(user_id))
                        .add(room_participant::Column::AnsweringConnectionId.is_null()),
                )
                .set(room_participant::ActiveModel {
                    answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    answering_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    answering_connection_lost: ActiveValue::set(false),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("room does not exist or was already joined"))?
            } else {
                let room = self.get_room(room_id, &tx).await?;
                Ok(room)
            }
        })
        .await
    }

    pub async fn rejoin_room(
        &self,
        rejoin_room: proto::RejoinRoom,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<RejoinedRoom>> {
        let room_id = RoomId::from_proto(rejoin_room.id);
        self.room_transaction(room_id, |tx| async {
            let tx = tx;
            let participant_update = room_participant::Entity::update_many()
                .filter(
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(room_participant::Column::UserId.eq(user_id))
                        .add(room_participant::Column::AnsweringConnectionId.is_not_null())
                        .add(
                            Condition::any()
                                .add(room_participant::Column::AnsweringConnectionLost.eq(true))
                                .add(
                                    room_participant::Column::AnsweringConnectionServerId
                                        .ne(connection.owner_id as i32),
                                ),
                        ),
                )
                .set(room_participant::ActiveModel {
                    answering_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    answering_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    answering_connection_lost: ActiveValue::set(false),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            if participant_update.rows_affected == 0 {
                return Err(anyhow!("room does not exist or was already joined"))?;
            }

            let mut reshared_projects = Vec::new();
            for reshared_project in &rejoin_room.reshared_projects {
                let project_id = ProjectId::from_proto(reshared_project.project_id);
                let project = project::Entity::find_by_id(project_id)
                    .one(&*tx)
                    .await?
                    .ok_or_else(|| anyhow!("project does not exist"))?;
                if project.host_user_id != user_id {
                    return Err(anyhow!("no such project"))?;
                }

                let mut collaborators = project
                    .find_related(project_collaborator::Entity)
                    .all(&*tx)
                    .await?;
                let host_ix = collaborators
                    .iter()
                    .position(|collaborator| {
                        collaborator.user_id == user_id && collaborator.is_host
                    })
                    .ok_or_else(|| anyhow!("host not found among collaborators"))?;
                let host = collaborators.swap_remove(host_ix);
                let old_connection_id = host.connection();

                project::Entity::update(project::ActiveModel {
                    host_connection_id: ActiveValue::set(Some(connection.id as i32)),
                    host_connection_server_id: ActiveValue::set(Some(ServerId(
                        connection.owner_id as i32,
                    ))),
                    ..project.into_active_model()
                })
                .exec(&*tx)
                .await?;
                project_collaborator::Entity::update(project_collaborator::ActiveModel {
                    connection_id: ActiveValue::set(connection.id as i32),
                    connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                    ..host.into_active_model()
                })
                .exec(&*tx)
                .await?;

                self.update_project_worktrees(project_id, &reshared_project.worktrees, &tx)
                    .await?;

                reshared_projects.push(ResharedProject {
                    id: project_id,
                    old_connection_id,
                    collaborators: collaborators
                        .iter()
                        .map(|collaborator| ProjectCollaborator {
                            connection_id: collaborator.connection(),
                            user_id: collaborator.user_id,
                            replica_id: collaborator.replica_id,
                            is_host: collaborator.is_host,
                        })
                        .collect(),
                    worktrees: reshared_project.worktrees.clone(),
                });
            }

            project::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(project::Column::RoomId.eq(room_id))
                        .add(project::Column::HostUserId.eq(user_id))
                        .add(
                            project::Column::Id
                                .is_not_in(reshared_projects.iter().map(|project| project.id)),
                        ),
                )
                .exec(&*tx)
                .await?;

            let mut rejoined_projects = Vec::new();
            for rejoined_project in &rejoin_room.rejoined_projects {
                let project_id = ProjectId::from_proto(rejoined_project.id);
                let Some(project) = project::Entity::find_by_id(project_id)
                    .one(&*tx)
                    .await? else { continue };

                let mut worktrees = Vec::new();
                let db_worktrees = project.find_related(worktree::Entity).all(&*tx).await?;
                for db_worktree in db_worktrees {
                    let mut worktree = RejoinedWorktree {
                        id: db_worktree.id as u64,
                        abs_path: db_worktree.abs_path,
                        root_name: db_worktree.root_name,
                        visible: db_worktree.visible,
                        updated_entries: Default::default(),
                        removed_entries: Default::default(),
                        updated_repositories: Default::default(),
                        removed_repositories: Default::default(),
                        diagnostic_summaries: Default::default(),
                        settings_files: Default::default(),
                        scan_id: db_worktree.scan_id as u64,
                        completed_scan_id: db_worktree.completed_scan_id as u64,
                    };

                    let rejoined_worktree = rejoined_project
                        .worktrees
                        .iter()
                        .find(|worktree| worktree.id == db_worktree.id as u64);

                    // File entries
                    {
                        let entry_filter = if let Some(rejoined_worktree) = rejoined_worktree {
                            worktree_entry::Column::ScanId.gt(rejoined_worktree.scan_id)
                        } else {
                            worktree_entry::Column::IsDeleted.eq(false)
                        };

                        let mut db_entries = worktree_entry::Entity::find()
                            .filter(
                                Condition::all()
                                    .add(worktree_entry::Column::ProjectId.eq(project.id))
                                    .add(worktree_entry::Column::WorktreeId.eq(worktree.id))
                                    .add(entry_filter),
                            )
                            .stream(&*tx)
                            .await?;

                        while let Some(db_entry) = db_entries.next().await {
                            let db_entry = db_entry?;
                            if db_entry.is_deleted {
                                worktree.removed_entries.push(db_entry.id as u64);
                            } else {
                                worktree.updated_entries.push(proto::Entry {
                                    id: db_entry.id as u64,
                                    is_dir: db_entry.is_dir,
                                    path: db_entry.path,
                                    inode: db_entry.inode as u64,
                                    mtime: Some(proto::Timestamp {
                                        seconds: db_entry.mtime_seconds as u64,
                                        nanos: db_entry.mtime_nanos as u32,
                                    }),
                                    is_symlink: db_entry.is_symlink,
                                    is_ignored: db_entry.is_ignored,
                                    git_status: db_entry.git_status.map(|status| status as i32),
                                });
                            }
                        }
                    }

                    // Repository Entries
                    {
                        let repository_entry_filter =
                            if let Some(rejoined_worktree) = rejoined_worktree {
                                worktree_repository::Column::ScanId.gt(rejoined_worktree.scan_id)
                            } else {
                                worktree_repository::Column::IsDeleted.eq(false)
                            };

                        let mut db_repositories = worktree_repository::Entity::find()
                            .filter(
                                Condition::all()
                                    .add(worktree_repository::Column::ProjectId.eq(project.id))
                                    .add(worktree_repository::Column::WorktreeId.eq(worktree.id))
                                    .add(repository_entry_filter),
                            )
                            .stream(&*tx)
                            .await?;

                        while let Some(db_repository) = db_repositories.next().await {
                            let db_repository = db_repository?;
                            if db_repository.is_deleted {
                                worktree
                                    .removed_repositories
                                    .push(db_repository.work_directory_id as u64);
                            } else {
                                worktree.updated_repositories.push(proto::RepositoryEntry {
                                    work_directory_id: db_repository.work_directory_id as u64,
                                    branch: db_repository.branch,
                                });
                            }
                        }
                    }

                    worktrees.push(worktree);
                }

                let language_servers = project
                    .find_related(language_server::Entity)
                    .all(&*tx)
                    .await?
                    .into_iter()
                    .map(|language_server| proto::LanguageServer {
                        id: language_server.id as u64,
                        name: language_server.name,
                    })
                    .collect::<Vec<_>>();

                {
                    let mut db_settings_files = worktree_settings_file::Entity::find()
                        .filter(worktree_settings_file::Column::ProjectId.eq(project_id))
                        .stream(&*tx)
                        .await?;
                    while let Some(db_settings_file) = db_settings_files.next().await {
                        let db_settings_file = db_settings_file?;
                        if let Some(worktree) = worktrees
                            .iter_mut()
                            .find(|w| w.id == db_settings_file.worktree_id as u64)
                        {
                            worktree.settings_files.push(WorktreeSettingsFile {
                                path: db_settings_file.path,
                                content: db_settings_file.content,
                            });
                        }
                    }
                }

                let mut collaborators = project
                    .find_related(project_collaborator::Entity)
                    .all(&*tx)
                    .await?;
                let self_collaborator = if let Some(self_collaborator_ix) = collaborators
                    .iter()
                    .position(|collaborator| collaborator.user_id == user_id)
                {
                    collaborators.swap_remove(self_collaborator_ix)
                } else {
                    continue;
                };
                let old_connection_id = self_collaborator.connection();
                project_collaborator::Entity::update(project_collaborator::ActiveModel {
                    connection_id: ActiveValue::set(connection.id as i32),
                    connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                    ..self_collaborator.into_active_model()
                })
                .exec(&*tx)
                .await?;

                let collaborators = collaborators
                    .into_iter()
                    .map(|collaborator| ProjectCollaborator {
                        connection_id: collaborator.connection(),
                        user_id: collaborator.user_id,
                        replica_id: collaborator.replica_id,
                        is_host: collaborator.is_host,
                    })
                    .collect::<Vec<_>>();

                rejoined_projects.push(RejoinedProject {
                    id: project_id,
                    old_connection_id,
                    collaborators,
                    worktrees,
                    language_servers,
                });
            }

            let room = self.get_room(room_id, &tx).await?;
            Ok(RejoinedRoom {
                room,
                rejoined_projects,
                reshared_projects,
            })
        })
        .await
    }

    pub async fn leave_room(
        &self,
        connection: ConnectionId,
    ) -> Result<Option<RoomGuard<LeftRoom>>> {
        self.optional_room_transaction(|tx| async move {
            let leaving_participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?;

            if let Some(leaving_participant) = leaving_participant {
                // Leave room.
                let room_id = leaving_participant.room_id;
                room_participant::Entity::delete_by_id(leaving_participant.id)
                    .exec(&*tx)
                    .await?;

                // Cancel pending calls initiated by the leaving user.
                let called_participants = room_participant::Entity::find()
                    .filter(
                        Condition::all()
                            .add(
                                room_participant::Column::CallingUserId
                                    .eq(leaving_participant.user_id),
                            )
                            .add(room_participant::Column::AnsweringConnectionId.is_null()),
                    )
                    .all(&*tx)
                    .await?;
                room_participant::Entity::delete_many()
                    .filter(
                        room_participant::Column::Id
                            .is_in(called_participants.iter().map(|participant| participant.id)),
                    )
                    .exec(&*tx)
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
                    .filter(
                        Condition::all()
                            .add(
                                project_collaborator::Column::ConnectionId.eq(connection.id as i32),
                            )
                            .add(
                                project_collaborator::Column::ConnectionServerId
                                    .eq(connection.owner_id as i32),
                            ),
                    )
                    .into_values::<_, QueryProjectIds>()
                    .all(&*tx)
                    .await?;
                let mut left_projects = HashMap::default();
                let mut collaborators = project_collaborator::Entity::find()
                    .filter(project_collaborator::Column::ProjectId.is_in(project_ids))
                    .stream(&*tx)
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

                    let collaborator_connection_id = collaborator.connection();
                    if collaborator_connection_id != connection {
                        left_project.connection_ids.push(collaborator_connection_id);
                    }

                    if collaborator.is_host {
                        left_project.host_user_id = collaborator.user_id;
                        left_project.host_connection_id = collaborator_connection_id;
                    }
                }
                drop(collaborators);

                // Leave projects.
                project_collaborator::Entity::delete_many()
                    .filter(
                        Condition::all()
                            .add(
                                project_collaborator::Column::ConnectionId.eq(connection.id as i32),
                            )
                            .add(
                                project_collaborator::Column::ConnectionServerId
                                    .eq(connection.owner_id as i32),
                            ),
                    )
                    .exec(&*tx)
                    .await?;

                // Unshare projects.
                project::Entity::delete_many()
                    .filter(
                        Condition::all()
                            .add(project::Column::RoomId.eq(room_id))
                            .add(project::Column::HostConnectionId.eq(connection.id as i32))
                            .add(
                                project::Column::HostConnectionServerId
                                    .eq(connection.owner_id as i32),
                            ),
                    )
                    .exec(&*tx)
                    .await?;

                let room = self.get_room(room_id, &tx).await?;
                if room.participants.is_empty() {
                    room::Entity::delete_by_id(room_id).exec(&*tx).await?;
                }

                let left_room = LeftRoom {
                    room,
                    left_projects,
                    canceled_calls_to_user_ids,
                };

                if left_room.room.participants.is_empty() {
                    self.rooms.remove(&room_id);
                }

                Ok(Some((room_id, left_room)))
            } else {
                Ok(None)
            }
        })
        .await
    }

    pub async fn follow(
        &self,
        project_id: ProjectId,
        leader_connection: ConnectionId,
        follower_connection: ConnectionId,
    ) -> Result<RoomGuard<proto::Room>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            follower::ActiveModel {
                room_id: ActiveValue::set(room_id),
                project_id: ActiveValue::set(project_id),
                leader_connection_server_id: ActiveValue::set(ServerId(
                    leader_connection.owner_id as i32,
                )),
                leader_connection_id: ActiveValue::set(leader_connection.id as i32),
                follower_connection_server_id: ActiveValue::set(ServerId(
                    follower_connection.owner_id as i32,
                )),
                follower_connection_id: ActiveValue::set(follower_connection.id as i32),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &*tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn unfollow(
        &self,
        project_id: ProjectId,
        leader_connection: ConnectionId,
        follower_connection: ConnectionId,
    ) -> Result<RoomGuard<proto::Room>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            follower::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(follower::Column::ProjectId.eq(project_id))
                        .add(
                            follower::Column::LeaderConnectionServerId
                                .eq(leader_connection.owner_id),
                        )
                        .add(follower::Column::LeaderConnectionId.eq(leader_connection.id))
                        .add(
                            follower::Column::FollowerConnectionServerId
                                .eq(follower_connection.owner_id),
                        )
                        .add(follower::Column::FollowerConnectionId.eq(follower_connection.id)),
                )
                .exec(&*tx)
                .await?;

            let room = self.get_room(room_id, &*tx).await?;
            Ok(room)
        })
        .await
    }

    pub async fn update_room_participant_location(
        &self,
        room_id: RoomId,
        connection: ConnectionId,
        location: proto::ParticipantLocation,
    ) -> Result<RoomGuard<proto::Room>> {
        self.room_transaction(room_id, |tx| async {
            let tx = tx;
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
                    Condition::all()
                        .add(room_participant::Column::RoomId.eq(room_id))
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .set(room_participant::ActiveModel {
                    location_kind: ActiveValue::set(Some(location_kind)),
                    location_project_id: ActiveValue::set(location_project_id),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;

            if result.rows_affected == 1 {
                let room = self.get_room(room_id, &tx).await?;
                Ok(room)
            } else {
                Err(anyhow!("could not update room participant location"))?
            }
        })
        .await
    }

    pub async fn connection_lost(&self, connection: ConnectionId) -> Result<()> {
        self.transaction(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("not a participant in any room"))?;

            room_participant::Entity::update(room_participant::ActiveModel {
                answering_connection_lost: ActiveValue::set(true),
                ..participant.into_active_model()
            })
            .exec(&*tx)
            .await?;

            Ok(())
        })
        .await
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
            if let Some((answering_connection_id, answering_connection_server_id)) = db_participant
                .answering_connection_id
                .zip(db_participant.answering_connection_server_id)
            {
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

                let answering_connection = ConnectionId {
                    owner_id: answering_connection_server_id.0 as u32,
                    id: answering_connection_id as u32,
                };
                participants.insert(
                    answering_connection,
                    proto::Participant {
                        user_id: db_participant.user_id.to_proto(),
                        peer_id: Some(answering_connection.into()),
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
            let host_connection = db_project.host_connection()?;
            if let Some(participant) = participants.get_mut(&host_connection) {
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
                    if db_worktree.visible {
                        project.worktree_root_names.push(db_worktree.root_name);
                    }
                }
            }
        }
        drop(db_projects);

        let mut db_followers = db_room.find_related(follower::Entity).stream(tx).await?;
        let mut followers = Vec::new();
        while let Some(db_follower) = db_followers.next().await {
            let db_follower = db_follower?;
            followers.push(proto::Follower {
                leader_id: Some(db_follower.leader_connection().into()),
                follower_id: Some(db_follower.follower_connection().into()),
                project_id: db_follower.project_id.to_proto(),
            });
        }

        Ok(proto::Room {
            id: db_room.id.to_proto(),
            live_kit_room: db_room.live_kit_room,
            participants: participants.into_values().collect(),
            pending_participants,
            followers,
        })
    }

    // projects

    pub async fn project_count_excluding_admins(&self) -> Result<usize> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            Count,
        }

        self.transaction(|tx| async move {
            Ok(project::Entity::find()
                .select_only()
                .column_as(project::Column::Id.count(), QueryAs::Count)
                .inner_join(user::Entity)
                .filter(user::Column::Admin.eq(false))
                .into_values::<_, QueryAs>()
                .one(&*tx)
                .await?
                .unwrap_or(0i64) as usize)
        })
        .await
    }

    pub async fn share_project(
        &self,
        room_id: RoomId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(ProjectId, proto::Room)>> {
        self.room_transaction(room_id, |tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("could not find participant"))?;
            if participant.room_id != room_id {
                return Err(anyhow!("shared project on unexpected room"))?;
            }

            let project = project::ActiveModel {
                room_id: ActiveValue::set(participant.room_id),
                host_user_id: ActiveValue::set(participant.user_id),
                host_connection_id: ActiveValue::set(Some(connection.id as i32)),
                host_connection_server_id: ActiveValue::set(Some(ServerId(
                    connection.owner_id as i32,
                ))),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            if !worktrees.is_empty() {
                worktree::Entity::insert_many(worktrees.iter().map(|worktree| {
                    worktree::ActiveModel {
                        id: ActiveValue::set(worktree.id as i64),
                        project_id: ActiveValue::set(project.id),
                        abs_path: ActiveValue::set(worktree.abs_path.clone()),
                        root_name: ActiveValue::set(worktree.root_name.clone()),
                        visible: ActiveValue::set(worktree.visible),
                        scan_id: ActiveValue::set(0),
                        completed_scan_id: ActiveValue::set(0),
                    }
                }))
                .exec(&*tx)
                .await?;
            }

            project_collaborator::ActiveModel {
                project_id: ActiveValue::set(project.id),
                connection_id: ActiveValue::set(connection.id as i32),
                connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                user_id: ActiveValue::set(participant.user_id),
                replica_id: ActiveValue::set(ReplicaId(0)),
                is_host: ActiveValue::set(true),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            let room = self.get_room(room_id, &tx).await?;
            Ok((project.id, room))
        })
        .await
    }

    pub async fn unshare_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<(proto::Room, Vec<ConnectionId>)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let guest_connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("project not found"))?;
            if project.host_connection()? == connection {
                project::Entity::delete(project.into_active_model())
                    .exec(&*tx)
                    .await?;
                let room = self.get_room(room_id, &tx).await?;
                Ok((room, guest_connection_ids))
            } else {
                Err(anyhow!("cannot unshare a project hosted by another user"))?
            }
        })
        .await
    }

    pub async fn update_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
        worktrees: &[proto::WorktreeMetadata],
    ) -> Result<RoomGuard<(proto::Room, Vec<ConnectionId>)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let project = project::Entity::find_by_id(project_id)
                .filter(
                    Condition::all()
                        .add(project::Column::HostConnectionId.eq(connection.id as i32))
                        .add(
                            project::Column::HostConnectionServerId.eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;

            self.update_project_worktrees(project.id, worktrees, &tx)
                .await?;

            let guest_connection_ids = self.project_guest_connection_ids(project.id, &tx).await?;
            let room = self.get_room(project.room_id, &tx).await?;
            Ok((room, guest_connection_ids))
        })
        .await
    }

    async fn update_project_worktrees(
        &self,
        project_id: ProjectId,
        worktrees: &[proto::WorktreeMetadata],
        tx: &DatabaseTransaction,
    ) -> Result<()> {
        if !worktrees.is_empty() {
            worktree::Entity::insert_many(worktrees.iter().map(|worktree| worktree::ActiveModel {
                id: ActiveValue::set(worktree.id as i64),
                project_id: ActiveValue::set(project_id),
                abs_path: ActiveValue::set(worktree.abs_path.clone()),
                root_name: ActiveValue::set(worktree.root_name.clone()),
                visible: ActiveValue::set(worktree.visible),
                scan_id: ActiveValue::set(0),
                completed_scan_id: ActiveValue::set(0),
            }))
            .on_conflict(
                OnConflict::columns([worktree::Column::ProjectId, worktree::Column::Id])
                    .update_column(worktree::Column::RootName)
                    .to_owned(),
            )
            .exec(&*tx)
            .await?;
        }

        worktree::Entity::delete_many()
            .filter(worktree::Column::ProjectId.eq(project_id).and(
                worktree::Column::Id.is_not_in(worktrees.iter().map(|worktree| worktree.id as i64)),
            ))
            .exec(&*tx)
            .await?;

        Ok(())
    }

    pub async fn update_worktree(
        &self,
        update: &proto::UpdateWorktree,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let worktree_id = update.worktree_id as i64;
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            // Ensure the update comes from the host.
            let _project = project::Entity::find_by_id(project_id)
                .filter(
                    Condition::all()
                        .add(project::Column::HostConnectionId.eq(connection.id as i32))
                        .add(
                            project::Column::HostConnectionServerId.eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;

            // Update metadata.
            worktree::Entity::update(worktree::ActiveModel {
                id: ActiveValue::set(worktree_id),
                project_id: ActiveValue::set(project_id),
                root_name: ActiveValue::set(update.root_name.clone()),
                scan_id: ActiveValue::set(update.scan_id as i64),
                completed_scan_id: if update.is_last_update {
                    ActiveValue::set(update.scan_id as i64)
                } else {
                    ActiveValue::default()
                },
                abs_path: ActiveValue::set(update.abs_path.clone()),
                ..Default::default()
            })
            .exec(&*tx)
            .await?;

            if !update.updated_entries.is_empty() {
                worktree_entry::Entity::insert_many(update.updated_entries.iter().map(|entry| {
                    let mtime = entry.mtime.clone().unwrap_or_default();
                    worktree_entry::ActiveModel {
                        project_id: ActiveValue::set(project_id),
                        worktree_id: ActiveValue::set(worktree_id),
                        id: ActiveValue::set(entry.id as i64),
                        is_dir: ActiveValue::set(entry.is_dir),
                        path: ActiveValue::set(entry.path.clone()),
                        inode: ActiveValue::set(entry.inode as i64),
                        mtime_seconds: ActiveValue::set(mtime.seconds as i64),
                        mtime_nanos: ActiveValue::set(mtime.nanos as i32),
                        is_symlink: ActiveValue::set(entry.is_symlink),
                        is_ignored: ActiveValue::set(entry.is_ignored),
                        git_status: ActiveValue::set(entry.git_status.map(|status| status as i64)),
                        is_deleted: ActiveValue::set(false),
                        scan_id: ActiveValue::set(update.scan_id as i64),
                    }
                }))
                .on_conflict(
                    OnConflict::columns([
                        worktree_entry::Column::ProjectId,
                        worktree_entry::Column::WorktreeId,
                        worktree_entry::Column::Id,
                    ])
                    .update_columns([
                        worktree_entry::Column::IsDir,
                        worktree_entry::Column::Path,
                        worktree_entry::Column::Inode,
                        worktree_entry::Column::MtimeSeconds,
                        worktree_entry::Column::MtimeNanos,
                        worktree_entry::Column::IsSymlink,
                        worktree_entry::Column::IsIgnored,
                        worktree_entry::Column::GitStatus,
                        worktree_entry::Column::ScanId,
                    ])
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            }

            if !update.removed_entries.is_empty() {
                worktree_entry::Entity::update_many()
                    .filter(
                        worktree_entry::Column::ProjectId
                            .eq(project_id)
                            .and(worktree_entry::Column::WorktreeId.eq(worktree_id))
                            .and(
                                worktree_entry::Column::Id
                                    .is_in(update.removed_entries.iter().map(|id| *id as i64)),
                            ),
                    )
                    .set(worktree_entry::ActiveModel {
                        is_deleted: ActiveValue::Set(true),
                        scan_id: ActiveValue::Set(update.scan_id as i64),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?;
            }

            if !update.updated_repositories.is_empty() {
                worktree_repository::Entity::insert_many(update.updated_repositories.iter().map(
                    |repository| worktree_repository::ActiveModel {
                        project_id: ActiveValue::set(project_id),
                        worktree_id: ActiveValue::set(worktree_id),
                        work_directory_id: ActiveValue::set(repository.work_directory_id as i64),
                        scan_id: ActiveValue::set(update.scan_id as i64),
                        branch: ActiveValue::set(repository.branch.clone()),
                        is_deleted: ActiveValue::set(false),
                    },
                ))
                .on_conflict(
                    OnConflict::columns([
                        worktree_repository::Column::ProjectId,
                        worktree_repository::Column::WorktreeId,
                        worktree_repository::Column::WorkDirectoryId,
                    ])
                    .update_columns([
                        worktree_repository::Column::ScanId,
                        worktree_repository::Column::Branch,
                    ])
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            }

            if !update.removed_repositories.is_empty() {
                worktree_repository::Entity::update_many()
                    .filter(
                        worktree_repository::Column::ProjectId
                            .eq(project_id)
                            .and(worktree_repository::Column::WorktreeId.eq(worktree_id))
                            .and(
                                worktree_repository::Column::WorkDirectoryId
                                    .is_in(update.removed_repositories.iter().map(|id| *id as i64)),
                            ),
                    )
                    .set(worktree_repository::ActiveModel {
                        is_deleted: ActiveValue::Set(true),
                        scan_id: ActiveValue::Set(update.scan_id as i64),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?;
            }

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    pub async fn update_diagnostic_summary(
        &self,
        update: &proto::UpdateDiagnosticSummary,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let worktree_id = update.worktree_id as i64;
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let summary = update
                .summary
                .as_ref()
                .ok_or_else(|| anyhow!("invalid summary"))?;

            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            // Update summary.
            worktree_diagnostic_summary::Entity::insert(worktree_diagnostic_summary::ActiveModel {
                project_id: ActiveValue::set(project_id),
                worktree_id: ActiveValue::set(worktree_id),
                path: ActiveValue::set(summary.path.clone()),
                language_server_id: ActiveValue::set(summary.language_server_id as i64),
                error_count: ActiveValue::set(summary.error_count as i32),
                warning_count: ActiveValue::set(summary.warning_count as i32),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::columns([
                    worktree_diagnostic_summary::Column::ProjectId,
                    worktree_diagnostic_summary::Column::WorktreeId,
                    worktree_diagnostic_summary::Column::Path,
                ])
                .update_columns([
                    worktree_diagnostic_summary::Column::LanguageServerId,
                    worktree_diagnostic_summary::Column::ErrorCount,
                    worktree_diagnostic_summary::Column::WarningCount,
                ])
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    pub async fn start_language_server(
        &self,
        update: &proto::StartLanguageServer,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let server = update
                .server
                .as_ref()
                .ok_or_else(|| anyhow!("invalid language server"))?;

            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            // Add the newly-started language server.
            language_server::Entity::insert(language_server::ActiveModel {
                project_id: ActiveValue::set(project_id),
                id: ActiveValue::set(server.id as i64),
                name: ActiveValue::set(server.name.clone()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::columns([
                    language_server::Column::ProjectId,
                    language_server::Column::Id,
                ])
                .update_column(language_server::Column::Name)
                .to_owned(),
            )
            .exec(&*tx)
            .await?;

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    pub async fn update_worktree_settings(
        &self,
        update: &proto::UpdateWorktreeSettings,
        connection: ConnectionId,
    ) -> Result<RoomGuard<Vec<ConnectionId>>> {
        let project_id = ProjectId::from_proto(update.project_id);
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            // Ensure the update comes from the host.
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.host_connection()? != connection {
                return Err(anyhow!("can't update a project hosted by someone else"))?;
            }

            if let Some(content) = &update.content {
                worktree_settings_file::Entity::insert(worktree_settings_file::ActiveModel {
                    project_id: ActiveValue::Set(project_id),
                    worktree_id: ActiveValue::Set(update.worktree_id as i64),
                    path: ActiveValue::Set(update.path.clone()),
                    content: ActiveValue::Set(content.clone()),
                })
                .on_conflict(
                    OnConflict::columns([
                        worktree_settings_file::Column::ProjectId,
                        worktree_settings_file::Column::WorktreeId,
                        worktree_settings_file::Column::Path,
                    ])
                    .update_column(worktree_settings_file::Column::Content)
                    .to_owned(),
                )
                .exec(&*tx)
                .await?;
            } else {
                worktree_settings_file::Entity::delete(worktree_settings_file::ActiveModel {
                    project_id: ActiveValue::Set(project_id),
                    worktree_id: ActiveValue::Set(update.worktree_id as i64),
                    path: ActiveValue::Set(update.path.clone()),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            }

            let connection_ids = self.project_guest_connection_ids(project_id, &tx).await?;
            Ok(connection_ids)
        })
        .await
    }

    pub async fn join_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<(Project, ReplicaId)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let participant = room_participant::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            room_participant::Column::AnsweringConnectionId
                                .eq(connection.id as i32),
                        )
                        .add(
                            room_participant::Column::AnsweringConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("must join a room first"))?;

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            if project.room_id != participant.room_id {
                return Err(anyhow!("no such project"))?;
            }

            let mut collaborators = project
                .find_related(project_collaborator::Entity)
                .all(&*tx)
                .await?;
            let replica_ids = collaborators
                .iter()
                .map(|c| c.replica_id)
                .collect::<HashSet<_>>();
            let mut replica_id = ReplicaId(1);
            while replica_ids.contains(&replica_id) {
                replica_id.0 += 1;
            }
            let new_collaborator = project_collaborator::ActiveModel {
                project_id: ActiveValue::set(project_id),
                connection_id: ActiveValue::set(connection.id as i32),
                connection_server_id: ActiveValue::set(ServerId(connection.owner_id as i32)),
                user_id: ActiveValue::set(participant.user_id),
                replica_id: ActiveValue::set(replica_id),
                is_host: ActiveValue::set(false),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            collaborators.push(new_collaborator);

            let db_worktrees = project.find_related(worktree::Entity).all(&*tx).await?;
            let mut worktrees = db_worktrees
                .into_iter()
                .map(|db_worktree| {
                    (
                        db_worktree.id as u64,
                        Worktree {
                            id: db_worktree.id as u64,
                            abs_path: db_worktree.abs_path,
                            root_name: db_worktree.root_name,
                            visible: db_worktree.visible,
                            entries: Default::default(),
                            repository_entries: Default::default(),
                            diagnostic_summaries: Default::default(),
                            settings_files: Default::default(),
                            scan_id: db_worktree.scan_id as u64,
                            completed_scan_id: db_worktree.completed_scan_id as u64,
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>();

            // Populate worktree entries.
            {
                let mut db_entries = worktree_entry::Entity::find()
                    .filter(
                        Condition::all()
                            .add(worktree_entry::Column::ProjectId.eq(project_id))
                            .add(worktree_entry::Column::IsDeleted.eq(false)),
                    )
                    .stream(&*tx)
                    .await?;
                while let Some(db_entry) = db_entries.next().await {
                    let db_entry = db_entry?;
                    if let Some(worktree) = worktrees.get_mut(&(db_entry.worktree_id as u64)) {
                        worktree.entries.push(proto::Entry {
                            id: db_entry.id as u64,
                            is_dir: db_entry.is_dir,
                            path: db_entry.path,
                            inode: db_entry.inode as u64,
                            mtime: Some(proto::Timestamp {
                                seconds: db_entry.mtime_seconds as u64,
                                nanos: db_entry.mtime_nanos as u32,
                            }),
                            is_symlink: db_entry.is_symlink,
                            is_ignored: db_entry.is_ignored,
                            git_status: db_entry.git_status.map(|status| status as i32),
                        });
                    }
                }
            }

            // Populate repository entries.
            {
                let mut db_repository_entries = worktree_repository::Entity::find()
                    .filter(
                        Condition::all()
                            .add(worktree_repository::Column::ProjectId.eq(project_id))
                            .add(worktree_repository::Column::IsDeleted.eq(false)),
                    )
                    .stream(&*tx)
                    .await?;
                while let Some(db_repository_entry) = db_repository_entries.next().await {
                    let db_repository_entry = db_repository_entry?;
                    if let Some(worktree) =
                        worktrees.get_mut(&(db_repository_entry.worktree_id as u64))
                    {
                        worktree.repository_entries.insert(
                            db_repository_entry.work_directory_id as u64,
                            proto::RepositoryEntry {
                                work_directory_id: db_repository_entry.work_directory_id as u64,
                                branch: db_repository_entry.branch,
                            },
                        );
                    }
                }
            }

            // Populate worktree diagnostic summaries.
            {
                let mut db_summaries = worktree_diagnostic_summary::Entity::find()
                    .filter(worktree_diagnostic_summary::Column::ProjectId.eq(project_id))
                    .stream(&*tx)
                    .await?;
                while let Some(db_summary) = db_summaries.next().await {
                    let db_summary = db_summary?;
                    if let Some(worktree) = worktrees.get_mut(&(db_summary.worktree_id as u64)) {
                        worktree
                            .diagnostic_summaries
                            .push(proto::DiagnosticSummary {
                                path: db_summary.path,
                                language_server_id: db_summary.language_server_id as u64,
                                error_count: db_summary.error_count as u32,
                                warning_count: db_summary.warning_count as u32,
                            });
                    }
                }
            }

            // Populate worktree settings files
            {
                let mut db_settings_files = worktree_settings_file::Entity::find()
                    .filter(worktree_settings_file::Column::ProjectId.eq(project_id))
                    .stream(&*tx)
                    .await?;
                while let Some(db_settings_file) = db_settings_files.next().await {
                    let db_settings_file = db_settings_file?;
                    if let Some(worktree) =
                        worktrees.get_mut(&(db_settings_file.worktree_id as u64))
                    {
                        worktree.settings_files.push(WorktreeSettingsFile {
                            path: db_settings_file.path,
                            content: db_settings_file.content,
                        });
                    }
                }
            }

            // Populate language servers.
            let language_servers = project
                .find_related(language_server::Entity)
                .all(&*tx)
                .await?;

            let project = Project {
                collaborators: collaborators
                    .into_iter()
                    .map(|collaborator| ProjectCollaborator {
                        connection_id: collaborator.connection(),
                        user_id: collaborator.user_id,
                        replica_id: collaborator.replica_id,
                        is_host: collaborator.is_host,
                    })
                    .collect(),
                worktrees,
                language_servers: language_servers
                    .into_iter()
                    .map(|language_server| proto::LanguageServer {
                        id: language_server.id as u64,
                        name: language_server.name,
                    })
                    .collect(),
            };
            Ok((project, replica_id as ReplicaId))
        })
        .await
    }

    pub async fn leave_project(
        &self,
        project_id: ProjectId,
        connection: ConnectionId,
    ) -> Result<RoomGuard<(proto::Room, LeftProject)>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let result = project_collaborator::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(project_collaborator::Column::ProjectId.eq(project_id))
                        .add(project_collaborator::Column::ConnectionId.eq(connection.id as i32))
                        .add(
                            project_collaborator::Column::ConnectionServerId
                                .eq(connection.owner_id as i32),
                        ),
                )
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("not a collaborator on this project"))?;
            }

            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such project"))?;
            let collaborators = project
                .find_related(project_collaborator::Entity)
                .all(&*tx)
                .await?;
            let connection_ids = collaborators
                .into_iter()
                .map(|collaborator| collaborator.connection())
                .collect();

            follower::Entity::delete_many()
                .filter(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(follower::Column::ProjectId.eq(project_id))
                                .add(
                                    follower::Column::LeaderConnectionServerId
                                        .eq(connection.owner_id),
                                )
                                .add(follower::Column::LeaderConnectionId.eq(connection.id)),
                        )
                        .add(
                            Condition::all()
                                .add(follower::Column::ProjectId.eq(project_id))
                                .add(
                                    follower::Column::FollowerConnectionServerId
                                        .eq(connection.owner_id),
                                )
                                .add(follower::Column::FollowerConnectionId.eq(connection.id)),
                        ),
                )
                .exec(&*tx)
                .await?;

            let room = self.get_room(project.room_id, &tx).await?;
            let left_project = LeftProject {
                id: project_id,
                host_user_id: project.host_user_id,
                host_connection_id: project.host_connection()?,
                connection_ids,
            };
            Ok((room, left_project))
        })
        .await
    }

    pub async fn project_collaborators(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<Vec<ProjectCollaborator>>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let collaborators = project_collaborator::Entity::find()
                .filter(project_collaborator::Column::ProjectId.eq(project_id))
                .all(&*tx)
                .await?
                .into_iter()
                .map(|collaborator| ProjectCollaborator {
                    connection_id: collaborator.connection(),
                    user_id: collaborator.user_id,
                    replica_id: collaborator.replica_id,
                    is_host: collaborator.is_host,
                })
                .collect::<Vec<_>>();

            if collaborators
                .iter()
                .any(|collaborator| collaborator.connection_id == connection_id)
            {
                Ok(collaborators)
            } else {
                Err(anyhow!("no such project"))?
            }
        })
        .await
    }

    pub async fn project_connection_ids(
        &self,
        project_id: ProjectId,
        connection_id: ConnectionId,
    ) -> Result<RoomGuard<HashSet<ConnectionId>>> {
        let room_id = self.room_id_for_project(project_id).await?;
        self.room_transaction(room_id, |tx| async move {
            let mut collaborators = project_collaborator::Entity::find()
                .filter(project_collaborator::Column::ProjectId.eq(project_id))
                .stream(&*tx)
                .await?;

            let mut connection_ids = HashSet::default();
            while let Some(collaborator) = collaborators.next().await {
                let collaborator = collaborator?;
                connection_ids.insert(collaborator.connection());
            }

            if connection_ids.contains(&connection_id) {
                Ok(connection_ids)
            } else {
                Err(anyhow!("no such project"))?
            }
        })
        .await
    }

    async fn project_guest_connection_ids(
        &self,
        project_id: ProjectId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ConnectionId>> {
        let mut collaborators = project_collaborator::Entity::find()
            .filter(
                project_collaborator::Column::ProjectId
                    .eq(project_id)
                    .and(project_collaborator::Column::IsHost.eq(false)),
            )
            .stream(tx)
            .await?;

        let mut guest_connection_ids = Vec::new();
        while let Some(collaborator) = collaborators.next().await {
            let collaborator = collaborator?;
            guest_connection_ids.push(collaborator.connection());
        }
        Ok(guest_connection_ids)
    }

    async fn room_id_for_project(&self, project_id: ProjectId) -> Result<RoomId> {
        self.transaction(|tx| async move {
            let project = project::Entity::find_by_id(project_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("project {} not found", project_id))?;
            Ok(project.room_id)
        })
        .await
    }

    // access tokens

    pub async fn create_access_token(
        &self,
        user_id: UserId,
        access_token_hash: &str,
        max_access_token_count: usize,
    ) -> Result<AccessTokenId> {
        self.transaction(|tx| async {
            let tx = tx;

            let token = access_token::ActiveModel {
                user_id: ActiveValue::set(user_id),
                hash: ActiveValue::set(access_token_hash.into()),
                ..Default::default()
            }
            .insert(&*tx)
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
                .exec(&*tx)
                .await?;
            Ok(token.id)
        })
        .await
    }

    pub async fn get_access_token(
        &self,
        access_token_id: AccessTokenId,
    ) -> Result<access_token::Model> {
        self.transaction(|tx| async move {
            Ok(access_token::Entity::find_by_id(access_token_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such access token"))?)
        })
        .await
    }

    async fn transaction<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let body = async {
            let mut i = 0;
            loop {
                let (tx, result) = self.with_transaction(&f).await?;
                match result {
                    Ok(result) => match tx.commit().await.map_err(Into::into) {
                        Ok(()) => return Ok(result),
                        Err(error) => {
                            if !self.retry_on_serialization_error(&error, i).await {
                                return Err(error);
                            }
                        }
                    },
                    Err(error) => {
                        tx.rollback().await?;
                        if !self.retry_on_serialization_error(&error, i).await {
                            return Err(error);
                        }
                    }
                }
                i += 1;
            }
        };

        self.run(body).await
    }

    async fn optional_room_transaction<F, Fut, T>(&self, f: F) -> Result<Option<RoomGuard<T>>>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<Option<(RoomId, T)>>>,
    {
        let body = async {
            let mut i = 0;
            loop {
                let (tx, result) = self.with_transaction(&f).await?;
                match result {
                    Ok(Some((room_id, data))) => {
                        let lock = self.rooms.entry(room_id).or_default().clone();
                        let _guard = lock.lock_owned().await;
                        match tx.commit().await.map_err(Into::into) {
                            Ok(()) => {
                                return Ok(Some(RoomGuard {
                                    data,
                                    _guard,
                                    _not_send: PhantomData,
                                }));
                            }
                            Err(error) => {
                                if !self.retry_on_serialization_error(&error, i).await {
                                    return Err(error);
                                }
                            }
                        }
                    }
                    Ok(None) => match tx.commit().await.map_err(Into::into) {
                        Ok(()) => return Ok(None),
                        Err(error) => {
                            if !self.retry_on_serialization_error(&error, i).await {
                                return Err(error);
                            }
                        }
                    },
                    Err(error) => {
                        tx.rollback().await?;
                        if !self.retry_on_serialization_error(&error, i).await {
                            return Err(error);
                        }
                    }
                }
                i += 1;
            }
        };

        self.run(body).await
    }

    async fn room_transaction<F, Fut, T>(&self, room_id: RoomId, f: F) -> Result<RoomGuard<T>>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let body = async {
            let mut i = 0;
            loop {
                let lock = self.rooms.entry(room_id).or_default().clone();
                let _guard = lock.lock_owned().await;
                let (tx, result) = self.with_transaction(&f).await?;
                match result {
                    Ok(data) => match tx.commit().await.map_err(Into::into) {
                        Ok(()) => {
                            return Ok(RoomGuard {
                                data,
                                _guard,
                                _not_send: PhantomData,
                            });
                        }
                        Err(error) => {
                            if !self.retry_on_serialization_error(&error, i).await {
                                return Err(error);
                            }
                        }
                    },
                    Err(error) => {
                        tx.rollback().await?;
                        if !self.retry_on_serialization_error(&error, i).await {
                            return Err(error);
                        }
                    }
                }
                i += 1;
            }
        };

        self.run(body).await
    }

    async fn with_transaction<F, Fut, T>(&self, f: &F) -> Result<(DatabaseTransaction, Result<T>)>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let tx = self
            .pool
            .begin_with_config(Some(IsolationLevel::Serializable), None)
            .await?;

        let mut tx = Arc::new(Some(tx));
        let result = f(TransactionHandle(tx.clone())).await;
        let Some(tx) = Arc::get_mut(&mut tx).and_then(|tx| tx.take()) else {
            return Err(anyhow!("couldn't complete transaction because it's still in use"))?;
        };

        Ok((tx, result))
    }

    async fn run<F, T>(&self, future: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        #[cfg(test)]
        {
            if let Executor::Deterministic(executor) = &self.executor {
                executor.simulate_random_delay().await;
            }

            self.runtime.as_ref().unwrap().block_on(future)
        }

        #[cfg(not(test))]
        {
            future.await
        }
    }

    async fn retry_on_serialization_error(&self, error: &Error, prev_attempt_count: u32) -> bool {
        // If the error is due to a failure to serialize concurrent transactions, then retry
        // this transaction after a delay. With each subsequent retry, double the delay duration.
        // Also vary the delay randomly in order to ensure different database connections retry
        // at different times.
        if is_serialization_error(error) {
            let base_delay = 4_u64 << prev_attempt_count.min(16);
            let randomized_delay = base_delay as f32 * self.rng.lock().await.gen_range(0.5..=2.0);
            log::info!(
                "retrying transaction after serialization error. delay: {} ms.",
                randomized_delay
            );
            self.executor
                .sleep(Duration::from_millis(randomized_delay as u64))
                .await;
            true
        } else {
            false
        }
    }
}

fn is_serialization_error(error: &Error) -> bool {
    const SERIALIZATION_FAILURE_CODE: &'static str = "40001";
    match error {
        Error::Database(
            DbErr::Exec(sea_orm::RuntimeErr::SqlxError(error))
            | DbErr::Query(sea_orm::RuntimeErr::SqlxError(error)),
        ) if error
            .as_database_error()
            .and_then(|error| error.code())
            .as_deref()
            == Some(SERIALIZATION_FAILURE_CODE) =>
        {
            true
        }
        _ => false,
    }
}

struct TransactionHandle(Arc<Option<DatabaseTransaction>>);

impl Deref for TransactionHandle {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().as_ref().unwrap()
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
            Serialize,
            Deserialize,
        )]
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
id_type!(FollowerId);
id_type!(RoomId);
id_type!(RoomParticipantId);
id_type!(ProjectId);
id_type!(ProjectCollaboratorId);
id_type!(ReplicaId);
id_type!(ServerId);
id_type!(SignupId);
id_type!(UserId);

pub struct RejoinedRoom {
    pub room: proto::Room,
    pub rejoined_projects: Vec<RejoinedProject>,
    pub reshared_projects: Vec<ResharedProject>,
}

pub struct ResharedProject {
    pub id: ProjectId,
    pub old_connection_id: ConnectionId,
    pub collaborators: Vec<ProjectCollaborator>,
    pub worktrees: Vec<proto::WorktreeMetadata>,
}

pub struct RejoinedProject {
    pub id: ProjectId,
    pub old_connection_id: ConnectionId,
    pub collaborators: Vec<ProjectCollaborator>,
    pub worktrees: Vec<RejoinedWorktree>,
    pub language_servers: Vec<proto::LanguageServer>,
}

#[derive(Debug)]
pub struct RejoinedWorktree {
    pub id: u64,
    pub abs_path: String,
    pub root_name: String,
    pub visible: bool,
    pub updated_entries: Vec<proto::Entry>,
    pub removed_entries: Vec<u64>,
    pub updated_repositories: Vec<proto::RepositoryEntry>,
    pub removed_repositories: Vec<u64>,
    pub diagnostic_summaries: Vec<proto::DiagnosticSummary>,
    pub settings_files: Vec<WorktreeSettingsFile>,
    pub scan_id: u64,
    pub completed_scan_id: u64,
}

pub struct LeftRoom {
    pub room: proto::Room,
    pub left_projects: HashMap<ProjectId, LeftProject>,
    pub canceled_calls_to_user_ids: Vec<UserId>,
}

pub struct RefreshedRoom {
    pub room: proto::Room,
    pub stale_participant_user_ids: Vec<UserId>,
    pub canceled_calls_to_user_ids: Vec<UserId>,
}

pub struct Project {
    pub collaborators: Vec<ProjectCollaborator>,
    pub worktrees: BTreeMap<u64, Worktree>,
    pub language_servers: Vec<proto::LanguageServer>,
}

pub struct ProjectCollaborator {
    pub connection_id: ConnectionId,
    pub user_id: UserId,
    pub replica_id: ReplicaId,
    pub is_host: bool,
}

impl ProjectCollaborator {
    pub fn to_proto(&self) -> proto::Collaborator {
        proto::Collaborator {
            peer_id: Some(self.connection_id.into()),
            replica_id: self.replica_id.0 as u32,
            user_id: self.user_id.to_proto(),
        }
    }
}

#[derive(Debug)]
pub struct LeftProject {
    pub id: ProjectId,
    pub host_user_id: UserId,
    pub host_connection_id: ConnectionId,
    pub connection_ids: Vec<ConnectionId>,
}

pub struct Worktree {
    pub id: u64,
    pub abs_path: String,
    pub root_name: String,
    pub visible: bool,
    pub entries: Vec<proto::Entry>,
    pub repository_entries: BTreeMap<u64, proto::RepositoryEntry>,
    pub diagnostic_summaries: Vec<proto::DiagnosticSummary>,
    pub settings_files: Vec<WorktreeSettingsFile>,
    pub scan_id: u64,
    pub completed_scan_id: u64,
}

#[derive(Debug)]
pub struct WorktreeSettingsFile {
    pub path: String,
    pub content: String,
}

#[cfg(test)]
pub use test::*;

#[cfg(test)]
mod test {
    use super::*;
    use gpui::executor::Background;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
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
                let db = Database::new(options, Executor::Deterministic(background))
                    .await
                    .unwrap();
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
                let db = Database::new(options, Executor::Deterministic(background))
                    .await
                    .unwrap();
                let migrations_path = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
                db.migrate(Path::new(migrations_path), false).await.unwrap();
                db
            });

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
            if let sea_orm::DatabaseBackend::Postgres = db.pool.get_database_backend() {
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
