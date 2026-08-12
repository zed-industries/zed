use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use cloud_api_client::{SYNCED_SETTINGS_KIND_SETTINGS, UpdateSyncedSettingsBody};
use feature_flags::{FeatureFlagAppExt as _, SettingsSyncFeatureFlag};
use fs::{Fs, RenameOptions};
use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{AppContext as _, AsyncApp, Context, EventEmitter, Task, WeakEntity};
use rand::Rng as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{Settings as _, watch_config_file};
use settings_json::parse_json_with_comments;

use crate::classifier::DocumentClassifier;
use crate::merge::{
    Conflict, ExclusionSet, PathMap, SyncOp, apply_ops_to_text, diff_paths, drop_prefix_overlaps,
    flatten_doc, merge_three_way, unflatten,
};
use crate::server::{PushResult, SettingsSyncServer, SyncNotImplementedError};
use crate::{SettingsSyncSettings, settings_schema_epoch};

pub(crate) const MAX_PUSH_ATTEMPTS: usize = 3;
pub(crate) const SYNC_DEBOUNCE: Duration = Duration::from_secs(1);
const PUSH_RETRY_BACKOFF: Duration = Duration::from_millis(500);
const PUSH_RETRY_MAX_JITTER_MILLIS: u64 = 250;

const SYNC_STATE_FORMAT_VERSION: u64 = 1;
const PRE_APPLY_BACKUP_SLOTS: usize = 3;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SyncState {
    pub format_version: u64,
    pub group_id: Option<String>,
    pub base: Option<SyncBase>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncBase {
    pub server_version: u64,
    pub schema_epoch: u64,
    pub doc: Value,
}

pub struct SyncedDocument {
    pub kind: &'static str,
    pub file_path: PathBuf,
    pub build_classifier: fn() -> DocumentClassifier,
}

impl SyncedDocument {
    pub fn user_settings() -> Self {
        Self {
            kind: SYNCED_SETTINGS_KIND_SETTINGS,
            file_path: paths::settings_file().clone(),
            build_classifier: DocumentClassifier::for_user_settings,
        }
    }
}

// TODO kb cloud: status surface — last sync time, cloud version, hosts per
// group with relabel/evict, history with restore, purge-on-disable; conflict
// attribution ("changed on work-laptop") once the server exposes per-path
// provenance.
#[derive(Debug, Clone)]
pub enum SettingsSyncEvent {
    ConflictsResolved(Vec<Conflict>),
    Paused,
    UpdateRequired,
}

pub struct SettingsSyncEngine {
    fs: Arc<dyn Fs>,
    server: Arc<dyn SettingsSyncServer>,
    document: SyncedDocument,
    state_file_path: PathBuf,
    classifier: Option<Arc<DocumentClassifier>>,
    state: Option<SyncState>,
    last_self_write: Option<String>,
    paused: bool,
    update_required_notified: bool,
    flag_gate_notified: bool,
    cloud_unsupported_notified: bool,
    sync_tx: mpsc::UnboundedSender<()>,
    _tasks: Vec<Task<()>>,
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
        let watch_task = cx.spawn(async move |this, cx| {
            let _document_file_watcher = document_file_watcher;
            while let Some(content) = document_file_rx.next().await {
                let update_result = this.update(cx, |this, cx| {
                    if this.last_self_write.as_deref() == Some(content.as_str()) {
                        this.last_self_write = None;
                    } else {
                        this.schedule_sync(cx);
                    }
                });
                if update_result.is_err() {
                    break;
                }
            }
        });

        let sync_task = cx.spawn(async move |this, cx| {
            while sync_rx.next().await.is_some() {
                cx.background_executor().timer(SYNC_DEBOUNCE).await;
                while sync_rx.try_recv().is_ok() {}
                if this.upgrade().is_none() {
                    break;
                }
                // TODO kb cloud: repeated hard failures (network, future size cap)
                // only log; pause + notify after N in a row, like the CAS livelock path.
                if let Err(error) = Self::run_sync(&this, cx).await {
                    log::error!("settings sync: sync cycle failed: {error:#}");
                }
            }
        });

        Self {
            fs,
            server,
            document,
            state_file_path,
            classifier: None,
            state: None,
            last_self_write: None,
            paused: false,
            update_required_notified: false,
            flag_gate_notified: false,
            cloud_unsupported_notified: false,
            sync_tx,
            _tasks: vec![watch_task, sync_task],
        }
    }

    pub fn schedule_sync(&mut self, _cx: &mut Context<Self>) {
        if self.paused {
            return;
        }
        self.sync_tx.unbounded_send(()).ok();
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self, cx: &mut Context<Self>) {
        self.paused = false;
        self.schedule_sync(cx);
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn state(&self) -> Option<&SyncState> {
        self.state.as_ref()
    }

    pub fn revert_conflicts(
        &mut self,
        conflicts: Vec<Conflict>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let fs = self.fs.clone();
        let document_file_path = self.document.file_path.clone();
        cx.spawn(async move |this, cx| {
            let text = fs
                .load(&document_file_path)
                .await
                .context("loading the settings file to revert conflicts")?;
            let ops = conflicts
                .into_iter()
                .map(|conflict| match conflict.local {
                    Some(value) => SyncOp::Set {
                        path: conflict.path,
                        value,
                    },
                    None => SyncOp::Delete {
                        path: conflict.path,
                    },
                })
                .collect::<Vec<_>>();
            let new_text = apply_ops_to_text(&text, &ops);
            if new_text != text {
                fs.atomic_write(document_file_path, new_text)
                    .await
                    .context("reverting conflicting settings")?;
            }
            this.update(cx, |this, cx| this.schedule_sync(cx))?;
            Ok(())
        })
    }

    pub fn handle_remote_changed(
        &mut self,
        group_id: &str,
        kind: &str,
        version: u64,
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
            if state
                .base
                .as_ref()
                .is_some_and(|base| base.server_version >= version)
            {
                return;
            }
        }
        self.schedule_sync(cx);
    }

    async fn run_sync(this: &WeakEntity<Self>, cx: &mut AsyncApp) -> Result<()> {
        let (fs, server, classifier, state_file_path, sync_settings, paused, flag_enabled) =
            this.read_with(cx, |this, cx| {
                (
                    this.fs.clone(),
                    this.server.clone(),
                    this.classifier.clone(),
                    this.state_file_path.clone(),
                    SettingsSyncSettings::get_global(cx).clone(),
                    this.paused,
                    cx.has_flag::<SettingsSyncFeatureFlag>(),
                )
            })?;
        let (document_kind, document_path, build_classifier) = this.read_with(cx, |this, _| {
            (
                this.document.kind,
                this.document.file_path.clone(),
                this.document.build_classifier,
            )
        })?;
        // TODO kb cloud: remove the feature flag gate at GA; until then
        // enabled-but-flag-off is a warn-once no-op.
        if !flag_enabled {
            if sync_settings.enabled {
                this.update(cx, |this, _| {
                    if !this.flag_gate_notified {
                        this.flag_gate_notified = true;
                        log::warn!(
                            "settings sync: enabled in settings, but the feature flag is off; \
                             not syncing"
                        );
                    }
                })?;
            }
            return Ok(());
        }
        if !sync_settings.enabled || paused || !server.is_ready() {
            return Ok(());
        }

        let classifier = match classifier {
            Some(classifier) => classifier,
            None => {
                let classifier = cx
                    .background_spawn(async move { Arc::new(build_classifier()) })
                    .await;
                this.update(cx, |this, _| {
                    this.classifier = Some(classifier.clone());
                })?;
                classifier
            }
        };

        let disk_state = load_state(fs.as_ref(), &state_file_path).await;
        let memory_state = this.read_with(cx, |this, _| this.state.clone())?;
        let mut state = reconcile_states(memory_state, disk_state);
        let state_before_sync = state.clone();

        let local_text = match fs.load(&document_path).await {
            Ok(text) => text,
            Err(error) => {
                let has_base = state.base.is_some();
                if has_base {
                    log::warn!(
                        "settings sync: skipping cycle, settings file is unreadable: {error:#}"
                    );
                    return Ok(());
                }
                String::new()
            }
        };
        let local_doc = if local_text.trim().is_empty() {
            Value::Object(serde_json::Map::new())
        } else {
            match parse_json_with_comments::<Value>(&local_text) {
                Ok(value) => value,
                Err(error) => {
                    log::warn!(
                        "settings sync: skipping cycle, settings file failed to parse: {error:#}"
                    );
                    return Ok(());
                }
            }
        };
        if !local_doc.is_object() {
            log::warn!("settings sync: skipping cycle, settings file root is not an object");
            return Ok(());
        }

        let mut current_remote = match server.fetch(document_kind).await {
            Ok(remote) => remote,
            Err(error) if error.downcast_ref::<SyncNotImplementedError>().is_some() => {
                notify_cloud_unsupported(this, cx)?;
                return Ok(());
            }
            Err(error) => return Err(error).context("fetching synced settings"),
        };

        // TODO kb cloud: observer semantics for renamed keys — pulling a migrated
        // doc deletes the old-shape keys locally, regressing the setting's effect
        // until this Zed updates; document, or skip deletions for observers.
        let schema_epoch = settings_schema_epoch();
        let observer_mode = current_remote
            .as_ref()
            .is_some_and(|remote| remote.schema_epoch > schema_epoch);
        if observer_mode {
            this.update(cx, |this, cx| {
                if !this.update_required_notified {
                    this.update_required_notified = true;
                    log::warn!(
                        "settings sync: the cloud document was written by a newer Zed; \
                         pulling only until this Zed is updated"
                    );
                    cx.emit(SettingsSyncEvent::UpdateRequired);
                }
            })?;
        }

        let mut exclusions = ExclusionSet::built_in();
        exclusions.extend_from_pointers(sync_settings.exclude.iter().map(String::as_str));
        let local_full = flatten_doc(&classifier, &local_doc);
        exclusions.extend_from_flattened(&local_full);

        let mut local_view = local_full.clone();
        let mut raw_conflicts = BTreeMap::new();
        let mut paused_now = false;
        let mut new_base = None;
        let mut attempts = 0;

        loop {
            let remote_full = current_remote
                .as_ref()
                .map(|remote| flatten_doc(&classifier, &remote.doc))
                .unwrap_or_default();
            exclusions.extend_from_flattened(&remote_full);

            // TODO kb: with no base yet (first enable / joining a group) this
            // degrades to a two-way union merge; the first-enable choice
            // (merge / replace local / replace remote + preview diff) is not
            // implemented yet.
            let mut base_paths = if current_remote.is_some() {
                state
                    .base
                    .as_ref()
                    .map(|base| flatten_doc(&classifier, &base.doc))
                    .unwrap_or_default()
            } else {
                PathMap::default()
            };
            let mut remote_paths = remote_full.clone();
            exclusions.strip(&mut base_paths);
            exclusions.strip(&mut remote_paths);
            exclusions.strip(&mut local_view);

            let merge = merge_three_way(&base_paths, &local_view, &remote_paths);
            for conflict in merge.conflicts {
                raw_conflicts.insert(conflict.path.clone(), conflict);
            }
            local_view = merge.merged;
            drop_prefix_overlaps(&mut local_view);

            if let Some(remote) = &current_remote
                && remote_full == local_view
            {
                new_base = Some(SyncBase {
                    server_version: remote.version,
                    schema_epoch: remote.schema_epoch,
                    doc: remote.doc.clone(),
                });
                break;
            }

            if observer_mode {
                if let Some(remote) = &current_remote {
                    new_base = Some(SyncBase {
                        server_version: remote.version,
                        schema_epoch: remote.schema_epoch,
                        doc: remote.doc.clone(),
                    });
                }
                break;
            }

            if !sync_still_enabled(this, cx)? {
                log::info!("settings sync: disabled mid-cycle, skipping the push");
                return Ok(());
            }
            let doc = unflatten(&local_view);
            let push_result = match server
                .push(UpdateSyncedSettingsBody {
                    kind: document_kind.to_string(),
                    base_version: current_remote.as_ref().map(|remote| remote.version),
                    schema_epoch,
                    doc: doc.clone(),
                })
                .await
            {
                Ok(push_result) => push_result,
                Err(error) if error.downcast_ref::<SyncNotImplementedError>().is_some() => {
                    notify_cloud_unsupported(this, cx)?;
                    return Ok(());
                }
                Err(error) => return Err(error).context("pushing synced settings"),
            };

            match push_result {
                PushResult::Written { version, group_id } => {
                    state.group_id = Some(group_id);
                    new_base = Some(SyncBase {
                        server_version: version,
                        schema_epoch,
                        doc,
                    });
                    break;
                }
                PushResult::Conflict { current } => {
                    attempts += 1;
                    if attempts >= MAX_PUSH_ATTEMPTS {
                        log::warn!(
                            "settings sync: pausing after {attempts} conflicting pushes in a row"
                        );
                        paused_now = true;
                        break;
                    }
                    let jitter = Duration::from_millis(
                        rand::rng().random_range(0..=PUSH_RETRY_MAX_JITTER_MILLIS),
                    );
                    cx.background_executor()
                        .timer(PUSH_RETRY_BACKOFF * attempts as u32 + jitter)
                        .await;
                    current_remote = current;
                }
            }
        }

        if let Some(remote) = &current_remote {
            state.group_id = Some(remote.group_id.clone());
        }

        if !sync_still_enabled(this, cx)? {
            log::info!("settings sync: disabled mid-cycle, skipping the apply");
            return Ok(());
        }

        let final_merged = local_view;
        let mut stripped_local = local_full;
        exclusions.strip(&mut stripped_local);

        let mut conflicts = Vec::new();
        for (path, conflict) in raw_conflicts {
            let original_local = stripped_local.get(&path);
            if original_local == conflict.base.as_ref() {
                continue;
            }
            if original_local == final_merged.get(&path) {
                continue;
            }
            conflicts.push(Conflict {
                local: original_local.cloned(),
                ..conflict
            });
        }

        let ops = diff_paths(&stripped_local, &final_merged);
        let mut applied = ops.is_empty();
        if !ops.is_empty() {
            let text_on_disk = fs.load(&document_path).await.unwrap_or_default();
            if text_on_disk == local_text {
                let new_text = apply_ops_to_text(&local_text, &ops);
                if new_text != local_text {
                    write_pre_apply_backup(fs.as_ref(), &state_file_path, &local_text).await;
                    this.update(cx, |this, _| {
                        this.last_self_write = Some(new_text.clone());
                    })?;
                    fs.atomic_write(document_path.clone(), new_text)
                        .await
                        .context("writing merged settings file")?;
                }
                applied = true;
            } else {
                log::info!("settings sync: settings file changed mid-cycle, rescheduling");
                this.update(cx, |this, cx| this.schedule_sync(cx))?;
            }
        }

        if applied && let Some(new_base) = new_base {
            state.base = Some(new_base);
        }
        state.format_version = SYNC_STATE_FORMAT_VERSION;
        if state != state_before_sync {
            persist_state(fs.as_ref(), &state_file_path, &state)
                .await
                .context("persisting sync state")?;
        }

        this.update(cx, |this, cx| {
            this.state = Some(state);
            if paused_now {
                this.paused = true;
                cx.emit(SettingsSyncEvent::Paused);
            }
            if !conflicts.is_empty() {
                for conflict in &conflicts {
                    log::warn!(
                        "settings sync: conflict at {}, kept the remote side",
                        conflict.path
                    );
                }
                cx.emit(SettingsSyncEvent::ConflictsResolved(conflicts));
            }
        })?;

        Ok(())
    }
}

pub(crate) async fn load_state(fs: &dyn Fs, state_file_path: &Path) -> SyncState {
    match fs.load(state_file_path).await {
        Ok(content) => match serde_json::from_str::<SyncState>(&content) {
            Ok(state) if state.format_version <= SYNC_STATE_FORMAT_VERSION => state,
            Ok(state) => {
                log::warn!(
                    "settings sync: state file has a newer format ({}), starting fresh",
                    state.format_version
                );
                SyncState::default()
            }
            Err(error) => {
                log::warn!("settings sync: failed to parse state file, starting fresh: {error:#}");
                SyncState::default()
            }
        },
        Err(_) => SyncState::default(),
    }
}

fn base_server_version(state: &SyncState) -> u64 {
    state.base.as_ref().map_or(0, |base| base.server_version)
}

fn reconcile_states(memory: Option<SyncState>, disk: SyncState) -> SyncState {
    let Some(memory) = memory else {
        return disk;
    };
    let (mut newest, other) = if base_server_version(&memory) >= base_server_version(&disk) {
        (memory, disk)
    } else {
        (disk, memory)
    };
    if newest.group_id.is_none() {
        newest.group_id = other.group_id;
    }
    newest
}

fn notify_cloud_unsupported(
    this: &WeakEntity<SettingsSyncEngine>,
    cx: &mut AsyncApp,
) -> Result<()> {
    this.update(cx, |this, _| {
        if !this.cloud_unsupported_notified {
            this.cloud_unsupported_notified = true;
            log::warn!(
                "settings sync: Zed Cloud does not implement settings sync yet; \
                 skipping sync until it does"
            );
        }
    })
}

fn sync_still_enabled(this: &WeakEntity<SettingsSyncEngine>, cx: &AsyncApp) -> Result<bool> {
    this.read_with(cx, |this, cx| {
        SettingsSyncSettings::get_global(cx).enabled && !this.paused
    })
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

async fn persist_state(fs: &dyn Fs, state_file_path: &Path, state: &SyncState) -> Result<()> {
    if let Some(parent) = state_file_path.parent() {
        fs.create_dir(parent).await?;
    }
    fs.atomic_write(
        state_file_path.to_path_buf(),
        serde_json::to_string_pretty(state)?,
    )
    .await
}
