mod ignore;
mod worktree_settings;
#[cfg(test)]
mod worktree_tests;

use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{Context as _, Result, anyhow};
use clock::ReplicaId;
use collections::{HashMap, HashSet, VecDeque};
use fs::{Fs, MTime, PathEvent, RemoveOptions, Watcher, copy_recursive, read_dir_items};
use futures::{
    FutureExt as _, Stream, StreamExt,
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    select_biased,
    task::Poll,
};
use fuzzy::CharBag;
use git::{
    COMMIT_MESSAGE, DOT_GIT, FSMONITOR_DAEMON, GITIGNORE, INDEX_LOCK, LFS_DIR,
    repository::RepoPath, status::GitSummary,
};
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, Task,
};
use ignore::IgnoreStack;
use language::DiskState;

use parking_lot::Mutex;
use paths::{local_settings_folder_relative_path, local_vscode_folder_relative_path};
use postage::{
    barrier,
    prelude::{Sink as _, Stream as _},
    watch,
};
use rpc::{
    AnyProtoClient,
    proto::{self, FromProto, ToProto, split_worktree_update},
};
pub use settings::WorktreeId;
use settings::{Settings, SettingsLocation, SettingsStore};
use smallvec::{SmallVec, smallvec};
use smol::channel::{self, Sender};
use std::{
    any::Any,
    borrow::Borrow as _,
    cmp::Ordering,
    collections::hash_map,
    convert::TryFrom,
    ffi::OsStr,
    fmt,
    future::Future,
    mem::{self},
    ops::{Deref, DerefMut},
    path::{Component, Path, PathBuf},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use sum_tree::{Bias, Edit, KeyedItem, SeekTarget, SumTree, Summary, TreeMap, TreeSet, Unit};
use text::{LineEnding, Rope};
use util::{
    ResultExt,
    paths::{PathMatcher, SanitizedPath, home_dir},
};
pub use worktree_settings::WorktreeSettings;

pub const FS_WATCH_LATENCY: Duration = Duration::from_millis(100);

/// A set of local or remote files that are being opened as part of a project.
/// Responsible for tracking related FS (for local)/collab (for remote) events and corresponding updates.
/// Stores git repositories data and the diagnostics for the file(s).
///
/// Has an absolute path, and may be set to be visible in Zed UI or not.
/// May correspond to a directory or a single file.
/// Possible examples:
/// * a drag and dropped file — may be added as an invisible, "ephemeral" entry to the current worktree
/// * a directory opened in Zed — may be added as a visible entry to the current worktree
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
}

pub struct LoadedBinaryFile {
    pub file: Arc<File>,
    pub content: Vec<u8>,
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    scan_requests_tx: channel::Sender<ScanRequest>,
    path_prefixes_to_scan_tx: channel::Sender<PathPrefixScanRequest>,
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

pub struct PathPrefixScanRequest {
    path: Arc<Path>,
    done: SmallVec<[barrier::Sender; 1]>,
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
    file_scan_inclusions: PathMatcher,
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
    abs_path: SanitizedPath,
    root_name: String,
    root_char_bag: CharBag,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
    always_included_entries: Vec<Arc<Path>>,

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

/// This path corresponds to the 'content path' of a repository in relation
/// to Zed's project root.
/// In the majority of the cases, this is the folder that contains the .git folder.
/// But if a sub-folder of a git repository is opened, this corresponds to the
/// project root and the .git folder is located in a parent directory.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WorkDirectory {
    InProject {
        relative_path: Arc<Path>,
    },
    AboveProject {
        absolute_path: Arc<Path>,
        location_in_repo: Arc<Path>,
    },
}

impl WorkDirectory {
    #[cfg(test)]
    fn in_project(path: &str) -> Self {
        let path = Path::new(path);
        Self::InProject {
            relative_path: path.into(),
        }
    }

    //#[cfg(test)]
    //fn canonicalize(&self) -> Self {
    //    match self {
    //        WorkDirectory::InProject { relative_path } => WorkDirectory::InProject {
    //            relative_path: relative_path.clone(),
    //        },
    //        WorkDirectory::AboveProject {
    //            absolute_path,
    //            location_in_repo,
    //        } => WorkDirectory::AboveProject {
    //            absolute_path: absolute_path.canonicalize().unwrap().into(),
    //            location_in_repo: location_in_repo.clone(),
    //        },
    //    }
    //}

    fn path_key(&self) -> PathKey {
        match self {
            WorkDirectory::InProject { relative_path } => PathKey(relative_path.clone()),
            WorkDirectory::AboveProject { .. } => PathKey(Path::new("").into()),
        }
    }

    /// Returns true if the given path is a child of the work directory.
    ///
    /// Note that the path may not be a member of this repository, if there
    /// is a repository in a directory between these two paths
    /// external .git folder in a parent folder of the project root.
    #[track_caller]
    pub fn directory_contains(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        debug_assert!(path.is_relative());
        match self {
            WorkDirectory::InProject { relative_path } => path.starts_with(relative_path),
            WorkDirectory::AboveProject { .. } => true,
        }
    }

    /// relativize returns the given project path relative to the root folder of the
    /// repository.
    /// If the root of the repository (and its .git folder) are located in a parent folder
    /// of the project root folder, then the returned RepoPath is relative to the root
    /// of the repository and not a valid path inside the project.
    pub fn relativize(&self, path: &Path) -> Result<RepoPath> {
        // path is assumed to be relative to worktree root.
        debug_assert!(path.is_relative());
        match self {
            WorkDirectory::InProject { relative_path } => Ok(path
                .strip_prefix(relative_path)
                .map_err(|_| {
                    anyhow!(
                        "could not relativize {:?} against {:?}",
                        path,
                        relative_path
                    )
                })?
                .into()),
            WorkDirectory::AboveProject {
                location_in_repo, ..
            } => {
                // Avoid joining a `/` to location_in_repo in the case of a single-file worktree.
                if path == Path::new("") {
                    Ok(RepoPath(location_in_repo.clone()))
                } else {
                    Ok(location_in_repo.join(path).into())
                }
            }
        }
    }

    /// This is the opposite operation to `relativize` above
    pub fn try_unrelativize(&self, path: &RepoPath) -> Option<Arc<Path>> {
        match self {
            WorkDirectory::InProject { relative_path } => Some(relative_path.join(path).into()),
            WorkDirectory::AboveProject {
                location_in_repo, ..
            } => {
                // If we fail to strip the prefix, that means this status entry is
                // external to this worktree, and we definitely won't have an entry_id
                path.strip_prefix(location_in_repo).ok().map(Into::into)
            }
        }
    }

    pub fn unrelativize(&self, path: &RepoPath) -> Arc<Path> {
        match self {
            WorkDirectory::InProject { relative_path } => relative_path.join(path).into(),
            WorkDirectory::AboveProject {
                location_in_repo, ..
            } => {
                if &path.0 == location_in_repo {
                    // Single-file worktree
                    return location_in_repo
                        .file_name()
                        .map(Path::new)
                        .unwrap_or(Path::new(""))
                        .into();
                }
                let mut location_in_repo = &**location_in_repo;
                let mut parents = PathBuf::new();
                loop {
                    if let Ok(segment) = path.strip_prefix(location_in_repo) {
                        return parents.join(segment).into();
                    }
                    location_in_repo = location_in_repo.parent().unwrap_or(Path::new(""));
                    parents.push(Component::ParentDir);
                }
            }
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            WorkDirectory::InProject { relative_path } => relative_path.display().to_string(),
            WorkDirectory::AboveProject {
                absolute_path,
                location_in_repo,
            } => {
                let num_of_dots = location_in_repo.components().count();

                "../".repeat(num_of_dots)
                    + &absolute_path
                        .file_name()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default()
                    + "/"
            }
        }
    }
}

impl Default for WorkDirectory {
    fn default() -> Self {
        Self::InProject {
            relative_path: Arc::from(Path::new("")),
        }
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
    /// The file handle of the root dir
    /// (so we can find it after it's been moved)
    root_file_handle: Option<Arc<dyn fs::FileHandle>>,
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
}

#[derive(Debug, Clone)]
struct LocalRepositoryEntry {
    work_directory_id: ProjectEntryId,
    work_directory: WorkDirectory,
    work_directory_abs_path: Arc<Path>,
    git_dir_scan_id: usize,
    /// Absolute path to the original .git entry that caused us to create this repository.
    ///
    /// This is normally a directory, but may be a "gitfile" that points to a directory elsewhere
    /// (whose path we then store in `repository_dir_abs_path`).
    dot_git_abs_path: Arc<Path>,
    /// Absolute path to the "commondir" for this repository.
    ///
    /// This is always a directory. For a normal repository, this is the same as dot_git_abs_path,
    /// but in the case of a submodule or a worktree it is the path to the "parent" .git directory
    /// from which the submodule/worktree was derived.
    common_dir_abs_path: Arc<Path>,
    /// Absolute path to the directory holding the repository's state.
    ///
    /// For a normal repository, this is a directory and coincides with `dot_git_abs_path` and
    /// `common_dir_abs_path`. For a submodule or worktree, this is some subdirectory of the
    /// commondir like `/project/.git/modules/foo`.
    repository_dir_abs_path: Arc<Path>,
}

impl sum_tree::Item for LocalRepositoryEntry {
    type Summary = PathSummary<Unit>;

    fn summary(&self, _: &<Self::Summary as Summary>::Context) -> Self::Summary {
        PathSummary {
            max_path: self.work_directory.path_key().0,
            item_summary: Unit,
        }
    }
}

impl KeyedItem for LocalRepositoryEntry {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        self.work_directory.path_key()
    }
}

//impl LocalRepositoryEntry {
//    pub fn repo(&self) -> &Arc<dyn GitRepository> {
//        &self.repo_ptr
//    }
//}

impl Deref for LocalRepositoryEntry {
    type Target = WorkDirectory;

    fn deref(&self) -> &Self::Target {
        &self.work_directory
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

#[derive(Debug)]
enum ScanState {
    Started,
    Updated {
        snapshot: LocalSnapshot,
        changes: UpdatedEntriesSet,
        barrier: SmallVec<[barrier::Sender; 1]>,
        scanning: bool,
    },
    RootUpdated {
        new_path: Option<SanitizedPath>,
    },
}

struct UpdateObservationState {
    snapshots_tx: mpsc::UnboundedSender<(LocalSnapshot, UpdatedEntriesSet)>,
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
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
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

        let root_file_handle = fs.open_handle(&abs_path).await.log_err();

        cx.new(move |cx: &mut Context<Worktree>| {
            let mut snapshot = LocalSnapshot {
                ignores_by_parent_abs_path: Default::default(),
                git_repositories: Default::default(),
                snapshot: Snapshot::new(
                    cx.entity_id().as_u64(),
                    abs_path
                        .file_name()
                        .map_or(String::new(), |f| f.to_string_lossy().to_string()),
                    abs_path.clone(),
                ),
                root_file_handle,
            };

            let worktree_id = snapshot.id();
            let settings_location = Some(SettingsLocation {
                worktree_id,
                path: Path::new(EMPTY_PATH),
            });

            let settings = WorktreeSettings::get(settings_location, cx).clone();
            cx.observe_global::<SettingsStore>(move |this, cx| {
                if let Self::Local(this) = this {
                    let settings = WorktreeSettings::get(settings_location, cx).clone();
                    if this.settings != settings {
                        this.settings = settings;
                        this.restart_background_scanners(cx);
                    }
                }
            })
            .detach();

            let share_private_files = false;
            if let Some(metadata) = metadata {
                let mut entry = Entry::new(
                    Arc::from(Path::new("")),
                    &metadata,
                    &next_entry_id,
                    snapshot.root_char_bag,
                    None,
                );
                if !metadata.is_dir {
                    entry.is_private = !share_private_files
                        && settings.is_path_private(abs_path.file_name().unwrap().as_ref());
                }
                snapshot.insert_entry(entry, fs.as_ref());
            }

            let (scan_requests_tx, scan_requests_rx) = channel::unbounded();
            let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = channel::unbounded();
            let mut worktree = LocalWorktree {
                share_private_files,
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
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let snapshot = Snapshot::new(
                worktree.id,
                worktree.root_name,
                Arc::<Path>::from_proto(worktree.abs_path),
            );

            let background_snapshot = Arc::new(Mutex::new((
                snapshot.clone(),
                Vec::<proto::UpdateWorktree>::new(),
            )));
            let (background_updates_tx, mut background_updates_rx) =
                mpsc::unbounded::<proto::UpdateWorktree>();
            let (mut snapshot_updated_tx, mut snapshot_updated_rx) = watch::channel();

            let worktree_id = snapshot.id();
            let settings_location = Some(SettingsLocation {
                worktree_id,
                path: Path::new(EMPTY_PATH),
            });

            let settings = WorktreeSettings::get(settings_location, cx).clone();
            let worktree = RemoteWorktree {
                client,
                project_id,
                replica_id,
                snapshot,
                file_scan_inclusions: settings.file_scan_inclusions.clone(),
                background_snapshot: background_snapshot.clone(),
                updates_tx: Some(background_updates_tx),
                update_observer: None,
                snapshot_subscriptions: Default::default(),
                visible: worktree.visible,
                disconnected: false,
            };

            // Apply updates to a separate snapshot in a background task, then
            // send them to a foreground task which updates the model.
            cx.background_spawn(async move {
                while let Some(update) = background_updates_rx.next().await {
                    {
                        let mut lock = background_snapshot.lock();
                        lock.0
                            .apply_remote_update(update.clone(), &settings.file_scan_inclusions)
                            .log_err();
                        lock.1.push(update);
                    }
                    snapshot_updated_tx.send(()).await.ok();
                }
            })
            .detach();

            // On the foreground task, update to the latest snapshot and notify
            // any update observer of all updates that led to that snapshot.
            cx.spawn(async move |this, cx| {
                while (snapshot_updated_rx.recv().await).is_some() {
                    this.update(cx, |this, cx| {
                        let mut entries_changed = false;
                        let this = this.as_remote_mut().unwrap();
                        {
                            let mut lock = this.background_snapshot.lock();
                            this.snapshot = lock.0.clone();
                            for update in lock.1.drain(..) {
                                entries_changed |= !update.updated_entries.is_empty()
                                    || !update.removed_entries.is_empty();
                                if let Some(tx) = &this.update_observer {
                                    tx.unbounded_send(update).ok();
                                }
                            }
                        };

                        if entries_changed {
                            cx.emit(Event::UpdatedEntries(Arc::default()));
                        }
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

    pub fn settings_location(&self, _: &Context<Self>) -> SettingsLocation<'static> {
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
            abs_path: self.abs_path().to_proto(),
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
            Worktree::Local(worktree) => worktree.abs_path.clone().into(),
            Worktree::Remote(worktree) => worktree.abs_path.clone().into(),
        }
    }

    pub fn root_file(&self, cx: &Context<Self>) -> Option<Arc<File>> {
        let entry = self.root_entry()?;
        Some(File::for_entry(entry.clone(), cx.entity()))
    }

    pub fn observe_updates<F, Fut>(&mut self, project_id: u64, cx: &Context<Worktree>, callback: F)
    where
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

    pub fn file_exists(&self, path: &Path, cx: &Context<Worktree>) -> Task<Result<bool>> {
        match self {
            Worktree::Local(this) => {
                let fs = this.fs.clone();
                let path = this.absolutize(path);
                cx.background_spawn(async move {
                    let path = path?;
                    let metadata = fs.metadata(&path).await?;
                    Ok(metadata.map_or(false, |metadata| !metadata.is_dir))
                })
            }
            Worktree::Remote(_) => Task::ready(Err(anyhow!(
                "remote worktrees can't yet check file existence"
            ))),
        }
    }

    pub fn load_file(&self, path: &Path, cx: &Context<Worktree>) -> Task<Result<LoadedFile>> {
        match self {
            Worktree::Local(this) => this.load_file(path, cx),
            Worktree::Remote(_) => {
                Task::ready(Err(anyhow!("remote worktrees can't yet load files")))
            }
        }
    }

    pub fn load_binary_file(
        &self,
        path: &Path,
        cx: &Context<Worktree>,
    ) -> Task<Result<LoadedBinaryFile>> {
        match self {
            Worktree::Local(this) => this.load_binary_file(path, cx),
            Worktree::Remote(_) => {
                Task::ready(Err(anyhow!("remote worktrees can't yet load binary files")))
            }
        }
    }

    pub fn write_file(
        &self,
        path: &Path,
        text: Rope,
        line_ending: LineEnding,
        cx: &Context<Worktree>,
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
        content: Option<Vec<u8>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let path: Arc<Path> = path.into();
        let worktree_id = self.id();
        match self {
            Worktree::Local(this) => this.create_entry(path, is_directory, content, cx),
            Worktree::Remote(this) => {
                let project_id = this.project_id;
                let request = this.client.request(proto::CreateProjectEntry {
                    worktree_id: worktree_id.to_proto(),
                    project_id,
                    path: path.as_ref().to_proto(),
                    content,
                    is_directory,
                });
                cx.spawn(async move |this, cx| {
                    let response = request.await?;
                    match response.entry {
                        Some(entry) => this
                            .update(cx, |worktree, cx| {
                                worktree.as_remote_mut().unwrap().insert_entry(
                                    entry,
                                    response.worktree_scan_id as usize,
                                    cx,
                                )
                            })?
                            .await
                            .map(CreatedEntry::Included),
                        None => {
                            let abs_path = this.update(cx, |worktree, _| {
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
        cx: &mut Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let task = match self {
            Worktree::Local(this) => this.delete_entry(entry_id, trash, cx),
            Worktree::Remote(this) => this.delete_entry(entry_id, trash, cx),
        }?;

        let entry = match &*self {
            Worktree::Local(this) => this.entry_for_id(entry_id),
            Worktree::Remote(this) => this.entry_for_id(entry_id),
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
        cx: &Context<Self>,
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
        cx: &Context<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let new_path: Arc<Path> = new_path.into();
        match self {
            Worktree::Local(this) => {
                this.copy_entry(entry_id, relative_worktree_source_path, new_path, cx)
            }
            Worktree::Remote(this) => {
                let relative_worktree_source_path = relative_worktree_source_path
                    .map(|relative_worktree_source_path| relative_worktree_source_path.to_proto());
                let response = this.client.request(proto::CopyProjectEntry {
                    project_id: this.project_id,
                    entry_id: entry_id.to_proto(),
                    relative_worktree_source_path,
                    new_path: new_path.to_proto(),
                });
                cx.spawn(async move |this, cx| {
                    let response = response.await?;
                    match response.entry {
                        Some(entry) => this
                            .update(cx, |worktree, cx| {
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
        target_directory: Arc<Path>,
        paths: Vec<Arc<Path>>,
        fs: Arc<dyn Fs>,
        cx: &Context<Worktree>,
    ) -> Task<Result<Vec<ProjectEntryId>>> {
        match self {
            Worktree::Local(this) => this.copy_external_entries(target_directory, paths, cx),
            Worktree::Remote(this) => this.copy_external_entries(target_directory, paths, fs, cx),
        }
    }

    pub fn expand_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        match self {
            Worktree::Local(this) => this.expand_entry(entry_id, cx),
            Worktree::Remote(this) => {
                let response = this.client.request(proto::ExpandProjectEntry {
                    project_id: this.project_id,
                    entry_id: entry_id.to_proto(),
                });
                Some(cx.spawn(async move |this, cx| {
                    let response = response.await?;
                    this.update(cx, |this, _| {
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

    pub fn expand_all_for_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        match self {
            Worktree::Local(this) => this.expand_all_for_entry(entry_id, cx),
            Worktree::Remote(this) => {
                let response = this.client.request(proto::ExpandAllForProjectEntry {
                    project_id: this.project_id,
                    entry_id: entry_id.to_proto(),
                });
                Some(cx.spawn(async move |this, cx| {
                    let response = response.await?;
                    this.update(cx, |this, _| {
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
        this: Entity<Self>,
        request: proto::CreateProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, entry) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.create_entry(
                    Arc::<Path>::from_proto(request.path),
                    request.is_directory,
                    request.content,
                    cx,
                ),
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
        this: Entity<Self>,
        request: proto::DeleteProjectEntry,
        mut cx: AsyncApp,
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
        this: Entity<Self>,
        request: proto::ExpandProjectEntry,
        mut cx: AsyncApp,
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

    pub async fn handle_expand_all_for_entry(
        this: Entity<Self>,
        request: proto::ExpandAllForProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::ExpandAllForProjectEntryResponse> {
        let task = this.update(&mut cx, |this, cx| {
            this.expand_all_for_entry(ProjectEntryId::from_proto(request.entry_id), cx)
        })?;
        task.ok_or_else(|| anyhow!("no such entry"))?.await?;
        let scan_id = this.read_with(&cx, |this, _| this.scan_id())?;
        Ok(proto::ExpandAllForProjectEntryResponse {
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_rename_entry(
        this: Entity<Self>,
        request: proto::RenameProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.rename_entry(
                    ProjectEntryId::from_proto(request.entry_id),
                    Arc::<Path>::from_proto(request.new_path),
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
        this: Entity<Self>,
        request: proto::CopyProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            let relative_worktree_source_path = request
                .relative_worktree_source_path
                .map(PathBuf::from_proto);
            (
                this.scan_id(),
                this.copy_entry(
                    ProjectEntryId::from_proto(request.entry_id),
                    relative_worktree_source_path,
                    PathBuf::from_proto(request.new_path),
                    cx,
                ),
            )
        })?;
        Ok(proto::ProjectEntryResponse {
            entry: task.await?.as_ref().map(|e| e.into()),
            worktree_scan_id: scan_id as u64,
        })
    }

    pub fn dot_git_abs_path(&self, work_directory: &WorkDirectory) -> PathBuf {
        let mut path = match work_directory {
            WorkDirectory::InProject { relative_path } => self.abs_path().join(relative_path),
            WorkDirectory::AboveProject { absolute_path, .. } => absolute_path.as_ref().to_owned(),
        };
        path.push(".git");
        path
    }

    pub fn is_single_file(&self) -> bool {
        self.root_dir().is_none()
    }

    /// For visible worktrees, returns the path with the worktree name as the first component.
    /// Otherwise, returns an absolute path.
    pub fn full_path(&self, worktree_relative_path: &Path) -> PathBuf {
        let mut full_path = PathBuf::new();

        if self.is_visible() {
            full_path.push(self.root_name());
        } else {
            let path = self.abs_path();

            if self.is_local() && path.starts_with(home_dir().as_path()) {
                full_path.push("~");
                full_path.push(path.strip_prefix(home_dir().as_path()).unwrap());
            } else {
                full_path.push(path)
            }
        }

        if worktree_relative_path.components().next().is_some() {
            full_path.push(&worktree_relative_path);
        }

        full_path
    }
}

impl LocalWorktree {
    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn is_path_private(&self, path: &Path) -> bool {
        !self.share_private_files && self.settings.is_path_private(path)
    }

    fn restart_background_scanners(&mut self, cx: &Context<Worktree>) {
        let (scan_requests_tx, scan_requests_rx) = channel::unbounded();
        let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = channel::unbounded();
        self.scan_requests_tx = scan_requests_tx;
        self.path_prefixes_to_scan_tx = path_prefixes_to_scan_tx;

        self.start_background_scanner(scan_requests_rx, path_prefixes_to_scan_rx, cx);
        let always_included_entries = mem::take(&mut self.snapshot.always_included_entries);
        log::debug!(
            "refreshing entries for the following always included paths: {:?}",
            always_included_entries
        );

        // Cleans up old always included entries to ensure they get updated properly. Otherwise,
        // nested always included entries may not get updated and will result in out-of-date info.
        self.refresh_entries_for_paths(always_included_entries);
    }

    fn start_background_scanner(
        &mut self,
        scan_requests_rx: channel::Receiver<ScanRequest>,
        path_prefixes_to_scan_rx: channel::Receiver<PathPrefixScanRequest>,
        cx: &Context<Worktree>,
    ) {
        let snapshot = self.snapshot();
        let share_private_files = self.share_private_files;
        let next_entry_id = self.next_entry_id.clone();
        let fs = self.fs.clone();
        let settings = self.settings.clone();
        let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded();
        let background_scanner = cx.background_spawn({
            let abs_path = snapshot.abs_path.as_path().to_path_buf();
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
                    }),
                    phase: BackgroundScannerPhase::InitialScan,
                    share_private_files,
                    settings,
                    watcher,
                };

                scanner
                    .run(Box::pin(events.map(|events| events.into_iter().collect())))
                    .await;
            }
        });
        let scan_state_updater = cx.spawn(async move |this, cx| {
            while let Some((state, this)) = scan_states_rx.next().await.zip(this.upgrade()) {
                this.update(cx, |this, cx| {
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
                        ScanState::RootUpdated { new_path } => {
                            this.update_abs_path_and_refresh(new_path, cx);
                        }
                    }
                })
                .ok();
            }
        });
        self._background_scanner_tasks = vec![background_scanner, scan_state_updater];
        self.is_scanning = watch::channel_with(true);
    }

    fn set_snapshot(
        &mut self,
        mut new_snapshot: LocalSnapshot,
        entry_changes: UpdatedEntriesSet,
        cx: &mut Context<Worktree>,
    ) {
        let repo_changes = self.changed_repos(&self.snapshot, &mut new_snapshot);
        self.snapshot = new_snapshot;

        if let Some(share) = self.update_observer.as_mut() {
            share
                .snapshots_tx
                .unbounded_send((self.snapshot.clone(), entry_changes.clone()))
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
        new_snapshot: &mut LocalSnapshot,
    ) -> UpdatedGitRepositoriesSet {
        let mut changes = Vec::new();
        let mut old_repos = old_snapshot.git_repositories.iter().peekable();
        let new_repos = new_snapshot.git_repositories.clone();
        let mut new_repos = new_repos.iter().peekable();

        loop {
            match (new_repos.peek().map(clone), old_repos.peek().map(clone)) {
                (Some((new_entry_id, new_repo)), Some((old_entry_id, old_repo))) => {
                    match Ord::cmp(&new_entry_id, &old_entry_id) {
                        Ordering::Less => {
                            changes.push(UpdatedGitRepository {
                                work_directory_id: new_entry_id,
                                old_work_directory_abs_path: None,
                                new_work_directory_abs_path: Some(
                                    new_repo.work_directory_abs_path.clone(),
                                ),
                                dot_git_abs_path: Some(new_repo.dot_git_abs_path.clone()),
                                repository_dir_abs_path: Some(
                                    new_repo.repository_dir_abs_path.clone(),
                                ),
                                common_dir_abs_path: Some(new_repo.common_dir_abs_path.clone()),
                            });
                            new_repos.next();
                        }
                        Ordering::Equal => {
                            if new_repo.git_dir_scan_id != old_repo.git_dir_scan_id
                                || new_repo.work_directory_abs_path
                                    != old_repo.work_directory_abs_path
                            {
                                changes.push(UpdatedGitRepository {
                                    work_directory_id: new_entry_id,
                                    old_work_directory_abs_path: Some(
                                        old_repo.work_directory_abs_path.clone(),
                                    ),
                                    new_work_directory_abs_path: Some(
                                        new_repo.work_directory_abs_path.clone(),
                                    ),
                                    dot_git_abs_path: Some(new_repo.dot_git_abs_path.clone()),
                                    repository_dir_abs_path: Some(
                                        new_repo.repository_dir_abs_path.clone(),
                                    ),
                                    common_dir_abs_path: Some(new_repo.common_dir_abs_path.clone()),
                                });
                            }
                            new_repos.next();
                            old_repos.next();
                        }
                        Ordering::Greater => {
                            changes.push(UpdatedGitRepository {
                                work_directory_id: old_entry_id,
                                old_work_directory_abs_path: Some(
                                    old_repo.work_directory_abs_path.clone(),
                                ),
                                new_work_directory_abs_path: None,
                                dot_git_abs_path: None,
                                repository_dir_abs_path: None,
                                common_dir_abs_path: None,
                            });
                            old_repos.next();
                        }
                    }
                }
                (Some((entry_id, repo)), None) => {
                    changes.push(UpdatedGitRepository {
                        work_directory_id: entry_id,
                        old_work_directory_abs_path: None,
                        new_work_directory_abs_path: Some(repo.work_directory_abs_path.clone()),
                        dot_git_abs_path: Some(repo.dot_git_abs_path.clone()),
                        repository_dir_abs_path: Some(repo.repository_dir_abs_path.clone()),
                        common_dir_abs_path: Some(repo.common_dir_abs_path.clone()),
                    });
                    new_repos.next();
                }
                (None, Some((entry_id, repo))) => {
                    changes.push(UpdatedGitRepository {
                        work_directory_id: entry_id,
                        old_work_directory_abs_path: Some(repo.work_directory_abs_path.clone()),
                        new_work_directory_abs_path: None,
                        dot_git_abs_path: Some(repo.dot_git_abs_path.clone()),
                        repository_dir_abs_path: Some(repo.repository_dir_abs_path.clone()),
                        common_dir_abs_path: Some(repo.common_dir_abs_path.clone()),
                    });
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

    pub fn scan_complete(&self) -> impl Future<Output = ()> + use<> {
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

    fn load_binary_file(
        &self,
        path: &Path,
        cx: &Context<Worktree>,
    ) -> Task<Result<LoadedBinaryFile>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let entry = self.refresh_entry(path.clone(), None, cx);
        let is_private = self.is_path_private(path.as_ref());

        let worktree = cx.weak_entity();
        cx.background_spawn(async move {
            let abs_path = abs_path?;
            let content = fs.load_bytes(&abs_path).await?;

            let worktree = worktree
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
                        disk_state: DiskState::Present {
                            mtime: metadata.mtime,
                        },
                        is_local: true,
                        is_private,
                    })
                }
            };

            Ok(LoadedBinaryFile { file, content })
        })
    }

    fn load_file(&self, path: &Path, cx: &Context<Worktree>) -> Task<Result<LoadedFile>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let entry = self.refresh_entry(path.clone(), None, cx);
        let is_private = self.is_path_private(path.as_ref());

        cx.spawn(async move |this, _cx| {
            let abs_path = abs_path?;
            // WARN: Temporary workaround for #27283.
            //       We are not efficient with our memory usage per file, and use in excess of 64GB for a 10GB file
            //       Therefore, as a temporary workaround to prevent system freezes, we just bail before opening a file
            //       if it is too large
            //       5GB seems to be more reasonable, peaking at ~16GB, while 6GB jumps up to >24GB which seems like a
            //       reasonable limit
            {
                const FILE_SIZE_MAX: u64 = 6 * 1024 * 1024 * 1024; // 6GB
                if let Ok(Some(metadata)) = fs.metadata(&abs_path).await {
                    if metadata.len >= FILE_SIZE_MAX {
                        anyhow::bail!("File is too large to load");
                    }
                }
            }
            let text = fs.load(&abs_path).await?;

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
                        disk_state: DiskState::Present {
                            mtime: metadata.mtime,
                        },
                        is_local: true,
                        is_private,
                    })
                }
            };

            Ok(LoadedFile { file, text })
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
        content: Option<Vec<u8>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let path = path.into();
        let abs_path = match self.absolutize(&path) {
            Ok(path) => path,
            Err(e) => return Task::ready(Err(e.context(format!("absolutizing path {path:?}")))),
        };
        let path_excluded = self.settings.is_path_excluded(&abs_path);
        let fs = self.fs.clone();
        let task_abs_path = abs_path.clone();
        let write = cx.background_spawn(async move {
            if is_dir {
                fs.create_dir(&task_abs_path)
                    .await
                    .with_context(|| format!("creating directory {task_abs_path:?}"))
            } else {
                fs.write(&task_abs_path, content.as_deref().unwrap_or(&[]))
                    .await
                    .with_context(|| format!("creating file {task_abs_path:?}"))
            }
        });

        let lowest_ancestor = self.lowest_ancestor(&path);
        cx.spawn(async move |this, cx| {
            write.await?;
            if path_excluded {
                return Ok(CreatedEntry::Excluded { abs_path });
            }

            let (result, refreshes) = this.update(cx, |this, cx| {
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
        cx: &Context<Worktree>,
    ) -> Task<Result<Arc<File>>> {
        let path = path.into();
        let fs = self.fs.clone();
        let is_private = self.is_path_private(&path);
        let Ok(abs_path) = self.absolutize(&path) else {
            return Task::ready(Err(anyhow!("invalid path {path:?}")));
        };

        let write = cx.background_spawn({
            let fs = fs.clone();
            let abs_path = abs_path.clone();
            async move { fs.save(&abs_path, &text, line_ending).await }
        });

        cx.spawn(async move |this, cx| {
            write.await?;
            let entry = this
                .update(cx, |this, cx| {
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
                    disk_state: DiskState::Present {
                        mtime: metadata.mtime,
                    },
                    entry_id: None,
                    is_local: true,
                    is_private,
                }))
            }
        })
    }

    fn delete_entry(
        &self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let entry = self.entry_for_id(entry_id)?.clone();
        let abs_path = self.absolutize(&entry.path);
        let fs = self.fs.clone();

        let delete = cx.background_spawn(async move {
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

        Some(cx.spawn(async move |this, cx| {
            let path = delete.await?;
            this.update(cx, |this, _| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entries_for_paths(vec![path])
            })?
            .recv()
            .await;
            Ok(())
        }))
    }

    /// Rename an entry.
    ///
    /// `new_path` is the new relative path to the worktree root.
    /// If the root entry is renamed then `new_path` is the new root name instead.
    fn rename_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let old_path = match self.entry_for_id(entry_id) {
            Some(entry) => entry.path.clone(),
            None => return Task::ready(Err(anyhow!("no entry to rename for id {entry_id:?}"))),
        };
        let new_path = new_path.into();
        let abs_old_path = self.absolutize(&old_path);

        let is_root_entry = self.root_entry().is_some_and(|e| e.id == entry_id);
        let abs_new_path = if is_root_entry {
            let Some(root_parent_path) = self.abs_path().parent() else {
                return Task::ready(Err(anyhow!("no parent for path {:?}", self.abs_path)));
            };
            root_parent_path.join(&new_path)
        } else {
            let Ok(absolutize_path) = self.absolutize(&new_path) else {
                return Task::ready(Err(anyhow!("absolutizing path {new_path:?}")));
            };
            absolutize_path
        };
        let abs_path = abs_new_path.clone();
        let fs = self.fs.clone();
        let case_sensitive = self.fs_case_sensitive;
        let rename = cx.background_spawn(async move {
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

        cx.spawn(async move |this, cx| {
            rename.await?;
            Ok(this
                .update(cx, |this, cx| {
                    let local = this.as_local_mut().unwrap();
                    if is_root_entry {
                        // We eagerly update `abs_path` and refresh this worktree.
                        // Otherwise, the FS watcher would do it on the `RootUpdated` event,
                        // but with a noticeable delay, so we handle it proactively.
                        local.update_abs_path_and_refresh(
                            Some(SanitizedPath::from(abs_path.clone())),
                            cx,
                        );
                        Task::ready(Ok(this.root_entry().cloned()))
                    } else {
                        local.refresh_entry(new_path.clone(), Some(old_path), cx)
                    }
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
        cx: &Context<Worktree>,
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
        let copy = cx.background_spawn(async move {
            copy_recursive(
                fs.as_ref(),
                &abs_old_path?,
                &abs_new_path?,
                Default::default(),
            )
            .await
        });

        cx.spawn(async move |this, cx| {
            copy.await?;
            this.update(cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entry(new_path.clone(), None, cx)
            })?
            .await
        })
    }

    pub fn copy_external_entries(
        &self,
        target_directory: Arc<Path>,
        paths: Vec<Arc<Path>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<Vec<ProjectEntryId>>> {
        let Ok(target_directory) = self.absolutize(&target_directory) else {
            return Task::ready(Err(anyhow!("invalid target path")));
        };
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

        cx.spawn(async move |this, cx| {
            cx.background_spawn(async move {
                for (source, target) in paths {
                    copy_recursive(
                        fs.as_ref(),
                        &source,
                        &target,
                        fs::CopyOptions {
                            overwrite: true,
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
            let mut refresh = cx.read_entity(
                &this.upgrade().with_context(|| "Dropped worktree")?,
                |this, _| {
                    Ok::<postage::barrier::Receiver, anyhow::Error>(
                        this.as_local()
                            .with_context(|| "Worktree is not local")?
                            .refresh_entries_for_paths(paths_to_refresh.clone()),
                    )
                },
            )??;

            cx.background_spawn(async move {
                refresh.next().await;
                Ok::<(), anyhow::Error>(())
            })
            .await
            .log_err();

            let this = this.upgrade().with_context(|| "Dropped worktree")?;
            cx.read_entity(&this, |this, _| {
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
        cx: &Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let path = self.entry_for_id(entry_id)?.path.clone();
        let mut refresh = self.refresh_entries_for_paths(vec![path]);
        Some(cx.background_spawn(async move {
            refresh.next().await;
            Ok(())
        }))
    }

    fn expand_all_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let path = self.entry_for_id(entry_id).unwrap().path.clone();
        let mut rx = self.add_path_prefix_to_scan(path.clone());
        Some(cx.background_spawn(async move {
            rx.next().await;
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

    #[cfg(feature = "test-support")]
    pub fn manually_refresh_entries_for_paths(&self, paths: Vec<Arc<Path>>) -> barrier::Receiver {
        self.refresh_entries_for_paths(paths)
    }

    pub fn add_path_prefix_to_scan(&self, path_prefix: Arc<Path>) -> barrier::Receiver {
        let (tx, rx) = barrier::channel();
        self.path_prefixes_to_scan_tx
            .try_send(PathPrefixScanRequest {
                path: path_prefix,
                done: smallvec![tx],
            })
            .ok();
        rx
    }

    fn refresh_entry(
        &self,
        path: Arc<Path>,
        old_path: Option<Arc<Path>>,
        cx: &Context<Worktree>,
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
        cx.spawn(async move |this, cx| {
            refresh.recv().await;
            log::trace!("refreshed entry {path:?} in {:?}", t0.elapsed());
            let new_entry = this.update(cx, |this, _| {
                this.entry_for_path(path)
                    .cloned()
                    .ok_or_else(|| anyhow!("failed to read path after update"))
            })??;
            Ok(Some(new_entry))
        })
    }

    fn observe_updates<F, Fut>(&mut self, project_id: u64, cx: &Context<Worktree>, callback: F)
    where
        F: 'static + Send + Fn(proto::UpdateWorktree) -> Fut,
        Fut: 'static + Send + Future<Output = bool>,
    {
        if let Some(observer) = self.update_observer.as_mut() {
            *observer.resume_updates.borrow_mut() = ();
            return;
        }

        let (resume_updates_tx, mut resume_updates_rx) = watch::channel::<()>();
        let (snapshots_tx, mut snapshots_rx) =
            mpsc::unbounded::<(LocalSnapshot, UpdatedEntriesSet)>();
        snapshots_tx
            .unbounded_send((self.snapshot(), Arc::default()))
            .ok();

        let worktree_id = cx.entity_id().as_u64();
        let _maintain_remote_snapshot = cx.background_spawn(async move {
            let mut is_first = true;
            while let Some((snapshot, entry_changes)) = snapshots_rx.next().await {
                let update = if is_first {
                    is_first = false;
                    snapshot.build_initial_update(project_id, worktree_id)
                } else {
                    snapshot.build_update(project_id, worktree_id, entry_changes)
                };

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

    pub fn share_private_files(&mut self, cx: &Context<Worktree>) {
        self.share_private_files = true;
        self.restart_background_scanners(cx);
    }

    fn update_abs_path_and_refresh(
        &mut self,
        new_path: Option<SanitizedPath>,
        cx: &Context<Worktree>,
    ) {
        if let Some(new_path) = new_path {
            self.snapshot.git_repositories = Default::default();
            self.snapshot.ignores_by_parent_abs_path = Default::default();
            let root_name = new_path
                .as_path()
                .file_name()
                .map_or(String::new(), |f| f.to_string_lossy().to_string());
            self.snapshot.update_abs_path(new_path, root_name);
        }
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

    fn observe_updates<F, Fut>(&mut self, project_id: u64, cx: &Context<Worktree>, callback: F)
    where
        F: 'static + Send + Fn(proto::UpdateWorktree) -> Fut,
        Fut: 'static + Send + Future<Output = bool>,
    {
        let (tx, mut rx) = mpsc::unbounded();
        let initial_update = self
            .snapshot
            .build_initial_update(project_id, self.id().to_proto());
        self.update_observer = Some(tx);
        cx.spawn(async move |this, cx| {
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
            this.update(cx, |this, _| {
                let this = this.as_remote_mut().unwrap();
                this.update_observer.take();
            })
        })
        .detach();
    }

    fn observed_snapshot(&self, scan_id: usize) -> bool {
        self.completed_scan_id >= scan_id
    }

    pub fn wait_for_snapshot(
        &mut self,
        scan_id: usize,
    ) -> impl Future<Output = Result<()>> + use<> {
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
        cx: &Context<Worktree>,
    ) -> Task<Result<Entry>> {
        let wait_for_snapshot = self.wait_for_snapshot(scan_id);
        cx.spawn(async move |this, cx| {
            wait_for_snapshot.await?;
            this.update(cx, |worktree, _| {
                let worktree = worktree.as_remote_mut().unwrap();
                let snapshot = &mut worktree.background_snapshot.lock().0;
                let entry = snapshot.insert_entry(entry, &worktree.file_scan_inclusions);
                worktree.snapshot = snapshot.clone();
                entry
            })?
        })
    }

    fn delete_entry(
        &self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let response = self.client.request(proto::DeleteProjectEntry {
            project_id: self.project_id,
            entry_id: entry_id.to_proto(),
            use_trash: trash,
        });
        Some(cx.spawn(async move |this, cx| {
            let response = response.await?;
            let scan_id = response.worktree_scan_id as usize;

            this.update(cx, move |this, _| {
                this.as_remote_mut().unwrap().wait_for_snapshot(scan_id)
            })?
            .await?;

            this.update(cx, |this, _| {
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
        cx: &Context<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let new_path: Arc<Path> = new_path.into();
        let response = self.client.request(proto::RenameProjectEntry {
            project_id: self.project_id,
            entry_id: entry_id.to_proto(),
            new_path: new_path.as_ref().to_proto(),
        });
        cx.spawn(async move |this, cx| {
            let response = response.await?;
            match response.entry {
                Some(entry) => this
                    .update(cx, |this, cx| {
                        this.as_remote_mut().unwrap().insert_entry(
                            entry,
                            response.worktree_scan_id as usize,
                            cx,
                        )
                    })?
                    .await
                    .map(CreatedEntry::Included),
                None => {
                    let abs_path = this.update(cx, |worktree, _| {
                        worktree
                            .absolutize(&new_path)
                            .with_context(|| format!("absolutizing {new_path:?}"))
                    })??;
                    Ok(CreatedEntry::Excluded { abs_path })
                }
            }
        })
    }

    fn copy_external_entries(
        &self,
        target_directory: Arc<Path>,
        paths_to_copy: Vec<Arc<Path>>,
        local_fs: Arc<dyn Fs>,
        cx: &Context<Worktree>,
    ) -> Task<Result<Vec<ProjectEntryId>, anyhow::Error>> {
        let client = self.client.clone();
        let worktree_id = self.id().to_proto();
        let project_id = self.project_id;

        cx.background_spawn(async move {
            let mut requests = Vec::new();
            for root_path_to_copy in paths_to_copy {
                let Some(filename) = root_path_to_copy.file_name() else {
                    continue;
                };
                for (abs_path, is_directory) in
                    read_dir_items(local_fs.as_ref(), &root_path_to_copy).await?
                {
                    let Ok(relative_path) = abs_path.strip_prefix(&root_path_to_copy) else {
                        continue;
                    };
                    let content = if is_directory {
                        None
                    } else {
                        Some(local_fs.load_bytes(&abs_path).await?)
                    };

                    let mut target_path = target_directory.join(filename);
                    if relative_path.file_name().is_some() {
                        target_path.push(relative_path)
                    }

                    requests.push(proto::CreateProjectEntry {
                        project_id,
                        worktree_id,
                        path: target_path.to_string_lossy().to_string(),
                        is_directory,
                        content,
                    });
                }
            }
            requests.sort_unstable_by(|a, b| a.path.cmp(&b.path));
            requests.dedup();

            let mut copied_entry_ids = Vec::new();
            for request in requests {
                let response = client.request(request).await?;
                copied_entry_ids.extend(response.entry.map(|e| ProjectEntryId::from_proto(e.id)));
            }

            Ok(copied_entry_ids)
        })
    }
}

impl Snapshot {
    pub fn new(id: u64, root_name: String, abs_path: Arc<Path>) -> Self {
        Snapshot {
            id: WorktreeId::from_usize(id as usize),
            abs_path: abs_path.into(),
            root_char_bag: root_name.chars().map(|c| c.to_ascii_lowercase()).collect(),
            root_name,
            always_included_entries: Default::default(),
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
            scan_id: 1,
            completed_scan_id: 0,
        }
    }

    pub fn id(&self) -> WorktreeId {
        self.id
    }

    // TODO:
    // Consider the following:
    //
    // ```rust
    // let abs_path: Arc<Path> = snapshot.abs_path(); // e.g. "C:\Users\user\Desktop\project"
    // let some_non_trimmed_path = Path::new("\\\\?\\C:\\Users\\user\\Desktop\\project\\main.rs");
    // // The caller perform some actions here:
    // some_non_trimmed_path.strip_prefix(abs_path);  // This fails
    // some_non_trimmed_path.starts_with(abs_path);   // This fails too
    // ```
    //
    // This is definitely a bug, but it's not clear if we should handle it here or not.
    pub fn abs_path(&self) -> &Arc<Path> {
        self.abs_path.as_path()
    }

    fn build_initial_update(&self, project_id: u64, worktree_id: u64) -> proto::UpdateWorktree {
        let mut updated_entries = self
            .entries_by_path
            .iter()
            .map(proto::Entry::from)
            .collect::<Vec<_>>();
        updated_entries.sort_unstable_by_key(|e| e.id);

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            abs_path: self.abs_path().to_proto(),
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries: Vec::new(),
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
            // Sent in separate messages.
            updated_repositories: Vec::new(),
            removed_repositories: Vec::new(),
        }
    }

    pub fn work_directory_abs_path(&self, work_directory: &WorkDirectory) -> Result<PathBuf> {
        match work_directory {
            WorkDirectory::InProject { relative_path } => self.absolutize(relative_path),
            WorkDirectory::AboveProject { absolute_path, .. } => {
                Ok(absolute_path.as_ref().to_owned())
            }
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
            Ok(self.abs_path.as_path().join(path))
        } else {
            Ok(self.abs_path.as_path().to_path_buf())
        }
    }

    pub fn contains_entry(&self, entry_id: ProjectEntryId) -> bool {
        self.entries_by_id.get(&entry_id, &()).is_some()
    }

    fn insert_entry(
        &mut self,
        entry: proto::Entry,
        always_included_paths: &PathMatcher,
    ) -> Result<Entry> {
        let entry = Entry::try_from((&self.root_char_bag, always_included_paths, entry))?;
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
                cursor.slice(&TraversalTarget::path(&removed_entry.path), Bias::Left, &());
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

    fn update_abs_path(&mut self, abs_path: SanitizedPath, root_name: String) {
        self.abs_path = abs_path;
        if root_name != self.root_name {
            self.root_char_bag = root_name.chars().map(|c| c.to_ascii_lowercase()).collect();
            self.root_name = root_name;
        }
    }

    fn apply_remote_update(
        &mut self,
        update: proto::UpdateWorktree,
        always_included_paths: &PathMatcher,
    ) -> Result<()> {
        log::debug!(
            "applying remote worktree update. {} entries updated, {} removed",
            update.updated_entries.len(),
            update.removed_entries.len()
        );
        self.update_abs_path(
            SanitizedPath::from(PathBuf::from_proto(update.abs_path)),
            update.root_name,
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
            let entry = Entry::try_from((&self.root_char_bag, always_included_paths, entry))?;
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
            snapshot: self,
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
        Traversal::new(self, include_files, include_dirs, include_ignored, path)
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

    pub fn paths(&self) -> impl Iterator<Item = &Arc<Path>> {
        let empty_path = Path::new("");
        self.entries_by_path
            .cursor::<()>(&())
            .filter(move |entry| entry.path.as_ref() != empty_path)
            .map(|entry| &entry.path)
    }

    pub fn child_entries<'a>(&'a self, parent_path: &'a Path) -> ChildEntriesIter<'a> {
        let options = ChildEntriesOptions {
            include_files: true,
            include_dirs: true,
            include_ignored: true,
        };
        self.child_entries_with_options(parent_path, options)
    }

    pub fn child_entries_with_options<'a>(
        &'a self,
        parent_path: &'a Path,
        options: ChildEntriesOptions,
    ) -> ChildEntriesIter<'a> {
        let mut cursor = self.entries_by_path.cursor(&());
        cursor.seek(&TraversalTarget::path(parent_path), Bias::Right, &());
        let traversal = Traversal {
            snapshot: self,
            cursor,
            include_files: options.include_files,
            include_dirs: options.include_dirs,
            include_ignored: options.include_ignored,
        };
        ChildEntriesIter {
            traversal,
            parent_path,
        }
    }

    pub fn root_entry(&self) -> Option<&Entry> {
        self.entry_for_path("")
    }

    /// TODO: what's the difference between `root_dir` and `abs_path`?
    /// is there any? if so, document it.
    pub fn root_dir(&self) -> Option<Arc<Path>> {
        self.root_entry()
            .filter(|entry| entry.is_dir())
            .map(|_| self.abs_path().clone())
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn scan_id(&self) -> usize {
        self.scan_id
    }

    pub fn entry_for_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());
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
    fn local_repo_for_work_directory_path(&self, path: &Path) -> Option<&LocalRepositoryEntry> {
        self.git_repositories
            .iter()
            .map(|(_, entry)| entry)
            .find(|entry| entry.work_directory.path_key() == PathKey(path.into()))
    }

    fn build_update(
        &self,
        project_id: u64,
        worktree_id: u64,
        entry_changes: UpdatedEntriesSet,
    ) -> proto::UpdateWorktree {
        let mut updated_entries = Vec::new();
        let mut removed_entries = Vec::new();

        for (_, entry_id, path_change) in entry_changes.iter() {
            if let PathChange::Removed = path_change {
                removed_entries.push(entry_id.0 as u64);
            } else if let Some(entry) = self.entry_for_id(*entry_id) {
                updated_entries.push(proto::Entry::from(entry));
            }
        }

        removed_entries.sort_unstable();
        updated_entries.sort_unstable_by_key(|e| e.id);

        // TODO - optimize, knowing that removed_entries are sorted.
        removed_entries.retain(|id| updated_entries.binary_search_by_key(id, |e| e.id).is_err());

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            abs_path: self.abs_path().to_proto(),
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries,
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
            // Sent in separate messages.
            updated_repositories: Vec::new(),
            removed_repositories: Vec::new(),
        }
    }

    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        if entry.is_file() && entry.path.file_name() == Some(&GITIGNORE) {
            let abs_path = self.abs_path.as_path().join(&entry.path);
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
    fn expanded_entries(&self) -> impl Iterator<Item = &Entry> {
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
                if (!entry.is_ignored && !entry.is_external) || entry.is_always_included {
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
                let ignore_parent_path = ignore_parent_abs_path
                    .strip_prefix(self.abs_path.as_path())
                    .unwrap();
                assert!(self.entry_for_path(ignore_parent_path).is_some());
                assert!(
                    self.entry_for_path(ignore_parent_path.join(*GITIGNORE))
                        .is_some()
                );
            }
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
        (!entry.is_external && (!entry.is_ignored || entry.is_always_included))
            || entry.path.file_name() == Some(*DOT_GIT)
            || entry.path.file_name() == Some(local_settings_folder_relative_path().as_os_str())
            || entry.path.file_name() == Some(local_vscode_folder_relative_path().as_os_str())
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
            let abs_parent_path = self.snapshot.abs_path.as_path().join(parent_path).into();
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
        log::trace!("background scanner removing path {path:?}");
        let mut new_entries;
        let removed_entries;
        {
            let mut cursor = self
                .snapshot
                .entries_by_path
                .cursor::<TraversalProgress>(&());
            new_entries = cursor.slice(&TraversalTarget::path(path), Bias::Left, &());
            removed_entries = cursor.slice(&TraversalTarget::successor(path), Bias::Left, &());
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
                let abs_parent_path = self
                    .snapshot
                    .abs_path
                    .as_path()
                    .join(entry.path.parent().unwrap());
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

        #[cfg(test)]
        self.snapshot.check_invariants(false);
    }

    fn insert_git_repository(
        &mut self,
        dot_git_path: Arc<Path>,
        fs: &dyn Fs,
        watcher: &dyn Watcher,
    ) {
        let work_dir_path: Arc<Path> = match dot_git_path.parent() {
            Some(parent_dir) => {
                // Guard against repositories inside the repository metadata
                if parent_dir.iter().any(|component| component == *DOT_GIT) {
                    log::debug!(
                        "not building git repository for nested `.git` directory, `.git` path in the worktree: {dot_git_path:?}"
                    );
                    return;
                };

                parent_dir.into()
            }
            None => {
                // `dot_git_path.parent().is_none()` means `.git` directory is the opened worktree itself,
                // no files inside that directory are tracked by git, so no need to build the repo around it
                log::debug!(
                    "not building git repository for the worktree itself, `.git` path in the worktree: {dot_git_path:?}"
                );
                return;
            }
        };

        self.insert_git_repository_for_path(
            WorkDirectory::InProject {
                relative_path: work_dir_path,
            },
            dot_git_path,
            fs,
            watcher,
        );
    }

    fn insert_git_repository_for_path(
        &mut self,
        work_directory: WorkDirectory,
        dot_git_path: Arc<Path>,
        fs: &dyn Fs,
        watcher: &dyn Watcher,
    ) -> Option<LocalRepositoryEntry> {
        let work_dir_entry = self.snapshot.entry_for_path(work_directory.path_key().0)?;
        let work_directory_abs_path = self
            .snapshot
            .work_directory_abs_path(&work_directory)
            .log_err()?;

        if self
            .snapshot
            .git_repositories
            .get(&work_dir_entry.id)
            .is_some()
        {
            log::trace!("existing git repository for {work_directory:?}");
            return None;
        }

        let dot_git_abs_path: Arc<Path> = self
            .snapshot
            .abs_path
            .as_path()
            .join(&dot_git_path)
            .as_path()
            .into();

        let (repository_dir_abs_path, common_dir_abs_path) =
            discover_git_paths(&dot_git_abs_path, fs);
        watcher.add(&common_dir_abs_path).log_err();
        if !repository_dir_abs_path.starts_with(&common_dir_abs_path) {
            watcher.add(&repository_dir_abs_path).log_err();
        }

        let work_directory_id = work_dir_entry.id;

        let local_repository = LocalRepositoryEntry {
            work_directory_id,
            work_directory,
            work_directory_abs_path: work_directory_abs_path.as_path().into(),
            git_dir_scan_id: 0,
            dot_git_abs_path,
            common_dir_abs_path,
            repository_dir_abs_path,
        };

        self.snapshot
            .git_repositories
            .insert(work_directory_id, local_repository.clone());

        log::trace!("inserting new local git repository");
        Some(local_repository)
    }
}

async fn is_git_dir(path: &Path, fs: &dyn Fs) -> bool {
    if path.file_name() == Some(&*DOT_GIT) {
        return true;
    }

    // If we're in a bare repository, we are not inside a `.git` folder. In a
    // bare repository, the root folder contains what would normally be in the
    // `.git` folder.
    let head_metadata = fs.metadata(&path.join("HEAD")).await;
    if !matches!(head_metadata, Ok(Some(_))) {
        return false;
    }
    let config_metadata = fs.metadata(&path.join("config")).await;
    matches!(config_metadata, Ok(Some(_)))
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

        impl fmt::Debug for EntriesByPath<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_map()
                    .entries(self.0.iter().map(|entry| (&entry.path, entry.id)))
                    .finish()
            }
        }

        impl fmt::Debug for EntriesById<'_> {
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
    pub worktree: Entity<Worktree>,
    pub path: Arc<Path>,
    pub disk_state: DiskState,
    pub entry_id: Option<ProjectEntryId>,
    pub is_local: bool,
    pub is_private: bool,
}

impl language::File for File {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        if self.is_local { Some(self) } else { None }
    }

    fn disk_state(&self) -> DiskState {
        self.disk_state
    }

    fn path(&self) -> &Arc<Path> {
        &self.path
    }

    fn full_path(&self, cx: &App) -> PathBuf {
        self.worktree.read(cx).full_path(&self.path)
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self, cx: &'a App) -> &'a OsStr {
        self.path
            .file_name()
            .unwrap_or_else(|| OsStr::new(&self.worktree.read(cx).root_name))
    }

    fn worktree_id(&self, cx: &App) -> WorktreeId {
        self.worktree.read(cx).id()
    }

    fn to_proto(&self, cx: &App) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: self.worktree.read(cx).id().to_proto(),
            entry_id: self.entry_id.map(|id| id.to_proto()),
            path: self.path.as_ref().to_proto(),
            mtime: self.disk_state.mtime().map(|time| time.into()),
            is_deleted: self.disk_state == DiskState::Deleted,
        }
    }

    fn is_private(&self) -> bool {
        self.is_private
    }
}

impl language::LocalFile for File {
    fn abs_path(&self, cx: &App) -> PathBuf {
        let worktree_path = &self.worktree.read(cx).abs_path();
        if self.path.as_ref() == Path::new("") {
            worktree_path.to_path_buf()
        } else {
            worktree_path.join(&self.path)
        }
    }

    fn load(&self, cx: &App) -> Task<Result<String>> {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        cx.background_spawn(async move { fs.load(&abs_path?).await })
    }

    fn load_bytes(&self, cx: &App) -> Task<Result<Vec<u8>>> {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        cx.background_spawn(async move { fs.load_bytes(&abs_path?).await })
    }
}

impl File {
    pub fn for_entry(entry: Entry, worktree: Entity<Worktree>) -> Arc<Self> {
        Arc::new(Self {
            worktree,
            path: entry.path.clone(),
            disk_state: if let Some(mtime) = entry.mtime {
                DiskState::Present { mtime }
            } else {
                DiskState::New
            },
            entry_id: Some(entry.id),
            is_local: true,
            is_private: entry.is_private,
        })
    }

    pub fn from_proto(
        proto: rpc::proto::File,
        worktree: Entity<Worktree>,
        cx: &App,
    ) -> Result<Self> {
        let worktree_id = worktree
            .read(cx)
            .as_remote()
            .ok_or_else(|| anyhow!("not remote"))?
            .id();

        if worktree_id.to_proto() != proto.worktree_id {
            return Err(anyhow!("worktree id does not match file"));
        }

        let disk_state = if proto.is_deleted {
            DiskState::Deleted
        } else {
            if let Some(mtime) = proto.mtime.map(&Into::into) {
                DiskState::Present { mtime }
            } else {
                DiskState::New
            }
        };

        Ok(Self {
            worktree,
            path: Arc::<Path>::from_proto(proto.path),
            disk_state,
            entry_id: proto.entry_id.map(ProjectEntryId::from_proto),
            is_local: false,
            is_private: false,
        })
    }

    pub fn from_dyn(file: Option<&Arc<dyn language::File>>) -> Option<&Self> {
        file.and_then(|f| {
            let f: &dyn language::File = f.borrow();
            let f: &dyn Any = f;
            f.downcast_ref()
        })
    }

    pub fn worktree_id(&self, cx: &App) -> WorktreeId {
        self.worktree.read(cx).id()
    }

    pub fn project_entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        match self.disk_state {
            DiskState::Deleted => None,
            _ => self.entry_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: ProjectEntryId,
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub inode: u64,
    pub mtime: Option<MTime>,

    pub canonical_path: Option<Arc<Path>>,
    /// Whether this entry is ignored by Git.
    ///
    /// We only scan ignored entries once the directory is expanded and
    /// exclude them from searches.
    pub is_ignored: bool,

    /// Whether this entry is always included in searches.
    ///
    /// This is used for entries that are always included in searches, even
    /// if they are ignored by git. Overridden by file_scan_exclusions.
    pub is_always_included: bool,

    /// Whether this entry's canonical path is outside of the worktree.
    /// This means the entry is only accessible from the worktree root via a
    /// symlink.
    ///
    /// We only scan entries outside of the worktree once the symlinked
    /// directory is expanded. External entries are treated like gitignored
    /// entries in that they are not included in searches.
    pub is_external: bool,

    /// Whether this entry is considered to be a `.env` file.
    pub is_private: bool,
    /// The entry's size on disk, in bytes.
    pub size: u64,
    pub char_bag: CharBag,
    pub is_fifo: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdatedGitRepository {
    /// ID of the repository's working directory.
    ///
    /// For a repo that's above the worktree root, this is the ID of the worktree root, and hence not unique.
    /// It's included here to aid the GitStore in detecting when a repository's working directory is renamed.
    pub work_directory_id: ProjectEntryId,
    pub old_work_directory_abs_path: Option<Arc<Path>>,
    pub new_work_directory_abs_path: Option<Arc<Path>>,
    /// For a normal git repository checkout, the absolute path to the .git directory.
    /// For a worktree, the absolute path to the worktree's subdirectory inside the .git directory.
    pub dot_git_abs_path: Option<Arc<Path>>,
    pub repository_dir_abs_path: Option<Arc<Path>>,
    pub common_dir_abs_path: Option<Arc<Path>>,
}

pub type UpdatedEntriesSet = Arc<[(Arc<Path>, ProjectEntryId, PathChange)]>;
pub type UpdatedGitRepositoriesSet = Arc<[UpdatedGitRepository]>;

#[derive(Clone, Debug)]
pub struct PathProgress<'a> {
    pub max_path: &'a Path,
}

#[derive(Clone, Debug)]
pub struct PathSummary<S> {
    pub max_path: Arc<Path>,
    pub item_summary: S,
}

impl<S: Summary> Summary for PathSummary<S> {
    type Context = S::Context;

    fn zero(cx: &Self::Context) -> Self {
        Self {
            max_path: Path::new("").into(),
            item_summary: S::zero(cx),
        }
    }

    fn add_summary(&mut self, rhs: &Self, cx: &Self::Context) {
        self.max_path = rhs.max_path.clone();
        self.item_summary.add_summary(&rhs.item_summary, cx);
    }
}

impl<'a, S: Summary> sum_tree::Dimension<'a, PathSummary<S>> for PathProgress<'a> {
    fn zero(_: &<PathSummary<S> as Summary>::Context) -> Self {
        Self {
            max_path: Path::new(""),
        }
    }

    fn add_summary(
        &mut self,
        summary: &'a PathSummary<S>,
        _: &<PathSummary<S> as Summary>::Context,
    ) {
        self.max_path = summary.max_path.as_ref()
    }
}

impl<'a> sum_tree::Dimension<'a, PathSummary<GitSummary>> for GitSummary {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathSummary<GitSummary>, _: &()) {
        *self += summary.item_summary
    }
}

impl<'a> sum_tree::SeekTarget<'a, PathSummary<GitSummary>, (TraversalProgress<'a>, GitSummary)>
    for PathTarget<'_>
{
    fn cmp(&self, cursor_location: &(TraversalProgress<'a>, GitSummary), _: &()) -> Ordering {
        self.cmp_path(&cursor_location.0.max_path)
    }
}

impl<'a, S: Summary> sum_tree::Dimension<'a, PathSummary<S>> for PathKey {
    fn zero(_: &S::Context) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathSummary<S>, _: &S::Context) {
        self.0 = summary.max_path.clone();
    }
}

impl<'a, S: Summary> sum_tree::Dimension<'a, PathSummary<S>> for TraversalProgress<'a> {
    fn zero(_cx: &S::Context) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathSummary<S>, _: &S::Context) {
        self.max_path = summary.max_path.as_ref();
    }
}

impl Entry {
    fn new(
        path: Arc<Path>,
        metadata: &fs::Metadata,
        next_entry_id: &AtomicUsize,
        root_char_bag: CharBag,
        canonical_path: Option<Arc<Path>>,
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
            is_always_included: false,
            is_external: false,
            is_private: false,
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
        let non_ignored_count = if (self.is_ignored || self.is_external) && !self.is_always_included
        {
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

        EntrySummary {
            max_path: self.path.clone(),
            count: 1,
            non_ignored_count,
            file_count,
            non_ignored_file_count,
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
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(Path::new("")),
            count: 0,
            non_ignored_count: 0,
            file_count: 0,
            non_ignored_file_count: 0,
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PathKey(pub Arc<Path>);

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
    path_prefixes_to_scan_rx: channel::Receiver<PathPrefixScanRequest>,
    next_entry_id: Arc<AtomicUsize>,
    phase: BackgroundScannerPhase,
    watcher: Arc<dyn Watcher>,
    settings: WorktreeSettings,
    share_private_files: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum BackgroundScannerPhase {
    InitialScan,
    EventsReceivedDuringInitialScan,
    Events,
}

impl BackgroundScanner {
    async fn run(&mut self, mut fs_events_rx: Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>) {
        // If the worktree root does not contain a git repository, then find
        // the git repository in an ancestor directory. Find any gitignore files
        // in ancestor directories.
        let root_abs_path = self.state.lock().snapshot.abs_path.clone();
        let (ignores, repo) = discover_ancestor_git_repo(self.fs.clone(), &root_abs_path).await;
        self.state
            .lock()
            .snapshot
            .ignores_by_parent_abs_path
            .extend(ignores);
        let containing_git_repository = repo.and_then(|(ancestor_dot_git, work_directory)| {
            self.state.lock().insert_git_repository_for_path(
                work_directory,
                ancestor_dot_git.as_path().into(),
                self.fs.as_ref(),
                self.watcher.as_ref(),
            )?;
            Some(ancestor_dot_git)
        });

        log::trace!("containing git repository: {containing_git_repository:?}");

        let (scan_job_tx, scan_job_rx) = channel::unbounded();
        {
            let mut state = self.state.lock();
            state.snapshot.scan_id += 1;
            if let Some(mut root_entry) = state.snapshot.root_entry().cloned() {
                let ignore_stack = state
                    .snapshot
                    .ignore_stack_for_abs_path(root_abs_path.as_path(), true);
                if ignore_stack.is_abs_path_ignored(root_abs_path.as_path(), true) {
                    root_entry.is_ignored = true;
                    state.insert_entry(root_entry.clone(), self.fs.as_ref(), self.watcher.as_ref());
                }
                state.enqueue_scan_dir(root_abs_path.into(), &root_entry, &scan_job_tx);
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
        if let Some(abs_path) = containing_git_repository {
            self.process_events(vec![abs_path]).await;
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

                path_prefix_request = self.path_prefixes_to_scan_rx.recv().fuse() => {
                    let Ok(request) = path_prefix_request else { break };
                    log::trace!("adding path prefix {:?}", request.path);

                    let did_scan = self.forcibly_load_paths(&[request.path.clone()]).await;
                    if did_scan {
                        let abs_path =
                        {
                            let mut state = self.state.lock();
                            state.path_prefixes_to_scan.insert(request.path.clone());
                            state.snapshot.abs_path.as_path().join(&request.path)
                        };

                        if let Some(abs_path) = self.fs.canonicalize(&abs_path).await.log_err() {
                            self.process_events(vec![abs_path]).await;
                        }
                    }
                    self.send_status_update(false, request.done);
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
        let root_canonical_path = match self.fs.canonicalize(root_path.as_path()).await {
            Ok(path) => SanitizedPath::from(path),
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
                    root_canonical_path.as_path().join(path).to_path_buf()
                } else {
                    root_canonical_path.as_path().to_path_buf()
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
        let root_canonical_path = match self.fs.canonicalize(root_path.as_path()).await {
            Ok(path) => SanitizedPath::from(path),
            Err(err) => {
                let new_path = self
                    .state
                    .lock()
                    .snapshot
                    .root_file_handle
                    .clone()
                    .and_then(|handle| handle.current_path(&self.fs).log_err())
                    .map(SanitizedPath::from)
                    .filter(|new_path| *new_path != root_path);

                if let Some(new_path) = new_path.as_ref() {
                    log::info!(
                        "root renamed from {} to {}",
                        root_path.as_path().display(),
                        new_path.as_path().display()
                    )
                } else {
                    log::warn!("root path could not be canonicalized: {}", err);
                }
                self.status_updates_tx
                    .unbounded_send(ScanState::RootUpdated { new_path })
                    .ok();
                return;
            }
        };

        // Certain directories may have FS changes, but do not lead to git data changes that Zed cares about.
        // Ignore these, to avoid Zed unnecessarily rescanning git metadata.
        let skipped_files_in_dot_git = HashSet::from_iter([*COMMIT_MESSAGE, *INDEX_LOCK]);
        let skipped_dirs_in_dot_git = [*FSMONITOR_DAEMON, *LFS_DIR];

        let mut relative_paths = Vec::with_capacity(abs_paths.len());
        let mut dot_git_abs_paths = Vec::new();
        abs_paths.sort_unstable();
        abs_paths.dedup_by(|a, b| a.starts_with(b));
        abs_paths.retain(|abs_path| {
            let abs_path = SanitizedPath::from(abs_path);

            let snapshot = &self.state.lock().snapshot;
            {
                let mut is_git_related = false;

                let dot_git_paths = abs_path.as_path().ancestors().find_map(|ancestor| {
                    if smol::block_on(is_git_dir(ancestor, self.fs.as_ref())) {
                        let path_in_git_dir = abs_path.as_path().strip_prefix(ancestor).expect("stripping off the ancestor");
                        Some((ancestor.to_owned(), path_in_git_dir.to_owned()))
                    } else {
                        None
                    }
                });

                if let Some((dot_git_abs_path, path_in_git_dir)) = dot_git_paths {
                    if skipped_files_in_dot_git.contains(path_in_git_dir.as_os_str()) || skipped_dirs_in_dot_git.iter().any(|skipped_git_subdir| path_in_git_dir.starts_with(skipped_git_subdir)) {
                        log::debug!("ignoring event {abs_path:?} as it's in the .git directory among skipped files or directories");
                        return false;
                    }

                    is_git_related = true;
                    if !dot_git_abs_paths.contains(&dot_git_abs_path) {
                        dot_git_abs_paths.push(dot_git_abs_path);
                    }
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

                if abs_path.0.file_name() == Some(*GITIGNORE) {
                    for (_, repo) in snapshot.git_repositories.iter().filter(|(_, repo)| repo.directory_contains(&relative_path)) {
                        if !dot_git_abs_paths.iter().any(|dot_git_abs_path| dot_git_abs_path == repo.common_dir_abs_path.as_ref()) {
                            dot_git_abs_paths.push(repo.common_dir_abs_path.to_path_buf());
                        }
                    }
                }

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
            self.update_git_repositories(dot_git_abs_paths);
        }

        {
            let mut state = self.state.lock();
            state.snapshot.completed_scan_id = state.snapshot.scan_id;
            for (_, entry) in mem::take(&mut state.removed_entries) {
                state.scanned_dirs.remove(&entry.id);
            }
        }
        self.send_status_update(false, SmallVec::new());
    }

    async fn forcibly_load_paths(&self, paths: &[Arc<Path>]) -> bool {
        let (scan_job_tx, scan_job_rx) = channel::unbounded();
        {
            let mut state = self.state.lock();
            let root_path = state.snapshot.abs_path.clone();
            for path in paths {
                for ancestor in path.ancestors() {
                    if let Some(entry) = state.snapshot.entry_for_path(ancestor) {
                        if entry.kind == EntryKind::UnloadedDir {
                            let abs_path = root_path.as_path().join(ancestor);
                            state.enqueue_scan_dir(abs_path.into(), entry, &scan_job_tx);
                            state.paths_to_scan.insert(path.clone());
                            break;
                        }
                    }
                }
            }
            drop(scan_job_tx);
        }
        while let Ok(job) = scan_job_rx.recv().await {
            self.scan_dir(&job).await.log_err();
        }

        !mem::take(&mut self.state.lock().paths_to_scan).is_empty()
    }

    async fn scan_dirs(
        &self,
        enable_progress_updates: bool,
        scan_jobs_rx: channel::Receiver<ScanJob>,
    ) {
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
                    });
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
        let changes = build_diff(
            self.phase,
            &old_snapshot,
            &new_snapshot,
            &state.changed_paths,
        );
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
            log::trace!("scanning directory {:?}", job.path);
            root_abs_path = snapshot.abs_path().clone();
            root_char_bag = snapshot.root_char_bag;
        }

        let next_entry_id = self.next_entry_id.clone();
        let mut ignore_stack = job.ignore_stack.clone();
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
                let mut state = self.state.lock();
                state.insert_git_repository(
                    child_path.clone(),
                    self.fs.as_ref(),
                    self.watcher.as_ref(),
                );
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
                child_entry.is_always_included = self.settings.is_path_always_included(&child_path);

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
                    }));
                }
            } else {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, false);
                child_entry.is_always_included = self.settings.is_path_always_included(&child_path);
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
            if entry.is_always_included {
                state
                    .snapshot
                    .always_included_entries
                    .push(entry.path.clone());
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

    /// All list arguments should be sorted before calling this function
    async fn reload_entries_for_paths(
        &self,
        root_abs_path: SanitizedPath,
        root_canonical_path: SanitizedPath,
        relative_paths: &[Arc<Path>],
        abs_paths: Vec<PathBuf>,
        scan_queue_tx: Option<Sender<ScanJob>>,
    ) {
        // grab metadata for all requested paths
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

                        anyhow::Ok(Some((metadata, SanitizedPath::from(canonical_path))))
                    } else {
                        Ok(None)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .await;

        let mut new_ancestor_repo = if relative_paths
            .iter()
            .any(|path| path.as_ref() == Path::new(""))
        {
            Some(discover_ancestor_git_repo(self.fs.clone(), &root_abs_path).await)
        } else {
            None
        };

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

        for (path, metadata) in relative_paths.iter().zip(metadata.into_iter()) {
            let abs_path: Arc<Path> = root_abs_path.as_path().join(path).into();
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
                            Some(canonical_path.as_path().to_path_buf().into())
                        } else {
                            None
                        },
                    );

                    let is_dir = fs_entry.is_dir();
                    fs_entry.is_ignored = ignore_stack.is_abs_path_ignored(&abs_path, is_dir);
                    fs_entry.is_external = is_external;
                    fs_entry.is_private = self.is_path_private(path);
                    fs_entry.is_always_included = self.settings.is_path_always_included(path);

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

                    state.insert_entry(fs_entry.clone(), self.fs.as_ref(), self.watcher.as_ref());

                    if path.as_ref() == Path::new("") {
                        if let Some((ignores, repo)) = new_ancestor_repo.take() {
                            log::trace!("updating ancestor git repository");
                            state.snapshot.ignores_by_parent_abs_path.extend(ignores);
                            if let Some((ancestor_dot_git, work_directory)) = repo {
                                state.insert_git_repository_for_path(
                                    work_directory,
                                    ancestor_dot_git.as_path().into(),
                                    self.fs.as_ref(),
                                    self.watcher.as_ref(),
                                );
                            }
                        }
                    }
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

    fn remove_repo_path(&self, path: &Arc<Path>, snapshot: &mut LocalSnapshot) -> Option<()> {
        if !path
            .components()
            .any(|component| component.as_os_str() == *DOT_GIT)
        {
            if let Some(local_repo) = snapshot.local_repo_for_work_directory_path(path) {
                let id = local_repo.work_directory_id;
                log::debug!("remove repo path: {:?}", path);
                snapshot.git_repositories.remove(&id);
                return Some(());
            }
        }

        Some(())
    }

    async fn update_ignore_statuses(&self, scan_job_tx: Sender<ScanJob>) {
        let mut ignores_to_update = Vec::new();
        let (ignore_queue_tx, ignore_queue_rx) = channel::unbounded();
        let prev_snapshot;
        {
            let snapshot = &mut self.state.lock().snapshot;
            let abs_path = snapshot.abs_path.clone();
            snapshot
                .ignores_by_parent_abs_path
                .retain(|parent_abs_path, (_, needs_update)| {
                    if let Ok(parent_path) = parent_abs_path.strip_prefix(abs_path.as_path()) {
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
        let path = job
            .abs_path
            .strip_prefix(snapshot.abs_path.as_path())
            .unwrap();

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

    fn update_git_repositories(&self, dot_git_paths: Vec<PathBuf>) {
        log::trace!("reloading repositories: {dot_git_paths:?}");
        let mut state = self.state.lock();
        let scan_id = state.snapshot.scan_id;
        for dot_git_dir in dot_git_paths {
            let existing_repository_entry =
                state
                    .snapshot
                    .git_repositories
                    .iter()
                    .find_map(|(_, repo)| {
                        if repo.common_dir_abs_path.as_ref() == &dot_git_dir
                            || repo.repository_dir_abs_path.as_ref() == &dot_git_dir
                        {
                            Some(repo.clone())
                        } else {
                            None
                        }
                    });

            match existing_repository_entry {
                None => {
                    let Ok(relative) = dot_git_dir.strip_prefix(state.snapshot.abs_path()) else {
                        return;
                    };
                    state.insert_git_repository(
                        relative.into(),
                        self.fs.as_ref(),
                        self.watcher.as_ref(),
                    );
                }
                Some(local_repository) => {
                    state.snapshot.git_repositories.update(
                        &local_repository.work_directory_id,
                        |entry| {
                            entry.git_dir_scan_id = scan_id;
                        },
                    );
                }
            };
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
                    smol::block_on(self.fs.metadata(&entry.common_dir_abs_path)),
                    Ok(Some(_))
                )
            {
                ids_to_preserve.insert(work_directory_id);
            }
        }

        snapshot
            .git_repositories
            .retain(|work_directory_id, _| ids_to_preserve.contains(work_directory_id));
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

async fn discover_ancestor_git_repo(
    fs: Arc<dyn Fs>,
    root_abs_path: &SanitizedPath,
) -> (
    HashMap<Arc<Path>, (Arc<Gitignore>, bool)>,
    Option<(PathBuf, WorkDirectory)>,
) {
    let mut ignores = HashMap::default();
    for (index, ancestor) in root_abs_path.as_path().ancestors().enumerate() {
        if index != 0 {
            if Some(ancestor) == fs.home_dir().as_deref() {
                // Unless $HOME is itself the worktree root, don't consider it as a
                // containing git repository---expensive and likely unwanted.
                break;
            } else if let Ok(ignore) =
                build_gitignore(&ancestor.join(*GITIGNORE), fs.as_ref()).await
            {
                ignores.insert(ancestor.into(), (ignore.into(), false));
            }
        }

        let ancestor_dot_git = ancestor.join(*DOT_GIT);
        log::trace!("considering ancestor: {ancestor_dot_git:?}");
        // Check whether the directory or file called `.git` exists (in the
        // case of worktrees it's a file.)
        if fs
            .metadata(&ancestor_dot_git)
            .await
            .is_ok_and(|metadata| metadata.is_some())
        {
            if index != 0 {
                // We canonicalize, since the FS events use the canonicalized path.
                if let Some(ancestor_dot_git) = fs.canonicalize(&ancestor_dot_git).await.log_err() {
                    let location_in_repo = root_abs_path
                        .as_path()
                        .strip_prefix(ancestor)
                        .unwrap()
                        .into();
                    log::info!("inserting parent git repo for this worktree: {location_in_repo:?}");
                    // We associate the external git repo with our root folder and
                    // also mark where in the git repo the root folder is located.
                    return (
                        ignores,
                        Some((
                            ancestor_dot_git,
                            WorkDirectory::AboveProject {
                                absolute_path: ancestor.into(),
                                location_in_repo,
                            },
                        )),
                    );
                };
            }

            // Reached root of git repository.
            break;
        }
    }

    (ignores, None)
}

fn build_diff(
    phase: BackgroundScannerPhase,
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
                            if phase == EventsReceivedDuringInitialScan {
                                if old_entry.id != new_entry.id {
                                    changes.push((old_entry.path.clone(), old_entry.id, Removed));
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
                                    changes.push((new_entry.path.clone(), new_entry.id, Loaded));
                                } else {
                                    changes.push((new_entry.path.clone(), new_entry.id, Updated));
                                }
                            }
                            old_paths.next(&());
                            new_paths.next(&());
                        }
                        Ordering::Greater => {
                            let is_newly_loaded = phase == InitialScan
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
                    let is_newly_loaded = phase == InitialScan
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

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    scan_queue: Sender<ScanJob>,
    ancestor_inodes: TreeSet<u64>,
    is_external: bool,
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

    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events_in_root_git_repository<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;
}

impl WorktreeModelHandle for Entity<Worktree> {
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

            let mut events = cx.events(&tree);
            while events.next().await.is_some() {
                if tree.update(cx, |tree, _| tree.entry_for_path(file_name).is_some()) {
                    break;
                }
            }

            fs.remove_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();
            while events.next().await.is_some() {
                if tree.update(cx, |tree, _| tree.entry_for_path(file_name).is_none()) {
                    break;
                }
            }

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
            let local_repo_entry = tree
                .git_repositories
                .values()
                .min_by_key(|local_repo_entry| local_repo_entry.work_directory.clone())
                .unwrap();
            (
                tree.fs.clone(),
                local_repo_entry.common_dir_abs_path.clone(),
                local_repo_entry.git_dir_scan_id,
            )
        });

        let scan_id_increased = |tree: &mut Worktree, git_dir_scan_id: &mut usize| {
            let tree = tree.as_local().unwrap();
            // let repository = tree.repositories.first().unwrap();
            let local_repo_entry = tree
                .git_repositories
                .values()
                .min_by_key(|local_repo_entry| local_repo_entry.work_directory.clone())
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

            let mut events = cx.events(&tree);
            while events.next().await.is_some() {
                if tree.update(cx, |tree, _| scan_id_increased(tree, &mut git_dir_scan_id)) {
                    break;
                }
            }

            fs.remove_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            while events.next().await.is_some() {
                if tree.update(cx, |tree, _| scan_id_increased(tree, &mut git_dir_scan_id)) {
                    break;
                }
            }

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

impl TraversalProgress<'_> {
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

impl Default for TraversalProgress<'_> {
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

#[derive(Debug)]
pub struct Traversal<'a> {
    snapshot: &'a Snapshot,
    cursor: sum_tree::Cursor<'a, Entry, TraversalProgress<'a>>,
    include_ignored: bool,
    include_files: bool,
    include_dirs: bool,
}

impl<'a> Traversal<'a> {
    fn new(
        snapshot: &'a Snapshot,
        include_files: bool,
        include_dirs: bool,
        include_ignored: bool,
        start_path: &Path,
    ) -> Self {
        let mut cursor = snapshot.entries_by_path.cursor(&());
        cursor.seek(&TraversalTarget::path(start_path), Bias::Left, &());
        let mut traversal = Self {
            snapshot,
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
            self.cursor
                .seek_forward(&TraversalTarget::successor(&entry.path), Bias::Left, &());
            if let Some(entry) = self.cursor.item() {
                if (self.include_files || !entry.is_file())
                    && (self.include_dirs || !entry.is_dir())
                    && (self.include_ignored || !entry.is_ignored || entry.is_always_included)
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
            .seek(&TraversalTarget::path(parent_path), Bias::Left, &())
    }

    pub fn entry(&self) -> Option<&'a Entry> {
        self.cursor.item()
    }

    pub fn snapshot(&self) -> &'a Snapshot {
        self.snapshot
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

#[derive(Debug, Clone, Copy)]
pub enum PathTarget<'a> {
    Path(&'a Path),
    Successor(&'a Path),
}

impl PathTarget<'_> {
    fn cmp_path(&self, other: &Path) -> Ordering {
        match self {
            PathTarget::Path(path) => path.cmp(&other),
            PathTarget::Successor(path) => {
                if other.starts_with(path) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl<'a, S: Summary> SeekTarget<'a, PathSummary<S>, PathProgress<'a>> for PathTarget<'_> {
    fn cmp(&self, cursor_location: &PathProgress<'a>, _: &S::Context) -> Ordering {
        self.cmp_path(&cursor_location.max_path)
    }
}

impl<'a, S: Summary> SeekTarget<'a, PathSummary<S>, TraversalProgress<'a>> for PathTarget<'_> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: &S::Context) -> Ordering {
        self.cmp_path(&cursor_location.max_path)
    }
}

#[derive(Debug)]
enum TraversalTarget<'a> {
    Path(PathTarget<'a>),
    Count {
        count: usize,
        include_files: bool,
        include_ignored: bool,
        include_dirs: bool,
    },
}

impl<'a> TraversalTarget<'a> {
    fn path(path: &'a Path) -> Self {
        Self::Path(PathTarget::Path(path))
    }

    fn successor(path: &'a Path) -> Self {
        Self::Path(PathTarget::Successor(path))
    }

    fn cmp_progress(&self, progress: &TraversalProgress) -> Ordering {
        match self {
            TraversalTarget::Path(path) => path.cmp_path(&progress.max_path),
            TraversalTarget::Count {
                count,
                include_files,
                include_dirs,
                include_ignored,
            } => Ord::cmp(
                count,
                &progress.count(*include_files, *include_dirs, *include_ignored),
            ),
        }
    }
}

impl<'a> SeekTarget<'a, EntrySummary, TraversalProgress<'a>> for TraversalTarget<'_> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: &()) -> Ordering {
        self.cmp_progress(cursor_location)
    }
}

impl<'a> SeekTarget<'a, PathSummary<Unit>, TraversalProgress<'a>> for TraversalTarget<'_> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: &()) -> Ordering {
        self.cmp_progress(cursor_location)
    }
}

pub struct ChildEntriesOptions {
    pub include_files: bool,
    pub include_dirs: bool,
    pub include_ignored: bool,
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
            path: entry.path.as_ref().to_proto(),
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            is_ignored: entry.is_ignored,
            is_external: entry.is_external,
            is_fifo: entry.is_fifo,
            size: Some(entry.size),
            canonical_path: entry
                .canonical_path
                .as_ref()
                .map(|path| path.as_ref().to_proto()),
        }
    }
}

impl<'a> TryFrom<(&'a CharBag, &PathMatcher, proto::Entry)> for Entry {
    type Error = anyhow::Error;

    fn try_from(
        (root_char_bag, always_included, entry): (&'a CharBag, &PathMatcher, proto::Entry),
    ) -> Result<Self> {
        let kind = if entry.is_dir {
            EntryKind::Dir
        } else {
            EntryKind::File
        };

        let path = Arc::<Path>::from_proto(entry.path);
        let char_bag = char_bag_for_path(*root_char_bag, &path);
        let is_always_included = always_included.is_match(path.as_ref());
        Ok(Entry {
            id: ProjectEntryId::from_proto(entry.id),
            kind,
            path,
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            size: entry.size.unwrap_or(0),
            canonical_path: entry
                .canonical_path
                .map(|path_string| Arc::from(PathBuf::from_proto(path_string))),
            is_ignored: entry.is_ignored,
            is_always_included,
            is_external: entry.is_external,
            is_private: false,
            char_bag,
            is_fifo: entry.is_fifo,
        })
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

    pub fn from_usize(id: usize) -> Self {
        ProjectEntryId(id)
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

fn parse_gitfile(content: &str) -> anyhow::Result<&Path> {
    let path = content
        .strip_prefix("gitdir:")
        .ok_or_else(|| anyhow!("failed to parse gitfile content {content:?}"))?;
    Ok(Path::new(path.trim()))
}

fn discover_git_paths(dot_git_abs_path: &Arc<Path>, fs: &dyn Fs) -> (Arc<Path>, Arc<Path>) {
    let mut repository_dir_abs_path = dot_git_abs_path.clone();
    let mut common_dir_abs_path = dot_git_abs_path.clone();

    if let Some(path) = smol::block_on(fs.load(&dot_git_abs_path))
        .ok()
        .as_ref()
        .and_then(|contents| parse_gitfile(contents).log_err())
    {
        let path = dot_git_abs_path
            .parent()
            .unwrap_or(Path::new(""))
            .join(path);
        if let Some(path) = smol::block_on(fs.canonicalize(&path)).log_err() {
            repository_dir_abs_path = Path::new(&path).into();
            common_dir_abs_path = repository_dir_abs_path.clone();
            if let Some(commondir_contents) = smol::block_on(fs.load(&path.join("commondir"))).ok()
            {
                if let Some(commondir_path) =
                    smol::block_on(fs.canonicalize(&path.join(commondir_contents.trim()))).log_err()
                {
                    common_dir_abs_path = commondir_path.as_path().into();
                }
            }
        }
    };

    (repository_dir_abs_path, common_dir_abs_path)
}
