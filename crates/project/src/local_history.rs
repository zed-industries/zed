use crate::debounced_delay::DebouncedDelay;
use crate::worktree_store::WorktreeStore;
use anyhow::{Context as _, Result};
use clock::Global;
use collections::HashMap;
use gpui::{App, AppContext, Context, Entity, EventEmitter, Task};
use language::Buffer;
use parking_lot::Mutex;
use paths::data_dir;
use rand::random;
use serde::{Deserialize, Serialize};
use settings::{LocalHistoryPrunePolicy, RegisterSetting, Settings, WorktreeId};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use text::BufferId;
use time::OffsetDateTime;
use util::ResultExt;
use util::paths::{PathMatcher, PathStyle};
use util::rel_path::RelPath;

use crate::ProjectPath;

const WORKTREES_DIR: &str = "worktrees";
const INDEX_FILE_NAME: &str = "index.jsonl";
const SNAPSHOTS_DIR_NAME: &str = "snapshots";
const DEFAULT_MIN_CAP_BYTES: u64 = 300 * 1024 * 1024;
const DEFAULT_FREE_SPACE_PERCENT: f32 = 0.12;
const DEFAULT_MIN_AGE_DAYS: u64 = 100;
const DEFAULT_EDIT_IDLE_MS: u64 = 1000;
const DEFAULT_EXCLUDE_GLOBS: &[&str] = &[
    "**/.git/**",
    "**/.hg/**",
    "**/.svn/**",
    "**/.jj/**",
    "**/node_modules/**",
    "**/target/**",
    "**/dist/**",
    "**/build/**",
    "**/out/**",
    "**/.gradle/**",
    "**/.idea/**",
    "**/.zed/**",
    "**/*.min.*",
    "**/*.map",
];

#[derive(Clone, Debug)]
pub struct LocalHistoryEntry {
    pub id: String,
    pub timestamp: OffsetDateTime,
    pub relative_path: Arc<str>,
    pub endpoint_root: PathBuf,
    pub snapshot_relative_path: PathBuf,
    pub compressed_bytes: u64,
    pub uncompressed_bytes: u64,
}

impl LocalHistoryEntry {
    pub fn snapshot_path(&self) -> PathBuf {
        self.endpoint_root.join(&self.snapshot_relative_path)
    }
}

#[derive(Clone, Debug, PartialEq, RegisterSetting)]
pub struct LocalHistorySettings {
    pub enabled: bool,
    pub capture_on_save: bool,
    pub capture_on_edit_idle_ms: Option<u64>,
    pub capture_on_focus_change: bool,
    pub capture_on_window_change: bool,
    pub capture_on_task: bool,
    pub capture_on_external_change: bool,
    pub storage_paths: Vec<String>,
    pub active_storage_path: Option<String>,
    pub min_age_days: u64,
    pub cap_free_space_percent: f32,
    pub cap_min_bytes: u64,
    pub prune_policy: LocalHistoryPrunePolicy,
    pub exclude_globs: Vec<String>,
}

impl Settings for LocalHistorySettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let local_history = content.local_history.clone().unwrap_or_default();
        let exclude_globs = local_history.exclude_globs.unwrap_or_else(|| {
            DEFAULT_EXCLUDE_GLOBS
                .iter()
                .map(|glob| (*glob).to_string())
                .collect()
        });
        Self {
            enabled: local_history.enabled.unwrap_or(true),
            capture_on_save: local_history.capture_on_save.unwrap_or(false),
            capture_on_edit_idle_ms: local_history
                .capture_on_edit_idle_ms
                .map(|delay| delay.0)
                .or(Some(DEFAULT_EDIT_IDLE_MS))
                .filter(|delay| *delay > 0),
            capture_on_focus_change: local_history.capture_on_focus_change.unwrap_or(true),
            capture_on_window_change: local_history.capture_on_window_change.unwrap_or(true),
            capture_on_task: local_history.capture_on_task.unwrap_or(true),
            capture_on_external_change: local_history.capture_on_external_change.unwrap_or(true),
            storage_paths: local_history.storage_paths.unwrap_or_default(),
            active_storage_path: local_history.active_storage_path,
            min_age_days: local_history.min_age_days.unwrap_or(DEFAULT_MIN_AGE_DAYS),
            cap_free_space_percent: local_history
                .cap_free_space_percent
                .unwrap_or(DEFAULT_FREE_SPACE_PERCENT),
            cap_min_bytes: local_history.cap_min_bytes.unwrap_or(DEFAULT_MIN_CAP_BYTES),
            prune_policy: local_history
                .prune_policy
                .unwrap_or(LocalHistoryPrunePolicy::Both),
            exclude_globs,
        }
    }
}

impl LocalHistorySettings {
    pub fn capture_on_edit_idle_delay(&self) -> Option<Duration> {
        self.capture_on_edit_idle_ms
            .map(Duration::from_millis)
            .filter(|delay| !delay.is_zero())
    }

    pub fn should_capture(&self, trigger: LocalHistoryCaptureTrigger) -> bool {
        match trigger {
            LocalHistoryCaptureTrigger::Save => self.capture_on_save,
            LocalHistoryCaptureTrigger::EditIdle => self.capture_on_edit_idle_ms.is_some(),
            LocalHistoryCaptureTrigger::FocusChange => self.capture_on_focus_change,
            LocalHistoryCaptureTrigger::WindowChange => self.capture_on_window_change,
            LocalHistoryCaptureTrigger::Task => self.capture_on_task,
            LocalHistoryCaptureTrigger::ExternalChange => self.capture_on_external_change,
        }
    }

    pub fn resolved_storage_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for path in &self.storage_paths {
            let expanded = shellexpand::tilde(path);
            paths.push(PathBuf::from(expanded.as_ref()));
        }
        if let Some(active) = &self.active_storage_path {
            let expanded = shellexpand::tilde(active);
            let active_path = PathBuf::from(expanded.as_ref());
            if !paths.contains(&active_path) {
                paths.push(active_path);
            }
        }
        if paths.is_empty() {
            paths.push(data_dir().join("local_history"));
        }
        paths
    }

    pub fn resolved_active_path(&self) -> PathBuf {
        if let Some(active) = &self.active_storage_path {
            let expanded = shellexpand::tilde(active);
            return PathBuf::from(expanded.as_ref());
        }
        self.resolved_storage_paths()
            .into_iter()
            .next()
            .unwrap_or_else(|| data_dir().join("local_history"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalHistoryCaptureTrigger {
    Save,
    EditIdle,
    FocusChange,
    WindowChange,
    Task,
    ExternalChange,
}

impl LocalHistoryCaptureTrigger {
    pub(crate) fn requires_dirty(self) -> bool {
        matches!(
            self,
            LocalHistoryCaptureTrigger::EditIdle
                | LocalHistoryCaptureTrigger::FocusChange
                | LocalHistoryCaptureTrigger::WindowChange
                | LocalHistoryCaptureTrigger::Task
        )
    }
}

#[derive(Debug)]
pub enum LocalHistoryEvent {
    EntriesUpdated,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct WorktreePathKey {
    worktree_id: WorktreeId,
    path: Arc<RelPath>,
}

pub struct LocalHistoryStore {
    worktree_store: Entity<WorktreeStore>,
    history_lock: Arc<Mutex<()>>,
    edit_debouncers: HashMap<BufferId, DebouncedDelay<Self>>,
    fs_change_debouncers: HashMap<WorktreePathKey, DebouncedDelay<Self>>,
    last_snapshot_versions: HashMap<BufferId, Global>,
    paths_with_history: HashMap<WorktreePathKey, ()>,
}

impl EventEmitter<LocalHistoryEvent> for LocalHistoryStore {}

impl LocalHistoryStore {
    pub fn new_local(worktree_store: Entity<WorktreeStore>) -> Self {
        Self {
            worktree_store,
            history_lock: Arc::new(Mutex::new(())),
            edit_debouncers: HashMap::default(),
            fs_change_debouncers: HashMap::default(),
            last_snapshot_versions: HashMap::default(),
            paths_with_history: HashMap::default(),
        }
    }

    pub fn record_snapshot(
        &mut self,
        buffer: Entity<Buffer>,
        trigger: LocalHistoryCaptureTrigger,
        cx: &mut Context<Self>,
    ) {
        let settings = LocalHistorySettings::get_global(cx).clone();
        if !settings.enabled || !settings.should_capture(trigger) {
            return;
        }

        let Some(file) = buffer.read(cx).file() else {
            return;
        };
        let project_path = ProjectPath::from_file(file.as_ref(), cx);
        let buffer_id = buffer.read(cx).remote_id();

        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return;
        };

        if !worktree.read(cx).is_local() {
            return;
        }

        let path_style = worktree.read(cx).path_style();
        if is_excluded(&settings, project_path.path.as_ref(), path_style) {
            return;
        }

        if trigger.requires_dirty() && !buffer.read(cx).is_dirty() {
            return;
        }

        let snapshot = buffer.read(cx).snapshot();
        let version = snapshot.text.version.clone();
        if self
            .last_snapshot_versions
            .get(&buffer_id)
            .is_some_and(|last_version| *last_version == version)
        {
            return;
        }
        self.last_snapshot_versions.insert(buffer_id, version);
        let path_key = WorktreePathKey {
            worktree_id: project_path.worktree_id,
            path: project_path.path.clone(),
        };
        self.paths_with_history.insert(path_key.clone(), ());

        let text = snapshot.text();
        let relative_path = project_path.path.display(path_style).to_string();
        let worktree_path = worktree.read(cx).abs_path().to_path_buf();
        let active_root = settings.resolved_active_path();
        let history_lock = self.history_lock.clone();
        let this = cx.weak_entity();

        let task = cx.background_spawn(async move {
            let _guard = history_lock.lock();
            let write_result = write_snapshot(
                &active_root,
                &worktree_path,
                &relative_path,
                &text,
                OffsetDateTime::now_utc(),
            );
            let prune_result = prune_worktree_if_needed(&active_root, &worktree_path, &settings);
            (write_result, prune_result)
        });

        cx.spawn(async move |_, cx| {
            let (write_result, prune_result) = task.await;
            if let Err(err) = write_result {
                log::warn!("local history snapshot failed: {err:#}");
            }
            if let Err(err) = prune_result {
                log::warn!("local history prune failed: {err:#}");
            }

            if let Some(this) = this.upgrade() {
                this.update(cx, |_, cx| {
                    cx.emit(LocalHistoryEvent::EntriesUpdated);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    pub fn schedule_edit_snapshot(
        &mut self,
        buffer: Entity<Buffer>,
        delay: Duration,
        cx: &mut Context<Self>,
    ) {
        if !self.path_has_history(&buffer, cx) {
            self.record_snapshot(buffer, LocalHistoryCaptureTrigger::EditIdle, cx);
            return;
        }

        let buffer_id = buffer.read(cx).remote_id();
        let debouncer = self
            .edit_debouncers
            .entry(buffer_id)
            .or_insert_with(DebouncedDelay::new);
        debouncer.fire_new(delay, cx, move |store, cx| {
            store.record_snapshot(buffer, LocalHistoryCaptureTrigger::EditIdle, cx);
            Task::ready(())
        });
    }

    pub fn schedule_filesystem_snapshot(
        &mut self,
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        delay: Duration,
        cx: &mut Context<Self>,
    ) {
        let key = WorktreePathKey {
            worktree_id,
            path: path.clone(),
        };
        let debouncer = self
            .fs_change_debouncers
            .entry(key)
            .or_insert_with(DebouncedDelay::new);
        debouncer.fire_new(delay, cx, move |store, cx| {
            store.record_filesystem_snapshot(worktree_id, path, cx);
            Task::ready(())
        });
    }

    pub fn record_filesystem_snapshot(
        &mut self,
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        cx: &mut Context<Self>,
    ) {
        let settings = LocalHistorySettings::get_global(cx).clone();
        if !settings.enabled || !settings.should_capture(LocalHistoryCaptureTrigger::ExternalChange)
        {
            return;
        }

        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(worktree_id, cx)
        else {
            return;
        };

        if !worktree.read(cx).is_local() {
            return;
        }

        let path_style = worktree.read(cx).path_style();
        if is_excluded(&settings, path.as_ref(), path_style) {
            return;
        }

        let relative_path = path.display(path_style).to_string();
        let worktree_path = worktree.read(cx).abs_path().to_path_buf();
        let active_root = settings.resolved_active_path();
        let history_lock = self.history_lock.clone();
        let this = cx.weak_entity();
        let path_key = WorktreePathKey {
            worktree_id,
            path: path.clone(),
        };
        self.paths_with_history.insert(path_key, ());

        let load_task = worktree.update(cx, |worktree, cx| worktree.load_file(path.as_ref(), cx));
        let task = cx.background_spawn(async move {
            let loaded = match load_task
                .await
                .with_context(|| format!("loading {:?}", worktree_path.join(relative_path.clone())))
            {
                Ok(loaded) => loaded,
                Err(err) => return (Err(err), Ok(0)),
            };
            let _guard = history_lock.lock();
            let write_result = write_snapshot(
                &active_root,
                &worktree_path,
                &relative_path,
                &loaded.text,
                OffsetDateTime::now_utc(),
            );
            let prune_result = prune_worktree_if_needed(&active_root, &worktree_path, &settings);
            (write_result, prune_result)
        });

        cx.spawn(async move |_, cx| {
            let (write_result, prune_result) = task.await;
            if let Err(err) = write_result {
                log::warn!("local history snapshot failed: {err:#}");
            }
            if let Err(err) = prune_result {
                log::warn!("local history prune failed: {err:#}");
            }

            if let Some(this) = this.upgrade() {
                this.update(cx, |_, cx| {
                    cx.emit(LocalHistoryEvent::EntriesUpdated);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    fn path_has_history(&mut self, buffer: &Entity<Buffer>, cx: &App) -> bool {
        let Some(file) = buffer.read(cx).file() else {
            return false;
        };
        let project_path = ProjectPath::from_file(file.as_ref(), cx);
        let path_key = WorktreePathKey {
            worktree_id: project_path.worktree_id,
            path: project_path.path.clone(),
        };
        if self.paths_with_history.contains_key(&path_key) {
            return true;
        }

        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return false;
        };
        if !worktree.read(cx).is_local() {
            return false;
        }

        let path_style = worktree.read(cx).path_style();
        let worktree_path = worktree.read(cx).abs_path().to_path_buf();
        let relative_path = project_path.path.display(path_style).to_string();
        let settings = LocalHistorySettings::get_global(cx).clone();

        let has_history = settings
            .resolved_storage_paths()
            .into_iter()
            .any(|root| endpoint_has_entries_for_path(&root, &worktree_path, &relative_path));
        if has_history {
            self.paths_with_history.insert(path_key, ());
        }
        has_history
    }

    pub fn migrate_worktree_root(
        &mut self,
        old_path: PathBuf,
        new_path: PathBuf,
        cx: &mut Context<Self>,
    ) {
        let settings = LocalHistorySettings::get_global(cx).clone();
        if !settings.enabled || old_path == new_path {
            return;
        }

        let history_lock = self.history_lock.clone();
        let endpoints = settings.resolved_storage_paths();
        let task = cx.background_spawn(async move {
            let _guard = history_lock.lock();
            for endpoint in endpoints {
                migrate_worktree_history_dir(&endpoint, &old_path, &new_path)?;
            }
            Ok::<_, anyhow::Error>(())
        });

        cx.spawn(async move |_, _| {
            if let Err(err) = task.await {
                log::warn!("local history worktree migration failed: {err:#}");
            }
        })
        .detach();
    }

    pub fn rewrite_relative_paths_for_renames(
        &mut self,
        worktree_path: PathBuf,
        renames: Vec<(String, String)>,
        cx: &mut Context<Self>,
    ) {
        let settings = LocalHistorySettings::get_global(cx).clone();
        if !settings.enabled || renames.is_empty() {
            return;
        }

        let history_lock = self.history_lock.clone();
        let endpoints = settings.resolved_storage_paths();
        let task = cx.background_spawn(async move {
            let _guard = history_lock.lock();
            let renames = renames.into_iter().collect::<HashMap<_, _>>();
            for endpoint in endpoints {
                rewrite_index_for_renames(&endpoint, &worktree_path, &renames)?;
            }
            Ok::<_, anyhow::Error>(())
        });

        cx.spawn(async move |_, _| {
            if let Err(err) = task.await {
                log::warn!("local history rename rewrite failed: {err:#}");
            }
        })
        .detach();
    }

    pub fn load_entries_for_path(
        &self,
        project_path: ProjectPath,
        cx: &App,
    ) -> Task<Result<Vec<LocalHistoryEntry>>> {
        let settings = LocalHistorySettings::get_global(cx).clone();
        let worktree = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx);

        let Some(worktree) = worktree else {
            return Task::ready(Ok(Vec::new()));
        };

        if !worktree.read(cx).is_local() {
            return Task::ready(Ok(Vec::new()));
        }

        let path_style = worktree.read(cx).path_style();
        let worktree_path = worktree.read(cx).abs_path().to_path_buf();
        let relative_path = project_path.path.display(path_style).to_string();
        let endpoints = settings.resolved_storage_paths();

        cx.background_spawn(async move {
            let mut entries = Vec::new();
            for endpoint in endpoints {
                entries.extend(load_entries_from_endpoint(
                    &endpoint,
                    &worktree_path,
                    &relative_path,
                ));
            }
            entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            Ok(entries)
        })
    }

    pub fn load_entry_text(&self, entry: LocalHistoryEntry, cx: &App) -> Task<Result<Arc<str>>> {
        cx.background_spawn(async move {
            let path = entry.snapshot_path();
            let data = fs::read(&path).with_context(|| format!("reading {:?}", path))?;
            let decoded = zstd::decode_all(&data[..])?;
            let text = String::from_utf8(decoded)?;
            Ok(Arc::<str>::from(text))
        })
    }

    pub fn prune_active_worktree(
        &self,
        project_path: ProjectPath,
        cx: &App,
    ) -> Task<Result<usize>> {
        let settings = LocalHistorySettings::get_global(cx).clone();
        let worktree = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx);

        let Some(worktree) = worktree else {
            return Task::ready(Ok(0));
        };

        if !worktree.read(cx).is_local() {
            return Task::ready(Ok(0));
        }

        let worktree_path = worktree.read(cx).abs_path().to_path_buf();
        let active_root = settings.resolved_active_path();

        cx.background_spawn(async move {
            let removed = prune_worktree_if_needed(&active_root, &worktree_path, &settings)?;
            Ok(removed)
        })
    }

    pub fn transfer_history(
        &self,
        source: PathBuf,
        destination: PathBuf,
        mode: LocalHistoryTransferMode,
        cx: &App,
    ) -> Task<Result<()>> {
        cx.background_spawn(async move {
            if !source.exists() {
                return Ok(());
            }
            copy_dir_recursive(&source, &destination)?;
            if matches!(mode, LocalHistoryTransferMode::Move) {
                if let Err(err) = fs::remove_dir_all(&source) {
                    log::warn!(
                        "local history failed to remove source directory {:?}: {err:#}",
                        source
                    );
                }
            }
            Ok(())
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalHistoryTransferMode {
    Copy,
    Move,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocalHistoryEntryRecord {
    id: String,
    timestamp: i64,
    relative_path: String,
    snapshot_path: String,
    compressed_bytes: u64,
    uncompressed_bytes: u64,
}

fn write_snapshot(
    root: &Path,
    worktree_path: &Path,
    relative_path: &str,
    text: &str,
    timestamp: OffsetDateTime,
) -> Result<LocalHistoryEntryRecord> {
    let worktree_hash = hash_worktree_path(worktree_path);
    let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
    let snapshots_dir = worktree_dir.join(SNAPSHOTS_DIR_NAME);
    fs::create_dir_all(&snapshots_dir).with_context(|| format!("creating {:?}", snapshots_dir))?;

    let entry_id = format!("{}-{}", timestamp.unix_timestamp(), random::<u64>());
    let snapshot_file_name = format!("{entry_id}.zst");
    let snapshot_file_path = snapshots_dir.join(&snapshot_file_name);
    let compressed = zstd::encode_all(text.as_bytes(), 0)?;
    fs::write(&snapshot_file_path, &compressed)
        .with_context(|| format!("writing {:?}", snapshot_file_path))?;

    let record = LocalHistoryEntryRecord {
        id: entry_id.clone(),
        timestamp: timestamp.unix_timestamp(),
        relative_path: relative_path.to_string(),
        snapshot_path: PathBuf::from(WORKTREES_DIR)
            .join(&worktree_hash)
            .join(SNAPSHOTS_DIR_NAME)
            .join(snapshot_file_name)
            .to_string_lossy()
            .to_string(),
        compressed_bytes: compressed.len() as u64,
        uncompressed_bytes: text.len() as u64,
    };

    append_record(&worktree_dir.join(INDEX_FILE_NAME), &record)?;
    Ok(record)
}

fn append_record(index_path: &Path, record: &LocalHistoryEntryRecord) -> Result<()> {
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(index_path)
        .with_context(|| format!("opening {:?}", index_path))?;
    let line = serde_json::to_string(record)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

fn load_entries_from_endpoint(
    root: &Path,
    worktree_path: &Path,
    relative_path: &str,
) -> Vec<LocalHistoryEntry> {
    let worktree_hash = hash_worktree_path(worktree_path);
    let index_path = root
        .join(WORKTREES_DIR)
        .join(&worktree_hash)
        .join(INDEX_FILE_NAME);

    let file = match fs::File::open(&index_path) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines().flatten() {
        let Ok(record) = serde_json::from_str::<LocalHistoryEntryRecord>(&line) else {
            continue;
        };
        if record.relative_path != relative_path {
            continue;
        }
        let Ok(timestamp) = OffsetDateTime::from_unix_timestamp(record.timestamp) else {
            continue;
        };
        let snapshot_relative_path = if record.snapshot_path.starts_with(WORKTREES_DIR) {
            PathBuf::from(&record.snapshot_path)
        } else {
            PathBuf::from(WORKTREES_DIR)
                .join(&worktree_hash)
                .join(&record.snapshot_path)
        };
        entries.push(LocalHistoryEntry {
            id: record.id,
            timestamp,
            relative_path: Arc::from(record.relative_path),
            endpoint_root: root.to_path_buf(),
            snapshot_relative_path,
            compressed_bytes: record.compressed_bytes,
            uncompressed_bytes: record.uncompressed_bytes,
        });
    }
    entries
}

fn prune_worktree_if_needed(
    root: &Path,
    worktree_path: &Path,
    settings: &LocalHistorySettings,
) -> Result<usize> {
    prune_worktree_if_needed_at(root, worktree_path, settings, OffsetDateTime::now_utc())
}

fn prune_worktree_if_needed_at(
    root: &Path,
    worktree_path: &Path,
    settings: &LocalHistorySettings,
    now: OffsetDateTime,
) -> Result<usize> {
    let worktree_hash = hash_worktree_path(worktree_path);
    let worktree_dir = root.join(WORKTREES_DIR).join(worktree_hash);
    let index_path = worktree_dir.join(INDEX_FILE_NAME);
    let mut entries = match read_index(&index_path) {
        Ok(entries) => entries,
        Err(err) => {
            if index_path.exists() {
                return Err(err);
            }
            return Ok(0);
        }
    };
    if entries.is_empty() {
        return Ok(0);
    }

    let mut total_size: u64 = entries.iter().map(|entry| entry.compressed_bytes).sum();
    let cap_bytes = cap_bytes_for_root(root, settings);
    let cutoff = now.saturating_sub(time::Duration::days(settings.min_age_days as i64));

    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut removed = Vec::new();
    match settings.prune_policy {
        LocalHistoryPrunePolicy::Both => {
            if total_size > cap_bytes {
                for entry in &entries {
                    if entry.timestamp <= cutoff.unix_timestamp() {
                        removed.push(entry.id.clone());
                        total_size = total_size.saturating_sub(entry.compressed_bytes);
                        if total_size <= cap_bytes {
                            break;
                        }
                    }
                }
            }
        }
        LocalHistoryPrunePolicy::SizeOnly => {
            if total_size > cap_bytes {
                for entry in &entries {
                    removed.push(entry.id.clone());
                    total_size = total_size.saturating_sub(entry.compressed_bytes);
                    if total_size <= cap_bytes {
                        break;
                    }
                }
            }
        }
        LocalHistoryPrunePolicy::AgeOnly => {
            for entry in &entries {
                if entry.timestamp <= cutoff.unix_timestamp() {
                    removed.push(entry.id.clone());
                }
            }
        }
        LocalHistoryPrunePolicy::Any => {
            for entry in &entries {
                if entry.timestamp <= cutoff.unix_timestamp() {
                    removed.push(entry.id.clone());
                    total_size = total_size.saturating_sub(entry.compressed_bytes);
                }
            }
            if total_size > cap_bytes {
                for entry in &entries {
                    if removed.contains(&entry.id) {
                        continue;
                    }
                    removed.push(entry.id.clone());
                    total_size = total_size.saturating_sub(entry.compressed_bytes);
                    if total_size <= cap_bytes {
                        break;
                    }
                }
            }
        }
    }

    if removed.is_empty() {
        return Ok(0);
    }

    let remaining: Vec<_> = entries
        .into_iter()
        .filter(|entry| !removed.contains(&entry.id))
        .collect();

    for entry_id in &removed {
        let snapshot_path = worktree_dir
            .join(SNAPSHOTS_DIR_NAME)
            .join(format!("{entry_id}.zst"));
        if let Err(err) = fs::remove_file(&snapshot_path) {
            log::warn!(
                "local history failed to remove snapshot {:?}: {err:#}",
                snapshot_path
            );
        }
    }

    write_index(&index_path, &remaining)?;
    Ok(removed.len())
}

fn read_index(index_path: &Path) -> Result<Vec<LocalHistoryEntryRecord>> {
    let file = fs::File::open(index_path).with_context(|| format!("opening {:?}", index_path))?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Ok(record) = serde_json::from_str::<LocalHistoryEntryRecord>(&line) {
            entries.push(record);
        }
    }
    Ok(entries)
}

fn endpoint_has_entries_for_path(root: &Path, worktree_path: &Path, relative_path: &str) -> bool {
    let index_path = root
        .join(WORKTREES_DIR)
        .join(hash_worktree_path(worktree_path))
        .join(INDEX_FILE_NAME);

    match index_contains_relative_path(&index_path, relative_path) {
        Ok(has_entries) => has_entries,
        Err(err) => {
            if index_path.exists() {
                log::warn!("local history failed to read {:?}: {err:#}", index_path);
            }
            false
        }
    }
}

fn index_contains_relative_path(index_path: &Path, relative_path: &str) -> Result<bool> {
    let file = match fs::File::open(index_path) {
        Ok(file) => file,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err).with_context(|| format!("opening {:?}", index_path)),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if let Ok(record) = serde_json::from_str::<LocalHistoryEntryRecord>(&line)
            && record.relative_path == relative_path
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn write_index(index_path: &Path, entries: &[LocalHistoryEntryRecord]) -> Result<()> {
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp_path = index_path.with_extension("jsonl.tmp");
    let mut file = fs::File::create(&tmp_path)?;
    for entry in entries {
        let line = serde_json::to_string(entry)?;
        writeln!(file, "{line}")?;
    }
    if index_path.exists() {
        fs::remove_file(index_path).with_context(|| format!("removing {:?}", index_path))?;
    }
    fs::rename(tmp_path, index_path)?;
    Ok(())
}

fn cap_bytes_for_root(root: &Path, settings: &LocalHistorySettings) -> u64 {
    let free_space = fs2::available_space(root).log_err().unwrap_or(0);
    cap_bytes_for_root_with_space(free_space, settings)
}

fn cap_bytes_for_root_with_space(available_space: u64, settings: &LocalHistorySettings) -> u64 {
    let percent = settings.cap_free_space_percent.max(0.0) as f64 / 100.0;
    let percent_cap = ((available_space as f64) * percent).round() as u64;
    percent_cap.max(settings.cap_min_bytes)
}

fn is_excluded(settings: &LocalHistorySettings, path: &RelPath, path_style: PathStyle) -> bool {
    if settings.exclude_globs.is_empty() {
        return false;
    }
    let matcher = PathMatcher::new(
        settings.exclude_globs.iter().map(String::as_str),
        path_style,
    )
    .log_err()
    .unwrap_or_default();
    matcher.is_match(path)
}

fn hash_worktree_path(path: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    hex::encode(hasher.finalize())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    if !source.exists() {
        return Ok(());
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &destination_path)
                .with_context(|| format!("copying {:?} to {:?}", source_path, destination_path))?;
        }
    }
    Ok(())
}

fn copy_dir_recursive_skip_existing(source: &Path, destination: &Path) -> Result<()> {
    if !source.exists() {
        return Ok(());
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive_skip_existing(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            if destination_path.exists() {
                continue;
            }
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &destination_path)
                .with_context(|| format!("copying {:?} to {:?}", source_path, destination_path))?;
        }
    }
    Ok(())
}

fn rewrite_index_records<F>(index_path: &Path, mut rewrite: F) -> Result<usize>
where
    F: FnMut(&mut LocalHistoryEntryRecord) -> bool,
{
    if !index_path.exists() {
        return Ok(0);
    }
    let temp_path = index_path.with_extension("jsonl.tmp");
    let file = fs::File::open(index_path)?;
    let reader = BufReader::new(file);
    let mut writer = std::io::BufWriter::new(fs::File::create(&temp_path)?);
    let mut modified = 0usize;

    for line in reader.lines().flatten() {
        if let Ok(mut record) = serde_json::from_str::<LocalHistoryEntryRecord>(&line) {
            if rewrite(&mut record) {
                modified += 1;
            }
            let encoded = serde_json::to_string(&record)?;
            writer.write_all(encoded.as_bytes())?;
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }
    writer.flush()?;
    fs::rename(&temp_path, index_path)?;
    Ok(modified)
}

fn append_rewritten_records<F>(source_index: &Path, dest_index: &Path, mut rewrite: F) -> Result<()>
where
    F: FnMut(&mut LocalHistoryEntryRecord) -> bool,
{
    if !source_index.exists() {
        return Ok(());
    }
    if let Some(parent) = dest_index.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = fs::File::open(source_index)?;
    let reader = BufReader::new(file);
    let mut writer = std::io::BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(dest_index)?,
    );

    for line in reader.lines().flatten() {
        if let Ok(mut record) = serde_json::from_str::<LocalHistoryEntryRecord>(&line) {
            rewrite(&mut record);
            let encoded = serde_json::to_string(&record)?;
            writer.write_all(encoded.as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }
    writer.flush()?;
    Ok(())
}

fn rewrite_snapshot_path_hash(
    record: &mut LocalHistoryEntryRecord,
    old_hash: &str,
    new_hash: &str,
) -> bool {
    let old_prefix = PathBuf::from(WORKTREES_DIR).join(old_hash);
    let path = Path::new(&record.snapshot_path);
    let Ok(suffix) = path.strip_prefix(&old_prefix) else {
        return false;
    };
    let new_path = PathBuf::from(WORKTREES_DIR).join(new_hash).join(suffix);
    record.snapshot_path = new_path.to_string_lossy().to_string();
    true
}

fn migrate_worktree_history_dir(root: &Path, old_path: &Path, new_path: &Path) -> Result<()> {
    let old_hash = hash_worktree_path(old_path);
    let new_hash = hash_worktree_path(new_path);
    if old_hash == new_hash {
        return Ok(());
    }

    let old_dir = root.join(WORKTREES_DIR).join(&old_hash);
    if !old_dir.exists() {
        return Ok(());
    }

    let new_dir = root.join(WORKTREES_DIR).join(&new_hash);
    if !new_dir.exists() {
        if let Some(parent) = new_dir.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Err(err) = fs::rename(&old_dir, &new_dir) {
            log::warn!(
                "local history failed to move worktree directory {:?} -> {:?}: {err:#}",
                old_dir,
                new_dir
            );
            copy_dir_recursive(&old_dir, &new_dir)?;
        }
        let index_path = new_dir.join(INDEX_FILE_NAME);
        let _ = rewrite_index_records(&index_path, |record| {
            rewrite_snapshot_path_hash(record, &old_hash, &new_hash)
        })?;
        if old_dir.exists() {
            fs::remove_dir_all(&old_dir).ok();
        }
        return Ok(());
    }

    let old_snapshots = old_dir.join(SNAPSHOTS_DIR_NAME);
    let new_snapshots = new_dir.join(SNAPSHOTS_DIR_NAME);
    copy_dir_recursive_skip_existing(&old_snapshots, &new_snapshots)?;

    let old_index = old_dir.join(INDEX_FILE_NAME);
    let new_index = new_dir.join(INDEX_FILE_NAME);
    append_rewritten_records(&old_index, &new_index, |record| {
        rewrite_snapshot_path_hash(record, &old_hash, &new_hash)
    })?;

    fs::remove_dir_all(&old_dir).ok();
    Ok(())
}

fn rewrite_index_for_renames(
    root: &Path,
    worktree_path: &Path,
    renames: &HashMap<String, String>,
) -> Result<()> {
    let worktree_hash = hash_worktree_path(worktree_path);
    let index_path = root
        .join(WORKTREES_DIR)
        .join(&worktree_hash)
        .join(INDEX_FILE_NAME);
    let _ = rewrite_index_records(&index_path, |record| {
        if let Some(new_path) = renames.get(&record.relative_path) {
            record.relative_path = new_path.clone();
            return true;
        }
        false
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    fn test_settings() -> LocalHistorySettings {
        LocalHistorySettings {
            enabled: true,
            capture_on_save: true,
            capture_on_edit_idle_ms: None,
            capture_on_focus_change: false,
            capture_on_window_change: false,
            capture_on_task: false,
            capture_on_external_change: false,
            storage_paths: Vec::new(),
            active_storage_path: None,
            min_age_days: 100,
            cap_free_space_percent: 10.0,
            cap_min_bytes: 300,
            prune_policy: LocalHistoryPrunePolicy::Both,
            exclude_globs: Vec::new(),
        }
    }

    #[test]
    fn cap_bytes_for_root_with_space_respects_min() {
        let mut settings = test_settings();
        let cap = cap_bytes_for_root_with_space(1_000, &settings);
        assert_eq!(cap, 300);

        settings.cap_min_bytes = 50;
        let cap = cap_bytes_for_root_with_space(1_000, &settings);
        assert_eq!(cap, 100);
    }

    #[test]
    fn load_entries_from_endpoint_back_compat_snapshot_path() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let worktree_hash = hash_worktree_path(worktree_path);
        let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
        let snapshots_dir = worktree_dir.join(SNAPSHOTS_DIR_NAME);
        std::fs::create_dir_all(&snapshots_dir).unwrap();

        let snapshot_file = snapshots_dir.join("entry.zst");
        std::fs::write(&snapshot_file, b"fake").unwrap();

        let index_path = worktree_dir.join(INDEX_FILE_NAME);
        let record = LocalHistoryEntryRecord {
            id: "entry".to_string(),
            timestamp: 1,
            relative_path: "file.txt".to_string(),
            snapshot_path: PathBuf::from(SNAPSHOTS_DIR_NAME)
                .join("entry.zst")
                .to_string_lossy()
                .to_string(),
            compressed_bytes: 4,
            uncompressed_bytes: 4,
        };
        append_record(&index_path, &record).unwrap();

        let entries = load_entries_from_endpoint(root, worktree_path, "file.txt");
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].snapshot_relative_path,
            PathBuf::from(WORKTREES_DIR)
                .join(&worktree_hash)
                .join(SNAPSHOTS_DIR_NAME)
                .join("entry.zst")
        );
    }

    #[test]
    fn migrate_worktree_history_updates_snapshot_paths() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let old_worktree_path = Path::new("/test/worktree-old");
        let new_worktree_path = Path::new("/test/worktree-new");

        let timestamp = OffsetDateTime::from_unix_timestamp(1).unwrap();
        write_snapshot(root, old_worktree_path, "file.txt", "contents\n", timestamp).unwrap();

        migrate_worktree_history_dir(root, old_worktree_path, new_worktree_path).unwrap();

        let entries = load_entries_from_endpoint(root, new_worktree_path, "file.txt");
        assert_eq!(entries.len(), 1);
        let new_hash = hash_worktree_path(new_worktree_path);
        assert!(
            entries[0]
                .snapshot_relative_path
                .to_string_lossy()
                .contains(&new_hash),
            "snapshot path should reference new worktree hash"
        );
    }

    #[test]
    fn prune_policy_both_requires_age_and_size() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let worktree_hash = hash_worktree_path(worktree_path);
        let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
        let snapshots_dir = worktree_dir.join(SNAPSHOTS_DIR_NAME);
        std::fs::create_dir_all(&snapshots_dir).unwrap();

        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let old_ts = now - time::Duration::days(200);
        let new_ts = now - time::Duration::days(10);

        let index_path = worktree_dir.join(INDEX_FILE_NAME);
        for (id, ts, size) in [("old", old_ts, 600_u64), ("new", new_ts, 600_u64)] {
            let snapshot_path = snapshots_dir.join(format!("{id}.zst"));
            std::fs::write(&snapshot_path, b"").unwrap();
            let record = LocalHistoryEntryRecord {
                id: id.to_string(),
                timestamp: ts.unix_timestamp(),
                relative_path: "file.txt".to_string(),
                snapshot_path: PathBuf::from(WORKTREES_DIR)
                    .join(&worktree_hash)
                    .join(SNAPSHOTS_DIR_NAME)
                    .join(format!("{id}.zst"))
                    .to_string_lossy()
                    .to_string(),
                compressed_bytes: size,
                uncompressed_bytes: size,
            };
            append_record(&index_path, &record).unwrap();
        }

        let settings = LocalHistorySettings {
            cap_free_space_percent: 0.0,
            cap_min_bytes: 1,
            ..test_settings()
        };

        let removed = prune_worktree_if_needed_at(root, worktree_path, &settings, now).unwrap();
        assert_eq!(removed, 1);

        let remaining = read_index(&index_path).unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "new");
    }

    #[test]
    fn is_excluded_matches_globs() {
        let settings = LocalHistorySettings {
            exclude_globs: vec!["**/*.log".to_string()],
            ..test_settings()
        };
        let path = RelPath::new(Path::new("foo.log"), PathStyle::Posix).unwrap();
        assert!(is_excluded(&settings, path.as_ref(), PathStyle::Posix));
        let other = RelPath::new(Path::new("foo.txt"), PathStyle::Posix).unwrap();
        assert!(!is_excluded(&settings, other.as_ref(), PathStyle::Posix));
    }

    #[test]
    fn write_snapshot_roundtrip_preserves_text() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let text = "hello local history\n";
        let timestamp = OffsetDateTime::from_unix_timestamp(1_700_000_123).unwrap();

        let record = write_snapshot(root, worktree_path, "file.txt", text, timestamp).unwrap();

        let snapshot_path = root.join(record.snapshot_path);
        let compressed = std::fs::read(&snapshot_path).unwrap();
        let decoded = zstd::decode_all(&compressed[..]).unwrap();
        let decoded_text = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_text, text);

        let entries = load_entries_from_endpoint(root, worktree_path, "file.txt");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].relative_path.as_ref(), "file.txt");
    }

    #[test]
    fn load_entries_skips_invalid_index_lines() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let worktree_hash = hash_worktree_path(worktree_path);
        let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
        std::fs::create_dir_all(&worktree_dir).unwrap();
        let index_path = worktree_dir.join(INDEX_FILE_NAME);

        std::fs::write(&index_path, "not json\n").unwrap();

        let record = LocalHistoryEntryRecord {
            id: "entry".to_string(),
            timestamp: 1,
            relative_path: "file.txt".to_string(),
            snapshot_path: PathBuf::from(WORKTREES_DIR)
                .join(&worktree_hash)
                .join(SNAPSHOTS_DIR_NAME)
                .join("entry.zst")
                .to_string_lossy()
                .to_string(),
            compressed_bytes: 4,
            uncompressed_bytes: 4,
        };
        append_record(&index_path, &record).unwrap();
        std::fs::OpenOptions::new()
            .append(true)
            .open(&index_path)
            .unwrap()
            .write_all(b"{broken json\n")
            .unwrap();

        let entries = load_entries_from_endpoint(root, worktree_path, "file.txt");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "entry");
    }

    #[test]
    fn prune_policy_age_only_removes_old_entries() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let worktree_hash = hash_worktree_path(worktree_path);
        let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
        let snapshots_dir = worktree_dir.join(SNAPSHOTS_DIR_NAME);
        std::fs::create_dir_all(&snapshots_dir).unwrap();

        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let old_ts = now - time::Duration::days(200);
        let new_ts = now - time::Duration::days(10);

        let index_path = worktree_dir.join(INDEX_FILE_NAME);
        for (id, ts) in [("old", old_ts), ("new", new_ts)] {
            let snapshot_path = snapshots_dir.join(format!("{id}.zst"));
            std::fs::write(&snapshot_path, b"").unwrap();
            let record = LocalHistoryEntryRecord {
                id: id.to_string(),
                timestamp: ts.unix_timestamp(),
                relative_path: "file.txt".to_string(),
                snapshot_path: PathBuf::from(WORKTREES_DIR)
                    .join(&worktree_hash)
                    .join(SNAPSHOTS_DIR_NAME)
                    .join(format!("{id}.zst"))
                    .to_string_lossy()
                    .to_string(),
                compressed_bytes: 10,
                uncompressed_bytes: 10,
            };
            append_record(&index_path, &record).unwrap();
        }

        let settings = LocalHistorySettings {
            prune_policy: LocalHistoryPrunePolicy::AgeOnly,
            ..test_settings()
        };

        let removed = prune_worktree_if_needed_at(root, worktree_path, &settings, now).unwrap();
        assert_eq!(removed, 1);
        let remaining = read_index(&index_path).unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "new");
    }

    #[test]
    fn prune_policy_size_only_removes_until_under_cap() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let worktree_hash = hash_worktree_path(worktree_path);
        let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
        let snapshots_dir = worktree_dir.join(SNAPSHOTS_DIR_NAME);
        std::fs::create_dir_all(&snapshots_dir).unwrap();

        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let t1 = now - time::Duration::days(3);
        let t2 = now - time::Duration::days(2);
        let t3 = now - time::Duration::days(1);

        let index_path = worktree_dir.join(INDEX_FILE_NAME);
        for (id, ts) in [("a", t1), ("b", t2), ("c", t3)] {
            let snapshot_path = snapshots_dir.join(format!("{id}.zst"));
            std::fs::write(&snapshot_path, b"").unwrap();
            let record = LocalHistoryEntryRecord {
                id: id.to_string(),
                timestamp: ts.unix_timestamp(),
                relative_path: "file.txt".to_string(),
                snapshot_path: PathBuf::from(WORKTREES_DIR)
                    .join(&worktree_hash)
                    .join(SNAPSHOTS_DIR_NAME)
                    .join(format!("{id}.zst"))
                    .to_string_lossy()
                    .to_string(),
                compressed_bytes: 60,
                uncompressed_bytes: 60,
            };
            append_record(&index_path, &record).unwrap();
        }

        let settings = LocalHistorySettings {
            cap_free_space_percent: 0.0,
            cap_min_bytes: 100,
            prune_policy: LocalHistoryPrunePolicy::SizeOnly,
            ..test_settings()
        };

        let removed = prune_worktree_if_needed_at(root, worktree_path, &settings, now).unwrap();
        assert_eq!(removed, 2);
        let remaining = read_index(&index_path).unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "c");
    }

    #[test]
    fn prune_policy_any_removes_old_then_size() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let worktree_path = Path::new("/test/worktree");
        let worktree_hash = hash_worktree_path(worktree_path);
        let worktree_dir = root.join(WORKTREES_DIR).join(&worktree_hash);
        let snapshots_dir = worktree_dir.join(SNAPSHOTS_DIR_NAME);
        std::fs::create_dir_all(&snapshots_dir).unwrap();

        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let old_ts = now - time::Duration::days(200);
        let new_ts = now - time::Duration::days(1);

        let index_path = worktree_dir.join(INDEX_FILE_NAME);
        for (id, ts, size) in [("old", old_ts, 60_u64), ("new", new_ts, 120_u64)] {
            let snapshot_path = snapshots_dir.join(format!("{id}.zst"));
            std::fs::write(&snapshot_path, b"").unwrap();
            let record = LocalHistoryEntryRecord {
                id: id.to_string(),
                timestamp: ts.unix_timestamp(),
                relative_path: "file.txt".to_string(),
                snapshot_path: PathBuf::from(WORKTREES_DIR)
                    .join(&worktree_hash)
                    .join(SNAPSHOTS_DIR_NAME)
                    .join(format!("{id}.zst"))
                    .to_string_lossy()
                    .to_string(),
                compressed_bytes: size,
                uncompressed_bytes: size,
            };
            append_record(&index_path, &record).unwrap();
        }

        let settings = LocalHistorySettings {
            cap_free_space_percent: 0.0,
            cap_min_bytes: 50,
            prune_policy: LocalHistoryPrunePolicy::Any,
            ..test_settings()
        };

        let removed = prune_worktree_if_needed_at(root, worktree_path, &settings, now).unwrap();
        assert_eq!(removed, 2);
        let remaining = read_index(&index_path).unwrap();
        assert!(remaining.is_empty());
    }
}
