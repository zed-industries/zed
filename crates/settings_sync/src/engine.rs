use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Context as _, Result};
use cloud_api_client::{
    JoinSyncedSettingsGroupBody, SYNCED_SETTINGS_KIND_SETTINGS, SyncedSettings,
    SyncedSettingsErasureScope, SyncedSettingsGroupTarget, SyncedSettingsHost,
    SyncedSettingsRevision, UpdateSyncedSettingsBody, UpdateSyncedSettingsHostBody,
};
use feature_flags::{FeatureFlagAppExt as _, FeatureFlagStore, SettingsSyncFeatureFlag};
use fs::{Fs, RenameOptions};
use futures::channel::{mpsc, oneshot};
use futures::lock::Mutex;
use futures::{FutureExt as _, StreamExt};
use gpui::{App, AppContext as _, AsyncApp, Context, EventEmitter, Subscription, Task, WeakEntity};
use rand::Rng as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{SettingsStore, watch_config_file};
use settings_json::parse_json_with_comments;

use crate::merge::{
    Conflict, ExclusionSet, PathMap, SyncOp, ThreeWayMerge, diff_paths, merge_three_way,
    union_exclusion_lists,
};
use crate::server::{FetchResult, PushResult, SettingsSyncServer};
use crate::strategy::{SettingsJsonStrategy, SyncStrategy};
use crate::sync_enabled;
use crate::sync_path::SyncPath;

pub(crate) const MAX_PUSH_ATTEMPTS: usize = 3;
pub(crate) const SYNC_DEBOUNCE: Duration = Duration::from_secs(1);
pub const HISTORY_LIMIT: u32 = 20;
const PUSH_RETRY_BACKOFF: Duration = Duration::from_millis(500);
const PUSH_RETRY_MAX_JITTER_MILLIS: u64 = 250;

const SYNC_STATE_FORMAT_VERSION: u64 = 1;
const MAX_STATE_WRITE_ATTEMPTS: usize = 3;
const PRE_APPLY_BACKUP_SLOTS: usize = 3;
const OTHER_ACCOUNT_ERROR: &str = "signed into another account, not changing settings sync";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SyncState {
    pub format_version: u64,
    #[serde(default)]
    pub user_id: Option<u64>,
    #[serde(default)]
    pub consented: bool,
    pub group_id: Option<String>,
    pub base: Option<SyncBase>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncBase {
    pub document_id: String,
    pub server_version: u64,
    pub schema_epoch: u64,
    pub doc: Value,
}

pub struct SyncedDocument {
    pub kind: &'static str,
    pub file_path: PathBuf,
    pub build_strategy: fn() -> Arc<dyn SyncStrategy>,
}

impl SyncedDocument {
    pub fn user_settings() -> Self {
        Self {
            kind: SYNCED_SETTINGS_KIND_SETTINGS,
            file_path: paths::settings_file().clone(),
            build_strategy: SettingsJsonStrategy::user_settings,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstSyncChoice {
    Merge,
    ReplaceLocal,
    ReplaceRemote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FirstSyncPreview {
    pub account_id: u64,
    pub group_id: String,
    pub remote_document_id: Option<String>,
    pub remote_version: Option<u64>,
    pub local: PathMap,
    pub remote: PathMap,
    pub can_replace_remote: bool,
}

impl FirstSyncPreview {
    pub fn choices(&self) -> Vec<FirstSyncChoice> {
        if self.remote_version.is_none() {
            return Vec::from([FirstSyncChoice::ReplaceRemote]);
        }
        [
            FirstSyncChoice::Merge,
            FirstSyncChoice::ReplaceLocal,
            FirstSyncChoice::ReplaceRemote,
        ]
        .into_iter()
        .filter(|choice| *choice != FirstSyncChoice::ReplaceRemote || self.can_replace_remote)
        .collect()
    }

    pub fn merge(&self, choice: FirstSyncChoice) -> ThreeWayMerge {
        let replaced_with = match choice {
            FirstSyncChoice::ReplaceRemote if self.can_replace_remote => &self.local,
            FirstSyncChoice::ReplaceLocal => &self.remote,
            FirstSyncChoice::Merge | FirstSyncChoice::ReplaceRemote => {
                return merge_three_way(&PathMap::new(), &self.local, &self.remote);
            }
        };
        let mut merged = replaced_with.clone();
        union_exclusion_lists(&mut merged, &self.local, &self.remote);
        ThreeWayMerge {
            merged,
            conflicts: Vec::new(),
        }
    }

    pub fn local_changes(&self, choice: FirstSyncChoice) -> Vec<SyncOp> {
        diff_paths(&self.local, &self.merge(choice).merged)
    }

    pub fn remote_changes(&self, choice: FirstSyncChoice) -> Vec<SyncOp> {
        diff_paths(&self.remote, &self.merge(choice).merged)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncedSettingsHistory {
    pub account_id: u64,
    pub group_id: Option<String>,
    pub revisions: Vec<SyncedSettingsRevision>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncedSettingsHosts {
    pub account_id: u64,
    pub hosts: Vec<SyncedSettingsHost>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PendingDecision {
    Consent,
    JoinGroup {
        account_id: u64,
        hosts: Vec<SyncedSettingsHost>,
    },
    FirstSync(FirstSyncPreview),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConflictBatch {
    pub account_id: u64,
    pub group_id: String,
    pub conflicts: Vec<Conflict>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseReason {
    RepeatedConflicts,
    ApplyFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncActivity {
    Off,
    FeatureUnavailable,
    SignedOut,
    Paused,
    WaitingForDecision,
    Failing,
    On,
}

#[derive(Debug, Clone)]
pub enum SettingsSyncEvent {
    ConflictsResolved(ConflictBatch),
    Paused(PauseReason),
    UpdateRequired,
    DecisionRequired,
    SyncFailed(String),
    Erased,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SettingsSyncStatus {
    pub last_synced_at: Option<SystemTime>,
    pub server_version: Option<u64>,
    pub group_id: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug)]
struct InvalidLocalDocument;

struct LoadedState {
    state: Option<SyncState>,
    text: String,
}

struct StateSnapshot {
    state: SyncState,
    text: String,
}

impl std::fmt::Display for InvalidLocalDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the settings file cannot be synced until it parses")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyncTrigger {
    Debounced,
    Immediate,
}

struct ApprovedFirstSync {
    preview: FirstSyncPreview,
    choice: FirstSyncChoice,
}

struct CycleOutcome {
    group_id: String,
    remote: Option<SyncedSettings>,
    base_paths: PathMap,
    merged: PathMap,
    conflicts: Vec<Conflict>,
    new_base: Option<SyncBase>,
    used_approval: Option<FirstSyncPreview>,
}

pub struct SettingsSyncEngine {
    fs: Arc<dyn Fs>,
    server: Arc<dyn SettingsSyncServer>,
    document: SyncedDocument,
    state_file_path: PathBuf,
    strategy: Option<Arc<dyn SyncStrategy>>,
    state: Option<SyncState>,
    last_self_write: Option<String>,
    paused: bool,
    pending_decision: Option<PendingDecision>,
    approved_first_sync: Option<ApprovedFirstSync>,
    recent_conflicts: Option<ConflictBatch>,
    status: SettingsSyncStatus,
    update_required_notified: bool,
    flag_gate_notified: bool,
    flag_enabled: bool,
    failure_notified: bool,
    hosts_revision: u64,
    state_generation: u64,
    state_write_lock: Arc<Mutex<()>>,
    erase_in_flight: bool,
    erase_task: Task<()>,
    membership_changes_in_flight: usize,
    owned_tasks: BTreeMap<usize, Task<()>>,
    next_owned_task_id: usize,
    sync_tx: mpsc::UnboundedSender<SyncTrigger>,
    _tasks: Vec<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<SettingsSyncEvent> for SettingsSyncEngine {}

impl SettingsSyncEngine {
    pub fn new(
        server: Arc<dyn SettingsSyncServer>,
        fs: Arc<dyn Fs>,
        document: SyncedDocument,
        state_file_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Self {
        let (sync_tx, mut sync_rx) = mpsc::unbounded();

        let (mut document_file_rx, document_file_watcher) = watch_config_file(
            cx.background_executor(),
            fs.clone(),
            document.file_path.clone(),
        );
        let watch_task = cx.spawn(async move |engine, cx| {
            let _document_file_watcher = document_file_watcher;
            while let Some(content) = document_file_rx.next().await {
                let update_result = engine.update(cx, |engine, cx| {
                    if engine.last_self_write.as_deref() == Some(content.as_str()) {
                        engine.last_self_write = None;
                    } else {
                        engine.schedule_sync(cx);
                    }
                });
                if update_result.is_err() {
                    break;
                }
            }
        });

        let sync_task = cx.spawn(async move |engine, cx| {
            while let Some(trigger) = sync_rx.next().await {
                let mut immediate = trigger == SyncTrigger::Immediate;
                while let Ok(trigger) = sync_rx.try_recv() {
                    immediate |= trigger == SyncTrigger::Immediate;
                }
                if !immediate {
                    cx.background_executor().timer(SYNC_DEBOUNCE).await;
                    while sync_rx.try_recv().is_ok() {}
                }
                if engine.upgrade().is_none() {
                    break;
                }
                let result = Self::run_sync(&engine, cx).await;
                if let Err(error) = &result {
                    log::error!("settings sync: sync cycle failed: {error:#}");
                }
                if engine
                    .update(cx, |engine, cx| engine.record_cycle_result(result, cx))
                    .is_err()
                {
                    break;
                }
            }
        });

        Self {
            fs,
            server,
            document,
            state_file_path,
            strategy: None,
            state: None,
            last_self_write: None,
            paused: false,
            pending_decision: None,
            approved_first_sync: None,
            recent_conflicts: None,
            status: SettingsSyncStatus::default(),
            update_required_notified: false,
            flag_gate_notified: false,
            flag_enabled: cx.has_flag::<SettingsSyncFeatureFlag>(),
            failure_notified: false,
            hosts_revision: 0,
            state_generation: 0,
            state_write_lock: Arc::default(),
            erase_in_flight: false,
            erase_task: Task::ready(()),
            membership_changes_in_flight: 0,
            owned_tasks: BTreeMap::new(),
            next_owned_task_id: 0,
            sync_tx,
            _tasks: Vec::from([watch_task, sync_task]),
            _subscriptions: Vec::from([
                cx.observe_global::<FeatureFlagStore>(Self::handle_flag_inputs_changed),
                cx.observe_global::<SettingsStore>(Self::handle_flag_inputs_changed),
            ]),
        }
    }

    pub fn schedule_sync(&mut self, _cx: &mut Context<Self>) {
        self.send_trigger(SyncTrigger::Debounced);
    }

    pub fn sync_now(&mut self, _cx: &mut Context<Self>) {
        self.send_trigger(SyncTrigger::Immediate);
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self, cx: &mut Context<Self>) {
        self.paused = false;
        self.sync_now(cx);
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn state(&self) -> Option<&SyncState> {
        self.state.as_ref()
    }

    pub fn status(&self) -> &SettingsSyncStatus {
        &self.status
    }

    pub fn activity(&self, cx: &App) -> SyncActivity {
        if !sync_enabled(cx) {
            SyncActivity::Off
        } else if !cx.has_flag::<SettingsSyncFeatureFlag>() {
            SyncActivity::FeatureUnavailable
        } else if !self.server.is_ready() {
            SyncActivity::SignedOut
        } else if self.paused {
            SyncActivity::Paused
        } else if self.pending_decision.is_some() {
            SyncActivity::WaitingForDecision
        } else if self.status.last_error.is_some() {
            SyncActivity::Failing
        } else {
            SyncActivity::On
        }
    }

    pub fn pending_decision(&self) -> Option<&PendingDecision> {
        self.pending_decision.as_ref()
    }

    pub fn recent_conflicts(&self) -> Option<&ConflictBatch> {
        self.recent_conflicts.as_ref()
    }

    pub fn this_system_id(&self) -> Option<String> {
        self.server.system_id()
    }

    pub fn account_id(&self) -> Option<u64> {
        self.server.account_id()
    }

    pub fn hosts_revision(&self) -> u64 {
        self.hosts_revision
    }

    pub fn effective_exclusions(&self, cx: &App) -> Vec<String> {
        let user_settings = cx
            .global::<SettingsStore>()
            .raw_user_settings()
            .and_then(|user_settings| serde_json::to_value(user_settings).ok());
        let base_doc = self
            .state
            .as_ref()
            .and_then(|state| state.base.as_ref())
            .map(|base| &base.doc);
        let docs = user_settings.iter().chain(base_doc);
        match &self.strategy {
            Some(strategy) => exclusions_for(strategy.as_ref(), docs),
            None => {
                let mut exclusions = ExclusionSet::default();
                for doc in docs {
                    exclusions.extend_from_doc(doc);
                }
                exclusions
            }
        }
        .pointers()
    }

    pub fn clear_pending_decision(&mut self, cx: &mut Context<Self>) {
        self.approved_first_sync = None;
        if self.pending_decision.take().is_some() {
            cx.notify();
        }
    }

    pub fn accept_consent(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let Some(account_id) = self.server.account_id() else {
            return Task::ready(Err(anyhow::anyhow!("not signed in")));
        };
        let fs = self.fs.clone();
        let state_file_path = self.state_file_path.clone();
        let state_write_lock = self.state_write_lock.clone();
        cx.spawn(async move |engine, cx| {
            let _state_write_guard = state_write_lock.lock().await;
            let state = update_state(&engine, cx, fs.as_ref(), &state_file_path, |state| {
                if state.user_id != Some(account_id) {
                    *state = SyncState {
                        user_id: Some(account_id),
                        ..SyncState::default()
                    };
                }
                state.format_version = SYNC_STATE_FORMAT_VERSION;
                state.consented = true;
                true
            })
            .await?
            .context("settings sync state is unavailable")?;
            engine.update(cx, |engine, cx| {
                engine.state_generation += 1;
                engine.state = Some(state);
                engine.pending_decision = None;
                cx.notify();
                engine.sync_now(cx);
            })
        })
    }

    pub fn choose_first_sync(
        &mut self,
        preview: FirstSyncPreview,
        choice: FirstSyncChoice,
        cx: &mut Context<Self>,
    ) {
        self.approved_first_sync = Some(ApprovedFirstSync { preview, choice });
        self.pending_decision = None;
        cx.notify();
        self.sync_now(cx);
    }

    pub fn join_group(
        &mut self,
        account_id: u64,
        group: SyncedSettingsGroupTarget,
        label: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<SyncedSettingsHost>> {
        if self.server.account_id() != Some(account_id) {
            return Task::ready(Err(anyhow::anyhow!(OTHER_ACCOUNT_ERROR)));
        }
        self.begin_membership_change();
        let server = self.server.clone();
        self.spawn_owned(cx, async move |engine, cx| {
            let result = server
                .join_group(account_id, JoinSyncedSettingsGroupBody { label, group })
                .await;
            if result.is_ok() {
                engine.update(cx, |engine, cx| engine.bump_hosts_revision(cx))?;
            }
            let new_group_id = result.as_ref().ok().map(|host| Some(host.group_id.clone()));
            Self::finish_membership_change(&engine, cx, account_id, new_group_id).await?;
            result
        })
    }

    pub fn list_hosts(&self, cx: &mut Context<Self>) -> Task<Result<SyncedSettingsHosts>> {
        let server = self.server.clone();
        cx.background_spawn(async move {
            let account_id = server.account_id().context("not signed in")?;
            let hosts = server.list_hosts(account_id).await?;
            Ok(SyncedSettingsHosts { account_id, hosts })
        })
    }

    pub fn update_host(
        &mut self,
        account_id: u64,
        host_system_id: String,
        body: UpdateSyncedSettingsHostBody,
        cx: &mut Context<Self>,
    ) -> Task<Result<SyncedSettingsHost>> {
        if self.server.account_id() != Some(account_id) {
            return Task::ready(Err(anyhow::anyhow!(OTHER_ACCOUNT_ERROR)));
        }
        let moves_this_device =
            body.group.is_some() && self.server.system_id().as_ref() == Some(&host_system_id);
        if moves_this_device {
            self.begin_membership_change();
        }
        let server = self.server.clone();
        self.spawn_owned(cx, async move |engine, cx| {
            let result = server.update_host(account_id, &host_system_id, body).await;
            if result.is_ok() {
                engine.update(cx, |engine, cx| engine.bump_hosts_revision(cx))?;
            }
            if moves_this_device {
                let new_group_id = result.as_ref().ok().map(|host| Some(host.group_id.clone()));
                Self::finish_membership_change(&engine, cx, account_id, new_group_id).await?;
            } else {
                engine.update(cx, |engine, cx| engine.sync_now(cx))?;
            }
            result
        })
    }

    pub fn remove_host(
        &mut self,
        account_id: u64,
        host_system_id: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.server.account_id() != Some(account_id) {
            return Task::ready(Err(anyhow::anyhow!(OTHER_ACCOUNT_ERROR)));
        }
        let removes_this_device = self.server.system_id().as_ref() == Some(&host_system_id);
        if removes_this_device {
            self.begin_membership_change();
        }
        let server = self.server.clone();
        self.spawn_owned(cx, async move |engine, cx| {
            let result = server.delete_host(account_id, &host_system_id).await;
            if result.is_ok() {
                engine.update(cx, |engine, cx| engine.bump_hosts_revision(cx))?;
            }
            if removes_this_device {
                let new_group_id = result.as_ref().ok().map(|()| None);
                Self::finish_membership_change(&engine, cx, account_id, new_group_id).await?;
            } else {
                engine.update(cx, |engine, cx| engine.sync_now(cx))?;
            }
            result
        })
    }

    pub fn list_history(&self, cx: &mut Context<Self>) -> Task<Result<SyncedSettingsHistory>> {
        let server = self.server.clone();
        let kind = self.document.kind;
        cx.background_spawn(async move {
            let account_id = server.account_id().context("not signed in")?;
            let history = server.list_history(account_id, kind, HISTORY_LIMIT).await?;
            Ok(SyncedSettingsHistory {
                account_id,
                group_id: history.group_id,
                revisions: history.revisions,
            })
        })
    }

    pub fn restore_revision(
        &mut self,
        revision: SyncedSettingsRevision,
        account_id: u64,
        group_id: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if !self.in_group(account_id, &group_id) {
            return Task::ready(Err(anyhow::anyhow!(
                "this history belongs to another account or sync group, not restoring it"
            )));
        }
        let generation = self.state_generation;
        self.spawn_owned(cx, async move |engine, cx| {
            Self::write_local_target(
                &engine,
                cx,
                move |engine, _| {
                    engine.fence_holds(generation) && engine.in_group(account_id, &group_id)
                },
                move |strategy, local_full, exclusions| {
                    let doc = if revision.schema_epoch < strategy.schema_epoch() {
                        strategy.migrate_doc(&revision.doc)?
                    } else {
                        revision.doc
                    };
                    let mut exclusions = exclusions.clone();
                    exclusions.extend_from_doc(&doc);
                    let mut revision_paths = strategy.flatten(&doc);
                    exclusions.strip(&mut revision_paths);
                    Ok(exclusions.restore(local_full, &revision_paths))
                },
            )
            .await
        })
    }

    pub fn revert_conflicts(
        &mut self,
        batch: ConflictBatch,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if !self.in_group(batch.account_id, &batch.group_id) {
            return Task::ready(Err(anyhow::anyhow!(
                "these conflicts belong to another account or sync group, not reverting them"
            )));
        }
        let reverted_paths = batch
            .conflicts
            .iter()
            .map(|conflict| conflict.path.clone())
            .collect::<Vec<_>>();
        let (account_id, group_id) = (batch.account_id, batch.group_id.clone());
        let generation = self.state_generation;
        self.spawn_owned(cx, async move |engine, cx| {
            Self::write_local_target(
                &engine,
                cx,
                move |engine, _| {
                    engine.fence_holds(generation) && engine.in_group(account_id, &group_id)
                },
                move |_, local_full, exclusions| {
                    let mut synced_local = local_full.clone();
                    exclusions.strip(&mut synced_local);
                    if let Some(changed) = batch.conflicts.iter().find(|conflict| {
                        synced_local.get(&conflict.path) != conflict.remote.as_ref()
                    }) {
                        anyhow::bail!(
                            "{} changed since the conflict, not reverting it",
                            changed.path
                        );
                    }
                    let mut target = local_full.clone();
                    for conflict in batch.conflicts {
                        target.retain(|path, _| {
                            !path.starts_with(&conflict.path) && !conflict.path.starts_with(path)
                        });
                        if let Some(value) = conflict.local {
                            target.insert(conflict.path, value);
                        }
                    }
                    Ok(target)
                },
            )
            .await?;
            engine.update(cx, |engine, cx| {
                if let Some(recent) = &mut engine.recent_conflicts {
                    recent
                        .conflicts
                        .retain(|conflict| !reverted_paths.contains(&conflict.path));
                    if recent.conflicts.is_empty() {
                        engine.recent_conflicts = None;
                    }
                }
                cx.notify();
            })
        })
    }

    pub fn erase_cloud_data(
        &mut self,
        scope: SyncedSettingsErasureScope,
        account_id: u64,
        expected_group_id: Option<String>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.erase_in_flight {
            return Task::ready(Err(anyhow::anyhow!("an erase is already in progress")));
        }
        if self.server.account_id() != Some(account_id) {
            return Task::ready(Err(anyhow::anyhow!(
                "signed into another account, not erasing"
            )));
        }
        if scope == SyncedSettingsErasureScope::Group && expected_group_id.is_none() {
            return Task::ready(Err(anyhow::anyhow!(
                "this device is not in a sync group, nothing to erase"
            )));
        }
        self.erase_in_flight = true;
        self.state_generation += 1;
        let (result_tx, result_rx) = oneshot::channel();
        let server = self.server.clone();
        let fs = self.fs.clone();
        let state_file_path = self.state_file_path.clone();
        let state_write_lock = self.state_write_lock.clone();
        self.erase_task = cx.spawn(async move |engine, cx| {
            let result = async {
                server.erase(account_id, scope, expected_group_id).await?;
                let state = SyncState {
                    format_version: SYNC_STATE_FORMAT_VERSION,
                    user_id: Some(account_id),
                    ..SyncState::default()
                };
                let _state_write_guard = state_write_lock.lock().await;
                persist_state(fs.as_ref(), &state_file_path, None, &state).await?;
                engine.update(cx, |engine, cx| {
                    engine.state_generation += 1;
                    engine.state = Some(state);
                    engine.pending_decision = None;
                    engine.approved_first_sync = None;
                    engine.recent_conflicts = None;
                    engine.status = SettingsSyncStatus::default();
                    settings::update_settings_file(engine.fs.clone(), cx, |content, _| {
                        content.settings_sync.get_or_insert_default().enabled = Some(false);
                    });
                    cx.emit(SettingsSyncEvent::Erased);
                    cx.notify();
                })
            }
            .await;
            engine
                .update(cx, |engine, _| engine.erase_in_flight = false)
                .ok();
            result_tx.send(result).ok();
        });
        cx.background_spawn(async move {
            result_rx
                .await
                .context("the erase was interrupted before it finished")?
        })
    }

    pub fn handle_remote_changed(
        &mut self,
        group_id: &str,
        kind: &str,
        version: u64,
        document_id: &str,
        cx: &mut Context<Self>,
    ) {
        if kind != self.document.kind {
            return;
        }
        if let Some(state) = &self.state {
            if state
                .group_id
                .as_deref()
                .is_some_and(|our_group_id| our_group_id != group_id)
            {
                return;
            }
            if state.base.as_ref().is_some_and(|base| {
                base.document_id == document_id && base.server_version >= version
            }) {
                return;
            }
        }
        self.schedule_sync(cx);
    }

    pub fn handle_remote_invalidated(&mut self, cx: &mut Context<Self>) {
        self.schedule_sync(cx);
    }

    fn bump_hosts_revision(&mut self, cx: &mut Context<Self>) {
        self.hosts_revision += 1;
        cx.notify();
    }

    fn fence_holds(&self, generation: u64) -> bool {
        self.state_generation == generation
            && !self.erase_in_flight
            && self.membership_changes_in_flight == 0
    }

    fn begin_membership_change(&mut self) {
        self.state_generation += 1;
        self.membership_changes_in_flight += 1;
    }

    async fn finish_membership_change(
        engine: &WeakEntity<Self>,
        cx: &mut AsyncApp,
        account_id: u64,
        new_group_id: Option<Option<String>>,
    ) -> Result<()> {
        let (fs, server, state_file_path, state_write_lock) =
            engine.read_with(cx, |engine, _| {
                (
                    engine.fs.clone(),
                    engine.server.clone(),
                    engine.state_file_path.clone(),
                    engine.state_write_lock.clone(),
                )
            })?;
        let result = async {
            let Some(new_group_id) = new_group_id else {
                return Ok(());
            };
            let _state_write_guard = state_write_lock.lock().await;
            let Some(state) = update_state(engine, cx, fs.as_ref(), &state_file_path, |state| {
                if server.account_id() != Some(account_id) || state.user_id != Some(account_id) {
                    return false;
                }
                state.group_id = new_group_id.clone();
                state.base = None;
                true
            })
            .await?
            else {
                return Ok(());
            };
            engine.update(cx, |engine, _| {
                engine.state = Some(state);
                engine.pending_decision = None;
                engine.approved_first_sync = None;
            })
        }
        .await;
        engine.update(cx, |engine, cx| {
            engine.state_generation += 1;
            engine.membership_changes_in_flight -= 1;
            cx.notify();
            engine.sync_now(cx);
        })?;
        result
    }

    fn spawn_owned<T: Send + 'static>(
        &mut self,
        cx: &mut Context<Self>,
        operation: impl AsyncFnOnce(WeakEntity<Self>, &mut AsyncApp) -> Result<T> + 'static,
    ) -> Task<Result<T>> {
        let task_id = self.next_owned_task_id;
        self.next_owned_task_id += 1;
        let (result_tx, result_rx) = oneshot::channel();
        let task = cx.spawn(async move |engine, cx| {
            let result = operation(engine.clone(), cx).await;
            result_tx.send(result).ok();
            engine
                .update(cx, |engine, _| engine.owned_tasks.remove(&task_id))
                .ok();
        });
        self.owned_tasks.insert(task_id, task);
        cx.background_spawn(async move {
            result_rx
                .await
                .context("the settings sync operation was interrupted")?
        })
    }

    fn send_trigger(&mut self, trigger: SyncTrigger) {
        if self.paused {
            return;
        }
        self.sync_tx.unbounded_send(trigger).ok();
    }

    fn record_cycle_result(&mut self, result: Result<()>, cx: &mut Context<Self>) {
        match result {
            Ok(()) => {
                self.status.last_error = None;
                self.failure_notified = false;
            }
            Err(error) => {
                let message = format!("{error:#}");
                self.status.last_error = Some(message.clone());
                if !self.failure_notified && error.downcast_ref::<InvalidLocalDocument>().is_none()
                {
                    self.failure_notified = true;
                    cx.emit(SettingsSyncEvent::SyncFailed(message));
                }
            }
        }
        cx.notify();
    }

    fn handle_flag_inputs_changed(&mut self, cx: &mut Context<Self>) {
        let flag_enabled = cx.has_flag::<SettingsSyncFeatureFlag>();
        if flag_enabled && !self.flag_enabled {
            self.schedule_sync(cx);
        }
        self.flag_enabled = flag_enabled;
    }

    async fn ensure_strategy(
        engine: &WeakEntity<Self>,
        cx: &mut AsyncApp,
    ) -> Result<Arc<dyn SyncStrategy>> {
        let (strategy, build_strategy) = engine.read_with(cx, |engine, _| {
            (engine.strategy.clone(), engine.document.build_strategy)
        })?;
        if let Some(strategy) = strategy {
            return Ok(strategy);
        }
        let strategy = cx.background_spawn(async move { build_strategy() }).await;
        engine.update(cx, |engine, _| {
            engine.strategy = Some(strategy.clone());
        })?;
        Ok(strategy)
    }

    async fn write_local_target(
        engine: &WeakEntity<Self>,
        cx: &mut AsyncApp,
        still_valid: impl 'static + FnOnce(&Self, &App) -> bool,
        build_target: impl FnOnce(&dyn SyncStrategy, &PathMap, &ExclusionSet) -> Result<PathMap>,
    ) -> Result<()> {
        let strategy = Self::ensure_strategy(engine, cx).await?;
        let (fs, document_path) = engine.read_with(cx, |engine, _| {
            (engine.fs.clone(), engine.document.file_path.clone())
        })?;
        let document_path = resolve_document_path(fs.as_ref(), &document_path).await;
        let text = fs
            .load(&document_path)
            .await
            .context("loading the settings file")?;
        let local_doc = parse_local_document(&text)?;
        let local_full = strategy.flatten(&local_doc);
        let exclusions = exclusions_for(strategy.as_ref(), [&local_doc]);
        let target = build_target(strategy.as_ref(), &local_full, &exclusions)?;
        let new_text = strategy.apply(&text, &local_full, &target)?;
        if new_text != text {
            anyhow::ensure!(
                Self::write_document_if_unchanged(
                    engine,
                    cx,
                    document_path,
                    text,
                    new_text,
                    still_valid
                )
                .await?,
                "the settings file or the sync group changed while writing synced settings, try again"
            );
        }
        engine.update(cx, |engine, cx| engine.sync_now(cx))?;
        Ok(())
    }

    async fn write_document_if_unchanged(
        engine: &WeakEntity<Self>,
        cx: &mut AsyncApp,
        document_path: PathBuf,
        expected_text: String,
        new_text: String,
        still_valid: impl 'static + FnOnce(&Self, &App) -> bool,
    ) -> Result<bool> {
        let (fs, state_file_path) = engine.read_with(cx, |engine, _| {
            (engine.fs.clone(), engine.state_file_path.clone())
        })?;
        write_pre_apply_backup(fs.as_ref(), &state_file_path, &expected_text).await;
        let (result_tx, result_rx) = oneshot::channel();
        let engine = engine.clone();
        cx.update(|cx| {
            cx.global::<SettingsStore>()
                .enqueue_settings_file_job(move |mut cx| {
                    async move {
                        let result = async {
                            if !engine.read_with(&cx, |engine, cx| still_valid(engine, cx))? {
                                return Ok(false);
                            }
                            let previous_self_write = engine.update(&mut cx, |engine, _| {
                                engine.last_self_write.replace(new_text.clone())
                            })?;
                            let written = fs
                                .atomic_write_if_unchanged(document_path, expected_text, new_text)
                                .await
                                .context("writing the settings file");
                            if !matches!(written, Ok(true)) {
                                engine.update(&mut cx, |engine, _| {
                                    engine.last_self_write = previous_self_write;
                                })?;
                            }
                            written
                        }
                        .await;
                        result_tx.send(result).ok();
                        Ok(())
                    }
                    .boxed_local()
                })
        });
        result_rx
            .await
            .context("the settings file write was dropped")?
    }

    fn in_group(&self, account_id: u64, group_id: &str) -> bool {
        self.server.account_id() == Some(account_id)
            && self.state.as_ref().is_some_and(|state| {
                state.user_id == Some(account_id) && state.group_id.as_deref() == Some(group_id)
            })
    }

    async fn run_sync(engine: &WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let (fs, server, state_file_path, enabled, paused, flag_enabled) =
            engine.read_with(cx, |engine, cx| {
                (
                    engine.fs.clone(),
                    engine.server.clone(),
                    engine.state_file_path.clone(),
                    sync_enabled(cx),
                    engine.paused,
                    cx.has_flag::<SettingsSyncFeatureFlag>(),
                )
            })?;
        let (document_kind, document_path, generation, fence_holds) =
            engine.read_with(cx, |engine, _| {
                (
                    engine.document.kind,
                    engine.document.file_path.clone(),
                    engine.state_generation,
                    engine.fence_holds(engine.state_generation),
                )
            })?;
        if !fence_holds {
            return Ok(());
        }
        // TODO kb cloud: remove the feature flag gate at GA; until then
        // enabled-but-flag-off is a warn-once no-op.
        if !flag_enabled {
            if enabled {
                engine.update(cx, |engine, _| {
                    if !engine.flag_gate_notified {
                        engine.flag_gate_notified = true;
                        log::warn!(
                            "settings sync: enabled in settings, but the feature flag is off; \
                             not syncing"
                        );
                    }
                })?;
            }
            return Ok(());
        }
        if !enabled || paused || !server.is_ready() {
            return Ok(());
        }
        let Some(account_id) = server.account_id() else {
            return Ok(());
        };

        let strategy = Self::ensure_strategy(engine, cx).await?;
        let schema_epoch = strategy.schema_epoch();

        let loaded_state = load_state_file(fs.as_ref(), &state_file_path).await;
        let memory_state = engine.read_with(cx, |engine, _| engine.state.clone())?;
        let mut state = current_state(memory_state, loaded_state.state);
        let state_before_sync = StateSnapshot {
            state: state.clone(),
            text: loaded_state.text,
        };
        if state.user_id != Some(account_id) {
            if state.user_id.is_some() {
                log::info!("settings sync: signed into another account, starting over");
            }
            state = SyncState {
                user_id: Some(account_id),
                ..SyncState::default()
            };
        }
        state.format_version = SYNC_STATE_FORMAT_VERSION;

        if !state.consented {
            set_pending_decision(engine, Some(PendingDecision::Consent), cx)?;
            return store_state(
                engine,
                cx,
                fs.as_ref(),
                &state_file_path,
                state,
                &state_before_sync,
                generation,
            )
            .await;
        }

        let document_path = resolve_document_path(fs.as_ref(), &document_path).await;
        let local_text = match fs.load(&document_path).await {
            Ok(text) => text,
            Err(error) => {
                if state.base.is_some() {
                    return Err(error).context("reading the settings file");
                }
                String::new()
            }
        };
        parse_local_document(&local_text).context(InvalidLocalDocument)?;
        let migration = if local_text.trim().is_empty() {
            Ok(None)
        } else {
            strategy.migrate(&local_text)
        };
        let local_text = match migration.context("migrating the settings file")? {
            Some(migrated_text) => {
                log::info!("settings sync: migrating the settings file before syncing");
                if !Self::write_document_if_unchanged(
                    engine,
                    cx,
                    document_path.clone(),
                    local_text,
                    migrated_text.clone(),
                    |_, _| true,
                )
                .await?
                {
                    engine.update(cx, |engine, cx| engine.schedule_sync(cx))?;
                    return Ok(());
                }
                migrated_text
            }
            None => local_text,
        };
        let local_doc = parse_local_document(&local_text).context(InvalidLocalDocument)?;
        let local_full = strategy.flatten(&local_doc);

        let mut fetched = match server.fetch(account_id, document_kind).await {
            Ok(fetched) => fetched,
            Err(_) if server.account_id() != Some(account_id) => return Ok(()),
            Err(error) => return Err(error).context("fetching synced settings"),
        };

        let mut observed_remote = false;
        let mut attempts = 0;
        let outcome = loop {
            if server.account_id() != Some(account_id) {
                return Ok(());
            }
            let FetchResult {
                group_id,
                document: remote,
            } = fetched;
            let Some(group_id) = group_id else {
                let hosts = server
                    .list_hosts(account_id)
                    .await
                    .context("listing settings sync hosts")?;
                if server.account_id() != Some(account_id) {
                    return Ok(());
                }
                state.group_id = None;
                state.base = None;
                set_pending_decision(
                    engine,
                    Some(PendingDecision::JoinGroup { account_id, hosts }),
                    cx,
                )?;
                return store_state(
                    engine,
                    cx,
                    fs.as_ref(),
                    &state_file_path,
                    state,
                    &state_before_sync,
                    generation,
                )
                .await;
            };
            let moved_unexpectedly = state
                .group_id
                .as_ref()
                .is_some_and(|known_group_id| *known_group_id != group_id);
            if state.group_id.as_deref() != Some(group_id.as_str()) {
                if moved_unexpectedly {
                    log::warn!(
                        "settings sync: this device was moved to another sync group or its group was erased"
                    );
                }
                state.base = None;
                state.group_id = Some(group_id.clone());
            }
            let remote = remote
                .map(|remote| migrate_remote(strategy.as_ref(), remote))
                .transpose()?;

            let erased = moved_unexpectedly
                || match (&remote, &state.base) {
                    (None, Some(_)) => true,
                    (None, None) => observed_remote,
                    (Some(remote), Some(base)) => remote.document_id != base.document_id,
                    (Some(_), None) => false,
                };
            if erased {
                log::warn!(
                    "settings sync: the synced settings were erased from the cloud, asking before uploading again"
                );
                state.base = None;
                state.consented = false;
                set_pending_decision(engine, Some(PendingDecision::Consent), cx)?;
                return store_state(
                    engine,
                    cx,
                    fs.as_ref(),
                    &state_file_path,
                    state,
                    &state_before_sync,
                    generation,
                )
                .await;
            }
            observed_remote |= remote.is_some();

            let remote_full = remote
                .as_ref()
                .map(|remote| strategy.flatten(&remote.doc))
                .unwrap_or_default();
            let exclusions = exclusions_for(
                strategy.as_ref(),
                std::iter::once(&local_doc).chain(remote.as_ref().map(|remote| &remote.doc)),
            );

            let mut local_view = local_full.clone();
            let mut remote_paths = remote_full.clone();
            exclusions.strip(&mut local_view);
            exclusions.strip(&mut remote_paths);
            let mut base_paths = match &state.base {
                Some(base) => {
                    let base_doc = if base.schema_epoch < schema_epoch {
                        strategy.migrate_doc(&base.doc)?
                    } else {
                        base.doc.clone()
                    };
                    Some(strategy.flatten(&base_doc))
                }
                None => None,
            };
            if let Some(base_paths) = &mut base_paths {
                exclusions.strip(base_paths);
            }

            let mut used_approval = None;
            let needs_first_sync_choice = match &remote {
                Some(_) => local_view != remote_paths,
                None => !local_view.is_empty(),
            };
            let merge = match (&base_paths, &remote) {
                (Some(base_paths), _) => merge_three_way(base_paths, &local_view, &remote_paths),
                (None, remote) if needs_first_sync_choice => {
                    let preview = FirstSyncPreview {
                        account_id,
                        group_id: group_id.clone(),
                        remote_document_id: remote
                            .as_ref()
                            .map(|remote| remote.document_id.clone()),
                        remote_version: remote.as_ref().map(|remote| remote.version),
                        local: local_view.clone(),
                        remote: remote_paths.clone(),
                        can_replace_remote: remote
                            .as_ref()
                            .is_none_or(|remote| remote.schema_epoch <= schema_epoch),
                    };
                    let choice = engine.update(cx, |engine, _| {
                        let approved = engine.approved_first_sync.as_ref()?;
                        (approved.preview == preview).then_some(approved.choice)
                    })?;
                    let Some(choice) = choice else {
                        engine.update(cx, |engine, _| engine.approved_first_sync = None)?;
                        set_pending_decision(
                            engine,
                            Some(PendingDecision::FirstSync(preview)),
                            cx,
                        )?;
                        return store_state(
                            engine,
                            cx,
                            fs.as_ref(),
                            &state_file_path,
                            state,
                            &state_before_sync,
                            generation,
                        )
                        .await;
                    };
                    let merge = preview.merge(choice);
                    used_approval = Some(preview);
                    merge
                }
                (None, _) => merge_three_way(&PathMap::new(), &local_view, &remote_paths),
            };
            set_pending_decision(engine, None, cx)?;

            // TODO kb cloud: observer semantics for renamed keys — pulling a migrated
            // doc deletes the old-shape keys locally, regressing the setting's effect
            // until this Zed updates; document, or skip deletions for observers.
            let observer_mode = remote
                .as_ref()
                .is_some_and(|remote| remote.schema_epoch > schema_epoch);
            if observer_mode {
                engine.update(cx, |engine, cx| {
                    if !engine.update_required_notified {
                        engine.update_required_notified = true;
                        log::warn!(
                            "settings sync: the cloud document was written by a newer Zed; \
                             pulling only until this Zed is updated"
                        );
                        cx.emit(SettingsSyncEvent::UpdateRequired);
                    }
                })?;
            }

            let cloud_is_current = remote.as_ref().is_some_and(|remote| {
                remote_full == merge.merged && remote.schema_epoch >= schema_epoch
            });
            if cloud_is_current || observer_mode {
                break CycleOutcome {
                    group_id,
                    new_base: remote.as_ref().map(base_from_remote),
                    remote,
                    base_paths: base_paths.unwrap_or_default(),
                    merged: merge.merged,
                    conflicts: merge.conflicts,
                    used_approval,
                };
            }

            let push_valid = server.account_id() == Some(account_id)
                && push_inputs_unchanged(
                    engine,
                    cx,
                    fs.as_ref(),
                    &document_path,
                    &local_text,
                    generation,
                )
                .await?
                && server.account_id() == Some(account_id);
            if !push_valid {
                log::info!("settings sync: inputs changed mid-cycle, skipping the push");
                if server.account_id() == Some(account_id) {
                    engine.update(cx, |engine, cx| engine.schedule_sync(cx))?;
                }
                return Ok(());
            }
            let doc = strategy.unflatten(&merge.merged);
            let push_result = match server
                .push(
                    account_id,
                    UpdateSyncedSettingsBody {
                        kind: document_kind.to_string(),
                        base_version: remote.as_ref().map(|remote| remote.version),
                        base_document_id: remote.as_ref().map(|remote| remote.document_id.clone()),
                        schema_epoch,
                        doc: doc.clone(),
                        expected_group_id: group_id.clone(),
                        exclusions: exclusions.pointers(),
                    },
                )
                .await
            {
                Ok(push_result) => push_result,
                Err(_) if server.account_id() != Some(account_id) => return Ok(()),
                Err(error) => return Err(error).context("pushing synced settings"),
            };

            match push_result {
                PushResult::Written {
                    version,
                    group_id: written_group_id,
                    document_id,
                } => {
                    state.group_id = Some(written_group_id.clone());
                    break CycleOutcome {
                        group_id: written_group_id,
                        remote,
                        base_paths: base_paths.unwrap_or_default(),
                        merged: merge.merged,
                        conflicts: merge.conflicts,
                        new_base: Some(SyncBase {
                            document_id,
                            server_version: version,
                            schema_epoch,
                            doc,
                        }),
                        used_approval,
                    };
                }
                PushResult::Conflict {
                    group_id: conflict_group_id,
                    current,
                } => {
                    if conflict_group_id.as_deref() != Some(group_id.as_str()) {
                        log::info!("settings sync: sync group changed mid-cycle, restarting");
                        engine.update(cx, |engine, cx| engine.schedule_sync(cx))?;
                        return store_state(
                            engine,
                            cx,
                            fs.as_ref(),
                            &state_file_path,
                            state,
                            &state_before_sync,
                            generation,
                        )
                        .await;
                    }
                    attempts += 1;
                    if attempts >= MAX_PUSH_ATTEMPTS {
                        log::warn!(
                            "settings sync: pausing after {attempts} conflicting pushes in a row"
                        );
                        engine.update(cx, |engine, cx| {
                            engine.paused = true;
                            cx.emit(SettingsSyncEvent::Paused(PauseReason::RepeatedConflicts));
                            cx.notify();
                        })?;
                        return store_state(
                            engine,
                            cx,
                            fs.as_ref(),
                            &state_file_path,
                            state,
                            &state_before_sync,
                            generation,
                        )
                        .await;
                    }
                    let jitter = Duration::from_millis(
                        rand::rng().random_range(0..=PUSH_RETRY_MAX_JITTER_MILLIS),
                    );
                    cx.background_executor()
                        .timer(PUSH_RETRY_BACKOFF * attempts as u32 + jitter)
                        .await;
                    fetched = FetchResult {
                        group_id: conflict_group_id,
                        document: current,
                    };
                }
                PushResult::NotRegistered => {
                    log::info!("settings sync: this device left its sync group, restarting");
                    state.group_id = None;
                    state.base = None;
                    engine.update(cx, |engine, cx| engine.schedule_sync(cx))?;
                    return store_state(
                        engine,
                        cx,
                        fs.as_ref(),
                        &state_file_path,
                        state,
                        &state_before_sync,
                        generation,
                    )
                    .await;
                }
            }
        };

        if server.account_id() != Some(account_id) {
            log::info!("settings sync: account changed mid-cycle, skipping the apply");
            return Ok(());
        }
        if !sync_still_enabled(engine, cx)? {
            log::info!("settings sync: disabled mid-cycle, skipping the apply");
            return Ok(());
        }

        let CycleOutcome {
            group_id,
            remote,
            base_paths,
            merged,
            conflicts,
            new_base,
            used_approval,
        } = outcome;
        let exclusions = exclusions_for(
            strategy.as_ref(),
            std::iter::once(&local_doc).chain(remote.as_ref().map(|remote| &remote.doc)),
        );
        let mut local_view = local_full.clone();
        exclusions.strip(&mut local_view);

        let mut conflicts = conflicts
            .into_iter()
            .filter(|conflict| {
                let local_value = local_view.get(&conflict.path);
                local_value != base_paths.get(&conflict.path)
                    && local_value != merged.get(&conflict.path)
            })
            .map(|conflict| Conflict {
                local: local_view.get(&conflict.path).cloned(),
                remote: merged.get(&conflict.path).cloned(),
                changed_by: remote
                    .as_ref()
                    .and_then(|remote| conflict_author(remote, &conflict.path)),
                ..conflict
            })
            .collect::<Vec<_>>();

        let target = exclusions.restore(&local_full, &merged);
        let mut applied = true;
        if target != local_full {
            match strategy.apply(&local_text, &local_full, &target) {
                Ok(new_text) => {
                    if new_text != local_text
                        && !Self::write_document_if_unchanged(
                            engine,
                            cx,
                            document_path.clone(),
                            local_text.clone(),
                            new_text,
                            move |engine, cx| {
                                engine.fence_holds(generation)
                                    && engine.server.account_id() == Some(account_id)
                                    && sync_enabled(cx)
                                    && !engine.paused
                            },
                        )
                        .await?
                    {
                        log::info!(
                            "settings sync: settings file or sync state changed mid-cycle, rescheduling"
                        );
                        engine.update(cx, |engine, cx| engine.schedule_sync(cx))?;
                        applied = false;
                    }
                }
                Err(error) => {
                    log::error!(
                        "settings sync: pausing, could not apply synced settings: {error:#}"
                    );
                    applied = false;
                    engine.update(cx, |engine, cx| {
                        engine.paused = true;
                        cx.emit(SettingsSyncEvent::Paused(PauseReason::ApplyFailed));
                    })?;
                }
            }
        }

        let local_after = if applied {
            strategy.unflatten(&target)
        } else {
            local_doc.clone()
        };
        let exclusions_settled = new_base.as_ref().is_none_or(|new_base| {
            exclusions_for(strategy.as_ref(), [&local_after, &new_base.doc]) == exclusions
        });
        let synced = applied && new_base.is_some();
        if let Some(new_base) = new_base {
            state.base = Some(if applied {
                new_base
            } else {
                SyncBase {
                    schema_epoch,
                    doc: strategy.unflatten(&local_view),
                    ..new_base
                }
            });
        }
        let mut conflict_labels = BTreeMap::new();
        if !conflicts.is_empty()
            && let Ok(hosts) = server.list_hosts(account_id).await
        {
            conflict_labels.extend(hosts.into_iter().map(|host| (host.system_id, host.label)));
        }
        for conflict in &mut conflicts {
            if conflict.changed_by == server.system_id() {
                conflict.changed_by = None;
            }
            if let Some(label) = conflict
                .changed_by
                .as_ref()
                .and_then(|system_id| conflict_labels.get(system_id))
            {
                conflict.changed_by = Some(label.clone());
            }
        }

        let server_version = state.base.as_ref().map(|base| base.server_version);
        let status_group_id = state.group_id.clone();
        store_state(
            engine,
            cx,
            fs.as_ref(),
            &state_file_path,
            state,
            &state_before_sync,
            generation,
        )
        .await?;
        engine.update(cx, |engine, cx| {
            if synced {
                if let Some(used_approval) = &used_approval
                    && engine
                        .approved_first_sync
                        .as_ref()
                        .is_some_and(|approved| approved.preview == *used_approval)
                {
                    engine.approved_first_sync = None;
                }
                engine.status.last_synced_at = Some(SystemTime::now());
                engine.status.server_version = server_version;
                engine.status.group_id = status_group_id;
                if !exclusions_settled {
                    engine.schedule_sync(cx);
                }
            }
            if synced {
                let mut retained = engine
                    .recent_conflicts
                    .take()
                    .filter(|recent| recent.account_id == account_id && recent.group_id == group_id)
                    .map(|recent| recent.conflicts)
                    .unwrap_or_default();
                retained.retain(|recent| {
                    merged.get(&recent.path) == recent.remote.as_ref()
                        && conflicts
                            .iter()
                            .all(|conflict| conflict.path != recent.path)
                });
                retained.extend(conflicts.iter().cloned());
                retained.sort_by(|left, right| left.path.cmp(&right.path));
                engine.recent_conflicts = (!retained.is_empty()).then(|| ConflictBatch {
                    account_id,
                    group_id: group_id.clone(),
                    conflicts: retained,
                });
            }
            if !conflicts.is_empty() {
                for conflict in &conflicts {
                    log::warn!(
                        "settings sync: conflict at {}, kept the remote side from {}",
                        conflict.path,
                        conflict.changed_by.as_deref().unwrap_or("another device")
                    );
                }
                cx.emit(SettingsSyncEvent::ConflictsResolved(ConflictBatch {
                    account_id,
                    group_id,
                    conflicts,
                }));
            }
            cx.notify();
        })?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) async fn load_state(fs: &dyn Fs, state_file_path: &Path) -> SyncState {
    current_state(None, load_state_file(fs, state_file_path).await.state)
}

async fn load_state_file(fs: &dyn Fs, state_file_path: &Path) -> LoadedState {
    let text = match fs.load(state_file_path).await {
        Ok(text) => text,
        Err(error) => {
            let missing = error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound)
                || !fs.is_file(state_file_path).await;
            if !missing {
                log::warn!("settings sync: failed to read the state file: {error:#}");
            }
            return LoadedState {
                state: missing.then(SyncState::default),
                text: String::new(),
            };
        }
    };
    let state = match serde_json::from_str::<SyncState>(&text) {
        Ok(state) if state.format_version <= SYNC_STATE_FORMAT_VERSION => Some(state),
        Ok(state) => {
            log::warn!(
                "settings sync: state file has a newer format ({}), starting fresh",
                state.format_version
            );
            None
        }
        Err(error) => {
            log::warn!("settings sync: failed to parse state file, starting fresh: {error:#}");
            None
        }
    };
    LoadedState { state, text }
}

fn current_state(memory: Option<SyncState>, disk: Option<SyncState>) -> SyncState {
    disk.or(memory).unwrap_or_default()
}

async fn update_state(
    engine: &WeakEntity<SettingsSyncEngine>,
    cx: &mut AsyncApp,
    fs: &dyn Fs,
    state_file_path: &Path,
    mut update: impl FnMut(&mut SyncState) -> bool,
) -> Result<Option<SyncState>> {
    for _ in 0..MAX_STATE_WRITE_ATTEMPTS {
        let loaded_state = load_state_file(fs, state_file_path).await;
        let memory_state = engine.read_with(cx, |engine, _| engine.state.clone())?;
        let mut state = current_state(memory_state, loaded_state.state);
        if !update(&mut state) {
            return Ok(None);
        }
        if persist_state(fs, state_file_path, Some(&loaded_state.text), &state).await? {
            return Ok(Some(state));
        }
    }
    anyhow::bail!("another Zed process kept changing the settings sync state, try again")
}

fn base_from_remote(remote: &SyncedSettings) -> SyncBase {
    SyncBase {
        document_id: remote.document_id.clone(),
        server_version: remote.version,
        schema_epoch: remote.schema_epoch,
        doc: remote.doc.clone(),
    }
}

fn migrate_remote(
    strategy: &dyn SyncStrategy,
    mut remote: SyncedSettings,
) -> Result<SyncedSettings> {
    if remote.schema_epoch < strategy.schema_epoch() {
        remote.doc = strategy
            .migrate_doc(&remote.doc)
            .context("migrating the synced settings document")?;
    }
    Ok(remote)
}

fn conflict_author(remote: &SyncedSettings, path: &SyncPath) -> Option<String> {
    let pointer = path.to_string();
    let descendant_prefix = format!("{pointer}/");
    remote
        .provenance
        .iter()
        .filter(|(provenance_path, _)| {
            **provenance_path == pointer || provenance_path.starts_with(&descendant_prefix)
        })
        .max_by_key(|(_, provenance)| provenance.version)
        .and_then(|(_, provenance)| provenance.system_id.clone())
        .or_else(|| remote.updated_by_system_id.clone())
}

fn parse_local_document(text: &str) -> Result<Value> {
    let doc = if text.trim().is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        parse_json_with_comments::<Value>(text).context("settings file failed to parse")?
    };
    anyhow::ensure!(doc.is_object(), "settings file root is not an object");
    Ok(doc)
}

async fn resolve_document_path(fs: &dyn Fs, path: &Path) -> PathBuf {
    if fs.is_file(path).await
        && let Ok(resolved_path) = fs.canonicalize(path).await
    {
        resolved_path
    } else {
        path.to_path_buf()
    }
}

fn exclusions_for<'a>(
    strategy: &dyn SyncStrategy,
    docs: impl IntoIterator<Item = &'a Value>,
) -> ExclusionSet {
    let docs = docs.into_iter().collect::<Vec<_>>();
    let mut exclusions = strategy.built_in_exclusions();
    exclusions.extend(strategy.user_exclusions(&docs));
    exclusions
}

async fn push_inputs_unchanged(
    engine: &WeakEntity<SettingsSyncEngine>,
    cx: &AsyncApp,
    fs: &dyn Fs,
    document_path: &Path,
    local_text: &str,
    generation: u64,
) -> Result<bool> {
    let inputs_valid = |cx: &AsyncApp| {
        engine.read_with(cx, |engine, cx| {
            engine.fence_holds(generation) && !engine.paused && sync_enabled(cx)
        })
    };
    if !inputs_valid(cx)? {
        return Ok(false);
    }
    let text_on_disk = fs.load(document_path).await.unwrap_or_default();
    Ok(text_on_disk == local_text && inputs_valid(cx)?)
}

fn set_pending_decision(
    engine: &WeakEntity<SettingsSyncEngine>,
    decision: Option<PendingDecision>,
    cx: &mut AsyncApp,
) -> Result<()> {
    engine.update(cx, |engine, cx| {
        if engine.pending_decision != decision {
            let requires_decision = decision.is_some();
            engine.pending_decision = decision;
            if requires_decision {
                cx.emit(SettingsSyncEvent::DecisionRequired);
            }
            cx.notify();
        }
    })
}

async fn store_state(
    engine: &WeakEntity<SettingsSyncEngine>,
    cx: &mut AsyncApp,
    fs: &dyn Fs,
    state_file_path: &Path,
    state: SyncState,
    state_before_sync: &StateSnapshot,
    generation: u64,
) -> Result<()> {
    let state_write_lock = engine.read_with(cx, |engine, _| engine.state_write_lock.clone())?;
    let _state_write_guard = state_write_lock.lock().await;
    if engine.read_with(cx, |engine, _| engine.state_generation)? != generation {
        log::info!("settings sync: sync state was reset mid-cycle, discarding the cycle's state");
        return Ok(());
    }
    if state != state_before_sync.state
        && !persist_state(fs, state_file_path, Some(&state_before_sync.text), &state)
            .await
            .context("persisting sync state")?
    {
        log::info!(
            "settings sync: another Zed process changed the sync state mid-cycle, rescheduling"
        );
        return engine.update(cx, |engine, cx| engine.schedule_sync(cx));
    }
    engine.update(cx, |engine, _| {
        if engine.state_generation == generation {
            engine.state = Some(state);
        }
    })
}

fn sync_still_enabled(engine: &WeakEntity<SettingsSyncEngine>, cx: &AsyncApp) -> Result<bool> {
    engine.read_with(cx, |engine, cx| sync_enabled(cx) && !engine.paused)
}

fn pre_apply_backup_file_name(slot: usize) -> String {
    if slot == 0 {
        "settings_pre_apply_backup.json".to_string()
    } else {
        format!("settings_pre_apply_backup.{slot}.json")
    }
}

async fn write_pre_apply_backup(fs: &dyn Fs, state_file_path: &Path, settings_text: &str) {
    let Some(parent) = state_file_path.parent() else {
        return;
    };
    if let Err(error) = fs.create_dir(parent).await {
        log::warn!("settings sync: failed to create the backup directory: {error:#}");
        return;
    }
    for slot in (1..PRE_APPLY_BACKUP_SLOTS).rev() {
        let source = parent.join(pre_apply_backup_file_name(slot - 1));
        let target = parent.join(pre_apply_backup_file_name(slot));
        if fs.is_file(&source).await
            && let Err(error) = fs
                .rename(
                    &source,
                    &target,
                    RenameOptions {
                        overwrite: true,
                        ..RenameOptions::default()
                    },
                )
                .await
        {
            log::warn!("settings sync: failed to rotate a pre-apply backup: {error:#}");
        }
    }
    if let Err(error) = fs
        .atomic_write(
            parent.join(pre_apply_backup_file_name(0)),
            settings_text.to_string(),
        )
        .await
    {
        log::warn!("settings sync: failed to write the pre-apply backup: {error:#}");
    }
}

async fn persist_state(
    fs: &dyn Fs,
    state_file_path: &Path,
    expected_text: Option<&str>,
    state: &SyncState,
) -> Result<bool> {
    if let Some(parent) = state_file_path.parent() {
        fs.create_dir(parent).await?;
    }
    let text = serde_json::to_string_pretty(state)?;
    match expected_text {
        Some(expected_text) => {
            fs.atomic_write_if_unchanged(
                state_file_path.to_path_buf(),
                expected_text.to_string(),
                text,
            )
            .await
        }
        None => {
            fs.atomic_write(state_file_path.to_path_buf(), text).await?;
            Ok(true)
        }
    }
}
