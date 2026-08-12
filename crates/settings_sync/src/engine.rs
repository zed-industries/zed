use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use cloud_api_client::{SYNCED_SETTINGS_KIND_SETTINGS, UpdateSyncedSettingsBody};
use fs::Fs;
use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{AppContext as _, AsyncApp, Context, EventEmitter, Task, WeakEntity};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{Settings as _, watch_config_file};
use settings_json::parse_json_with_comments;

use crate::classifier::DocumentClassifier;
use crate::merge::{
    Conflict, ExclusionSet, PathMap, apply_ops_to_text, diff_paths, drop_prefix_overlaps,
    flatten_doc, merge_three_way, unflatten,
};
use crate::{SettingsSyncSettings, settings_schema_epoch};

pub const MAX_PUSH_ATTEMPTS: usize = 3;
const PUSH_RETRY_BACKOFF: Duration = Duration::from_millis(500);

const SYNC_STATE_FORMAT_VERSION: u64 = 1;

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

// TODO kb: surface these in the UI — a dismissible conflict notification with
// one-click revert, a pause notification, and an update prompt.
#[derive(Debug, Clone)]
pub enum SettingsSyncEvent {
    ConflictsResolved(Vec<Conflict>),
    Paused,
    UpdateRequired,
}

pub struct SettingsSyncEngine {
    fs: Arc<dyn Fs>,
    server: Arc<dyn crate::SettingsSyncServer>,
    settings_file_path: PathBuf,
    state_file_path: PathBuf,
    classifier: Option<Arc<DocumentClassifier>>,
    state: Option<SyncState>,
    last_self_write: Option<String>,
    paused: bool,
    update_required_notified: bool,
    sync_tx: mpsc::UnboundedSender<()>,
    _tasks: Vec<Task<()>>,
}

impl EventEmitter<SettingsSyncEvent> for SettingsSyncEngine {}

impl SettingsSyncEngine {
    pub fn new(
        server: Arc<dyn crate::SettingsSyncServer>,
        fs: Arc<dyn Fs>,
        settings_file_path: PathBuf,
        state_file_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Self {
        let (sync_tx, mut sync_rx) = mpsc::unbounded();

        let (mut settings_file_rx, settings_file_watcher) = watch_config_file(
            cx.background_executor(),
            fs.clone(),
            settings_file_path.clone(),
        );
        let watch_task = cx.spawn(async move |this, cx| {
            let _settings_file_watcher = settings_file_watcher;
            while let Some(content) = settings_file_rx.next().await {
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
                while sync_rx.try_recv().is_ok() {}
                if this.upgrade().is_none() {
                    break;
                }
                if let Err(error) = Self::run_sync(&this, cx).await {
                    log::error!("settings sync: sync cycle failed: {error:#}");
                }
            }
        });

        Self {
            fs,
            server,
            settings_file_path,
            state_file_path,
            classifier: None,
            state: None,
            last_self_write: None,
            paused: false,
            update_required_notified: false,
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
        let settings_file_path = self.settings_file_path.clone();
        cx.spawn(async move |this, cx| {
            let text = fs.load(&settings_file_path).await.unwrap_or_default();
            let ops = conflicts
                .into_iter()
                .map(|conflict| match conflict.local {
                    Some(value) => crate::SyncOp::Set {
                        path: conflict.path,
                        value,
                    },
                    None => crate::SyncOp::Delete {
                        path: conflict.path,
                    },
                })
                .collect::<Vec<_>>();
            let new_text = apply_ops_to_text(&text, &ops);
            if new_text != text {
                fs.atomic_write(settings_file_path, new_text)
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
        if kind != SYNCED_SETTINGS_KIND_SETTINGS {
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
        let (fs, server, classifier, settings_file_path, state_file_path, enabled, paused) =
            this.read_with(cx, |this, cx| {
                (
                    this.fs.clone(),
                    this.server.clone(),
                    this.classifier.clone(),
                    this.settings_file_path.clone(),
                    this.state_file_path.clone(),
                    SettingsSyncSettings::get_global(cx).enabled,
                    this.paused,
                )
            })?;
        if !enabled || paused || !server.is_ready() {
            return Ok(());
        }

        let classifier = match classifier {
            Some(classifier) => classifier,
            None => {
                let classifier = cx
                    .background_spawn(
                        async move { Arc::new(DocumentClassifier::for_user_settings()) },
                    )
                    .await;
                this.update(cx, |this, _| {
                    this.classifier = Some(classifier.clone());
                })?;
                classifier
            }
        };

        let state = this.read_with(cx, |this, _| this.state.clone())?;
        let mut state = match state {
            Some(state) => state,
            None => load_state(fs.as_ref(), &state_file_path).await,
        };
        let state_before_sync = state.clone();

        let local_text = match fs.load(&settings_file_path).await {
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

        let mut current_remote = server
            .fetch(SYNCED_SETTINGS_KIND_SETTINGS)
            .await
            .context("fetching synced settings")?;

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
        let local_full = flatten_doc(&classifier, &local_doc);
        exclusions.extend_from_flattened(&local_full);

        let mut local_view = local_full.clone();
        let mut all_conflicts = Vec::new();
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
            all_conflicts.extend(merge.conflicts);
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

            let doc = unflatten(&local_view);
            let push_result = server
                .push(UpdateSyncedSettingsBody {
                    kind: SYNCED_SETTINGS_KIND_SETTINGS.to_string(),
                    base_version: current_remote.as_ref().map(|remote| remote.version),
                    schema_epoch,
                    doc: doc.clone(),
                })
                .await
                .context("pushing synced settings")?;

            match push_result {
                crate::PushResult::Written { version } => {
                    new_base = Some(SyncBase {
                        server_version: version,
                        schema_epoch,
                        doc,
                    });
                    break;
                }
                crate::PushResult::Conflict { current } => {
                    attempts += 1;
                    if attempts >= MAX_PUSH_ATTEMPTS {
                        log::warn!(
                            "settings sync: pausing after {attempts} conflicting pushes in a row"
                        );
                        paused_now = true;
                        break;
                    }
                    cx.background_executor()
                        .timer(PUSH_RETRY_BACKOFF * attempts as u32)
                        .await;
                    current_remote = current;
                }
            }
        }

        if let Some(remote) = &current_remote {
            state.group_id = Some(remote.group_id.clone());
        }

        let final_merged = local_view;
        let mut local_current = local_full;
        exclusions.strip(&mut local_current);
        let ops = diff_paths(&local_current, &final_merged);
        if !ops.is_empty() {
            let text_on_disk = fs.load(&settings_file_path).await.unwrap_or_default();
            if text_on_disk == local_text {
                let new_text = apply_ops_to_text(&local_text, &ops);
                if new_text != local_text {
                    write_pre_apply_backup(fs.as_ref(), &state_file_path, &local_text).await;
                    this.update(cx, |this, _| {
                        this.last_self_write = Some(new_text.clone());
                    })?;
                    fs.atomic_write(settings_file_path.clone(), new_text)
                        .await
                        .context("writing merged settings file")?;
                }
            } else {
                log::info!("settings sync: settings file changed mid-cycle, rescheduling");
                this.update(cx, |this, cx| this.schedule_sync(cx))?;
            }
        }

        if let Some(new_base) = new_base {
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
            if !all_conflicts.is_empty() {
                for conflict in &all_conflicts {
                    log::warn!(
                        "settings sync: conflict at {}, kept the remote side",
                        conflict.path
                    );
                }
                cx.emit(SettingsSyncEvent::ConflictsResolved(all_conflicts));
            }
        })?;

        Ok(())
    }
}

async fn load_state(fs: &dyn Fs, state_file_path: &Path) -> SyncState {
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

async fn write_pre_apply_backup(fs: &dyn Fs, state_file_path: &Path, settings_text: &str) {
    let Some(parent) = state_file_path.parent() else {
        return;
    };
    let backup_path = parent.join("settings_pre_apply_backup.json");
    if let Err(error) = fs.create_dir(parent).await {
        log::warn!("settings sync: failed to create the backup directory: {error:#}");
        return;
    }
    if let Err(error) = fs
        .atomic_write(backup_path, settings_text.to_string())
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
