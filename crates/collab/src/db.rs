#[cfg(test)]
mod db_tests;
#[cfg(test)]
pub mod test_db;

mod ids;
mod queries;
mod tables;

use crate::{executor::Executor, Error, Result};
use anyhow::anyhow;
use collections::{BTreeMap, HashMap, HashSet};
use dashmap::DashMap;
use futures::StreamExt;
use rand::{prelude::StdRng, Rng, SeedableRng};
use rpc::{proto, ConnectionId};
use sea_orm::{
    entity::prelude::*, ActiveValue, Condition, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, DbErr, FromQueryResult, IntoActiveModel, IsolationLevel, JoinType,
    QueryOrder, QuerySelect, Statement, TransactionTrait,
};
use sea_query::{Alias, Expr, OnConflict, Query};
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::{Migrate, Migration, MigrationSource},
    Connection,
};
use std::{
    fmt::Write as _,
    future::Future,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::Path,
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use tables::*;
use tokio::sync::{Mutex, OwnedMutexGuard};

pub use ids::*;
pub use sea_orm::ConnectOptions;
pub use tables::user::Model as User;

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

impl<T> RoomGuard<T> {
    pub fn into_inner(self) -> T {
        self.data
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contact {
    Accepted {
        user_id: UserId,
        should_notify: bool,
        busy: bool,
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

#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult, Serialize, Deserialize)]
pub struct Invite {
    pub email_address: String,
    pub email_confirmation_code: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewSignup {
    pub email_address: String,
    pub platform_mac: bool,
    pub platform_windows: bool,
    pub platform_linux: bool,
    pub editor_features: Vec<String>,
    pub programming_languages: Vec<String>,
    pub device_id: Option<String>,
    pub added_to_mailing_list: bool,
    pub created_at: Option<DateTime>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromQueryResult)]
pub struct WaitlistSummary {
    pub count: i64,
    pub linux_count: i64,
    pub mac_count: i64,
    pub windows_count: i64,
    pub unknown_count: i64,
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

#[derive(FromQueryResult, Debug, PartialEq)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub parent_id: Option<ChannelId>,
}

#[derive(Debug, PartialEq)]
pub struct ChannelsForUser {
    pub channels: Vec<Channel>,
    pub channel_participants: HashMap<ChannelId, Vec<UserId>>,
    pub channels_with_admin_privileges: HashSet<ChannelId>,
}

#[derive(Clone)]
pub struct JoinRoom {
    pub room: proto::Room,
    pub channel_id: Option<ChannelId>,
    pub channel_members: Vec<UserId>,
}

pub struct RejoinedRoom {
    pub room: proto::Room,
    pub rejoined_projects: Vec<RejoinedProject>,
    pub reshared_projects: Vec<ResharedProject>,
    pub channel_id: Option<ChannelId>,
    pub channel_members: Vec<UserId>,
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
    pub channel_id: Option<ChannelId>,
    pub channel_members: Vec<UserId>,
    pub left_projects: HashMap<ProjectId, LeftProject>,
    pub canceled_calls_to_user_ids: Vec<UserId>,
    pub deleted: bool,
}

pub struct RefreshedRoom {
    pub room: proto::Room,
    pub channel_id: Option<ChannelId>,
    pub channel_members: Vec<UserId>,
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
