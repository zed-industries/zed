mod ignore;
mod worktree_settings;

use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{Context as _, Result, anyhow};
use chardetng::EncodingDetector;
use clock::ReplicaId;
use collections::{BTreeMap, HashMap, HashSet, VecDeque};
use encoding_rs::Encoding;
use fs::{
    Fs, MTime, PathEvent, PathEventKind, RemoveOptions, TrashId, Watcher, copy_recursive,
    io_error_is_absence, read_dir_items,
};
use futures::{
    FutureExt as _, Stream, StreamExt,
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    select_biased, stream,
    task::Poll,
};
use fuzzy::CharBag;
use git::{
    BISECT_LOG, COMMIT_MESSAGE, DOT_GIT, FETCH_HEAD, FSMONITOR_DAEMON, GC_PID, GITIGNORE,
    HOOKS_DIR, INFO_DIR, LFS_DIR, LOGS_DIR, LOGS_REF_STASH, OBJECTS_DIR, ORIG_HEAD,
    REBASE_APPLY_DIR, REBASE_MERGE_DIR, REFS_DIR, REFTABLE_DIR, REPO_EXCLUDE, SEQUENCER_DIR,
    status::GitSummary,
};
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, Priority,
    Task,
};
use ignore::IgnoreStack;
use language::{ByteContent, DiskState, FILE_ANALYSIS_BYTES, analyze_byte_content};

use async_channel::{self, Sender};
use parking_lot::Mutex;
use paths::{local_settings_folder_name, local_vscode_folder_name};
use postage::{
    barrier,
    prelude::{Sink as _, Stream as _},
    watch,
};
use rpc::{
    AnyProtoClient,
    proto::{self, split_worktree_update},
};
pub use settings::WorktreeId;
use settings::{Settings, SettingsLocation, SettingsStore};
use smallvec::{SmallVec, smallvec};
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
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use sum_tree::{Bias, Dimensions, Edit, SeekTarget, SumTree, Summary, TreeMap, TreeSet};
use text::{LineEnding, Rope};
use util::{
    ResultExt,
    paths::{PathMatcher, PathStyle, SanitizedPath, home_dir},
    rel_path::RelPath,
};
pub use worktree_settings::WorktreeSettings;

use crate::ignore::IgnoreKind;

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

#[derive(Debug)]
pub struct LoadedFile {
    pub file: Arc<File>,
    pub text: String,
    pub encoding: &'static Encoding,
    pub has_bom: bool,
    pub is_writable: bool,
}

pub struct LoadedBinaryFile {
    pub file: Arc<File>,
    pub content: Vec<u8>,
}

impl fmt::Debug for LoadedBinaryFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadedBinaryFile")
            .field("file", &self.file)
            .field("content_bytes", &self.content.len())
            .finish()
    }
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    scan_requests_tx: async_channel::Sender<ScanRequest>,
    path_prefixes_to_scan_tx: async_channel::Sender<PathPrefixScanRequest>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    snapshot_subscriptions: VecDeque<(usize, oneshot::Sender<()>)>,
    _background_scanner_tasks: Vec<Task<()>>,
    update_observer: Option<UpdateObservationState>,
    fs: Arc<dyn Fs>,
    fs_case_sensitive: bool,
    visible: bool,
    next_entry_id: Arc<AtomicUsize>,
    settings: WorktreeSettings,
    share_private_files: bool,
    scanning_enabled: bool,
    force_defer_watch: bool,
}

pub struct PathPrefixScanRequest {
    path: Arc<RelPath>,
    done: SmallVec<[barrier::Sender; 1]>,
}

struct ScanRequest {
    relative_paths: Vec<Arc<RelPath>>,
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
    received_initial_update: bool,
}

#[derive(Clone)]
pub struct Snapshot {
    id: WorktreeId,
    /// The absolute path of the worktree root.
    abs_path: Arc<SanitizedPath>,
    path_style: PathStyle,
    root_name: Arc<RelPath>,
    root_char_bag: CharBag,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
    root_repo_common_dir: Option<Arc<SanitizedPath>>,
    root_repo_is_linked_worktree: bool,
    always_included_entries: Vec<Arc<RelPath>>,

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
    InProject { relative_path: Arc<RelPath> },
    AboveProject { absolute_path: Arc<Path> },
}

impl WorkDirectory {
    fn path_key(&self) -> PathKey {
        match self {
            WorkDirectory::InProject { relative_path } => PathKey(relative_path.clone()),
            WorkDirectory::AboveProject { .. } => PathKey(RelPath::empty_arc()),
        }
    }

    /// Returns true if the given path is a child of the work directory.
    ///
    /// Note that the path may not be a member of this repository, if there
    /// is a repository in a directory between these two paths
    /// external .git folder in a parent folder of the project root.
    #[track_caller]
    pub fn directory_contains(&self, path: &RelPath) -> bool {
        match self {
            WorkDirectory::InProject { relative_path } => path.starts_with(relative_path),
            WorkDirectory::AboveProject { .. } => true,
        }
    }
}

impl Default for WorkDirectory {
    fn default() -> Self {
        Self::InProject {
            relative_path: Arc::from(RelPath::empty()),
        }
    }
}

/// Absolute identity of a registered git repository.
///
/// Paths are local-only and never enter the worktree proto envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitRepositoryIdentity {
    pub work_directory_abs_path: Arc<Path>,
    pub dot_git_abs_path: Arc<Path>,
    pub repository_dir_abs_path: Arc<Path>,
    pub common_dir_abs_path: Arc<Path>,
}

#[derive(Clone)]
pub struct LocalSnapshot {
    snapshot: Snapshot,
    global_gitignore: Option<Arc<Gitignore>>,
    /// All of the gitignore files in the worktree, indexed by their absolute path.
    /// The boolean indicates whether the gitignore needs to be updated.
    ignores_by_parent_abs_path: HashMap<Arc<Path>, (Arc<Gitignore>, bool)>,
    /// All of the git repositories in the worktree, indexed by the project entry
    /// id of their parent directory.
    git_repositories: TreeMap<ProjectEntryId, LocalRepositoryEntry>,
    /// The file handle of the worktree root
    /// (so we can find it after it's been moved)
    root_file_handle: Option<Arc<dyn fs::FileHandle>>,
    /// Maps canonical absolute paths of externally watched symlinked directories
    /// to their relative paths within the worktree, used to translate FSEvents
    /// canonical-path events back to worktree-relative paths.
    external_canonical_to_relative: BTreeMap<Arc<Path>, Arc<RelPath>>,
}

struct BackgroundScannerState {
    snapshot: LocalSnapshot,
    symlink_paths_by_target: HashMap<Arc<Path>, SmallVec<[Arc<RelPath>; 1]>>,
    scanned_dirs: HashSet<ProjectEntryId>,
    watched_dir_abs_paths_by_entry_id: HashMap<ProjectEntryId, Arc<Path>>,
    path_prefixes_to_scan: HashSet<Arc<RelPath>>,
    paths_to_scan: HashSet<Arc<RelPath>>,
    removed_entries: RemovedEntries,
    changed_paths: Vec<Arc<RelPath>>,
    prev_snapshot: Snapshot,
    scanning_enabled: bool,
}

/// The entries that were removed from the snapshot as part of the current
/// update. Their entry ids may be re-used if the same inode is discovered
/// at a new path, or if the given path is re-created after being deleted.
///
/// Symlink aliases inside the worktree share their inode (and usually mtime)
/// with the symlink target, so an inode may correspond to several entries.
/// The path index allows an exact match to take precedence over the
/// inode-based rename heuristics in that case.
#[derive(Default)]
struct RemovedEntries {
    by_inode: HashMap<u64, Entry>,
    by_path: HashMap<Arc<RelPath>, Entry>,
}

impl RemovedEntries {
    fn insert(&mut self, entry: &Entry) {
        self.by_path.insert(entry.path.clone(), entry.clone());
        match self.by_inode.entry(entry.inode) {
            hash_map::Entry::Occupied(mut o) => {
                if entry.id > o.get().id {
                    o.insert(entry.clone());
                }
            }
            hash_map::Entry::Vacant(v) => {
                v.insert(entry.clone());
            }
        }
    }

    fn take_by_path(&mut self, path: &RelPath, inode: u64) -> Option<Entry> {
        if self.by_path.get(path)?.inode != inode {
            return None;
        }
        let removed = self.by_path.remove(path)?;
        if let hash_map::Entry::Occupied(o) = self.by_inode.entry(removed.inode)
            && o.get().id == removed.id
        {
            o.remove();
        }
        Some(removed)
    }

    fn take_by_inode(&mut self, inode: u64) -> Option<Entry> {
        let removed = self.by_inode.remove(&inode)?;
        if let hash_map::Entry::Occupied(o) = self.by_path.entry(removed.path.clone())
            && o.get().id == removed.id
        {
            o.remove();
        }
        Some(removed)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EventRoot {
    path: Arc<RelPath>,
    was_rescanned: bool,
}

#[derive(Clone, Debug)]
struct LocalRepositoryEntry {
    identity: GitRepositoryIdentity,
    git_dir_scan_id: usize,
    /// Cached `info/exclude` for this repository (owned by the registration).
    info_exclude: Arc<Gitignore>,
    /// When true, re-read `info/exclude` from disk on the next batch.
    /// Opposite of `ignores_by_parent_abs_path`'s dirty bit (which means recompute
    /// statuses without re-reading).
    info_exclude_needs_update: bool,
}

/// Outcome of inserting a validated repository into the local snapshot.
struct RegistrationOutcome {
    work_directory_id: ProjectEntryId,
    was_added: bool,
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
    RootUpdated {
        new_path: Arc<SanitizedPath>,
    },
    RootDeleted,
}

struct UpdateObservationState {
    snapshots_tx: mpsc::UnboundedSender<(LocalSnapshot, UpdatedEntriesSet)>,
    resume_updates: watch::Sender<()>,
    _maintain_remote_snapshot: Task<Option<()>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    UpdatedEntries(UpdatedEntriesSet),
    UpdatedGitRepositories(GitRepositoryChanges),
    UpdatedRootRepoCommonDir {
        old: Option<Arc<SanitizedPath>>,
    },
    DeletedEntry(ProjectEntryId),
    /// The worktree root itself has been deleted (for single-file worktrees)
    Deleted,
}

impl EventEmitter<Event> for Worktree {}

impl Worktree {
    pub async fn local(
        path: impl Into<Arc<Path>>,
        visible: bool,
        fs: Arc<dyn Fs>,
        next_entry_id: Arc<AtomicUsize>,
        scanning_enabled: bool,
        worktree_id: WorktreeId,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let abs_path = path.into();
        let metadata = fs
            .metadata(&abs_path)
            .await
            .context("failed to stat worktree path")?;

        let fs_case_sensitive = fs.is_case_sensitive().await;

        let root_file_handle = if metadata.as_ref().is_some() {
            fs.open_handle(&abs_path)
                .await
                .with_context(|| {
                    format!(
                        "failed to open local worktree root at {}",
                        abs_path.display()
                    )
                })
                .log_err()
        } else {
            None
        };

        let (root_repo_common_dir, root_repo_is_linked_worktree) = if visible {
            discover_root_repo_metadata(&abs_path, fs.as_ref())
                .await
                .map(|(common_dir, is_linked_worktree)| {
                    (
                        Some(SanitizedPath::from_arc(common_dir)),
                        is_linked_worktree,
                    )
                })
                .unwrap_or((None, false))
        } else {
            (None, false)
        };
        Ok(cx.new(move |cx: &mut Context<Worktree>| {
            let mut snapshot = LocalSnapshot {
                ignores_by_parent_abs_path: Default::default(),
                global_gitignore: Default::default(),
                git_repositories: Default::default(),
                external_canonical_to_relative: Default::default(),
                snapshot: Snapshot::new(
                    worktree_id,
                    abs_path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .map_or(RelPath::empty_arc(), |f| {
                            RelPath::from_unix_str(f).unwrap().into()
                        }),
                    abs_path.clone(),
                    PathStyle::local(),
                ),
                root_file_handle,
            };
            snapshot.root_repo_common_dir = root_repo_common_dir;
            snapshot.root_repo_is_linked_worktree = root_repo_is_linked_worktree;

            let worktree_id = snapshot.id();
            let settings_location = Some(SettingsLocation {
                worktree_id,
                path: RelPath::empty(),
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
                    RelPath::empty_arc(),
                    &metadata,
                    ProjectEntryId::new(&next_entry_id),
                    snapshot.root_char_bag,
                    None,
                );
                if metadata.is_dir {
                    if !scanning_enabled {
                        entry.kind = EntryKind::UnloadedDir;
                    }
                } else {
                    if let Some(file_name) = abs_path.file_name()
                        && let Some(file_name) = file_name.to_str()
                        && let Ok(path) = RelPath::from_unix_str(file_name)
                    {
                        entry.is_private = !share_private_files && settings.is_path_private(path);
                        entry.is_hidden = settings.is_path_hidden(path);
                    }
                }
                cx.foreground_executor()
                    .block_on(snapshot.insert_entry(entry, fs.as_ref()));
            }

            let (scan_requests_tx, scan_requests_rx) = async_channel::unbounded();
            let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = async_channel::unbounded();
            let mut worktree = LocalWorktree {
                share_private_files,
                next_entry_id,
                snapshot,
                is_scanning: watch::channel_with(true),
                snapshot_subscriptions: Default::default(),
                update_observer: None,
                scan_requests_tx,
                path_prefixes_to_scan_tx,
                _background_scanner_tasks: Vec::new(),
                fs,
                fs_case_sensitive,
                visible,
                settings,
                scanning_enabled,
                force_defer_watch: false,
            };
            worktree.start_background_scanner(scan_requests_rx, path_prefixes_to_scan_rx, cx);
            Worktree::Local(worktree)
        }))
    }

    pub fn remote(
        project_id: u64,
        replica_id: ReplicaId,
        worktree: proto::WorktreeMetadata,
        client: AnyProtoClient,
        path_style: PathStyle,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let mut snapshot = Snapshot::new(
                WorktreeId::from_proto(worktree.id),
                RelPath::from_unix_str(&worktree.root_name)
                    .map_or_else(|_| RelPath::empty_arc(), Into::into),
                Path::new(&worktree.abs_path).into(),
                path_style,
            );

            snapshot.root_repo_common_dir = worktree
                .root_repo_common_dir
                .map(|p| SanitizedPath::new_arc(Path::new(&p)));
            snapshot.root_repo_is_linked_worktree = worktree.root_repo_is_linked_worktree;

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
                path: RelPath::empty(),
            });

            let settings = WorktreeSettings::get(settings_location, cx).clone();
            let worktree = RemoteWorktree {
                client,
                project_id,
                replica_id,
                snapshot,
                file_scan_inclusions: settings.parent_dir_scan_inclusions.clone(),
                background_snapshot: background_snapshot.clone(),
                updates_tx: Some(background_updates_tx),
                update_observer: None,
                snapshot_subscriptions: Default::default(),
                visible: worktree.visible,
                disconnected: false,
                received_initial_update: false,
            };

            // Apply updates to a separate snapshot in a background task, then
            // send them to a foreground task which updates the model.
            cx.background_spawn(async move {
                while let Some(update) = background_updates_rx.next().await {
                    {
                        let mut lock = background_snapshot.lock();
                        lock.0.apply_remote_update(
                            update.clone(),
                            &settings.parent_dir_scan_inclusions,
                        );
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
                        let this = this.as_remote_mut().unwrap();

                        // The watch channel delivers an initial signal before
                        // any real updates arrive. Skip these spurious wakeups.
                        if this.background_snapshot.lock().1.is_empty() {
                            return;
                        }

                        let old_root_repo_common_dir = this.snapshot.root_repo_common_dir.clone();
                        let old_root_repo_is_linked_worktree =
                            this.snapshot.root_repo_is_linked_worktree;
                        let mut changed_entries: Vec<(Arc<RelPath>, ProjectEntryId, PathChange)> =
                            Vec::new();
                        {
                            let mut lock = this.background_snapshot.lock();
                            // Replace the snapshot, keeping the previous one around so we can
                            // resolve the paths of removed entries (the new snapshot no longer
                            // contains them, and the wire format only carries their ids).
                            let old_snapshot = mem::replace(&mut this.snapshot, lock.0.clone());
                            for update in lock.1.drain(..) {
                                for entry_id in &update.removed_entries {
                                    let entry_id = ProjectEntryId::from_proto(*entry_id);
                                    if let Some(entry) = old_snapshot.entry_for_id(entry_id) {
                                        changed_entries.push((
                                            entry.path.clone(),
                                            entry_id,
                                            PathChange::Removed,
                                        ));
                                    }
                                }
                                for entry in &update.updated_entries {
                                    // Remote updates don't distinguish creation from
                                    // modification, so report `AddedOrUpdated`.
                                    if let Some(path) =
                                        RelPath::from_unix_str(&entry.path).log_err()
                                    {
                                        changed_entries.push((
                                            path.into(),
                                            ProjectEntryId::from_proto(entry.id),
                                            PathChange::AddedOrUpdated,
                                        ));
                                    }
                                }
                                if let Some(tx) = &this.update_observer {
                                    tx.unbounded_send(update).ok();
                                }
                            }
                        };

                        if !changed_entries.is_empty() {
                            cx.emit(Event::UpdatedEntries(changed_entries.into()));
                        }
                        let is_first_update = !this.received_initial_update;
                        this.received_initial_update = true;
                        if this.snapshot.root_repo_common_dir != old_root_repo_common_dir
                            || this.snapshot.root_repo_is_linked_worktree
                                != old_root_repo_is_linked_worktree
                            || (is_first_update && this.snapshot.root_repo_common_dir.is_none())
                        {
                            cx.emit(Event::UpdatedRootRepoCommonDir {
                                old: old_root_repo_common_dir,
                            });
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
            path: RelPath::empty(),
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
            root_name: self.root_name().as_unix_str().to_owned(),
            visible: self.is_visible(),
            abs_path: self.abs_path().to_string_lossy().into_owned(),
            root_repo_common_dir: self
                .root_repo_common_dir()
                .map(|p| p.to_string_lossy().into_owned()),
            root_repo_is_linked_worktree: self.root_repo_is_linked_worktree(),
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
            Worktree::Local(_) => ReplicaId::LOCAL,
            Worktree::Remote(worktree) => worktree.replica_id,
        }
    }

    pub fn abs_path(&self) -> Arc<Path> {
        match self {
            Worktree::Local(worktree) => SanitizedPath::cast_arc(worktree.abs_path.clone()),
            Worktree::Remote(worktree) => SanitizedPath::cast_arc(worktree.abs_path.clone()),
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

    pub fn wait_for_snapshot(
        &mut self,
        scan_id: usize,
    ) -> impl Future<Output = Result<()>> + use<> {
        match self {
            Worktree::Local(this) => this.wait_for_snapshot(scan_id).boxed(),
            Worktree::Remote(this) => this.wait_for_snapshot(scan_id).boxed(),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn has_update_observer(&self) -> bool {
        match self {
            Worktree::Local(this) => this.update_observer.is_some(),
            Worktree::Remote(this) => this.update_observer.is_some(),
        }
    }

    pub fn load_file(&self, path: &RelPath, cx: &Context<Worktree>) -> Task<Result<LoadedFile>> {
        match self {
            Worktree::Local(this) => this.load_file(path, cx),
            Worktree::Remote(_) => {
                Task::ready(Err(anyhow!("remote worktrees can't yet load files")))
            }
        }
    }

    pub fn load_binary_file(
        &self,
        path: &RelPath,
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
        path: Arc<RelPath>,
        text: Rope,
        line_ending: LineEnding,
        encoding: &'static Encoding,
        has_bom: bool,
        cx: &Context<Worktree>,
    ) -> Task<Result<Arc<File>>> {
        match self {
            Worktree::Local(this) => {
                this.write_file(path, text, line_ending, encoding, has_bom, cx)
            }
            Worktree::Remote(_) => {
                Task::ready(Err(anyhow!("remote worktree can't yet write files")))
            }
        }
    }

    pub fn create_entry(
        &mut self,
        path: Arc<RelPath>,
        is_directory: bool,
        content: Option<Vec<u8>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let worktree_id = self.id();
        match self {
            Worktree::Local(this) => this.create_entry(path, is_directory, content, cx),
            Worktree::Remote(this) => {
                let project_id = this.project_id;
                let request = this.client.request(proto::CreateProjectEntry {
                    worktree_id: worktree_id.to_proto(),
                    project_id,
                    path: path.as_ref().as_unix_str().to_owned(),
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
                            let abs_path =
                                this.read_with(cx, |worktree, _| worktree.absolutize(&path))?;
                            Ok(CreatedEntry::Excluded { abs_path })
                        }
                    }
                })
            }
        }
    }

    pub fn trash_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut Context<Worktree>,
    ) -> Option<Task<Result<TrashId>>> {
        let entry = match self {
            Worktree::Local(this) => this.entry_for_id(entry_id),
            Worktree::Remote(this) => this.entry_for_id(entry_id),
        }?
        .clone();

        let task = match self {
            Worktree::Local(this) => this.trash_entry(entry.clone(), cx),
            Worktree::Remote(this) => this.trash_entry(entry_id, cx),
        };

        let mut ids = vec![entry_id];
        self.get_children_ids_recursive(&entry.path, &mut ids);

        for id in ids {
            cx.emit(Event::DeletedEntry(id));
        }
        Some(task)
    }

    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut Context<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let entry = match self {
            Worktree::Local(this) => this.entry_for_id(entry_id),
            Worktree::Remote(this) => this.entry_for_id(entry_id),
        }?
        .clone();

        let task = match self {
            Worktree::Local(this) => this.delete_entry(entry.clone(), cx),
            Worktree::Remote(this) => this.delete_entry(entry_id, cx),
        };

        let mut ids = vec![entry_id];
        let path = entry.path;

        self.get_children_ids_recursive(&path, &mut ids);

        for id in ids {
            cx.emit(Event::DeletedEntry(id));
        }
        Some(task)
    }

    pub fn restore_entry(
        &mut self,
        trash_id: TrashId,
        cx: &mut Context<'_, Worktree>,
    ) -> Task<Result<Entry>> {
        match self {
            Worktree::Local(this) => this.restore_entry(trash_id, cx),
            Worktree::Remote(this) => this.restore_entry(trash_id, cx),
        }
    }

    fn get_children_ids_recursive(&self, path: &RelPath, ids: &mut Vec<ProjectEntryId>) {
        let children_iter = self.child_entries(path);
        for child in children_iter {
            ids.push(child.id);
            self.get_children_ids_recursive(&child.path, ids);
        }
    }

    pub fn copy_external_entries(
        &mut self,
        target_directory: Arc<RelPath>,
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
            anyhow::Ok((
                this.scan_id(),
                this.create_entry(
                    RelPath::from_unix_str(&request.path)
                        .with_context(|| {
                            format!("received invalid relative path {:?}", request.path)
                        })?
                        .into(),
                    request.is_directory,
                    request.content,
                    cx,
                ),
            ))
        })?;
        Ok(proto::ProjectEntryResponse {
            entry: match &entry.await? {
                CreatedEntry::Included(entry) => Some(entry.into()),
                CreatedEntry::Excluded { .. } => None,
            },
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_trash_entry(
        this: Entity<Self>,
        request: proto::TrashProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::TrashProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.trash_entry(ProjectEntryId::from_proto(request.entry_id), cx),
            )
        });
        let trash_id = task
            .ok_or_else(|| anyhow::anyhow!("invalid entry"))?
            .await?;

        Ok(proto::TrashProjectEntryResponse {
            trash_id: trash_id.to_proto(),
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_delete_entry(
        this: Entity<Self>,
        request: proto::DeleteProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            // While the `use_trash` field is deprecated but not removed, we
            // still need to support either trashing or deleting the file.
            // Otherwise, if an older client sends the `DeleteProjectEntry {
            // use_trash: true }` rather than the newer `TrashProjectEntry`, and
            // the flag was ignored, we'd permanently delete a file that was
            // actually meant to be trashed.
            #[allow(deprecated)]
            let task = if request.use_trash {
                this.trash_entry(ProjectEntryId::from_proto(request.entry_id), cx)
                    .map(|task| cx.background_spawn(async move { task.await.map(|_| ()) }))
            } else {
                this.delete_entry(ProjectEntryId::from_proto(request.entry_id), cx)
            };

            (this.scan_id(), task)
        });
        task.ok_or_else(|| anyhow::anyhow!("invalid entry"))?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: None,
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_restore_entry(
        this: Entity<Self>,
        request: proto::RestoreProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::RestoreProjectEntryResponse> {
        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            (
                this.scan_id(),
                this.restore_entry(TrashId::from_proto(request.trash_id), cx),
            )
        });

        let entry = task.await?;

        Ok(proto::RestoreProjectEntryResponse {
            entry: Some(proto::Entry::from(&entry)),
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
        });
        task.ok_or_else(|| anyhow::anyhow!("no such entry"))?
            .await?;
        let scan_id = this.read_with(&cx, |this, _| this.scan_id());
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
        });
        task.ok_or_else(|| anyhow::anyhow!("no such entry"))?
            .await?;
        let scan_id = this.read_with(&cx, |this, _| this.scan_id());
        Ok(proto::ExpandAllForProjectEntryResponse {
            worktree_scan_id: scan_id as u64,
        })
    }

    pub fn is_single_file(&self) -> bool {
        self.root_dir().is_none()
    }

    /// For visible worktrees, returns the path with the worktree name as the first component.
    /// Otherwise, returns an absolute path.
    pub fn full_path(&self, worktree_relative_path: &RelPath) -> PathBuf {
        if self.is_visible() {
            self.root_name()
                .join(worktree_relative_path)
                .display(self.path_style)
                .to_string()
                .into()
        } else {
            let full_path = self.abs_path();
            let mut full_path_string = if self.is_local()
                && let Ok(stripped) = full_path.strip_prefix(home_dir())
            {
                self.path_style
                    .join("~", &*stripped.to_string_lossy())
                    .unwrap()
            } else {
                full_path.to_string_lossy().into_owned()
            };

            if worktree_relative_path.components().next().is_some() {
                full_path_string.push_str(self.path_style.primary_separator());
                full_path_string.push_str(&worktree_relative_path.display(self.path_style));
            }

            full_path_string.into()
        }
    }
}

impl LocalWorktree {
    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn is_path_private(&self, path: &RelPath) -> bool {
        !self.share_private_files && self.settings.is_path_private(path)
    }

    pub fn fs_is_case_sensitive(&self) -> bool {
        self.fs_case_sensitive
    }

    fn restart_background_scanners(&mut self, cx: &Context<Worktree>) {
        let (scan_requests_tx, scan_requests_rx) = async_channel::unbounded();
        let (path_prefixes_to_scan_tx, path_prefixes_to_scan_rx) = async_channel::unbounded();
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
        scan_requests_rx: async_channel::Receiver<ScanRequest>,
        path_prefixes_to_scan_rx: async_channel::Receiver<PathPrefixScanRequest>,
        cx: &Context<Worktree>,
    ) {
        let snapshot = self.snapshot();
        let share_private_files = self.share_private_files;
        let next_entry_id = self.next_entry_id.clone();
        let fs = self.fs.clone();
        let scanning_enabled = self.scanning_enabled;
        let force_defer_watch = self.force_defer_watch;
        let track_git_repositories = self.visible;
        let settings = self.settings.clone();
        let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded();
        let background_scanner = cx.background_spawn({
            let abs_path = snapshot.abs_path.as_path().to_path_buf();
            let background = cx.background_executor().clone();
            async move {
                let defer_watch =
                    force_defer_watch || (scanning_enabled && fs::requires_poll_watcher(&abs_path));

                let (events, watcher) = if scanning_enabled && !defer_watch {
                    fs.watch(&abs_path, FS_WATCH_LATENCY).await
                } else {
                    (Box::pin(stream::pending()) as _, Arc::new(NullWatcher) as _)
                };
                let fs_case_sensitive = fs.is_case_sensitive().await;

                let is_single_file = snapshot.snapshot.root_dir().is_none();
                let mut scanner = BackgroundScanner {
                    fs,
                    fs_case_sensitive,
                    status_updates_tx: scan_states_tx,
                    executor: background,
                    scan_requests_rx,
                    path_prefixes_to_scan_rx,
                    next_entry_id,
                    state: async_lock::Mutex::new(BackgroundScannerState {
                        prev_snapshot: snapshot.snapshot.clone(),
                        snapshot,
                        symlink_paths_by_target: Default::default(),
                        scanned_dirs: Default::default(),
                        watched_dir_abs_paths_by_entry_id: Default::default(),
                        scanning_enabled,
                        path_prefixes_to_scan: Default::default(),
                        paths_to_scan: Default::default(),
                        removed_entries: RemovedEntries::default(),
                        changed_paths: Default::default(),
                    }),
                    phase: BackgroundScannerPhase::InitialScan,
                    share_private_files,
                    settings,
                    watcher,
                    track_git_repositories,
                    is_single_file,
                    defer_watch,
                };

                scanner.run(events).await;
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
                        ScanState::RootDeleted => {
                            log::info!(
                                "worktree root {} no longer exists, closing worktree",
                                this.abs_path().display()
                            );
                            cx.emit(Event::Deleted);
                        }
                    }
                });
            }
        });
        self._background_scanner_tasks = vec![background_scanner, scan_state_updater];
        *self.is_scanning.0.borrow_mut() = true;
    }

    fn set_snapshot(
        &mut self,
        mut new_snapshot: LocalSnapshot,
        entry_changes: UpdatedEntriesSet,
        cx: &mut Context<Worktree>,
    ) {
        // Diff repos before root-metadata derivation so ordering of set_snapshot
        // (and WorktreeMetadata / project grouping consumers) stays unchanged.
        let repo_changes = changed_git_repositories(&self.snapshot, &new_snapshot);

        if let Some((common_dir, is_linked_worktree)) = new_snapshot
            .local_repo_for_work_directory_path(RelPath::empty())
            .map(|repo| {
                (
                    SanitizedPath::from_arc(repo.identity.common_dir_abs_path.clone()),
                    repo.identity.repository_dir_abs_path != repo.identity.common_dir_abs_path,
                )
            })
        {
            new_snapshot.root_repo_common_dir = Some(common_dir);
            new_snapshot.root_repo_is_linked_worktree = is_linked_worktree;
        } else {
            new_snapshot.root_repo_common_dir = None;
            new_snapshot.root_repo_is_linked_worktree = false;
        }

        let root_repo_metadata_changed = self.snapshot.root_repo_common_dir
            != new_snapshot.root_repo_common_dir
            || self.snapshot.root_repo_is_linked_worktree
                != new_snapshot.root_repo_is_linked_worktree;
        let old_root_repo_common_dir =
            root_repo_metadata_changed.then(|| self.snapshot.root_repo_common_dir.clone());
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
        if let Some(old) = old_root_repo_common_dir {
            cx.emit(Event::UpdatedRootRepoCommonDir { old });
        }

        while let Some((scan_id, _)) = self.snapshot_subscriptions.front() {
            if self.snapshot.completed_scan_id >= *scan_id {
                let (_, tx) = self.snapshot_subscriptions.pop_front().unwrap();
                tx.send(()).ok();
            } else {
                break;
            }
        }
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

    pub fn wait_for_snapshot(
        &mut self,
        scan_id: usize,
    ) -> impl Future<Output = Result<()>> + use<> {
        let (tx, rx) = oneshot::channel();
        if self.snapshot.completed_scan_id >= scan_id {
            tx.send(()).ok();
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

    pub fn snapshot(&self) -> LocalSnapshot {
        self.snapshot.clone()
    }

    pub fn settings(&self) -> WorktreeSettings {
        self.settings.clone()
    }

    fn load_binary_file(
        &self,
        path: &RelPath,
        cx: &Context<Worktree>,
    ) -> Task<Result<LoadedBinaryFile>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let entry = self.refresh_entry(path.clone(), None, cx);
        let is_private = self.is_path_private(&path);

        let worktree = cx.weak_entity();
        cx.background_spawn(async move {
            let content = fs.load_bytes(&abs_path).await?;

            let worktree = worktree.upgrade().context("worktree was dropped")?;
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
                            size: metadata.len,
                        },
                        is_local: true,
                        is_private,
                    })
                }
            };

            Ok(LoadedBinaryFile { file, content })
        })
    }

    #[ztracing::instrument(skip_all)]
    fn load_file(&self, path: &RelPath, cx: &Context<Worktree>) -> Task<Result<LoadedFile>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let entry = self.refresh_entry(path.clone(), None, cx);
        let is_private = self.is_path_private(path.as_ref());

        let this = cx.weak_entity();
        cx.background_spawn(async move {
            // WARN: Temporary workaround for #27283.
            //       We are not efficient with our memory usage per file, and use in excess of 64GB for a 10GB file
            //       Therefore, as a temporary workaround to prevent system freezes, we just bail before opening a file
            //       if it is too large
            //       5GB seems to be more reasonable, peaking at ~16GB, while 6GB jumps up to >24GB which seems like a
            //       reasonable limit
            const FILE_SIZE_MAX: u64 = 6 * 1024 * 1024 * 1024; // 6GB
            let metadata = fs.metadata(&abs_path).await?;
            if let Some(metadata) = metadata.as_ref()
                && metadata.len >= FILE_SIZE_MAX
            {
                anyhow::bail!("File is too large to load");
            }
            let (text, encoding, has_bom) = decode_file_text(fs.as_ref(), &abs_path).await?;
            let is_writable = metadata.is_some_and(|metadata| metadata.is_writable);

            let worktree = this.upgrade().context("worktree was dropped")?;
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
                            size: metadata.len,
                        },
                        is_local: true,
                        is_private,
                    })
                }
            };

            Ok(LoadedFile {
                file,
                text,
                encoding,
                has_bom,
                is_writable,
            })
        })
    }

    /// Find the lowest path in the worktree's datastructures that is an ancestor
    fn lowest_ancestor(&self, path: &RelPath) -> Arc<RelPath> {
        let mut lowest_ancestor = None;
        for path in path.ancestors() {
            if self.entry_for_path(path).is_some() {
                lowest_ancestor = Some(path.into());
                break;
            }
        }

        lowest_ancestor.unwrap_or_else(|| RelPath::empty_arc())
    }

    pub fn create_entry(
        &self,
        path: Arc<RelPath>,
        is_dir: bool,
        content: Option<Vec<u8>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<CreatedEntry>> {
        let abs_path = self.absolutize(&path);
        let path_excluded = self.settings.is_path_excluded(&path);
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
                    if refresh_path == RelPath::empty() {
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

    pub fn write_file(
        &self,
        path: Arc<RelPath>,
        text: Rope,
        line_ending: LineEnding,
        encoding: &'static Encoding,
        has_bom: bool,
        cx: &Context<Worktree>,
    ) -> Task<Result<Arc<File>>> {
        let fs = self.fs.clone();
        let is_private = self.is_path_private(&path);
        let abs_path = self.absolutize(&path);

        let write = cx.background_spawn({
            let fs = fs.clone();
            let abs_path = abs_path.clone();
            async move {
                // For UTF-8, use the optimized `fs.save` which writes Rope chunks directly to disk
                // without allocating a contiguous string.
                if encoding == encoding_rs::UTF_8 && !has_bom {
                    return fs.save(&abs_path, &text, line_ending).await;
                }

                // For legacy encodings (e.g. Shift-JIS), we fall back to converting the entire Rope
                // to a String/Bytes in memory before writing.
                //
                // Note: This is inefficient for very large files compared to the streaming approach above,
                // but supporting streaming writes for arbitrary encodings would require a significant
                // refactor of the `fs` crate to expose a Writer interface.
                let text_string = text.to_string();
                let normalized_text = match line_ending {
                    LineEnding::Unix => text_string,
                    LineEnding::Windows => text_string.replace('\n', "\r\n"),
                };

                // Create the byte vector manually for UTF-16 encodings because encoding_rs encodes to UTF-8 by default (per WHATWG standards),
                //  which is not what we want for saving files.
                let bytes = if encoding == encoding_rs::UTF_16BE {
                    let mut data = Vec::with_capacity(normalized_text.len() * 2 + 2);
                    if has_bom {
                        data.extend_from_slice(&[0xFE, 0xFF]); // BOM
                    }
                    let utf16be_bytes =
                        normalized_text.encode_utf16().flat_map(|u| u.to_be_bytes());
                    data.extend(utf16be_bytes);
                    data.into()
                } else if encoding == encoding_rs::UTF_16LE {
                    let mut data = Vec::with_capacity(normalized_text.len() * 2 + 2);
                    if has_bom {
                        data.extend_from_slice(&[0xFF, 0xFE]); // BOM
                    }
                    let utf16le_bytes =
                        normalized_text.encode_utf16().flat_map(|u| u.to_le_bytes());
                    data.extend(utf16le_bytes);
                    data.into()
                } else {
                    // For other encodings (Shift-JIS, UTF-8 with BOM, etc.), delegate to encoding_rs.
                    let bom_bytes = if has_bom {
                        if encoding == encoding_rs::UTF_8 {
                            vec![0xEF, 0xBB, 0xBF]
                        } else {
                            vec![]
                        }
                    } else {
                        vec![]
                    };
                    let (cow, _, _) = encoding.encode(&normalized_text);
                    if !bom_bytes.is_empty() {
                        let mut bytes = bom_bytes;
                        bytes.extend_from_slice(&cow);
                        bytes.into()
                    } else {
                        cow
                    }
                };

                fs.write(&abs_path, &bytes).await
            }
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
            let worktree = this.upgrade().context("worktree dropped")?;
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
                        size: metadata.len,
                    },
                    entry_id: None,
                    is_local: true,
                    is_private,
                }))
            }
        })
    }

    pub fn trash_entry(&self, entry: Entry, cx: &Context<Worktree>) -> Task<Result<TrashId>> {
        let abs_path = self.absolutize(&entry.path);
        let fs = self.fs.clone();

        cx.spawn(async move |this, cx| {
            let trash_id = if entry.is_file() {
                fs.trash(&abs_path, Default::default()).await?
            } else {
                fs.trash(
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: false,
                    },
                )
                .await?
            };

            this.update(cx, |this, _| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entries_for_paths(vec![entry.path])
            })?
            .recv()
            .await;

            Ok(trash_id)
        })
    }

    pub fn delete_entry(&self, entry: Entry, cx: &Context<Worktree>) -> Task<Result<()>> {
        let abs_path = self.absolutize(&entry.path);
        let fs = self.fs.clone();

        cx.spawn(async move |this, cx| {
            if entry.is_file() {
                fs.remove_file(&abs_path, Default::default()).await?
            } else {
                fs.remove_dir(
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: false,
                    },
                )
                .await?
            };

            this.update(cx, |this, _| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entries_for_paths(vec![entry.path])
            })?
            .recv()
            .await;

            Ok(())
        })
    }

    pub fn restore_entry(
        &mut self,
        trash_id: TrashId,
        cx: &mut Context<'_, Worktree>,
    ) -> Task<Result<Entry>> {
        let fs = self.fs.clone();
        let worktree_abs_path = self.abs_path().clone();
        let path_style = self.path_style();

        cx.spawn(async move |this, cx| {
            let path_buf = fs.restore(trash_id).await?;
            let path = path_buf
                .strip_prefix(worktree_abs_path)
                .context("Could not strip prefix")?;
            let path = Arc::from(RelPath::new(&path, path_style)?.as_ref());

            let entry = this
                .update(cx, |this, cx| {
                    this.as_local_mut().unwrap().refresh_entry(path, None, cx)
                })?
                .await?
                .context("Entry not found after restore")?;

            Ok(entry)
        })
    }

    pub fn copy_external_entries(
        &self,
        target_directory: Arc<RelPath>,
        paths: Vec<Arc<Path>>,
        cx: &Context<Worktree>,
    ) -> Task<Result<Vec<ProjectEntryId>>> {
        let target_directory = self.absolutize(&target_directory);
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
            .filter_map(|(_, target)| {
                RelPath::new(
                    target.strip_prefix(&worktree_path).ok()?,
                    PathStyle::local(),
                )
                .ok()
                .map(|path| path.into_arc())
            })
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
                        format!("Failed to copy file from {source:?} to {target:?}")
                    })?;
                }
                anyhow::Ok(())
            })
            .await
            .log_err();
            let mut refresh = cx.read_entity(
                &this.upgrade().with_context(|| "Dropped worktree")?,
                |this, _| {
                    anyhow::Ok::<postage::barrier::Receiver>(
                        this.as_local()
                            .with_context(|| "Worktree is not local")?
                            .refresh_entries_for_paths(paths_to_refresh.clone()),
                    )
                },
            )?;

            cx.background_spawn(async move {
                refresh.next().await;
                anyhow::Ok(())
            })
            .await
            .log_err();

            let this = this.upgrade().with_context(|| "Dropped worktree")?;
            Ok(cx.read_entity(&this, |this, _| {
                paths_to_refresh
                    .iter()
                    .filter_map(|path| Some(this.entry_for_path(path)?.id))
                    .collect()
            }))
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
        let mut rx = self.add_path_prefix_to_scan(path);
        Some(cx.background_spawn(async move {
            rx.next().await;
            Ok(())
        }))
    }

    pub fn refresh_entries_for_paths(&self, paths: Vec<Arc<RelPath>>) -> barrier::Receiver {
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
    pub fn manually_refresh_entries_for_paths(
        &self,
        paths: Vec<Arc<RelPath>>,
    ) -> barrier::Receiver {
        self.refresh_entries_for_paths(paths)
    }

    pub fn add_path_prefix_to_scan(&self, path_prefix: Arc<RelPath>) -> barrier::Receiver {
        let (tx, rx) = barrier::channel();
        self.path_prefixes_to_scan_tx
            .try_send(PathPrefixScanRequest {
                path: path_prefix,
                done: smallvec![tx],
            })
            .ok();
        rx
    }

    pub fn refresh_entry(
        &self,
        path: Arc<RelPath>,
        old_path: Option<Arc<RelPath>>,
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
        // todo(lw): Hot foreground spawn
        cx.spawn(async move |this, cx| {
            refresh.recv().await;
            log::trace!("refreshed entry {path:?} in {:?}", t0.elapsed());
            let new_entry = this.read_with(cx, |this, _| {
                this.entry_for_path(&path).cloned().with_context(|| {
                    format!("Could not find entry in worktree for {path:?} after refresh")
                })
            })??;
            Ok(Some(new_entry))
        })
    }

    pub fn observe_updates<F, Fut>(&mut self, project_id: u64, cx: &Context<Worktree>, callback: F)
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

        let worktree_id = self.id.to_proto();
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

    pub fn update_abs_path_and_refresh(
        &mut self,
        new_path: Arc<SanitizedPath>,
        cx: &Context<Worktree>,
    ) {
        self.snapshot.git_repositories = Default::default();
        self.snapshot.ignores_by_parent_abs_path = Default::default();
        let root_name = new_path
            .as_path()
            .file_name()
            .and_then(|f| f.to_str())
            .map_or(RelPath::empty_arc(), |f| {
                RelPath::from_unix_str(f).unwrap().into()
            });
        self.snapshot.update_abs_path(new_path, root_name);
        self.restart_background_scanners(cx);
    }
    #[cfg(feature = "test-support")]
    pub fn set_defer_watch(&mut self, defer: bool, cx: &mut Context<Worktree>) {
        self.force_defer_watch = defer;
        self.restart_background_scanners(cx);
    }

    #[cfg(feature = "test-support")]
    pub fn repositories(&self) -> Vec<Arc<Path>> {
        self.git_repositories
            .values()
            .map(|entry| entry.identity.work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    }

    /// `(repository_dir, common_dir)` for each registered repository.
    #[cfg(feature = "test-support")]
    pub fn repository_identities(&self) -> Vec<(Arc<Path>, Arc<Path>)> {
        self.git_repositories
            .values()
            .map(|entry| {
                (
                    entry.identity.repository_dir_abs_path.clone(),
                    entry.identity.common_dir_abs_path.clone(),
                )
            })
            .collect()
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

    pub fn insert_entry(
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

    fn trash_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &Context<Worktree>,
    ) -> Task<Result<TrashId>> {
        let response = self.client.request(proto::TrashProjectEntry {
            project_id: self.project_id,
            entry_id: entry_id.to_proto(),
        });

        cx.spawn(async move |this, cx| {
            let response = response.await?;
            let scan_id = response.worktree_scan_id as usize;
            let trash_id = response.trash_id;

            this.update(cx, move |this, _| {
                this.as_remote_mut().unwrap().wait_for_snapshot(scan_id)
            })?
            .await?;

            this.update(cx, |this, _| {
                let this = this.as_remote_mut().unwrap();
                let snapshot = &mut this.background_snapshot.lock().0;
                snapshot.delete_entry(entry_id);
                this.snapshot = snapshot.clone();
            })?;

            Ok(TrashId::from_proto(trash_id))
        })
    }

    fn delete_entry(&self, entry_id: ProjectEntryId, cx: &Context<Worktree>) -> Task<Result<()>> {
        let response = self.client.request(proto::DeleteProjectEntry {
            project_id: self.project_id,
            entry_id: entry_id.to_proto(),
            // The `use_trash` field is being deprecated but it's still required
            // in the message, hence the `#[allow(deprecated)]` attribute.
            #[allow(deprecated)]
            use_trash: false,
        });

        cx.spawn(async move |this, cx| {
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
        })
    }

    fn restore_entry(&mut self, trash_id: TrashId, cx: &Context<Worktree>) -> Task<Result<Entry>> {
        let project_id = self.project_id();
        let worktree_id = self.id().to_proto();
        let trash_id = trash_id.to_proto();

        let request = self.client.request(proto::RestoreProjectEntry {
            project_id,
            worktree_id,
            trash_id,
        });

        cx.spawn(async move |this, cx| {
            let response = request.await?;
            let scan_id = response.worktree_scan_id as usize;
            let proto_entry = response.entry.context("Missing entry in in response")?;

            this.update(cx, move |worktree, cx| {
                worktree
                    .as_remote_mut()
                    .unwrap()
                    .insert_entry(proto_entry, scan_id, cx)
            })?
            .await
        })
    }

    fn copy_external_entries(
        &self,
        target_directory: Arc<RelPath>,
        paths_to_copy: Vec<Arc<Path>>,
        local_fs: Arc<dyn Fs>,
        cx: &Context<Worktree>,
    ) -> Task<anyhow::Result<Vec<ProjectEntryId>>> {
        let client = self.client.clone();
        let worktree_id = self.id().to_proto();
        let project_id = self.project_id;

        cx.background_spawn(async move {
            let mut requests = Vec::new();
            for root_path_to_copy in paths_to_copy {
                let Some(filename) = root_path_to_copy
                    .file_name()
                    .and_then(|name| name.to_str())
                    .and_then(|filename| RelPath::from_unix_str(filename).ok())
                else {
                    continue;
                };
                for (abs_path, is_directory) in
                    read_dir_items(local_fs.as_ref(), &root_path_to_copy).await?
                {
                    let Some(relative_path) = abs_path
                        .strip_prefix(&root_path_to_copy)
                        .map_err(|e| anyhow::Error::from(e))
                        .and_then(|relative_path| RelPath::new(relative_path, PathStyle::local()))
                        .log_err()
                    else {
                        continue;
                    };
                    let content = if is_directory {
                        None
                    } else {
                        Some(local_fs.load_bytes(&abs_path).await?)
                    };

                    let mut target_path = target_directory.join(filename);
                    if relative_path.file_name().is_some() {
                        target_path = target_path.join(&relative_path);
                    }

                    requests.push(proto::CreateProjectEntry {
                        project_id,
                        worktree_id,
                        path: target_path.as_unix_str().to_owned(),
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
    pub fn new(
        id: WorktreeId,
        root_name: Arc<RelPath>,
        abs_path: Arc<Path>,
        path_style: PathStyle,
    ) -> Self {
        Snapshot {
            id,
            abs_path: SanitizedPath::from_arc(abs_path),
            path_style,
            root_char_bag: root_name
                .as_unix_str()
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect(),
            root_name,
            always_included_entries: Default::default(),
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
            root_repo_common_dir: None,
            root_repo_is_linked_worktree: false,
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
        SanitizedPath::cast_arc_ref(&self.abs_path)
    }

    pub fn root_repo_common_dir(&self) -> Option<&Arc<Path>> {
        self.root_repo_common_dir
            .as_ref()
            .map(SanitizedPath::cast_arc_ref)
    }

    pub fn root_repo_is_linked_worktree(&self) -> bool {
        self.root_repo_is_linked_worktree
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
            abs_path: self.abs_path().to_string_lossy().into_owned(),
            root_name: self.root_name().as_unix_str().to_owned(),
            root_repo_common_dir: self
                .root_repo_common_dir()
                .map(|p| p.to_string_lossy().into_owned()),
            root_repo_is_linked_worktree: self.root_repo_is_linked_worktree,
            updated_entries,
            removed_entries: Vec::new(),
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
            // Sent in separate messages.
            updated_repositories: Vec::new(),
            removed_repositories: Vec::new(),
        }
    }

    pub fn work_directory_abs_path(&self, work_directory: &WorkDirectory) -> PathBuf {
        match work_directory {
            WorkDirectory::InProject { relative_path } => self.absolutize(relative_path),
            WorkDirectory::AboveProject { absolute_path } => absolute_path.as_ref().to_owned(),
        }
    }

    pub fn absolutize(&self, path: &RelPath) -> PathBuf {
        if path.file_name().is_some() {
            let mut abs_path = self.abs_path.to_string();
            for component in path.components() {
                if !abs_path.ends_with(self.path_style.primary_separator()) {
                    abs_path.push_str(self.path_style.primary_separator());
                }
                abs_path.push_str(component);
            }
            PathBuf::from(abs_path)
        } else {
            self.abs_path.as_path().to_path_buf()
        }
    }

    pub fn contains_entry(&self, entry_id: ProjectEntryId) -> bool {
        self.entries_by_id.get(&entry_id, ()).is_some()
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
            (),
        );
        if let Some(old_entry) = old_entry {
            self.entries_by_path.remove(&PathKey(old_entry.path), ());
        }
        self.entries_by_path.insert_or_replace(entry.clone(), ());
        Ok(entry)
    }

    fn delete_entry(&mut self, entry_id: ProjectEntryId) -> Option<Arc<RelPath>> {
        let removed_entry = self.entries_by_id.remove(&entry_id, ())?;
        self.entries_by_path = {
            let mut cursor = self.entries_by_path.cursor::<TraversalProgress>(());
            let mut new_entries_by_path =
                cursor.slice(&TraversalTarget::path(&removed_entry.path), Bias::Left);
            while let Some(entry) = cursor.item() {
                if entry.path.starts_with(&removed_entry.path) {
                    self.entries_by_id.remove(&entry.id, ());
                    cursor.next();
                } else {
                    break;
                }
            }
            new_entries_by_path.append(cursor.suffix(), ());
            new_entries_by_path
        };

        Some(removed_entry.path)
    }

    fn update_abs_path(&mut self, abs_path: Arc<SanitizedPath>, root_name: Arc<RelPath>) {
        self.abs_path = abs_path;
        if root_name != self.root_name {
            self.root_char_bag = root_name
                .as_unix_str()
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect();
            self.root_name = root_name;
        }
    }

    pub fn apply_remote_update(
        &mut self,
        update: proto::UpdateWorktree,
        always_included_paths: &PathMatcher,
    ) {
        log::debug!(
            "applying remote worktree update. {} entries updated, {} removed",
            update.updated_entries.len(),
            update.removed_entries.len()
        );
        if let Some(root_name) = RelPath::from_unix_str(&update.root_name).log_err() {
            self.update_abs_path(
                SanitizedPath::new_arc(&Path::new(&update.abs_path)),
                root_name.into(),
            );
        }

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
            let Some(entry) =
                Entry::try_from((&self.root_char_bag, always_included_paths, entry)).log_err()
            else {
                continue;
            };
            if let Some(PathEntry { path, .. }) = self.entries_by_id.get(&entry.id, ()) {
                entries_by_path_edits.push(Edit::Remove(PathKey(path.clone())));
            }
            if let Some(old_entry) = self.entries_by_path.get(&PathKey(entry.path.clone()), ())
                && old_entry.id != entry.id
            {
                entries_by_id_edits.push(Edit::Remove(old_entry.id));
            }
            entries_by_id_edits.push(Edit::Insert(PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: 0,
            }));
            entries_by_path_edits.push(Edit::Insert(entry));
        }

        self.entries_by_path.edit(entries_by_path_edits, ());
        self.entries_by_id.edit(entries_by_id_edits, ());

        // A `None` from a completed scan is a real repo removal, whereas a `None`
        // mid-scan may just mean the sender hasn't registered the root repo yet.
        match update
            .root_repo_common_dir
            .map(|p| SanitizedPath::new_arc(Path::new(&p)))
        {
            Some(dir) => {
                self.root_repo_common_dir = Some(dir);
                self.root_repo_is_linked_worktree = update.root_repo_is_linked_worktree;
            }
            None if update.is_last_update => {
                self.root_repo_common_dir = None;
                self.root_repo_is_linked_worktree = false;
            }
            None => {}
        }

        self.scan_id = update.scan_id as usize;
        if update.is_last_update {
            self.completed_scan_id = update.scan_id as usize;
        }
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
    ) -> Traversal<'_> {
        let mut cursor = self.entries_by_path.cursor(());
        cursor.seek(
            &TraversalTarget::Count {
                count: start_offset,
                include_files,
                include_dirs,
                include_ignored,
            },
            Bias::Right,
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
        path: &RelPath,
    ) -> Traversal<'_> {
        Traversal::new(self, include_files, include_dirs, include_ignored, path)
    }

    pub fn files(&self, include_ignored: bool, start: usize) -> Traversal<'_> {
        self.traverse_from_offset(true, false, include_ignored, start)
    }

    pub fn directories(&self, include_ignored: bool, start: usize) -> Traversal<'_> {
        self.traverse_from_offset(false, true, include_ignored, start)
    }

    pub fn entries(&self, include_ignored: bool, start: usize) -> Traversal<'_> {
        self.traverse_from_offset(true, true, include_ignored, start)
    }

    pub fn paths(&self) -> impl Iterator<Item = &RelPath> {
        self.entries_by_path
            .cursor::<()>(())
            .filter(move |entry| !entry.path.is_empty())
            .map(|entry| entry.path.as_ref())
    }

    pub fn child_entries<'a>(&'a self, parent_path: &'a RelPath) -> ChildEntriesIter<'a> {
        let options = ChildEntriesOptions {
            include_files: true,
            include_dirs: true,
            include_ignored: true,
        };
        self.child_entries_with_options(parent_path, options)
    }

    pub fn child_entries_with_options<'a>(
        &'a self,
        parent_path: &'a RelPath,
        options: ChildEntriesOptions,
    ) -> ChildEntriesIter<'a> {
        let mut cursor = self.entries_by_path.cursor(());
        cursor.seek(&TraversalTarget::path(parent_path), Bias::Right);
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
        self.entries_by_path.first()
    }

    /// Returns `None` for a single file worktree, or `Some(self.abs_path())` if
    /// it is a directory.
    pub fn root_dir(&self) -> Option<Arc<Path>> {
        self.root_entry()
            .filter(|entry| entry.is_dir())
            .map(|_| self.abs_path().clone())
    }

    pub fn root_name(&self) -> &RelPath {
        &self.root_name
    }

    pub fn root_name_str(&self) -> &str {
        self.root_name.as_unix_str()
    }

    pub fn scan_id(&self) -> usize {
        self.scan_id
    }

    pub fn entry_for_path(&self, path: &RelPath) -> Option<&Entry> {
        let entry = self.traverse_from_path(true, true, true, path).entry();
        entry.and_then(|entry| {
            if entry.path.as_ref() == path {
                Some(entry)
            } else {
                None
            }
        })
    }

    /// Whether `path` is gitignored, or lies inside a gitignored directory.
    ///
    /// The contents of ignored directories aren't scanned until explicitly
    /// expanded, so when `path` has no entry this falls back to the ignore
    /// status of its nearest scanned ancestor.
    pub fn is_path_ignored(&self, path: &RelPath) -> bool {
        path.ancestors()
            .find_map(|ancestor| self.entry_for_path(ancestor))
            .is_some_and(|entry| entry.is_ignored)
    }

    /// Resolves a path to an executable using the following heuristics:
    ///
    /// 1. If the path starts with `~`, it is expanded to the user's home directory.
    /// 2. If the path is relative and contains more than one component,
    ///    it is joined to the worktree root path.
    /// 3. If the path is relative and exists in the worktree
    ///    (even if falls under an exclusion filter),
    ///    it is joined to the worktree root path.
    /// 4. Otherwise the path is returned unmodified.
    ///
    /// Relative paths that do not exist in the worktree may
    /// still be found using the `PATH` environment variable.
    pub fn resolve_relative_path(&self, path: PathBuf) -> PathBuf {
        if let Some(path_str) = path.to_str() {
            if let Some(remaining_path) = path_str.strip_prefix("~/") {
                return home_dir().join(remaining_path);
            } else if path_str == "~" {
                return home_dir().to_path_buf();
            }
        }

        if let Ok(rel_path) = RelPath::new(&path, self.path_style)
            && (path.components().count() > 1 || self.entry_for_path(&rel_path).is_some())
        {
            self.abs_path().join(path)
        } else {
            path
        }
    }

    pub fn entry_for_id(&self, id: ProjectEntryId) -> Option<&Entry> {
        let entry = self.entries_by_id.get(&id, ())?;
        self.entry_for_path(&entry.path)
    }

    pub fn path_style(&self) -> PathStyle {
        self.path_style
    }
}

impl LocalSnapshot {
    /// Whether `path` is under this registration's work directory, using the
    /// already-translated logical relative event path (not a raw absolute path)
    /// so external-symlink routing stays correct.
    fn repository_directory_contains(&self, entry: &LocalRepositoryEntry, path: &RelPath) -> bool {
        match entry
            .identity
            .work_directory_abs_path
            .strip_prefix(self.abs_path.as_path())
        {
            // AboveProject: work dir is outside the project root and covers all paths.
            Err(_) => true,
            Ok(relative) => RelPath::new(relative, self.path_style)
                .map(|work_rel| path.starts_with(work_rel.as_ref()))
                .unwrap_or(false),
        }
    }

    /// PathKey formerly stored on `WorkDirectory`: empty for root InProject and
    /// AboveProject; otherwise the relative path of the work dir within the project.
    fn work_directory_path_key(&self, entry: &LocalRepositoryEntry) -> PathKey {
        match entry
            .identity
            .work_directory_abs_path
            .strip_prefix(self.abs_path.as_path())
        {
            Ok(relative) if !relative.as_os_str().is_empty() => {
                match RelPath::new(relative, self.path_style) {
                    Ok(work_rel) => PathKey(work_rel.into_owned().into()),
                    Err(_) => PathKey(RelPath::empty_arc()),
                }
            }
            _ => PathKey(RelPath::empty_arc()),
        }
    }

    fn local_repo_for_work_directory_path(&self, path: &RelPath) -> Option<&LocalRepositoryEntry> {
        self.git_repositories
            .values()
            .find(|entry| self.work_directory_path_key(entry).0.as_ref() == path)
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
            abs_path: self.abs_path().to_string_lossy().into_owned(),
            root_name: self.root_name().as_unix_str().to_owned(),
            root_repo_common_dir: self
                .root_repo_common_dir()
                .map(|p| p.to_string_lossy().into_owned()),
            root_repo_is_linked_worktree: self.root_repo_is_linked_worktree,
            updated_entries,
            removed_entries,
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
            // Sent in separate messages.
            updated_repositories: Vec::new(),
            removed_repositories: Vec::new(),
        }
    }

    async fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        log::trace!("insert entry {:?}", entry.path);
        if entry.is_file() && entry.path.file_name() == Some(&GITIGNORE) {
            let abs_path = self.absolutize(&entry.path);
            match build_gitignore(&abs_path, fs).await {
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

        if entry.kind == EntryKind::PendingDir
            && let Some(existing_entry) = self.entries_by_path.get(&PathKey(entry.path.clone()), ())
        {
            entry.kind = existing_entry.kind;
        }

        let scan_id = self.scan_id;
        let removed = self.entries_by_path.insert_or_replace(entry.clone(), ());
        if let Some(removed) = removed
            && removed.id != entry.id
        {
            self.entries_by_id.remove(&removed.id, ());
        }
        self.entries_by_id.insert_or_replace(
            PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id,
            },
            (),
        );

        entry
    }

    fn ancestor_inodes_for_path(&self, path: &RelPath) -> TreeSet<u64> {
        let mut inodes = TreeSet::default();
        for ancestor in path.ancestors().skip(1) {
            if let Some(entry) = self.entry_for_path(ancestor) {
                inodes.insert(entry.inode);
            }
        }
        inodes
    }

    async fn ignore_stack_for_abs_path(
        &self,
        abs_path: &Path,
        is_dir: bool,
        fs: &dyn Fs,
    ) -> IgnoreStack {
        let mut new_ignores = Vec::new();
        let mut repo_root = None;
        // Deepest registered work dir covering this path — already validated at
        // registration, so `reload_entries_for_paths` can skip a full resolve on the
        // hot path (thousands of event paths per checkout). Invalid `.git` never
        // enters `git_repositories`, so a phantom cannot win as a registered root.
        // Prefer the covering entry's owned `info_exclude` over a path-keyed lookup.
        let covering_registered = self
            .git_repositories
            .values()
            .filter(|repo| abs_path.starts_with(repo.identity.work_directory_abs_path.as_ref()))
            .max_by_key(|repo| repo.identity.work_directory_abs_path.as_os_str().len());
        let covering_work_dir =
            covering_registered.map(|repo| repo.identity.work_directory_abs_path.clone());
        let covering_exclude = covering_registered.map(|repo| repo.info_exclude.clone());
        for (index, ancestor) in abs_path.ancestors().enumerate() {
            if index > 0 {
                if let Some((ignore, _)) = self.ignores_by_parent_abs_path.get(ancestor) {
                    new_ignores.push((ancestor, Some(ignore.clone())));
                } else {
                    new_ignores.push((ancestor, None));
                }
            }

            // Anchor the ignore root at a *validated* repository, so an invalid nested
            // `.git` cannot shadow the real parent's excludes/ignore rooting.
            if repo_root.is_some() {
                continue;
            }

            // Covering registered root: free, no filesystem.
            if covering_work_dir
                .as_ref()
                .is_some_and(|root| root.as_ref() == ancestor)
            {
                repo_root = covering_work_dir.clone();
                continue;
            }

            // Deeper than the covering registration, or nothing registered yet: resolve.
            // `reload_entries_for_paths` runs before `update_git_repositories`, so a
            // newly-appeared nested repo may not be registered yet and must still win.
            // Unregistered deeper repos intentionally miss the registered exclude —
            // preserved by construction (no parallel exclude map).
            if discover_valid_git_repository(&ancestor.join(DOT_GIT), fs)
                .await
                .is_some()
            {
                repo_root = Some(Arc::from(ancestor));
            }
        }

        let mut ignore_stack = if let Some(global_gitignore) = self.global_gitignore.clone() {
            IgnoreStack::global(global_gitignore)
        } else {
            IgnoreStack::none()
        };

        // Only the covering *registered* root contributes its exclude. An unregistered
        // deeper discovery sets `repo_root` without an exclude entry — same miss as before.
        if covering_work_dir.as_ref() == repo_root.as_ref()
            && let Some(repo_exclude) = covering_exclude
        {
            ignore_stack = ignore_stack.append(IgnoreKind::RepoExclude, repo_exclude);
        }
        ignore_stack.repo_root = repo_root;
        let mut ancestor_ignore_stack = ignore_stack.clone();
        for (parent_abs_path, ignore) in new_ignores.into_iter().rev() {
            if ancestor_ignore_stack.is_abs_path_ignored(parent_abs_path, true) {
                ignore_stack = IgnoreStack::all();
                break;
            } else if let Some(ignore) = ignore {
                let kind = IgnoreKind::Gitignore(parent_abs_path.into());
                ancestor_ignore_stack = ancestor_ignore_stack.append(kind, ignore.clone());
                ignore_stack =
                    ignore_stack.append(IgnoreKind::Gitignore(parent_abs_path.into()), ignore);
            }
        }

        if ignore_stack.is_abs_path_ignored(abs_path, is_dir) {
            ignore_stack = IgnoreStack::all();
        }

        ignore_stack
    }

    #[cfg(feature = "test-support")]
    pub fn expanded_entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries_by_path
            .cursor::<()>(())
            .filter(|entry| entry.kind == EntryKind::Dir && (entry.is_external || entry.is_ignored))
    }

    #[cfg(feature = "test-support")]
    pub fn check_invariants(&self, git_state: bool) {
        use pretty_assertions::assert_eq;

        assert_eq!(
            self.entries_by_path
                .cursor::<()>(())
                .map(|e| (&e.path, e.id))
                .collect::<Vec<_>>(),
            self.entries_by_id
                .cursor::<()>(())
                .map(|e| (&e.path, e.id))
                .collect::<collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            "entries_by_path and entries_by_id are inconsistent"
        );

        let mut files = self.files(true, 0);
        let mut visible_files = self.files(false, 0);
        for entry in self.entries_by_path.cursor::<()>(()) {
            if entry.is_file() {
                assert_eq!(files.next().unwrap().inode, entry.inode);
                if !entry.is_ignored || entry.is_always_included {
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
            .cursor::<()>(())
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
                let ignore_parent_path = &RelPath::new(
                    ignore_parent_abs_path
                        .strip_prefix(self.abs_path.as_path())
                        .unwrap(),
                    PathStyle::local(),
                )
                .unwrap();
                assert!(self.entry_for_path(ignore_parent_path).is_some());
                assert!(
                    self.entry_for_path(
                        &ignore_parent_path.join(RelPath::from_unix_str(GITIGNORE).unwrap())
                    )
                    .is_some()
                );
            }
        }
    }

    #[cfg(feature = "test-support")]
    pub fn entries_without_ids(&self, include_ignored: bool) -> Vec<(&RelPath, u64, bool)> {
        let mut paths = Vec::new();
        for entry in self.entries_by_path.cursor::<()>(()) {
            if include_ignored || !entry.is_ignored {
                paths.push((entry.path.as_ref(), entry.inode, entry.is_ignored));
            }
        }
        paths.sort_by(|a, b| a.0.cmp(b.0));
        paths
    }
}

impl BackgroundScannerState {
    async fn enqueue_scan_dir(
        &self,
        abs_path: Arc<Path>,
        entry: &Entry,
        scan_job_tx: &Sender<ScanJob>,
        fs: &dyn Fs,
    ) {
        let path = entry.path.clone();
        let ignore_stack = self
            .snapshot
            .ignore_stack_for_abs_path(&abs_path, true, fs)
            .await;
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
        let Some(mtime) = entry.mtime else {
            return;
        };
        if let Some(entry_id) = self.reused_entry_id(&entry.path, entry.inode, mtime) {
            entry.id = entry_id;
        }
    }

    fn entry_id_for(
        &mut self,
        next_entry_id: &AtomicUsize,
        path: &RelPath,
        metadata: &fs::Metadata,
    ) -> ProjectEntryId {
        self.reused_entry_id(path, metadata.inode, metadata.mtime)
            .unwrap_or_else(|| ProjectEntryId::new(next_entry_id))
    }

    fn reused_entry_id(
        &mut self,
        path: &RelPath,
        inode: u64,
        mtime: MTime,
    ) -> Option<ProjectEntryId> {
        if let Some(removed_entry) = self.removed_entries.take_by_path(path, inode) {
            return Some(removed_entry.id);
        }

        // If an entry with the same inode was removed from the worktree during this scan,
        // then it *might* represent the same file or directory. But the OS might also have
        // re-used the inode for a completely different file or directory.
        //
        // Conditionally reuse the old entry's id:
        // * if the mtime is the same, the file was probably been renamed.
        // * if the path is the same, the file may just have been updated
        if let Some(removed_entry) = self.removed_entries.take_by_inode(inode) {
            (removed_entry.mtime == Some(mtime) || *removed_entry.path == *path)
                .then_some(removed_entry.id)
        } else {
            Some(self.snapshot.entry_for_path(path)?.id)
        }
    }

    async fn insert_entry(&mut self, entry: Entry, fs: &dyn Fs, watcher: &dyn Watcher) -> Entry {
        let entry = self.snapshot.insert_entry(entry, fs).await;
        if entry.path.file_name() == Some(&DOT_GIT) {
            self.insert_git_repository(entry.path.clone(), fs, watcher)
                .await;
        }

        #[cfg(feature = "test-support")]
        self.snapshot.check_invariants(false);

        entry
    }

    fn populate_dir(
        &mut self,
        parent_path: Arc<RelPath>,
        entries: impl IntoIterator<Item = Entry>,
        ignore: Option<Arc<Gitignore>>,
    ) {
        let mut parent_entry = if let Some(parent_entry) = self
            .snapshot
            .entries_by_path
            .get(&PathKey(parent_path.clone()), ())
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
            let abs_parent_path = self
                .snapshot
                .abs_path
                .as_path()
                .join(parent_path.as_std_path())
                .into();
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
            .edit(entries_by_path_edits, ());
        self.snapshot.entries_by_id.edit(entries_by_id_edits, ());

        if let Err(ix) = self.changed_paths.binary_search(&parent_path) {
            self.changed_paths.insert(ix, parent_path.clone());
        }

        #[cfg(feature = "test-support")]
        self.snapshot.check_invariants(false);
    }

    fn remove_path_from_snapshot_and_unwatch(
        &mut self,
        path: &RelPath,
        watcher: &dyn Watcher,
        preserve_repository_watches: bool,
    ) {
        // When the caller preserves repository watches, it intends to re-scan
        // this subtree and keep its git repositories; pruning them here would
        // transiently drop and then re-create them with fresh `RepositoryId`s.
        let prune_repositories = !preserve_repository_watches;
        let removed_descendant_abs_paths = self.remove_path_from_snapshot(path, prune_repositories);
        self.unwatch_path(
            watcher,
            path,
            removed_descendant_abs_paths,
            preserve_repository_watches,
        );
    }

    fn unwatch_path(
        &mut self,
        watcher: &dyn Watcher,
        path: &RelPath,
        removed_descendant_abs_paths: Vec<PathBuf>,
        preserve_repository_watches: bool,
    ) {
        let mut repository_watches_to_preserve = HashSet::<Arc<Path>>::default();
        if preserve_repository_watches {
            for repository in self.snapshot.git_repositories.values() {
                repository_watches_to_preserve
                    .insert(repository.identity.common_dir_abs_path.clone());
                repository_watches_to_preserve
                    .insert(repository.identity.repository_dir_abs_path.clone());
            }
        }

        for removed_dir_abs_path in removed_descendant_abs_paths {
            if repository_watches_to_preserve.contains(removed_dir_abs_path.as_path()) {
                continue;
            }
            watcher.remove(&removed_dir_abs_path).log_err();
        }

        self.snapshot
            .external_canonical_to_relative
            .retain(|canonical, relative| {
                if relative.starts_with(path) {
                    if !repository_watches_to_preserve.contains(canonical.as_ref()) {
                        watcher.remove(canonical.as_ref()).log_err();
                    }
                    false
                } else {
                    true
                }
            });
    }

    fn remove_path_from_snapshot(
        &mut self,
        path: &RelPath,
        prune_repositories: bool,
    ) -> Vec<PathBuf> {
        log::trace!("background scanner removing path {path:?}");
        let mut new_entries;
        let removed_entries;
        {
            let mut cursor = self
                .snapshot
                .entries_by_path
                .cursor::<TraversalProgress>(());
            new_entries = cursor.slice(&TraversalTarget::path(path), Bias::Left);
            removed_entries = cursor.slice(&TraversalTarget::successor(path), Bias::Left);
            new_entries.append(cursor.suffix(), ());
        }
        self.snapshot.entries_by_path = new_entries;

        let mut removed_ids = Vec::with_capacity(removed_entries.summary().count);
        let mut removed_dir_abs_paths = Vec::new();
        for entry in removed_entries.cursor::<()>(()) {
            if entry.is_dir() {
                let watch_path = self
                    .watched_dir_abs_paths_by_entry_id
                    .remove(&entry.id)
                    .map(|path| path.as_ref().to_path_buf())
                    .unwrap_or_else(|| self.snapshot.absolutize(&entry.path));
                removed_dir_abs_paths.push(watch_path);
            }

            self.removed_entries.insert(entry);

            if entry.path.file_name() == Some(GITIGNORE) {
                let abs_parent_path = self.snapshot.absolutize(&entry.path.parent().unwrap());
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

        self.snapshot
            .entries_by_id
            .edit(removed_ids.iter().map(|&id| Edit::Remove(id)).collect(), ());

        // Only prune git repositories when the entries are being genuinely
        // removed. During a recursive refresh (e.g. a watcher-forced rescan),
        // the subtree is removed and immediately re-scanned; dropping the
        // repositories here would make them flap, causing the GitStore to
        // tear them down and re-create them with fresh `RepositoryId`s. Stale
        // repositories are instead reaped authoritatively (against the actual
        // filesystem) in `update_git_repositories`.
        if prune_repositories {
            self.snapshot
                .git_repositories
                .retain(|id, _| removed_ids.binary_search(id).is_err());
        }

        #[cfg(feature = "test-support")]
        self.snapshot.check_invariants(false);

        removed_dir_abs_paths
    }

    async fn insert_git_repository(
        &mut self,
        dot_git_path: Arc<RelPath>,
        fs: &dyn Fs,
        watcher: &dyn Watcher,
    ) -> Option<RegistrationOutcome> {
        let work_dir_path: Arc<RelPath> = match dot_git_path.parent() {
            Some(parent_dir) => {
                // Guard against repositories inside the repository metadata
                if parent_dir
                    .components()
                    .any(|component| component == DOT_GIT)
                {
                    log::debug!(
                        "not building git repository for nested `.git` directory, `.git` path in the worktree: {dot_git_path:?}"
                    );
                    return None;
                };

                parent_dir.into()
            }
            None => {
                // `dot_git_path.parent().is_none()` means `.git` directory is the opened worktree itself,
                // no files inside that directory are tracked by git, so no need to build the repo around it
                log::debug!(
                    "not building git repository for the worktree itself, `.git` path in the worktree: {dot_git_path:?}"
                );
                return None;
            }
        };

        let dot_git_abs_path = Arc::from(self.snapshot.absolutize(&dot_git_path).as_ref());

        self.insert_git_repository_for_path(
            WorkDirectory::InProject {
                relative_path: work_dir_path,
            },
            dot_git_abs_path,
            fs,
            watcher,
        )
        .await
    }

    /// Registers a validated git repository for `work_directory`.
    ///
    /// Returns `None` when the work directory is not indexed or the `.git` entry is not a
    /// repository; otherwise the registration outcome (`was_added` is false on re-insert
    /// of the same key).
    async fn insert_git_repository_for_path(
        &mut self,
        work_directory: WorkDirectory,
        dot_git_abs_path: Arc<Path>,
        fs: &dyn Fs,
        watcher: &dyn Watcher,
    ) -> Option<RegistrationOutcome> {
        let Some(work_dir_entry) = self.snapshot.entry_for_path(&work_directory.path_key().0)
        else {
            log::error!(
                "working directory `{}` not indexed",
                work_directory
                    .path_key()
                    .0
                    .display(self.snapshot.path_style)
            );
            return None;
        };
        let work_directory_abs_path = self.snapshot.work_directory_abs_path(&work_directory);

        // Bail before setting up any watches if the `.git` entry isn't a real repository.
        let Some((repository_dir_abs_path, common_dir_abs_path)) =
            discover_valid_git_repository(&dot_git_abs_path, fs).await
        else {
            return None;
        };

        watcher
            .add(&common_dir_abs_path)
            .context("failed to add common directory to watcher")
            .log_err();
        watcher
            .add(&repository_dir_abs_path)
            .context("failed to add repository directory to watcher")
            .log_err();

        watch_git_dir_subdirectories(&common_dir_abs_path, fs, watcher).await;
        if repository_dir_abs_path != common_dir_abs_path {
            watch_git_dir_subdirectories(&repository_dir_abs_path, fs, watcher).await;
        }

        let work_directory_id = work_dir_entry.id;
        // Clone last-good before rebuild; KeepLastGood* leave the seed in place.
        let previous_info_exclude = self
            .snapshot
            .git_repositories
            .get(&work_directory_id)
            .map(|previous| previous.info_exclude.clone());
        let was_added = previous_info_exclude.is_none();

        let work_directory_abs_path: Arc<Path> = work_directory_abs_path.as_path().into();
        // Registration owns exclude state: load a present info/exclude now so
        // ignore updates that run after insert (affected_repo_roots) see it.
        // Present/Absent/Unparsable start clean; Indeterminate starts dirty so
        // the reload path retries.
        let exclude_abs_path = common_dir_abs_path.join(REPO_EXCLUDE);
        let apply = load_gitignore_existing(&exclude_abs_path, &work_directory_abs_path, fs).await;
        let entry = local_repository_entry_for_registration(
            GitRepositoryIdentity {
                work_directory_abs_path,
                dot_git_abs_path,
                common_dir_abs_path,
                repository_dir_abs_path,
            },
            previous_info_exclude,
            apply,
        );

        self.snapshot
            .git_repositories
            .insert(work_directory_id, entry);

        log::trace!("inserting new local git repository");
        Some(RegistrationOutcome {
            work_directory_id,
            was_added,
        })
    }
}

/// Watches the directories inside a git directory that git writes ref updates to.
///
/// On Linux and FreeBSD the native file watcher is non-recursive, so a watch on the git
/// directory itself does not report changes to files nested below it, such as the loose
/// refs that git updates on commit, fetch, and branch operations. Watch the `refs` tree
/// (its directories are watched individually because branch names may contain slashes)
/// and, for repositories using the reftable backend, the `reftable` directory. On
/// platforms with recursive watchers these calls are deduplicated against the existing
/// recursive registration, making them effectively free.
async fn watch_git_dir_subdirectories(git_dir_abs_path: &Path, fs: &dyn Fs, watcher: &dyn Watcher) {
    let reftable_dir_abs_path = git_dir_abs_path.join(REFTABLE_DIR);
    if fs.is_dir(&reftable_dir_abs_path).await {
        watcher
            .add(&reftable_dir_abs_path)
            .context("failed to add reftable directory to watcher")
            .log_err();
    }

    watch_dir_tree(git_dir_abs_path.join(REFS_DIR), fs, watcher).await;
}

/// Watches a directory and all of its descendant directories.
///
/// Each directory is watched before its children are enumerated, so that a child
/// created concurrently is either seen by the enumeration or reported by the watch.
async fn watch_dir_tree(root_abs_path: PathBuf, fs: &dyn Fs, watcher: &dyn Watcher) {
    let mut dirs_to_watch = vec![root_abs_path];
    while let Some(dir_abs_path) = dirs_to_watch.pop() {
        if !fs.is_dir(&dir_abs_path).await {
            continue;
        }
        watcher
            .add(&dir_abs_path)
            .with_context(|| format!("failed to watch directory {dir_abs_path:?}"))
            .log_err();
        let Some(mut children) = fs.read_dir(&dir_abs_path).await.log_err() else {
            continue;
        };
        while let Some(child_abs_path) = children.next().await {
            let Some(child_abs_path) = child_abs_path.log_err() else {
                continue;
            };
            if fs.is_dir(&child_abs_path).await {
                dirs_to_watch.push(child_abs_path);
            }
        }
    }
}

/// Diffs local repository registrations between two snapshots.
///
/// Emits: new-only → AddedOrUpdated{identity_changed:false}; same key + scan-token
/// change → AddedOrUpdated{false}; same key + identity change → AddedOrUpdated{true};
/// old-only (both merge branches) → identical Removed carrying the complete old
/// registration.
fn changed_git_repositories(
    old_snapshot: &LocalSnapshot,
    new_snapshot: &LocalSnapshot,
) -> GitRepositoryChanges {
    fn registration(id: ProjectEntryId, entry: &LocalRepositoryEntry) -> GitRepositoryRegistration {
        GitRepositoryRegistration {
            work_directory_id: id,
            identity: entry.identity.clone(),
        }
    }

    let mut changes = Vec::new();
    let mut old_repos = old_snapshot.git_repositories.iter().peekable();
    let mut new_repos = new_snapshot.git_repositories.iter().peekable();

    loop {
        match (new_repos.peek().copied(), old_repos.peek().copied()) {
            (Some((&new_entry_id, new_repo)), Some((&old_entry_id, old_repo))) => {
                match Ord::cmp(&new_entry_id, &old_entry_id) {
                    Ordering::Less => {
                        changes.push(GitRepositoryChange::AddedOrUpdated {
                            repository: registration(new_entry_id, new_repo),
                            identity_changed: false,
                        });
                        new_repos.next();
                    }
                    Ordering::Equal => {
                        let identity_changed = new_repo.identity != old_repo.identity;
                        let scan_changed = new_repo.git_dir_scan_id != old_repo.git_dir_scan_id;
                        if identity_changed || scan_changed {
                            changes.push(GitRepositoryChange::AddedOrUpdated {
                                repository: registration(new_entry_id, new_repo),
                                identity_changed,
                            });
                        }
                        new_repos.next();
                        old_repos.next();
                    }
                    Ordering::Greater => {
                        changes.push(GitRepositoryChange::Removed {
                            repository: registration(old_entry_id, old_repo),
                        });
                        old_repos.next();
                    }
                }
            }
            (Some((&entry_id, repo)), None) => {
                changes.push(GitRepositoryChange::AddedOrUpdated {
                    repository: registration(entry_id, repo),
                    identity_changed: false,
                });
                new_repos.next();
            }
            (None, Some((&entry_id, repo))) => {
                changes.push(GitRepositoryChange::Removed {
                    repository: registration(entry_id, repo),
                });
                old_repos.next();
            }
            (None, None) => break,
        }
    }

    changes.into()
}

/// Classifies an fs event against git metadata directories, returning the matched
/// git-dir root and the event's path within it, or `None` when it is not git metadata.
/// Registered repositories are matched by their resolved `.git`/repository/common dirs,
/// so linked worktrees and bare repositories are recognized by identity rather than by
/// a name or the old `HEAD`+`config` heuristic (which missed config-less bare repos).
fn match_git_metadata_event(
    snapshot: &LocalSnapshot,
    abs_path: &Path,
) -> Option<(PathBuf, PathBuf)> {
    let root = snapshot
        .git_repositories
        .values()
        .flat_map(|repo| {
            [
                repo.identity.dot_git_abs_path.as_ref(),
                repo.identity.repository_dir_abs_path.as_ref(),
                repo.identity.common_dir_abs_path.as_ref(),
            ]
        })
        .filter(|root| abs_path.starts_with(root))
        // The deepest matching root wins, so an event in a nested repository is
        // attributed to it rather than an ancestor (a deeper root is necessarily longer).
        .max_by_key(|root| root.as_os_str().len())
        // Only a literally-named `.git` ancestor may seed a new (unregistered)
        // repository, so a nested bare `cache.git` never claims a spurious worktree.
        .or_else(|| {
            abs_path
                .ancestors()
                .find(|ancestor| ancestor.file_name() == Some(OsStr::new(DOT_GIT)))
        })?;
    Some((
        root.to_owned(),
        abs_path.strip_prefix(root).ok()?.to_owned(),
    ))
}

async fn build_gitignore(abs_path: &Path, fs: &dyn Fs) -> Result<Gitignore> {
    let root = abs_path.parent().unwrap_or_else(|| Path::new("/"));
    let contents = fs
        .load(abs_path)
        .await
        .with_context(|| format!("failed to load gitignore file at {}", abs_path.display()))?;
    parse_gitignore_contents(abs_path, root, &contents)
}

/// Outcome of loading an exclude/gitignore file that may be absent or bad.
///
/// Action-ready outcome of loading a repository's `info/exclude`.
///
/// Four states map 1:1 onto apply transitions. Error logging, empty-matcher
/// creation, and Arc construction happen in the loader (before the snapshot lock).
/// Unit tests pin Unparsable→clear dirty (keep last-good) vs Indeterminate→stay
/// pending without a filesystem.
#[derive(Debug, Clone)]
enum RepositoryExcludeApply {
    /// Install matcher and clear dirty.
    Present(Arc<Gitignore>),
    /// Install empty matcher and clear dirty.
    Absent(Arc<Gitignore>),
    /// Keep last-good matcher, clear dirty (no re-read loop). Error already logged.
    Unparsable,
    /// Keep last-good matcher, leave dirty for a later retry. Error already logged.
    Indeterminate,
}

/// Applies an exclude-load decision to a repository entry.
///
/// Returns whether ignore statuses under this work directory should recompute
/// (only when the installed matcher actually changed — Present/Absent).
fn apply_repository_exclude_to_entry(
    entry: &mut LocalRepositoryEntry,
    apply: RepositoryExcludeApply,
) -> bool {
    match apply {
        RepositoryExcludeApply::Present(exclude) | RepositoryExcludeApply::Absent(exclude) => {
            entry.info_exclude = exclude;
            entry.info_exclude_needs_update = false;
            true
        }
        RepositoryExcludeApply::Unparsable => {
            entry.info_exclude_needs_update = false;
            false
        }
        RepositoryExcludeApply::Indeterminate => false,
    }
}

/// Builds a registration entry after loading `info/exclude`.
///
/// First insert seeds an empty matcher; re-insert carries the previous matcher
/// so Unparsable/Indeterminate preserve real last-good rather than an empty rebuild seed.
fn local_repository_entry_for_registration(
    identity: GitRepositoryIdentity,
    previous_info_exclude: Option<Arc<Gitignore>>,
    apply: RepositoryExcludeApply,
) -> LocalRepositoryEntry {
    let mut entry = LocalRepositoryEntry {
        identity,
        git_dir_scan_id: 0,
        info_exclude: previous_info_exclude.unwrap_or_else(|| Arc::new(Gitignore::empty())),
        info_exclude_needs_update: true,
    };
    apply_repository_exclude_to_entry(&mut entry, apply);
    entry
}

/// Loads `info/exclude` into an action-ready four-state outcome.
///
/// Arc construction and error logging complete before the caller takes the
/// snapshot lock. Unparsable content fails identically forever (clear dirty);
/// transient I/O is Indeterminate (leave dirty and retry).
async fn load_gitignore_existing(
    abs_path: &Path,
    root: &Path,
    fs: &dyn Fs,
) -> RepositoryExcludeApply {
    let bytes = match fs.load_bytes(abs_path).await {
        Ok(bytes) => bytes,
        Err(error) if io_error_is_absence(&error) => {
            return RepositoryExcludeApply::Absent(Arc::new(Gitignore::empty()));
        }
        Err(error) => {
            Err::<(), _>(error.context(format!(
                "failed to load gitignore file at {}",
                abs_path.display()
            )))
            .log_err();
            return RepositoryExcludeApply::Indeterminate;
        }
    };
    let contents = match String::from_utf8(bytes) {
        Ok(contents) => contents,
        Err(error) => {
            Err::<(), _>(anyhow::Error::new(error).context(format!(
                "gitignore file at {} is not valid UTF-8",
                abs_path.display()
            )))
            .log_err();
            return RepositoryExcludeApply::Unparsable;
        }
    };
    match parse_gitignore_contents(abs_path, root, &contents) {
        Ok(gitignore) => RepositoryExcludeApply::Present(Arc::new(gitignore)),
        Err(error) => {
            Err::<(), _>(error).log_err();
            RepositoryExcludeApply::Unparsable
        }
    }
}

fn parse_gitignore_contents(abs_path: &Path, root: &Path, contents: &str) -> Result<Gitignore> {
    let mut builder = GitignoreBuilder::new(root);
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

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub worktree: Entity<Worktree>,
    pub path: Arc<RelPath>,
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

    fn path(&self) -> &Arc<RelPath> {
        &self.path
    }

    fn full_path(&self, cx: &App) -> PathBuf {
        self.worktree.read(cx).full_path(&self.path)
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self, cx: &'a App) -> &'a str {
        self.path
            .file_name()
            .unwrap_or_else(|| self.worktree.read(cx).root_name_str())
    }

    fn worktree_id(&self, cx: &App) -> WorktreeId {
        self.worktree.read(cx).id()
    }

    fn to_proto(&self, cx: &App) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: self.worktree.read(cx).id().to_proto(),
            entry_id: self.entry_id.map(|id| id.to_proto()),
            path: self.path.as_ref().as_unix_str().to_owned(),
            mtime: self.disk_state.mtime().map(|time| time.into()),
            is_deleted: self.disk_state.is_deleted(),
            is_historic: matches!(self.disk_state, DiskState::Historic { .. }),
        }
    }

    fn is_private(&self) -> bool {
        self.is_private
    }

    fn path_style(&self, cx: &App) -> PathStyle {
        self.worktree.read(cx).path_style()
    }

    fn can_open(&self) -> bool {
        true
    }
}

impl language::LocalFile for File {
    fn abs_path(&self, cx: &App) -> PathBuf {
        self.worktree.read(cx).absolutize(&self.path)
    }

    fn load(&self, cx: &App) -> Task<Result<String>> {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        cx.background_spawn(async move { fs.load(&abs_path).await })
    }

    fn load_bytes(&self, cx: &App) -> Task<Result<Vec<u8>>> {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        cx.background_spawn(async move { fs.load_bytes(&abs_path).await })
    }
}

impl File {
    pub fn for_entry(entry: Entry, worktree: Entity<Worktree>) -> Arc<Self> {
        Arc::new(Self {
            worktree,
            path: entry.path.clone(),
            disk_state: if let Some(mtime) = entry.mtime {
                DiskState::Present {
                    mtime,
                    size: entry.size,
                }
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
        let worktree_id = worktree.read(cx).as_remote().context("not remote")?.id();

        anyhow::ensure!(
            worktree_id.to_proto() == proto.worktree_id,
            "worktree id does not match file"
        );

        let disk_state = if proto.is_historic {
            DiskState::Historic {
                was_deleted: proto.is_deleted,
            }
        } else if proto.is_deleted {
            DiskState::Deleted
        } else if let Some(mtime) = proto.mtime.map(&Into::into) {
            DiskState::Present { mtime, size: 0 }
        } else {
            DiskState::New
        };

        Ok(Self {
            worktree,
            path: RelPath::from_unix_str(&proto.path)
                .context("invalid path in file protobuf")?
                .into(),
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

    pub fn project_entry_id(&self) -> Option<ProjectEntryId> {
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
    pub path: Arc<RelPath>,
    pub inode: u64,
    pub mtime: Option<MTime>,

    pub canonical_path: Option<Arc<Path>>,
    /// Whether this entry is ignored by Git.
    ///
    /// We only scan ignored entries once the directory is expanded and
    /// exclude them from searches.
    pub is_ignored: bool,

    /// Whether this entry is hidden or inside hidden directory.
    ///
    /// We only scan hidden entries once the directory is expanded.
    pub is_hidden: bool,

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
    /// directory is expanded.
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

/// A worktree-local registration of a git repository (entry id + absolute identity).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitRepositoryRegistration {
    pub work_directory_id: ProjectEntryId,
    pub identity: GitRepositoryIdentity,
}

/// Typed snapshot-diff for local repository bookkeeping.
///
/// `Removed` always carries the complete *old* registration. Both merge-removal
/// branches of the sorted key walk emit identical `Removed` payloads.
///
/// `identity_changed` is load-bearing at the consumer (reinit = job cancellation +
/// failed-open retry + trust re-install for coalesced siblings) and must not be
/// derived from canonical comparison alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GitRepositoryChange {
    AddedOrUpdated {
        repository: GitRepositoryRegistration,
        identity_changed: bool,
    },
    Removed {
        repository: GitRepositoryRegistration,
    },
}

pub type UpdatedEntriesSet = Arc<[(Arc<RelPath>, ProjectEntryId, PathChange)]>;
pub type GitRepositoryChanges = Arc<[GitRepositoryChange]>;

#[derive(Clone, Debug)]
pub struct PathProgress<'a> {
    pub max_path: &'a RelPath,
}

#[derive(Clone, Debug)]
pub struct PathSummary<S> {
    pub max_path: Arc<RelPath>,
    pub item_summary: S,
}

impl<S: Summary> Summary for PathSummary<S> {
    type Context<'a> = S::Context<'a>;

    fn zero(cx: Self::Context<'_>) -> Self {
        Self {
            max_path: RelPath::empty_arc(),
            item_summary: S::zero(cx),
        }
    }

    fn add_summary(&mut self, rhs: &Self, cx: Self::Context<'_>) {
        self.max_path = rhs.max_path.clone();
        self.item_summary.add_summary(&rhs.item_summary, cx);
    }
}

impl<'a, S: Summary> sum_tree::Dimension<'a, PathSummary<S>> for PathProgress<'a> {
    fn zero(_: <PathSummary<S> as Summary>::Context<'_>) -> Self {
        Self {
            max_path: RelPath::empty(),
        }
    }

    fn add_summary(
        &mut self,
        summary: &'a PathSummary<S>,
        _: <PathSummary<S> as Summary>::Context<'_>,
    ) {
        self.max_path = summary.max_path.as_ref()
    }
}

impl<'a> sum_tree::Dimension<'a, PathSummary<GitSummary>> for GitSummary {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathSummary<GitSummary>, _: ()) {
        *self += summary.item_summary
    }
}

impl<'a>
    sum_tree::SeekTarget<'a, PathSummary<GitSummary>, Dimensions<TraversalProgress<'a>, GitSummary>>
    for PathTarget<'_>
{
    fn cmp(
        &self,
        cursor_location: &Dimensions<TraversalProgress<'a>, GitSummary>,
        _: (),
    ) -> Ordering {
        self.cmp_path(cursor_location.0.max_path)
    }
}

impl<'a, S: Summary> sum_tree::Dimension<'a, PathSummary<S>> for PathKey {
    fn zero(_: S::Context<'_>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathSummary<S>, _: S::Context<'_>) {
        self.0 = summary.max_path.clone();
    }
}

impl<'a, S: Summary> sum_tree::Dimension<'a, PathSummary<S>> for TraversalProgress<'a> {
    fn zero(_cx: S::Context<'_>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathSummary<S>, _: S::Context<'_>) {
        self.max_path = summary.max_path.as_ref();
    }
}

impl Entry {
    fn new(
        path: Arc<RelPath>,
        metadata: &fs::Metadata,
        id: ProjectEntryId,
        root_char_bag: CharBag,
        canonical_path: Option<Arc<Path>>,
    ) -> Self {
        let char_bag = char_bag_for_path(root_char_bag, &path);
        Self {
            id,
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
            is_hidden: false,
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

    fn summary(&self, _cx: ()) -> Self::Summary {
        let non_ignored_count = if self.is_ignored && !self.is_always_included {
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
    max_path: Arc<RelPath>,
    count: usize,
    non_ignored_count: usize,
    file_count: usize,
    non_ignored_file_count: usize,
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(RelPath::empty()),
            count: 0,
            non_ignored_count: 0,
            file_count: 0,
            non_ignored_file_count: 0,
        }
    }
}

impl sum_tree::ContextLessSummary for EntrySummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, rhs: &Self) {
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
    path: Arc<RelPath>,
    is_ignored: bool,
    scan_id: usize,
}

impl sum_tree::Item for PathEntry {
    type Summary = PathEntrySummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
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

impl sum_tree::ContextLessSummary for PathEntrySummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.max_id = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, PathEntrySummary> for ProjectEntryId {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a PathEntrySummary, _: ()) {
        *self = summary.max_id;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PathKey(pub Arc<RelPath>);

impl Default for PathKey {
    fn default() -> Self {
        Self(RelPath::empty_arc())
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for PathKey {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a EntrySummary, _: ()) {
        self.0 = summary.max_path.clone();
    }
}

struct BackgroundScanner {
    state: async_lock::Mutex<BackgroundScannerState>,
    fs: Arc<dyn Fs>,
    fs_case_sensitive: bool,
    status_updates_tx: UnboundedSender<ScanState>,
    executor: BackgroundExecutor,
    scan_requests_rx: async_channel::Receiver<ScanRequest>,
    path_prefixes_to_scan_rx: async_channel::Receiver<PathPrefixScanRequest>,
    next_entry_id: Arc<AtomicUsize>,
    phase: BackgroundScannerPhase,
    watcher: Arc<dyn Watcher>,
    settings: WorktreeSettings,
    share_private_files: bool,
    track_git_repositories: bool,
    /// Whether this is a single-file worktree (root is a file, not a directory).
    /// Used to determine if we should give up after repeated canonicalization failures.
    is_single_file: bool,
    defer_watch: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum BackgroundScannerPhase {
    InitialScan,
    EventsReceivedDuringInitialScan,
    Events,
}

impl BackgroundScanner {
    async fn run(&mut self, mut fs_events_rx: Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>) {
        let root_abs_path;
        let scanning_enabled;
        {
            let state = self.state.lock().await;
            root_abs_path = state.snapshot.abs_path.clone();
            scanning_enabled = state.scanning_enabled;
        }

        // If the worktree root does not contain a git repository, then find
        // the git repository in an ancestor directory. Find any gitignore files
        // in ancestor directories. Exclude is loaded at registration time.
        let repo = if scanning_enabled && self.track_git_repositories {
            let (ignores, repo) = discover_ancestor_git_repo(self.fs.clone(), &root_abs_path).await;
            let mut state = self.state.lock().await;
            state.snapshot.ignores_by_parent_abs_path.extend(ignores);
            repo
        } else {
            None
        };

        // `repo` is only `Some` when scanning and git tracking are enabled above.
        let containing_git_repository = if let Some((ancestor_dot_git, work_directory)) = repo {
            let outcome = self
                .state
                .lock()
                .await
                .insert_git_repository_for_path(
                    work_directory,
                    ancestor_dot_git.clone().into(),
                    self.fs.as_ref(),
                    self.watcher.as_ref(),
                )
                .await;
            // Bail on an invalid `.git` (`None`), so a non-repository is never reported
            // as the containing repo.
            outcome.map(|_| ancestor_dot_git)
        } else {
            None
        };

        log::trace!("containing git repository: {containing_git_repository:?}");

        let global_gitignore_file = paths::global_gitignore_path();
        let mut global_gitignore_events = if let Some(global_gitignore_path) =
            &global_gitignore_file
            && scanning_enabled
            && self.track_git_repositories
        {
            let is_file = self.fs.is_file(&global_gitignore_path).await;
            self.state.lock().await.snapshot.global_gitignore = if is_file {
                build_gitignore(global_gitignore_path, self.fs.as_ref())
                    .await
                    .ok()
                    .map(Arc::new)
            } else {
                None
            };
            if is_file {
                self.fs
                    .watch(global_gitignore_path, FS_WATCH_LATENCY)
                    .await
                    .0
            } else {
                Box::pin(futures::stream::pending())
            }
        } else {
            self.state.lock().await.snapshot.global_gitignore = None;
            Box::pin(futures::stream::pending())
        };

        let (scan_job_tx, scan_job_rx) = async_channel::unbounded();
        {
            let mut state = self.state.lock().await;
            state.snapshot.scan_id += 1;
            if let Some(mut root_entry) = state.snapshot.root_entry().cloned() {
                let ignore_stack = state
                    .snapshot
                    .ignore_stack_for_abs_path(root_abs_path.as_path(), true, self.fs.as_ref())
                    .await;
                if ignore_stack.is_abs_path_ignored(root_abs_path.as_path(), true) {
                    root_entry.is_ignored = true;
                    let mut root_entry = root_entry.clone();
                    state.reuse_entry_id(&mut root_entry);
                    state
                        .insert_entry(root_entry, self.fs.as_ref(), self.watcher.as_ref())
                        .await;
                }
                if root_entry.is_dir() && state.scanning_enabled {
                    state
                        .enqueue_scan_dir(
                            root_abs_path.as_path().into(),
                            &root_entry,
                            &scan_job_tx,
                            self.fs.as_ref(),
                        )
                        .await;
                }
            }
        };

        // Perform an initial scan of the directory.
        drop(scan_job_tx);
        self.scan_dirs(true, scan_job_rx).await;
        {
            let mut state = self.state.lock().await;
            state.snapshot.completed_scan_id = state.snapshot.scan_id;
        }

        self.send_status_update(false, SmallVec::new(), &[]).await;

        if self.defer_watch {
            let (events, watcher) = self
                .fs
                .watch(root_abs_path.as_path(), FS_WATCH_LATENCY)
                .await;
            self.watcher = watcher;
            fs_events_rx = Box::pin(events.map(|events| events.into_iter().collect()));

            let state = self.state.lock().await;
            for target in state.symlink_paths_by_target.keys() {
                if !target.starts_with(root_abs_path.as_path()) {
                    self.watcher.add(target).log_err();
                }
            }
            for repo in state.snapshot.git_repositories.values() {
                if !repo
                    .identity
                    .common_dir_abs_path
                    .starts_with(root_abs_path.as_path())
                {
                    self.watcher
                        .add(&repo.identity.common_dir_abs_path)
                        .log_err();
                }
                if !repo
                    .identity
                    .repository_dir_abs_path
                    .starts_with(root_abs_path.as_path())
                {
                    self.watcher
                        .add(&repo.identity.repository_dir_abs_path)
                        .log_err();
                }
            }
            drop(state);
        }

        // Process any any FS events that occurred while performing the initial scan.
        // For these events, update events cannot be as precise, because we didn't
        // have the previous state loaded yet.
        self.phase = BackgroundScannerPhase::EventsReceivedDuringInitialScan;
        if let Poll::Ready(Some(mut paths)) = futures::poll!(fs_events_rx.next()) {
            while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_events_rx.next()) {
                paths.extend(more_paths);
            }
            self.process_events(
                paths
                    .into_iter()
                    .filter(|event| event.kind.is_some())
                    .collect(),
            )
            .await;
        }
        if let Some(abs_path) = containing_git_repository {
            self.process_events(vec![PathEvent {
                path: abs_path,
                kind: Some(fs::PathEventKind::Changed),
            }])
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

                path_prefix_request = self.path_prefixes_to_scan_rx.recv().fuse() => {
                    let Ok(request) = path_prefix_request else { break };

                    if self.state.lock().await.path_prefixes_to_scan.contains(&request.path) {
                        self.send_status_update(false, request.done, &[]).await;
                        continue;
                    }

                    log::trace!("adding path prefix {:?}", request.path);

                    let did_scan = self.forcibly_load_paths(std::slice::from_ref(&request.path)).await;
                    if did_scan {
                        let abs_path =
                        {
                            let mut state = self.state.lock().await;
                            state.path_prefixes_to_scan.insert(request.path.clone());
                            state.snapshot.absolutize(&request.path)
                        };

                        if let Some(abs_path) = self.fs.canonicalize(&abs_path).await.log_err() {
                            self.process_events(vec![PathEvent {
                                path: abs_path,
                                kind: Some(fs::PathEventKind::Changed),
                            }])
                            .await;
                        }
                    }
                    self.send_status_update(false, request.done, &[]).await;
                }

                paths = fs_events_rx.next().fuse() => {
                    let Some(mut paths) = paths else { break };
                    while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_events_rx.next()) {
                        paths.extend(more_paths);
                    }
                    self.process_events(paths.into_iter().filter(|event| event.kind.is_some()).collect()).await;
                }

                _ = global_gitignore_events.next().fuse() => {
                    if let Some(path) = &global_gitignore_file {
                        self.update_global_gitignore(&path).await;
                    }
                }
            }
        }
    }

    async fn process_scan_request(&self, mut request: ScanRequest, scanning: bool) -> bool {
        log::debug!("rescanning paths {:?}", request.relative_paths);

        request.relative_paths.sort_unstable();
        self.forcibly_load_paths(&request.relative_paths).await;

        let root_path = self.state.lock().await.snapshot.abs_path.clone();
        let root_canonical_path = self.fs.canonicalize(root_path.as_path()).await;
        let root_canonical_path = match &root_canonical_path {
            Ok(path) => SanitizedPath::new(path),
            Err(err) => {
                log::error!("failed to canonicalize root path {root_path:?}: {err:#}");
                return true;
            }
        };
        let abs_paths = request
            .relative_paths
            .iter()
            .map(|path| {
                if path.file_name().is_some() {
                    root_canonical_path.as_path().join(path.as_std_path())
                } else {
                    root_canonical_path.as_path().to_path_buf()
                }
            })
            .collect::<Vec<_>>();

        {
            let mut state = self.state.lock().await;
            let is_idle = state.snapshot.completed_scan_id == state.snapshot.scan_id;
            state.snapshot.scan_id += 1;
            if is_idle {
                state.snapshot.completed_scan_id = state.snapshot.scan_id;
            }
        }

        self.reload_entries_for_paths(
            &root_path,
            &root_canonical_path,
            &request.relative_paths,
            abs_paths,
            None,
        )
        .await;

        self.send_status_update(scanning, request.done, &[]).await
    }

    fn normalized_events_for_worktree(
        state: &BackgroundScannerState,
        root_canonical_path: &SanitizedPath,
        mut events: Vec<PathEvent>,
    ) -> Vec<PathEvent> {
        if state.symlink_paths_by_target.is_empty() {
            return events;
        }
        let mut mapped_events = Vec::new();

        events.retain(|event| {
            let abs_path = SanitizedPath::new(&event.path);

            let mut best_match: Option<(&Arc<Path>, &SmallVec<[Arc<RelPath>; 1]>)> = None;
            let mut best_depth = 0;
            for (target_root, symlink_paths) in &state.symlink_paths_by_target {
                if abs_path.as_path().starts_with(target_root.as_ref()) {
                    let depth = target_root.as_ref().components().count();
                    if depth > best_depth {
                        best_depth = depth;
                        best_match = Some((target_root, symlink_paths));
                    }
                }
            }

            let Some((target_root, symlink_paths)) = best_match else {
                return true;
            };

            let Ok(suffix) = abs_path.as_path().strip_prefix(target_root.as_ref()) else {
                return true;
            };

            // If the symlink's real target is outside this worktree, the original path
            // isn't visible to the worktree. Keep only the remapped symlink events.
            let keep_original = target_root.starts_with(root_canonical_path.as_path());

            for symlink_path in symlink_paths {
                let mapped_path = if suffix.as_os_str().is_empty() {
                    root_canonical_path
                        .as_path()
                        .join(symlink_path.as_std_path())
                } else {
                    root_canonical_path
                        .as_path()
                        .join(symlink_path.as_std_path())
                        .join(suffix)
                };
                if mapped_path != event.path {
                    mapped_events.push(PathEvent {
                        path: mapped_path,
                        kind: event.kind,
                    });
                }
            }
            keep_original
        });
        events.extend(mapped_events);
        events
    }

    async fn process_events(&self, mut events: Vec<PathEvent>) {
        let root_path = self.state.lock().await.snapshot.abs_path.clone();
        let root_canonical_path = self.fs.canonicalize(root_path.as_path()).await;
        let root_canonical_path = match &root_canonical_path {
            Ok(path) => SanitizedPath::new(path),
            Err(err) => {
                let new_path = self
                    .state
                    .lock()
                    .await
                    .snapshot
                    .root_file_handle
                    .clone()
                    .and_then(|handle| match handle.current_path(&self.fs) {
                        Ok(new_path) => Some(new_path),
                        Err(e) => {
                            log::error!("Failed to refresh worktree root path: {e:#}");
                            None
                        }
                    })
                    .map(|path| SanitizedPath::new_arc(&path))
                    .filter(|new_path| *new_path != root_path);

                if let Some(new_path) = new_path {
                    log::info!(
                        "root renamed from {:?} to {:?}",
                        root_path.as_path(),
                        new_path.as_path(),
                    );
                    self.status_updates_tx
                        .unbounded_send(ScanState::RootUpdated { new_path })
                        .ok();
                } else {
                    log::error!("root path could not be canonicalized: {err:#}");

                    // For single-file worktrees, if we can't canonicalize and the file handle
                    // fallback also failed, the file is gone - close the worktree
                    if self.is_single_file {
                        log::info!(
                            "single-file worktree root {:?} no longer exists, marking as deleted",
                            root_path.as_path()
                        );
                        self.status_updates_tx
                            .unbounded_send(ScanState::RootDeleted)
                            .ok();
                    }
                }
                return;
            }
        };

        {
            let state = self.state.lock().await;
            events = Self::normalized_events_for_worktree(&state, &root_canonical_path, events);
        }

        log::debug!("raw events for process_events: {events:?}");

        fn skip_ix(ranges: &mut SmallVec<[Range<usize>; 4]>, ix: usize) {
            if let Some(last_range) = ranges.last_mut()
                && last_range.end == ix
            {
                last_range.end += 1;
            } else {
                ranges.push(ix..ix + 1);
            }
        }

        // Check for events inside .git directories, so that we know which repositories need their git state reloaded.
        //
        // Certain directories may have FS changes, but do not lead to git data changes that Zed cares about.
        // Ignore these, to avoid Zed unnecessarily rescanning git metadata.
        let skipped_file_names_in_dot_git =
            [COMMIT_MESSAGE, FETCH_HEAD, ORIG_HEAD, BISECT_LOG, GC_PID];
        // `objects` is intentionally absent: it is a repository-validity input
        // (`common_dir_has_stores`), so the bare `objects` path must revalidate. Only
        // descendants (loose-object writes) are skipped, via the carve-out below.
        let skipped_dirs_in_dot_git = [
            FSMONITOR_DAEMON,
            LFS_DIR,
            HOOKS_DIR,
            REBASE_MERGE_DIR,
            REBASE_APPLY_DIR,
            SEQUENCER_DIR,
        ];

        let mut dot_git_abs_paths = Vec::new();
        let mut repo_ids_needing_exclude_update = Vec::new();

        {
            let snapshot = &self.state.lock().await.snapshot;

            let mut ranges_to_drop = SmallVec::<[Range<usize>; 4]>::new();

            for (ix, event) in events.iter().enumerate() {
                let abs_path = SanitizedPath::new(&event.path);

                let dot_git_paths = if self.track_git_repositories {
                    match_git_metadata_event(snapshot, abs_path.as_path())
                } else {
                    None
                };

                if let Some((dot_git_abs_path, path_in_git_dir)) = dot_git_paths {
                    let is_ignored = skipped_file_names_in_dot_git.iter().any(|skipped| {
                        path_in_git_dir
                            .file_name()
                            .is_some_and(|file_name| file_name == OsStr::new(skipped))
                    }) || (path_in_git_dir.starts_with(LOGS_DIR)
                        && path_in_git_dir != Path::new(LOGS_REF_STASH))
                        || (path_in_git_dir.starts_with(INFO_DIR)
                            && path_in_git_dir != Path::new(REPO_EXCLUDE))
                        || (path_in_git_dir.starts_with(OBJECTS_DIR)
                            && path_in_git_dir != Path::new(OBJECTS_DIR))
                        || skipped_dirs_in_dot_git.iter().any(|skipped_git_subdir| {
                            path_in_git_dir.starts_with(skipped_git_subdir)
                        })
                        || path_in_git_dir.extension().is_some_and(|ext| ext == "lock")
                        || (path_in_git_dir.components().count() == 1
                            && path_in_git_dir
                                .extension()
                                .is_some_and(|ext| ext == "new" || ext == "tmp"));
                    let is_dot_git = path_in_git_dir == Path::new("")
                        && matches!(event.kind, Some(PathEventKind::Changed))
                        && self.fs.is_dir(&dot_git_abs_path).await;
                    if is_ignored {
                        log::debug!(
                            "ignoring event {abs_path:?} as it's in the .git directory among skipped files or directories"
                        );
                        skip_ix(&mut ranges_to_drop, ix);
                        continue;
                    }
                    if is_dot_git {
                        log::debug!(
                            "ignoring event {abs_path:?} for .git directory itself (kind: {:?})",
                            event.kind
                        );
                        skip_ix(&mut ranges_to_drop, ix);
                        continue;
                    }

                    // New directories can appear under the `refs` tree at any time, e.g. when a
                    // remote is added or a branch name contains slashes. On platforms where the
                    // native watcher is non-recursive they need their own watches, or subsequent
                    // ref updates inside them would go unnoticed. The subtree is walked because
                    // nested directories may have been created before this watch took effect.
                    if matches!(event.kind, Some(PathEventKind::Created))
                        && path_in_git_dir
                            .components()
                            .any(|component| component.as_os_str() == OsStr::new(REFS_DIR))
                    {
                        watch_dir_tree(
                            abs_path.as_path().to_path_buf(),
                            self.fs.as_ref(),
                            self.watcher.as_ref(),
                        )
                        .await;
                    }

                    if !dot_git_abs_paths.contains(&dot_git_abs_path) {
                        log::debug!(
                            "detected update within git repo at {dot_git_abs_path:?}: {abs_path:?}"
                        );
                        dot_git_abs_paths.push(dot_git_abs_path);
                    }
                }

                // Route `info/exclude` changes by the resolved common dir, not by a
                // literal `.git/info/exclude` suffix, so a linked worktree or bare repo
                // whose common dir is named `foo.git`/`.bare` still refreshes its cache.
                // A main checkout and linked worktree can share one common dir while
                // each holding its own entry — dirty every match.
                if self.track_git_repositories && abs_path.as_path().ends_with(REPO_EXCLUDE) {
                    for (id, _) in snapshot.git_repositories.iter().filter(|(_, repo)| {
                        repo.identity.common_dir_abs_path.join(REPO_EXCLUDE) == abs_path.as_path()
                    }) {
                        repo_ids_needing_exclude_update.push(*id);
                    }
                }
            }

            for range_to_drop in ranges_to_drop.into_iter().rev() {
                events.drain(range_to_drop);
            }
        }

        events.sort_unstable_by(|left, right| left.path.cmp(&right.path));
        events.dedup_by(|left, right| {
            if left.path == right.path {
                if matches!(left.kind, Some(fs::PathEventKind::Rescan)) {
                    right.kind = left.kind;
                }
                true
            } else if left.path.starts_with(&right.path) {
                if matches!(left.kind, Some(fs::PathEventKind::Rescan)) {
                    right.kind = left.kind;
                }
                true
            } else {
                false
            }
        });

        let mut relative_paths = Vec::with_capacity(events.len());

        {
            let snapshot = &self.state.lock().await.snapshot;

            let mut ranges_to_drop = SmallVec::<[Range<usize>; 4]>::new();

            for (ix, event) in events.iter().enumerate() {
                let abs_path = SanitizedPath::new(&event.path);
                // TODO: this strips the root case-sensitively, so on a case-insensitive
                // volume an event whose casing differs from the canonical root is
                // dropped. Once `fs` exposes per-volume case-sensitivity (e.g. on the
                // `Fs` trait, with a per-volume cache + `FakeFs` support), fold this
                // comparison on case-insensitive volumes.
                let relative_path = if let Ok(path) = abs_path.strip_prefix(&root_canonical_path)
                    && let Ok(path) = RelPath::new(path, PathStyle::local())
                {
                    path
                } else if let Ok(path) = abs_path.strip_prefix(&root_path)
                    && let Ok(path) = RelPath::new(path, PathStyle::local())
                {
                    path
                } else if let Some(path) = snapshot.external_canonical_to_relative.iter().find_map(
                    |(canonical, relative)| {
                        abs_path
                            .as_path()
                            .strip_prefix(canonical.as_ref())
                            .ok()
                            .and_then(|suffix| {
                                RelPath::new(suffix, PathStyle::local())
                                    .ok()
                                    .map(|suffix_rel| {
                                        std::borrow::Cow::Owned(
                                            relative.join(&suffix_rel).to_rel_path_buf(),
                                        )
                                    })
                            })
                    },
                ) {
                    path
                } else {
                    skip_ix(&mut ranges_to_drop, ix);
                    continue;
                };

                if self.track_git_repositories
                    && abs_path.file_name() == Some(OsStr::new(GITIGNORE))
                {
                    for (_, repo) in snapshot.git_repositories.iter().filter(|(_, repo)| {
                        // Logical relative event path — not the raw absolute path —
                        // preserves external-symlink routing.
                        snapshot.repository_directory_contains(repo, &relative_path)
                    }) {
                        if !dot_git_abs_paths.iter().any(|dot_git_abs_path| {
                            dot_git_abs_path == repo.identity.common_dir_abs_path.as_ref()
                        }) {
                            dot_git_abs_paths.push(repo.identity.common_dir_abs_path.to_path_buf());
                        }
                    }
                }

                let parent_dir_is_loaded = relative_path.parent().is_none_or(|parent| {
                    snapshot
                        .entry_for_path(parent)
                        .is_some_and(|entry| entry.kind == EntryKind::Dir)
                });
                if !parent_dir_is_loaded {
                    log::debug!("filtering event {relative_path:?} within unloaded directory");
                    skip_ix(&mut ranges_to_drop, ix);
                    continue;
                }

                if self.settings.is_path_excluded(&relative_path) {
                    skip_ix(&mut ranges_to_drop, ix);
                    continue;
                }

                relative_paths.push(EventRoot {
                    path: relative_path.into_arc(),
                    was_rescanned: matches!(event.kind, Some(fs::PathEventKind::Rescan)),
                });
            }

            for range_to_drop in ranges_to_drop.into_iter().rev() {
                events.drain(range_to_drop);
            }
        }

        if relative_paths.is_empty() && dot_git_abs_paths.is_empty() {
            return;
        }

        if !repo_ids_needing_exclude_update.is_empty() {
            let mut state = self.state.lock().await;
            for work_directory_id in repo_ids_needing_exclude_update {
                state
                    .snapshot
                    .git_repositories
                    .update(&work_directory_id, |entry| {
                        entry.info_exclude_needs_update = true;
                    });
            }
        }

        self.state.lock().await.snapshot.scan_id += 1;

        let (scan_job_tx, scan_job_rx) = async_channel::unbounded();
        if !relative_paths.is_empty() {
            log::debug!(
                "will update project paths {:?}",
                relative_paths
                    .iter()
                    .map(|event_root| &event_root.path)
                    .collect::<Vec<_>>()
            );
        }
        self.reload_entries_for_paths(
            &root_path,
            &root_canonical_path,
            &relative_paths
                .iter()
                .map(|event_root| event_root.path.clone())
                .collect::<Vec<_>>(),
            events
                .into_iter()
                .map(|event| event.path)
                .collect::<Vec<_>>(),
            Some(scan_job_tx.clone()),
        )
        .await;

        let affected_repo_roots = if !dot_git_abs_paths.is_empty() {
            self.update_git_repositories(dot_git_abs_paths).await
        } else {
            Vec::new()
        };

        {
            let mut ignores_to_update = self.ignores_needing_update().await;
            ignores_to_update.extend(self.reload_repository_excludes().await);
            ignores_to_update.extend(affected_repo_roots);
            let ignores_to_update = self.order_ignores(ignores_to_update).await;
            let snapshot = self.state.lock().await.snapshot.clone();
            self.update_ignore_statuses_for_paths(scan_job_tx, snapshot, ignores_to_update)
                .await;
            self.scan_dirs(false, scan_job_rx).await;
        }

        {
            let mut state = self.state.lock().await;
            state.snapshot.completed_scan_id = state.snapshot.scan_id;
            let RemovedEntries { by_inode, by_path } = mem::take(&mut state.removed_entries);
            for entry in by_inode.into_values().chain(by_path.into_values()) {
                state.scanned_dirs.remove(&entry.id);
            }
        }
        self.send_status_update(false, SmallVec::new(), &relative_paths)
            .await;
    }

    async fn update_global_gitignore(&self, abs_path: &Path) {
        let ignore = build_gitignore(abs_path, self.fs.as_ref())
            .await
            .log_err()
            .map(Arc::new);
        let (prev_snapshot, ignore_stack, abs_path) = {
            let mut state = self.state.lock().await;
            state.snapshot.global_gitignore = ignore;
            let abs_path = state.snapshot.abs_path().clone();
            let ignore_stack = state
                .snapshot
                .ignore_stack_for_abs_path(&abs_path, true, self.fs.as_ref())
                .await;
            (state.snapshot.clone(), ignore_stack, abs_path)
        };
        let (scan_job_tx, scan_job_rx) = async_channel::unbounded();
        self.update_ignore_statuses_for_paths(
            scan_job_tx,
            prev_snapshot,
            vec![(abs_path, ignore_stack)],
        )
        .await;
        self.scan_dirs(false, scan_job_rx).await;
        self.send_status_update(false, SmallVec::new(), &[]).await;
    }

    async fn forcibly_load_paths(&self, paths: &[Arc<RelPath>]) -> bool {
        let (scan_job_tx, scan_job_rx) = async_channel::unbounded();
        {
            let mut state = self.state.lock().await;
            let root_path = state.snapshot.abs_path.clone();
            for path in paths {
                for ancestor in path.ancestors() {
                    if let Some(entry) = state.snapshot.entry_for_path(ancestor)
                        && entry.kind == EntryKind::UnloadedDir
                    {
                        let abs_path = if entry.is_external {
                            entry
                                .canonical_path
                                .as_ref()
                                .map(|path| path.as_ref().to_path_buf())
                                .unwrap_or_else(|| root_path.join(ancestor.as_std_path()))
                        } else {
                            root_path.join(ancestor.as_std_path())
                        };
                        state
                            .enqueue_scan_dir(
                                abs_path.into(),
                                entry,
                                &scan_job_tx,
                                self.fs.as_ref(),
                            )
                            .await;
                        state.paths_to_scan.insert(path.clone());
                        break;
                    }
                }
            }
            drop(scan_job_tx);
        }
        while let Ok(job) = scan_job_rx.recv().await {
            self.scan_dir(&job).await.log_err();
        }

        !mem::take(&mut self.state.lock().await.paths_to_scan).is_empty()
    }

    async fn scan_dirs(
        &self,
        enable_progress_updates: bool,
        scan_jobs_rx: async_channel::Receiver<ScanJob>,
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
            .scoped_priority(Priority::Low, |scope| {
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
                                            self.send_status_update(true, SmallVec::new(), &[])
                                                .await;
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
                                    if let Err(err) = self.scan_dir(&job).await
                                        && job.path.is_empty() {
                                            log::error!("error scanning directory {:?}: {}", job.abs_path, err);
                                        }
                                }
                            }
                        }
                    });
                }
            })
            .await;
    }

    async fn send_status_update(
        &self,
        scanning: bool,
        barrier: SmallVec<[barrier::Sender; 1]>,
        event_roots: &[EventRoot],
    ) -> bool {
        let mut state = self.state.lock().await;
        if state.changed_paths.is_empty() && event_roots.is_empty() && scanning {
            return true;
        }

        let merged_event_roots = merge_event_roots(&state.changed_paths, event_roots);

        let new_snapshot = state.snapshot.clone();
        let old_snapshot = mem::replace(&mut state.prev_snapshot, new_snapshot.snapshot.clone());
        let changes = build_diff(
            self.phase,
            &old_snapshot,
            &new_snapshot,
            &merged_event_roots,
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
            let snapshot = &self.state.lock().await.snapshot;
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
        swap_to_front(&mut child_paths, GITIGNORE);
        swap_to_front(&mut child_paths, DOT_GIT);

        if let Some(path) = child_paths.first()
            && path.ends_with(DOT_GIT)
            && discover_valid_git_repository(path, self.fs.as_ref())
                .await
                .is_some()
        {
            ignore_stack.repo_root = Some(job.abs_path.clone());
        }

        for child_abs_path in child_paths {
            let child_abs_path: Arc<Path> = child_abs_path.into();
            let child_name = child_abs_path.file_name().unwrap();
            let Some(child_path) = child_name
                .to_str()
                .and_then(|name| Some(job.path.join(RelPath::from_unix_str(name).ok()?)))
            else {
                continue;
            };
            let child_path: Arc<RelPath> = child_path.into();

            if self.track_git_repositories {
                if child_name == DOT_GIT {
                    let mut state = self.state.lock().await;
                    // On the initial scan a nested repo is not yet registered when
                    // the job's stack was built (child jobs inherit the parent's
                    // stack), so siblings would miss info/exclude. Append after a
                    // *new* registration only — rescans already have the exclude
                    // on the stack from `ignore_stack_for_abs_path`.
                    let outcome = state
                        .insert_git_repository(
                            child_path.clone(),
                            self.fs.as_ref(),
                            self.watcher.as_ref(),
                        )
                        .await;
                    if let Some(outcome) = outcome
                        && outcome.was_added
                        && let Some(entry) = state
                            .snapshot
                            .git_repositories
                            .get(&outcome.work_directory_id)
                    {
                        ignore_stack = ignore_stack
                            .append(IgnoreKind::RepoExclude, entry.info_exclude.clone());
                    }
                } else if child_name == GITIGNORE {
                    match build_gitignore(&child_abs_path, self.fs.as_ref()).await {
                        Ok(ignore) => {
                            let ignore = Arc::new(ignore);
                            ignore_stack = ignore_stack.append(
                                IgnoreKind::Gitignore(job.abs_path.clone()),
                                ignore.clone(),
                            );
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
            }

            if self.settings.is_path_excluded(&child_path) {
                log::debug!("skipping excluded child entry {child_path:?}");

                self.state
                    .lock()
                    .await
                    .remove_path_from_snapshot_and_unwatch(
                        &child_path,
                        self.watcher.as_ref(),
                        true,
                    );
                continue;
            }

            let child_metadata = match self.fs.metadata(&child_abs_path).await {
                Ok(Some(metadata)) => metadata,
                Ok(None) => continue,
                Err(err) => {
                    log::error!("error processing {:?}: {err:#}", child_abs_path.display());
                    continue;
                }
            };

            let mut child_entry = Entry::new(
                child_path.clone(),
                &child_metadata,
                ProjectEntryId::new(&next_entry_id),
                root_char_bag,
                None,
            );

            if job.is_external {
                child_entry.is_external = true;
            } else if child_metadata.is_symlink {
                let canonical_path = match self.fs.canonicalize(&child_abs_path).await {
                    Ok(path) => path,
                    Err(err) => {
                        log::error!("error reading target of symlink {child_abs_path:?}: {err:#}",);
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

                if child_metadata.is_dir {
                    let mut state = self.state.lock().await;
                    let paths = state
                        .symlink_paths_by_target
                        .entry(Arc::from(canonical_path.clone()))
                        .or_default();
                    if !paths.iter().any(|path| path == &child_path) {
                        paths.push(child_path.clone());
                    }
                }

                child_entry.canonical_path = Some(canonical_path.into());
            }

            if child_entry.is_dir() {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, true);
                child_entry.is_always_included =
                    self.settings.is_path_always_included(&child_path, true);

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
                child_entry.is_always_included =
                    self.settings.is_path_always_included(&child_path, false);
            }

            {
                let relative_path = job
                    .path
                    .join(RelPath::from_unix_str(child_name.to_str().unwrap()).unwrap());
                if self.is_path_private(&relative_path) {
                    log::debug!("detected private file: {relative_path:?}");
                    child_entry.is_private = true;
                }
                if self.settings.is_path_hidden(&relative_path) {
                    log::debug!("detected hidden file: {relative_path:?}");
                    child_entry.is_hidden = true;
                }
            }

            new_entries.push(child_entry);
        }

        let mut state = self.state.lock().await;
        // Identify any subdirectories that should not be scanned.
        let mut job_ix = 0;
        for entry in &mut new_entries {
            state.reuse_entry_id(entry);
            if entry.is_dir() {
                if self.should_scan_directory(&state, entry) {
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

        state.populate_dir(job.path.clone(), new_entries, new_ignore);
        // For external entries, watch the canonical (resolved) path so OS-level
        // FS events on the real filesystem location are observed. The same
        // canonical path is stored in both `external_canonical_to_relative`
        // (for translating canonical-path FS events back to worktree-relative
        // paths) and `watched_dir_abs_paths_by_entry_id` (used by `remove_path`
        // to know which abs path to unwatch), so both cleanup paths agree on
        // the path the watcher was actually registered on.
        //
        // `canonicalize` is an async filesystem operation that may suspend, so
        // the lock must not be held across the await point below.
        drop(state);
        let watched_abs_path: Option<Arc<Path>> = if job.is_external {
            self.fs
                .canonicalize(job.abs_path.as_ref())
                .await
                .ok()
                .map(|canonical| {
                    let canonical: Arc<Path> = canonical.into();
                    self.watcher.add(&canonical).log_err();
                    canonical
                })
        } else {
            self.watcher.add(job.abs_path.as_ref()).log_err();
            Some(job.abs_path.clone())
        };

        let mut state = self.state.lock().await;
        if let Some(watched_abs_path) = &watched_abs_path {
            if job.is_external {
                state
                    .snapshot
                    .external_canonical_to_relative
                    .insert(watched_abs_path.clone(), job.path.clone());
            }
            if let Some(entry_id) = state
                .snapshot
                .entry_for_path(&job.path)
                .map(|entry| entry.id)
            {
                state
                    .watched_dir_abs_paths_by_entry_id
                    .insert(entry_id, watched_abs_path.clone());
            }
        }

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
        root_abs_path: &SanitizedPath,
        root_canonical_path: &SanitizedPath,
        relative_paths: &[Arc<RelPath>],
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

                        anyhow::Ok(Some((metadata, SanitizedPath::new_arc(&canonical_path))))
                    } else {
                        Ok(None)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .await;

        let mut new_ancestor_repo =
            if self.track_git_repositories && relative_paths.iter().any(|path| path.is_empty()) {
                Some(discover_ancestor_git_repo(self.fs.clone(), &root_abs_path).await)
            } else {
                None
            };

        let mut state = self.state.lock().await;
        let doing_recursive_update = scan_queue_tx.is_some();

        // Remove any entries for paths that no longer exist or are being recursively
        // refreshed. Do this before adding any new entries, so that renames can be
        // detected regardless of the order of the paths.
        let mut paths_to_process = Vec::with_capacity(relative_paths.len());
        for (path, metadata) in relative_paths.iter().zip(metadata.iter()) {
            let path_was_removed = matches!(metadata, Ok(None));
            let removed_descendant_paths = if path_was_removed || doing_recursive_update {
                state.remove_path_from_snapshot(path, path_was_removed)
            } else {
                Vec::new()
            };
            paths_to_process.push((path, metadata, removed_descendant_paths));
        }

        for (path, metadata, removed_descendant_abs_paths) in paths_to_process {
            let abs_path: Arc<Path> = root_abs_path.join(path.as_std_path()).into();
            match metadata {
                Ok(Some((metadata, canonical_path))) => {
                    let ignore_stack = state
                        .snapshot
                        .ignore_stack_for_abs_path(&abs_path, metadata.is_dir, self.fs.as_ref())
                        .await;
                    let is_external = !canonical_path.starts_with(&root_canonical_path);
                    let entry_id = state.entry_id_for(self.next_entry_id.as_ref(), path, &metadata);
                    let mut fs_entry = Entry::new(
                        path.clone(),
                        &metadata,
                        entry_id,
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
                    fs_entry.is_always_included =
                        self.settings.is_path_always_included(path, is_dir);
                    fs_entry.is_hidden = self.settings.is_path_hidden(path);

                    if let (Some(scan_queue_tx), true) = (&scan_queue_tx, is_dir) {
                        if self.should_scan_directory(&state, &fs_entry)
                            || (self.track_git_repositories
                                && fs_entry.path.is_empty()
                                && abs_path.file_name() == Some(OsStr::new(DOT_GIT)))
                        {
                            state
                                .enqueue_scan_dir(
                                    abs_path,
                                    &fs_entry,
                                    scan_queue_tx,
                                    self.fs.as_ref(),
                                )
                                .await;
                        } else {
                            fs_entry.kind = EntryKind::UnloadedDir;
                        }
                    }

                    state
                        .insert_entry(fs_entry.clone(), self.fs.as_ref(), self.watcher.as_ref())
                        .await;

                    if path.is_empty()
                        && let Some((ignores, repo)) = new_ancestor_repo.take()
                    {
                        log::trace!("updating ancestor git repository");
                        state.snapshot.ignores_by_parent_abs_path.extend(ignores);
                        if let Some((ancestor_dot_git, work_directory)) = repo {
                            state
                                .insert_git_repository_for_path(
                                    work_directory,
                                    ancestor_dot_git.into(),
                                    self.fs.as_ref(),
                                    self.watcher.as_ref(),
                                )
                                .await;
                        }
                    }
                }
                Ok(None) => {
                    // Path+registration pruning already done by
                    // `remove_path_from_snapshot` with repository pruning enabled.
                    state.unwatch_path(
                        self.watcher.as_ref(),
                        path,
                        removed_descendant_abs_paths,
                        false,
                    );
                }
                Err(err) => {
                    log::error!("error reading file {abs_path:?} on event: {err:#}");
                    state.unwatch_path(
                        self.watcher.as_ref(),
                        path,
                        removed_descendant_abs_paths,
                        false,
                    );
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

    async fn update_ignore_statuses_for_paths(
        &self,
        scan_job_tx: Sender<ScanJob>,
        prev_snapshot: LocalSnapshot,
        ignores_to_update: Vec<(Arc<Path>, IgnoreStack)>,
    ) {
        let (ignore_queue_tx, ignore_queue_rx) = async_channel::unbounded();
        {
            for (parent_abs_path, ignore_stack) in ignores_to_update {
                ignore_queue_tx
                    .send_blocking(UpdateIgnoreStatusJob {
                        abs_path: parent_abs_path,
                        ignore_stack,
                        ignore_queue: ignore_queue_tx.clone(),
                        scan_queue: scan_job_tx.clone(),
                    })
                    .unwrap();
            }
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

    /// Collects `.gitignore` parents whose dirty bit is set. Repository
    /// `info/exclude` reloads live in `reload_repository_excludes`.
    async fn ignores_needing_update(&self) -> Vec<Arc<Path>> {
        let mut ignores_to_update = Vec::new();
        {
            let snapshot = &mut self.state.lock().await.snapshot;
            let abs_path = snapshot.abs_path.clone();

            snapshot
                .ignores_by_parent_abs_path
                .retain(|parent_abs_path, (_, needs_update)| {
                    if let Ok(parent_path) = parent_abs_path.strip_prefix(abs_path.as_path())
                        && let Some(parent_path) =
                            RelPath::new(&parent_path, PathStyle::local()).log_err()
                    {
                        if *needs_update {
                            *needs_update = false;
                            if snapshot.snapshot.entry_for_path(&parent_path).is_some() {
                                ignores_to_update.push(parent_abs_path.clone());
                            }
                        }

                        let ignore_path =
                            parent_path.join(RelPath::from_unix_str(GITIGNORE).unwrap());
                        if snapshot.snapshot.entry_for_path(&ignore_path).is_none() {
                            return false;
                        }
                    }
                    true
                });
        }
        ignores_to_update
    }

    /// Reloads pending `info/exclude` files for registered repositories.
    ///
    /// Snapshots `(ProjectEntryId, common_dir, work_dir)` under the lock, loads
    /// outside it, then reapplies only when the entry's common dir still matches
    /// (closes the load-after-retarget race). Four-outcome semantics:
    /// Present=install+clear; Absent=empty+clear; Unparsable=keep last-good+clear+log;
    /// Indeterminate=keep+stay-pending+log.
    async fn reload_repository_excludes(&self) -> Vec<Arc<Path>> {
        let pending: Vec<(ProjectEntryId, Arc<Path>, Arc<Path>)> = {
            let snapshot = &self.state.lock().await.snapshot;
            snapshot
                .git_repositories
                .iter()
                .filter(|(_, entry)| entry.info_exclude_needs_update)
                .map(|(&id, entry)| {
                    (
                        id,
                        entry.identity.common_dir_abs_path.clone(),
                        entry.identity.work_directory_abs_path.clone(),
                    )
                })
                .collect()
        };

        if pending.is_empty() {
            return Vec::new();
        }

        // Load (and Arc/empty-matcher/log) before taking the snapshot lock.
        let mut loaded: Vec<(ProjectEntryId, Arc<Path>, Arc<Path>, RepositoryExcludeApply)> =
            Vec::new();
        for (id, common_dir, work_dir) in pending {
            let exclude_abs_path = common_dir.join(REPO_EXCLUDE);
            let apply =
                load_gitignore_existing(&exclude_abs_path, &work_dir, self.fs.as_ref()).await;
            loaded.push((id, common_dir, work_dir, apply));
        }

        let mut ignores_to_update = Vec::new();
        {
            let snapshot = &mut self.state.lock().await.snapshot;
            let abs_path = snapshot.abs_path.clone();
            for (id, expected_common_dir, expected_work_dir, apply) in loaded {
                let Some(entry) = snapshot.git_repositories.get(&id) else {
                    continue;
                };
                // Drop loads whose identity moved underfoot (retarget / concurrent scan).
                if entry.identity.common_dir_abs_path != expected_common_dir
                    || entry.identity.work_directory_abs_path != expected_work_dir
                {
                    continue;
                }
                let work_dir = entry.identity.work_directory_abs_path.clone();
                let recompute = snapshot
                    .git_repositories
                    .update(&id, |entry| apply_repository_exclude_to_entry(entry, apply))
                    .unwrap_or(false);
                if recompute {
                    if work_dir.starts_with(abs_path.as_path()) {
                        ignores_to_update.push(work_dir);
                    } else {
                        ignores_to_update.push(abs_path.as_path().into());
                    }
                }
            }
        }
        ignores_to_update
    }

    async fn order_ignores(&self, mut ignores: Vec<Arc<Path>>) -> Vec<(Arc<Path>, IgnoreStack)> {
        let fs = self.fs.clone();
        let snapshot = self.state.lock().await.snapshot.clone();
        ignores.sort_unstable();
        let mut ignores_to_update = ignores.into_iter().peekable();

        let mut result = vec![];
        while let Some(parent_abs_path) = ignores_to_update.next() {
            while ignores_to_update
                .peek()
                .map_or(false, |p| p.starts_with(&parent_abs_path))
            {
                ignores_to_update.next().unwrap();
            }
            let ignore_stack = snapshot
                .ignore_stack_for_abs_path(&parent_abs_path, true, fs.as_ref())
                .await;
            result.push((parent_abs_path, ignore_stack));
        }

        result
    }

    async fn update_ignore_status(&self, job: UpdateIgnoreStatusJob, snapshot: &LocalSnapshot) {
        log::trace!("update ignore status {:?}", job.abs_path);

        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores_by_parent_abs_path.get(&job.abs_path) {
            ignore_stack =
                ignore_stack.append(IgnoreKind::Gitignore(job.abs_path.clone()), ignore.clone());
        }

        let mut entries_by_id_edits = Vec::new();
        let mut entries_by_path_edits = Vec::new();
        let Some(path) = job
            .abs_path
            .strip_prefix(snapshot.abs_path.as_path())
            .map_err(|_| {
                anyhow::anyhow!(
                    "Failed to strip prefix '{}' from path '{}'",
                    snapshot.abs_path.as_path().display(),
                    job.abs_path.display()
                )
            })
            .log_err()
        else {
            return;
        };

        let Some(path) = RelPath::new(&path, PathStyle::local()).log_err() else {
            return;
        };

        // Anchor the ignore root at a *validated* repository (a `.git` directory or a
        // linked-worktree gitfile), so an invalid/leftover `.git` doesn't shadow it.
        if discover_valid_git_repository(&job.abs_path.join(DOT_GIT), self.fs.as_ref())
            .await
            .is_some()
        {
            ignore_stack.repo_root = Some(job.abs_path.clone());
        }

        for mut entry in snapshot.child_entries(&path).cloned() {
            let was_ignored = entry.is_ignored;
            let abs_path: Arc<Path> = snapshot.absolutize(&entry.path).into();
            entry.is_ignored = ignore_stack.is_abs_path_ignored(&abs_path, entry.is_dir());

            if entry.is_dir() {
                let child_ignore_stack = if entry.is_ignored {
                    IgnoreStack::all()
                } else {
                    ignore_stack.clone()
                };

                // Scan any directories that were previously ignored and weren't previously scanned.
                if was_ignored && !entry.is_ignored && entry.kind.is_unloaded() {
                    let state = self.state.lock().await;
                    if self.should_scan_directory(&state, &entry) {
                        state
                            .enqueue_scan_dir(
                                abs_path.clone(),
                                &entry,
                                &job.scan_queue,
                                self.fs.as_ref(),
                            )
                            .await;
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
                let mut path_entry = snapshot.entries_by_id.get(&entry.id, ()).unwrap().clone();
                path_entry.scan_id = snapshot.scan_id;
                path_entry.is_ignored = entry.is_ignored;
                entries_by_id_edits.push(Edit::Insert(path_entry));
                entries_by_path_edits.push(Edit::Insert(entry));
            }
        }

        let state = &mut self.state.lock().await;
        for edit in &entries_by_path_edits {
            if let Edit::Insert(entry) = edit
                && let Err(ix) = state.changed_paths.binary_search(&entry.path)
            {
                state.changed_paths.insert(ix, entry.path.clone());
            }
        }

        state
            .snapshot
            .entries_by_path
            .edit(entries_by_path_edits, ());
        state.snapshot.entries_by_id.edit(entries_by_id_edits, ());
    }

    async fn update_git_repositories(&self, dot_git_paths: Vec<PathBuf>) -> Vec<Arc<Path>> {
        log::trace!("reloading repositories: {dot_git_paths:?}");
        let mut state = self.state.lock().await;
        let scan_id = state.snapshot.scan_id;
        let mut affected_repo_roots = Vec::new();
        for dot_git_dir in dot_git_paths {
            // Several repositories can share a git directory: a linked worktree's
            // commondir is the main checkout's `.git`, so a ref update there must
            // refresh every repository that reads from it.
            let existing_work_directory_ids = state
                .snapshot
                .git_repositories
                .iter()
                .filter_map(|(&work_directory_id, repo)| {
                    let dot_git_dir = SanitizedPath::new(&dot_git_dir);
                    if SanitizedPath::new(repo.identity.common_dir_abs_path.as_ref()) == dot_git_dir
                        || SanitizedPath::new(repo.identity.repository_dir_abs_path.as_ref())
                            == dot_git_dir
                        || SanitizedPath::new(repo.identity.dot_git_abs_path.as_ref())
                            == dot_git_dir
                    {
                        Some(work_directory_id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if existing_work_directory_ids.is_empty() {
                let Ok(relative) = dot_git_dir.strip_prefix(state.snapshot.abs_path()) else {
                    // A `.git` path outside the worktree root is not
                    // ours to register. This happens legitimately when
                    // `.git` is a gitfile pointing outside the worktree
                    // (linked worktrees and submodules), and also when
                    // a rescan of a linked worktree's commondir arrives
                    // after the worktree's repository has already been
                    // unregistered.
                    continue;
                };
                affected_repo_roots.push(dot_git_dir.parent().unwrap().into());
                state
                    .insert_git_repository(
                        RelPath::new(relative, PathStyle::local())
                            .unwrap()
                            .into_arc(),
                        self.fs.as_ref(),
                        self.watcher.as_ref(),
                    )
                    .await;
            } else {
                for work_directory_id in existing_work_directory_ids {
                    state
                        .snapshot
                        .git_repositories
                        .update(&work_directory_id, |entry| {
                            entry.git_dir_scan_id = scan_id;
                        });
                }
            }
        }

        // Revalidate every registered repository against the filesystem, relying on the
        // resolver's tri-state: `Ok(None)` is a confirmed non-repository and removes the
        // registration, `Err(_)` is a transient failure that must *preserve* the entry —
        // otherwise the repository flaps out of and back into the snapshot, churning its
        // `RepositoryId` — and a valid-but-different result retargets the entry in place
        // (same identity).
        let registered = state
            .snapshot
            .git_repositories
            .iter()
            .map(|(&id, repo)| {
                (
                    id,
                    repo.identity.dot_git_abs_path.clone(),
                    repo.identity.repository_dir_abs_path.clone(),
                    repo.identity.common_dir_abs_path.clone(),
                    repo.identity.work_directory_abs_path.clone(),
                )
            })
            .collect::<Vec<_>>();
        for (id, dot_git_abs_path, repository_dir, common_dir, work_directory_abs_path) in
            registered
        {
            match fs::resolve_git_repository(&dot_git_abs_path, self.fs.as_ref()).await {
                Ok(Some((new_repository_dir, new_common_dir)))
                    if new_repository_dir.as_path() != repository_dir.as_ref()
                        || new_common_dir.as_path() != common_dir.as_ref() =>
                {
                    let new_repository_dir = Arc::<Path>::from(new_repository_dir);
                    let new_common_dir = Arc::<Path>::from(new_common_dir);
                    // Add watches for the new metadata dirs before publishing the retarget
                    // so no events under them are missed. Old watches are left in place: a
                    // watch that may be shared with another linked worktree is unsafe to
                    // remove, and a stale watch only produces events that no longer match a
                    // registered repository and are ignored.
                    self.watcher.add(&new_common_dir).log_err();
                    self.watcher.add(&new_repository_dir).log_err();
                    watch_git_dir_subdirectories(
                        &new_common_dir,
                        self.fs.as_ref(),
                        self.watcher.as_ref(),
                    )
                    .await;
                    if new_repository_dir != new_common_dir {
                        watch_git_dir_subdirectories(
                            &new_repository_dir,
                            self.fs.as_ref(),
                            self.watcher.as_ref(),
                        )
                        .await;
                    }
                    // Retarget updates identity + dirties exclude in the same `.update()`.
                    state.snapshot.git_repositories.update(&id, |entry| {
                        entry.identity.repository_dir_abs_path = new_repository_dir;
                        entry.identity.common_dir_abs_path = new_common_dir;
                        entry.git_dir_scan_id = scan_id;
                        entry.info_exclude_needs_update = true;
                    });
                    affected_repo_roots.push(work_directory_abs_path);
                }
                Ok(Some(_)) => {}
                Ok(None) => {
                    if let Some(entry) = state.snapshot.git_repositories.remove(&id) {
                        affected_repo_roots.push(entry.identity.work_directory_abs_path);
                    }
                }
                Err(error) => log::debug!(
                    "preserving repository at {dot_git_abs_path:?} after transient resolve error: {error:#}"
                ),
            }
        }

        affected_repo_roots
    }

    async fn progress_timer(&self, running: bool) {
        if !running {
            return futures::future::pending().await;
        }

        #[cfg(feature = "test-support")]
        if self.fs.is_fake() {
            return self.executor.simulate_random_delay().await;
        }

        self.executor.timer(FS_WATCH_LATENCY).await
    }

    fn is_path_private(&self, path: &RelPath) -> bool {
        !self.share_private_files && self.settings.is_path_private(path)
    }

    fn should_scan_directory(&self, state: &BackgroundScannerState, entry: &Entry) -> bool {
        let scannable = state.scanning_enabled
            && (!entry.is_external
                || self.settings.scan_symlinks == settings::ScanSymlinksSetting::Always)
            && (!entry.is_ignored || entry.is_always_included);

        scannable
            || entry.path.file_name() == Some(DOT_GIT)
            || entry.path.file_name() == Some(local_settings_folder_name())
            || entry.path.file_name() == Some(local_vscode_folder_name())
            || state.scanned_dirs.contains(&entry.id) // If we've ever scanned it, keep scanning
            || state
                .paths_to_scan
                .iter()
                .any(|p| p.starts_with(&entry.path))
            || state
                .path_prefixes_to_scan
                .iter()
                .any(|p| entry.path.starts_with(p))
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
            if ancestor == paths::home_dir() {
                // Unless $HOME is itself the worktree root, don't consider it as a
                // containing git repository---expensive and likely unwanted.
                break;
            } else if let Ok(ignore) = build_gitignore(&ancestor.join(GITIGNORE), fs.as_ref()).await
            {
                ignores.insert(ancestor.into(), (ignore.into(), false));
            }
        }

        let ancestor_dot_git = ancestor.join(DOT_GIT);
        log::trace!("considering ancestor: {ancestor_dot_git:?}");
        // Check whether the directory or file called `.git` exists (in the
        // case of worktrees it's a file.)
        if fs
            .metadata(&ancestor_dot_git)
            .await
            .is_ok_and(|metadata| metadata.is_some())
        {
            let dot_git_abs_path = if index != 0 {
                // We canonicalize, since the FS events use the canonicalized path.
                match fs.canonicalize(&ancestor_dot_git).await.log_err() {
                    Some(path) => path,
                    None => continue,
                }
            } else {
                ancestor_dot_git.clone()
            };
            let dot_git_abs_path: Arc<Path> = dot_git_abs_path.as_path().into();

            // Keep walking up past an invalid `.git`. Exclude is loaded at registration.
            if discover_valid_git_repository(&dot_git_abs_path, fs.as_ref())
                .await
                .is_none()
            {
                continue;
            }

            if index != 0 {
                log::info!("inserting parent git repo for this worktree: {ancestor:?}");
                // We associate the external git repo with our root folder.
                return (
                    ignores,
                    Some((
                        dot_git_abs_path.as_ref().into(),
                        WorkDirectory::AboveProject {
                            absolute_path: ancestor.into(),
                        },
                    )),
                );
            }

            break;
        }
    }

    (ignores, None)
}

fn merge_event_roots(changed_paths: &[Arc<RelPath>], event_roots: &[EventRoot]) -> Vec<EventRoot> {
    let mut merged_event_roots = Vec::with_capacity(changed_paths.len() + event_roots.len());
    let mut changed_paths = changed_paths.iter().peekable();
    let mut event_roots = event_roots.iter().peekable();
    while let (Some(path), Some(event_root)) = (changed_paths.peek(), event_roots.peek()) {
        match path.cmp(&&event_root.path) {
            Ordering::Less => {
                merged_event_roots.push(EventRoot {
                    path: (*changed_paths.next().expect("peeked changed path")).clone(),
                    was_rescanned: false,
                });
            }
            Ordering::Equal => {
                merged_event_roots.push((*event_roots.next().expect("peeked event root")).clone());
                changed_paths.next();
            }
            Ordering::Greater => {
                merged_event_roots.push((*event_roots.next().expect("peeked event root")).clone());
            }
        }
    }
    merged_event_roots.extend(changed_paths.map(|path| EventRoot {
        path: path.clone(),
        was_rescanned: false,
    }));
    merged_event_roots.extend(event_roots.cloned());
    merged_event_roots
}

fn build_diff(
    phase: BackgroundScannerPhase,
    old_snapshot: &Snapshot,
    new_snapshot: &Snapshot,
    event_roots: &[EventRoot],
) -> UpdatedEntriesSet {
    use BackgroundScannerPhase::*;
    use PathChange::{Added, AddedOrUpdated, Loaded, Removed, Updated};

    // Identify which paths have changed. Use the known set of changed
    // parent paths to optimize the search.
    let mut changes = Vec::new();

    let mut old_paths = old_snapshot.entries_by_path.cursor::<PathKey>(());
    let mut new_paths = new_snapshot.entries_by_path.cursor::<PathKey>(());
    let mut last_newly_loaded_dir_path = None;
    old_paths.next();
    new_paths.next();
    for event_root in event_roots {
        let path = PathKey(event_root.path.clone());
        if old_paths.item().is_some_and(|e| e.path < path.0) {
            old_paths.seek_forward(&path, Bias::Left);
        }
        if new_paths.item().is_some_and(|e| e.path < path.0) {
            new_paths.seek_forward(&path, Bias::Left);
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
                            old_paths.next();
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
                            } else if event_root.was_rescanned {
                                changes.push((new_entry.path.clone(), new_entry.id, Updated));
                            }
                            old_paths.next();
                            new_paths.next();
                        }
                        Ordering::Greater => {
                            let is_newly_loaded = phase == InitialScan
                                || last_newly_loaded_dir_path
                                    .as_ref()
                                    .is_some_and(|dir| new_entry.path.starts_with(dir));
                            changes.push((
                                new_entry.path.clone(),
                                new_entry.id,
                                if is_newly_loaded { Loaded } else { Added },
                            ));
                            new_paths.next();
                        }
                    }
                }
                (Some(old_entry), None) => {
                    changes.push((old_entry.path.clone(), old_entry.id, Removed));
                    old_paths.next();
                }
                (None, Some(new_entry)) => {
                    let is_newly_loaded = phase == InitialScan
                        || last_newly_loaded_dir_path
                            .as_ref()
                            .is_some_and(|dir| new_entry.path.starts_with(dir));
                    changes.push((
                        new_entry.path.clone(),
                        new_entry.id,
                        if is_newly_loaded { Loaded } else { Added },
                    ));
                    new_paths.next();
                }
                (None, None) => break,
            }
        }
    }

    changes.into()
}

fn swap_to_front(child_paths: &mut Vec<PathBuf>, file: &str) {
    let position = child_paths
        .iter()
        .position(|path| path.file_name().unwrap() == file);
    if let Some(position) = position {
        let temp = child_paths.remove(position);
        child_paths.insert(0, temp);
    }
}

fn char_bag_for_path(root_char_bag: CharBag, path: &RelPath) -> CharBag {
    let mut result = root_char_bag;
    result.extend(path.as_unix_str().chars().map(|c| c.to_ascii_lowercase()));
    result
}

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<RelPath>,
    ignore_stack: IgnoreStack,
    scan_queue: Sender<ScanJob>,
    ancestor_inodes: TreeSet<u64>,
    is_external: bool,
}

struct UpdateIgnoreStatusJob {
    abs_path: Arc<Path>,
    ignore_stack: IgnoreStack,
    ignore_queue: Sender<UpdateIgnoreStatusJob>,
    scan_queue: Sender<ScanJob>,
}

pub trait WorktreeModelHandle {
    #[cfg(feature = "test-support")]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;

    #[cfg(feature = "test-support")]
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
    #[cfg(feature = "test-support")]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        let file_name = "fs-event-sentinel";

        let tree = self.clone();
        let (fs, root_path) = self.read_with(cx, |tree, _| {
            let tree = tree.as_local().unwrap();
            (tree.fs.clone(), tree.abs_path.clone())
        });

        async move {
            // Subscribe to events BEFORE creating the file to avoid race condition
            // where events fire before subscription is set up
            let mut events = cx.events(&tree);

            fs.create_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            // Check if condition is already met before waiting for events
            let file_exists = || {
                tree.read_with(cx, |tree, _| {
                    tree.entry_for_path(RelPath::from_unix_str(file_name).unwrap())
                        .is_some()
                })
            };

            // Use select to avoid blocking indefinitely if events are delayed
            while !file_exists() {
                futures::select_biased! {
                    _ = events.next() => {}
                    _ = futures::FutureExt::fuse(cx.background_executor.timer(std::time::Duration::from_millis(10))) => {}
                }
            }

            fs.remove_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            // Check if condition is already met before waiting for events
            let file_gone = || {
                tree.read_with(cx, |tree, _| {
                    tree.entry_for_path(RelPath::from_unix_str(file_name).unwrap())
                        .is_none()
                })
            };

            // Use select to avoid blocking indefinitely if events are delayed
            while !file_gone() {
                futures::select_biased! {
                    _ = events.next() => {}
                    _ = futures::FutureExt::fuse(cx.background_executor.timer(std::time::Duration::from_millis(10))) => {}
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
    #[cfg(feature = "test-support")]
    fn flush_fs_events_in_root_git_repository<'a>(
        &self,
        cx: &'a mut gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        let file_name = "fs-event-sentinel";

        let tree = self.clone();
        let (fs, root_path, mut git_dir_scan_id) = self.read_with(cx, |tree, _| {
            let tree = tree.as_local().unwrap();
            let local_repo_entry = tree
                .git_repositories
                .values()
                .min_by_key(|local_repo_entry| {
                    local_repo_entry.identity.work_directory_abs_path.clone()
                })
                .unwrap();
            (
                tree.fs.clone(),
                local_repo_entry.identity.common_dir_abs_path.clone(),
                local_repo_entry.git_dir_scan_id,
            )
        });

        let scan_id_increased = |tree: &mut Worktree, git_dir_scan_id: &mut usize| {
            let tree = tree.as_local().unwrap();
            // let repository = tree.repositories.first().unwrap();
            let local_repo_entry = tree
                .git_repositories
                .values()
                .min_by_key(|local_repo_entry| {
                    local_repo_entry.identity.work_directory_abs_path.clone()
                })
                .unwrap();

            if local_repo_entry.git_dir_scan_id > *git_dir_scan_id {
                *git_dir_scan_id = local_repo_entry.git_dir_scan_id;
                true
            } else {
                false
            }
        };

        async move {
            // Subscribe to events BEFORE creating the file to avoid race condition
            // where events fire before subscription is set up
            let mut events = cx.events(&tree);

            fs.create_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            // Use select to avoid blocking indefinitely if events are delayed
            while !tree.update(cx, |tree, _| scan_id_increased(tree, &mut git_dir_scan_id)) {
                futures::select_biased! {
                    _ = events.next() => {}
                    _ = futures::FutureExt::fuse(cx.background_executor.timer(std::time::Duration::from_millis(10))) => {}
                }
            }

            fs.remove_file(&root_path.join(file_name), Default::default())
                .await
                .unwrap();

            // Use select to avoid blocking indefinitely if events are delayed
            while !tree.update(cx, |tree, _| scan_id_increased(tree, &mut git_dir_scan_id)) {
                futures::select_biased! {
                    _ = events.next() => {}
                    _ = futures::FutureExt::fuse(cx.background_executor.timer(std::time::Duration::from_millis(10))) => {}
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
    max_path: &'a RelPath,
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
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a EntrySummary, _: ()) {
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
            max_path: RelPath::empty(),
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
    cursor: sum_tree::Cursor<'a, 'static, Entry, TraversalProgress<'a>>,
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
        start_path: &RelPath,
    ) -> Self {
        let mut cursor = snapshot.entries_by_path.cursor(());
        cursor.seek(&TraversalTarget::path(start_path), Bias::Left);
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
        )
    }

    pub fn advance_to_sibling(&mut self) -> bool {
        while let Some(entry) = self.cursor.item() {
            self.cursor
                .seek_forward(&TraversalTarget::successor(&entry.path), Bias::Left);
            if let Some(entry) = self.cursor.item()
                && (self.include_files || !entry.is_file())
                && (self.include_dirs || !entry.is_dir())
                && (self.include_ignored || !entry.is_ignored || entry.is_always_included)
            {
                return true;
            }
        }
        false
    }

    pub fn back_to_parent(&mut self) -> bool {
        let Some(parent_path) = self.cursor.item().and_then(|entry| entry.path.parent()) else {
            return false;
        };
        self.cursor
            .seek(&TraversalTarget::path(parent_path), Bias::Left)
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
            .end()
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
    Path(&'a RelPath),
    Successor(&'a RelPath),
}

impl PathTarget<'_> {
    fn cmp_path(&self, other: &RelPath) -> Ordering {
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
    fn cmp(&self, cursor_location: &PathProgress<'a>, _: S::Context<'_>) -> Ordering {
        self.cmp_path(cursor_location.max_path)
    }
}

impl<'a, S: Summary> SeekTarget<'a, PathSummary<S>, TraversalProgress<'a>> for PathTarget<'_> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: S::Context<'_>) -> Ordering {
        self.cmp_path(cursor_location.max_path)
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
    fn path(path: &'a RelPath) -> Self {
        Self::Path(PathTarget::Path(path))
    }

    fn successor(path: &'a RelPath) -> Self {
        Self::Path(PathTarget::Successor(path))
    }

    fn cmp_progress(&self, progress: &TraversalProgress) -> Ordering {
        match self {
            TraversalTarget::Path(path) => path.cmp_path(progress.max_path),
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
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: ()) -> Ordering {
        self.cmp_progress(cursor_location)
    }
}

impl<'a> SeekTarget<'a, PathSummary<sum_tree::NoSummary>, TraversalProgress<'a>>
    for TraversalTarget<'_>
{
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: ()) -> Ordering {
        self.cmp_progress(cursor_location)
    }
}

pub struct ChildEntriesOptions {
    pub include_files: bool,
    pub include_dirs: bool,
    pub include_ignored: bool,
}

pub struct ChildEntriesIter<'a> {
    parent_path: &'a RelPath,
    traversal: Traversal<'a>,
}

impl<'a> Iterator for ChildEntriesIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.traversal.entry()
            && item.path.starts_with(self.parent_path)
        {
            self.traversal.advance_to_sibling();
            return Some(item);
        }
        None
    }
}

impl<'a> From<&'a Entry> for proto::Entry {
    fn from(entry: &'a Entry) -> Self {
        Self {
            id: entry.id.to_proto(),
            is_dir: entry.is_dir(),
            path: entry.path.as_ref().as_unix_str().to_owned(),
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            is_ignored: entry.is_ignored,
            is_hidden: entry.is_hidden,
            is_external: entry.is_external,
            is_fifo: entry.is_fifo,
            size: Some(entry.size),
            canonical_path: entry
                .canonical_path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
        }
    }
}

impl TryFrom<(&CharBag, &PathMatcher, proto::Entry)> for Entry {
    type Error = anyhow::Error;

    fn try_from(
        (root_char_bag, always_included, entry): (&CharBag, &PathMatcher, proto::Entry),
    ) -> Result<Self> {
        let kind = if entry.is_dir {
            EntryKind::Dir
        } else {
            EntryKind::File
        };

        let path = RelPath::from_unix_str(&entry.path)
            .context("invalid relative path in proto message")?;
        let char_bag = char_bag_for_path(*root_char_bag, &path);
        let is_always_included = always_included.is_match(&path);
        Ok(Entry {
            id: ProjectEntryId::from_proto(entry.id),
            kind,
            path: path.into(),
            inode: entry.inode,
            mtime: entry.mtime.map(|time| time.into()),
            size: entry.size.unwrap_or(0),
            canonical_path: entry
                .canonical_path
                .map(|path_string| Arc::from(PathBuf::from(path_string))),
            is_ignored: entry.is_ignored,
            is_hidden: entry.is_hidden,
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

    pub fn to_proto(self) -> u64 {
        self.0 as u64
    }

    pub fn from_usize(id: usize) -> Self {
        ProjectEntryId(id)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

#[cfg(feature = "test-support")]
impl CreatedEntry {
    pub fn into_included(self) -> Option<Entry> {
        match self {
            CreatedEntry::Included(entry) => Some(entry),
            CreatedEntry::Excluded { .. } => None,
        }
    }
}

pub async fn discover_root_repo_common_dir(root_abs_path: &Path, fs: &dyn Fs) -> Option<Arc<Path>> {
    discover_root_repo_metadata(root_abs_path, fs)
        .await
        .map(|(common_dir, _)| common_dir)
}

async fn discover_root_repo_metadata(
    root_abs_path: &Path,
    fs: &dyn Fs,
) -> Option<(Arc<Path>, bool)> {
    let (repository_dir, common_dir) =
        discover_valid_git_repository(&root_abs_path.join(DOT_GIT), fs).await?;
    let is_linked_worktree = repository_dir != common_dir;
    Some((common_dir, is_linked_worktree))
}

/// Adapts [`fs::resolve_git_repository`] to the `Arc<Path>` identities discovery stores.
/// A transient I/O error is collapsed to `None` here (a later rescan retries) — unlike
/// event revalidation, which must tell a transient failure apart from a confirmed
/// removal and so calls the resolver directly.
async fn discover_valid_git_repository(
    dot_git_abs_path: &Path,
    fs: &dyn Fs,
) -> Option<(Arc<Path>, Arc<Path>)> {
    match fs::resolve_git_repository(dot_git_abs_path, fs).await {
        Ok(Some((repository_dir, common_dir))) => Some((repository_dir.into(), common_dir.into())),
        Ok(None) => None,
        Err(error) => {
            log::debug!("failed to resolve git repository at {dot_git_abs_path:?}: {error:#}");
            None
        }
    }
}

struct NullWatcher;

impl fs::Watcher for NullWatcher {
    fn add(&self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn remove(&self, _path: &Path) -> Result<()> {
        Ok(())
    }
}

async fn decode_file_text(
    fs: &dyn Fs,
    abs_path: &Path,
) -> Result<(String, &'static Encoding, bool)> {
    let mut file = fs
        .open_sync(&abs_path)
        .await
        .with_context(|| format!("opening file {abs_path:?}"))?;

    // First, read the beginning of the file to determine its kind and encoding.
    // We do not want to load an entire large blob into memory only to discard it.
    let mut file_first_bytes = Vec::with_capacity(FILE_ANALYSIS_BYTES);
    let mut buf = [0u8; FILE_ANALYSIS_BYTES];
    let mut reached_eof = false;
    loop {
        if file_first_bytes.len() >= FILE_ANALYSIS_BYTES {
            break;
        }
        let n = file
            .read(&mut buf)
            .with_context(|| format!("reading bytes of the file {abs_path:?}"))?;
        if n == 0 {
            reached_eof = true;
            break;
        }
        file_first_bytes.extend_from_slice(&buf[..n]);
    }
    let (bom_encoding, byte_content) = decode_byte_header(&file_first_bytes);
    anyhow::ensure!(
        byte_content != ByteContent::Binary,
        "Binary files are not supported"
    );

    // If the file is eligible for opening, read the rest of the file.
    let mut content = file_first_bytes;
    if !reached_eof {
        let mut buf = [0u8; 8 * 1024];
        loop {
            let n = file
                .read(&mut buf)
                .with_context(|| format!("reading remaining bytes of the file {abs_path:?}"))?;
            if n == 0 {
                break;
            }
            content.extend_from_slice(&buf[..n]);
        }
    }
    decode_byte_full(content, bom_encoding, byte_content)
}

fn decode_byte_header(prefix: &[u8]) -> (Option<&'static Encoding>, ByteContent) {
    if let Some((encoding, _bom_len)) = Encoding::for_bom(prefix) {
        return (Some(encoding), ByteContent::Unknown);
    }
    (None, analyze_byte_content(prefix))
}

fn decode_byte_full(
    bytes: Vec<u8>,
    bom_encoding: Option<&'static Encoding>,
    byte_content: ByteContent,
) -> Result<(String, &'static Encoding, bool)> {
    if let Some(encoding) = bom_encoding {
        let (cow, _) = encoding.decode_with_bom_removal(&bytes);
        return Ok((cow.into_owned(), encoding, true));
    }

    match byte_content {
        ByteContent::Utf16Le => {
            let encoding = encoding_rs::UTF_16LE;
            let (cow, _, _) = encoding.decode(&bytes);
            return Ok((cow.into_owned(), encoding, false));
        }
        ByteContent::Utf16Be => {
            let encoding = encoding_rs::UTF_16BE;
            let (cow, _, _) = encoding.decode(&bytes);
            return Ok((cow.into_owned(), encoding, false));
        }
        ByteContent::Binary => {
            anyhow::bail!("Binary files are not supported");
        }
        ByteContent::Unknown => {}
    }

    fn detect_encoding(bytes: Vec<u8>) -> (String, &'static Encoding) {
        let mut detector = EncodingDetector::new();
        detector.feed(&bytes, true);

        let encoding = detector.guess(None, true); // Use None for TLD hint to ensure neutral detection logic.

        let (cow, _, _) = encoding.decode(&bytes);
        (cow.into_owned(), encoding)
    }

    match String::from_utf8(bytes) {
        Ok(text) => {
            // ISO-2022-JP (and other ISO-2022 variants) consists entirely of 7-bit ASCII bytes,
            // so it is valid UTF-8. However, it contains escape sequences starting with '\x1b'.
            // If we find an escape character, we double-check the encoding to prevent
            // displaying raw escape sequences instead of the correct characters.
            if text.contains('\x1b') {
                let (s, enc) = detect_encoding(text.into_bytes());
                Ok((s, enc, false))
            } else {
                Ok((text, encoding_rs::UTF_8, false))
            }
        }
        Err(e) => {
            let (s, enc) = detect_encoding(e.into_bytes());
            Ok((s, enc, false))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// reproduction of issue #50785
    fn build_pcm16_wav_bytes() -> Vec<u8> {
        let header: Vec<u8> = vec![
            /*  RIFF header  */
            0x52, 0x49, 0x46, 0x46, // "RIFF"
            0xc6, 0xcf, 0x00, 0x00, // file size: 8
            0x57, 0x41, 0x56, 0x45, // "WAVE"
            /*  fmt chunk  */
            0x66, 0x6d, 0x74, 0x20, // "fmt "
            0x10, 0x00, 0x00, 0x00, // chunk size: 16
            0x01, 0x00, // format: PCM (1)
            0x01, 0x00, // channels: 1 (mono)
            0x80, 0x3e, 0x00, 0x00, // sample rate: 16000
            0x00, 0x7d, 0x00, 0x00, // byte rate: 32000
            0x02, 0x00, // block align: 2
            0x10, 0x00, // bits per sample: 16
            /*  LIST chunk  */
            0x4c, 0x49, 0x53, 0x54, // "LIST"
            0x1a, 0x00, 0x00, 0x00, // chunk size: 26
            0x49, 0x4e, 0x46, 0x4f, // "INFO"
            0x49, 0x53, 0x46, 0x54, // "ISFT"
            0x0d, 0x00, 0x00, 0x00, // sub-chunk size: 13
            0x4c, 0x61, 0x76, 0x66, 0x36, 0x32, 0x2e, 0x33, // "Lavf62.3"
            0x2e, 0x31, 0x30, 0x30, 0x00, // ".100\0"
            /* padding byte for word alignment */
            0x00, // data chunk header
            0x64, 0x61, 0x74, 0x61, // "data"
            0x80, 0xcf, 0x00, 0x00, // chunk size
        ];

        let mut bytes = header;

        // fill remaining space up to `FILE_ANALYSIS_BYTES` with synthetic PCM
        let audio_bytes_needed = FILE_ANALYSIS_BYTES - bytes.len();
        for i in 0..(audio_bytes_needed / 2) {
            let sample = (i & 0xFF) as u8;
            bytes.push(sample); // low byte: varies
            bytes.push(0x00); // high byte: zero for small values
        }

        bytes
    }

    #[test]
    fn test_pcm16_wav_detected_as_binary() {
        let wav_bytes = build_pcm16_wav_bytes();
        assert_eq!(wav_bytes.len(), FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&wav_bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "PCM 16-bit WAV should be detected as Binary via RIFF header"
        );
    }

    #[test]
    fn test_le16_binary_not_misdetected_as_utf16le() {
        let mut bytes = b"FAKE".to_vec();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            let sample = (bytes.len() & 0xFF) as u8;
            bytes.push(sample);
            bytes.push(0x00);
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "LE 16-bit binary with control characters should be detected as Binary"
        );
    }

    fn sample_local_snapshot(entries: Vec<(u64, LocalRepositoryEntry)>) -> LocalSnapshot {
        let mut snapshot = LocalSnapshot {
            snapshot: Snapshot::new(
                WorktreeId::from_proto(0),
                RelPath::empty_arc(),
                Path::new("/project").into(),
                PathStyle::local(),
            ),
            global_gitignore: None,
            ignores_by_parent_abs_path: Default::default(),
            git_repositories: Default::default(),
            root_file_handle: None,
            external_canonical_to_relative: Default::default(),
        };
        for (id, entry) in entries {
            snapshot
                .git_repositories
                .insert(ProjectEntryId::from_proto(id), entry);
        }
        snapshot
    }

    fn sample_repo_entry(
        work_dir: &str,
        dot_git: &str,
        repository_dir: &str,
        common_dir: &str,
        git_dir_scan_id: usize,
    ) -> LocalRepositoryEntry {
        LocalRepositoryEntry {
            identity: GitRepositoryIdentity {
                work_directory_abs_path: Path::new(work_dir).into(),
                dot_git_abs_path: Path::new(dot_git).into(),
                repository_dir_abs_path: Path::new(repository_dir).into(),
                common_dir_abs_path: Path::new(common_dir).into(),
            },
            git_dir_scan_id,
            info_exclude: Arc::new(Gitignore::empty()),
            info_exclude_needs_update: false,
        }
    }

    /// Unit transition table for info/exclude apply: Unparsable clears dirty and
    /// keeps last-good; Indeterminate keeps last-good and stays pending for retry.
    /// Present/Absent install and clear. Drives the real production helpers.
    #[test]
    fn test_repository_exclude_apply_transition_table() {
        let last_good = Arc::new(Gitignore::empty());
        // Distinct Arc so keep-last-good paths can be checked by pointer identity.
        let last_good_ptr = Arc::as_ptr(&last_good) as usize;

        // Present → install + clear dirty + recompute
        {
            let mut entry = sample_repo_entry("/p", "/p/.git", "/p/.git", "/p/.git", 0);
            entry.info_exclude = last_good.clone();
            entry.info_exclude_needs_update = true;
            let apply = RepositoryExcludeApply::Present(Arc::new(Gitignore::empty()));
            assert!(matches!(apply, RepositoryExcludeApply::Present(_)));
            let recompute = apply_repository_exclude_to_entry(&mut entry, apply);
            assert!(recompute, "Present must recompute ignore statuses");
            assert!(!entry.info_exclude_needs_update, "Present must clear dirty");
        }

        // Absent → install empty + clear dirty + recompute
        {
            let mut entry = sample_repo_entry("/p", "/p/.git", "/p/.git", "/p/.git", 0);
            entry.info_exclude = last_good.clone();
            entry.info_exclude_needs_update = true;
            let apply = RepositoryExcludeApply::Absent(Arc::new(Gitignore::empty()));
            assert!(matches!(apply, RepositoryExcludeApply::Absent(_)));
            let recompute = apply_repository_exclude_to_entry(&mut entry, apply);
            assert!(recompute, "Absent must recompute ignore statuses");
            assert!(!entry.info_exclude_needs_update, "Absent must clear dirty");
        }

        // Unparsable → keep last-good, clear dirty, no recompute
        {
            let mut entry = sample_repo_entry("/p", "/p/.git", "/p/.git", "/p/.git", 0);
            entry.info_exclude = last_good.clone();
            entry.info_exclude_needs_update = true;
            let apply = RepositoryExcludeApply::Unparsable;
            assert!(
                matches!(apply, RepositoryExcludeApply::Unparsable),
                "Unparsable must stay Unparsable, got {apply:?}"
            );
            let recompute = apply_repository_exclude_to_entry(&mut entry, apply);
            assert!(
                !recompute,
                "Unparsable must not recompute (matcher unchanged)"
            );
            assert!(
                !entry.info_exclude_needs_update,
                "Unparsable must clear dirty so later batches do not re-read forever"
            );
            assert_eq!(
                Arc::as_ptr(&entry.info_exclude) as usize,
                last_good_ptr,
                "Unparsable must keep last-good matcher"
            );
        }

        // Indeterminate → keep last-good, stay pending, no recompute
        {
            let mut entry = sample_repo_entry("/p", "/p/.git", "/p/.git", "/p/.git", 0);
            entry.info_exclude = last_good;
            entry.info_exclude_needs_update = true;
            let apply = RepositoryExcludeApply::Indeterminate;
            assert!(
                matches!(apply, RepositoryExcludeApply::Indeterminate),
                "Indeterminate must stay Indeterminate, got {apply:?}"
            );
            let recompute = apply_repository_exclude_to_entry(&mut entry, apply);
            assert!(
                !recompute,
                "Indeterminate must not recompute (matcher unchanged)"
            );
            assert!(
                entry.info_exclude_needs_update,
                "Indeterminate must leave dirty so a later batch retries"
            );
            assert_eq!(
                Arc::as_ptr(&entry.info_exclude) as usize,
                last_good_ptr,
                "Indeterminate must keep last-good matcher"
            );
        }
    }

    /// Re-registration must seed info_exclude from the previous entry so
    /// Unparsable/Indeterminate preserve a non-empty installed matcher (not an empty seed).
    #[test]
    fn test_reregistration_seeds_info_exclude_from_previous() {
        let root = Path::new("/p");
        let exclude_path = root.join(".git/info/exclude");
        let installed = Arc::new(
            parse_gitignore_contents(&exclude_path, root, "*.secret\n")
                .expect("sample exclude patterns must parse"),
        );
        assert!(
            installed
                .matched(root.join("foo.secret"), false)
                .is_ignore(),
            "fixture matcher must match *.secret"
        );
        let installed_ptr = Arc::as_ptr(&installed) as usize;
        let previous = {
            let mut entry = sample_repo_entry("/p", "/p/.git", "/p/.git", "/p/.git", 0);
            entry.info_exclude = installed;
            entry.info_exclude_needs_update = false;
            entry
        };

        // Unparsable re-registration: keep last-good, clear dirty.
        {
            let apply = RepositoryExcludeApply::Unparsable;
            let entry = local_repository_entry_for_registration(
                previous.identity.clone(),
                Some(previous.info_exclude.clone()),
                apply,
            );
            assert!(
                !entry.info_exclude_needs_update,
                "Unparsable re-registration must clear dirty"
            );
            assert_eq!(
                Arc::as_ptr(&entry.info_exclude) as usize,
                installed_ptr,
                "Unparsable re-registration must keep previous non-empty matcher"
            );
            assert!(
                entry
                    .info_exclude
                    .matched(root.join("foo.secret"), false)
                    .is_ignore(),
                "previous matcher must still match after Unparsable re-registration"
            );
        }

        // Indeterminate re-registration: keep last-good, stay pending.
        {
            let apply = RepositoryExcludeApply::Indeterminate;
            let entry = local_repository_entry_for_registration(
                previous.identity,
                Some(previous.info_exclude),
                apply,
            );
            assert!(
                entry.info_exclude_needs_update,
                "Indeterminate re-registration must leave dirty pending"
            );
            assert_eq!(
                Arc::as_ptr(&entry.info_exclude) as usize,
                installed_ptr,
                "Indeterminate re-registration must keep previous non-empty matcher"
            );
            assert!(
                entry
                    .info_exclude
                    .matched(root.join("foo.secret"), false)
                    .is_ignore(),
                "previous matcher must still match after Indeterminate re-registration"
            );
        }
    }

    #[test]
    fn test_changed_git_repositories_snapshot_diff_table() {
        let base = sample_repo_entry(
            "/linked",
            "/linked/.git",
            "/repo_a/.git/worktrees/feature",
            "/repo_a/.git",
            0,
        );

        // empty → AddedOrUpdated{identity_changed:false}
        {
            let old = sample_local_snapshot(vec![]);
            let new = sample_local_snapshot(vec![(1, base.clone())]);
            let changes = changed_git_repositories(&old, &new);
            assert_eq!(changes.len(), 1);
            match &changes[0] {
                GitRepositoryChange::AddedOrUpdated {
                    repository,
                    identity_changed: false,
                } => {
                    assert_eq!(repository.work_directory_id, ProjectEntryId::from_proto(1));
                    assert_eq!(repository.identity, base.identity);
                }
                other => panic!("expected AddedOrUpdated{{false}}, got {other:?}"),
            }
        }

        // scan-token-only → AddedOrUpdated{identity_changed:false}
        {
            let mut updated = base.clone();
            updated.git_dir_scan_id = 7;
            let old = sample_local_snapshot(vec![(1, base.clone())]);
            let new = sample_local_snapshot(vec![(1, updated)]);
            let changes = changed_git_repositories(&old, &new);
            assert_eq!(changes.len(), 1);
            match &changes[0] {
                GitRepositoryChange::AddedOrUpdated {
                    identity_changed: false,
                    repository,
                } => {
                    assert_eq!(repository.identity, base.identity);
                }
                other => panic!("expected AddedOrUpdated{{false}}, got {other:?}"),
            }
        }

        // each of the 4 identity paths individually → AddedOrUpdated{true}
        for (field, altered) in [
            ("work_directory_abs_path", {
                let mut e = base.clone();
                e.identity.work_directory_abs_path = Path::new("/other").into();
                e
            }),
            ("dot_git_abs_path", {
                let mut e = base.clone();
                e.identity.dot_git_abs_path = Path::new("/other/.git").into();
                e
            }),
            ("repository_dir_abs_path", {
                let mut e = base.clone();
                e.identity.repository_dir_abs_path =
                    Path::new("/repo_b/.git/worktrees/feature").into();
                e
            }),
            ("common_dir_abs_path", {
                let mut e = base.clone();
                e.identity.common_dir_abs_path = Path::new("/repo_b/.git").into();
                e
            }),
        ] {
            let old = sample_local_snapshot(vec![(1, base.clone())]);
            let new = sample_local_snapshot(vec![(1, altered.clone())]);
            let changes = changed_git_repositories(&old, &new);
            assert_eq!(changes.len(), 1, "field {field}");
            match &changes[0] {
                GitRepositoryChange::AddedOrUpdated {
                    identity_changed: true,
                    repository,
                } => {
                    assert_eq!(repository.identity, altered.identity, "field {field}");
                }
                other => panic!("field {field}: expected AddedOrUpdated{{true}}, got {other:?}"),
            }
        }

        // unchanged → no event
        {
            let old = sample_local_snapshot(vec![(1, base.clone())]);
            let new = sample_local_snapshot(vec![(1, base.clone())]);
            let changes = changed_git_repositories(&old, &new);
            assert!(changes.is_empty());
        }

        // removal carries complete old identity
        {
            let old = sample_local_snapshot(vec![(1, base.clone())]);
            let new = sample_local_snapshot(vec![]);
            let changes = changed_git_repositories(&old, &new);
            assert_eq!(changes.len(), 1);
            match &changes[0] {
                GitRepositoryChange::Removed { repository } => {
                    assert_eq!(repository.identity, base.identity);
                    assert_eq!(repository.work_directory_id, ProjectEntryId::from_proto(1));
                }
                other => panic!("expected Removed, got {other:?}"),
            }
        }

        // keys [1,3,5] → [3] forces BOTH removal branches — identical Removed payloads
        {
            let entry1 = sample_repo_entry("/a", "/a/.git", "/a/.git", "/a/.git", 0);
            let entry3 = sample_repo_entry("/b", "/b/.git", "/b/.git", "/b/.git", 0);
            let entry5 = sample_repo_entry("/c", "/c/.git", "/c/.git", "/c/.git", 0);
            let old = sample_local_snapshot(vec![
                (1, entry1.clone()),
                (3, entry3.clone()),
                (5, entry5.clone()),
            ]);
            let new = sample_local_snapshot(vec![(3, entry3)]);
            let changes = changed_git_repositories(&old, &new);
            // Both merge-removal arms (Greater mid-walk + old-only tail) emit
            // identical Removed payloads carrying the complete old identity.
            assert_eq!(
                changes.as_ref(),
                &[
                    GitRepositoryChange::Removed {
                        repository: GitRepositoryRegistration {
                            work_directory_id: ProjectEntryId::from_proto(1),
                            identity: entry1.identity,
                        },
                    },
                    GitRepositoryChange::Removed {
                        repository: GitRepositoryRegistration {
                            work_directory_id: ProjectEntryId::from_proto(5),
                            identity: entry5.identity,
                        },
                    },
                ]
            );
        }
    }

    #[test]
    fn test_be16_binary_not_misdetected_as_utf16be() {
        let mut bytes = b"FAKE".to_vec();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.push(0x00);
            let sample = (bytes.len() & 0xFF) as u8;
            bytes.push(sample);
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        let result = analyze_byte_content(&bytes);
        assert_eq!(
            result,
            ByteContent::Binary,
            "BE 16-bit binary with control characters should be detected as Binary"
        );
    }

    #[test]
    fn test_utf16le_text_detected_as_utf16le() {
        let text = "Hello, world! This is a UTF-16 test string. ";
        let mut bytes = Vec::new();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.extend(text.encode_utf16().flat_map(|u| u.to_le_bytes()));
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        assert_eq!(analyze_byte_content(&bytes), ByteContent::Utf16Le);
    }

    #[test]
    fn test_utf16be_text_detected_as_utf16be() {
        let text = "Hello, world! This is a UTF-16 test string. ";
        let mut bytes = Vec::new();
        while bytes.len() < FILE_ANALYSIS_BYTES {
            bytes.extend(text.encode_utf16().flat_map(|u| u.to_be_bytes()));
        }
        bytes.truncate(FILE_ANALYSIS_BYTES);

        assert_eq!(analyze_byte_content(&bytes), ByteContent::Utf16Be);
    }

    #[test]
    fn test_known_binary_headers() {
        let cases: &[(&[u8], &str)] = &[
            (b"RIFF\x00\x00\x00\x00WAVE", "WAV"),
            (b"RIFF\x00\x00\x00\x00AVI ", "AVI"),
            (b"OggS\x00\x02", "OGG"),
            (b"fLaC\x00\x00", "FLAC"),
            (b"ID3\x03\x00", "MP3 ID3v2"),
            (b"\xFF\xFB\x90\x00", "MP3 MPEG1 Layer3"),
            (b"\xFF\xF3\x90\x00", "MP3 MPEG2 Layer3"),
        ];

        for (header, label) in cases {
            let mut bytes = header.to_vec();
            bytes.resize(FILE_ANALYSIS_BYTES, 0x41); // pad with 'A'
            assert_eq!(
                analyze_byte_content(&bytes),
                ByteContent::Binary,
                "{label} should be detected as Binary"
            );
        }
    }
}
