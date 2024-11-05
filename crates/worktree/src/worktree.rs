mod ignore;
mod worktree_settings;
#[cfg(test)]
mod worktree_tests;

use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{anyhow, Context as _, Result};
use clock::ReplicaId;
use collections::{HashMap, HashSet, VecDeque};
use fs::{copy_recursive, Fs, PathEvent, RemoveOptions, Watcher};
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
use git::GitHostingProviderRegistry;
use git::{
    repository::{GitFileStatus, GitRepository, RepoPath},
    status::GitStatus,
    COOKIES, DOT_GIT, FSMONITOR_DAEMON, GITIGNORE,
};
use gpui::{
    AppContext, AsyncAppContext, BackgroundExecutor, Context, EventEmitter, Model, ModelContext,
    Task,
};
use ignore::IgnoreStack;
use parking_lot::Mutex;
use paths::local_settings_folder_relative_path;
use postage::{
    barrier,
    prelude::{Sink as _, Stream as _},
    watch,
};
use rpc::{
    proto::{self, split_worktree_update},
    AnyProtoClient,
};
pub use settings::WorktreeId;
use settings::{Settings, SettingsLocation, SettingsStore};
use smallvec::{smallvec, SmallVec};
use smol::channel::{self, Sender};
use std::{
    any::Any,
    cmp::Ordering,
    collections::hash_map,
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
    time::{Duration, Instant, SystemTime},
};
use sum_tree::{Bias, Edit, SeekTarget, SumTree, TreeMap, TreeSet};
use text::{LineEnding, Rope};
use util::{paths::home_dir, ResultExt};
pub use worktree_settings::WorktreeSettings;

#[cfg(feature = "test-support")]
pub const FS_WATCH_LATENCY: Duration = Duration::from_millis(100);
#[cfg(not(feature = "test-support"))]
pub const FS_WATCH_LATENCY: Duration = Duration::from_millis(100);

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

/// An entry, created in the worktree.
#[derive(Debug)]
pub enum CreatedEntry {
    /// Got created and indexed by the worktree, receiving a corresponding entry.
    Included(Entry),
    /// Got created, but not indexed due to falling under exclusion filters.
    Excluded { abs_path: PathBuf },
}

pub struct LoadedFile {
    pub file: Arc<File>,
    pub text: String,
    pub diff_base: Option<String>,
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    scan_requests_tx: channel::Sender<ScanRequest>,
    path_prefixes_to_scan_tx: channel::Sender<Arc<Path>>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    _background_scanner_tasks: Vec<Task<()>>,
    update_observer: Option<UpdateObservationState>,
    fs: Arc<dyn Fs>,
    fs_case_sensitive: bool,
    visible: bool,
    next_entry_id: Arc<AtomicUsize>,
    settings: WorktreeSettings,
    share_private_files: bool,
}

struct ScanRequest {
    relative_paths: Vec<Arc<Path>>,
    done: SmallVec<[barrier::Sender; 1]>,
}

pub struct RemoteWorktree {
    snapshot: Snapshot,
    background_snapshot: Arc<Mutex<(Snapshot, Vec<proto::UpdateWorktree>)>>,
    project_id: u64,
    client: AnyProtoClient,
    updates_tx: Option<UnboundedSender<proto::UpdateWorktree>>,
    update_observer: Option<mpsc::UnboundedSender<proto::UpdateWorktree>>,
    snapshot_subscriptions: VecDeque<(usize, oneshot::Sender<()>)>,
    replica_id: ReplicaId,
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

    /// If location_in_repo is set, it means the .git folder is external
    /// and in a parent folder of the project root.
    /// In that case, the work_directory field will point to the
    /// project-root and location_in_repo contains the location of the
    /// project-root in the repository.
    ///
    /// Example:
    ///
    ///     my_root_folder/          <-- repository root
    ///       .git
    ///       my_sub_folder_1/
    ///         project_root/        <-- Project root, Zed opened here
    ///           ...
    ///
    /// For this setup, the attributes will have the following values:
    ///
    ///     work_directory: pointing to "" entry
    ///     location_in_repo: Some("my_sub_folder_1/project_root")
    pub(crate) location_in_repo: Option<Arc<Path>>,
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
        self.into()
    }

    /// relativize returns the given project path relative to the root folder of the
    /// repository.
    /// If the root of the repository (and its .git folder) are located in a parent folder
    /// of the project root folder, then the returned RepoPath is relative to the root
    /// of the repository and not a valid path inside the project.
    pub fn relativize(&self, worktree: &Snapshot, path: &Path) -> Result<RepoPath> {
        let relativize_path = |path: &Path| {
            let entry = worktree
                .entry_for_id(self.work_directory.0)
                .ok_or_else(|| anyhow!("entry not found"))?;

            let relativized_path = path
                .strip_prefix(&entry.path)
                .map_err(|_| anyhow!("could not relativize {:?} against {:?}", path, entry.path))?;

            Ok(relativized_path.into())
        };

        if let Some(location_in_repo) = &self.location_in_repo {
            relativize_path(&location_in_repo.join(path))
        } else {
            relativize_path(path)
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

/// This path corresponds to the 'content path' of a repository in relation
/// to Zed's project root.
/// In the majority of the cases, this is the folder that contains the .git folder.
/// But if a sub-folder of a git repository is opened, this corresponds to the
/// project root and the .git folder is located in a parent directory.
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
    removed_entries: HashMap<u64, Entry>,
    changed_paths: Vec<Arc<Path>>,
    prev_snapshot: Snapshot,
    git_hosting_provider_registry: Option<Arc<GitHostingProviderRegistry>>,
}

#[derive(Debug, Clone)]
pub struct LocalRepositoryEntry {
    pub(crate) git_dir_scan_id: usize,
    pub(crate) repo_ptr: Arc<dyn GitRepository>,
    /// Absolute path to the actual .git folder.
    /// Note: if .git is a file, this points to the folder indicated by the .git file
    pub(crate) git_dir_abs_path: Arc<Path>,
}

impl LocalRepositoryEntry {
    pub fn repo(&self) -> &Arc<dyn GitRepository> {
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
        barrier: SmallVec<[barrier::Sender; 1]>,
        scanning: bool,
    },
}

struct UpdateObservationState {
    snapshots_tx:
        mpsc::UnboundedSender<(LocalSnapshot, UpdatedEntriesSet, UpdatedGitRepositoriesSet)>,
    resume_updates: watch::Sender<()>,
    _maintain_remote_snapshot: Task<Option<()>>,
}

#[derive(Clone)]
pub enum Event {
    UpdatedEntries(UpdatedEntriesSet),
    UpdatedGitRepositories(UpdatedGitRepositoriesSet),
    DeletedEntry(ProjectEntryId),
}

const EMPTY_PATH: &str = "";

impl EventEmitter<Event> for Worktree {}

impl Worktree {
    pub async fn local(
        path: impl Into<Arc<Path>>,
        visible: bool,
        fs: Arc<dyn Fs>,
        next_entry_id: Arc<AtomicUsize>,
        cx: &mut AsyncAppContext,
    ) -> Result<Model<Self>> {
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

        cx.new_model(move |cx: &mut ModelContext<Worktree>| {
            let mut snapshot = LocalSnapshot {
                ignores_by_parent_abs_path: Default::default(),
                git_repositories: Default::default(),
                snapshot: Snapshot::new(
                    cx.entity_id().as_u64(),
                    abs_path
                        .file_name()
                        .map_or(String::new(), |f| f.to_string_lossy().to_string()),
                    abs_path,
                ),
            };

            if let Some(metadata) = metadata {
                snapshot.insert_entry(
                    Entry::new(
                        Arc::from(Path::new("")),
                        &metadata,
                        &next_entry_id,
                        snapshot.root_char_bag,
                        None,
                    ),
                    fs.as_ref(),
                );
            }

            let worktree_id = snapshot.id();
            let settings_location = Some(SettingsLocation {
                worktree_id,
                path: Path::new(EMPTY_PATH),
            });

            let settings = WorktreeSettings::get(settings_location, cx).clone();
            cx.observe_global::<SettingsStore>(move |this, cx| {
                if let Self::Local(this) = this {
                    let settings = WorktreeSettings::get(settings_location, cx).clone();
                    if settings != this.settings {
                        this.settings = settings;
                        this.restart_background_scanners(cx);
                    }
                }
            })
            .detach();

            let (scan_requests_tx, scan_requests_rx) = channel::unbounded();
            let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = channel::unbounded();
            let mut worktree = LocalWorktree {
                share_private_files: false,
                next_entry_id,
                snapshot,
                is_scanning: watch::channel_with(true),
                update_observer: None,
                scan_requests_tx,
                path_prefixes_to_scan_tx,
                _background_scanner_tasks: Vec::new(),
                fs,
                fs_case_sensitive,
                visible,
                settings,
            };
            worktree.start_background_scanner(scan_requests_rx, path_prefixes_to_scan_rx, cx);
            Worktree::Local(worktree)
        })
    }

    pub fn remote(
        project_id: u64,
        replica_id: ReplicaId,
        worktree: proto::WorktreeMetadata,
        client: AnyProtoClient,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx: &mut ModelContext<Self>| {
            let snapshot = Snapshot::new(
                worktree.id,
                worktree.root_name,
                Arc::from(PathBuf::from(worktree.abs_path)),
            );

            let background_snapshot = Arc::new(Mutex::new((snapshot.clone(), Vec::new())));
            let (background_updates_tx, mut background_updates_rx) = mpsc::unbounded();
            let (mut snapshot_updated_tx, mut snapshot_updated_rx) = watch::channel();

            let worktree = RemoteWorktree {
                client,
                project_id,
                replica_id,
                snapshot,
                background_snapshot: background_snapshot.clone(),
                updates_tx: Some(background_updates_tx),
                update_observer: None,
                snapshot_subscriptions: Default::default(),
                visible: worktree.visible,
                disconnected: false,
            };

            // Apply updates to a separate snapshot in a background task, then
            // send them to a foreground task which updates the model.
            cx.background_executor()
                .spawn(async move {
                    while let Some(update) = background_updates_rx.next().await {
                        {
                            let mut lock = background_snapshot.lock();
                            if let Err(error) = lock.0.apply_remote_update(update.clone()) {
                                log::error!("error applying worktree update: {}", error);
                            }
                            lock.1.push(update);
                        }
                        snapshot_updated_tx.send(()).await.ok();
                    }
                })
                .detach();

            // On the foreground task, update to the latest snapshot and notify
            // any update observer of all updates that led to that snapshot.
            cx.spawn(|this, mut cx| async move {
                while (snapshot_updated_rx.recv().await).is_some() {
                    this.update(&mut cx, |this, cx| {
                        let this = this.as_remote_mut().unwrap();
                        {
                            let mut lock = this.background_snapshot.lock();
                            this.snapshot = lock.0.clone();
                            if let Some(tx) = &this.update_observer {
                                for update in lock.1.drain(..) {
                                    tx.unbounded_send(update).ok();
                                }
                            }
                        };
                        cx.emit(Event::UpdatedEntries(Arc::default()));
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

            Worktree::Remote(worktree)
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

    pub fn settings_location(&self, _: &ModelContext<Self>) -> SettingsLocation<'static> {
        SettingsLocation {
            worktree_id: self.id(),
            path: Path::new(EMPTY_PATH),
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        match self {
            Worktree::Local(worktree) => worktree.snapshot.snapshot.clone(),
            Worktree::Remote(worktree) => worktree.snapshot.clone(),
        }
    }

    pub fn scan_id(&self) -> usize {
        match self {
            Worktree::Local(worktree) => worktree.snapshot.scan_id,
            Worktree::Remote(worktree) => worktree.snapshot.scan_id,
        }
    }

    pub fn metadata_proto(&self) -> proto::WorktreeMetadata {
        proto::WorktreeMetadata {
            id: self.id().to_proto(),
            root_name: self.root_name().to_string(),
            visible: self.is_visible(),
            abs_path: self.abs_path().as_os_str().to_string_lossy().into(),
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

    pub fn abs_path(&self) -> Arc<Path> {
        match self {
            Worktree::Local(worktree) => worktree.abs_path.clone(),
            Worktree::Remote(worktree) => worktree.abs_path.clone(),
        }
    }

    pub fn root_file(&self, cx: &ModelContext<Self>) -> Option<Arc<File>> {
        let entry = self.root_entry()?;
        Some(File::for_entry(entry.clone(), cx.handle()))
    }

    pub fn observe_updates<F, Fut>(
        &mut self,
        project_id: u64,
        cx: &ModelContext<Worktree>,
        callback: F,
    ) where
        F: 'static + Send + Fn(proto::UpdateWorktree) -> Fut,
        Fut: 'static + Send + Future<Output = bool>,
    {
        match self {
            Worktree::Local(this) => this.observe_updates(project_id, cx, callback),
            Worktree::Remote(this) => this.observe_updates(project_id, cx, callback),
        }
    }

    pub fn stop_observing_updates(&mut self) {
        match self {
            Worktree::Local(this) => {
                this.update_observer.take();
            }
            Worktree::Remote(this) => {
                this.update_observer.take();
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_update_observer(&self) -> bool {
        match self {
            Worktree::Local(this) => this.update_observer.is_some(),
            Worktree::Remote(this) => this.update_observer.is_some(),
        }
    }

    pub fn load_file(&self, path: &Path, cx: &ModelContext<Worktree>) -> Task<Result<LoadedFile>> {
        match self {
            Worktree::Local(this) => this.load_file(path, cx),
            Worktree::Remote(_) => {
                Task::ready(Err(anyhow!("remote worktrees can't yet load files")))
            }
        }
    }

    pub fn write_file(
        &self,
        path: &Path,
        text: Rope,
        line_ending: LineEnding,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Arc<File>>> {
        match self {
            Worktree::Local(this) => this.write_file(path, text, line_ending, cx),
            Worktree::Remote(_) => {
                Task::ready(Err(anyhow!("remote worktree can't yet write files")))
            }
        }
    }

    pub fn create_entry(
        &mut self,
        path: impl Into<Arc<Path>>,
        is_directory: bool,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let path = path.into();
        let worktree_id = self.id();
        match self {
            Worktree::Local(this) => this.create_entry(path, is_directory, cx),
            Worktree::Remote(this) => {
                let project_id = this.project_id;
                let request = this.client.request(proto::CreateProjectEntry {
                    worktree_id: worktree_id.to_proto(),
                    project_id,
                    path: path.to_string_lossy().into(),
                    is_directory,
                });
                cx.spawn(move |this, mut cx| async move {
                    let response = request.await?;
                    match response.entry {
                        Some(entry) => this
                            .update(&mut cx, |worktree, cx| {
                                worktree.as_remote_mut().unwrap().insert_entry(
                                    entry,
                                    response.worktree_scan_id as usize,
                                    cx,
                                )
                            })?
                            .await
                            .map(CreatedEntry::Included),
                        None => {
                            let abs_path = this.update(&mut cx, |worktree, _| {
                                worktree
                                    .absolutize(&path)
                                    .with_context(|| format!("absolutizing {path:?}"))
                            })??;
                            Ok(CreatedEntry::Excluded { abs_path })
                        }
                    }
                })
            }
        }
    }

    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let task = match self {
            Worktree::Local(this) => this.delete_entry(entry_id, trash, cx),
            Worktree::Remote(this) => this.delete_entry(entry_id, trash, cx),
        }?;

        let entry = match self {
            Worktree::Local(ref this) => this.entry_for_id(entry_id),
            Worktree::Remote(ref this) => this.entry_for_id(entry_id),
        }?;

        let mut ids = vec![entry_id];
        let path = &*entry.path;

        self.get_children_ids_recursive(path, &mut ids);

        for id in ids {
            cx.emit(Event::DeletedEntry(id));
        }
        Some(task)
    }

    fn get_children_ids_recursive(&self, path: &Path, ids: &mut Vec<ProjectEntryId>) {
        let children_iter = self.child_entries(path);
        for child in children_iter {
            ids.push(child.id);
            self.get_children_ids_recursive(&child.path, ids);
        }
    }

    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &ModelContext<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let new_path = new_path.into();
        match self {
            Worktree::Local(this) => this.rename_entry(entry_id, new_path, cx),
            Worktree::Remote(this) => this.rename_entry(entry_id, new_path, cx),
        }
    }

    pub fn copy_entry(
        &mut self,
        entry_id: ProjectEntryId,
        relative_worktree_source_path: Option<PathBuf>,
        new_path: impl Into<Arc<Path>>,
        cx: &ModelContext<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let new_path = new_path.into();
        match self {
            Worktree::Local(this) => {
                this.copy_entry(entry_id, relative_worktree_source_path, new_path, cx)
            }
            Worktree::Remote(this) => {
                let relative_worktree_source_path =
                    relative_worktree_source_path.map(|relative_worktree_source_path| {
                        relative_worktree_source_path.to_string_lossy().into()
                    });
                let response = this.client.request(proto::CopyProjectEntry {
                    project_id: this.project_id,
                    entry_id: entry_id.to_proto(),
                    relative_worktree_source_path,
                    new_path: new_path.to_string_lossy().into(),
                });
                cx.spawn(move |this, mut cx| async move {
                    let response = response.await?;
                    match response.entry {
                        Some(entry) => this
                            .update(&mut cx, |worktree, cx| {
                                worktree.as_remote_mut().unwrap().insert_entry(
                                    entry,
                                    response.worktree_scan_id as usize,
                                    cx,
                                )
                            })?
                            .await
                            .map(Some),
                        None => Ok(None),
                    }
                })
            }
        }
    }

    pub fn copy_external_entries(
        &mut self,
        target_directory: PathBuf,
        paths: Vec<Arc<Path>>,
        overwrite_existing_files: bool,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Vec<ProjectEntryId>>> {
        match self {
            Worktree::Local(this) => {
                this.copy_external_entries(target_directory, paths, overwrite_existing_files, cx)
            }
            _ => Task::ready(Err(anyhow!(
                "Copying external entries is not supported for remote worktrees"
            ))),
        }
    }

    pub fn expand_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        match self {
            Worktree::Local(this) => this.expand_entry(entry_id, cx),
            Worktree::Remote(this) => {
                let response = this.client.request(proto::ExpandProjectEntry {
                    project_id: this.project_id,
                    entry_id: entry_id.to_proto(),
                });
                Some(cx.spawn(move |this, mut cx| async move {
                    let response = response.await?;
                    this.update(&mut cx, |this, _| {
                        this.as_remote_mut()
                            .unwrap()
                            .wait_for_snapshot(response.worktree_scan_id as usize)
                    })?
                    .await?;
                    Ok(())
                }))
            }
        }
    }

    pub async fn handle_create_entry(
        this: Model<Self>,
        request: proto::CreateProjectEntry,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, entry) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.create_entry(PathBuf::from(request.path), request.is_directory, cx),
            )
        })?;
        Ok(proto::ProjectEntryResponse {
            entry: match &entry.await? {
                CreatedEntry::Included(entry) => Some(entry.into()),
                CreatedEntry::Excluded { .. } => None,
            },
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_delete_entry(
        this: Model<Self>,
        request: proto::DeleteProjectEntry,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.delete_entry(
                    ProjectEntryId::from_proto(request.entry_id),
                    request.use_trash,
                    cx,
                ),
            )
        })?;
        task.ok_or_else(|| anyhow!("invalid entry"))?.await?;
        Ok(proto::ProjectEntryResponse {
            entry: None,
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_expand_entry(
        this: Model<Self>,
        request: proto::ExpandProjectEntry,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ExpandProjectEntryResponse> {
        let task = this.update(&mut cx, |this, cx| {
            this.expand_entry(ProjectEntryId::from_proto(request.entry_id), cx)
        })?;
        task.ok_or_else(|| anyhow!("no such entry"))?.await?;
        let scan_id = this.read_with(&cx, |this, _| this.scan_id())?;
        Ok(proto::ExpandProjectEntryResponse {
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_rename_entry(
        this: Model<Self>,
        request: proto::RenameProjectEntry,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.rename_entry(
                    ProjectEntryId::from_proto(request.entry_id),
                    PathBuf::from(request.new_path),
                    cx,
                ),
            )
        })?;
        Ok(proto::ProjectEntryResponse {
            entry: match &task.await? {
                CreatedEntry::Included(entry) => Some(entry.into()),
                CreatedEntry::Excluded { .. } => None,
            },
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_copy_entry(
        this: Model<Self>,
        request: proto::CopyProjectEntry,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            let relative_worktree_source_path =
                request.relative_worktree_source_path.map(PathBuf::from);
            (
                this.scan_id(),
                this.copy_entry(
                    ProjectEntryId::from_proto(request.entry_id),
                    relative_worktree_source_path,
                    PathBuf::from(request.new_path),
                    cx,
                ),
            )
        })?;
        Ok(proto::ProjectEntryResponse {
            entry: task.await?.as_ref().map(|e| e.into()),
            worktree_scan_id: scan_id as u64,
        })
    }
}

impl LocalWorktree {
    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn contains_abs_path(&self, path: &Path) -> bool {
        path.starts_with(&self.abs_path)
    }

    pub fn is_path_private(&self, path: &Path) -> bool {
        !self.share_private_files && self.settings.is_path_private(path)
    }

    fn restart_background_scanners(&mut self, cx: &ModelContext<Worktree>) {
        let (scan_requests_tx, scan_requests_rx) = channel::unbounded();
        let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = channel::unbounded();
        self.scan_requests_tx = scan_requests_tx;
        self.path_prefixes_to_scan_tx = path_prefixes_to_scan_tx;
        self.start_background_scanner(scan_requests_rx, path_prefixes_to_scan_rx, cx);
    }

    fn start_background_scanner(
        &mut self,
        scan_requests_rx: channel::Receiver<ScanRequest>,
        path_prefixes_to_scan_rx: channel::Receiver<Arc<Path>>,
        cx: &ModelContext<Worktree>,
    ) {
        let snapshot = self.snapshot();
        let share_private_files = self.share_private_files;
        let next_entry_id = self.next_entry_id.clone();
        let fs = self.fs.clone();
        let git_hosting_provider_registry = GitHostingProviderRegistry::try_global(cx);
        let settings = self.settings.clone();
        let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded();
        let background_scanner = cx.background_executor().spawn({
            let abs_path = &snapshot.abs_path;
            let abs_path = if cfg!(target_os = "windows") {
                abs_path
                    .canonicalize()
                    .unwrap_or_else(|_| abs_path.to_path_buf())
            } else {
                abs_path.to_path_buf()
            };
            let background = cx.background_executor().clone();
            async move {
                let (events, watcher) = fs.watch(&abs_path, FS_WATCH_LATENCY).await;
                let fs_case_sensitive = fs.is_case_sensitive().await.unwrap_or_else(|e| {
                    log::error!("Failed to determine whether filesystem is case sensitive: {e:#}");
                    true
                });

                let mut scanner = BackgroundScanner {
                    fs,
                    fs_case_sensitive,
                    status_updates_tx: scan_states_tx,
                    executor: background,
                    scan_requests_rx,
                    path_prefixes_to_scan_rx,
                    next_entry_id,
                    state: Mutex::new(BackgroundScannerState {
                        prev_snapshot: snapshot.snapshot.clone(),
                        snapshot,
                        scanned_dirs: Default::default(),
                        path_prefixes_to_scan: Default::default(),
                        paths_to_scan: Default::default(),
                        removed_entries: Default::default(),
                        changed_paths: Default::default(),
                        git_hosting_provider_registry,
                    }),
                    phase: BackgroundScannerPhase::InitialScan,
                    share_private_files,
                    settings,
                    watcher,
                };

                scanner
                    .run(Box::pin(
                        events.map(|events| events.into_iter().map(Into::into).collect()),
                    ))
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
        self._background_scanner_tasks = vec![background_scanner, scan_state_updater];
        self.is_scanning = watch::channel_with(true);
    }

    fn set_snapshot(
        &mut self,
        new_snapshot: LocalSnapshot,
        entry_changes: UpdatedEntriesSet,
        cx: &mut ModelContext<Worktree>,
    ) {
        let repo_changes = self.changed_repos(&self.snapshot, &new_snapshot);
        self.snapshot = new_snapshot;

        if let Some(share) = self.update_observer.as_mut() {
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

    pub fn settings(&self) -> WorktreeSettings {
        self.settings.clone()
    }

    pub fn local_git_repo(&self, path: &Path) -> Option<Arc<dyn GitRepository>> {
        self.repo_for_path(path)
            .map(|(_, entry)| entry.repo_ptr.clone())
    }

    pub fn get_local_repo(&self, repo: &RepositoryEntry) -> Option<&LocalRepositoryEntry> {
        self.git_repositories.get(&repo.work_directory.0)
    }

    fn load_file(&self, path: &Path, cx: &ModelContext<Worktree>) -> Task<Result<LoadedFile>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let entry = self.refresh_entry(path.clone(), None, cx);
        let is_private = self.is_path_private(path.as_ref());

        cx.spawn(|this, mut cx| async move {
            let abs_path = abs_path?;
            let text = fs.load(&abs_path).await?;
            let mut index_task = None;
            let snapshot = this.update(&mut cx, |this, _| this.as_local().unwrap().snapshot())?;
            if let Some(repo) = snapshot.repository_for_path(&path) {
                if let Some(repo_path) = repo.relativize(&snapshot, &path).log_err() {
                    if let Some(git_repo) = snapshot.git_repositories.get(&*repo.work_directory) {
                        let git_repo = git_repo.repo_ptr.clone();
                        index_task = Some(
                            cx.background_executor()
                                .spawn(async move { git_repo.load_index_text(&repo_path) }),
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
            let file = match entry.await? {
                Some(entry) => File::for_entry(entry, worktree),
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
                    Arc::new(File {
                        entry_id: None,
                        worktree,
                        path,
                        mtime: Some(metadata.mtime),
                        is_local: true,
                        is_deleted: false,
                        is_private,
                    })
                }
            };

            Ok(LoadedFile {
                file,
                text,
                diff_base,
            })
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

    fn create_entry(
        &self,
        path: impl Into<Arc<Path>>,
        is_dir: bool,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let path = path.into();
        let abs_path = match self.absolutize(&path) {
            Ok(path) => path,
            Err(e) => return Task::ready(Err(e.context(format!("absolutizing path {path:?}")))),
        };
        let path_excluded = self.settings.is_path_excluded(&abs_path);
        let fs = self.fs.clone();
        let task_abs_path = abs_path.clone();
        let write = cx.background_executor().spawn(async move {
            if is_dir {
                fs.create_dir(&task_abs_path)
                    .await
                    .with_context(|| format!("creating directory {task_abs_path:?}"))
            } else {
                fs.save(&task_abs_path, &Rope::default(), LineEnding::default())
                    .await
                    .with_context(|| format!("creating file {task_abs_path:?}"))
            }
        });

        let lowest_ancestor = self.lowest_ancestor(&path);
        cx.spawn(|this, mut cx| async move {
            write.await?;
            if path_excluded {
                return Ok(CreatedEntry::Excluded { abs_path });
            }

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

            Ok(result
                .await?
                .map(CreatedEntry::Included)
                .unwrap_or_else(|| CreatedEntry::Excluded { abs_path }))
        })
    }

    fn write_file(
        &self,
        path: impl Into<Arc<Path>>,
        text: Rope,
        line_ending: LineEnding,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Arc<File>>> {
        let path = path.into();
        let fs = self.fs.clone();
        let is_private = self.is_path_private(&path);
        let Ok(abs_path) = self.absolutize(&path) else {
            return Task::ready(Err(anyhow!("invalid path {path:?}")));
        };

        let write = cx.background_executor().spawn({
            let fs = fs.clone();
            let abs_path = abs_path.clone();
            async move { fs.save(&abs_path, &text, line_ending).await }
        });

        cx.spawn(move |this, mut cx| async move {
            write.await?;
            let entry = this
                .update(&mut cx, |this, cx| {
                    this.as_local_mut()
                        .unwrap()
                        .refresh_entry(path.clone(), None, cx)
                })?
                .await?;
            let worktree = this.upgrade().ok_or_else(|| anyhow!("worktree dropped"))?;
            if let Some(entry) = entry {
                Ok(File::for_entry(entry, worktree))
            } else {
                let metadata = fs
                    .metadata(&abs_path)
                    .await
                    .with_context(|| {
                        format!("Fetching metadata after saving the excluded buffer {abs_path:?}")
                    })?
                    .with_context(|| {
                        format!("Excluded buffer {path:?} got removed during saving")
                    })?;
                Ok(Arc::new(File {
                    worktree,
                    path,
                    mtime: Some(metadata.mtime),
                    entry_id: None,
                    is_local: true,
                    is_deleted: false,
                    is_private,
                }))
            }
        })
    }

    fn delete_entry(
        &self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let entry = self.entry_for_id(entry_id)?.clone();
        let abs_path = self.absolutize(&entry.path);
        let fs = self.fs.clone();

        let delete = cx.background_executor().spawn(async move {
            if entry.is_file() {
                if trash {
                    fs.trash_file(&abs_path?, Default::default()).await?;
                } else {
                    fs.remove_file(&abs_path?, Default::default()).await?;
                }
            } else if trash {
                fs.trash_dir(
                    &abs_path?,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: false,
                    },
                )
                .await?;
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

    fn rename_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let old_path = match self.entry_for_id(entry_id) {
            Some(entry) => entry.path.clone(),
            None => return Task::ready(Err(anyhow!("no entry to rename for id {entry_id:?}"))),
        };
        let new_path = new_path.into();
        let abs_old_path = self.absolutize(&old_path);
        let Ok(abs_new_path) = self.absolutize(&new_path) else {
            return Task::ready(Err(anyhow!("absolutizing path {new_path:?}")));
        };
        let abs_path = abs_new_path.clone();
        let fs = self.fs.clone();
        let case_sensitive = self.fs_case_sensitive;
        let rename = cx.background_executor().spawn(async move {
            let abs_old_path = abs_old_path?;
            let abs_new_path = abs_new_path;

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
            .with_context(|| format!("Renaming {abs_old_path:?} into {abs_new_path:?}"))
        });

        cx.spawn(|this, mut cx| async move {
            rename.await?;
            Ok(this
                .update(&mut cx, |this, cx| {
                    this.as_local_mut()
                        .unwrap()
                        .refresh_entry(new_path.clone(), Some(old_path), cx)
                })?
                .await?
                .map(CreatedEntry::Included)
                .unwrap_or_else(|| CreatedEntry::Excluded { abs_path }))
        })
    }

    fn copy_entry(
        &self,
        entry_id: ProjectEntryId,
        relative_worktree_source_path: Option<PathBuf>,
        new_path: impl Into<Arc<Path>>,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        let old_path = match self.entry_for_id(entry_id) {
            Some(entry) => entry.path.clone(),
            None => return Task::ready(Ok(None)),
        };
        let new_path = new_path.into();
        let abs_old_path =
            if let Some(relative_worktree_source_path) = relative_worktree_source_path {
                Ok(self.abs_path().join(relative_worktree_source_path))
            } else {
                self.absolutize(&old_path)
            };
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

    pub fn copy_external_entries(
        &self,
        target_directory: PathBuf,
        paths: Vec<Arc<Path>>,
        overwrite_existing_files: bool,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Vec<ProjectEntryId>>> {
        let worktree_path = self.abs_path().clone();
        let fs = self.fs.clone();
        let paths = paths
            .into_iter()
            .filter_map(|source| {
                let file_name = source.file_name()?;
                let mut target = target_directory.clone();
                target.push(file_name);

                // Do not allow copying the same file to itself.
                if source.as_ref() != target.as_path() {
                    Some((source, target))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let paths_to_refresh = paths
            .iter()
            .filter_map(|(_, target)| Some(target.strip_prefix(&worktree_path).ok()?.into()))
            .collect::<Vec<_>>();

        cx.spawn(|this, cx| async move {
            cx.background_executor()
                .spawn(async move {
                    for (source, target) in paths {
                        copy_recursive(
                            fs.as_ref(),
                            &source,
                            &target,
                            fs::CopyOptions {
                                overwrite: overwrite_existing_files,
                                ..Default::default()
                            },
                        )
                        .await
                        .with_context(|| {
                            anyhow!("Failed to copy file from {source:?} to {target:?}")
                        })?;
                    }
                    Ok::<(), anyhow::Error>(())
                })
                .await
                .log_err();
            let mut refresh = cx.read_model(
                &this.upgrade().with_context(|| "Dropped worktree")?,
                |this, _| {
                    Ok::<postage::barrier::Receiver, anyhow::Error>(
                        this.as_local()
                            .with_context(|| "Worktree is not local")?
                            .refresh_entries_for_paths(paths_to_refresh.clone()),
                    )
                },
            )??;

            cx.background_executor()
                .spawn(async move {
                    refresh.next().await;
                    Ok::<(), anyhow::Error>(())
                })
                .await
                .log_err();

            let this = this.upgrade().with_context(|| "Dropped worktree")?;
            cx.read_model(&this, |this, _| {
                paths_to_refresh
                    .iter()
                    .filter_map(|path| Some(this.entry_for_path(path)?.id))
                    .collect()
            })
        })
    }

    fn expand_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let path = self.entry_for_id(entry_id)?.path.clone();
        let mut refresh = self.refresh_entries_for_paths(vec![path]);
        Some(cx.background_executor().spawn(async move {
            refresh.next().await;
            Ok(())
        }))
    }

    fn refresh_entries_for_paths(&self, paths: Vec<Arc<Path>>) -> barrier::Receiver {
        let (tx, rx) = barrier::channel();
        self.scan_requests_tx
            .try_send(ScanRequest {
                relative_paths: paths,
                done: smallvec![tx],
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
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Option<Entry>>> {
        if self.settings.is_path_excluded(&path) {
            return Task::ready(Ok(None));
        }
        let paths = if let Some(old_path) = old_path.as_ref() {
            vec![old_path.clone(), path.clone()]
        } else {
            vec![path.clone()]
        };
        let t0 = Instant::now();
        let mut refresh = self.refresh_entries_for_paths(paths);
        cx.spawn(move |this, mut cx| async move {
            refresh.recv().await;
            log::trace!("refreshed entry {path:?} in {:?}", t0.elapsed());
            let new_entry = this.update(&mut cx, |this, _| {
                this.entry_for_path(path)
                    .cloned()
                    .ok_or_else(|| anyhow!("failed to read path after update"))
            })??;
            Ok(Some(new_entry))
        })
    }

    fn observe_updates<F, Fut>(&mut self, project_id: u64, cx: &ModelContext<Worktree>, callback: F)
    where
        F: 'static + Send + Fn(proto::UpdateWorktree) -> Fut,
        Fut: Send + Future<Output = bool>,
    {
        if let Some(observer) = self.update_observer.as_mut() {
            *observer.resume_updates.borrow_mut() = ();
            return;
        }

        let (resume_updates_tx, mut resume_updates_rx) = watch::channel::<()>();
        let (snapshots_tx, mut snapshots_rx) =
            mpsc::unbounded::<(LocalSnapshot, UpdatedEntriesSet, UpdatedGitRepositoriesSet)>();
        snapshots_tx
            .unbounded_send((self.snapshot(), Arc::default(), Arc::default()))
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

                for update in proto::split_worktree_update(update) {
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
            Some(())
        });

        self.update_observer = Some(UpdateObservationState {
            snapshots_tx,
            resume_updates: resume_updates_tx,
            _maintain_remote_snapshot,
        });
    }

    pub fn share_private_files(&mut self, cx: &ModelContext<Worktree>) {
        self.share_private_files = true;
        self.restart_background_scanners(cx);
    }
}

impl RemoteWorktree {
    pub fn project_id(&self) -> u64 {
        self.project_id
    }

    pub fn client(&self) -> AnyProtoClient {
        self.client.clone()
    }

    pub fn disconnected_from_host(&mut self) {
        self.updates_tx.take();
        self.snapshot_subscriptions.clear();
        self.disconnected = true;
    }

    pub fn update_from_remote(&self, update: proto::UpdateWorktree) {
        if let Some(updates_tx) = &self.updates_tx {
            updates_tx
                .unbounded_send(update)
                .expect("consumer runs to completion");
        }
    }

    fn observe_updates<F, Fut>(&mut self, project_id: u64, cx: &ModelContext<Worktree>, callback: F)
    where
        F: 'static + Send + Fn(proto::UpdateWorktree) -> Fut,
        Fut: 'static + Send + Future<Output = bool>,
    {
        let (tx, mut rx) = mpsc::unbounded();
        let initial_update = self
            .snapshot
            .build_initial_update(project_id, self.id().to_proto());
        self.update_observer = Some(tx);
        cx.spawn(|this, mut cx| async move {
            let mut update = initial_update;
            'outer: loop {
                // SSH projects use a special project ID of 0, and we need to
                // remap it to the correct one here.
                update.project_id = project_id;

                for chunk in split_worktree_update(update) {
                    if !callback(chunk).await {
                        break 'outer;
                    }
                }

                if let Some(next_update) = rx.next().await {
                    update = next_update;
                } else {
                    break;
                }
            }
            this.update(&mut cx, |this, _| {
                let this = this.as_remote_mut().unwrap();
                this.update_observer.take();
            })
        })
        .detach();
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

    fn insert_entry(
        &mut self,
        entry: proto::Entry,
        scan_id: usize,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let wait_for_snapshot = self.wait_for_snapshot(scan_id);
        cx.spawn(|this, mut cx| async move {
            wait_for_snapshot.await?;
            this.update(&mut cx, |worktree, _| {
                let worktree = worktree.as_remote_mut().unwrap();
                let snapshot = &mut worktree.background_snapshot.lock().0;
                let entry = snapshot.insert_entry(entry);
                worktree.snapshot = snapshot.clone();
                entry
            })?
        })
    }

    fn delete_entry(
        &self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let response = self.client.request(proto::DeleteProjectEntry {
            project_id: self.project_id,
            entry_id: entry_id.to_proto(),
            use_trash: trash,
        });
        Some(cx.spawn(move |this, mut cx| async move {
            let response = response.await?;
            let scan_id = response.worktree_scan_id as usize;

            this.update(&mut cx, move |this, _| {
                this.as_remote_mut().unwrap().wait_for_snapshot(scan_id)
            })?
            .await?;

            this.update(&mut cx, |this, _| {
                let this = this.as_remote_mut().unwrap();
                let snapshot = &mut this.background_snapshot.lock().0;
                snapshot.delete_entry(entry_id);
                this.snapshot = snapshot.clone();
            })
        }))
    }

    fn rename_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &ModelContext<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let new_path = new_path.into();
        let response = self.client.request(proto::RenameProjectEntry {
            project_id: self.project_id,
            entry_id: entry_id.to_proto(),
            new_path: new_path.to_string_lossy().into(),
        });
        cx.spawn(move |this, mut cx| async move {
            let response = response.await?;
            match response.entry {
                Some(entry) => this
                    .update(&mut cx, |this, cx| {
                        this.as_remote_mut().unwrap().insert_entry(
                            entry,
                            response.worktree_scan_id as usize,
                            cx,
                        )
                    })?
                    .await
                    .map(CreatedEntry::Included),
                None => {
                    let abs_path = this.update(&mut cx, |worktree, _| {
                        worktree
                            .absolutize(&new_path)
                            .with_context(|| format!("absolutizing {new_path:?}"))
                    })??;
                    Ok(CreatedEntry::Excluded { abs_path })
                }
            }
        })
    }
}

impl Snapshot {
    pub fn new(id: u64, root_name: String, abs_path: Arc<Path>) -> Self {
        Snapshot {
            id: WorktreeId::from_usize(id as usize),
            abs_path,
            root_char_bag: root_name.chars().map(|c| c.to_ascii_lowercase()).collect(),
            root_name,
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
            repository_entries: Default::default(),
            scan_id: 1,
            completed_scan_id: 0,
        }
    }

    pub fn id(&self) -> WorktreeId {
        self.id
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
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
            let mut cursor = self.entries_by_path.cursor::<TraversalProgress>(&());
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
        log::trace!(
            "applying remote worktree update. {} entries updated, {} removed",
            update.updated_entries.len(),
            update.removed_entries.len()
        );

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
            update
                .removed_repositories
                .binary_search(&entry.work_directory.to_proto())
                .is_err()
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
                            // When syncing repository entries from a peer, we don't need
                            // the location_in_repo field, since git operations don't happen locally
                            // anyway.
                            location_in_repo: None,
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

    pub fn entry_count(&self) -> usize {
        self.entries_by_path.summary().count
    }

    pub fn visible_entry_count(&self) -> usize {
        self.entries_by_path.summary().non_ignored_count
    }

    pub fn dir_count(&self) -> usize {
        let summary = self.entries_by_path.summary();
        summary.count - summary.file_count
    }

    pub fn visible_dir_count(&self) -> usize {
        let summary = self.entries_by_path.summary();
        summary.non_ignored_count - summary.non_ignored_file_count
    }

    pub fn file_count(&self) -> usize {
        self.entries_by_path.summary().file_count
    }

    pub fn visible_file_count(&self) -> usize {
        self.entries_by_path.summary().non_ignored_file_count
    }

    fn traverse_from_offset(
        &self,
        include_files: bool,
        include_dirs: bool,
        include_ignored: bool,
        start_offset: usize,
    ) -> Traversal {
        let mut cursor = self.entries_by_path.cursor(&());
        cursor.seek(
            &TraversalTarget::Count {
                count: start_offset,
                include_files,
                include_dirs,
                include_ignored,
            },
            Bias::Right,
            &(),
        );
        Traversal {
            cursor,
            include_files,
            include_dirs,
            include_ignored,
        }
    }

    pub fn traverse_from_path(
        &self,
        include_files: bool,
        include_dirs: bool,
        include_ignored: bool,
        path: &Path,
    ) -> Traversal {
        Traversal::new(
            &self.entries_by_path,
            include_files,
            include_dirs,
            include_ignored,
            path,
        )
    }

    pub fn files(&self, include_ignored: bool, start: usize) -> Traversal {
        self.traverse_from_offset(true, false, include_ignored, start)
    }

    pub fn directories(&self, include_ignored: bool, start: usize) -> Traversal {
        self.traverse_from_offset(false, true, include_ignored, start)
    }

    pub fn entries(&self, include_ignored: bool, start: usize) -> Traversal {
        self.traverse_from_offset(true, true, include_ignored, start)
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
                if entry.path.starts_with(repo_path) {
                    break;
                } else {
                    containing_repos.pop();
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
            .cursor::<(TraversalProgress, GitStatuses)>(&());
        let mut entry_stack = Vec::<(usize, GitStatuses)>::new();

        let mut result_ix = 0;
        loop {
            let next_entry = result.get(result_ix);
            let containing_entry = entry_stack.last().map(|(ix, _)| &result[*ix]);

            let entry_to_finish = match (containing_entry, next_entry) {
                (Some(_), None) => entry_stack.pop(),
                (Some(containing_entry), Some(next_path)) => {
                    if next_path.path.starts_with(&containing_entry.path) {
                        None
                    } else {
                        entry_stack.pop()
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
            .cursor::<()>(&())
            .filter(move |entry| entry.path.as_ref() != empty_path)
            .map(|entry| &entry.path)
    }

    pub fn child_entries<'a>(&'a self, parent_path: &'a Path) -> ChildEntriesIter<'a> {
        let mut cursor = self.entries_by_path.cursor(&());
        cursor.seek(&TraversalTarget::Path(parent_path), Bias::Right, &());
        let traversal = Traversal {
            cursor,
            include_files: true,
            include_dirs: true,
            include_ignored: true,
        };
        ChildEntriesIter {
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

    pub fn git_entry(&self, work_directory_path: Arc<Path>) -> Option<RepositoryEntry> {
        self.repository_entries
            .get(&RepositoryWorkDirectory(work_directory_path))
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
        self.traverse_from_path(true, true, true, path)
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
    pub fn repo_for_path(&self, path: &Path) -> Option<(RepositoryEntry, &LocalRepositoryEntry)> {
        let (_, repo_entry) = self.repository_and_work_directory_for_path(path)?;
        let work_directory_id = repo_entry.work_directory_id();
        Some((repo_entry, self.git_repositories.get(&work_directory_id)?))
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
            if ancestor.join(*DOT_GIT).exists() {
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
            .cursor::<()>(&())
            .filter(|entry| entry.kind == EntryKind::Dir && (entry.is_external || entry.is_ignored))
    }

    #[cfg(test)]
    pub fn check_invariants(&self, git_state: bool) {
        use pretty_assertions::assert_eq;

        assert_eq!(
            self.entries_by_path
                .cursor::<()>(&())
                .map(|e| (&e.path, e.id))
                .collect::<Vec<_>>(),
            self.entries_by_id
                .cursor::<()>(&())
                .map(|e| (&e.path, e.id))
                .collect::<collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            "entries_by_path and entries_by_id are inconsistent"
        );

        let mut files = self.files(true, 0);
        let mut visible_files = self.files(false, 0);
        for entry in self.entries_by_path.cursor::<()>(&()) {
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
            .cursor::<()>(&())
            .map(|e| e.path.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(bfs_paths, dfs_paths_via_iter);

        let dfs_paths_via_traversal = self
            .entries(true, 0)
            .map(|e| e.path.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(dfs_paths_via_traversal, dfs_paths_via_iter);

        if git_state {
            for ignore_parent_abs_path in self.ignores_by_parent_abs_path.keys() {
                let ignore_parent_path =
                    ignore_parent_abs_path.strip_prefix(&self.abs_path).unwrap();
                assert!(self.entry_for_path(ignore_parent_path).is_some());
                assert!(self
                    .entry_for_path(ignore_parent_path.join(*GITIGNORE))
                    .is_some());
            }
        }
    }

    #[cfg(test)]
    fn check_git_invariants(&self) {
        let dotgit_paths = self
            .git_repositories
            .iter()
            .map(|repo| repo.1.git_dir_abs_path.clone())
            .collect::<HashSet<_>>();
        let work_dir_paths = self
            .repository_entries
            .iter()
            .map(|repo| repo.0.clone().0)
            .collect::<HashSet<_>>();
        assert_eq!(dotgit_paths.len(), work_dir_paths.len());
        assert_eq!(self.repository_entries.iter().count(), work_dir_paths.len());
        assert_eq!(self.git_repositories.iter().count(), work_dir_paths.len());
        for (_, entry) in self.repository_entries.iter() {
            self.git_repositories.get(&entry.work_directory).unwrap();
        }
    }

    #[cfg(test)]
    pub fn entries_without_ids(&self, include_ignored: bool) -> Vec<(&Path, u64, bool)> {
        let mut paths = Vec::new();
        for entry in self.entries_by_path.cursor::<()>(&()) {
            if include_ignored || !entry.is_ignored {
                paths.push((entry.path.as_ref(), entry.inode, entry.is_ignored));
            }
        }
        paths.sort_by(|a, b| a.0.cmp(b.0));
        paths
    }
}

impl BackgroundScannerState {
    fn should_scan_directory(&self, entry: &Entry) -> bool {
        (!entry.is_external && !entry.is_ignored)
            || entry.path.file_name() == Some(*DOT_GIT)
            || entry.path.file_name() == Some(local_settings_folder_relative_path().as_os_str())
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
            if let Some((repo_entry, repo)) = self.snapshot.repo_for_path(&path) {
                if let Some(workdir_path) = repo_entry.work_directory(&self.snapshot) {
                    if let Ok(repo_path) = repo_entry.relativize(&self.snapshot, &path) {
                        containing_repository = Some(ScanJobContainingRepository {
                            work_directory: workdir_path,
                            statuses: repo
                                .repo_ptr
                                .status(&[repo_path.0])
                                .log_err()
                                .unwrap_or_default(),
                        });
                    }
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
        if let Some(mtime) = entry.mtime {
            // If an entry with the same inode was removed from the worktree during this scan,
            // then it *might* represent the same file or directory. But the OS might also have
            // re-used the inode for a completely different file or directory.
            //
            // Conditionally reuse the old entry's id:
            // * if the mtime is the same, the file was probably been renamed.
            // * if the path is the same, the file may just have been updated
            if let Some(removed_entry) = self.removed_entries.remove(&entry.inode) {
                if removed_entry.mtime == Some(mtime) || removed_entry.path == entry.path {
                    entry.id = removed_entry.id;
                }
            } else if let Some(existing_entry) = self.snapshot.entry_for_path(&entry.path) {
                entry.id = existing_entry.id;
            }
        }
    }

    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs, watcher: &dyn Watcher) -> Entry {
        self.reuse_entry_id(&mut entry);
        let entry = self.snapshot.insert_entry(entry, fs);
        if entry.path.file_name() == Some(&DOT_GIT) {
            self.insert_git_repository(entry.path.clone(), fs, watcher);
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
            let abs_parent_path = self.snapshot.abs_path.join(parent_path).into();
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
            let mut cursor = self
                .snapshot
                .entries_by_path
                .cursor::<TraversalProgress>(&());
            new_entries = cursor.slice(&TraversalTarget::Path(path), Bias::Left, &());
            removed_entries = cursor.slice(&TraversalTarget::PathSuccessor(path), Bias::Left, &());
            new_entries.append(cursor.suffix(&()), &());
        }
        self.snapshot.entries_by_path = new_entries;

        let mut removed_ids = Vec::with_capacity(removed_entries.summary().count);
        for entry in removed_entries.cursor::<()>(&()) {
            match self.removed_entries.entry(entry.inode) {
                hash_map::Entry::Occupied(mut e) => {
                    let prev_removed_entry = e.get_mut();
                    if entry.id > prev_removed_entry.id {
                        *prev_removed_entry = entry.clone();
                    }
                }
                hash_map::Entry::Vacant(e) => {
                    e.insert(entry.clone());
                }
            }

            if entry.path.file_name() == Some(&GITIGNORE) {
                let abs_parent_path = self.snapshot.abs_path.join(entry.path.parent().unwrap());
                if let Some((_, needs_update)) = self
                    .snapshot
                    .ignores_by_parent_abs_path
                    .get_mut(abs_parent_path.as_path())
                {
                    *needs_update = true;
                }
            }

            if let Err(ix) = removed_ids.binary_search(&entry.id) {
                removed_ids.insert(ix, entry.id);
            }
        }

        self.snapshot.entries_by_id.edit(
            removed_ids.iter().map(|&id| Edit::Remove(id)).collect(),
            &(),
        );
        self.snapshot
            .git_repositories
            .retain(|id, _| removed_ids.binary_search(id).is_err());
        self.snapshot
            .repository_entries
            .retain(|repo_path, _| !repo_path.0.starts_with(path));

        #[cfg(test)]
        self.snapshot.check_invariants(false);
    }

    fn insert_git_repository(
        &mut self,
        dot_git_path: Arc<Path>,
        fs: &dyn Fs,
        watcher: &dyn Watcher,
    ) -> Option<(RepositoryWorkDirectory, Arc<dyn GitRepository>)> {
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

        self.insert_git_repository_for_path(work_dir_path, dot_git_path, None, fs, watcher)
    }

    fn insert_git_repository_for_path(
        &mut self,
        work_dir_path: Arc<Path>,
        dot_git_path: Arc<Path>,
        location_in_repo: Option<Arc<Path>>,
        fs: &dyn Fs,
        watcher: &dyn Watcher,
    ) -> Option<(RepositoryWorkDirectory, Arc<dyn GitRepository>)> {
        let work_dir_id = self
            .snapshot
            .entry_for_path(work_dir_path.clone())
            .map(|entry| entry.id)?;

        if self.snapshot.git_repositories.get(&work_dir_id).is_some() {
            return None;
        }

        let dot_git_abs_path = self.snapshot.abs_path.join(&dot_git_path);
        let t0 = Instant::now();
        let repository = fs.open_repo(&dot_git_abs_path)?;
        let git_dir_path = Arc::from(
            repository
                .path()
                .ancestors()
                .find(|ancestor| ancestor.file_name() == Some(&*DOT_GIT))?,
        );

        watcher.add(&git_dir_path).log_err()?;
        if git_dir_path.as_ref() != dot_git_abs_path {
            // The two paths could be different because we opened a git worktree.
            // When that happens, the .git path in the worktree (`dot_git_abs_path`) is a file that
            // points to the worktree-subdirectory in the actual .git directory (`git_dir_path`)
            watcher.add(&dot_git_abs_path).log_err()?;
        }

        log::trace!("constructed libgit2 repo in {:?}", t0.elapsed());
        let work_directory = RepositoryWorkDirectory(work_dir_path.clone());

        if let Some(git_hosting_provider_registry) = self.git_hosting_provider_registry.clone() {
            git_hosting_providers::register_additional_providers(
                git_hosting_provider_registry,
                repository.clone(),
            );
        }

        self.snapshot.repository_entries.insert(
            work_directory.clone(),
            RepositoryEntry {
                work_directory: work_dir_id.into(),
                branch: repository.branch_name().map(Into::into),
                location_in_repo,
            },
        );
        self.snapshot.git_repositories.insert(
            work_dir_id,
            LocalRepositoryEntry {
                git_dir_scan_id: 0,
                repo_ptr: repository.clone(),
                git_dir_abs_path: git_dir_path,
            },
        );

        Some((work_directory, repository))
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

            if worktree.is_local() && path.starts_with(home_dir().as_path()) {
                full_path.push("~");
                full_path.push(path.strip_prefix(home_dir().as_path()).unwrap());
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

    fn worktree_id(&self, cx: &AppContext) -> WorktreeId {
        self.worktree.read(cx).id()
    }

    fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_proto(&self, cx: &AppContext) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: self.worktree.read(cx).id().to_proto(),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entry {
    pub id: ProjectEntryId,
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub inode: u64,
    pub mtime: Option<SystemTime>,

    pub canonical_path: Option<Box<Path>>,
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
    /// The entry's size on disk, in bytes.
    pub size: u64,
    pub char_bag: CharBag,
    pub is_fifo: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryKind {
    UnloadedDir,
    PendingDir,
    Dir,
    File,
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
        canonical_path: Option<Box<Path>>,
    ) -> Self {
        let char_bag = char_bag_for_path(root_char_bag, &path);
        Self {
            id: ProjectEntryId::new(next_entry_id),
            kind: if metadata.is_dir {
                EntryKind::PendingDir
            } else {
                EntryKind::File
            },
            path,
            inode: metadata.inode,
            mtime: Some(metadata.mtime),
            size: metadata.len,
            canonical_path,
            is_ignored: false,
            is_external: false,
            is_private: false,
            git_status: None,
            char_bag,
            is_fifo: metadata.is_fifo,
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
        matches!(self, EntryKind::File)
    }
}

impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self, _cx: &()) -> Self::Summary {
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
        if let Some(status) = self.git_status {
            match status {
                GitFileStatus::Added => statuses.added = 1,
                GitFileStatus::Modified => statuses.modified = 1,
                GitFileStatus::Conflict => statuses.conflict = 1,
            }
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

    fn zero(_cx: &()) -> Self {
        Default::default()
    }

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

    fn summary(&self, _cx: &()) -> Self::Summary {
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

    fn zero(_cx: &Self::Context) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.max_id = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, PathEntrySummary> for ProjectEntryId {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

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
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

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
    watcher: Arc<dyn Watcher>,
    settings: WorktreeSettings,
    share_private_files: bool,
}

#[derive(PartialEq)]
enum BackgroundScannerPhase {
    InitialScan,
    EventsReceivedDuringInitialScan,
    Events,
}

impl BackgroundScanner {
    async fn run(&mut self, mut fs_events_rx: Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>) {
        use futures::FutureExt as _;

        // If the worktree root does not contain a git repository, then find
        // the git repository in an ancestor directory. Find any gitignore files
        // in ancestor directories.
        let root_abs_path = self.state.lock().snapshot.abs_path.clone();
        for (index, ancestor) in root_abs_path.ancestors().enumerate() {
            if index != 0 {
                if let Ok(ignore) =
                    build_gitignore(&ancestor.join(*GITIGNORE), self.fs.as_ref()).await
                {
                    self.state
                        .lock()
                        .snapshot
                        .ignores_by_parent_abs_path
                        .insert(ancestor.into(), (ignore.into(), false));
                }
            }

            let ancestor_dot_git = ancestor.join(*DOT_GIT);
            // Check whether the directory or file called `.git` exists (in the
            // case of worktrees it's a file.)
            if self
                .fs
                .metadata(&ancestor_dot_git)
                .await
                .is_ok_and(|metadata| metadata.is_some())
            {
                if index != 0 {
                    // We canonicalize, since the FS events use the canonicalized path.
                    if let Some(ancestor_dot_git) =
                        self.fs.canonicalize(&ancestor_dot_git).await.log_err()
                    {
                        // We associate the external git repo with our root folder and
                        // also mark where in the git repo the root folder is located.
                        self.state.lock().insert_git_repository_for_path(
                            Path::new("").into(),
                            ancestor_dot_git.into(),
                            Some(root_abs_path.strip_prefix(ancestor).unwrap().into()),
                            self.fs.as_ref(),
                            self.watcher.as_ref(),
                        );
                    };
                }

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
                    state.insert_entry(root_entry.clone(), self.fs.as_ref(), self.watcher.as_ref());
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

        self.send_status_update(false, SmallVec::new());

        // Process any any FS events that occurred while performing the initial scan.
        // For these events, update events cannot be as precise, because we didn't
        // have the previous state loaded yet.
        self.phase = BackgroundScannerPhase::EventsReceivedDuringInitialScan;
        if let Poll::Ready(Some(mut paths)) = futures::poll!(fs_events_rx.next()) {
            while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_events_rx.next()) {
                paths.extend(more_paths);
            }
            self.process_events(paths.into_iter().map(Into::into).collect())
                .await;
        }

        // Continue processing events until the worktree is dropped.
        self.phase = BackgroundScannerPhase::Events;

        loop {
            select_biased! {
                // Process any path refresh requests from the worktree. Prioritize
                // these before handling changes reported by the filesystem.
                request = self.next_scan_request().fuse() => {
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
                    self.process_events(paths.into_iter().map(Into::into).collect()).await;
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

        {
            let mut state = self.state.lock();
            let is_idle = state.snapshot.completed_scan_id == state.snapshot.scan_id;
            state.snapshot.scan_id += 1;
            if is_idle {
                state.snapshot.completed_scan_id = state.snapshot.scan_id;
            }
        }

        self.reload_entries_for_paths(
            root_path,
            root_canonical_path,
            &request.relative_paths,
            abs_paths,
            None,
        )
        .await;

        self.send_status_update(scanning, request.done)
    }

    async fn process_events(&self, mut abs_paths: Vec<PathBuf>) {
        let root_path = self.state.lock().snapshot.abs_path.clone();
        let root_canonical_path = match self.fs.canonicalize(&root_path).await {
            Ok(path) => path,
            Err(err) => {
                log::error!("failed to canonicalize root path: {}", err);
                return;
            }
        };

        let mut relative_paths = Vec::with_capacity(abs_paths.len());
        let mut dot_git_abs_paths = Vec::new();
        abs_paths.sort_unstable();
        abs_paths.dedup_by(|a, b| a.starts_with(b));
        abs_paths.retain(|abs_path| {
            let snapshot = &self.state.lock().snapshot;
            {
                let mut is_git_related = false;

                // We don't want to trigger .git rescan for events within .git/fsmonitor--daemon/cookies directory.
                #[derive(PartialEq)]
                enum FsMonitorParseState {
                    Cookies,
                    FsMonitor
                }
                let mut fsmonitor_parse_state = None;
                if let Some(dot_git_abs_path) = abs_path
                    .ancestors()
                    .find(|ancestor| {
                        let file_name = ancestor.file_name();
                        if file_name == Some(*COOKIES) {
                            fsmonitor_parse_state = Some(FsMonitorParseState::Cookies);
                            false
                        } else if fsmonitor_parse_state == Some(FsMonitorParseState::Cookies) && file_name == Some(*FSMONITOR_DAEMON) {
                            fsmonitor_parse_state = Some(FsMonitorParseState::FsMonitor);
                            false
                        } else if fsmonitor_parse_state != Some(FsMonitorParseState::FsMonitor) && file_name == Some(*DOT_GIT) {
                            true
                        } else {
                            fsmonitor_parse_state.take();
                            false
                        }

                    })
                {
                    let dot_git_abs_path = dot_git_abs_path.to_path_buf();
                    if !dot_git_abs_paths.contains(&dot_git_abs_path) {
                        dot_git_abs_paths.push(dot_git_abs_path);
                    }
                    is_git_related = true;
                }

                let relative_path: Arc<Path> =
                    if let Ok(path) = abs_path.strip_prefix(&root_canonical_path) {
                        path.into()
                    } else {
                        if is_git_related {
                            log::debug!(
                              "ignoring event {abs_path:?}, since it's in git dir outside of root path {root_canonical_path:?}",
                            );
                        } else {
                            log::error!(
                              "ignoring event {abs_path:?} outside of root path {root_canonical_path:?}",
                            );
                        }
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

                if self.settings.is_path_excluded(&relative_path) {
                    if !is_git_related {
                        log::debug!("ignoring FS event for excluded path {relative_path:?}");
                    }
                    return false;
                }

                relative_paths.push(relative_path);
                true
            }
        });

        if relative_paths.is_empty() && dot_git_abs_paths.is_empty() {
            return;
        }

        self.state.lock().snapshot.scan_id += 1;

        let (scan_job_tx, scan_job_rx) = channel::unbounded();
        log::debug!("received fs events {:?}", relative_paths);
        self.reload_entries_for_paths(
            root_path,
            root_canonical_path,
            &relative_paths,
            abs_paths,
            Some(scan_job_tx.clone()),
        )
        .await;

        self.update_ignore_statuses(scan_job_tx).await;
        self.scan_dirs(false, scan_job_rx).await;

        if !dot_git_abs_paths.is_empty() {
            self.update_git_repositories(dot_git_abs_paths).await;
        }

        {
            let mut state = self.state.lock();
            state.snapshot.completed_scan_id = state.snapshot.scan_id;
            for (_, entry) in mem::take(&mut state.removed_entries) {
                state.scanned_dirs.remove(&entry.id);
            }
        }

        #[cfg(test)]
        self.state.lock().snapshot.check_git_invariants();

        self.send_status_update(false, SmallVec::new());
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

        !mem::take(&mut self.state.lock().paths_to_scan).is_empty()
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
                                request = self.next_scan_request().fuse() => {
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
                                            self.send_status_update(true, SmallVec::new());
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

    fn send_status_update(&self, scanning: bool, barrier: SmallVec<[barrier::Sender; 1]>) -> bool {
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
        let root_char_bag;
        {
            let snapshot = &self.state.lock().snapshot;
            if self.settings.is_path_excluded(&job.path) {
                log::error!("skipping excluded directory {:?}", job.path);
                return Ok(());
            }
            log::debug!("scanning directory {:?}", job.path);
            root_abs_path = snapshot.abs_path().clone();
            root_char_bag = snapshot.root_char_bag;
        }

        let next_entry_id = self.next_entry_id.clone();
        let mut ignore_stack = job.ignore_stack.clone();
        let mut containing_repository = job.containing_repository.clone();
        let mut new_ignore = None;
        let mut root_canonical_path = None;
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();
        let mut child_paths = self
            .fs
            .read_dir(&job.abs_path)
            .await?
            .filter_map(|entry| async {
                match entry {
                    Ok(entry) => Some(entry),
                    Err(error) => {
                        log::error!("error processing entry {:?}", error);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
            .await;

        // Ensure that .git and .gitignore are processed first.
        swap_to_front(&mut child_paths, *GITIGNORE);
        swap_to_front(&mut child_paths, *DOT_GIT);

        for child_abs_path in child_paths {
            let child_abs_path: Arc<Path> = child_abs_path.into();
            let child_name = child_abs_path.file_name().unwrap();
            let child_path: Arc<Path> = job.path.join(child_name).into();

            if child_name == *DOT_GIT {
                let repo = self.state.lock().insert_git_repository(
                    child_path.clone(),
                    self.fs.as_ref(),
                    self.watcher.as_ref(),
                );

                if let Some((work_directory, repository)) = repo {
                    let t0 = Instant::now();
                    let statuses = repository
                        .status(&[PathBuf::from("")])
                        .log_err()
                        .unwrap_or_default();
                    log::trace!("computed git status in {:?}", t0.elapsed());
                    containing_repository = Some(ScanJobContainingRepository {
                        work_directory,
                        statuses,
                    });
                }
            } else if child_name == *GITIGNORE {
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
            }

            if self.settings.is_path_excluded(&child_path) {
                log::debug!("skipping excluded child entry {child_path:?}");
                self.state.lock().remove_path(&child_path);
                continue;
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
                None,
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

                child_entry.canonical_path = Some(canonical_path.into());
            }

            if child_entry.is_dir() {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, true);

                // Avoid recursing until crash in the case of a recursive symlink
                if job.ancestor_inodes.contains(&child_entry.inode) {
                    new_jobs.push(None);
                } else {
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
                        containing_repository: containing_repository.clone(),
                    }));
                }
            } else {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, false);
                if !child_entry.is_ignored {
                    if let Some(repo) = &containing_repository {
                        if let Ok(repo_path) = child_entry.path.strip_prefix(&repo.work_directory) {
                            let repo_path = RepoPath(repo_path.into());
                            child_entry.git_status = repo.statuses.get(&repo_path);
                        }
                    }
                }
            }

            {
                let relative_path = job.path.join(child_name);
                if self.is_path_private(&relative_path) {
                    log::debug!("detected private file: {relative_path:?}");
                    child_entry.is_private = true;
                }
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
        self.watcher.add(job.abs_path.as_ref()).log_err();

        for new_job in new_jobs.into_iter().flatten() {
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
        let doing_recursive_update = scan_queue_tx.is_some();

        // Remove any entries for paths that no longer exist or are being recursively
        // refreshed. Do this before adding any new entries, so that renames can be
        // detected regardless of the order of the paths.
        for (path, metadata) in relative_paths.iter().zip(metadata.iter()) {
            if matches!(metadata, Ok(None)) || doing_recursive_update {
                log::trace!("remove path {:?}", path);
                state.remove_path(path);
            }
        }

        // Group all relative paths by their git repository.
        let mut paths_by_git_repo = HashMap::default();
        for relative_path in relative_paths.iter() {
            if let Some((repo_entry, repo)) = state.snapshot.repo_for_path(relative_path) {
                if let Ok(repo_path) = repo_entry.relativize(&state.snapshot, relative_path) {
                    paths_by_git_repo
                        .entry(repo.git_dir_abs_path.clone())
                        .or_insert_with(|| RepoPaths {
                            repo: repo.repo_ptr.clone(),
                            repo_paths: Vec::new(),
                            relative_paths: Vec::new(),
                        })
                        .add_paths(relative_path, repo_path);
                }
            }
        }

        // Now call `git status` once per repository and collect each file's git status.
        let mut git_statuses_by_relative_path =
            paths_by_git_repo
                .into_values()
                .fold(HashMap::default(), |mut map, repo_paths| {
                    map.extend(repo_paths.into_git_file_statuses());
                    map
                });

        for (path, metadata) in relative_paths.iter().zip(metadata.into_iter()) {
            let abs_path: Arc<Path> = root_abs_path.join(path).into();
            match metadata {
                Ok(Some((metadata, canonical_path))) => {
                    let ignore_stack = state
                        .snapshot
                        .ignore_stack_for_abs_path(&abs_path, metadata.is_dir);
                    let is_external = !canonical_path.starts_with(&root_canonical_path);
                    let mut fs_entry = Entry::new(
                        path.clone(),
                        &metadata,
                        self.next_entry_id.as_ref(),
                        state.snapshot.root_char_bag,
                        if metadata.is_symlink {
                            Some(canonical_path.into())
                        } else {
                            None
                        },
                    );

                    let is_dir = fs_entry.is_dir();
                    fs_entry.is_ignored = ignore_stack.is_abs_path_ignored(&abs_path, is_dir);
                    fs_entry.is_external = is_external;
                    fs_entry.is_private = self.is_path_private(path);

                    if let (Some(scan_queue_tx), true) = (&scan_queue_tx, is_dir) {
                        if state.should_scan_directory(&fs_entry)
                            || (fs_entry.path.as_os_str().is_empty()
                                && abs_path.file_name() == Some(*DOT_GIT))
                        {
                            state.enqueue_scan_dir(abs_path, &fs_entry, scan_queue_tx);
                        } else {
                            fs_entry.kind = EntryKind::UnloadedDir;
                        }
                    }

                    if !is_dir && !fs_entry.is_ignored && !fs_entry.is_external {
                        fs_entry.git_status = git_statuses_by_relative_path.remove(path);
                    }

                    state.insert_entry(fs_entry.clone(), self.fs.as_ref(), self.watcher.as_ref());
                }
                Ok(None) => {
                    self.remove_repo_path(path, &mut state.snapshot);
                }
                Err(err) => {
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

        Some(())
    }

    async fn update_ignore_statuses(&self, scan_job_tx: Sender<ScanJob>) {
        use futures::FutureExt as _;

        let mut ignores_to_update = Vec::new();
        let (ignore_queue_tx, ignore_queue_rx) = channel::unbounded();
        let prev_snapshot;
        {
            let snapshot = &mut self.state.lock().snapshot;
            let abs_path = snapshot.abs_path.clone();
            snapshot
                .ignores_by_parent_abs_path
                .retain(|parent_abs_path, (_, needs_update)| {
                    if let Ok(parent_path) = parent_abs_path.strip_prefix(&abs_path) {
                        if *needs_update {
                            *needs_update = false;
                            if snapshot.snapshot.entry_for_path(parent_path).is_some() {
                                ignores_to_update.push(parent_abs_path.clone());
                            }
                        }

                        let ignore_path = parent_path.join(*GITIGNORE);
                        if snapshot.snapshot.entry_for_path(ignore_path).is_none() {
                            return false;
                        }
                    }
                    true
                });

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
                ignore_queue_tx
                    .send_blocking(UpdateIgnoreStatusJob {
                        abs_path: parent_abs_path,
                        ignore_stack,
                        ignore_queue: ignore_queue_tx.clone(),
                        scan_queue: scan_job_tx.clone(),
                    })
                    .unwrap();
            }

            prev_snapshot = snapshot.clone();
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
                                request = self.next_scan_request().fuse() => {
                                    let Ok(request) = request else { break };
                                    if !self.process_scan_request(request, true).await {
                                        return;
                                    }
                                }

                                // Recursively process directories whose ignores have changed.
                                job = ignore_queue_rx.recv().fuse() => {
                                    let Ok(job) = job else { break };
                                    self.update_ignore_status(job, &prev_snapshot).await;
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
        let repo = snapshot.repo_for_path(path);
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
                    if let Some((ref repo_entry, local_repo)) = repo {
                        if let Ok(repo_path) = repo_entry.relativize(snapshot, &entry.path) {
                            let status = local_repo
                                .repo_ptr
                                .status(&[repo_path.0.clone()])
                                .ok()
                                .and_then(|status| status.get(&repo_path));
                            entry.git_status = status;
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

    async fn update_git_repositories(&self, dot_git_paths: Vec<PathBuf>) {
        log::debug!("reloading repositories: {dot_git_paths:?}");

        let mut repo_updates = Vec::new();
        {
            let mut state = self.state.lock();
            let scan_id = state.snapshot.scan_id;
            for dot_git_dir in dot_git_paths {
                let existing_repository_entry =
                    state
                        .snapshot
                        .git_repositories
                        .iter()
                        .find_map(|(entry_id, repo)| {
                            (repo.git_dir_abs_path.as_ref() == &dot_git_dir)
                                .then(|| (*entry_id, repo.clone()))
                        });

                let (work_directory, repository) = match existing_repository_entry {
                    None => {
                        match state.insert_git_repository(
                            dot_git_dir.into(),
                            self.fs.as_ref(),
                            self.watcher.as_ref(),
                        ) {
                            Some(output) => output,
                            None => continue,
                        }
                    }
                    Some((entry_id, repository)) => {
                        if repository.git_dir_scan_id == scan_id {
                            continue;
                        }
                        let Some(work_dir) = state
                            .snapshot
                            .entry_for_id(entry_id)
                            .map(|entry| RepositoryWorkDirectory(entry.path.clone()))
                        else {
                            continue;
                        };

                        let repo = &repository.repo_ptr;
                        let branch = repo.branch_name();
                        repo.reload_index();

                        state
                            .snapshot
                            .git_repositories
                            .update(&entry_id, |entry| entry.git_dir_scan_id = scan_id);
                        state
                            .snapshot
                            .snapshot
                            .repository_entries
                            .update(&work_dir, |entry| entry.branch = branch.map(Into::into));
                        (work_dir, repository.repo_ptr.clone())
                    }
                };

                repo_updates.push(UpdateGitStatusesJob {
                    location_in_repo: state
                        .snapshot
                        .repository_entries
                        .get(&work_directory)
                        .and_then(|repo| repo.location_in_repo.clone())
                        .clone(),
                    work_directory,
                    repository,
                });
            }

            // Remove any git repositories whose .git entry no longer exists.
            let snapshot = &mut state.snapshot;
            let mut ids_to_preserve = HashSet::default();
            for (&work_directory_id, entry) in snapshot.git_repositories.iter() {
                let exists_in_snapshot = snapshot
                    .entry_for_id(work_directory_id)
                    .map_or(false, |entry| {
                        snapshot.entry_for_path(entry.path.join(*DOT_GIT)).is_some()
                    });

                if exists_in_snapshot
                    || matches!(
                        smol::block_on(self.fs.metadata(&entry.git_dir_abs_path)),
                        Ok(Some(_))
                    )
                {
                    ids_to_preserve.insert(work_directory_id);
                }
            }

            snapshot
                .git_repositories
                .retain(|work_directory_id, _| ids_to_preserve.contains(work_directory_id));
            snapshot
                .repository_entries
                .retain(|_, entry| ids_to_preserve.contains(&entry.work_directory.0));
        }

        let (mut updates_done_tx, mut updates_done_rx) = barrier::channel();
        self.executor
            .scoped(|scope| {
                scope.spawn(async {
                    for repo_update in repo_updates {
                        self.update_git_statuses(repo_update);
                    }
                    updates_done_tx.blocking_send(()).ok();
                });

                scope.spawn(async {
                    loop {
                        select_biased! {
                            // Process any path refresh requests before moving on to process
                            // the queue of git statuses.
                            request = self.next_scan_request().fuse() => {
                                let Ok(request) = request else { break };
                                if !self.process_scan_request(request, true).await {
                                    return;
                                }
                            }
                            _ = updates_done_rx.recv().fuse() =>  break,
                        }
                    }
                });
            })
            .await;
    }

    /// Update the git statuses for a given batch of entries.
    fn update_git_statuses(&self, job: UpdateGitStatusesJob) {
        log::trace!("updating git statuses for repo {:?}", job.work_directory.0);
        let t0 = Instant::now();
        let Some(statuses) = job.repository.status(&[PathBuf::from("")]).log_err() else {
            return;
        };
        log::trace!(
            "computed git statuses for repo {:?} in {:?}",
            job.work_directory.0,
            t0.elapsed()
        );

        let t0 = Instant::now();
        let mut changes = Vec::new();
        let snapshot = self.state.lock().snapshot.snapshot.clone();
        for file in snapshot.traverse_from_path(true, false, false, job.work_directory.0.as_ref()) {
            let Ok(repo_path) = file.path.strip_prefix(&job.work_directory.0) else {
                break;
            };
            let git_status = if let Some(location) = &job.location_in_repo {
                statuses.get(&location.join(repo_path))
            } else {
                statuses.get(repo_path)
            };
            if file.git_status != git_status {
                let mut entry = file.clone();
                entry.git_status = git_status;
                changes.push((entry.path, git_status));
            }
        }

        let mut state = self.state.lock();
        let edits = changes
            .iter()
            .filter_map(|(path, git_status)| {
                let entry = state.snapshot.entry_for_path(path)?.clone();
                Some(Edit::Insert(Entry {
                    git_status: *git_status,
                    ..entry.clone()
                }))
            })
            .collect();

        // Apply the git status changes.
        util::extend_sorted(
            &mut state.changed_paths,
            changes.iter().map(|p| p.0.clone()),
            usize::MAX,
            Ord::cmp,
        );
        state.snapshot.entries_by_path.edit(edits, &());
        log::trace!(
            "applied git status updates for repo {:?} in {:?}",
            job.work_directory.0,
            t0.elapsed(),
        );
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
        let mut old_paths = old_snapshot.entries_by_path.cursor::<PathKey>(&());
        let mut new_paths = new_snapshot.entries_by_path.cursor::<PathKey>(&());
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
                                        .map_or(false, |dir| new_entry.path.starts_with(dir));
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
                                .map_or(false, |dir| new_entry.path.starts_with(dir));
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

    fn is_path_private(&self, path: &Path) -> bool {
        !self.share_private_files && self.settings.is_path_private(path)
    }

    async fn next_scan_request(&self) -> Result<ScanRequest> {
        let mut request = self.scan_requests_rx.recv().await?;
        while let Ok(next_request) = self.scan_requests_rx.try_recv() {
            request.relative_paths.extend(next_request.relative_paths);
            request.done.extend(next_request.done);
        }
        Ok(request)
    }
}

fn swap_to_front(child_paths: &mut Vec<PathBuf>, file: &OsStr) {
    let position = child_paths
        .iter()
        .position(|path| path.file_name().unwrap() == file);
    if let Some(position) = position {
        let temp = child_paths.remove(position);
        child_paths.insert(0, temp);
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

struct RepoPaths {
    repo: Arc<dyn GitRepository>,
    relative_paths: Vec<Arc<Path>>,
    repo_paths: Vec<PathBuf>,
}

impl RepoPaths {
    fn add_paths(&mut self, relative_path: &Arc<Path>, repo_path: RepoPath) {
        self.relative_paths.push(relative_path.clone());
        self.repo_paths.push(repo_path.0);
    }

    fn into_git_file_statuses(self) -> HashMap<Arc<Path>, GitFileStatus> {
        let mut statuses = HashMap::default();
        if let Ok(status) = self.repo.status(&self.repo_paths) {
            for (repo_path, relative_path) in self.repo_paths.into_iter().zip(self.relative_paths) {
                if let Some(path_status) = status.get(&repo_path) {
                    statuses.insert(relative_path, path_status);
                }
            }
        }
        statuses
    }
}

struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    scan_queue: Sender<ScanJob>,
    ancestor_inodes: TreeSet<u64>,
    is_external: bool,
    containing_repository: Option<ScanJobContainingRepository>,
}

#[derive(Clone)]
struct ScanJobContainingRepository {
    work_directory: RepositoryWorkDirectory,
    statuses: GitStatus,
}

struct UpdateIgnoreStatusJob {
    abs_path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    ignore_queue: Sender<UpdateIgnoreStatusJob>,
    scan_queue: Sender<ScanJob>,
}

struct UpdateGitStatusesJob {
    work_directory: RepositoryWorkDirectory,
    location_in_repo: Option<Arc<Path>>,
    repository: Arc<dyn GitRepository>,
}

pub trait WorktreeModelHandle {
    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;

    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events_in_root_git_repository<'a>(
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

    // This function is similar to flush_fs_events, except that it waits for events to be flushed in
    // the .git folder of the root repository.
    // The reason for its existence is that a repository's .git folder might live *outside* of the
    // worktree and thus its FS events might go through a different path.
    // In order to flush those, we need to create artificial events in the .git folder and wait
    // for the repository to be reloaded.
    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events_in_root_git_repository<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        let file_name = "fs-event-sentinel";

        let tree = self.clone();
        let (fs, root_path, mut git_dir_scan_id) = self.update(cx, |tree, _| {
            let tree = tree.as_local().unwrap();
            let root_entry = tree.root_git_entry().unwrap();
            let local_repo_entry = tree.get_local_repo(&root_entry).unwrap();
            (
                tree.fs.clone(),
                local_repo_entry.git_dir_abs_path.clone(),
                local_repo_entry.git_dir_scan_id,
            )
        });

        let scan_id_increased = |tree: &mut Worktree, git_dir_scan_id: &mut usize| {
            let root_entry = tree.root_git_entry().unwrap();
            let local_repo_entry = tree
                .as_local()
                .unwrap()
                .get_local_repo(&root_entry)
                .unwrap();

            if local_repo_entry.git_dir_scan_id > *git_dir_scan_id {
                *git_dir_scan_id = local_repo_entry.git_dir_scan_id;
                true
            } else {
                false
            }
        };

        async move {
            fs.create_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            cx.condition(&tree, |tree, _| {
                scan_id_increased(tree, &mut git_dir_scan_id)
            })
            .await;

            fs.remove_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            cx.condition(&tree, |tree, _| {
                scan_id_increased(tree, &mut git_dir_scan_id)
            })
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
    fn count(&self, include_files: bool, include_dirs: bool, include_ignored: bool) -> usize {
        match (include_files, include_dirs, include_ignored) {
            (true, true, true) => self.count,
            (true, true, false) => self.non_ignored_count,
            (true, false, true) => self.file_count,
            (true, false, false) => self.non_ignored_file_count,
            (false, true, true) => self.count - self.file_count,
            (false, true, false) => self.non_ignored_count - self.non_ignored_file_count,
            (false, false, _) => 0,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for TraversalProgress<'a> {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

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
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        *self += summary.statuses
    }
}

pub struct Traversal<'a> {
    cursor: sum_tree::Cursor<'a, Entry, TraversalProgress<'a>>,
    include_ignored: bool,
    include_files: bool,
    include_dirs: bool,
}

impl<'a> Traversal<'a> {
    fn new(
        entries: &'a SumTree<Entry>,
        include_files: bool,
        include_dirs: bool,
        include_ignored: bool,
        start_path: &Path,
    ) -> Self {
        let mut cursor = entries.cursor(&());
        cursor.seek(&TraversalTarget::Path(start_path), Bias::Left, &());
        let mut traversal = Self {
            cursor,
            include_files,
            include_dirs,
            include_ignored,
        };
        if traversal.end_offset() == traversal.start_offset() {
            traversal.next();
        }
        traversal
    }
    pub fn advance(&mut self) -> bool {
        self.advance_by(1)
    }

    pub fn advance_by(&mut self, count: usize) -> bool {
        self.cursor.seek_forward(
            &TraversalTarget::Count {
                count: self.end_offset() + count,
                include_dirs: self.include_dirs,
                include_files: self.include_files,
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
                if (self.include_files || !entry.is_file())
                    && (self.include_dirs || !entry.is_dir())
                    && (self.include_ignored || !entry.is_ignored)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn back_to_parent(&mut self) -> bool {
        let Some(parent_path) = self.cursor.item().and_then(|entry| entry.path.parent()) else {
            return false;
        };
        self.cursor
            .seek(&TraversalTarget::Path(parent_path), Bias::Left, &())
    }

    pub fn entry(&self) -> Option<&'a Entry> {
        self.cursor.item()
    }

    pub fn start_offset(&self) -> usize {
        self.cursor
            .start()
            .count(self.include_files, self.include_dirs, self.include_ignored)
    }

    pub fn end_offset(&self) -> usize {
        self.cursor
            .end(&())
            .count(self.include_files, self.include_dirs, self.include_ignored)
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
        include_files: bool,
        include_ignored: bool,
        include_dirs: bool,
    },
}

impl<'a, 'b> SeekTarget<'a, EntrySummary, TraversalProgress<'a>> for TraversalTarget<'b> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: &()) -> Ordering {
        match self {
            TraversalTarget::Path(path) => path.cmp(&cursor_location.max_path),
            TraversalTarget::PathSuccessor(path) => {
                if cursor_location.max_path.starts_with(path) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            TraversalTarget::Count {
                count,
                include_files,
                include_dirs,
                include_ignored,
            } => Ord::cmp(
                count,
                &cursor_location.count(*include_files, *include_dirs, *include_ignored),
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

pub struct ChildEntriesIter<'a> {
    parent_path: &'a Path,
    traversal: Traversal<'a>,
}

impl<'a> Iterator for ChildEntriesIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.traversal.entry() {
            if item.path.starts_with(self.parent_path) {
                self.traversal.advance_to_sibling();
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
            is_ignored: entry.is_ignored,
            is_external: entry.is_external,
            git_status: entry.git_status.map(git_status_to_proto),
            is_fifo: entry.is_fifo,
            size: Some(entry.size),
            canonical_path: entry
                .canonical_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string()),
        }
    }
}

impl<'a> TryFrom<(&'a CharBag, proto::Entry)> for Entry {
    type Error = anyhow::Error;

    fn try_from((root_char_bag, entry): (&'a CharBag, proto::Entry)) -> Result<Self> {
        let kind = if entry.is_dir {
            EntryKind::Dir
        } else {
            EntryKind::File
        };
        let path: Arc<Path> = PathBuf::from(entry.path).into();
        let char_bag = char_bag_for_path(*root_char_bag, &path);
        Ok(Entry {
            id: ProjectEntryId::from_proto(entry.id),
            kind,
            path,
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            size: entry.size.unwrap_or(0),
            canonical_path: entry
                .canonical_path
                .map(|path_string| Box::from(Path::new(&path_string))),
            is_ignored: entry.is_ignored,
            is_external: entry.is_external,
            git_status: git_status_from_proto(entry.git_status),
            is_private: false,
            char_bag,
            is_fifo: entry.is_fifo,
        })
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
    pub const MIN: Self = Self(usize::MIN);

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

#[cfg(any(test, feature = "test-support"))]
impl CreatedEntry {
    pub fn to_included(self) -> Option<Entry> {
        match self {
            CreatedEntry::Included(entry) => Some(entry),
            CreatedEntry::Excluded { .. } => None,
        }
    }
}
