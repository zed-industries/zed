mod ignore;
mod worktree_settings;
#[cfg(test)]
mod worktree_tests;

use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{anyhow, Context as _, Result};
use client::{proto, Client};
use clock::ReplicaId;
use collections::{HashMap, HashSet, VecDeque};
use fs::{copy_recursive, RemoveOptions};
use fs::{
    repository::{GitFileStatus, GitRepository, RepoPath},
    Fs,
};
use futures::{
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    select_biased,
    task::Poll,
    FutureExt as _, Stream, StreamExt,
};
use fuzzy::CharBag;
use git::{DOT_GIT, GITIGNORE};
use gpui::{
    AppContext, AsyncAppContext, BackgroundExecutor, Context, EventEmitter, Model, ModelContext,
    Task,
};
use ignore::IgnoreStack;
use itertools::Itertools;
use language::{
    proto::{deserialize_version, serialize_fingerprint, serialize_line_ending, serialize_version},
    Buffer, Capability, DiagnosticEntry, File as _, LineEnding, PointUtf16, Rope, RopeFingerprint,
    Unclipped,
};
use lsp::{DiagnosticSeverity, LanguageServerId};
use parking_lot::Mutex;
use postage::{
    barrier,
    prelude::{Sink as _, Stream as _},
    watch,
};
use serde::Serialize;
use settings::{Settings, SettingsLocation, SettingsStore};
use smol::channel::{self, Sender};
use std::{
    any::Any,
    cmp::{self, Ordering},
    convert::TryFrom,
    ffi::OsStr,
    fmt,
    future::Future,
    mem,
    ops::{AddAssign, Deref, DerefMut, Sub},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, SystemTime},
};
use sum_tree::{Bias, Edit, SeekTarget, SumTree, TreeMap, TreeSet};
use text::BufferId;
use util::{
    paths::{PathMatcher, HOME},
    ResultExt,
};

pub use worktree_settings::WorktreeSettings;

#[cfg(feature = "test-support")]
pub const FS_WATCH_LATENCY: Duration = Duration::from_millis(100);
#[cfg(not(feature = "test-support"))]
pub const FS_WATCH_LATENCY: Duration = Duration::from_millis(100);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct WorktreeId(usize);

/// A set of local or remote files that are being opened as part of a project.
/// Responsible for tracking related FS (for local)/collab (for remote) events and corresponding updates.
/// Stores git repositories data and the diagnostics for the file(s).
///
/// Has an absolute path, and may be set to be visible in Zed UI or not.
/// May correspond to a directory or a single file.
/// Possible examples:
/// * a drag and dropped file — may be added as an invisible, "ephemeral" entry to the current worktree
/// * a directory opened in Zed — may be added as a visible entry to the current worktree
///
/// Uses [`Entry`] to track the state of each file/directory, can look up absolute paths for entries.
pub enum Worktree {
    Local(LocalWorktree),
    Remote(RemoteWorktree),
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    scan_requests_tx: channel::Sender<ScanRequest>,
    path_prefixes_to_scan_tx: channel::Sender<Arc<Path>>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    _background_scanner_tasks: Vec<Task<()>>,
    share: Option<ShareState>,
    diagnostics: HashMap<
        Arc<Path>,
        Vec<(
            LanguageServerId,
            Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        )>,
    >,
    diagnostic_summaries: HashMap<Arc<Path>, HashMap<LanguageServerId, DiagnosticSummary>>,
    client: Arc<Client>,
    fs: Arc<dyn Fs>,
    fs_case_sensitive: bool,
    visible: bool,
}

struct ScanRequest {
    relative_paths: Vec<Arc<Path>>,
    done: barrier::Sender,
}

pub struct RemoteWorktree {
    snapshot: Snapshot,
    background_snapshot: Arc<Mutex<Snapshot>>,
    project_id: u64,
    client: Arc<Client>,
    updates_tx: Option<UnboundedSender<proto::UpdateWorktree>>,
    snapshot_subscriptions: VecDeque<(usize, oneshot::Sender<()>)>,
    replica_id: ReplicaId,
    diagnostic_summaries: HashMap<Arc<Path>, HashMap<LanguageServerId, DiagnosticSummary>>,
    visible: bool,
    disconnected: bool,
}

#[derive(Clone)]
pub struct Snapshot {
    id: WorktreeId,
    abs_path: Arc<Path>,
    root_name: String,
    root_char_bag: CharBag,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
    repository_entries: TreeMap<RepositoryWorkDirectory, RepositoryEntry>,

    /// A number that increases every time the worktree begins scanning
    /// a set of paths from the filesystem. This scanning could be caused
    /// by some operation performed on the worktree, such as reading or
    /// writing a file, or by an event reported by the filesystem.
    scan_id: usize,

    /// The latest scan id that has completed, and whose preceding scans
    /// have all completed. The current `scan_id` could be more than one
    /// greater than the `completed_scan_id` if operations are performed
    /// on the worktree while it is processing a file-system event.
    completed_scan_id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryEntry {
    pub(crate) work_directory: WorkDirectoryEntry,
    pub(crate) branch: Option<Arc<str>>,
}

impl RepositoryEntry {
    pub fn branch(&self) -> Option<Arc<str>> {
        self.branch.clone()
    }

    pub fn work_directory_id(&self) -> ProjectEntryId {
        *self.work_directory
    }

    pub fn work_directory(&self, snapshot: &Snapshot) -> Option<RepositoryWorkDirectory> {
        snapshot
            .entry_for_id(self.work_directory_id())
            .map(|entry| RepositoryWorkDirectory(entry.path.clone()))
    }

    pub fn build_update(&self, _: &Self) -> proto::RepositoryEntry {
        proto::RepositoryEntry {
            work_directory_id: self.work_directory_id().to_proto(),
            branch: self.branch.as_ref().map(|str| str.to_string()),
        }
    }
}

impl From<&RepositoryEntry> for proto::RepositoryEntry {
    fn from(value: &RepositoryEntry) -> Self {
        proto::RepositoryEntry {
            work_directory_id: value.work_directory.to_proto(),
            branch: value.branch.as_ref().map(|str| str.to_string()),
        }
    }
}

/// This path corresponds to the 'content path' (the folder that contains the .git)
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct RepositoryWorkDirectory(pub(crate) Arc<Path>);

impl Default for RepositoryWorkDirectory {
    fn default() -> Self {
        RepositoryWorkDirectory(Arc::from(Path::new("")))
    }
}

impl AsRef<Path> for RepositoryWorkDirectory {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct WorkDirectoryEntry(ProjectEntryId);

impl WorkDirectoryEntry {
    pub(crate) fn relativize(&self, worktree: &Snapshot, path: &Path) -> Result<RepoPath> {
        let entry = worktree
            .entry_for_id(self.0)
            .ok_or_else(|| anyhow!("entry not found"))?;
        let path = path
            .strip_prefix(&entry.path)
            .map_err(|_| anyhow!("could not relativize {:?} against {:?}", path, entry.path))?;
        Ok(path.into())
    }
}

impl Deref for WorkDirectoryEntry {
    type Target = ProjectEntryId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProjectEntryId> for WorkDirectoryEntry {
    fn from(value: ProjectEntryId) -> Self {
        WorkDirectoryEntry(value)
    }
}

#[derive(Debug, Clone)]
pub struct LocalSnapshot {
    snapshot: Snapshot,
    /// All of the gitignore files in the worktree, indexed by their relative path.
    /// The boolean indicates whether the gitignore needs to be updated.
    ignores_by_parent_abs_path: HashMap<Arc<Path>, (Arc<Gitignore>, bool)>,
    /// All of the git repositories in the worktree, indexed by the project entry
    /// id of their parent directory.
    git_repositories: TreeMap<ProjectEntryId, LocalRepositoryEntry>,
    file_scan_exclusions: Vec<PathMatcher>,
    private_files: Vec<PathMatcher>,
}

struct BackgroundScannerState {
    snapshot: LocalSnapshot,
    scanned_dirs: HashSet<ProjectEntryId>,
    path_prefixes_to_scan: HashSet<Arc<Path>>,
    paths_to_scan: HashSet<Arc<Path>>,
    /// The ids of all of the entries that were removed from the snapshot
    /// as part of the current update. These entry ids may be re-used
    /// if the same inode is discovered at a new path, or if the given
    /// path is re-created after being deleted.
    removed_entry_ids: HashMap<u64, ProjectEntryId>,
    changed_paths: Vec<Arc<Path>>,
    prev_snapshot: Snapshot,
}

#[derive(Debug, Clone)]
pub struct LocalRepositoryEntry {
    pub(crate) git_dir_scan_id: usize,
    pub(crate) repo_ptr: Arc<Mutex<dyn GitRepository>>,
    /// Path to the actual .git folder.
    /// Note: if .git is a file, this points to the folder indicated by the .git file
    pub(crate) git_dir_path: Arc<Path>,
}

impl LocalRepositoryEntry {
    pub fn repo(&self) -> &Arc<Mutex<dyn GitRepository>> {
        &self.repo_ptr
    }
}

impl Deref for LocalSnapshot {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl DerefMut for LocalSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.snapshot
    }
}

enum ScanState {
    Started,
    Updated {
        snapshot: LocalSnapshot,
        changes: UpdatedEntriesSet,
        barrier: Option<barrier::Sender>,
        scanning: bool,
    },
}

struct ShareState {
    project_id: u64,
    snapshots_tx:
        mpsc::UnboundedSender<(LocalSnapshot, UpdatedEntriesSet, UpdatedGitRepositoriesSet)>,
    resume_updates: watch::Sender<()>,
    _maintain_remote_snapshot: Task<Option<()>>,
}

#[derive(Clone)]
pub enum Event {
    UpdatedEntries(UpdatedEntriesSet),
    UpdatedGitRepositories(UpdatedGitRepositoriesSet),
}

impl EventEmitter<Event> for Worktree {}

impl Worktree {
    pub async fn local(
        client: Arc<Client>,
        path: impl Into<Arc<Path>>,
        visible: bool,
        fs: Arc<dyn Fs>,
        next_entry_id: Arc<AtomicUsize>,
        cx: &mut AsyncAppContext,
    ) -> Result<Model<Self>> {
        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let abs_path = path.into();

        let metadata = fs
            .metadata(&abs_path)
            .await
            .context("failed to stat worktree path")?;

        let fs_case_sensitive = fs.is_case_sensitive().await.unwrap_or_else(|e| {
            log::error!(
                "Failed to determine whether filesystem is case sensitive (falling back to true) due to error: {e:#}"
            );
            true
        });

        let closure_fs = Arc::clone(&fs);
        let closure_next_entry_id = Arc::clone(&next_entry_id);
        let closure_abs_path = abs_path.to_path_buf();
        cx.new_model(move |cx: &mut ModelContext<Worktree>| {
            cx.observe_global::<SettingsStore>(move |this, cx| {
                if let Self::Local(this) = this {
                    let new_file_scan_exclusions = path_matchers(
                        WorktreeSettings::get_global(cx)
                            .file_scan_exclusions
                            .as_deref(),
                        "file_scan_exclusions",
                    );
                    let new_private_files = path_matchers(
                        WorktreeSettings::get(Some(settings::SettingsLocation {
                            worktree_id: cx.handle().entity_id().as_u64() as usize,
                            path: Path::new("")
                        }), cx).private_files.as_deref(),
                        "private_files",
                    );

                    if new_file_scan_exclusions != this.snapshot.file_scan_exclusions
                        || new_private_files != this.snapshot.private_files
                    {
                        this.snapshot.file_scan_exclusions = new_file_scan_exclusions;
                        this.snapshot.private_files = new_private_files;

                        log::info!(
                            "Re-scanning directories, new scan exclude files: {:?}, new dotenv files: {:?}",
                            this.snapshot
                                .file_scan_exclusions
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>(),
                            this.snapshot
                                .private_files
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                        );

                        let (scan_requests_tx, scan_requests_rx) = channel::unbounded();
                        let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) =
                            channel::unbounded();
                        this.scan_requests_tx = scan_requests_tx;
                        this.path_prefixes_to_scan_tx = path_prefixes_to_scan_tx;
                        this._background_scanner_tasks = start_background_scan_tasks(
                            &closure_abs_path,
                            this.snapshot(),
                            scan_requests_rx,
                            path_prefixes_to_scan_rx,
                            Arc::clone(&closure_next_entry_id),
                            Arc::clone(&closure_fs),
                            cx,
                        );
                        this.is_scanning = watch::channel_with(true);
                    }
                }
            })
            .detach();

            let root_name = abs_path
                .file_name()
                .map_or(String::new(), |f| f.to_string_lossy().to_string());

            let mut snapshot = LocalSnapshot {
                file_scan_exclusions: path_matchers(
                    WorktreeSettings::get_global(cx)
                        .file_scan_exclusions
                        .as_deref(),
                    "file_scan_exclusions",
                ),
                private_files: path_matchers(
                    WorktreeSettings::get(Some(SettingsLocation {
                        worktree_id: cx.handle().entity_id().as_u64() as usize,
                        path: Path::new(""),
                    }), cx).private_files.as_deref(),
                    "private_files",
                ),
                ignores_by_parent_abs_path: Default::default(),
                git_repositories: Default::default(),
                snapshot: Snapshot {
                    id: WorktreeId::from_usize(cx.entity_id().as_u64() as usize),
                    abs_path: abs_path.to_path_buf().into(),
                    root_name: root_name.clone(),
                    root_char_bag: root_name.chars().map(|c| c.to_ascii_lowercase()).collect(),
                    entries_by_path: Default::default(),
                    entries_by_id: Default::default(),
                    repository_entries: Default::default(),
                    scan_id: 1,
                    completed_scan_id: 0,
                },
            };

            if let Some(metadata) = metadata {
                snapshot.insert_entry(
                    Entry::new(
                        Arc::from(Path::new("")),
                        &metadata,
                        &next_entry_id,
                        snapshot.root_char_bag,
                    ),
                    fs.as_ref(),
                );
            }

            let (scan_requests_tx, scan_requests_rx) = channel::unbounded();
            let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = channel::unbounded();
            let task_snapshot = snapshot.clone();
            Worktree::Local(LocalWorktree {
                snapshot,
                is_scanning: watch::channel_with(true),
                share: None,
                scan_requests_tx,
                path_prefixes_to_scan_tx,
                _background_scanner_tasks: start_background_scan_tasks(
                    &abs_path,
                    task_snapshot,
                    scan_requests_rx,
                    path_prefixes_to_scan_rx,
                    Arc::clone(&next_entry_id),
                    Arc::clone(&fs),
                    cx,
                ),
                diagnostics: Default::default(),
                diagnostic_summaries: Default::default(),
                client,
                fs,
                fs_case_sensitive,
                visible,
            })
        })
    }

    pub fn remote(
        project_remote_id: u64,
        replica_id: ReplicaId,
        worktree: proto::WorktreeMetadata,
        client: Arc<Client>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx: &mut ModelContext<Self>| {
            let snapshot = Snapshot {
                id: WorktreeId(worktree.id as usize),
                abs_path: Arc::from(PathBuf::from(worktree.abs_path)),
                root_name: worktree.root_name.clone(),
                root_char_bag: worktree
                    .root_name
                    .chars()
                    .map(|c| c.to_ascii_lowercase())
                    .collect(),
                entries_by_path: Default::default(),
                entries_by_id: Default::default(),
                repository_entries: Default::default(),
                scan_id: 1,
                completed_scan_id: 0,
            };

            let (updates_tx, mut updates_rx) = mpsc::unbounded();
            let background_snapshot = Arc::new(Mutex::new(snapshot.clone()));
            let (mut snapshot_updated_tx, mut snapshot_updated_rx) = watch::channel();

            cx.background_executor()
                .spawn({
                    let background_snapshot = background_snapshot.clone();
                    async move {
                        while let Some(update) = updates_rx.next().await {
                            if let Err(error) =
                                background_snapshot.lock().apply_remote_update(update)
                            {
                                log::error!("error applying worktree update: {}", error);
                            }
                            snapshot_updated_tx.send(()).await.ok();
                        }
                    }
                })
                .detach();

            cx.spawn(|this, mut cx| async move {
                while (snapshot_updated_rx.recv().await).is_some() {
                    this.update(&mut cx, |this, cx| {
                        let this = this.as_remote_mut().unwrap();
                        this.snapshot = this.background_snapshot.lock().clone();
                        cx.emit(Event::UpdatedEntries(Arc::from([])));
                        cx.notify();
                        while let Some((scan_id, _)) = this.snapshot_subscriptions.front() {
                            if this.observed_snapshot(*scan_id) {
                                let (_, tx) = this.snapshot_subscriptions.pop_front().unwrap();
                                let _ = tx.send(());
                            } else {
                                break;
                            }
                        }
                    })?;
                }
                anyhow::Ok(())
            })
            .detach();

            Worktree::Remote(RemoteWorktree {
                project_id: project_remote_id,
                replica_id,
                snapshot: snapshot.clone(),
                background_snapshot,
                updates_tx: Some(updates_tx),
                snapshot_subscriptions: Default::default(),
                client: client.clone(),
                diagnostic_summaries: Default::default(),
                visible: worktree.visible,
                disconnected: false,
            })
        })
    }

    pub fn as_local(&self) -> Option<&LocalWorktree> {
        if let Worktree::Local(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn as_remote(&self) -> Option<&RemoteWorktree> {
        if let Worktree::Remote(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalWorktree> {
        if let Worktree::Local(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn as_remote_mut(&mut self) -> Option<&mut RemoteWorktree> {
        if let Worktree::Remote(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Worktree::Local(_))
    }

    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }

    pub fn snapshot(&self) -> Snapshot {
        match self {
            Worktree::Local(worktree) => worktree.snapshot().snapshot,
            Worktree::Remote(worktree) => worktree.snapshot(),
        }
    }

    pub fn scan_id(&self) -> usize {
        match self {
            Worktree::Local(worktree) => worktree.snapshot.scan_id,
            Worktree::Remote(worktree) => worktree.snapshot.scan_id,
        }
    }

    pub fn completed_scan_id(&self) -> usize {
        match self {
            Worktree::Local(worktree) => worktree.snapshot.completed_scan_id,
            Worktree::Remote(worktree) => worktree.snapshot.completed_scan_id,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Worktree::Local(worktree) => worktree.visible,
            Worktree::Remote(worktree) => worktree.visible,
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match self {
            Worktree::Local(_) => 0,
            Worktree::Remote(worktree) => worktree.replica_id,
        }
    }

    pub fn diagnostic_summaries(
        &self,
    ) -> impl Iterator<Item = (Arc<Path>, LanguageServerId, DiagnosticSummary)> + '_ {
        match self {
            Worktree::Local(worktree) => &worktree.diagnostic_summaries,
            Worktree::Remote(worktree) => &worktree.diagnostic_summaries,
        }
        .iter()
        .flat_map(|(path, summaries)| {
            summaries
                .iter()
                .map(move |(&server_id, &summary)| (path.clone(), server_id, summary))
        })
    }

    pub fn abs_path(&self) -> Arc<Path> {
        match self {
            Worktree::Local(worktree) => worktree.abs_path.clone(),
            Worktree::Remote(worktree) => worktree.abs_path.clone(),
        }
    }

    pub fn root_file(&self, cx: &mut ModelContext<Self>) -> Option<Arc<File>> {
        let entry = self.root_entry()?;
        Some(File::for_entry(entry.clone(), cx.handle()))
    }
}

fn start_background_scan_tasks(
    abs_path: &Path,
    snapshot: LocalSnapshot,
    scan_requests_rx: channel::Receiver<ScanRequest>,
    path_prefixes_to_scan_rx: channel::Receiver<Arc<Path>>,
    next_entry_id: Arc<AtomicUsize>,
    fs: Arc<dyn Fs>,
    cx: &mut ModelContext<'_, Worktree>,
) -> Vec<Task<()>> {
    let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded();
    let background_scanner = cx.background_executor().spawn({
        let abs_path = abs_path.to_path_buf();
        let background = cx.background_executor().clone();
        async move {
            let events = fs.watch(&abs_path, FS_WATCH_LATENCY).await;
            let case_sensitive = fs.is_case_sensitive().await.unwrap_or_else(|e| {
                log::error!(
                    "Failed to determine whether filesystem is case sensitive (falling back to true) due to error: {e:#}"
                );
                true
            });

            BackgroundScanner::new(
                snapshot,
                next_entry_id,
                fs,
                case_sensitive,
                scan_states_tx,
                background,
                scan_requests_rx,
                path_prefixes_to_scan_rx,
            )
            .run(events)
            .await;
        }
    });
    let scan_state_updater = cx.spawn(|this, mut cx| async move {
        while let Some((state, this)) = scan_states_rx.next().await.zip(this.upgrade()) {
            this.update(&mut cx, |this, cx| {
                let this = this.as_local_mut().unwrap();
                match state {
                    ScanState::Started => {
                        *this.is_scanning.0.borrow_mut() = true;
                    }
                    ScanState::Updated {
                        snapshot,
                        changes,
                        barrier,
                        scanning,
                    } => {
                        *this.is_scanning.0.borrow_mut() = scanning;
                        this.set_snapshot(snapshot, changes, cx);
                        drop(barrier);
                    }
                }
                cx.notify();
            })
            .ok();
        }
    });
    vec![background_scanner, scan_state_updater]
}

fn path_matchers(values: Option<&[String]>, context: &'static str) -> Vec<PathMatcher> {
    values
        .unwrap_or(&[])
        .iter()
        .sorted()
        .filter_map(|pattern| {
            PathMatcher::new(pattern)
                .map(Some)
                .unwrap_or_else(|e| {
                    log::error!(
                        "Skipping pattern {pattern} in `{}` project settings due to parsing error: {e:#}", context
                    );
                    None
                })
        })
        .collect()
}

impl LocalWorktree {
    pub fn contains_abs_path(&self, path: &Path) -> bool {
        path.starts_with(&self.abs_path)
    }

    pub fn load_buffer(
        &mut self,
        id: BufferId,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Model<Buffer>>> {
        let path = Arc::from(path);
        cx.spawn(move |this, mut cx| async move {
            let (file, contents, diff_base) = this
                .update(&mut cx, |t, cx| t.as_local().unwrap().load(&path, cx))?
                .await?;
            let text_buffer = cx
                .background_executor()
                .spawn(async move { text::Buffer::new(0, id, contents) })
                .await;
            cx.new_model(|_| {
                Buffer::build(
                    text_buffer,
                    diff_base,
                    Some(Arc::new(file)),
                    Capability::ReadWrite,
                )
            })
        })
    }

    pub fn new_buffer(
        &mut self,
        buffer_id: BufferId,
        path: Arc<Path>,
        cx: &mut ModelContext<Worktree>,
    ) -> Model<Buffer> {
        let text_buffer = text::Buffer::new(0, buffer_id, "".into());
        let worktree = cx.handle();
        cx.new_model(|_| {
            Buffer::build(
                text_buffer,
                None,
                Some(Arc::new(File {
                    worktree,
                    path,
                    mtime: None,
                    entry_id: None,
                    is_local: true,
                    is_deleted: false,
                    is_private: false,
                })),
                Capability::ReadWrite,
            )
        })
    }

    pub fn diagnostics_for_path(
        &self,
        path: &Path,
    ) -> Vec<(
        LanguageServerId,
        Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
    )> {
        self.diagnostics.get(path).cloned().unwrap_or_default()
    }

    pub fn clear_diagnostics_for_language_server(
        &mut self,
        server_id: LanguageServerId,
        _: &mut ModelContext<Worktree>,
    ) {
        let worktree_id = self.id().to_proto();
        self.diagnostic_summaries
            .retain(|path, summaries_by_server_id| {
                if summaries_by_server_id.remove(&server_id).is_some() {
                    if let Some(share) = self.share.as_ref() {
                        self.client
                            .send(proto::UpdateDiagnosticSummary {
                                project_id: share.project_id,
                                worktree_id,
                                summary: Some(proto::DiagnosticSummary {
                                    path: path.to_string_lossy().to_string(),
                                    language_server_id: server_id.0 as u64,
                                    error_count: 0,
                                    warning_count: 0,
                                }),
                            })
                            .log_err();
                    }
                    !summaries_by_server_id.is_empty()
                } else {
                    true
                }
            });

        self.diagnostics.retain(|_, diagnostics_by_server_id| {
            if let Ok(ix) = diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
                diagnostics_by_server_id.remove(ix);
                !diagnostics_by_server_id.is_empty()
            } else {
                true
            }
        });
    }

    pub fn update_diagnostics(
        &mut self,
        server_id: LanguageServerId,
        worktree_path: Arc<Path>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        _: &mut ModelContext<Worktree>,
    ) -> Result<bool> {
        let summaries_by_server_id = self
            .diagnostic_summaries
            .entry(worktree_path.clone())
            .or_default();

        let old_summary = summaries_by_server_id
            .remove(&server_id)
            .unwrap_or_default();

        let new_summary = DiagnosticSummary::new(&diagnostics);
        if new_summary.is_empty() {
            if let Some(diagnostics_by_server_id) = self.diagnostics.get_mut(&worktree_path) {
                if let Ok(ix) = diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
                    diagnostics_by_server_id.remove(ix);
                }
                if diagnostics_by_server_id.is_empty() {
                    self.diagnostics.remove(&worktree_path);
                }
            }
        } else {
            summaries_by_server_id.insert(server_id, new_summary);
            let diagnostics_by_server_id =
                self.diagnostics.entry(worktree_path.clone()).or_default();
            match diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
                Ok(ix) => {
                    diagnostics_by_server_id[ix] = (server_id, diagnostics);
                }
                Err(ix) => {
                    diagnostics_by_server_id.insert(ix, (server_id, diagnostics));
                }
            }
        }

        if !old_summary.is_empty() || !new_summary.is_empty() {
            if let Some(share) = self.share.as_ref() {
                self.client
                    .send(proto::UpdateDiagnosticSummary {
                        project_id: share.project_id,
                        worktree_id: self.id().to_proto(),
                        summary: Some(proto::DiagnosticSummary {
                            path: worktree_path.to_string_lossy().to_string(),
                            language_server_id: server_id.0 as u64,
                            error_count: new_summary.error_count as u32,
                            warning_count: new_summary.warning_count as u32,
                        }),
                    })
                    .log_err();
            }
        }

        Ok(!old_summary.is_empty() || !new_summary.is_empty())
    }

    fn set_snapshot(
        &mut self,
        new_snapshot: LocalSnapshot,
        entry_changes: UpdatedEntriesSet,
        cx: &mut ModelContext<Worktree>,
    ) {
        let repo_changes = self.changed_repos(&self.snapshot, &new_snapshot);

        self.snapshot = new_snapshot;

        if let Some(share) = self.share.as_mut() {
            share
                .snapshots_tx
                .unbounded_send((
                    self.snapshot.clone(),
                    entry_changes.clone(),
                    repo_changes.clone(),
                ))
                .ok();
        }

        if !entry_changes.is_empty() {
            cx.emit(Event::UpdatedEntries(entry_changes));
        }
        if !repo_changes.is_empty() {
            cx.emit(Event::UpdatedGitRepositories(repo_changes));
        }
    }

    fn changed_repos(
        &self,
        old_snapshot: &LocalSnapshot,
        new_snapshot: &LocalSnapshot,
    ) -> UpdatedGitRepositoriesSet {
        let mut changes = Vec::new();
        let mut old_repos = old_snapshot.git_repositories.iter().peekable();
        let mut new_repos = new_snapshot.git_repositories.iter().peekable();
        loop {
            match (new_repos.peek().map(clone), old_repos.peek().map(clone)) {
                (Some((new_entry_id, new_repo)), Some((old_entry_id, old_repo))) => {
                    match Ord::cmp(&new_entry_id, &old_entry_id) {
                        Ordering::Less => {
                            if let Some(entry) = new_snapshot.entry_for_id(new_entry_id) {
                                changes.push((
                                    entry.path.clone(),
                                    GitRepositoryChange {
                                        old_repository: None,
                                    },
                                ));
                            }
                            new_repos.next();
                        }
                        Ordering::Equal => {
                            if new_repo.git_dir_scan_id != old_repo.git_dir_scan_id {
                                if let Some(entry) = new_snapshot.entry_for_id(new_entry_id) {
                                    let old_repo = old_snapshot
                                        .repository_entries
                                        .get(&RepositoryWorkDirectory(entry.path.clone()))
                                        .cloned();
                                    changes.push((
                                        entry.path.clone(),
                                        GitRepositoryChange {
                                            old_repository: old_repo,
                                        },
                                    ));
                                }
                            }
                            new_repos.next();
                            old_repos.next();
                        }
                        Ordering::Greater => {
                            if let Some(entry) = old_snapshot.entry_for_id(old_entry_id) {
                                let old_repo = old_snapshot
                                    .repository_entries
                                    .get(&RepositoryWorkDirectory(entry.path.clone()))
                                    .cloned();
                                changes.push((
                                    entry.path.clone(),
                                    GitRepositoryChange {
                                        old_repository: old_repo,
                                    },
                                ));
                            }
                            old_repos.next();
                        }
                    }
                }
                (Some((entry_id, _)), None) => {
                    if let Some(entry) = new_snapshot.entry_for_id(entry_id) {
                        changes.push((
                            entry.path.clone(),
                            GitRepositoryChange {
                                old_repository: None,
                            },
                        ));
                    }
                    new_repos.next();
                }
                (None, Some((entry_id, _))) => {
                    if let Some(entry) = old_snapshot.entry_for_id(entry_id) {
                        let old_repo = old_snapshot
                            .repository_entries
                            .get(&RepositoryWorkDirectory(entry.path.clone()))
                            .cloned();
                        changes.push((
                            entry.path.clone(),
                            GitRepositoryChange {
                                old_repository: old_repo,
                            },
                        ));
                    }
                    old_repos.next();
                }
                (None, None) => break,
            }
        }

        fn clone<T: Clone, U: Clone>(value: &(&T, &U)) -> (T, U) {
            (value.0.clone(), value.1.clone())
        }

        changes.into()
    }

    pub fn scan_complete(&self) -> impl Future<Output = ()> {
        let mut is_scanning_rx = self.is_scanning.1.clone();
        async move {
            let mut is_scanning = *is_scanning_rx.borrow();
            while is_scanning {
                if let Some(value) = is_scanning_rx.recv().await {
                    is_scanning = value;
                } else {
                    break;
                }
            }
        }
    }

    pub fn snapshot(&self) -> LocalSnapshot {
        self.snapshot.clone()
    }

    pub fn metadata_proto(&self) -> proto::WorktreeMetadata {
        proto::WorktreeMetadata {
            id: self.id().to_proto(),
            root_name: self.root_name().to_string(),
            visible: self.visible,
            abs_path: self.abs_path().as_os_str().to_string_lossy().into(),
        }
    }

    fn load(
        &self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<(File, String, Option<String>)>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let entry = self.refresh_entry(path.clone(), None, cx);

        cx.spawn(|this, mut cx| async move {
            let abs_path = abs_path?;
            let text = fs.load(&abs_path).await?;
            let mut index_task = None;
            let snapshot = this.update(&mut cx, |this, _| this.as_local().unwrap().snapshot())?;
            if let Some(repo) = snapshot.repository_for_path(&path) {
                if let Some(repo_path) = repo.work_directory.relativize(&snapshot, &path).log_err()
                {
                    if let Some(git_repo) = snapshot.git_repositories.get(&*repo.work_directory) {
                        let git_repo = git_repo.repo_ptr.clone();
                        index_task = Some(
                            cx.background_executor()
                                .spawn(async move { git_repo.lock().load_index_text(&repo_path) }),
                        );
                    }
                }
            }

            let diff_base = if let Some(index_task) = index_task {
                index_task.await
            } else {
                None
            };

            let worktree = this
                .upgrade()
                .ok_or_else(|| anyhow!("worktree was dropped"))?;
            match entry.await? {
                Some(entry) => Ok((
                    File {
                        entry_id: Some(entry.id),
                        worktree,
                        path: entry.path,
                        mtime: entry.mtime,
                        is_local: true,
                        is_deleted: false,
                        is_private: entry.is_private,
                    },
                    text,
                    diff_base,
                )),
                None => {
                    let metadata = fs
                        .metadata(&abs_path)
                        .await
                        .with_context(|| {
                            format!("Loading metadata for excluded file {abs_path:?}")
                        })?
                        .with_context(|| {
                            format!("Excluded file {abs_path:?} got removed during loading")
                        })?;
                    let is_private = snapshot.is_path_private(path.as_ref());
                    Ok((
                        File {
                            entry_id: None,
                            worktree,
                            path,
                            mtime: Some(metadata.mtime),
                            is_local: true,
                            is_deleted: false,
                            is_private,
                        },
                        text,
                        diff_base,
                    ))
                }
            }
        })
    }

    pub fn save_buffer(
        &self,
        buffer_handle: Model<Buffer>,
        path: Arc<Path>,
        mut has_changed_file: bool,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);

        let rpc = self.client.clone();
        let buffer_id: u64 = buffer.remote_id().into();
        let project_id = self.share.as_ref().map(|share| share.project_id);

        if buffer.file().is_some_and(|file| !file.is_created()) {
            has_changed_file = true;
        }

        let text = buffer.as_rope().clone();
        let fingerprint = text.fingerprint();
        let version = buffer.version();
        let save = self.write_file(path.as_ref(), text, buffer.line_ending(), cx);
        let fs = Arc::clone(&self.fs);
        let abs_path = self.absolutize(&path);
        let is_private = self.snapshot.is_path_private(&path);

        cx.spawn(move |this, mut cx| async move {
            let entry = save.await?;
            let abs_path = abs_path?;
            let this = this.upgrade().context("worktree dropped")?;

            let (entry_id, mtime, path, is_dotenv) = match entry {
                Some(entry) => (Some(entry.id), entry.mtime, entry.path, entry.is_private),
                None => {
                    let metadata = fs
                        .metadata(&abs_path)
                        .await
                        .with_context(|| {
                            format!(
                                "Fetching metadata after saving the excluded buffer {abs_path:?}"
                            )
                        })?
                        .with_context(|| {
                            format!("Excluded buffer {path:?} got removed during saving")
                        })?;
                    (None, Some(metadata.mtime), path, is_private)
                }
            };

            if has_changed_file {
                let new_file = Arc::new(File {
                    entry_id,
                    worktree: this,
                    path,
                    mtime,
                    is_local: true,
                    is_deleted: false,
                    is_private: is_dotenv,
                });

                if let Some(project_id) = project_id {
                    rpc.send(proto::UpdateBufferFile {
                        project_id,
                        buffer_id,
                        file: Some(new_file.to_proto()),
                    })
                    .log_err();
                }

                buffer_handle.update(&mut cx, |buffer, cx| {
                    if has_changed_file {
                        buffer.file_updated(new_file, cx);
                    }
                })?;
            }

            if let Some(project_id) = project_id {
                rpc.send(proto::BufferSaved {
                    project_id,
                    buffer_id,
                    version: serialize_version(&version),
                    mtime: mtime.map(|time| time.into()),
                    fingerprint: serialize_fingerprint(fingerprint),
                })?;
            }

            buffer_handle.update(&mut cx, |buffer, cx| {
                buffer.did_save(version.clone(), fingerprint, mtime, cx);
            })?;

            Ok(())
        })
    }

    /// Find the lowest path in the worktree's datastructures that is an ancestor
    fn lowest_ancestor(&self, path: &Path) -> PathBuf {
        let mut lowest_ancestor = None;
        for path in path.ancestors() {
            if self.entry_for_path(path).is_some() {
                lowest_ancestor = Some(path.to_path_buf());
                break;
            }
        }

        lowest_ancestor.unwrap_or_else(|| PathBuf::from(""))
    }

    pub fn create_entry(
        &self,
        path: impl Into<Arc<Path>>,
        is_dir: bool,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        let path = path.into();
        let lowest_ancestor = self.lowest_ancestor(&path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let write = cx.background_executor().spawn(async move {
            if is_dir {
                fs.create_dir(&abs_path?).await
            } else {
                fs.save(&abs_path?, &Default::default(), Default::default())
                    .await
            }
        });

        cx.spawn(|this, mut cx| async move {
            write.await?;
            let (result, refreshes) = this.update(&mut cx, |this, cx| {
                let mut refreshes = Vec::new();
                let refresh_paths = path.strip_prefix(&lowest_ancestor).unwrap();
                for refresh_path in refresh_paths.ancestors() {
                    if refresh_path == Path::new("") {
                        continue;
                    }
                    let refresh_full_path = lowest_ancestor.join(refresh_path);

                    refreshes.push(this.as_local_mut().unwrap().refresh_entry(
                        refresh_full_path.into(),
                        None,
                        cx,
                    ));
                }
                (
                    this.as_local_mut().unwrap().refresh_entry(path, None, cx),
                    refreshes,
                )
            })?;
            for refresh in refreshes {
                refresh.await.log_err();
            }

            result.await
        })
    }

    pub(crate) fn write_file(
        &self,
        path: impl Into<Arc<Path>>,
        text: Rope,
        line_ending: LineEnding,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        let path: Arc<Path> = path.into();
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let write = cx
            .background_executor()
            .spawn(async move { fs.save(&abs_path?, &text, line_ending).await });

        cx.spawn(|this, mut cx| async move {
            write.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut().unwrap().refresh_entry(path, None, cx)
            })?
            .await
        })
    }

    pub fn delete_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let entry = self.entry_for_id(entry_id)?.clone();
        let abs_path = self.absolutize(&entry.path);
        let fs = self.fs.clone();

        let delete = cx.background_executor().spawn(async move {
            if entry.is_file() {
                fs.remove_file(&abs_path?, Default::default()).await?;
            } else {
                fs.remove_dir(
                    &abs_path?,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: false,
                    },
                )
                .await?;
            }
            anyhow::Ok(entry.path)
        });

        Some(cx.spawn(|this, mut cx| async move {
            let path = delete.await?;
            this.update(&mut cx, |this, _| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entries_for_paths(vec![path])
            })?
            .recv()
            .await;
            Ok(())
        }))
    }

    pub fn rename_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        let old_path = match self.entry_for_id(entry_id) {
            Some(entry) => entry.path.clone(),
            None => return Task::ready(Ok(None)),
        };
        let new_path = new_path.into();
        let abs_old_path = self.absolutize(&old_path);
        let abs_new_path = self.absolutize(&new_path);
        let fs = self.fs.clone();
        let case_sensitive = self.fs_case_sensitive;
        let rename = cx.background_executor().spawn(async move {
            let abs_old_path = abs_old_path?;
            let abs_new_path = abs_new_path?;

            let abs_old_path_lower = abs_old_path.to_str().map(|p| p.to_lowercase());
            let abs_new_path_lower = abs_new_path.to_str().map(|p| p.to_lowercase());

            // If we're on a case-insensitive FS and we're doing a case-only rename (i.e. `foobar` to `FOOBAR`)
            // we want to overwrite, because otherwise we run into a file-already-exists error.
            let overwrite = !case_sensitive
                && abs_old_path != abs_new_path
                && abs_old_path_lower == abs_new_path_lower;

            fs.rename(
                &abs_old_path,
                &abs_new_path,
                fs::RenameOptions {
                    overwrite,
                    ..Default::default()
                },
            )
            .await
        });

        cx.spawn(|this, mut cx| async move {
            rename.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entry(new_path.clone(), Some(old_path), cx)
            })?
            .await
        })
    }

    pub fn copy_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        let old_path = match self.entry_for_id(entry_id) {
            Some(entry) => entry.path.clone(),
            None => return Task::ready(Ok(None)),
        };
        let new_path = new_path.into();
        let abs_old_path = self.absolutize(&old_path);
        let abs_new_path = self.absolutize(&new_path);
        let fs = self.fs.clone();
        let copy = cx.background_executor().spawn(async move {
            copy_recursive(
                fs.as_ref(),
                &abs_old_path?,
                &abs_new_path?,
                Default::default(),
            )
            .await
        });

        cx.spawn(|this, mut cx| async move {
            copy.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entry(new_path.clone(), None, cx)
            })?
            .await
        })
    }

    pub fn expand_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let path = self.entry_for_id(entry_id)?.path.clone();
        let mut refresh = self.refresh_entries_for_paths(vec![path]);
        Some(cx.background_executor().spawn(async move {
            refresh.next().await;
            Ok(())
        }))
    }

    pub fn refresh_entries_for_paths(&self, paths: Vec<Arc<Path>>) -> barrier::Receiver {
        let (tx, rx) = barrier::channel();
        self.scan_requests_tx
            .try_send(ScanRequest {
                relative_paths: paths,
                done: tx,
            })
            .ok();
        rx
    }

    pub fn add_path_prefix_to_scan(&self, path_prefix: Arc<Path>) {
        self.path_prefixes_to_scan_tx.try_send(path_prefix).ok();
    }

    fn refresh_entry(
        &self,
        path: Arc<Path>,
        old_path: Option<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        if self.is_path_excluded(path.to_path_buf()) {
            return Task::ready(Ok(None));
        }
        let paths = if let Some(old_path) = old_path.as_ref() {
            vec![old_path.clone(), path.clone()]
        } else {
            vec![path.clone()]
        };
        let mut refresh = self.refresh_entries_for_paths(paths);
        cx.spawn(move |this, mut cx| async move {
            refresh.recv().await;
            let new_entry = this.update(&mut cx, |this, _| {
                this.entry_for_path(path)
                    .cloned()
                    .ok_or_else(|| anyhow!("failed to read path after update"))
            })??;
            Ok(Some(new_entry))
        })
    }

    pub fn observe_updates<F, Fut>(
        &mut self,
        project_id: u64,
        cx: &mut ModelContext<Worktree>,
        callback: F,
    ) -> oneshot::Receiver<()>
    where
        F: 'static + Send + Fn(proto::UpdateWorktree) -> Fut,
        Fut: Send + Future<Output = bool>,
    {
        #[cfg(any(test, feature = "test-support"))]
        const MAX_CHUNK_SIZE: usize = 2;
        #[cfg(not(any(test, feature = "test-support")))]
        const MAX_CHUNK_SIZE: usize = 256;

        let (share_tx, share_rx) = oneshot::channel();

        if let Some(share) = self.share.as_mut() {
            share_tx.send(()).ok();
            *share.resume_updates.borrow_mut() = ();
            return share_rx;
        }

        let (resume_updates_tx, mut resume_updates_rx) = watch::channel::<()>();
        let (snapshots_tx, mut snapshots_rx) =
            mpsc::unbounded::<(LocalSnapshot, UpdatedEntriesSet, UpdatedGitRepositoriesSet)>();
        snapshots_tx
            .unbounded_send((self.snapshot(), Arc::from([]), Arc::from([])))
            .ok();

        let worktree_id = cx.entity_id().as_u64();
        let _maintain_remote_snapshot = cx.background_executor().spawn(async move {
            let mut is_first = true;
            while let Some((snapshot, entry_changes, repo_changes)) = snapshots_rx.next().await {
                let update;
                if is_first {
                    update = snapshot.build_initial_update(project_id, worktree_id);
                    is_first = false;
                } else {
                    update =
                        snapshot.build_update(project_id, worktree_id, entry_changes, repo_changes);
                }

                for update in proto::split_worktree_update(update, MAX_CHUNK_SIZE) {
                    let _ = resume_updates_rx.try_recv();
                    loop {
                        let result = callback(update.clone());
                        if result.await {
                            break;
                        } else {
                            log::info!("waiting to resume updates");
                            if resume_updates_rx.next().await.is_none() {
                                return Some(());
                            }
                        }
                    }
                }
            }
            share_tx.send(()).ok();
            Some(())
        });

        self.share = Some(ShareState {
            project_id,
            snapshots_tx,
            resume_updates: resume_updates_tx,
            _maintain_remote_snapshot,
        });
        share_rx
    }

    pub fn share(&mut self, project_id: u64, cx: &mut ModelContext<Worktree>) -> Task<Result<()>> {
        let client = self.client.clone();

        for (path, summaries) in &self.diagnostic_summaries {
            for (&server_id, summary) in summaries {
                if let Err(e) = self.client.send(proto::UpdateDiagnosticSummary {
                    project_id,
                    worktree_id: cx.entity_id().as_u64(),
                    summary: Some(summary.to_proto(server_id, path)),
                }) {
                    return Task::ready(Err(e));
                }
            }
        }

        let rx = self.observe_updates(project_id, cx, move |update| {
            client.request(update).map(|result| result.is_ok())
        });
        cx.background_executor()
            .spawn(async move { rx.await.map_err(|_| anyhow!("share ended")) })
    }

    pub fn unshare(&mut self) {
        self.share.take();
    }

    pub fn is_shared(&self) -> bool {
        self.share.is_some()
    }
}

impl RemoteWorktree {
    fn snapshot(&self) -> Snapshot {
        self.snapshot.clone()
    }

    pub fn disconnected_from_host(&mut self) {
        self.updates_tx.take();
        self.snapshot_subscriptions.clear();
        self.disconnected = true;
    }

    pub fn save_buffer(
        &self,
        buffer_handle: Model<Buffer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id().into();
        let version = buffer.version();
        let rpc = self.client.clone();
        let project_id = self.project_id;
        cx.spawn(move |_, mut cx| async move {
            let response = rpc
                .request(proto::SaveBuffer {
                    project_id,
                    buffer_id,
                    version: serialize_version(&version),
                })
                .await?;
            let version = deserialize_version(&response.version);
            let fingerprint = RopeFingerprint::default();
            let mtime = response.mtime.map(|mtime| mtime.into());

            buffer_handle.update(&mut cx, |buffer, cx| {
                buffer.did_save(version.clone(), fingerprint, mtime, cx);
            })?;

            Ok(())
        })
    }

    pub fn update_from_remote(&mut self, update: proto::UpdateWorktree) {
        if let Some(updates_tx) = &self.updates_tx {
            updates_tx
                .unbounded_send(update)
                .expect("consumer runs to completion");
        }
    }

    fn observed_snapshot(&self, scan_id: usize) -> bool {
        self.completed_scan_id >= scan_id
    }

    pub fn wait_for_snapshot(&mut self, scan_id: usize) -> impl Future<Output = Result<()>> {
        let (tx, rx) = oneshot::channel();
        if self.observed_snapshot(scan_id) {
            let _ = tx.send(());
        } else if self.disconnected {
            drop(tx);
        } else {
            match self
                .snapshot_subscriptions
                .binary_search_by_key(&scan_id, |probe| probe.0)
            {
                Ok(ix) | Err(ix) => self.snapshot_subscriptions.insert(ix, (scan_id, tx)),
            }
        }

        async move {
            rx.await?;
            Ok(())
        }
    }

    pub fn update_diagnostic_summary(
        &mut self,
        path: Arc<Path>,
        summary: &proto::DiagnosticSummary,
    ) {
        let server_id = LanguageServerId(summary.language_server_id as usize);
        let summary = DiagnosticSummary {
            error_count: summary.error_count as usize,
            warning_count: summary.warning_count as usize,
        };

        if summary.is_empty() {
            if let Some(summaries) = self.diagnostic_summaries.get_mut(&path) {
                summaries.remove(&server_id);
                if summaries.is_empty() {
                    self.diagnostic_summaries.remove(&path);
                }
            }
        } else {
            self.diagnostic_summaries
                .entry(path)
                .or_default()
                .insert(server_id, summary);
        }
    }

    pub fn insert_entry(
        &mut self,
        entry: proto::Entry,
        scan_id: usize,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let wait_for_snapshot = self.wait_for_snapshot(scan_id);
        cx.spawn(|this, mut cx| async move {
            wait_for_snapshot.await?;
            this.update(&mut cx, |worktree, _| {
                let worktree = worktree.as_remote_mut().unwrap();
                let mut snapshot = worktree.background_snapshot.lock();
                let entry = snapshot.insert_entry(entry);
                worktree.snapshot = snapshot.clone();
                entry
            })?
        })
    }

    pub fn delete_entry(
        &mut self,
        id: ProjectEntryId,
        scan_id: usize,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        let wait_for_snapshot = self.wait_for_snapshot(scan_id);
        cx.spawn(move |this, mut cx| async move {
            wait_for_snapshot.await?;
            this.update(&mut cx, |worktree, _| {
                let worktree = worktree.as_remote_mut().unwrap();
                let mut snapshot = worktree.background_snapshot.lock();
                snapshot.delete_entry(id);
                worktree.snapshot = snapshot.clone();
            })?;
            Ok(())
        })
    }
}

impl Snapshot {
    pub fn id(&self) -> WorktreeId {
        self.id
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub fn absolutize(&self, path: &Path) -> Result<PathBuf> {
        if path
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(anyhow!("invalid path"));
        }
        if path.file_name().is_some() {
            Ok(self.abs_path.join(path))
        } else {
            Ok(self.abs_path.to_path_buf())
        }
    }

    pub fn contains_entry(&self, entry_id: ProjectEntryId) -> bool {
        self.entries_by_id.get(&entry_id, &()).is_some()
    }

    fn insert_entry(&mut self, entry: proto::Entry) -> Result<Entry> {
        let entry = Entry::try_from((&self.root_char_bag, entry))?;
        let old_entry = self.entries_by_id.insert_or_replace(
            PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: 0,
            },
            &(),
        );
        if let Some(old_entry) = old_entry {
            self.entries_by_path.remove(&PathKey(old_entry.path), &());
        }
        self.entries_by_path.insert_or_replace(entry.clone(), &());
        Ok(entry)
    }

    fn delete_entry(&mut self, entry_id: ProjectEntryId) -> Option<Arc<Path>> {
        let removed_entry = self.entries_by_id.remove(&entry_id, &())?;
        self.entries_by_path = {
            let mut cursor = self.entries_by_path.cursor::<TraversalProgress>();
            let mut new_entries_by_path =
                cursor.slice(&TraversalTarget::Path(&removed_entry.path), Bias::Left, &());
            while let Some(entry) = cursor.item() {
                if entry.path.starts_with(&removed_entry.path) {
                    self.entries_by_id.remove(&entry.id, &());
                    cursor.next(&());
                } else {
                    break;
                }
            }
            new_entries_by_path.append(cursor.suffix(&()), &());
            new_entries_by_path
        };

        Some(removed_entry.path)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn status_for_file(&self, path: impl Into<PathBuf>) -> Option<GitFileStatus> {
        let path = path.into();
        self.entries_by_path
            .get(&PathKey(Arc::from(path)), &())
            .and_then(|entry| entry.git_status)
    }

    pub(crate) fn apply_remote_update(&mut self, mut update: proto::UpdateWorktree) -> Result<()> {
        let mut entries_by_path_edits = Vec::new();
        let mut entries_by_id_edits = Vec::new();

        for entry_id in update.removed_entries {
            let entry_id = ProjectEntryId::from_proto(entry_id);
            entries_by_id_edits.push(Edit::Remove(entry_id));
            if let Some(entry) = self.entry_for_id(entry_id) {
                entries_by_path_edits.push(Edit::Remove(PathKey(entry.path.clone())));
            }
        }

        for entry in update.updated_entries {
            let entry = Entry::try_from((&self.root_char_bag, entry))?;
            if let Some(PathEntry { path, .. }) = self.entries_by_id.get(&entry.id, &()) {
                entries_by_path_edits.push(Edit::Remove(PathKey(path.clone())));
            }
            if let Some(old_entry) = self.entries_by_path.get(&PathKey(entry.path.clone()), &()) {
                if old_entry.id != entry.id {
                    entries_by_id_edits.push(Edit::Remove(old_entry.id));
                }
            }
            entries_by_id_edits.push(Edit::Insert(PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: 0,
            }));
            entries_by_path_edits.push(Edit::Insert(entry));
        }

        self.entries_by_path.edit(entries_by_path_edits, &());
        self.entries_by_id.edit(entries_by_id_edits, &());

        update.removed_repositories.sort_unstable();
        self.repository_entries.retain(|_, entry| {
            if let Ok(_) = update
                .removed_repositories
                .binary_search(&entry.work_directory.to_proto())
            {
                false
            } else {
                true
            }
        });

        for repository in update.updated_repositories {
            let work_directory_entry: WorkDirectoryEntry =
                ProjectEntryId::from_proto(repository.work_directory_id).into();

            if let Some(entry) = self.entry_for_id(*work_directory_entry) {
                let work_directory = RepositoryWorkDirectory(entry.path.clone());
                if self.repository_entries.get(&work_directory).is_some() {
                    self.repository_entries.update(&work_directory, |repo| {
                        repo.branch = repository.branch.map(Into::into);
                    });
                } else {
                    self.repository_entries.insert(
                        work_directory,
                        RepositoryEntry {
                            work_directory: work_directory_entry,
                            branch: repository.branch.map(Into::into),
                        },
                    )
                }
            } else {
                log::error!("no work directory entry for repository {:?}", repository)
            }
        }

        self.scan_id = update.scan_id as usize;
        if update.is_last_update {
            self.completed_scan_id = update.scan_id as usize;
        }

        Ok(())
    }

    pub fn file_count(&self) -> usize {
        self.entries_by_path.summary().file_count
    }

    pub fn visible_file_count(&self) -> usize {
        self.entries_by_path.summary().non_ignored_file_count
    }

    fn traverse_from_offset(
        &self,
        include_dirs: bool,
        include_ignored: bool,
        start_offset: usize,
    ) -> Traversal {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(
            &TraversalTarget::Count {
                count: start_offset,
                include_dirs,
                include_ignored,
            },
            Bias::Right,
            &(),
        );
        Traversal {
            cursor,
            include_dirs,
            include_ignored,
        }
    }

    fn traverse_from_path(
        &self,
        include_dirs: bool,
        include_ignored: bool,
        path: &Path,
    ) -> Traversal {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(&TraversalTarget::Path(path), Bias::Left, &());
        Traversal {
            cursor,
            include_dirs,
            include_ignored,
        }
    }

    pub fn files(&self, include_ignored: bool, start: usize) -> Traversal {
        self.traverse_from_offset(false, include_ignored, start)
    }

    pub fn entries(&self, include_ignored: bool) -> Traversal {
        self.traverse_from_offset(true, include_ignored, 0)
    }

    pub fn repositories(&self) -> impl Iterator<Item = (&Arc<Path>, &RepositoryEntry)> {
        self.repository_entries
            .iter()
            .map(|(path, entry)| (&path.0, entry))
    }

    /// Get the repository whose work directory contains the given path.
    pub fn repository_for_work_directory(&self, path: &Path) -> Option<RepositoryEntry> {
        self.repository_entries
            .get(&RepositoryWorkDirectory(path.into()))
            .cloned()
    }

    /// Get the repository whose work directory contains the given path.
    pub fn repository_for_path(&self, path: &Path) -> Option<RepositoryEntry> {
        self.repository_and_work_directory_for_path(path)
            .map(|e| e.1)
    }

    pub fn repository_and_work_directory_for_path(
        &self,
        path: &Path,
    ) -> Option<(RepositoryWorkDirectory, RepositoryEntry)> {
        self.repository_entries
            .iter()
            .filter(|(workdir_path, _)| path.starts_with(workdir_path))
            .last()
            .map(|(path, repo)| (path.clone(), repo.clone()))
    }

    /// Given an ordered iterator of entries, returns an iterator of those entries,
    /// along with their containing git repository.
    pub fn entries_with_repositories<'a>(
        &'a self,
        entries: impl 'a + Iterator<Item = &'a Entry>,
    ) -> impl 'a + Iterator<Item = (&'a Entry, Option<&'a RepositoryEntry>)> {
        let mut containing_repos = Vec::<(&Arc<Path>, &RepositoryEntry)>::new();
        let mut repositories = self.repositories().peekable();
        entries.map(move |entry| {
            while let Some((repo_path, _)) = containing_repos.last() {
                if !entry.path.starts_with(repo_path) {
                    containing_repos.pop();
                } else {
                    break;
                }
            }
            while let Some((repo_path, _)) = repositories.peek() {
                if entry.path.starts_with(repo_path) {
                    containing_repos.push(repositories.next().unwrap());
                } else {
                    break;
                }
            }
            let repo = containing_repos.last().map(|(_, repo)| *repo);
            (entry, repo)
        })
    }

    /// Updates the `git_status` of the given entries such that files'
    /// statuses bubble up to their ancestor directories.
    pub fn propagate_git_statuses(&self, result: &mut [Entry]) {
        let mut cursor = self
            .entries_by_path
            .cursor::<(TraversalProgress, GitStatuses)>();
        let mut entry_stack = Vec::<(usize, GitStatuses)>::new();

        let mut result_ix = 0;
        loop {
            let next_entry = result.get(result_ix);
            let containing_entry = entry_stack.last().map(|(ix, _)| &result[*ix]);

            let entry_to_finish = match (containing_entry, next_entry) {
                (Some(_), None) => entry_stack.pop(),
                (Some(containing_entry), Some(next_path)) => {
                    if !next_path.path.starts_with(&containing_entry.path) {
                        entry_stack.pop()
                    } else {
                        None
                    }
                }
                (None, Some(_)) => None,
                (None, None) => break,
            };

            if let Some((entry_ix, prev_statuses)) = entry_to_finish {
                cursor.seek_forward(
                    &TraversalTarget::PathSuccessor(&result[entry_ix].path),
                    Bias::Left,
                    &(),
                );

                let statuses = cursor.start().1 - prev_statuses;

                result[entry_ix].git_status = if statuses.conflict > 0 {
                    Some(GitFileStatus::Conflict)
                } else if statuses.modified > 0 {
                    Some(GitFileStatus::Modified)
                } else if statuses.added > 0 {
                    Some(GitFileStatus::Added)
                } else {
                    None
                };
            } else {
                if result[result_ix].is_dir() {
                    cursor.seek_forward(
                        &TraversalTarget::Path(&result[result_ix].path),
                        Bias::Left,
                        &(),
                    );
                    entry_stack.push((result_ix, cursor.start().1));
                }
                result_ix += 1;
            }
        }
    }

    pub fn paths(&self) -> impl Iterator<Item = &Arc<Path>> {
        let empty_path = Path::new("");
        self.entries_by_path
            .cursor::<()>()
            .filter(move |entry| entry.path.as_ref() != empty_path)
            .map(|entry| &entry.path)
    }

    fn child_entries<'a>(&'a self, parent_path: &'a Path) -> ChildEntriesIter<'a> {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(&TraversalTarget::Path(parent_path), Bias::Right, &());
        let traversal = Traversal {
            cursor,
            include_dirs: true,
            include_ignored: true,
        };
        ChildEntriesIter {
            traversal,
            parent_path,
        }
    }

    pub fn descendent_entries<'a>(
        &'a self,
        include_dirs: bool,
        include_ignored: bool,
        parent_path: &'a Path,
    ) -> DescendentEntriesIter<'a> {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(&TraversalTarget::Path(parent_path), Bias::Left, &());
        let mut traversal = Traversal {
            cursor,
            include_dirs,
            include_ignored,
        };

        if traversal.end_offset() == traversal.start_offset() {
            traversal.advance();
        }

        DescendentEntriesIter {
            traversal,
            parent_path,
        }
    }

    pub fn root_entry(&self) -> Option<&Entry> {
        self.entry_for_path("")
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn root_git_entry(&self) -> Option<RepositoryEntry> {
        self.repository_entries
            .get(&RepositoryWorkDirectory(Path::new("").into()))
            .map(|entry| entry.to_owned())
    }

    pub fn git_entries(&self) -> impl Iterator<Item = &RepositoryEntry> {
        self.repository_entries.values()
    }

    pub fn scan_id(&self) -> usize {
        self.scan_id
    }

    pub fn entry_for_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let path = path.as_ref();
        self.traverse_from_path(true, true, path)
            .entry()
            .and_then(|entry| {
                if entry.path.as_ref() == path {
                    Some(entry)
                } else {
                    None
                }
            })
    }

    pub fn entry_for_id(&self, id: ProjectEntryId) -> Option<&Entry> {
        let entry = self.entries_by_id.get(&id, &())?;
        self.entry_for_path(&entry.path)
    }

    pub fn inode_for_path(&self, path: impl AsRef<Path>) -> Option<u64> {
        self.entry_for_path(path.as_ref()).map(|e| e.inode)
    }
}

impl LocalSnapshot {
    pub fn get_local_repo(&self, repo: &RepositoryEntry) -> Option<&LocalRepositoryEntry> {
        self.git_repositories.get(&repo.work_directory.0)
    }

    pub(crate) fn local_repo_for_path(
        &self,
        path: &Path,
    ) -> Option<(RepositoryWorkDirectory, &LocalRepositoryEntry)> {
        let (path, repo) = self.repository_and_work_directory_for_path(path)?;
        Some((path, self.git_repositories.get(&repo.work_directory_id())?))
    }

    pub fn local_git_repo(&self, path: &Path) -> Option<Arc<Mutex<dyn GitRepository>>> {
        self.local_repo_for_path(path)
            .map(|(_, entry)| entry.repo_ptr.clone())
    }

    fn build_update(
        &self,
        project_id: u64,
        worktree_id: u64,
        entry_changes: UpdatedEntriesSet,
        repo_changes: UpdatedGitRepositoriesSet,
    ) -> proto::UpdateWorktree {
        let mut updated_entries = Vec::new();
        let mut removed_entries = Vec::new();
        let mut updated_repositories = Vec::new();
        let mut removed_repositories = Vec::new();

        for (_, entry_id, path_change) in entry_changes.iter() {
            if let PathChange::Removed = path_change {
                removed_entries.push(entry_id.0 as u64);
            } else if let Some(entry) = self.entry_for_id(*entry_id) {
                updated_entries.push(proto::Entry::from(entry));
            }
        }

        for (work_dir_path, change) in repo_changes.iter() {
            let new_repo = self
                .repository_entries
                .get(&RepositoryWorkDirectory(work_dir_path.clone()));
            match (&change.old_repository, new_repo) {
                (Some(old_repo), Some(new_repo)) => {
                    updated_repositories.push(new_repo.build_update(old_repo));
                }
                (None, Some(new_repo)) => {
                    updated_repositories.push(proto::RepositoryEntry::from(new_repo));
                }
                (Some(old_repo), None) => {
                    removed_repositories.push(old_repo.work_directory.0.to_proto());
                }
                _ => {}
            }
        }

        removed_entries.sort_unstable();
        updated_entries.sort_unstable_by_key(|e| e.id);
        removed_repositories.sort_unstable();
        updated_repositories.sort_unstable_by_key(|e| e.work_directory_id);

        // TODO - optimize, knowing that removed_entries are sorted.
        removed_entries.retain(|id| updated_entries.binary_search_by_key(id, |e| e.id).is_err());

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            abs_path: self.abs_path().to_string_lossy().into(),
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries,
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
            updated_repositories,
            removed_repositories,
        }
    }

    fn build_initial_update(&self, project_id: u64, worktree_id: u64) -> proto::UpdateWorktree {
        let mut updated_entries = self
            .entries_by_path
            .iter()
            .map(proto::Entry::from)
            .collect::<Vec<_>>();
        updated_entries.sort_unstable_by_key(|e| e.id);

        let mut updated_repositories = self
            .repository_entries
            .values()
            .map(proto::RepositoryEntry::from)
            .collect::<Vec<_>>();
        updated_repositories.sort_unstable_by_key(|e| e.work_directory_id);

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            abs_path: self.abs_path().to_string_lossy().into(),
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries: Vec::new(),
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
            updated_repositories,
            removed_repositories: Vec::new(),
        }
    }

    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        if entry.is_file() && entry.path.file_name() == Some(&GITIGNORE) {
            let abs_path = self.abs_path.join(&entry.path);
            match smol::block_on(build_gitignore(&abs_path, fs)) {
                Ok(ignore) => {
                    self.ignores_by_parent_abs_path
                        .insert(abs_path.parent().unwrap().into(), (Arc::new(ignore), true));
                }
                Err(error) => {
                    log::error!(
                        "error loading .gitignore file {:?} - {:?}",
                        &entry.path,
                        error
                    );
                }
            }
        }

        if entry.kind == EntryKind::PendingDir {
            if let Some(existing_entry) =
                self.entries_by_path.get(&PathKey(entry.path.clone()), &())
            {
                entry.kind = existing_entry.kind;
            }
        }

        let scan_id = self.scan_id;
        let removed = self.entries_by_path.insert_or_replace(entry.clone(), &());
        if let Some(removed) = removed {
            if removed.id != entry.id {
                self.entries_by_id.remove(&removed.id, &());
            }
        }
        self.entries_by_id.insert_or_replace(
            PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id,
            },
            &(),
        );

        entry
    }

    fn ancestor_inodes_for_path(&self, path: &Path) -> TreeSet<u64> {
        let mut inodes = TreeSet::default();
        for ancestor in path.ancestors().skip(1) {
            if let Some(entry) = self.entry_for_path(ancestor) {
                inodes.insert(entry.inode);
            }
        }
        inodes
    }

    fn ignore_stack_for_abs_path(&self, abs_path: &Path, is_dir: bool) -> Arc<IgnoreStack> {
        let mut new_ignores = Vec::new();
        for (index, ancestor) in abs_path.ancestors().enumerate() {
            if index > 0 {
                if let Some((ignore, _)) = self.ignores_by_parent_abs_path.get(ancestor) {
                    new_ignores.push((ancestor, Some(ignore.clone())));
                } else {
                    new_ignores.push((ancestor, None));
                }
            }
            if ancestor.join(&*DOT_GIT).is_dir() {
                break;
            }
        }

        let mut ignore_stack = IgnoreStack::none();
        for (parent_abs_path, ignore) in new_ignores.into_iter().rev() {
            if ignore_stack.is_abs_path_ignored(parent_abs_path, true) {
                ignore_stack = IgnoreStack::all();
                break;
            } else if let Some(ignore) = ignore {
                ignore_stack = ignore_stack.append(parent_abs_path.into(), ignore);
            }
        }

        if ignore_stack.is_abs_path_ignored(abs_path, is_dir) {
            ignore_stack = IgnoreStack::all();
        }

        ignore_stack
    }

    #[cfg(test)]
    pub(crate) fn expanded_entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries_by_path
            .cursor::<()>()
            .filter(|entry| entry.kind == EntryKind::Dir && (entry.is_external || entry.is_ignored))
    }

    #[cfg(test)]
    pub fn check_invariants(&self, git_state: bool) {
        use pretty_assertions::assert_eq;

        assert_eq!(
            self.entries_by_path
                .cursor::<()>()
                .map(|e| (&e.path, e.id))
                .collect::<Vec<_>>(),
            self.entries_by_id
                .cursor::<()>()
                .map(|e| (&e.path, e.id))
                .collect::<collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            "entries_by_path and entries_by_id are inconsistent"
        );

        let mut files = self.files(true, 0);
        let mut visible_files = self.files(false, 0);
        for entry in self.entries_by_path.cursor::<()>() {
            if entry.is_file() {
                assert_eq!(files.next().unwrap().inode, entry.inode);
                if !entry.is_ignored && !entry.is_external {
                    assert_eq!(visible_files.next().unwrap().inode, entry.inode);
                }
            }
        }

        assert!(files.next().is_none());
        assert!(visible_files.next().is_none());

        let mut bfs_paths = Vec::new();
        let mut stack = self
            .root_entry()
            .map(|e| e.path.as_ref())
            .into_iter()
            .collect::<Vec<_>>();
        while let Some(path) = stack.pop() {
            bfs_paths.push(path);
            let ix = stack.len();
            for child_entry in self.child_entries(path) {
                stack.insert(ix, &child_entry.path);
            }
        }

        let dfs_paths_via_iter = self
            .entries_by_path
            .cursor::<()>()
            .map(|e| e.path.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(bfs_paths, dfs_paths_via_iter);

        let dfs_paths_via_traversal = self
            .entries(true)
            .map(|e| e.path.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(dfs_paths_via_traversal, dfs_paths_via_iter);

        if git_state {
            for ignore_parent_abs_path in self.ignores_by_parent_abs_path.keys() {
                let ignore_parent_path =
                    ignore_parent_abs_path.strip_prefix(&self.abs_path).unwrap();
                assert!(self.entry_for_path(&ignore_parent_path).is_some());
                assert!(self
                    .entry_for_path(ignore_parent_path.join(&*GITIGNORE))
                    .is_some());
            }
        }
    }

    #[cfg(test)]
    pub fn entries_without_ids(&self, include_ignored: bool) -> Vec<(&Path, u64, bool)> {
        let mut paths = Vec::new();
        for entry in self.entries_by_path.cursor::<()>() {
            if include_ignored || !entry.is_ignored {
                paths.push((entry.path.as_ref(), entry.inode, entry.is_ignored));
            }
        }
        paths.sort_by(|a, b| a.0.cmp(b.0));
        paths
    }

    pub fn is_path_private(&self, path: &Path) -> bool {
        path.ancestors().any(|ancestor| {
            self.private_files
                .iter()
                .any(|exclude_matcher| exclude_matcher.is_match(&ancestor))
        })
    }

    pub fn is_path_excluded(&self, mut path: PathBuf) -> bool {
        loop {
            if self
                .file_scan_exclusions
                .iter()
                .any(|exclude_matcher| exclude_matcher.is_match(&path))
            {
                return true;
            }
            if !path.pop() {
                return false;
            }
        }
    }
}

impl BackgroundScannerState {
    fn should_scan_directory(&self, entry: &Entry) -> bool {
        (!entry.is_external && !entry.is_ignored)
            || entry.path.file_name() == Some(*DOT_GIT)
            || self.scanned_dirs.contains(&entry.id) // If we've ever scanned it, keep scanning
            || self
                .paths_to_scan
                .iter()
                .any(|p| p.starts_with(&entry.path))
            || self
                .path_prefixes_to_scan
                .iter()
                .any(|p| entry.path.starts_with(p))
    }

    fn enqueue_scan_dir(&self, abs_path: Arc<Path>, entry: &Entry, scan_job_tx: &Sender<ScanJob>) {
        let path = entry.path.clone();
        let ignore_stack = self.snapshot.ignore_stack_for_abs_path(&abs_path, true);
        let mut ancestor_inodes = self.snapshot.ancestor_inodes_for_path(&path);
        let mut containing_repository = None;
        if !ignore_stack.is_abs_path_ignored(&abs_path, true) {
            if let Some((workdir_path, repo)) = self.snapshot.local_repo_for_path(&path) {
                if let Ok(repo_path) = path.strip_prefix(&workdir_path.0) {
                    containing_repository = Some((
                        workdir_path,
                        repo.repo_ptr.clone(),
                        repo.repo_ptr.lock().staged_statuses(repo_path),
                    ));
                }
            }
        }
        if !ancestor_inodes.contains(&entry.inode) {
            ancestor_inodes.insert(entry.inode);
            scan_job_tx
                .try_send(ScanJob {
                    abs_path,
                    path,
                    ignore_stack,
                    scan_queue: scan_job_tx.clone(),
                    ancestor_inodes,
                    is_external: entry.is_external,
                    containing_repository,
                })
                .unwrap();
        }
    }

    fn reuse_entry_id(&mut self, entry: &mut Entry) {
        if let Some(removed_entry_id) = self.removed_entry_ids.remove(&entry.inode) {
            entry.id = removed_entry_id;
        } else if let Some(existing_entry) = self.snapshot.entry_for_path(&entry.path) {
            entry.id = existing_entry.id;
        }
    }

    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        self.reuse_entry_id(&mut entry);
        let entry = self.snapshot.insert_entry(entry, fs);
        if entry.path.file_name() == Some(&DOT_GIT) {
            self.build_git_repository(entry.path.clone(), fs);
        }

        #[cfg(test)]
        self.snapshot.check_invariants(false);

        entry
    }

    fn populate_dir(
        &mut self,
        parent_path: &Arc<Path>,
        entries: impl IntoIterator<Item = Entry>,
        ignore: Option<Arc<Gitignore>>,
    ) {
        let mut parent_entry = if let Some(parent_entry) = self
            .snapshot
            .entries_by_path
            .get(&PathKey(parent_path.clone()), &())
        {
            parent_entry.clone()
        } else {
            log::warn!(
                "populating a directory {:?} that has been removed",
                parent_path
            );
            return;
        };

        match parent_entry.kind {
            EntryKind::PendingDir | EntryKind::UnloadedDir => parent_entry.kind = EntryKind::Dir,
            EntryKind::Dir => {}
            _ => return,
        }

        if let Some(ignore) = ignore {
            let abs_parent_path = self.snapshot.abs_path.join(&parent_path).into();
            self.snapshot
                .ignores_by_parent_abs_path
                .insert(abs_parent_path, (ignore, false));
        }

        let parent_entry_id = parent_entry.id;
        self.scanned_dirs.insert(parent_entry_id);
        let mut entries_by_path_edits = vec![Edit::Insert(parent_entry)];
        let mut entries_by_id_edits = Vec::new();

        for entry in entries {
            entries_by_id_edits.push(Edit::Insert(PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: self.snapshot.scan_id,
            }));
            entries_by_path_edits.push(Edit::Insert(entry));
        }

        self.snapshot
            .entries_by_path
            .edit(entries_by_path_edits, &());
        self.snapshot.entries_by_id.edit(entries_by_id_edits, &());

        if let Err(ix) = self.changed_paths.binary_search(parent_path) {
            self.changed_paths.insert(ix, parent_path.clone());
        }

        #[cfg(test)]
        self.snapshot.check_invariants(false);
    }

    fn remove_path(&mut self, path: &Path) {
        let mut new_entries;
        let removed_entries;
        {
            let mut cursor = self.snapshot.entries_by_path.cursor::<TraversalProgress>();
            new_entries = cursor.slice(&TraversalTarget::Path(path), Bias::Left, &());
            removed_entries = cursor.slice(&TraversalTarget::PathSuccessor(path), Bias::Left, &());
            new_entries.append(cursor.suffix(&()), &());
        }
        self.snapshot.entries_by_path = new_entries;

        let mut entries_by_id_edits = Vec::new();
        for entry in removed_entries.cursor::<()>() {
            let removed_entry_id = self
                .removed_entry_ids
                .entry(entry.inode)
                .or_insert(entry.id);
            *removed_entry_id = cmp::max(*removed_entry_id, entry.id);
            entries_by_id_edits.push(Edit::Remove(entry.id));
        }
        self.snapshot.entries_by_id.edit(entries_by_id_edits, &());

        if path.file_name() == Some(&GITIGNORE) {
            let abs_parent_path = self.snapshot.abs_path.join(path.parent().unwrap());
            if let Some((_, needs_update)) = self
                .snapshot
                .ignores_by_parent_abs_path
                .get_mut(abs_parent_path.as_path())
            {
                *needs_update = true;
            }
        }

        #[cfg(test)]
        self.snapshot.check_invariants(false);
    }

    fn reload_repositories(&mut self, dot_git_dirs_to_reload: &HashSet<PathBuf>, fs: &dyn Fs) {
        let scan_id = self.snapshot.scan_id;

        for dot_git_dir in dot_git_dirs_to_reload {
            // If there is already a repository for this .git directory, reload
            // the status for all of its files.
            let repository = self
                .snapshot
                .git_repositories
                .iter()
                .find_map(|(entry_id, repo)| {
                    (repo.git_dir_path.as_ref() == dot_git_dir).then(|| (*entry_id, repo.clone()))
                });
            match repository {
                None => {
                    self.build_git_repository(Arc::from(dot_git_dir.as_path()), fs);
                }
                Some((entry_id, repository)) => {
                    if repository.git_dir_scan_id == scan_id {
                        continue;
                    }
                    let Some(work_dir) = self
                        .snapshot
                        .entry_for_id(entry_id)
                        .map(|entry| RepositoryWorkDirectory(entry.path.clone()))
                    else {
                        continue;
                    };

                    log::info!("reload git repository {dot_git_dir:?}");
                    let repository = repository.repo_ptr.lock();
                    let branch = repository.branch_name();
                    repository.reload_index();

                    self.snapshot
                        .git_repositories
                        .update(&entry_id, |entry| entry.git_dir_scan_id = scan_id);
                    self.snapshot
                        .snapshot
                        .repository_entries
                        .update(&work_dir, |entry| entry.branch = branch.map(Into::into));

                    self.update_git_statuses(&work_dir, &*repository);
                }
            }
        }

        // Remove any git repositories whose .git entry no longer exists.
        let snapshot = &mut self.snapshot;
        let mut ids_to_preserve = HashSet::default();
        for (&work_directory_id, entry) in snapshot.git_repositories.iter() {
            let exists_in_snapshot = snapshot
                .entry_for_id(work_directory_id)
                .map_or(false, |entry| {
                    snapshot.entry_for_path(entry.path.join(*DOT_GIT)).is_some()
                });
            if exists_in_snapshot {
                ids_to_preserve.insert(work_directory_id);
            } else {
                let git_dir_abs_path = snapshot.abs_path().join(&entry.git_dir_path);
                let git_dir_excluded = snapshot.is_path_excluded(entry.git_dir_path.to_path_buf());
                if git_dir_excluded
                    && !matches!(smol::block_on(fs.metadata(&git_dir_abs_path)), Ok(None))
                {
                    ids_to_preserve.insert(work_directory_id);
                }
            }
        }
        snapshot
            .git_repositories
            .retain(|work_directory_id, _| ids_to_preserve.contains(work_directory_id));
        snapshot
            .repository_entries
            .retain(|_, entry| ids_to_preserve.contains(&entry.work_directory.0));
    }

    fn build_git_repository(
        &mut self,
        dot_git_path: Arc<Path>,
        fs: &dyn Fs,
    ) -> Option<(
        RepositoryWorkDirectory,
        Arc<Mutex<dyn GitRepository>>,
        TreeMap<RepoPath, GitFileStatus>,
    )> {
        let work_dir_path: Arc<Path> = match dot_git_path.parent() {
            Some(parent_dir) => {
                // Guard against repositories inside the repository metadata
                if parent_dir.iter().any(|component| component == *DOT_GIT) {
                    log::info!(
                        "not building git repository for nested `.git` directory, `.git` path in the worktree: {dot_git_path:?}"
                    );
                    return None;
                };
                log::info!(
                    "building git repository, `.git` path in the worktree: {dot_git_path:?}"
                );
                parent_dir.into()
            }
            None => {
                // `dot_git_path.parent().is_none()` means `.git` directory is the opened worktree itself,
                // no files inside that directory are tracked by git, so no need to build the repo around it
                log::info!(
                    "not building git repository for the worktree itself, `.git` path in the worktree: {dot_git_path:?}"
                );
                return None;
            }
        };

        let work_dir_id = self
            .snapshot
            .entry_for_path(work_dir_path.clone())
            .map(|entry| entry.id)?;

        if self.snapshot.git_repositories.get(&work_dir_id).is_some() {
            return None;
        }

        let abs_path = self.snapshot.abs_path.join(&dot_git_path);
        let repository = fs.open_repo(abs_path.as_path())?;
        let work_directory = RepositoryWorkDirectory(work_dir_path.clone());

        let repo_lock = repository.lock();
        self.snapshot.repository_entries.insert(
            work_directory.clone(),
            RepositoryEntry {
                work_directory: work_dir_id.into(),
                branch: repo_lock.branch_name().map(Into::into),
            },
        );

        let staged_statuses = self.update_git_statuses(&work_directory, &*repo_lock);
        drop(repo_lock);

        self.snapshot.git_repositories.insert(
            work_dir_id,
            LocalRepositoryEntry {
                git_dir_scan_id: 0,
                repo_ptr: repository.clone(),
                git_dir_path: dot_git_path.clone(),
            },
        );

        Some((work_directory, repository, staged_statuses))
    }

    fn update_git_statuses(
        &mut self,
        work_directory: &RepositoryWorkDirectory,
        repo: &dyn GitRepository,
    ) -> TreeMap<RepoPath, GitFileStatus> {
        let staged_statuses = repo.staged_statuses(Path::new(""));

        let mut changes = vec![];
        let mut edits = vec![];

        for mut entry in self
            .snapshot
            .descendent_entries(false, false, &work_directory.0)
            .cloned()
        {
            let Ok(repo_path) = entry.path.strip_prefix(&work_directory.0) else {
                continue;
            };
            let Some(mtime) = entry.mtime else {
                continue;
            };
            let repo_path = RepoPath(repo_path.to_path_buf());
            let git_file_status = combine_git_statuses(
                staged_statuses.get(&repo_path).copied(),
                repo.unstaged_status(&repo_path, mtime),
            );
            if entry.git_status != git_file_status {
                entry.git_status = git_file_status;
                changes.push(entry.path.clone());
                edits.push(Edit::Insert(entry));
            }
        }

        self.snapshot.entries_by_path.edit(edits, &());
        util::extend_sorted(&mut self.changed_paths, changes, usize::MAX, Ord::cmp);
        staged_statuses
    }
}

async fn build_gitignore(abs_path: &Path, fs: &dyn Fs) -> Result<Gitignore> {
    let contents = fs.load(abs_path).await?;
    let parent = abs_path.parent().unwrap_or_else(|| Path::new("/"));
    let mut builder = GitignoreBuilder::new(parent);
    for line in contents.lines() {
        builder.add_line(Some(abs_path.into()), line)?;
    }
    Ok(builder.build()?)
}

impl WorktreeId {
    pub fn from_usize(handle_id: usize) -> Self {
        Self(handle_id)
    }

    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl fmt::Display for WorktreeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Worktree {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        match self {
            Worktree::Local(worktree) => &worktree.snapshot,
            Worktree::Remote(worktree) => &worktree.snapshot,
        }
    }
}

impl Deref for LocalWorktree {
    type Target = LocalSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl Deref for RemoteWorktree {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl fmt::Debug for LocalWorktree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.snapshot.fmt(f)
    }
}

impl fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct EntriesById<'a>(&'a SumTree<PathEntry>);
        struct EntriesByPath<'a>(&'a SumTree<Entry>);

        impl<'a> fmt::Debug for EntriesByPath<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_map()
                    .entries(self.0.iter().map(|entry| (&entry.path, entry.id)))
                    .finish()
            }
        }

        impl<'a> fmt::Debug for EntriesById<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        f.debug_struct("Snapshot")
            .field("id", &self.id)
            .field("root_name", &self.root_name)
            .field("entries_by_path", &EntriesByPath(&self.entries_by_path))
            .field("entries_by_id", &EntriesById(&self.entries_by_id))
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct File {
    pub worktree: Model<Worktree>,
    pub path: Arc<Path>,
    pub mtime: Option<SystemTime>,
    pub entry_id: Option<ProjectEntryId>,
    pub is_local: bool,
    pub is_deleted: bool,
    pub is_private: bool,
}

impl language::File for File {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        if self.is_local {
            Some(self)
        } else {
            None
        }
    }

    fn mtime(&self) -> Option<SystemTime> {
        self.mtime
    }

    fn path(&self) -> &Arc<Path> {
        &self.path
    }

    fn full_path(&self, cx: &AppContext) -> PathBuf {
        let mut full_path = PathBuf::new();
        let worktree = self.worktree.read(cx);

        if worktree.is_visible() {
            full_path.push(worktree.root_name());
        } else {
            let path = worktree.abs_path();

            if worktree.is_local() && path.starts_with(HOME.as_path()) {
                full_path.push("~");
                full_path.push(path.strip_prefix(HOME.as_path()).unwrap());
            } else {
                full_path.push(path)
            }
        }

        if self.path.components().next().is_some() {
            full_path.push(&self.path);
        }

        full_path
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self, cx: &'a AppContext) -> &'a OsStr {
        self.path
            .file_name()
            .unwrap_or_else(|| OsStr::new(&self.worktree.read(cx).root_name))
    }

    fn worktree_id(&self) -> usize {
        self.worktree.entity_id().as_u64() as usize
    }

    fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_proto(&self) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: self.worktree.entity_id().as_u64(),
            entry_id: self.entry_id.map(|id| id.to_proto()),
            path: self.path.to_string_lossy().into(),
            mtime: self.mtime.map(|time| time.into()),
            is_deleted: self.is_deleted,
        }
    }

    fn is_private(&self) -> bool {
        self.is_private
    }
}

impl language::LocalFile for File {
    fn abs_path(&self, cx: &AppContext) -> PathBuf {
        let worktree_path = &self.worktree.read(cx).as_local().unwrap().abs_path;
        if self.path.as_ref() == Path::new("") {
            worktree_path.to_path_buf()
        } else {
            worktree_path.join(&self.path)
        }
    }

    fn load(&self, cx: &AppContext) -> Task<Result<String>> {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        cx.background_executor()
            .spawn(async move { fs.load(&abs_path?).await })
    }

    fn buffer_reloaded(
        &self,
        buffer_id: BufferId,
        version: &clock::Global,
        fingerprint: RopeFingerprint,
        line_ending: LineEnding,
        mtime: Option<SystemTime>,
        cx: &mut AppContext,
    ) {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        if let Some(project_id) = worktree.share.as_ref().map(|share| share.project_id) {
            worktree
                .client
                .send(proto::BufferReloaded {
                    project_id,
                    buffer_id: buffer_id.into(),
                    version: serialize_version(version),
                    mtime: mtime.map(|time| time.into()),
                    fingerprint: serialize_fingerprint(fingerprint),
                    line_ending: serialize_line_ending(line_ending) as i32,
                })
                .log_err();
        }
    }
}

impl File {
    pub fn for_entry(entry: Entry, worktree: Model<Worktree>) -> Arc<Self> {
        Arc::new(Self {
            worktree,
            path: entry.path.clone(),
            mtime: entry.mtime,
            entry_id: Some(entry.id),
            is_local: true,
            is_deleted: false,
            is_private: entry.is_private,
        })
    }

    pub fn from_proto(
        proto: rpc::proto::File,
        worktree: Model<Worktree>,
        cx: &AppContext,
    ) -> Result<Self> {
        let worktree_id = worktree
            .read(cx)
            .as_remote()
            .ok_or_else(|| anyhow!("not remote"))?
            .id();

        if worktree_id.to_proto() != proto.worktree_id {
            return Err(anyhow!("worktree id does not match file"));
        }

        Ok(Self {
            worktree,
            path: Path::new(&proto.path).into(),
            mtime: proto.mtime.map(|time| time.into()),
            entry_id: proto.entry_id.map(ProjectEntryId::from_proto),
            is_local: false,
            is_deleted: proto.is_deleted,
            is_private: false,
        })
    }

    pub fn from_dyn(file: Option<&Arc<dyn language::File>>) -> Option<&Self> {
        file.and_then(|f| f.as_any().downcast_ref())
    }

    pub fn worktree_id(&self, cx: &AppContext) -> WorktreeId {
        self.worktree.read(cx).id()
    }

    pub fn project_entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        if self.is_deleted {
            None
        } else {
            self.entry_id
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: ProjectEntryId,
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub inode: u64,
    pub mtime: Option<SystemTime>,
    pub is_symlink: bool,

    /// Whether this entry is ignored by Git.
    ///
    /// We only scan ignored entries once the directory is expanded and
    /// exclude them from searches.
    pub is_ignored: bool,

    /// Whether this entry's canonical path is outside of the worktree.
    /// This means the entry is only accessible from the worktree root via a
    /// symlink.
    ///
    /// We only scan entries outside of the worktree once the symlinked
    /// directory is expanded. External entries are treated like gitignored
    /// entries in that they are not included in searches.
    pub is_external: bool,
    pub git_status: Option<GitFileStatus>,
    /// Whether this entry is considered to be a `.env` file.
    pub is_private: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    UnloadedDir,
    PendingDir,
    Dir,
    File(CharBag),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathChange {
    /// A filesystem entry was was created.
    Added,
    /// A filesystem entry was removed.
    Removed,
    /// A filesystem entry was updated.
    Updated,
    /// A filesystem entry was either updated or added. We don't know
    /// whether or not it already existed, because the path had not
    /// been loaded before the event.
    AddedOrUpdated,
    /// A filesystem entry was found during the initial scan of the worktree.
    Loaded,
}

pub struct GitRepositoryChange {
    /// The previous state of the repository, if it already existed.
    pub old_repository: Option<RepositoryEntry>,
}

pub type UpdatedEntriesSet = Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>;
pub type UpdatedGitRepositoriesSet = Arc<[(Arc<Path>, GitRepositoryChange)]>;

impl Entry {
    fn new(
        path: Arc<Path>,
        metadata: &fs::Metadata,
        next_entry_id: &AtomicUsize,
        root_char_bag: CharBag,
    ) -> Self {
        Self {
            id: ProjectEntryId::new(next_entry_id),
            kind: if metadata.is_dir {
                EntryKind::PendingDir
            } else {
                EntryKind::File(char_bag_for_path(root_char_bag, &path))
            },
            path,
            inode: metadata.inode,
            mtime: Some(metadata.mtime),
            is_symlink: metadata.is_symlink,
            is_ignored: false,
            is_external: false,
            is_private: false,
            git_status: None,
        }
    }

    pub fn is_created(&self) -> bool {
        self.mtime.is_some()
    }

    pub fn is_dir(&self) -> bool {
        self.kind.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.kind.is_file()
    }

    pub fn git_status(&self) -> Option<GitFileStatus> {
        self.git_status
    }
}

impl EntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(
            self,
            EntryKind::Dir | EntryKind::PendingDir | EntryKind::UnloadedDir
        )
    }

    pub fn is_unloaded(&self) -> bool {
        matches!(self, EntryKind::UnloadedDir)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, EntryKind::File(_))
    }
}

impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self) -> Self::Summary {
        let non_ignored_count = if self.is_ignored || self.is_external {
            0
        } else {
            1
        };
        let file_count;
        let non_ignored_file_count;
        if self.is_file() {
            file_count = 1;
            non_ignored_file_count = non_ignored_count;
        } else {
            file_count = 0;
            non_ignored_file_count = 0;
        }

        let mut statuses = GitStatuses::default();
        match self.git_status {
            Some(status) => match status {
                GitFileStatus::Added => statuses.added = 1,
                GitFileStatus::Modified => statuses.modified = 1,
                GitFileStatus::Conflict => statuses.conflict = 1,
            },
            None => {}
        }

        EntrySummary {
            max_path: self.path.clone(),
            count: 1,
            non_ignored_count,
            file_count,
            non_ignored_file_count,
            statuses,
        }
    }
}

impl sum_tree::KeyedItem for Entry {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        PathKey(self.path.clone())
    }
}

#[derive(Clone, Debug)]
pub struct EntrySummary {
    max_path: Arc<Path>,
    count: usize,
    non_ignored_count: usize,
    file_count: usize,
    non_ignored_file_count: usize,
    statuses: GitStatuses,
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(Path::new("")),
            count: 0,
            non_ignored_count: 0,
            file_count: 0,
            non_ignored_file_count: 0,
            statuses: Default::default(),
        }
    }
}

impl sum_tree::Summary for EntrySummary {
    type Context = ();

    fn add_summary(&mut self, rhs: &Self, _: &()) {
        self.max_path = rhs.max_path.clone();
        self.count += rhs.count;
        self.non_ignored_count += rhs.non_ignored_count;
        self.file_count += rhs.file_count;
        self.non_ignored_file_count += rhs.non_ignored_file_count;
        self.statuses += rhs.statuses;
    }
}

#[derive(Clone, Debug)]
struct PathEntry {
    id: ProjectEntryId,
    path: Arc<Path>,
    is_ignored: bool,
    scan_id: usize,
}

impl sum_tree::Item for PathEntry {
    type Summary = PathEntrySummary;

    fn summary(&self) -> Self::Summary {
        PathEntrySummary { max_id: self.id }
    }
}

impl sum_tree::KeyedItem for PathEntry {
    type Key = ProjectEntryId;

    fn key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Clone, Debug, Default)]
struct PathEntrySummary {
    max_id: ProjectEntryId,
}

impl sum_tree::Summary for PathEntrySummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.max_id = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, PathEntrySummary> for ProjectEntryId {
    fn add_summary(&mut self, summary: &'a PathEntrySummary, _: &()) {
        *self = summary.max_id;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PathKey(Arc<Path>);

impl Default for PathKey {
    fn default() -> Self {
        Self(Path::new("").into())
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for PathKey {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.0 = summary.max_path.clone();
    }
}

struct BackgroundScanner {
    state: Mutex<BackgroundScannerState>,
    fs: Arc<dyn Fs>,
    fs_case_sensitive: bool,
    status_updates_tx: UnboundedSender<ScanState>,
    executor: BackgroundExecutor,
    scan_requests_rx: channel::Receiver<ScanRequest>,
    path_prefixes_to_scan_rx: channel::Receiver<Arc<Path>>,
    next_entry_id: Arc<AtomicUsize>,
    phase: BackgroundScannerPhase,
}

#[derive(PartialEq)]
enum BackgroundScannerPhase {
    InitialScan,
    EventsReceivedDuringInitialScan,
    Events,
}

impl BackgroundScanner {
    #[allow(clippy::too_many_arguments)]
    fn new(
        snapshot: LocalSnapshot,
        next_entry_id: Arc<AtomicUsize>,
        fs: Arc<dyn Fs>,
        fs_case_sensitive: bool,
        status_updates_tx: UnboundedSender<ScanState>,
        executor: BackgroundExecutor,
        scan_requests_rx: channel::Receiver<ScanRequest>,
        path_prefixes_to_scan_rx: channel::Receiver<Arc<Path>>,
    ) -> Self {
        Self {
            fs,
            fs_case_sensitive,
            status_updates_tx,
            executor,
            scan_requests_rx,
            path_prefixes_to_scan_rx,
            next_entry_id,
            state: Mutex::new(BackgroundScannerState {
                prev_snapshot: snapshot.snapshot.clone(),
                snapshot,
                scanned_dirs: Default::default(),
                path_prefixes_to_scan: Default::default(),
                paths_to_scan: Default::default(),
                removed_entry_ids: Default::default(),
                changed_paths: Default::default(),
            }),
            phase: BackgroundScannerPhase::InitialScan,
        }
    }

    async fn run(&mut self, mut fs_events_rx: Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>>) {
        use futures::FutureExt as _;

        // Populate ignores above the root.
        let root_abs_path = self.state.lock().snapshot.abs_path.clone();
        for (index, ancestor) in root_abs_path.ancestors().enumerate() {
            if index != 0 {
                if let Ok(ignore) =
                    build_gitignore(&ancestor.join(&*GITIGNORE), self.fs.as_ref()).await
                {
                    self.state
                        .lock()
                        .snapshot
                        .ignores_by_parent_abs_path
                        .insert(ancestor.into(), (ignore.into(), false));
                }
            }
            if ancestor.join(&*DOT_GIT).is_dir() {
                // Reached root of git repository.
                break;
            }
        }

        let (scan_job_tx, scan_job_rx) = channel::unbounded();
        {
            let mut state = self.state.lock();
            state.snapshot.scan_id += 1;
            if let Some(mut root_entry) = state.snapshot.root_entry().cloned() {
                let ignore_stack = state
                    .snapshot
                    .ignore_stack_for_abs_path(&root_abs_path, true);
                if ignore_stack.is_abs_path_ignored(&root_abs_path, true) {
                    root_entry.is_ignored = true;
                    state.insert_entry(root_entry.clone(), self.fs.as_ref());
                }
                state.enqueue_scan_dir(root_abs_path, &root_entry, &scan_job_tx);
            }
        };

        // Perform an initial scan of the directory.
        drop(scan_job_tx);
        self.scan_dirs(true, scan_job_rx).await;
        {
            let mut state = self.state.lock();
            state.snapshot.completed_scan_id = state.snapshot.scan_id;
        }

        self.send_status_update(false, None);

        // Process any any FS events that occurred while performing the initial scan.
        // For these events, update events cannot be as precise, because we didn't
        // have the previous state loaded yet.
        self.phase = BackgroundScannerPhase::EventsReceivedDuringInitialScan;
        if let Poll::Ready(Some(mut paths)) = futures::poll!(fs_events_rx.next()) {
            while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_events_rx.next()) {
                paths.extend(more_paths);
            }
            self.process_events(paths).await;
        }

        // Continue processing events until the worktree is dropped.
        self.phase = BackgroundScannerPhase::Events;
        loop {
            select_biased! {
                // Process any path refresh requests from the worktree. Prioritize
                // these before handling changes reported by the filesystem.
                request = self.scan_requests_rx.recv().fuse() => {
                    let Ok(request) = request else { break };
                    if !self.process_scan_request(request, false).await {
                        return;
                    }
                }

                path_prefix = self.path_prefixes_to_scan_rx.recv().fuse() => {
                    let Ok(path_prefix) = path_prefix else { break };
                    log::trace!("adding path prefix {:?}", path_prefix);

                    let did_scan = self.forcibly_load_paths(&[path_prefix.clone()]).await;
                    if did_scan {
                        let abs_path =
                        {
                            let mut state = self.state.lock();
                            state.path_prefixes_to_scan.insert(path_prefix.clone());
                            state.snapshot.abs_path.join(&path_prefix)
                        };

                        if let Some(abs_path) = self.fs.canonicalize(&abs_path).await.log_err() {
                            self.process_events(vec![abs_path]).await;
                        }
                    }
                }

                paths = fs_events_rx.next().fuse() => {
                    let Some(mut paths) = paths else { break };
                    while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_events_rx.next()) {
                        paths.extend(more_paths);
                    }
                    self.process_events(paths.clone()).await;
                }
            }
        }
    }

    async fn process_scan_request(&self, mut request: ScanRequest, scanning: bool) -> bool {
        log::debug!("rescanning paths {:?}", request.relative_paths);

        request.relative_paths.sort_unstable();
        self.forcibly_load_paths(&request.relative_paths).await;

        let root_path = self.state.lock().snapshot.abs_path.clone();
        let root_canonical_path = match self.fs.canonicalize(&root_path).await {
            Ok(path) => path,
            Err(err) => {
                log::error!("failed to canonicalize root path: {}", err);
                return true;
            }
        };
        let abs_paths = request
            .relative_paths
            .iter()
            .map(|path| {
                if path.file_name().is_some() {
                    root_canonical_path.join(path)
                } else {
                    root_canonical_path.clone()
                }
            })
            .collect::<Vec<_>>();

        self.reload_entries_for_paths(
            root_path,
            root_canonical_path,
            &request.relative_paths,
            abs_paths,
            None,
        )
        .await;
        self.send_status_update(scanning, Some(request.done))
    }

    async fn process_events(&mut self, mut abs_paths: Vec<PathBuf>) {
        let root_path = self.state.lock().snapshot.abs_path.clone();
        let root_canonical_path = match self.fs.canonicalize(&root_path).await {
            Ok(path) => path,
            Err(err) => {
                log::error!("failed to canonicalize root path: {}", err);
                return;
            }
        };

        let mut relative_paths = Vec::with_capacity(abs_paths.len());
        let mut dot_git_paths_to_reload = HashSet::default();
        abs_paths.sort_unstable();
        abs_paths.dedup_by(|a, b| a.starts_with(&b));
        abs_paths.retain(|abs_path| {
            let snapshot = &self.state.lock().snapshot;
            {
                let mut is_git_related = false;
                if let Some(dot_git_dir) = abs_path
                    .ancestors()
                    .find(|ancestor| ancestor.file_name() == Some(*DOT_GIT))
                {
                    let dot_git_path = dot_git_dir
                        .strip_prefix(&root_canonical_path)
                        .ok()
                        .map(|path| path.to_path_buf())
                        .unwrap_or_else(|| dot_git_dir.to_path_buf());
                    dot_git_paths_to_reload.insert(dot_git_path.to_path_buf());
                    is_git_related = true;
                }

                let relative_path: Arc<Path> =
                    if let Ok(path) = abs_path.strip_prefix(&root_canonical_path) {
                        path.into()
                    } else {
                        log::error!(
                        "ignoring event {abs_path:?} outside of root path {root_canonical_path:?}",
                    );
                        return false;
                    };

                let parent_dir_is_loaded = relative_path.parent().map_or(true, |parent| {
                    snapshot
                        .entry_for_path(parent)
                        .map_or(false, |entry| entry.kind == EntryKind::Dir)
                });
                if !parent_dir_is_loaded {
                    log::debug!("ignoring event {relative_path:?} within unloaded directory");
                    return false;
                }

                if snapshot.is_path_excluded(relative_path.to_path_buf()) {
                    if !is_git_related {
                        log::debug!("ignoring FS event for excluded path {relative_path:?}");
                    }
                    return false;
                }

                relative_paths.push(relative_path);
                true
            }
        });

        if dot_git_paths_to_reload.is_empty() && relative_paths.is_empty() {
            return;
        }

        if !relative_paths.is_empty() {
            log::debug!("received fs events {:?}", relative_paths);

            let (scan_job_tx, scan_job_rx) = channel::unbounded();
            self.reload_entries_for_paths(
                root_path,
                root_canonical_path,
                &relative_paths,
                abs_paths,
                Some(scan_job_tx.clone()),
            )
            .await;
            drop(scan_job_tx);
            self.scan_dirs(false, scan_job_rx).await;

            let (scan_job_tx, scan_job_rx) = channel::unbounded();
            self.update_ignore_statuses(scan_job_tx).await;
            self.scan_dirs(false, scan_job_rx).await;
        }

        {
            let mut state = self.state.lock();
            if !dot_git_paths_to_reload.is_empty() {
                if relative_paths.is_empty() {
                    state.snapshot.scan_id += 1;
                }
                log::debug!("reloading repositories: {dot_git_paths_to_reload:?}");
                state.reload_repositories(&dot_git_paths_to_reload, self.fs.as_ref());
            }
            state.snapshot.completed_scan_id = state.snapshot.scan_id;
            for (_, entry_id) in mem::take(&mut state.removed_entry_ids) {
                state.scanned_dirs.remove(&entry_id);
            }
        }

        self.send_status_update(false, None);
    }

    async fn forcibly_load_paths(&self, paths: &[Arc<Path>]) -> bool {
        let (scan_job_tx, mut scan_job_rx) = channel::unbounded();
        {
            let mut state = self.state.lock();
            let root_path = state.snapshot.abs_path.clone();
            for path in paths {
                for ancestor in path.ancestors() {
                    if let Some(entry) = state.snapshot.entry_for_path(ancestor) {
                        if entry.kind == EntryKind::UnloadedDir {
                            let abs_path = root_path.join(ancestor);
                            state.enqueue_scan_dir(abs_path.into(), entry, &scan_job_tx);
                            state.paths_to_scan.insert(path.clone());
                            break;
                        }
                    }
                }
            }
            drop(scan_job_tx);
        }
        while let Some(job) = scan_job_rx.next().await {
            self.scan_dir(&job).await.log_err();
        }

        mem::take(&mut self.state.lock().paths_to_scan).len() > 0
    }

    async fn scan_dirs(
        &self,
        enable_progress_updates: bool,
        scan_jobs_rx: channel::Receiver<ScanJob>,
    ) {
        use futures::FutureExt as _;

        if self
            .status_updates_tx
            .unbounded_send(ScanState::Started)
            .is_err()
        {
            return;
        }

        let progress_update_count = AtomicUsize::new(0);
        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        let mut last_progress_update_count = 0;
                        let progress_update_timer = self.progress_timer(enable_progress_updates).fuse();
                        futures::pin_mut!(progress_update_timer);

                        loop {
                            select_biased! {
                                // Process any path refresh requests before moving on to process
                                // the scan queue, so that user operations are prioritized.
                                request = self.scan_requests_rx.recv().fuse() => {
                                    let Ok(request) = request else { break };
                                    if !self.process_scan_request(request, true).await {
                                        return;
                                    }
                                }

                                // Send periodic progress updates to the worktree. Use an atomic counter
                                // to ensure that only one of the workers sends a progress update after
                                // the update interval elapses.
                                _ = progress_update_timer => {
                                    match progress_update_count.compare_exchange(
                                        last_progress_update_count,
                                        last_progress_update_count + 1,
                                        SeqCst,
                                        SeqCst
                                    ) {
                                        Ok(_) => {
                                            last_progress_update_count += 1;
                                            self.send_status_update(true, None);
                                        }
                                        Err(count) => {
                                            last_progress_update_count = count;
                                        }
                                    }
                                    progress_update_timer.set(self.progress_timer(enable_progress_updates).fuse());
                                }

                                // Recursively load directories from the file system.
                                job = scan_jobs_rx.recv().fuse() => {
                                    let Ok(job) = job else { break };
                                    if let Err(err) = self.scan_dir(&job).await {
                                        if job.path.as_ref() != Path::new("") {
                                            log::error!("error scanning directory {:?}: {}", job.abs_path, err);
                                        }
                                    }
                                }
                            }
                        }
                    })
                }
            })
            .await;
    }

    fn send_status_update(&self, scanning: bool, barrier: Option<barrier::Sender>) -> bool {
        let mut state = self.state.lock();
        if state.changed_paths.is_empty() && scanning {
            return true;
        }

        let new_snapshot = state.snapshot.clone();
        let old_snapshot = mem::replace(&mut state.prev_snapshot, new_snapshot.snapshot.clone());
        let changes = self.build_change_set(&old_snapshot, &new_snapshot, &state.changed_paths);
        state.changed_paths.clear();

        self.status_updates_tx
            .unbounded_send(ScanState::Updated {
                snapshot: new_snapshot,
                changes,
                scanning,
                barrier,
            })
            .is_ok()
    }

    async fn scan_dir(&self, job: &ScanJob) -> Result<()> {
        let root_abs_path;
        let mut ignore_stack;
        let mut new_ignore;
        let root_char_bag;
        let next_entry_id;
        {
            let state = self.state.lock();
            let snapshot = &state.snapshot;
            root_abs_path = snapshot.abs_path().clone();
            if snapshot.is_path_excluded(job.path.to_path_buf()) {
                log::error!("skipping excluded directory {:?}", job.path);
                return Ok(());
            }
            log::debug!("scanning directory {:?}", job.path);
            ignore_stack = job.ignore_stack.clone();
            new_ignore = None;
            root_char_bag = snapshot.root_char_bag;
            next_entry_id = self.next_entry_id.clone();
            drop(state);
        }

        let mut dotgit_path = None;
        let mut root_canonical_path = None;
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();
        let mut child_paths = self.fs.read_dir(&job.abs_path).await?;
        while let Some(child_abs_path) = child_paths.next().await {
            let child_abs_path: Arc<Path> = match child_abs_path {
                Ok(child_abs_path) => child_abs_path.into(),
                Err(error) => {
                    log::error!("error processing entry {:?}", error);
                    continue;
                }
            };
            let child_name = child_abs_path.file_name().unwrap();
            let child_path: Arc<Path> = job.path.join(child_name).into();
            // If we find a .gitignore, add it to the stack of ignores used to determine which paths are ignored
            if child_name == *GITIGNORE {
                match build_gitignore(&child_abs_path, self.fs.as_ref()).await {
                    Ok(ignore) => {
                        let ignore = Arc::new(ignore);
                        ignore_stack = ignore_stack.append(job.abs_path.clone(), ignore.clone());
                        new_ignore = Some(ignore);
                    }
                    Err(error) => {
                        log::error!(
                            "error loading .gitignore file {:?} - {:?}",
                            child_name,
                            error
                        );
                    }
                }

                // Update ignore status of any child entries we've already processed to reflect the
                // ignore file in the current directory. Because `.gitignore` starts with a `.`,
                // there should rarely be too numerous. Update the ignore stack associated with any
                // new jobs as well.
                let mut new_jobs = new_jobs.iter_mut();
                for entry in &mut new_entries {
                    let entry_abs_path = root_abs_path.join(&entry.path);
                    entry.is_ignored =
                        ignore_stack.is_abs_path_ignored(&entry_abs_path, entry.is_dir());

                    if entry.is_dir() {
                        if let Some(job) = new_jobs.next().expect("missing scan job for entry") {
                            job.ignore_stack = if entry.is_ignored {
                                IgnoreStack::all()
                            } else {
                                ignore_stack.clone()
                            };
                        }
                    }
                }
            }
            // If we find a .git, we'll need to load the repository.
            else if child_name == *DOT_GIT {
                dotgit_path = Some(child_path.clone());
            }

            {
                let relative_path = job.path.join(child_name);
                let mut state = self.state.lock();
                if state.snapshot.is_path_excluded(relative_path.clone()) {
                    log::debug!("skipping excluded child entry {relative_path:?}");
                    state.remove_path(&relative_path);
                    continue;
                }
                drop(state);
            }

            let child_metadata = match self.fs.metadata(&child_abs_path).await {
                Ok(Some(metadata)) => metadata,
                Ok(None) => continue,
                Err(err) => {
                    log::error!("error processing {child_abs_path:?}: {err:?}");
                    continue;
                }
            };

            let mut child_entry = Entry::new(
                child_path.clone(),
                &child_metadata,
                &next_entry_id,
                root_char_bag,
            );

            if job.is_external {
                child_entry.is_external = true;
            } else if child_metadata.is_symlink {
                let canonical_path = match self.fs.canonicalize(&child_abs_path).await {
                    Ok(path) => path,
                    Err(err) => {
                        log::error!(
                            "error reading target of symlink {:?}: {:?}",
                            child_abs_path,
                            err
                        );
                        continue;
                    }
                };

                // lazily canonicalize the root path in order to determine if
                // symlinks point outside of the worktree.
                let root_canonical_path = match &root_canonical_path {
                    Some(path) => path,
                    None => match self.fs.canonicalize(&root_abs_path).await {
                        Ok(path) => root_canonical_path.insert(path),
                        Err(err) => {
                            log::error!("error canonicalizing root {:?}: {:?}", root_abs_path, err);
                            continue;
                        }
                    },
                };

                if !canonical_path.starts_with(root_canonical_path) {
                    child_entry.is_external = true;
                }
            }

            if child_entry.is_dir() {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, true);

                // Avoid recursing until crash in the case of a recursive symlink
                if !job.ancestor_inodes.contains(&child_entry.inode) {
                    let mut ancestor_inodes = job.ancestor_inodes.clone();
                    ancestor_inodes.insert(child_entry.inode);

                    new_jobs.push(Some(ScanJob {
                        abs_path: child_abs_path.clone(),
                        path: child_path,
                        is_external: child_entry.is_external,
                        ignore_stack: if child_entry.is_ignored {
                            IgnoreStack::all()
                        } else {
                            ignore_stack.clone()
                        },
                        ancestor_inodes,
                        scan_queue: job.scan_queue.clone(),
                        containing_repository: job.containing_repository.clone(),
                    }));
                } else {
                    new_jobs.push(None);
                }
            } else {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, false);
                if !child_entry.is_ignored {
                    if let Some((repository_dir, repository, staged_statuses)) =
                        &job.containing_repository
                    {
                        if let Ok(repo_path) = child_entry.path.strip_prefix(&repository_dir.0) {
                            if let Some(mtime) = child_entry.mtime {
                                let repo_path = RepoPath(repo_path.into());
                                child_entry.git_status = combine_git_statuses(
                                    staged_statuses.get(&repo_path).copied(),
                                    repository.lock().unstaged_status(&repo_path, mtime),
                                );
                            }
                        }
                    }
                }
            }

            {
                let relative_path = job.path.join(child_name);
                let state = self.state.lock();
                if state.snapshot.is_path_private(&relative_path) {
                    log::debug!("detected private file: {relative_path:?}");
                    child_entry.is_private = true;
                }
                drop(state)
            }

            new_entries.push(child_entry);
        }

        let mut state = self.state.lock();

        // Identify any subdirectories that should not be scanned.
        let mut job_ix = 0;
        for entry in &mut new_entries {
            state.reuse_entry_id(entry);
            if entry.is_dir() {
                if state.should_scan_directory(entry) {
                    job_ix += 1;
                } else {
                    log::debug!("defer scanning directory {:?}", entry.path);
                    entry.kind = EntryKind::UnloadedDir;
                    new_jobs.remove(job_ix);
                }
            }
        }

        state.populate_dir(&job.path, new_entries, new_ignore);

        let repository =
            dotgit_path.and_then(|path| state.build_git_repository(path, self.fs.as_ref()));

        for mut new_job in new_jobs.into_iter().flatten() {
            if let Some(containing_repository) = &repository {
                new_job.containing_repository = Some(containing_repository.clone());
            }

            job.scan_queue
                .try_send(new_job)
                .expect("channel is unbounded");
        }

        Ok(())
    }

    async fn reload_entries_for_paths(
        &self,
        root_abs_path: Arc<Path>,
        root_canonical_path: PathBuf,
        relative_paths: &[Arc<Path>],
        abs_paths: Vec<PathBuf>,
        scan_queue_tx: Option<Sender<ScanJob>>,
    ) {
        let metadata = futures::future::join_all(
            abs_paths
                .iter()
                .map(|abs_path| async move {
                    let metadata = self.fs.metadata(abs_path).await?;
                    if let Some(metadata) = metadata {
                        let canonical_path = self.fs.canonicalize(abs_path).await?;

                        // If we're on a case-insensitive filesystem (default on macOS), we want
                        // to only ignore metadata for non-symlink files if their absolute-path matches
                        // the canonical-path.
                        // Because if not, this might be a case-only-renaming (`mv test.txt TEST.TXT`)
                        // and we want to ignore the metadata for the old path (`test.txt`) so it's
                        // treated as removed.
                        if !self.fs_case_sensitive && !metadata.is_symlink {
                            let canonical_file_name = canonical_path.file_name();
                            let file_name = abs_path.file_name();
                            if canonical_file_name != file_name {
                                return Ok(None);
                            }
                        }

                        anyhow::Ok(Some((metadata, canonical_path)))
                    } else {
                        Ok(None)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .await;

        let mut state = self.state.lock();
        let snapshot = &mut state.snapshot;
        let is_idle = snapshot.completed_scan_id == snapshot.scan_id;
        let doing_recursive_update = scan_queue_tx.is_some();
        snapshot.scan_id += 1;
        if is_idle && !doing_recursive_update {
            snapshot.completed_scan_id = snapshot.scan_id;
        }

        // Remove any entries for paths that no longer exist or are being recursively
        // refreshed. Do this before adding any new entries, so that renames can be
        // detected regardless of the order of the paths.
        for (path, metadata) in relative_paths.iter().zip(metadata.iter()) {
            if matches!(metadata, Ok(None)) || doing_recursive_update {
                log::trace!("remove path {:?}", path);
                state.remove_path(path);
            }
        }

        for (path, metadata) in relative_paths.iter().zip(metadata.iter()) {
            let abs_path: Arc<Path> = root_abs_path.join(&path).into();
            match metadata {
                Ok(Some((metadata, canonical_path))) => {
                    let ignore_stack = state
                        .snapshot
                        .ignore_stack_for_abs_path(&abs_path, metadata.is_dir);

                    let mut fs_entry = Entry::new(
                        path.clone(),
                        metadata,
                        self.next_entry_id.as_ref(),
                        state.snapshot.root_char_bag,
                    );
                    let is_dir = fs_entry.is_dir();
                    fs_entry.is_ignored = ignore_stack.is_abs_path_ignored(&abs_path, is_dir);
                    fs_entry.is_external = !canonical_path.starts_with(&root_canonical_path);
                    fs_entry.is_private = state.snapshot.is_path_private(path);

                    if !is_dir && !fs_entry.is_ignored && !fs_entry.is_external {
                        if let Some((work_dir, repo)) = state.snapshot.local_repo_for_path(path) {
                            if let Ok(repo_path) = path.strip_prefix(work_dir.0) {
                                if let Some(mtime) = fs_entry.mtime {
                                    let repo_path = RepoPath(repo_path.into());
                                    let repo = repo.repo_ptr.lock();
                                    fs_entry.git_status = repo.status(&repo_path, mtime);
                                }
                            }
                        }
                    }

                    if let (Some(scan_queue_tx), true) = (&scan_queue_tx, fs_entry.is_dir()) {
                        if state.should_scan_directory(&fs_entry) {
                            state.enqueue_scan_dir(abs_path, &fs_entry, scan_queue_tx);
                        } else {
                            fs_entry.kind = EntryKind::UnloadedDir;
                        }
                    }

                    state.insert_entry(fs_entry, self.fs.as_ref());
                }
                Ok(None) => {
                    self.remove_repo_path(path, &mut state.snapshot);
                }
                Err(err) => {
                    // TODO - create a special 'error' entry in the entries tree to mark this
                    log::error!("error reading file {abs_path:?} on event: {err:#}");
                }
            }
        }

        util::extend_sorted(
            &mut state.changed_paths,
            relative_paths.iter().cloned(),
            usize::MAX,
            Ord::cmp,
        );
    }

    fn remove_repo_path(&self, path: &Path, snapshot: &mut LocalSnapshot) -> Option<()> {
        if !path
            .components()
            .any(|component| component.as_os_str() == *DOT_GIT)
        {
            if let Some(repository) = snapshot.repository_for_work_directory(path) {
                let entry = repository.work_directory.0;
                snapshot.git_repositories.remove(&entry);
                snapshot
                    .snapshot
                    .repository_entries
                    .remove(&RepositoryWorkDirectory(path.into()));
                return Some(());
            }
        }

        // TODO statuses
        // Track when a .git is removed and iterate over the file system there

        Some(())
    }

    async fn update_ignore_statuses(&self, scan_job_tx: Sender<ScanJob>) {
        use futures::FutureExt as _;

        let mut snapshot = self.state.lock().snapshot.clone();
        let mut ignores_to_update = Vec::new();
        let mut ignores_to_delete = Vec::new();
        let abs_path = snapshot.abs_path.clone();
        for (parent_abs_path, (_, needs_update)) in &mut snapshot.ignores_by_parent_abs_path {
            if let Ok(parent_path) = parent_abs_path.strip_prefix(&abs_path) {
                if *needs_update {
                    *needs_update = false;
                    if snapshot.snapshot.entry_for_path(parent_path).is_some() {
                        ignores_to_update.push(parent_abs_path.clone());
                    }
                }

                let ignore_path = parent_path.join(&*GITIGNORE);
                if snapshot.snapshot.entry_for_path(ignore_path).is_none() {
                    ignores_to_delete.push(parent_abs_path.clone());
                }
            }
        }

        for parent_abs_path in ignores_to_delete {
            snapshot.ignores_by_parent_abs_path.remove(&parent_abs_path);
            self.state
                .lock()
                .snapshot
                .ignores_by_parent_abs_path
                .remove(&parent_abs_path);
        }

        let (ignore_queue_tx, ignore_queue_rx) = channel::unbounded();
        ignores_to_update.sort_unstable();
        let mut ignores_to_update = ignores_to_update.into_iter().peekable();
        while let Some(parent_abs_path) = ignores_to_update.next() {
            while ignores_to_update
                .peek()
                .map_or(false, |p| p.starts_with(&parent_abs_path))
            {
                ignores_to_update.next().unwrap();
            }

            let ignore_stack = snapshot.ignore_stack_for_abs_path(&parent_abs_path, true);
            smol::block_on(ignore_queue_tx.send(UpdateIgnoreStatusJob {
                abs_path: parent_abs_path,
                ignore_stack,
                ignore_queue: ignore_queue_tx.clone(),
                scan_queue: scan_job_tx.clone(),
            }))
            .unwrap();
        }
        drop(ignore_queue_tx);

        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        loop {
                            select_biased! {
                                // Process any path refresh requests before moving on to process
                                // the queue of ignore statuses.
                                request = self.scan_requests_rx.recv().fuse() => {
                                    let Ok(request) = request else { break };
                                    if !self.process_scan_request(request, true).await {
                                        return;
                                    }
                                }

                                // Recursively process directories whose ignores have changed.
                                job = ignore_queue_rx.recv().fuse() => {
                                    let Ok(job) = job else { break };
                                    self.update_ignore_status(job, &snapshot).await;
                                }
                            }
                        }
                    });
                }
            })
            .await;
    }

    async fn update_ignore_status(&self, job: UpdateIgnoreStatusJob, snapshot: &LocalSnapshot) {
        log::trace!("update ignore status {:?}", job.abs_path);

        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores_by_parent_abs_path.get(&job.abs_path) {
            ignore_stack = ignore_stack.append(job.abs_path.clone(), ignore.clone());
        }

        let mut entries_by_id_edits = Vec::new();
        let mut entries_by_path_edits = Vec::new();
        let path = job.abs_path.strip_prefix(&snapshot.abs_path).unwrap();
        let repo = snapshot
            .local_repo_for_path(path)
            .map_or(None, |local_repo| Some(local_repo.1));
        for mut entry in snapshot.child_entries(path).cloned() {
            let was_ignored = entry.is_ignored;
            let abs_path: Arc<Path> = snapshot.abs_path().join(&entry.path).into();
            entry.is_ignored = ignore_stack.is_abs_path_ignored(&abs_path, entry.is_dir());
            if entry.is_dir() {
                let child_ignore_stack = if entry.is_ignored {
                    IgnoreStack::all()
                } else {
                    ignore_stack.clone()
                };

                // Scan any directories that were previously ignored and weren't previously scanned.
                if was_ignored && !entry.is_ignored && entry.kind.is_unloaded() {
                    let state = self.state.lock();
                    if state.should_scan_directory(&entry) {
                        state.enqueue_scan_dir(abs_path.clone(), &entry, &job.scan_queue);
                    }
                }

                job.ignore_queue
                    .send(UpdateIgnoreStatusJob {
                        abs_path: abs_path.clone(),
                        ignore_stack: child_ignore_stack,
                        ignore_queue: job.ignore_queue.clone(),
                        scan_queue: job.scan_queue.clone(),
                    })
                    .await
                    .unwrap();
            }

            if entry.is_ignored != was_ignored {
                let mut path_entry = snapshot.entries_by_id.get(&entry.id, &()).unwrap().clone();
                path_entry.scan_id = snapshot.scan_id;
                path_entry.is_ignored = entry.is_ignored;
                if !entry.is_dir() && !entry.is_ignored && !entry.is_external {
                    if let Some(repo) = repo {
                        if let Some(mtime) = &entry.mtime {
                            let repo_path = RepoPath(entry.path.to_path_buf());
                            let repo = repo.repo_ptr.lock();
                            entry.git_status = repo.status(&repo_path, *mtime);
                        }
                    }
                }
                entries_by_id_edits.push(Edit::Insert(path_entry));
                entries_by_path_edits.push(Edit::Insert(entry));
            }
        }

        let state = &mut self.state.lock();
        for edit in &entries_by_path_edits {
            if let Edit::Insert(entry) = edit {
                if let Err(ix) = state.changed_paths.binary_search(&entry.path) {
                    state.changed_paths.insert(ix, entry.path.clone());
                }
            }
        }

        state
            .snapshot
            .entries_by_path
            .edit(entries_by_path_edits, &());
        state.snapshot.entries_by_id.edit(entries_by_id_edits, &());
    }

    fn build_change_set(
        &self,
        old_snapshot: &Snapshot,
        new_snapshot: &Snapshot,
        event_paths: &[Arc<Path>],
    ) -> UpdatedEntriesSet {
        use BackgroundScannerPhase::*;
        use PathChange::{Added, AddedOrUpdated, Loaded, Removed, Updated};

        // Identify which paths have changed. Use the known set of changed
        // parent paths to optimize the search.
        let mut changes = Vec::new();
        let mut old_paths = old_snapshot.entries_by_path.cursor::<PathKey>();
        let mut new_paths = new_snapshot.entries_by_path.cursor::<PathKey>();
        let mut last_newly_loaded_dir_path = None;
        old_paths.next(&());
        new_paths.next(&());
        for path in event_paths {
            let path = PathKey(path.clone());
            if old_paths.item().map_or(false, |e| e.path < path.0) {
                old_paths.seek_forward(&path, Bias::Left, &());
            }
            if new_paths.item().map_or(false, |e| e.path < path.0) {
                new_paths.seek_forward(&path, Bias::Left, &());
            }
            loop {
                match (old_paths.item(), new_paths.item()) {
                    (Some(old_entry), Some(new_entry)) => {
                        if old_entry.path > path.0
                            && new_entry.path > path.0
                            && !old_entry.path.starts_with(&path.0)
                            && !new_entry.path.starts_with(&path.0)
                        {
                            break;
                        }

                        match Ord::cmp(&old_entry.path, &new_entry.path) {
                            Ordering::Less => {
                                changes.push((old_entry.path.clone(), old_entry.id, Removed));
                                old_paths.next(&());
                            }
                            Ordering::Equal => {
                                if self.phase == EventsReceivedDuringInitialScan {
                                    if old_entry.id != new_entry.id {
                                        changes.push((
                                            old_entry.path.clone(),
                                            old_entry.id,
                                            Removed,
                                        ));
                                    }
                                    // If the worktree was not fully initialized when this event was generated,
                                    // we can't know whether this entry was added during the scan or whether
                                    // it was merely updated.
                                    changes.push((
                                        new_entry.path.clone(),
                                        new_entry.id,
                                        AddedOrUpdated,
                                    ));
                                } else if old_entry.id != new_entry.id {
                                    changes.push((old_entry.path.clone(), old_entry.id, Removed));
                                    changes.push((new_entry.path.clone(), new_entry.id, Added));
                                } else if old_entry != new_entry {
                                    if old_entry.kind.is_unloaded() {
                                        last_newly_loaded_dir_path = Some(&new_entry.path);
                                        changes.push((
                                            new_entry.path.clone(),
                                            new_entry.id,
                                            Loaded,
                                        ));
                                    } else {
                                        changes.push((
                                            new_entry.path.clone(),
                                            new_entry.id,
                                            Updated,
                                        ));
                                    }
                                }
                                old_paths.next(&());
                                new_paths.next(&());
                            }
                            Ordering::Greater => {
                                let is_newly_loaded = self.phase == InitialScan
                                    || last_newly_loaded_dir_path
                                        .as_ref()
                                        .map_or(false, |dir| new_entry.path.starts_with(&dir));
                                changes.push((
                                    new_entry.path.clone(),
                                    new_entry.id,
                                    if is_newly_loaded { Loaded } else { Added },
                                ));
                                new_paths.next(&());
                            }
                        }
                    }
                    (Some(old_entry), None) => {
                        changes.push((old_entry.path.clone(), old_entry.id, Removed));
                        old_paths.next(&());
                    }
                    (None, Some(new_entry)) => {
                        let is_newly_loaded = self.phase == InitialScan
                            || last_newly_loaded_dir_path
                                .as_ref()
                                .map_or(false, |dir| new_entry.path.starts_with(&dir));
                        changes.push((
                            new_entry.path.clone(),
                            new_entry.id,
                            if is_newly_loaded { Loaded } else { Added },
                        ));
                        new_paths.next(&());
                    }
                    (None, None) => break,
                }
            }
        }

        changes.into()
    }

    async fn progress_timer(&self, running: bool) {
        if !running {
            return futures::future::pending().await;
        }

        #[cfg(any(test, feature = "test-support"))]
        if self.fs.is_fake() {
            return self.executor.simulate_random_delay().await;
        }

        smol::Timer::after(FS_WATCH_LATENCY).await;
    }
}

fn char_bag_for_path(root_char_bag: CharBag, path: &Path) -> CharBag {
    let mut result = root_char_bag;
    result.extend(
        path.to_string_lossy()
            .chars()
            .map(|c| c.to_ascii_lowercase()),
    );
    result
}

struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    scan_queue: Sender<ScanJob>,
    ancestor_inodes: TreeSet<u64>,
    is_external: bool,
    containing_repository: Option<(
        RepositoryWorkDirectory,
        Arc<Mutex<dyn GitRepository>>,
        TreeMap<RepoPath, GitFileStatus>,
    )>,
}

struct UpdateIgnoreStatusJob {
    abs_path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    ignore_queue: Sender<UpdateIgnoreStatusJob>,
    scan_queue: Sender<ScanJob>,
}

pub trait WorktreeModelHandle {
    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;
}

impl WorktreeModelHandle for Model<Worktree> {
    // When the worktree's FS event stream sometimes delivers "redundant" events for FS changes that
    // occurred before the worktree was constructed. These events can cause the worktree to perform
    // extra directory scans, and emit extra scan-state notifications.
    //
    // This function mutates the worktree's directory and waits for those mutations to be picked up,
    // to ensure that all redundant FS events have already been processed.
    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        let file_name = "fs-event-sentinel";

        let tree = self.clone();
        let (fs, root_path) = self.update(cx, |tree, _| {
            let tree = tree.as_local().unwrap();
            (tree.fs.clone(), tree.abs_path().clone())
        });

        async move {
            fs.create_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            cx.condition(&tree, |tree, _| tree.entry_for_path(file_name).is_some())
                .await;

            fs.remove_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();
            cx.condition(&tree, |tree, _| tree.entry_for_path(file_name).is_none())
                .await;

            cx.update(|cx| tree.read(cx).as_local().unwrap().scan_complete())
                .await;
        }
        .boxed_local()
    }
}

#[derive(Clone, Debug)]
struct TraversalProgress<'a> {
    max_path: &'a Path,
    count: usize,
    non_ignored_count: usize,
    file_count: usize,
    non_ignored_file_count: usize,
}

impl<'a> TraversalProgress<'a> {
    fn count(&self, include_dirs: bool, include_ignored: bool) -> usize {
        match (include_ignored, include_dirs) {
            (true, true) => self.count,
            (true, false) => self.file_count,
            (false, true) => self.non_ignored_count,
            (false, false) => self.non_ignored_file_count,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for TraversalProgress<'a> {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.max_path = summary.max_path.as_ref();
        self.count += summary.count;
        self.non_ignored_count += summary.non_ignored_count;
        self.file_count += summary.file_count;
        self.non_ignored_file_count += summary.non_ignored_file_count;
    }
}

impl<'a> Default for TraversalProgress<'a> {
    fn default() -> Self {
        Self {
            max_path: Path::new(""),
            count: 0,
            non_ignored_count: 0,
            file_count: 0,
            non_ignored_file_count: 0,
        }
    }
}

#[derive(Clone, Debug, Default, Copy)]
struct GitStatuses {
    added: usize,
    modified: usize,
    conflict: usize,
}

impl AddAssign for GitStatuses {
    fn add_assign(&mut self, rhs: Self) {
        self.added += rhs.added;
        self.modified += rhs.modified;
        self.conflict += rhs.conflict;
    }
}

impl Sub for GitStatuses {
    type Output = GitStatuses;

    fn sub(self, rhs: Self) -> Self::Output {
        GitStatuses {
            added: self.added - rhs.added,
            modified: self.modified - rhs.modified,
            conflict: self.conflict - rhs.conflict,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for GitStatuses {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        *self += summary.statuses
    }
}

pub struct Traversal<'a> {
    cursor: sum_tree::Cursor<'a, Entry, TraversalProgress<'a>>,
    include_ignored: bool,
    include_dirs: bool,
}

impl<'a> Traversal<'a> {
    pub fn advance(&mut self) -> bool {
        self.cursor.seek_forward(
            &TraversalTarget::Count {
                count: self.end_offset() + 1,
                include_dirs: self.include_dirs,
                include_ignored: self.include_ignored,
            },
            Bias::Left,
            &(),
        )
    }

    pub fn advance_to_sibling(&mut self) -> bool {
        while let Some(entry) = self.cursor.item() {
            self.cursor.seek_forward(
                &TraversalTarget::PathSuccessor(&entry.path),
                Bias::Left,
                &(),
            );
            if let Some(entry) = self.cursor.item() {
                if (self.include_dirs || !entry.is_dir())
                    && (self.include_ignored || !entry.is_ignored)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn entry(&self) -> Option<&'a Entry> {
        self.cursor.item()
    }

    pub fn start_offset(&self) -> usize {
        self.cursor
            .start()
            .count(self.include_dirs, self.include_ignored)
    }

    pub fn end_offset(&self) -> usize {
        self.cursor
            .end(&())
            .count(self.include_dirs, self.include_ignored)
    }
}

impl<'a> Iterator for Traversal<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.entry() {
            self.advance();
            Some(item)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum TraversalTarget<'a> {
    Path(&'a Path),
    PathSuccessor(&'a Path),
    Count {
        count: usize,
        include_ignored: bool,
        include_dirs: bool,
    },
}

impl<'a, 'b> SeekTarget<'a, EntrySummary, TraversalProgress<'a>> for TraversalTarget<'b> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: &()) -> Ordering {
        match self {
            TraversalTarget::Path(path) => path.cmp(&cursor_location.max_path),
            TraversalTarget::PathSuccessor(path) => {
                if !cursor_location.max_path.starts_with(path) {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
            TraversalTarget::Count {
                count,
                include_dirs,
                include_ignored,
            } => Ord::cmp(
                count,
                &cursor_location.count(*include_dirs, *include_ignored),
            ),
        }
    }
}

impl<'a, 'b> SeekTarget<'a, EntrySummary, (TraversalProgress<'a>, GitStatuses)>
    for TraversalTarget<'b>
{
    fn cmp(&self, cursor_location: &(TraversalProgress<'a>, GitStatuses), _: &()) -> Ordering {
        self.cmp(&cursor_location.0, &())
    }
}

struct ChildEntriesIter<'a> {
    parent_path: &'a Path,
    traversal: Traversal<'a>,
}

impl<'a> Iterator for ChildEntriesIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.traversal.entry() {
            if item.path.starts_with(&self.parent_path) {
                self.traversal.advance_to_sibling();
                return Some(item);
            }
        }
        None
    }
}

pub struct DescendentEntriesIter<'a> {
    parent_path: &'a Path,
    traversal: Traversal<'a>,
}

impl<'a> Iterator for DescendentEntriesIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.traversal.entry() {
            if item.path.starts_with(&self.parent_path) {
                self.traversal.advance();
                return Some(item);
            }
        }
        None
    }
}

impl<'a> From<&'a Entry> for proto::Entry {
    fn from(entry: &'a Entry) -> Self {
        Self {
            id: entry.id.to_proto(),
            is_dir: entry.is_dir(),
            path: entry.path.to_string_lossy().into(),
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            is_symlink: entry.is_symlink,
            is_ignored: entry.is_ignored,
            is_external: entry.is_external,
            git_status: entry.git_status.map(git_status_to_proto),
        }
    }
}

impl<'a> TryFrom<(&'a CharBag, proto::Entry)> for Entry {
    type Error = anyhow::Error;

    fn try_from((root_char_bag, entry): (&'a CharBag, proto::Entry)) -> Result<Self> {
        let kind = if entry.is_dir {
            EntryKind::Dir
        } else {
            let mut char_bag = *root_char_bag;
            char_bag.extend(entry.path.chars().map(|c| c.to_ascii_lowercase()));
            EntryKind::File(char_bag)
        };
        let path: Arc<Path> = PathBuf::from(entry.path).into();
        Ok(Entry {
            id: ProjectEntryId::from_proto(entry.id),
            kind,
            path,
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            is_symlink: entry.is_symlink,
            is_ignored: entry.is_ignored,
            is_external: entry.is_external,
            git_status: git_status_from_proto(entry.git_status),
            is_private: false,
        })
    }
}

fn combine_git_statuses(
    staged: Option<GitFileStatus>,
    unstaged: Option<GitFileStatus>,
) -> Option<GitFileStatus> {
    if let Some(staged) = staged {
        if let Some(unstaged) = unstaged {
            if unstaged != staged {
                Some(GitFileStatus::Modified)
            } else {
                Some(staged)
            }
        } else {
            Some(staged)
        }
    } else {
        unstaged
    }
}

fn git_status_from_proto(git_status: Option<i32>) -> Option<GitFileStatus> {
    git_status.and_then(|status| {
        proto::GitStatus::from_i32(status).map(|status| match status {
            proto::GitStatus::Added => GitFileStatus::Added,
            proto::GitStatus::Modified => GitFileStatus::Modified,
            proto::GitStatus::Conflict => GitFileStatus::Conflict,
        })
    })
}

fn git_status_to_proto(status: GitFileStatus) -> i32 {
    match status {
        GitFileStatus::Added => proto::GitStatus::Added as i32,
        GitFileStatus::Modified => proto::GitStatus::Modified as i32,
        GitFileStatus::Conflict => proto::GitStatus::Conflict as i32,
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectEntryId(usize);

impl ProjectEntryId {
    pub const MAX: Self = Self(usize::MAX);

    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub struct DiagnosticSummary {
    pub error_count: usize,
    pub warning_count: usize,
}

impl DiagnosticSummary {
    fn new<'a, T: 'a>(diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<T>>) -> Self {
        let mut this = Self {
            error_count: 0,
            warning_count: 0,
        };

        for entry in diagnostics {
            if entry.diagnostic.is_primary {
                match entry.diagnostic.severity {
                    DiagnosticSeverity::ERROR => this.error_count += 1,
                    DiagnosticSeverity::WARNING => this.warning_count += 1,
                    _ => {}
                }
            }
        }

        this
    }

    pub fn is_empty(&self) -> bool {
        self.error_count == 0 && self.warning_count == 0
    }

    pub fn to_proto(
        &self,
        language_server_id: LanguageServerId,
        path: &Path,
    ) -> proto::DiagnosticSummary {
        proto::DiagnosticSummary {
            path: path.to_string_lossy().to_string(),
            language_server_id: language_server_id.0 as u64,
            error_count: self.error_count as u32,
            warning_count: self.warning_count as u32,
        }
    }
}
