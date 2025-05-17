mod ids;
mod queries;
mod tables;
#[cfg(test)]
pub mod tests;

use crate::{Error, Result, executor::Executor};
use anyhow::anyhow;
use collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use dashmap::DashMap;
use futures::StreamExt;
use project_repository_statuses::StatusKind;
use rand::{Rng, SeedableRng, prelude::StdRng};
use rpc::ExtensionProvides;
use rpc::{
    ConnectionId, ExtensionMetadata,
    proto::{self},
};
use sea_orm::{
    ActiveValue, Condition, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    FromQueryResult, IntoActiveModel, IsolationLevel, JoinType, QueryOrder, QuerySelect, Statement,
    TransactionTrait,
    entity::prelude::*,
    sea_query::{Alias, Expr, OnConflict},
};
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use std::{
    fmt::Write as _,
    future::Future,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use time::PrimitiveDateTime;
use tokio::sync::{Mutex, OwnedMutexGuard};
use worktree_settings_file::LocalSettingsKind;

#[cfg(test)]
pub use tests::TestDb;

pub use ids::*;
pub use queries::billing_customers::{CreateBillingCustomerParams, UpdateBillingCustomerParams};
pub use queries::billing_preferences::{
    CreateBillingPreferencesParams, UpdateBillingPreferencesParams,
};
pub use queries::billing_subscriptions::{
    CreateBillingSubscriptionParams, UpdateBillingSubscriptionParams,
};
pub use queries::contributors::ContributorSelector;
pub use queries::processed_stripe_events::CreateProcessedStripeEventParams;
pub use sea_orm::ConnectOptions;
pub use tables::user::Model as User;
pub use tables::*;

/// Database gives you a handle that lets you access the database.
/// It handles pooling internally.
pub struct Database {
    options: ConnectOptions,
    pool: DatabaseConnection,
    rooms: DashMap<RoomId, Arc<Mutex<()>>>,
    projects: DashMap<ProjectId, Arc<Mutex<()>>>,
    rng: Mutex<StdRng>,
    executor: Executor,
    notification_kinds_by_id: HashMap<NotificationKindId, &'static str>,
    notification_kinds_by_name: HashMap<String, NotificationKindId>,
    #[cfg(test)]
    runtime: Option<tokio::runtime::Runtime>,
}

// The `Database` type has so many methods that its impl blocks are split into
// separate files in the `queries` folder.
impl Database {
    /// Connects to the database with the given options
    pub async fn new(options: ConnectOptions, executor: Executor) -> Result<Self> {
        sqlx::any::install_default_drivers();
        Ok(Self {
            options: options.clone(),
            pool: sea_orm::Database::connect(options).await?,
            rooms: DashMap::with_capacity(16384),
            projects: DashMap::with_capacity(16384),
            rng: Mutex::new(StdRng::seed_from_u64(0)),
            notification_kinds_by_id: HashMap::default(),
            notification_kinds_by_name: HashMap::default(),
            executor,
            #[cfg(test)]
            runtime: None,
        })
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }

    #[cfg(test)]
    pub fn reset(&self) {
        self.rooms.clear();
        self.projects.clear();
    }

    /// Transaction runs things in a transaction. If you want to call other methods
    /// and pass the transaction around you need to reborrow the transaction at each
    /// call site with: `&*tx`.
    pub async fn transaction<F, Fut, T>(&self, f: F) -> Result<T>
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

    pub async fn weak_transaction<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let body = async {
            let (tx, result) = self.with_weak_transaction(&f).await?;
            match result {
                Ok(result) => match tx.commit().await.map_err(Into::into) {
                    Ok(()) => Ok(result),
                    Err(error) => Err(error),
                },
                Err(error) => {
                    tx.rollback().await?;
                    Err(error)
                }
            }
        };

        self.run(body).await
    }

    /// The same as room_transaction, but if you need to only optionally return a Room.
    async fn optional_room_transaction<F, Fut, T>(
        &self,
        f: F,
    ) -> Result<Option<TransactionGuard<T>>>
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
                                return Ok(Some(TransactionGuard {
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

    async fn project_transaction<F, Fut, T>(
        &self,
        project_id: ProjectId,
        f: F,
    ) -> Result<TransactionGuard<T>>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let room_id = Database::room_id_for_project(self, project_id).await?;
        let body = async {
            let mut i = 0;
            loop {
                let lock = if let Some(room_id) = room_id {
                    self.rooms.entry(room_id).or_default().clone()
                } else {
                    self.projects.entry(project_id).or_default().clone()
                };
                let _guard = lock.lock_owned().await;
                let (tx, result) = self.with_transaction(&f).await?;
                match result {
                    Ok(data) => match tx.commit().await.map_err(Into::into) {
                        Ok(()) => {
                            return Ok(TransactionGuard {
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

    /// room_transaction runs the block in a transaction. It returns a RoomGuard, that keeps
    /// the database locked until it is dropped. This ensures that updates sent to clients are
    /// properly serialized with respect to database changes.
    async fn room_transaction<F, Fut, T>(
        &self,
        room_id: RoomId,
        f: F,
    ) -> Result<TransactionGuard<T>>
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
                            return Ok(TransactionGuard {
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
            return Err(anyhow!(
                "couldn't complete transaction because it's still in use"
            ))?;
        };

        Ok((tx, result))
    }

    async fn with_weak_transaction<F, Fut, T>(
        &self,
        f: &F,
    ) -> Result<(DatabaseTransaction, Result<T>)>
    where
        F: Send + Fn(TransactionHandle) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        let tx = self
            .pool
            .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
            .await?;

        let mut tx = Arc::new(Some(tx));
        let result = f(TransactionHandle(tx.clone())).await;
        let Some(tx) = Arc::get_mut(&mut tx).and_then(|tx| tx.take()) else {
            return Err(anyhow!(
                "couldn't complete transaction because it's still in use"
            ))?;
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

    async fn retry_on_serialization_error(&self, error: &Error, prev_attempt_count: usize) -> bool {
        // If the error is due to a failure to serialize concurrent transactions, then retry
        // this transaction after a delay. With each subsequent retry, double the delay duration.
        // Also vary the delay randomly in order to ensure different database connections retry
        // at different times.
        const SLEEPS: [f32; 10] = [10., 20., 40., 80., 160., 320., 640., 1280., 2560., 5120.];
        if is_serialization_error(error) && prev_attempt_count < SLEEPS.len() {
            let base_delay = SLEEPS[prev_attempt_count];
            let randomized_delay = base_delay * self.rng.lock().await.gen_range(0.5..=2.0);
            log::warn!(
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
    const SERIALIZATION_FAILURE_CODE: &str = "40001";
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

/// A handle to a [`DatabaseTransaction`].
pub struct TransactionHandle(pub(crate) Arc<Option<DatabaseTransaction>>);

impl Deref for TransactionHandle {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().as_ref().unwrap()
    }
}

/// [`TransactionGuard`] keeps a database transaction alive until it is dropped.
/// It wraps data that depends on the state of the database and prevents an additional
/// transaction from starting that would invalidate that data.
pub struct TransactionGuard<T> {
    data: T,
    _guard: OwnedMutexGuard<()>,
    _not_send: PhantomData<Rc<()>>,
}

impl<T> Deref for TransactionGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for TransactionGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> TransactionGuard<T> {
    /// Returns the inner value of the guard.
    pub fn into_inner(self) -> T {
        self.data
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contact {
    Accepted { user_id: UserId, busy: bool },
    Outgoing { user_id: UserId },
    Incoming { user_id: UserId },
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

pub type NotificationBatch = Vec<(UserId, proto::Notification)>;

pub struct CreatedChannelMessage {
    pub message_id: MessageId,
    pub participant_connection_ids: HashSet<ConnectionId>,
    pub notifications: NotificationBatch,
}

pub struct UpdatedChannelMessage {
    pub message_id: MessageId,
    pub participant_connection_ids: Vec<ConnectionId>,
    pub notifications: NotificationBatch,
    pub reply_to_message_id: Option<MessageId>,
    pub timestamp: PrimitiveDateTime,
    pub deleted_mention_notification_ids: Vec<NotificationId>,
    pub updated_mention_notifications: Vec<rpc::proto::Notification>,
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

/// The parameters to create a new user.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserParams {
    pub github_login: String,
    pub github_user_id: i32,
}

/// The result of creating a new user.
#[derive(Debug)]
pub struct NewUserResult {
    pub user_id: UserId,
    pub metrics_id: String,
    pub inviting_user_id: Option<UserId>,
    pub signup_device_id: Option<String>,
}

/// The result of updating a channel membership.
#[derive(Debug)]
pub struct MembershipUpdated {
    pub channel_id: ChannelId,
    pub new_channels: ChannelsForUser,
    pub removed_channels: Vec<ChannelId>,
}

/// The result of setting a member's role.
#[derive(Debug)]

pub enum SetMemberRoleResult {
    InviteUpdated(Channel),
    MembershipUpdated(MembershipUpdated),
}

/// The result of inviting a member to a channel.
#[derive(Debug)]
pub struct InviteMemberResult {
    pub channel: Channel,
    pub notifications: NotificationBatch,
}

#[derive(Debug)]
pub struct RespondToChannelInvite {
    pub membership_update: Option<MembershipUpdated>,
    pub notifications: NotificationBatch,
}

#[derive(Debug)]
pub struct RemoveChannelMemberResult {
    pub membership_update: MembershipUpdated,
    pub notification_id: Option<NotificationId>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub visibility: ChannelVisibility,
    /// parent_path is the channel ids from the root to this one (not including this one)
    pub parent_path: Vec<ChannelId>,
}

impl Channel {
    pub fn from_model(value: channel::Model) -> Self {
        Channel {
            id: value.id,
            visibility: value.visibility,
            name: value.clone().name,
            parent_path: value.ancestors().collect(),
        }
    }

    pub fn to_proto(&self) -> proto::Channel {
        proto::Channel {
            id: self.id.to_proto(),
            name: self.name.clone(),
            visibility: self.visibility.into(),
            parent_path: self.parent_path.iter().map(|c| c.to_proto()).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChannelMember {
    pub role: ChannelRole,
    pub user_id: UserId,
    pub kind: proto::channel_member::Kind,
}

impl ChannelMember {
    pub fn to_proto(&self) -> proto::ChannelMember {
        proto::ChannelMember {
            role: self.role.into(),
            user_id: self.user_id.to_proto(),
            kind: self.kind.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ChannelsForUser {
    pub channels: Vec<Channel>,
    pub channel_memberships: Vec<channel_member::Model>,
    pub channel_participants: HashMap<ChannelId, Vec<UserId>>,
    pub invited_channels: Vec<Channel>,

    pub observed_buffer_versions: Vec<proto::ChannelBufferVersion>,
    pub observed_channel_messages: Vec<proto::ChannelMessageId>,
    pub latest_buffer_versions: Vec<proto::ChannelBufferVersion>,
    pub latest_channel_messages: Vec<proto::ChannelMessageId>,
}

#[derive(Debug)]
pub struct RejoinedChannelBuffer {
    pub buffer: proto::RejoinedChannelBuffer,
    pub old_connection_id: ConnectionId,
}

#[derive(Clone)]
pub struct JoinRoom {
    pub room: proto::Room,
    pub channel: Option<channel::Model>,
}

pub struct RejoinedRoom {
    pub room: proto::Room,
    pub rejoined_projects: Vec<RejoinedProject>,
    pub reshared_projects: Vec<ResharedProject>,
    pub channel: Option<channel::Model>,
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
    pub updated_repositories: Vec<proto::UpdateRepository>,
    pub removed_repositories: Vec<u64>,
    pub language_servers: Vec<proto::LanguageServer>,
}

impl RejoinedProject {
    pub fn to_proto(&self) -> proto::RejoinedProject {
        proto::RejoinedProject {
            id: self.id.to_proto(),
            worktrees: self
                .worktrees
                .iter()
                .map(|worktree| proto::WorktreeMetadata {
                    id: worktree.id,
                    root_name: worktree.root_name.clone(),
                    visible: worktree.visible,
                    abs_path: worktree.abs_path.clone(),
                })
                .collect(),
            collaborators: self
                .collaborators
                .iter()
                .map(|collaborator| collaborator.to_proto())
                .collect(),
            language_servers: self.language_servers.clone(),
        }
    }
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
    pub channel: Option<channel::Model>,
    pub left_projects: HashMap<ProjectId, LeftProject>,
    pub canceled_calls_to_user_ids: Vec<UserId>,
    pub deleted: bool,
}

pub struct RefreshedRoom {
    pub room: proto::Room,
    pub channel: Option<channel::Model>,
    pub stale_participant_user_ids: Vec<UserId>,
    pub canceled_calls_to_user_ids: Vec<UserId>,
}

pub struct RefreshedChannelBuffer {
    pub connection_ids: Vec<ConnectionId>,
    pub collaborators: Vec<proto::Collaborator>,
}

pub struct Project {
    pub id: ProjectId,
    pub role: ChannelRole,
    pub collaborators: Vec<ProjectCollaborator>,
    pub worktrees: BTreeMap<u64, Worktree>,
    pub repositories: Vec<proto::UpdateRepository>,
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
            is_host: self.is_host,
        }
    }
}

#[derive(Debug)]
pub struct LeftProject {
    pub id: ProjectId,
    pub should_unshare: bool,
    pub connection_ids: Vec<ConnectionId>,
}

pub struct Worktree {
    pub id: u64,
    pub abs_path: String,
    pub root_name: String,
    pub visible: bool,
    pub entries: Vec<proto::Entry>,
    pub legacy_repository_entries: BTreeMap<u64, proto::RepositoryEntry>,
    pub diagnostic_summaries: Vec<proto::DiagnosticSummary>,
    pub settings_files: Vec<WorktreeSettingsFile>,
    pub scan_id: u64,
    pub completed_scan_id: u64,
}

#[derive(Debug)]
pub struct WorktreeSettingsFile {
    pub path: String,
    pub content: String,
    pub kind: LocalSettingsKind,
}

pub struct NewExtensionVersion {
    pub name: String,
    pub version: semver::Version,
    pub description: String,
    pub authors: Vec<String>,
    pub repository: String,
    pub schema_version: i32,
    pub wasm_api_version: Option<String>,
    pub provides: BTreeSet<ExtensionProvides>,
    pub published_at: PrimitiveDateTime,
}

pub struct ExtensionVersionConstraints {
    pub schema_versions: RangeInclusive<i32>,
    pub wasm_api_versions: RangeInclusive<SemanticVersion>,
}

impl LocalSettingsKind {
    pub fn from_proto(proto_kind: proto::LocalSettingsKind) -> Self {
        match proto_kind {
            proto::LocalSettingsKind::Settings => Self::Settings,
            proto::LocalSettingsKind::Tasks => Self::Tasks,
            proto::LocalSettingsKind::Editorconfig => Self::Editorconfig,
            proto::LocalSettingsKind::Debug => Self::Debug,
        }
    }

    pub fn to_proto(&self) -> proto::LocalSettingsKind {
        match self {
            Self::Settings => proto::LocalSettingsKind::Settings,
            Self::Tasks => proto::LocalSettingsKind::Tasks,
            Self::Editorconfig => proto::LocalSettingsKind::Editorconfig,
            Self::Debug => proto::LocalSettingsKind::Debug,
        }
    }
}

fn db_status_to_proto(
    entry: project_repository_statuses::Model,
) -> anyhow::Result<proto::StatusEntry> {
    use proto::git_file_status::{Tracked, Unmerged, Variant};

    let (simple_status, variant) =
        match (entry.status_kind, entry.first_status, entry.second_status) {
            (StatusKind::Untracked, None, None) => (
                proto::GitStatus::Added as i32,
                Variant::Untracked(Default::default()),
            ),
            (StatusKind::Ignored, None, None) => (
                proto::GitStatus::Added as i32,
                Variant::Ignored(Default::default()),
            ),
            (StatusKind::Unmerged, Some(first_head), Some(second_head)) => (
                proto::GitStatus::Conflict as i32,
                Variant::Unmerged(Unmerged {
                    first_head,
                    second_head,
                }),
            ),
            (StatusKind::Tracked, Some(index_status), Some(worktree_status)) => {
                let simple_status = if worktree_status != proto::GitStatus::Unmodified as i32 {
                    worktree_status
                } else if index_status != proto::GitStatus::Unmodified as i32 {
                    index_status
                } else {
                    proto::GitStatus::Unmodified as i32
                };
                (
                    simple_status,
                    Variant::Tracked(Tracked {
                        index_status,
                        worktree_status,
                    }),
                )
            }
            _ => {
                return Err(anyhow!(
                    "Unexpected combination of status fields: {entry:?}"
                ));
            }
        };
    Ok(proto::StatusEntry {
        repo_path: entry.repo_path,
        simple_status,
        status: Some(proto::GitFileStatus {
            variant: Some(variant),
        }),
    })
}

fn proto_status_to_db(
    status_entry: proto::StatusEntry,
) -> (String, StatusKind, Option<i32>, Option<i32>) {
    use proto::git_file_status::{Tracked, Unmerged, Variant};

    let (status_kind, first_status, second_status) = status_entry
        .status
        .clone()
        .and_then(|status| status.variant)
        .map_or(
            (StatusKind::Untracked, None, None),
            |variant| match variant {
                Variant::Untracked(_) => (StatusKind::Untracked, None, None),
                Variant::Ignored(_) => (StatusKind::Ignored, None, None),
                Variant::Unmerged(Unmerged {
                    first_head,
                    second_head,
                }) => (StatusKind::Unmerged, Some(first_head), Some(second_head)),
                Variant::Tracked(Tracked {
                    index_status,
                    worktree_status,
                }) => (
                    StatusKind::Tracked,
                    Some(index_status),
                    Some(worktree_status),
                ),
            },
        );
    (
        status_entry.repo_path,
        status_kind,
        first_status,
        second_status,
    )
}
