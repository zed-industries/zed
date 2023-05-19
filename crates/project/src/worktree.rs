use crate::{
    copy_recursive, ignore::IgnoreStack, DiagnosticSummary, ProjectEntryId, RemoveOptions,
};
use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{anyhow, Context, Result};
use client::{proto, Client};
use clock::ReplicaId;
use collections::{HashMap, VecDeque};
use fs::{
    repository::{GitFileStatus, GitRepository, RepoPath, RepoPathDescendants},
    Fs, LineEnding,
};
use futures::{
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    select_biased,
    task::Poll,
    Stream, StreamExt,
};
use fuzzy::CharBag;
use git::{DOT_GIT, GITIGNORE};
use gpui::{executor, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use language::{
    proto::{
        deserialize_fingerprint, deserialize_version, serialize_fingerprint, serialize_line_ending,
        serialize_version,
    },
    Buffer, DiagnosticEntry, File as _, PointUtf16, Rope, RopeFingerprint, Unclipped,
};
use lsp::LanguageServerId;
use parking_lot::Mutex;
use postage::{
    barrier,
    prelude::{Sink as _, Stream as _},
    watch,
};
use smol::channel::{self, Sender};
use std::{
    any::Any,
    cmp::{self, Ordering},
    convert::TryFrom,
    ffi::OsStr,
    fmt,
    future::Future,
    mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, SystemTime},
};
use sum_tree::{Bias, Edit, SeekTarget, SumTree, TreeMap, TreeSet};
use util::{paths::HOME, ResultExt, TakeUntilExt, TryFutureExt};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct WorktreeId(usize);

pub enum Worktree {
    Local(LocalWorktree),
    Remote(RemoteWorktree),
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    path_changes_tx: channel::Sender<(Vec<PathBuf>, barrier::Sender)>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    _background_scanner_task: Task<()>,
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
    visible: bool,
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

impl Snapshot {
    pub fn repo_for(&self, path: &Path) -> Option<RepositoryEntry> {
        let mut max_len = 0;
        let mut current_candidate = None;
        for (work_directory, repo) in (&self.repository_entries).iter() {
            if path.starts_with(&work_directory.0) {
                if work_directory.0.as_os_str().len() >= max_len {
                    current_candidate = Some(repo);
                    max_len = work_directory.0.as_os_str().len();
                } else {
                    break;
                }
            }
        }

        current_candidate.map(|entry| entry.to_owned())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryEntry {
    pub(crate) work_directory: WorkDirectoryEntry,
    pub(crate) branch: Option<Arc<str>>,
    pub(crate) statuses: TreeMap<RepoPath, GitFileStatus>,
}

fn read_git_status(git_status: i32) -> Option<GitFileStatus> {
    proto::GitStatus::from_i32(git_status).map(|status| match status {
        proto::GitStatus::Added => GitFileStatus::Added,
        proto::GitStatus::Modified => GitFileStatus::Modified,
        proto::GitStatus::Conflict => GitFileStatus::Conflict,
    })
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

    pub fn status_for_file(&self, snapshot: &Snapshot, path: &Path) -> Option<GitFileStatus> {
        self.work_directory
            .relativize(snapshot, path)
            .and_then(|repo_path| self.statuses.get(&repo_path))
            .cloned()
    }

    pub fn status_for_path(&self, snapshot: &Snapshot, path: &Path) -> Option<GitFileStatus> {
        self.work_directory
            .relativize(snapshot, path)
            .and_then(|repo_path| {
                self.statuses
                    .iter_from(&repo_path)
                    .take_while(|(key, _)| key.starts_with(&repo_path))
                    // Short circut once we've found the highest level
                    .take_until(|(_, status)| status == &&GitFileStatus::Conflict)
                    .map(|(_, status)| status)
                    .reduce(
                        |status_first, status_second| match (status_first, status_second) {
                            (GitFileStatus::Conflict, _) | (_, GitFileStatus::Conflict) => {
                                &GitFileStatus::Conflict
                            }
                            (GitFileStatus::Modified, _) | (_, GitFileStatus::Modified) => {
                                &GitFileStatus::Modified
                            }
                            _ => &GitFileStatus::Added,
                        },
                    )
                    .copied()
            })
    }

    pub fn build_update(&self, other: &Self) -> proto::RepositoryEntry {
        let mut updated_statuses: Vec<proto::StatusEntry> = Vec::new();
        let mut removed_statuses: Vec<String> = Vec::new();

        let mut self_statuses = self.statuses.iter().peekable();
        let mut other_statuses = other.statuses.iter().peekable();
        loop {
            match (self_statuses.peek(), other_statuses.peek()) {
                (Some((self_repo_path, self_status)), Some((other_repo_path, other_status))) => {
                    match Ord::cmp(self_repo_path, other_repo_path) {
                        Ordering::Less => {
                            updated_statuses.push(make_status_entry(self_repo_path, self_status));
                            self_statuses.next();
                        }
                        Ordering::Equal => {
                            if self_status != other_status {
                                updated_statuses
                                    .push(make_status_entry(self_repo_path, self_status));
                            }

                            self_statuses.next();
                            other_statuses.next();
                        }
                        Ordering::Greater => {
                            removed_statuses.push(make_repo_path(other_repo_path));
                            other_statuses.next();
                        }
                    }
                }
                (Some((self_repo_path, self_status)), None) => {
                    updated_statuses.push(make_status_entry(self_repo_path, self_status));
                    self_statuses.next();
                }
                (None, Some((other_repo_path, _))) => {
                    removed_statuses.push(make_repo_path(other_repo_path));
                    other_statuses.next();
                }
                (None, None) => break,
            }
        }

        proto::RepositoryEntry {
            work_directory_id: self.work_directory_id().to_proto(),
            branch: self.branch.as_ref().map(|str| str.to_string()),
            removed_repo_paths: removed_statuses,
            updated_statuses: updated_statuses,
        }
    }
}

fn make_repo_path(path: &RepoPath) -> String {
    path.as_os_str().to_string_lossy().to_string()
}

fn make_status_entry(path: &RepoPath, status: &GitFileStatus) -> proto::StatusEntry {
    proto::StatusEntry {
        repo_path: make_repo_path(path),
        status: match status {
            GitFileStatus::Added => proto::GitStatus::Added.into(),
            GitFileStatus::Modified => proto::GitStatus::Modified.into(),
            GitFileStatus::Conflict => proto::GitStatus::Conflict.into(),
        },
    }
}

impl From<&RepositoryEntry> for proto::RepositoryEntry {
    fn from(value: &RepositoryEntry) -> Self {
        proto::RepositoryEntry {
            work_directory_id: value.work_directory.to_proto(),
            branch: value.branch.as_ref().map(|str| str.to_string()),
            updated_statuses: value
                .statuses
                .iter()
                .map(|(repo_path, status)| make_status_entry(repo_path, status))
                .collect(),
            removed_repo_paths: Default::default(),
        }
    }
}

/// This path corresponds to the 'content path' (the folder that contains the .git)
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct RepositoryWorkDirectory(Arc<Path>);

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
    pub(crate) fn relativize(&self, worktree: &Snapshot, path: &Path) -> Option<RepoPath> {
        worktree.entry_for_id(self.0).and_then(|entry| {
            path.strip_prefix(&entry.path)
                .ok()
                .map(move |path| path.into())
        })
    }
}

impl Deref for WorkDirectoryEntry {
    type Target = ProjectEntryId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<ProjectEntryId> for WorkDirectoryEntry {
    fn from(value: ProjectEntryId) -> Self {
        WorkDirectoryEntry(value)
    }
}

#[derive(Debug, Clone)]
pub struct LocalSnapshot {
    ignores_by_parent_abs_path: HashMap<Arc<Path>, (Arc<Gitignore>, bool)>, // (gitignore, needs_update)
    // The ProjectEntryId corresponds to the entry for the .git dir
    // work_directory_id
    git_repositories: TreeMap<ProjectEntryId, LocalRepositoryEntry>,
    removed_entry_ids: HashMap<u64, ProjectEntryId>,
    next_entry_id: Arc<AtomicUsize>,
    snapshot: Snapshot,
}

#[derive(Debug, Clone)]
pub struct LocalRepositoryEntry {
    pub(crate) scan_id: usize,
    pub(crate) full_scan_id: usize,
    pub(crate) repo_ptr: Arc<Mutex<dyn GitRepository>>,
    /// Path to the actual .git folder.
    /// Note: if .git is a file, this points to the folder indicated by the .git file
    pub(crate) git_dir_path: Arc<Path>,
}

impl LocalRepositoryEntry {
    // Note that this path should be relative to the worktree root.
    pub(crate) fn in_dot_git(&self, path: &Path) -> bool {
        path.starts_with(self.git_dir_path.as_ref())
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
        changes: HashMap<(Arc<Path>, ProjectEntryId), PathChange>,
        barrier: Option<barrier::Sender>,
        scanning: bool,
    },
}

struct ShareState {
    project_id: u64,
    snapshots_tx: watch::Sender<LocalSnapshot>,
    resume_updates: watch::Sender<()>,
    _maintain_remote_snapshot: Task<Option<()>>,
}

pub enum Event {
    UpdatedEntries(HashMap<(Arc<Path>, ProjectEntryId), PathChange>),
    UpdatedGitRepositories(HashMap<Arc<Path>, LocalRepositoryEntry>),
}

impl Entity for Worktree {
    type Event = Event;
}

impl Worktree {
    pub async fn local(
        client: Arc<Client>,
        path: impl Into<Arc<Path>>,
        visible: bool,
        fs: Arc<dyn Fs>,
        next_entry_id: Arc<AtomicUsize>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let abs_path = path.into();
        let metadata = fs
            .metadata(&abs_path)
            .await
            .context("failed to stat worktree path")?;

        Ok(cx.add_model(move |cx: &mut ModelContext<Worktree>| {
            let root_name = abs_path
                .file_name()
                .map_or(String::new(), |f| f.to_string_lossy().to_string());

            let mut snapshot = LocalSnapshot {
                ignores_by_parent_abs_path: Default::default(),
                removed_entry_ids: Default::default(),
                git_repositories: Default::default(),
                next_entry_id,
                snapshot: Snapshot {
                    id: WorktreeId::from_usize(cx.model_id()),
                    abs_path: abs_path.clone(),
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
                        &snapshot.next_entry_id,
                        snapshot.root_char_bag,
                    ),
                    fs.as_ref(),
                );
            }

            let (path_changes_tx, path_changes_rx) = channel::unbounded();
            let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded();

            cx.spawn_weak(|this, mut cx| async move {
                while let Some((state, this)) = scan_states_rx.next().await.zip(this.upgrade(&cx)) {
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
                                this.set_snapshot(snapshot, cx);
                                cx.emit(Event::UpdatedEntries(changes));
                                drop(barrier);
                            }
                        }
                        cx.notify();
                    });
                }
            })
            .detach();

            let background_scanner_task = cx.background().spawn({
                let fs = fs.clone();
                let snapshot = snapshot.clone();
                let background = cx.background().clone();
                async move {
                    let events = fs.watch(&abs_path, Duration::from_millis(100)).await;
                    BackgroundScanner::new(
                        snapshot,
                        fs,
                        scan_states_tx,
                        background,
                        path_changes_rx,
                    )
                    .run(events)
                    .await;
                }
            });

            Worktree::Local(LocalWorktree {
                snapshot,
                is_scanning: watch::channel_with(true),
                share: None,
                path_changes_tx,
                _background_scanner_task: background_scanner_task,
                diagnostics: Default::default(),
                diagnostic_summaries: Default::default(),
                client,
                fs,
                visible,
            })
        }))
    }

    pub fn remote(
        project_remote_id: u64,
        replica_id: ReplicaId,
        worktree: proto::WorktreeMetadata,
        client: Arc<Client>,
        cx: &mut AppContext,
    ) -> ModelHandle<Self> {
        cx.add_model(|cx: &mut ModelContext<Self>| {
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

            cx.background()
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

            cx.spawn_weak(|this, mut cx| async move {
                while (snapshot_updated_rx.recv().await).is_some() {
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| {
                            let this = this.as_remote_mut().unwrap();
                            this.snapshot = this.background_snapshot.lock().clone();
                            cx.emit(Event::UpdatedEntries(Default::default()));
                            cx.notify();
                            while let Some((scan_id, _)) = this.snapshot_subscriptions.front() {
                                if this.observed_snapshot(*scan_id) {
                                    let (_, tx) = this.snapshot_subscriptions.pop_front().unwrap();
                                    let _ = tx.send(());
                                } else {
                                    break;
                                }
                            }
                        });
                    } else {
                        break;
                    }
                }
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
}

impl LocalWorktree {
    pub fn contains_abs_path(&self, path: &Path) -> bool {
        path.starts_with(&self.abs_path)
    }

    fn absolutize(&self, path: &Path) -> PathBuf {
        if path.file_name().is_some() {
            self.abs_path.join(path)
        } else {
            self.abs_path.to_path_buf()
        }
    }

    pub(crate) fn load_buffer(
        &mut self,
        id: u64,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let path = Arc::from(path);
        cx.spawn(move |this, mut cx| async move {
            let (file, contents, diff_base) = this
                .update(&mut cx, |t, cx| t.as_local().unwrap().load(&path, cx))
                .await?;
            let text_buffer = cx
                .background()
                .spawn(async move { text::Buffer::new(0, id, contents) })
                .await;
            Ok(cx.add_model(|cx| {
                let mut buffer = Buffer::build(text_buffer, diff_base, Some(Arc::new(file)));
                buffer.git_diff_recalc(cx);
                buffer
            }))
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

    fn set_snapshot(&mut self, new_snapshot: LocalSnapshot, cx: &mut ModelContext<Worktree>) {
        let updated_repos =
            self.changed_repos(&self.git_repositories, &new_snapshot.git_repositories);
        self.snapshot = new_snapshot;

        if let Some(share) = self.share.as_mut() {
            *share.snapshots_tx.borrow_mut() = self.snapshot.clone();
        }

        if !updated_repos.is_empty() {
            cx.emit(Event::UpdatedGitRepositories(updated_repos));
        }
    }

    fn changed_repos(
        &self,
        old_repos: &TreeMap<ProjectEntryId, LocalRepositoryEntry>,
        new_repos: &TreeMap<ProjectEntryId, LocalRepositoryEntry>,
    ) -> HashMap<Arc<Path>, LocalRepositoryEntry> {
        let mut diff = HashMap::default();
        let mut old_repos = old_repos.iter().peekable();
        let mut new_repos = new_repos.iter().peekable();
        loop {
            match (old_repos.peek(), new_repos.peek()) {
                (Some((old_entry_id, old_repo)), Some((new_entry_id, new_repo))) => {
                    match Ord::cmp(old_entry_id, new_entry_id) {
                        Ordering::Less => {
                            if let Some(entry) = self.entry_for_id(**old_entry_id) {
                                diff.insert(entry.path.clone(), (*old_repo).clone());
                            }
                            old_repos.next();
                        }
                        Ordering::Equal => {
                            if old_repo.scan_id != new_repo.scan_id {
                                if let Some(entry) = self.entry_for_id(**new_entry_id) {
                                    diff.insert(entry.path.clone(), (*new_repo).clone());
                                }
                            }

                            old_repos.next();
                            new_repos.next();
                        }
                        Ordering::Greater => {
                            if let Some(entry) = self.entry_for_id(**new_entry_id) {
                                diff.insert(entry.path.clone(), (*new_repo).clone());
                            }
                            new_repos.next();
                        }
                    }
                }
                (Some((old_entry_id, old_repo)), None) => {
                    if let Some(entry) = self.entry_for_id(**old_entry_id) {
                        diff.insert(entry.path.clone(), (*old_repo).clone());
                    }
                    old_repos.next();
                }
                (None, Some((new_entry_id, new_repo))) => {
                    if let Some(entry) = self.entry_for_id(**new_entry_id) {
                        diff.insert(entry.path.clone(), (*new_repo).clone());
                    }
                    new_repos.next();
                }
                (None, None) => break,
            }
        }
        diff
    }

    pub fn scan_complete(&self) -> impl Future<Output = ()> {
        let mut is_scanning_rx = self.is_scanning.1.clone();
        async move {
            let mut is_scanning = is_scanning_rx.borrow().clone();
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
        let handle = cx.handle();
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let snapshot = self.snapshot();

        let mut index_task = None;

        if let Some(repo) = snapshot.repo_for(&path) {
            let repo_path = repo.work_directory.relativize(self, &path).unwrap();
            if let Some(repo) = self.git_repositories.get(&*repo.work_directory) {
                let repo = repo.repo_ptr.to_owned();
                index_task = Some(
                    cx.background()
                        .spawn(async move { repo.lock().load_index_text(&repo_path) }),
                );
            }
        }

        cx.spawn(|this, mut cx| async move {
            let text = fs.load(&abs_path).await?;

            let diff_base = if let Some(index_task) = index_task {
                index_task.await
            } else {
                None
            };

            // Eagerly populate the snapshot with an updated entry for the loaded file
            let entry = this
                .update(&mut cx, |this, cx| {
                    this.as_local().unwrap().refresh_entry(path, None, cx)
                })
                .await?;

            Ok((
                File {
                    entry_id: entry.id,
                    worktree: handle,
                    path: entry.path,
                    mtime: entry.mtime,
                    is_local: true,
                    is_deleted: false,
                },
                text,
                diff_base,
            ))
        })
    }

    pub fn save_buffer(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        path: Arc<Path>,
        has_changed_file: bool,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<(clock::Global, RopeFingerprint, SystemTime)>> {
        let handle = cx.handle();
        let buffer = buffer_handle.read(cx);

        let rpc = self.client.clone();
        let buffer_id = buffer.remote_id();
        let project_id = self.share.as_ref().map(|share| share.project_id);

        let text = buffer.as_rope().clone();
        let fingerprint = text.fingerprint();
        let version = buffer.version();
        let save = self.write_file(path, text, buffer.line_ending(), cx);

        cx.as_mut().spawn(|mut cx| async move {
            let entry = save.await?;

            if has_changed_file {
                let new_file = Arc::new(File {
                    entry_id: entry.id,
                    worktree: handle,
                    path: entry.path,
                    mtime: entry.mtime,
                    is_local: true,
                    is_deleted: false,
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
                        buffer.file_updated(new_file, cx).detach();
                    }
                });
            }

            if let Some(project_id) = project_id {
                rpc.send(proto::BufferSaved {
                    project_id,
                    buffer_id,
                    version: serialize_version(&version),
                    mtime: Some(entry.mtime.into()),
                    fingerprint: serialize_fingerprint(fingerprint),
                })?;
            }

            buffer_handle.update(&mut cx, |buffer, cx| {
                buffer.did_save(version.clone(), fingerprint, entry.mtime, cx);
            });

            Ok((version, fingerprint, entry.mtime))
        })
    }

    pub fn create_entry(
        &self,
        path: impl Into<Arc<Path>>,
        is_dir: bool,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let path = path.into();
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let write = cx.background().spawn(async move {
            if is_dir {
                fs.create_dir(&abs_path).await
            } else {
                fs.save(&abs_path, &Default::default(), Default::default())
                    .await
            }
        });

        cx.spawn(|this, mut cx| async move {
            write.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut().unwrap().refresh_entry(path, None, cx)
            })
            .await
        })
    }

    pub fn write_file(
        &self,
        path: impl Into<Arc<Path>>,
        text: Rope,
        line_ending: LineEnding,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let path = path.into();
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        let write = cx
            .background()
            .spawn(async move { fs.save(&abs_path, &text, line_ending).await });

        cx.spawn(|this, mut cx| async move {
            write.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut().unwrap().refresh_entry(path, None, cx)
            })
            .await
        })
    }

    pub fn delete_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<()>>> {
        let entry = self.entry_for_id(entry_id)?.clone();
        let abs_path = self.abs_path.clone();
        let fs = self.fs.clone();

        let delete = cx.background().spawn(async move {
            let mut abs_path = fs.canonicalize(&abs_path).await?;
            if entry.path.file_name().is_some() {
                abs_path = abs_path.join(&entry.path);
            }
            if entry.is_file() {
                fs.remove_file(&abs_path, Default::default()).await?;
            } else {
                fs.remove_dir(
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: false,
                    },
                )
                .await?;
            }
            anyhow::Ok(abs_path)
        });

        Some(cx.spawn(|this, mut cx| async move {
            let abs_path = delete.await?;
            let (tx, mut rx) = barrier::channel();
            this.update(&mut cx, |this, _| {
                this.as_local_mut()
                    .unwrap()
                    .path_changes_tx
                    .try_send((vec![abs_path], tx))
            })?;
            rx.recv().await;
            Ok(())
        }))
    }

    pub fn rename_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<Entry>>> {
        let old_path = self.entry_for_id(entry_id)?.path.clone();
        let new_path = new_path.into();
        let abs_old_path = self.absolutize(&old_path);
        let abs_new_path = self.absolutize(&new_path);
        let fs = self.fs.clone();
        let rename = cx.background().spawn(async move {
            fs.rename(&abs_old_path, &abs_new_path, Default::default())
                .await
        });

        Some(cx.spawn(|this, mut cx| async move {
            rename.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entry(new_path.clone(), Some(old_path), cx)
            })
            .await
        }))
    }

    pub fn copy_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Task<Result<Entry>>> {
        let old_path = self.entry_for_id(entry_id)?.path.clone();
        let new_path = new_path.into();
        let abs_old_path = self.absolutize(&old_path);
        let abs_new_path = self.absolutize(&new_path);
        let fs = self.fs.clone();
        let copy = cx.background().spawn(async move {
            copy_recursive(
                fs.as_ref(),
                &abs_old_path,
                &abs_new_path,
                Default::default(),
            )
            .await
        });

        Some(cx.spawn(|this, mut cx| async move {
            copy.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .refresh_entry(new_path.clone(), None, cx)
            })
            .await
        }))
    }

    fn refresh_entry(
        &self,
        path: Arc<Path>,
        old_path: Option<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let fs = self.fs.clone();
        let abs_root_path = self.abs_path.clone();
        let path_changes_tx = self.path_changes_tx.clone();
        cx.spawn_weak(move |this, mut cx| async move {
            let abs_path = fs.canonicalize(&abs_root_path).await?;
            let mut paths = Vec::with_capacity(2);
            paths.push(if path.file_name().is_some() {
                abs_path.join(&path)
            } else {
                abs_path.clone()
            });
            if let Some(old_path) = old_path {
                paths.push(if old_path.file_name().is_some() {
                    abs_path.join(&old_path)
                } else {
                    abs_path.clone()
                });
            }

            let (tx, mut rx) = barrier::channel();
            path_changes_tx.try_send((paths, tx))?;
            rx.recv().await;
            this.upgrade(&cx)
                .ok_or_else(|| anyhow!("worktree was dropped"))?
                .update(&mut cx, |this, _| {
                    this.entry_for_path(path)
                        .cloned()
                        .ok_or_else(|| anyhow!("failed to read path after update"))
                })
        })
    }

    pub fn share(&mut self, project_id: u64, cx: &mut ModelContext<Worktree>) -> Task<Result<()>> {
        let (share_tx, share_rx) = oneshot::channel();

        if let Some(share) = self.share.as_mut() {
            let _ = share_tx.send(());
            *share.resume_updates.borrow_mut() = ();
        } else {
            let (snapshots_tx, mut snapshots_rx) = watch::channel_with(self.snapshot());
            let (resume_updates_tx, mut resume_updates_rx) = watch::channel();
            let worktree_id = cx.model_id() as u64;

            for (path, summaries) in &self.diagnostic_summaries {
                for (&server_id, summary) in summaries {
                    if let Err(e) = self.client.send(proto::UpdateDiagnosticSummary {
                        project_id,
                        worktree_id,
                        summary: Some(summary.to_proto(server_id, &path)),
                    }) {
                        return Task::ready(Err(e));
                    }
                }
            }

            let _maintain_remote_snapshot = cx.background().spawn({
                let client = self.client.clone();
                async move {
                    let mut share_tx = Some(share_tx);
                    let mut prev_snapshot = LocalSnapshot {
                        ignores_by_parent_abs_path: Default::default(),
                        removed_entry_ids: Default::default(),
                        next_entry_id: Default::default(),
                        git_repositories: Default::default(),
                        snapshot: Snapshot {
                            id: WorktreeId(worktree_id as usize),
                            abs_path: Path::new("").into(),
                            root_name: Default::default(),
                            root_char_bag: Default::default(),
                            entries_by_path: Default::default(),
                            entries_by_id: Default::default(),
                            repository_entries: Default::default(),
                            scan_id: 0,
                            completed_scan_id: 0,
                        },
                    };
                    while let Some(snapshot) = snapshots_rx.recv().await {
                        #[cfg(any(test, feature = "test-support"))]
                        const MAX_CHUNK_SIZE: usize = 2;
                        #[cfg(not(any(test, feature = "test-support")))]
                        const MAX_CHUNK_SIZE: usize = 256;

                        let update =
                            snapshot.build_update(&prev_snapshot, project_id, worktree_id, true);
                        for update in proto::split_worktree_update(update, MAX_CHUNK_SIZE) {
                            let _ = resume_updates_rx.try_recv();
                            while let Err(error) = client.request(update.clone()).await {
                                log::error!("failed to send worktree update: {}", error);
                                log::info!("waiting to resume updates");
                                if resume_updates_rx.next().await.is_none() {
                                    return Ok(());
                                }
                            }
                        }

                        if let Some(share_tx) = share_tx.take() {
                            let _ = share_tx.send(());
                        }

                        prev_snapshot = snapshot;
                    }

                    Ok::<_, anyhow::Error>(())
                }
                .log_err()
            });

            self.share = Some(ShareState {
                project_id,
                snapshots_tx,
                resume_updates: resume_updates_tx,
                _maintain_remote_snapshot,
            });
        }

        cx.foreground()
            .spawn(async move { share_rx.await.map_err(|_| anyhow!("share ended")) })
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
        buffer_handle: ModelHandle<Buffer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<(clock::Global, RopeFingerprint, SystemTime)>> {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();
        let version = buffer.version();
        let rpc = self.client.clone();
        let project_id = self.project_id;
        cx.as_mut().spawn(|mut cx| async move {
            let response = rpc
                .request(proto::SaveBuffer {
                    project_id,
                    buffer_id,
                    version: serialize_version(&version),
                })
                .await?;
            let version = deserialize_version(&response.version);
            let fingerprint = deserialize_fingerprint(&response.fingerprint)?;
            let mtime = response
                .mtime
                .ok_or_else(|| anyhow!("missing mtime"))?
                .into();

            buffer_handle.update(&mut cx, |buffer, cx| {
                buffer.did_save(version.clone(), fingerprint, mtime, cx);
            });

            Ok((version, fingerprint, mtime))
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

    fn wait_for_snapshot(&mut self, scan_id: usize) -> impl Future<Output = Result<()>> {
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
            })
        })
    }

    pub(crate) fn delete_entry(
        &mut self,
        id: ProjectEntryId,
        scan_id: usize,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        let wait_for_snapshot = self.wait_for_snapshot(scan_id);
        cx.spawn(|this, mut cx| async move {
            wait_for_snapshot.await?;
            this.update(&mut cx, |worktree, _| {
                let worktree = worktree.as_remote_mut().unwrap();
                let mut snapshot = worktree.background_snapshot.lock();
                snapshot.delete_entry(id);
                worktree.snapshot = snapshot.clone();
            });
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

    pub fn contains_entry(&self, entry_id: ProjectEntryId) -> bool {
        self.entries_by_id.get(&entry_id, &()).is_some()
    }

    pub(crate) fn insert_entry(&mut self, entry: proto::Entry) -> Result<Entry> {
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
            let mut cursor = self.entries_by_path.cursor();
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
            new_entries_by_path.push_tree(cursor.suffix(&()), &());
            new_entries_by_path
        };

        Some(removed_entry.path)
    }

    pub(crate) fn apply_remote_update(&mut self, mut update: proto::UpdateWorktree) -> Result<()> {
        let mut entries_by_path_edits = Vec::new();
        let mut entries_by_id_edits = Vec::new();
        for entry_id in update.removed_entries {
            if let Some(entry) = self.entry_for_id(ProjectEntryId::from_proto(entry_id)) {
                entries_by_path_edits.push(Edit::Remove(PathKey(entry.path.clone())));
                entries_by_id_edits.push(Edit::Remove(entry.id));
            }
        }

        for entry in update.updated_entries {
            let entry = Entry::try_from((&self.root_char_bag, entry))?;
            if let Some(PathEntry { path, .. }) = self.entries_by_id.get(&entry.id, &()) {
                entries_by_path_edits.push(Edit::Remove(PathKey(path.clone())));
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
                let mut statuses = TreeMap::default();
                for status_entry in repository.updated_statuses {
                    let Some(git_file_status) = read_git_status(status_entry.status) else {
                        continue;
                    };

                    let repo_path = RepoPath::new(status_entry.repo_path.into());
                    statuses.insert(repo_path, git_file_status);
                }

                let work_directory = RepositoryWorkDirectory(entry.path.clone());
                if self.repository_entries.get(&work_directory).is_some() {
                    self.repository_entries.update(&work_directory, |repo| {
                        repo.branch = repository.branch.map(Into::into);
                        repo.statuses.insert_tree(statuses);

                        for repo_path in repository.removed_repo_paths {
                            let repo_path = RepoPath::new(repo_path.into());
                            repo.statuses.remove(&repo_path);
                        }
                    });
                } else {
                    self.repository_entries.insert(
                        work_directory,
                        RepositoryEntry {
                            work_directory: work_directory_entry,
                            branch: repository.branch.map(Into::into),
                            statuses,
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
        self.entries_by_path.summary().visible_file_count
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

    /// Given an ordered iterator of entries, returns an iterator of those entries,
    /// along with their containing git repository.
    pub fn entries_with_repos<'a>(
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

    fn descendent_entries<'a>(
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
    pub(crate) fn get_local_repo(&self, repo: &RepositoryEntry) -> Option<&LocalRepositoryEntry> {
        self.git_repositories.get(&repo.work_directory.0)
    }

    pub(crate) fn repo_for_metadata(
        &self,
        path: &Path,
    ) -> Option<(&ProjectEntryId, &LocalRepositoryEntry)> {
        self.git_repositories
            .iter()
            .find(|(_, repo)| repo.in_dot_git(path))
    }

    #[cfg(test)]
    pub(crate) fn build_initial_update(&self, project_id: u64) -> proto::UpdateWorktree {
        let root_name = self.root_name.clone();
        proto::UpdateWorktree {
            project_id,
            worktree_id: self.id().to_proto(),
            abs_path: self.abs_path().to_string_lossy().into(),
            root_name,
            updated_entries: self.entries_by_path.iter().map(Into::into).collect(),
            removed_entries: Default::default(),
            scan_id: self.scan_id as u64,
            is_last_update: true,
            updated_repositories: self.repository_entries.values().map(Into::into).collect(),
            removed_repositories: Default::default(),
        }
    }

    pub(crate) fn build_update(
        &self,
        other: &Self,
        project_id: u64,
        worktree_id: u64,
        include_ignored: bool,
    ) -> proto::UpdateWorktree {
        let mut updated_entries = Vec::new();
        let mut removed_entries = Vec::new();
        let mut self_entries = self
            .entries_by_id
            .cursor::<()>()
            .filter(|e| include_ignored || !e.is_ignored)
            .peekable();
        let mut other_entries = other
            .entries_by_id
            .cursor::<()>()
            .filter(|e| include_ignored || !e.is_ignored)
            .peekable();
        loop {
            match (self_entries.peek(), other_entries.peek()) {
                (Some(self_entry), Some(other_entry)) => {
                    match Ord::cmp(&self_entry.id, &other_entry.id) {
                        Ordering::Less => {
                            let entry = self.entry_for_id(self_entry.id).unwrap().into();
                            updated_entries.push(entry);
                            self_entries.next();
                        }
                        Ordering::Equal => {
                            if self_entry.scan_id != other_entry.scan_id {
                                let entry = self.entry_for_id(self_entry.id).unwrap().into();
                                updated_entries.push(entry);
                            }

                            self_entries.next();
                            other_entries.next();
                        }
                        Ordering::Greater => {
                            removed_entries.push(other_entry.id.to_proto());
                            other_entries.next();
                        }
                    }
                }
                (Some(self_entry), None) => {
                    let entry = self.entry_for_id(self_entry.id).unwrap().into();
                    updated_entries.push(entry);
                    self_entries.next();
                }
                (None, Some(other_entry)) => {
                    removed_entries.push(other_entry.id.to_proto());
                    other_entries.next();
                }
                (None, None) => break,
            }
        }

        let mut updated_repositories: Vec<proto::RepositoryEntry> = Vec::new();
        let mut removed_repositories = Vec::new();
        let mut self_repos = self.snapshot.repository_entries.iter().peekable();
        let mut other_repos = other.snapshot.repository_entries.iter().peekable();
        loop {
            match (self_repos.peek(), other_repos.peek()) {
                (Some((self_work_dir, self_repo)), Some((other_work_dir, other_repo))) => {
                    match Ord::cmp(self_work_dir, other_work_dir) {
                        Ordering::Less => {
                            updated_repositories.push((*self_repo).into());
                            self_repos.next();
                        }
                        Ordering::Equal => {
                            if self_repo != other_repo {
                                updated_repositories.push(self_repo.build_update(other_repo));
                            }

                            self_repos.next();
                            other_repos.next();
                        }
                        Ordering::Greater => {
                            removed_repositories.push(other_repo.work_directory.to_proto());
                            other_repos.next();
                        }
                    }
                }
                (Some((_, self_repo)), None) => {
                    updated_repositories.push((*self_repo).into());
                    self_repos.next();
                }
                (None, Some((_, other_repo))) => {
                    removed_repositories.push(other_repo.work_directory.to_proto());
                    other_repos.next();
                }
                (None, None) => break,
            }
        }

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

        self.reuse_entry_id(&mut entry);

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

    fn populate_dir(
        &mut self,
        parent_path: Arc<Path>,
        entries: impl IntoIterator<Item = Entry>,
        ignore: Option<Arc<Gitignore>>,
        fs: &dyn Fs,
    ) {
        let mut parent_entry = if let Some(parent_entry) =
            self.entries_by_path.get(&PathKey(parent_path.clone()), &())
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
            EntryKind::PendingDir => {
                parent_entry.kind = EntryKind::Dir;
            }
            EntryKind::Dir => {}
            _ => return,
        }

        if let Some(ignore) = ignore {
            self.ignores_by_parent_abs_path
                .insert(self.abs_path.join(&parent_path).into(), (ignore, false));
        }

        if parent_path.file_name() == Some(&DOT_GIT) {
            self.build_repo(parent_path, fs);
        }

        let mut entries_by_path_edits = vec![Edit::Insert(parent_entry)];
        let mut entries_by_id_edits = Vec::new();

        for mut entry in entries {
            self.reuse_entry_id(&mut entry);
            entries_by_id_edits.push(Edit::Insert(PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: self.scan_id,
            }));
            entries_by_path_edits.push(Edit::Insert(entry));
        }

        self.entries_by_path.edit(entries_by_path_edits, &());
        self.entries_by_id.edit(entries_by_id_edits, &());
    }

    fn build_repo(&mut self, parent_path: Arc<Path>, fs: &dyn Fs) -> Option<()> {
        let abs_path = self.abs_path.join(&parent_path);
        let work_dir: Arc<Path> = parent_path.parent().unwrap().into();

        // Guard against repositories inside the repository metadata
        if work_dir
            .components()
            .find(|component| component.as_os_str() == *DOT_GIT)
            .is_some()
        {
            return None;
        };

        let work_dir_id = self
            .entry_for_path(work_dir.clone())
            .map(|entry| entry.id)?;

        if self.git_repositories.get(&work_dir_id).is_none() {
            let repo = fs.open_repo(abs_path.as_path())?;
            let work_directory = RepositoryWorkDirectory(work_dir.clone());
            let scan_id = self.scan_id;

            let repo_lock = repo.lock();

            self.repository_entries.insert(
                work_directory,
                RepositoryEntry {
                    work_directory: work_dir_id.into(),
                    branch: repo_lock.branch_name().map(Into::into),
                    statuses: repo_lock.statuses().unwrap_or_default(),
                },
            );
            drop(repo_lock);

            self.git_repositories.insert(
                work_dir_id,
                LocalRepositoryEntry {
                    scan_id,
                    full_scan_id: scan_id,
                    repo_ptr: repo,
                    git_dir_path: parent_path.clone(),
                },
            )
        }

        Some(())
    }
    fn reuse_entry_id(&mut self, entry: &mut Entry) {
        if let Some(removed_entry_id) = self.removed_entry_ids.remove(&entry.inode) {
            entry.id = removed_entry_id;
        } else if let Some(existing_entry) = self.entry_for_path(&entry.path) {
            entry.id = existing_entry.id;
        }
    }

    fn remove_path(&mut self, path: &Path) {
        let mut new_entries;
        let removed_entries;
        {
            let mut cursor = self.entries_by_path.cursor::<TraversalProgress>();
            new_entries = cursor.slice(&TraversalTarget::Path(path), Bias::Left, &());
            removed_entries = cursor.slice(&TraversalTarget::PathSuccessor(path), Bias::Left, &());
            new_entries.push_tree(cursor.suffix(&()), &());
        }
        self.entries_by_path = new_entries;

        let mut entries_by_id_edits = Vec::new();
        for entry in removed_entries.cursor::<()>() {
            let removed_entry_id = self
                .removed_entry_ids
                .entry(entry.inode)
                .or_insert(entry.id);
            *removed_entry_id = cmp::max(*removed_entry_id, entry.id);
            entries_by_id_edits.push(Edit::Remove(entry.id));
        }
        self.entries_by_id.edit(entries_by_id_edits, &());

        if path.file_name() == Some(&GITIGNORE) {
            let abs_parent_path = self.abs_path.join(path.parent().unwrap());
            if let Some((_, needs_update)) = self
                .ignores_by_parent_abs_path
                .get_mut(abs_parent_path.as_path())
            {
                *needs_update = true;
            }
        }
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
        for ancestor in abs_path.ancestors().skip(1) {
            if let Some((ignore, _)) = self.ignores_by_parent_abs_path.get(ancestor) {
                new_ignores.push((ancestor, Some(ignore.clone())));
            } else {
                new_ignores.push((ancestor, None));
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

    pub(crate) fn from_proto(id: u64) -> Self {
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
    pub worktree: ModelHandle<Worktree>,
    pub path: Arc<Path>,
    pub mtime: SystemTime,
    pub(crate) entry_id: ProjectEntryId,
    pub(crate) is_local: bool,
    pub(crate) is_deleted: bool,
}

impl language::File for File {
    fn as_local(&self) -> Option<&dyn language::LocalFile> {
        if self.is_local {
            Some(self)
        } else {
            None
        }
    }

    fn mtime(&self) -> SystemTime {
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

    fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_proto(&self) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: self.worktree.id() as u64,
            entry_id: self.entry_id.to_proto(),
            path: self.path.to_string_lossy().into(),
            mtime: Some(self.mtime.into()),
            is_deleted: self.is_deleted,
        }
    }
}

impl language::LocalFile for File {
    fn abs_path(&self, cx: &AppContext) -> PathBuf {
        self.worktree
            .read(cx)
            .as_local()
            .unwrap()
            .abs_path
            .join(&self.path)
    }

    fn load(&self, cx: &AppContext) -> Task<Result<String>> {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        cx.background()
            .spawn(async move { fs.load(&abs_path).await })
    }

    fn buffer_reloaded(
        &self,
        buffer_id: u64,
        version: &clock::Global,
        fingerprint: RopeFingerprint,
        line_ending: LineEnding,
        mtime: SystemTime,
        cx: &mut AppContext,
    ) {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        if let Some(project_id) = worktree.share.as_ref().map(|share| share.project_id) {
            worktree
                .client
                .send(proto::BufferReloaded {
                    project_id,
                    buffer_id,
                    version: serialize_version(version),
                    mtime: Some(mtime.into()),
                    fingerprint: serialize_fingerprint(fingerprint),
                    line_ending: serialize_line_ending(line_ending) as i32,
                })
                .log_err();
        }
    }
}

impl File {
    pub fn from_proto(
        proto: rpc::proto::File,
        worktree: ModelHandle<Worktree>,
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
            mtime: proto.mtime.ok_or_else(|| anyhow!("no timestamp"))?.into(),
            entry_id: ProjectEntryId::from_proto(proto.entry_id),
            is_local: false,
            is_deleted: proto.is_deleted,
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
            Some(self.entry_id)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: ProjectEntryId,
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub inode: u64,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    pub is_ignored: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    PendingDir,
    Dir,
    File(CharBag),
}

#[derive(Clone, Copy, Debug)]
pub enum PathChange {
    Added,
    Removed,
    Updated,
    AddedOrUpdated,
}

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
            mtime: metadata.mtime,
            is_symlink: metadata.is_symlink,
            is_ignored: false,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryKind::Dir | EntryKind::PendingDir)
    }

    pub fn is_file(&self) -> bool {
        matches!(self.kind, EntryKind::File(_))
    }
}

impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self) -> Self::Summary {
        let visible_count = if self.is_ignored { 0 } else { 1 };
        let file_count;
        let visible_file_count;
        if self.is_file() {
            file_count = 1;
            visible_file_count = visible_count;
        } else {
            file_count = 0;
            visible_file_count = 0;
        }

        EntrySummary {
            max_path: self.path.clone(),
            count: 1,
            visible_count,
            file_count,
            visible_file_count,
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
    visible_count: usize,
    file_count: usize,
    visible_file_count: usize,
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(Path::new("")),
            count: 0,
            visible_count: 0,
            file_count: 0,
            visible_file_count: 0,
        }
    }
}

impl sum_tree::Summary for EntrySummary {
    type Context = ();

    fn add_summary(&mut self, rhs: &Self, _: &()) {
        self.max_path = rhs.max_path.clone();
        self.count += rhs.count;
        self.visible_count += rhs.visible_count;
        self.file_count += rhs.file_count;
        self.visible_file_count += rhs.visible_file_count;
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
    snapshot: Mutex<LocalSnapshot>,
    fs: Arc<dyn Fs>,
    status_updates_tx: UnboundedSender<ScanState>,
    executor: Arc<executor::Background>,
    refresh_requests_rx: channel::Receiver<(Vec<PathBuf>, barrier::Sender)>,
    prev_state: Mutex<BackgroundScannerState>,
    finished_initial_scan: bool,
}

struct BackgroundScannerState {
    snapshot: Snapshot,
    event_paths: Vec<Arc<Path>>,
}

impl BackgroundScanner {
    fn new(
        snapshot: LocalSnapshot,
        fs: Arc<dyn Fs>,
        status_updates_tx: UnboundedSender<ScanState>,
        executor: Arc<executor::Background>,
        refresh_requests_rx: channel::Receiver<(Vec<PathBuf>, barrier::Sender)>,
    ) -> Self {
        Self {
            fs,
            status_updates_tx,
            executor,
            refresh_requests_rx,
            prev_state: Mutex::new(BackgroundScannerState {
                snapshot: snapshot.snapshot.clone(),
                event_paths: Default::default(),
            }),
            snapshot: Mutex::new(snapshot),
            finished_initial_scan: false,
        }
    }

    async fn run(
        &mut self,
        mut events_rx: Pin<Box<dyn Send + Stream<Item = Vec<fsevent::Event>>>>,
    ) {
        use futures::FutureExt as _;

        let (root_abs_path, root_inode) = {
            let snapshot = self.snapshot.lock();
            (
                snapshot.abs_path.clone(),
                snapshot.root_entry().map(|e| e.inode),
            )
        };

        // Populate ignores above the root.
        let ignore_stack;
        for ancestor in root_abs_path.ancestors().skip(1) {
            if let Ok(ignore) = build_gitignore(&ancestor.join(&*GITIGNORE), self.fs.as_ref()).await
            {
                self.snapshot
                    .lock()
                    .ignores_by_parent_abs_path
                    .insert(ancestor.into(), (ignore.into(), false));
            }
        }
        {
            let mut snapshot = self.snapshot.lock();
            snapshot.scan_id += 1;
            ignore_stack = snapshot.ignore_stack_for_abs_path(&root_abs_path, true);
            if ignore_stack.is_all() {
                if let Some(mut root_entry) = snapshot.root_entry().cloned() {
                    root_entry.is_ignored = true;
                    snapshot.insert_entry(root_entry, self.fs.as_ref());
                }
            }
        };

        // Perform an initial scan of the directory.
        let (scan_job_tx, scan_job_rx) = channel::unbounded();
        smol::block_on(scan_job_tx.send(ScanJob {
            abs_path: root_abs_path,
            path: Arc::from(Path::new("")),
            ignore_stack,
            ancestor_inodes: TreeSet::from_ordered_entries(root_inode),
            scan_queue: scan_job_tx.clone(),
        }))
        .unwrap();
        drop(scan_job_tx);
        self.scan_dirs(true, scan_job_rx).await;
        {
            let mut snapshot = self.snapshot.lock();
            snapshot.completed_scan_id = snapshot.scan_id;
        }
        self.send_status_update(false, None);

        // Process any any FS events that occurred while performing the initial scan.
        // For these events, update events cannot be as precise, because we didn't
        // have the previous state loaded yet.
        if let Poll::Ready(Some(events)) = futures::poll!(events_rx.next()) {
            let mut paths = events.into_iter().map(|e| e.path).collect::<Vec<_>>();
            while let Poll::Ready(Some(more_events)) = futures::poll!(events_rx.next()) {
                paths.extend(more_events.into_iter().map(|e| e.path));
            }
            self.process_events(paths).await;
        }

        self.finished_initial_scan = true;

        // Continue processing events until the worktree is dropped.
        loop {
            select_biased! {
                // Process any path refresh requests from the worktree. Prioritize
                // these before handling changes reported by the filesystem.
                request = self.refresh_requests_rx.recv().fuse() => {
                    let Ok((paths, barrier)) = request else { break };
                    if !self.process_refresh_request(paths.clone(), barrier).await {
                        return;
                    }
                }

                events = events_rx.next().fuse() => {
                    let Some(events) = events else { break };
                    let mut paths = events.into_iter().map(|e| e.path).collect::<Vec<_>>();
                    while let Poll::Ready(Some(more_events)) = futures::poll!(events_rx.next()) {
                        paths.extend(more_events.into_iter().map(|e| e.path));
                    }
                    self.process_events(paths.clone()).await;
                }
            }
        }
    }

    async fn process_refresh_request(&self, paths: Vec<PathBuf>, barrier: barrier::Sender) -> bool {
        if let Some(mut paths) = self.reload_entries_for_paths(paths, None).await {
            paths.sort_unstable();
            util::extend_sorted(
                &mut self.prev_state.lock().event_paths,
                paths,
                usize::MAX,
                Ord::cmp,
            );
        }
        self.send_status_update(false, Some(barrier))
    }

    async fn process_events(&mut self, paths: Vec<PathBuf>) {
        let (scan_job_tx, scan_job_rx) = channel::unbounded();
        let paths = self
            .reload_entries_for_paths(paths, Some(scan_job_tx.clone()))
            .await;
        if let Some(paths) = &paths {
            util::extend_sorted(
                &mut self.prev_state.lock().event_paths,
                paths.iter().cloned(),
                usize::MAX,
                Ord::cmp,
            );
        }
        drop(scan_job_tx);
        self.scan_dirs(false, scan_job_rx).await;

        self.update_ignore_statuses().await;

        let mut snapshot = self.snapshot.lock();

        if let Some(paths) = paths {
            for path in paths {
                self.reload_repo_for_file_path(&path, &mut *snapshot, self.fs.as_ref());
            }
        }

        let mut git_repositories = mem::take(&mut snapshot.git_repositories);
        git_repositories.retain(|work_directory_id, _| {
            snapshot
                .entry_for_id(*work_directory_id)
                .map_or(false, |entry| {
                    snapshot.entry_for_path(entry.path.join(*DOT_GIT)).is_some()
                })
        });
        snapshot.git_repositories = git_repositories;

        let mut git_repository_entries = mem::take(&mut snapshot.snapshot.repository_entries);
        git_repository_entries.retain(|_, entry| {
            snapshot
                .git_repositories
                .get(&entry.work_directory.0)
                .is_some()
        });
        snapshot.snapshot.repository_entries = git_repository_entries;

        snapshot.removed_entry_ids.clear();
        snapshot.completed_scan_id = snapshot.scan_id;

        drop(snapshot);

        self.send_status_update(false, None);
        self.prev_state.lock().event_paths.clear();
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
                                request = self.refresh_requests_rx.recv().fuse() => {
                                    let Ok((paths, barrier)) = request else { break };
                                    if !self.process_refresh_request(paths, barrier).await {
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
        let mut prev_state = self.prev_state.lock();
        let new_snapshot = self.snapshot.lock().clone();
        let old_snapshot = mem::replace(&mut prev_state.snapshot, new_snapshot.snapshot.clone());

        let changes = self.build_change_set(
            &old_snapshot,
            &new_snapshot.snapshot,
            &prev_state.event_paths,
        );

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
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();
        let mut ignore_stack = job.ignore_stack.clone();
        let mut new_ignore = None;
        let (root_abs_path, root_char_bag, next_entry_id) = {
            let snapshot = self.snapshot.lock();
            (
                snapshot.abs_path().clone(),
                snapshot.root_char_bag,
                snapshot.next_entry_id.clone(),
            )
        };
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
            let child_metadata = match self.fs.metadata(&child_abs_path).await {
                Ok(Some(metadata)) => metadata,
                Ok(None) => continue,
                Err(err) => {
                    log::error!("error processing {:?}: {:?}", child_abs_path, err);
                    continue;
                }
            };

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
                        if let Some(job) = new_jobs.next().expect("Missing scan job for entry") {
                            job.ignore_stack = if entry.is_ignored {
                                IgnoreStack::all()
                            } else {
                                ignore_stack.clone()
                            };
                        }
                    }
                }
            }

            let mut child_entry = Entry::new(
                child_path.clone(),
                &child_metadata,
                &next_entry_id,
                root_char_bag,
            );

            if child_entry.is_dir() {
                let is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, true);
                child_entry.is_ignored = is_ignored;

                // Avoid recursing until crash in the case of a recursive symlink
                if !job.ancestor_inodes.contains(&child_entry.inode) {
                    let mut ancestor_inodes = job.ancestor_inodes.clone();
                    ancestor_inodes.insert(child_entry.inode);

                    new_jobs.push(Some(ScanJob {
                        abs_path: child_abs_path,
                        path: child_path,
                        ignore_stack: if is_ignored {
                            IgnoreStack::all()
                        } else {
                            ignore_stack.clone()
                        },
                        ancestor_inodes,
                        scan_queue: job.scan_queue.clone(),
                    }));
                } else {
                    new_jobs.push(None);
                }
            } else {
                child_entry.is_ignored = ignore_stack.is_abs_path_ignored(&child_abs_path, false);
            }

            new_entries.push(child_entry);
        }

        self.snapshot.lock().populate_dir(
            job.path.clone(),
            new_entries,
            new_ignore,
            self.fs.as_ref(),
        );

        for new_job in new_jobs {
            if let Some(new_job) = new_job {
                job.scan_queue.send(new_job).await.unwrap();
            }
        }

        Ok(())
    }

    async fn reload_entries_for_paths(
        &self,
        mut abs_paths: Vec<PathBuf>,
        scan_queue_tx: Option<Sender<ScanJob>>,
    ) -> Option<Vec<Arc<Path>>> {
        let doing_recursive_update = scan_queue_tx.is_some();

        abs_paths.sort_unstable();
        abs_paths.dedup_by(|a, b| a.starts_with(&b));

        let root_abs_path = self.snapshot.lock().abs_path.clone();
        let root_canonical_path = self.fs.canonicalize(&root_abs_path).await.log_err()?;
        let metadata = futures::future::join_all(
            abs_paths
                .iter()
                .map(|abs_path| self.fs.metadata(&abs_path))
                .collect::<Vec<_>>(),
        )
        .await;

        let mut snapshot = self.snapshot.lock();
        let is_idle = snapshot.completed_scan_id == snapshot.scan_id;
        snapshot.scan_id += 1;
        if is_idle && !doing_recursive_update {
            snapshot.completed_scan_id = snapshot.scan_id;
        }

        // Remove any entries for paths that no longer exist or are being recursively
        // refreshed. Do this before adding any new entries, so that renames can be
        // detected regardless of the order of the paths.
        let mut event_paths = Vec::<Arc<Path>>::with_capacity(abs_paths.len());
        for (abs_path, metadata) in abs_paths.iter().zip(metadata.iter()) {
            if let Ok(path) = abs_path.strip_prefix(&root_canonical_path) {
                if matches!(metadata, Ok(None)) || doing_recursive_update {
                    snapshot.remove_path(path);
                }
                event_paths.push(path.into());
            } else {
                log::error!(
                    "unexpected event {:?} for root path {:?}",
                    abs_path,
                    root_canonical_path
                );
            }
        }

        for (path, metadata) in event_paths.iter().cloned().zip(metadata.into_iter()) {
            let abs_path: Arc<Path> = root_abs_path.join(&path).into();

            match metadata {
                Ok(Some(metadata)) => {
                    let ignore_stack =
                        snapshot.ignore_stack_for_abs_path(&abs_path, metadata.is_dir);
                    let mut fs_entry = Entry::new(
                        path.clone(),
                        &metadata,
                        snapshot.next_entry_id.as_ref(),
                        snapshot.root_char_bag,
                    );
                    fs_entry.is_ignored = ignore_stack.is_all();
                    snapshot.insert_entry(fs_entry, self.fs.as_ref());

                    if let Some(scan_queue_tx) = &scan_queue_tx {
                        let mut ancestor_inodes = snapshot.ancestor_inodes_for_path(&path);
                        if metadata.is_dir && !ancestor_inodes.contains(&metadata.inode) {
                            ancestor_inodes.insert(metadata.inode);
                            smol::block_on(scan_queue_tx.send(ScanJob {
                                abs_path,
                                path,
                                ignore_stack,
                                ancestor_inodes,
                                scan_queue: scan_queue_tx.clone(),
                            }))
                            .unwrap();
                        }
                    }
                }
                Ok(None) => {
                    self.remove_repo_path(&path, &mut snapshot);
                }
                Err(err) => {
                    // TODO - create a special 'error' entry in the entries tree to mark this
                    log::error!("error reading file on event {:?}", err);
                }
            }
        }

        Some(event_paths)
    }

    fn remove_repo_path(&self, path: &Path, snapshot: &mut LocalSnapshot) -> Option<()> {
        if !path
            .components()
            .any(|component| component.as_os_str() == *DOT_GIT)
        {
            let scan_id = snapshot.scan_id;
            let repo = snapshot.repo_for(&path)?;

            let repo_path = repo.work_directory.relativize(&snapshot, &path)?;

            let work_dir = repo.work_directory(snapshot)?;
            let work_dir_id = repo.work_directory;

            snapshot
                .git_repositories
                .update(&work_dir_id, |entry| entry.scan_id = scan_id);

            snapshot.repository_entries.update(&work_dir, |entry| {
                entry
                    .statuses
                    .remove_range(&repo_path, &RepoPathDescendants(&repo_path))
            });
        }

        Some(())
    }

    fn reload_repo_for_file_path(
        &self,
        path: &Path,
        snapshot: &mut LocalSnapshot,
        fs: &dyn Fs,
    ) -> Option<()> {
        let scan_id = snapshot.scan_id;

        if path
            .components()
            .any(|component| component.as_os_str() == *DOT_GIT)
        {
            let (entry_id, repo_ptr) = {
                let Some((entry_id, repo)) = snapshot.repo_for_metadata(&path) else {
                    let dot_git_dir = path.ancestors()
                    .skip_while(|ancestor| ancestor.file_name() != Some(&*DOT_GIT))
                    .next()?;

                    snapshot.build_repo(dot_git_dir.into(), fs);
                    return None;
                };
                if repo.full_scan_id == scan_id {
                    return None;
                }
                (*entry_id, repo.repo_ptr.to_owned())
            };

            let work_dir = snapshot
                .entry_for_id(entry_id)
                .map(|entry| RepositoryWorkDirectory(entry.path.clone()))?;

            let repo = repo_ptr.lock();
            repo.reload_index();
            let branch = repo.branch_name();
            let statuses = repo.statuses().unwrap_or_default();

            snapshot.git_repositories.update(&entry_id, |entry| {
                entry.scan_id = scan_id;
                entry.full_scan_id = scan_id;
            });

            snapshot.repository_entries.update(&work_dir, |entry| {
                entry.branch = branch.map(Into::into);
                entry.statuses = statuses;
            });
        } else {
            if snapshot
                .entry_for_path(&path)
                .map(|entry| entry.is_ignored)
                .unwrap_or(false)
            {
                self.remove_repo_path(&path, snapshot);
                return None;
            }

            let repo = snapshot.repo_for(&path)?;

            let work_dir = repo.work_directory(snapshot)?;
            let work_dir_id = repo.work_directory.clone();

            snapshot
                .git_repositories
                .update(&work_dir_id, |entry| entry.scan_id = scan_id);

            let local_repo = snapshot.get_local_repo(&repo)?.to_owned();

            // Short circuit if we've already scanned everything
            if local_repo.full_scan_id == scan_id {
                return None;
            }

            let mut repository = snapshot.repository_entries.remove(&work_dir)?;

            for entry in snapshot.descendent_entries(false, false, path) {
                let Some(repo_path) = repo.work_directory.relativize(snapshot, &entry.path) else {
                    continue;
                };

                let status = local_repo.repo_ptr.lock().status(&repo_path);
                if let Some(status) = status {
                    repository.statuses.insert(repo_path.clone(), status);
                } else {
                    repository.statuses.remove(&repo_path);
                }
            }

            snapshot.repository_entries.insert(work_dir, repository)
        }

        Some(())
    }

    async fn update_ignore_statuses(&self) {
        use futures::FutureExt as _;

        let mut snapshot = self.snapshot.lock().clone();
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
            self.snapshot
                .lock()
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
                                request = self.refresh_requests_rx.recv().fuse() => {
                                    let Ok((paths, barrier)) = request else { break };
                                    if !self.process_refresh_request(paths, barrier).await {
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
        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores_by_parent_abs_path.get(&job.abs_path) {
            ignore_stack = ignore_stack.append(job.abs_path.clone(), ignore.clone());
        }

        let mut entries_by_id_edits = Vec::new();
        let mut entries_by_path_edits = Vec::new();
        let path = job.abs_path.strip_prefix(&snapshot.abs_path).unwrap();
        for mut entry in snapshot.child_entries(path).cloned() {
            let was_ignored = entry.is_ignored;
            let abs_path = snapshot.abs_path().join(&entry.path);
            entry.is_ignored = ignore_stack.is_abs_path_ignored(&abs_path, entry.is_dir());
            if entry.is_dir() {
                let child_ignore_stack = if entry.is_ignored {
                    IgnoreStack::all()
                } else {
                    ignore_stack.clone()
                };
                job.ignore_queue
                    .send(UpdateIgnoreStatusJob {
                        abs_path: abs_path.into(),
                        ignore_stack: child_ignore_stack,
                        ignore_queue: job.ignore_queue.clone(),
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

        let mut snapshot = self.snapshot.lock();
        snapshot.entries_by_path.edit(entries_by_path_edits, &());
        snapshot.entries_by_id.edit(entries_by_id_edits, &());
    }

    fn build_change_set(
        &self,
        old_snapshot: &Snapshot,
        new_snapshot: &Snapshot,
        event_paths: &[Arc<Path>],
    ) -> HashMap<(Arc<Path>, ProjectEntryId), PathChange> {
        use PathChange::{Added, AddedOrUpdated, Removed, Updated};

        let mut changes = HashMap::default();
        let mut old_paths = old_snapshot.entries_by_path.cursor::<PathKey>();
        let mut new_paths = new_snapshot.entries_by_path.cursor::<PathKey>();
        let received_before_initialized = !self.finished_initial_scan;

        for path in event_paths {
            let path = PathKey(path.clone());
            old_paths.seek(&path, Bias::Left, &());
            new_paths.seek(&path, Bias::Left, &());

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
                                changes.insert((old_entry.path.clone(), old_entry.id), Removed);
                                old_paths.next(&());
                            }
                            Ordering::Equal => {
                                if received_before_initialized {
                                    // If the worktree was not fully initialized when this event was generated,
                                    // we can't know whether this entry was added during the scan or whether
                                    // it was merely updated.
                                    changes.insert(
                                        (new_entry.path.clone(), new_entry.id),
                                        AddedOrUpdated,
                                    );
                                } else if old_entry.mtime != new_entry.mtime {
                                    changes.insert((new_entry.path.clone(), new_entry.id), Updated);
                                }
                                old_paths.next(&());
                                new_paths.next(&());
                            }
                            Ordering::Greater => {
                                changes.insert((new_entry.path.clone(), new_entry.id), Added);
                                new_paths.next(&());
                            }
                        }
                    }
                    (Some(old_entry), None) => {
                        changes.insert((old_entry.path.clone(), old_entry.id), Removed);
                        old_paths.next(&());
                    }
                    (None, Some(new_entry)) => {
                        changes.insert((new_entry.path.clone(), new_entry.id), Added);
                        new_paths.next(&());
                    }
                    (None, None) => break,
                }
            }
        }

        changes
    }

    async fn progress_timer(&self, running: bool) {
        if !running {
            return futures::future::pending().await;
        }

        #[cfg(any(test, feature = "test-support"))]
        if self.fs.is_fake() {
            return self.executor.simulate_random_delay().await;
        }

        smol::Timer::after(Duration::from_millis(100)).await;
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
}

struct UpdateIgnoreStatusJob {
    abs_path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    ignore_queue: Sender<UpdateIgnoreStatusJob>,
}

pub trait WorktreeHandle {
    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;
}

impl WorktreeHandle for ModelHandle<Worktree> {
    // When the worktree's FS event stream sometimes delivers "redundant" events for FS changes that
    // occurred before the worktree was constructed. These events can cause the worktree to perfrom
    // extra directory scans, and emit extra scan-state notifications.
    //
    // This function mutates the worktree's directory and waits for those mutations to be picked up,
    // to ensure that all redundant FS events have already been processed.
    #[cfg(any(test, feature = "test-support"))]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        use smol::future::FutureExt;

        let filename = "fs-event-sentinel";
        let tree = self.clone();
        let (fs, root_path) = self.read_with(cx, |tree, _| {
            let tree = tree.as_local().unwrap();
            (tree.fs.clone(), tree.abs_path().clone())
        });

        async move {
            fs.create_file(&root_path.join(filename), Default::default())
                .await
                .unwrap();
            tree.condition(cx, |tree, _| tree.entry_for_path(filename).is_some())
                .await;

            fs.remove_file(&root_path.join(filename), Default::default())
                .await
                .unwrap();
            tree.condition(cx, |tree, _| tree.entry_for_path(filename).is_none())
                .await;

            cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
                .await;
        }
        .boxed_local()
    }
}

#[derive(Clone, Debug)]
struct TraversalProgress<'a> {
    max_path: &'a Path,
    count: usize,
    visible_count: usize,
    file_count: usize,
    visible_file_count: usize,
}

impl<'a> TraversalProgress<'a> {
    fn count(&self, include_dirs: bool, include_ignored: bool) -> usize {
        match (include_ignored, include_dirs) {
            (true, true) => self.count,
            (true, false) => self.file_count,
            (false, true) => self.visible_count,
            (false, false) => self.visible_file_count,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for TraversalProgress<'a> {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.max_path = summary.max_path.as_ref();
        self.count += summary.count;
        self.visible_count += summary.visible_count;
        self.file_count += summary.file_count;
        self.visible_file_count += summary.visible_file_count;
    }
}

impl<'a> Default for TraversalProgress<'a> {
    fn default() -> Self {
        Self {
            max_path: Path::new(""),
            count: 0,
            visible_count: 0,
            file_count: 0,
            visible_file_count: 0,
        }
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

struct DescendentEntriesIter<'a> {
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
            mtime: Some(entry.mtime.into()),
            is_symlink: entry.is_symlink,
            is_ignored: entry.is_ignored,
        }
    }
}

impl<'a> TryFrom<(&'a CharBag, proto::Entry)> for Entry {
    type Error = anyhow::Error;

    fn try_from((root_char_bag, entry): (&'a CharBag, proto::Entry)) -> Result<Self> {
        if let Some(mtime) = entry.mtime {
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
                mtime: mtime.into(),
                is_symlink: entry.is_symlink,
                is_ignored: entry.is_ignored,
            })
        } else {
            Err(anyhow!(
                "missing mtime in remote worktree entry {:?}",
                entry.path
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, RealFs};
    use gpui::{executor::Deterministic, TestAppContext};
    use pretty_assertions::assert_eq;
    use rand::prelude::*;
    use serde_json::json;
    use std::{env, fmt::Write};
    use util::{http::FakeHttpClient, test::temp_tree};

    #[gpui::test]
    async fn test_traversal(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
               ".gitignore": "a/b\n",
               "a": {
                   "b": "",
                   "c": "",
               }
            }),
        )
        .await;

        let http_client = FakeHttpClient::with_404_response();
        let client = cx.read(|cx| Client::new(http_client, cx));

        let tree = Worktree::local(
            client,
            Path::new("/root"),
            true,
            fs,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        tree.read_with(cx, |tree, _| {
            assert_eq!(
                tree.entries(false)
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![
                    Path::new(""),
                    Path::new(".gitignore"),
                    Path::new("a"),
                    Path::new("a/c"),
                ]
            );
            assert_eq!(
                tree.entries(true)
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![
                    Path::new(""),
                    Path::new(".gitignore"),
                    Path::new("a"),
                    Path::new("a/b"),
                    Path::new("a/c"),
                ]
            );
        })
    }

    #[gpui::test]
    async fn test_descendent_entries(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "a": "",
                "b": {
                   "c": {
                       "d": ""
                   },
                   "e": {}
                },
                "f": "",
                "g": {
                    "h": {}
                },
                "i": {
                    "j": {
                        "k": ""
                    },
                    "l": {

                    }
                },
                ".gitignore": "i/j\n",
            }),
        )
        .await;

        let http_client = FakeHttpClient::with_404_response();
        let client = cx.read(|cx| Client::new(http_client, cx));

        let tree = Worktree::local(
            client,
            Path::new("/root"),
            true,
            fs,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        tree.read_with(cx, |tree, _| {
            assert_eq!(
                tree.descendent_entries(false, false, Path::new("b"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![Path::new("b/c/d"),]
            );
            assert_eq!(
                tree.descendent_entries(true, false, Path::new("b"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![
                    Path::new("b"),
                    Path::new("b/c"),
                    Path::new("b/c/d"),
                    Path::new("b/e"),
                ]
            );

            assert_eq!(
                tree.descendent_entries(false, false, Path::new("g"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                Vec::<PathBuf>::new()
            );
            assert_eq!(
                tree.descendent_entries(true, false, Path::new("g"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![Path::new("g"), Path::new("g/h"),]
            );

            assert_eq!(
                tree.descendent_entries(false, false, Path::new("i"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                Vec::<PathBuf>::new()
            );
            assert_eq!(
                tree.descendent_entries(false, true, Path::new("i"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![Path::new("i/j/k")]
            );
            assert_eq!(
                tree.descendent_entries(true, false, Path::new("i"))
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![Path::new("i"), Path::new("i/l"),]
            );
        })
    }

    #[gpui::test(iterations = 10)]
    async fn test_circular_symlinks(executor: Arc<Deterministic>, cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "lib": {
                    "a": {
                        "a.txt": ""
                    },
                    "b": {
                        "b.txt": ""
                    }
                }
            }),
        )
        .await;
        fs.insert_symlink("/root/lib/a/lib", "..".into()).await;
        fs.insert_symlink("/root/lib/b/lib", "..".into()).await;

        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));
        let tree = Worktree::local(
            client,
            Path::new("/root"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        tree.read_with(cx, |tree, _| {
            assert_eq!(
                tree.entries(false)
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![
                    Path::new(""),
                    Path::new("lib"),
                    Path::new("lib/a"),
                    Path::new("lib/a/a.txt"),
                    Path::new("lib/a/lib"),
                    Path::new("lib/b"),
                    Path::new("lib/b/b.txt"),
                    Path::new("lib/b/lib"),
                ]
            );
        });

        fs.rename(
            Path::new("/root/lib/a/lib"),
            Path::new("/root/lib/a/lib-2"),
            Default::default(),
        )
        .await
        .unwrap();
        executor.run_until_parked();
        tree.read_with(cx, |tree, _| {
            assert_eq!(
                tree.entries(false)
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![
                    Path::new(""),
                    Path::new("lib"),
                    Path::new("lib/a"),
                    Path::new("lib/a/a.txt"),
                    Path::new("lib/a/lib-2"),
                    Path::new("lib/b"),
                    Path::new("lib/b/b.txt"),
                    Path::new("lib/b/lib"),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_rescan_with_gitignore(cx: &mut TestAppContext) {
        let parent_dir = temp_tree(json!({
            ".gitignore": "ancestor-ignored-file1\nancestor-ignored-file2\n",
            "tree": {
                ".git": {},
                ".gitignore": "ignored-dir\n",
                "tracked-dir": {
                    "tracked-file1": "",
                    "ancestor-ignored-file1": "",
                },
                "ignored-dir": {
                    "ignored-file1": ""
                }
            }
        }));
        let dir = parent_dir.path().join("tree");

        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));

        let tree = Worktree::local(
            client,
            dir.as_path(),
            true,
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        tree.flush_fs_events(cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            assert!(
                !tree
                    .entry_for_path("tracked-dir/tracked-file1")
                    .unwrap()
                    .is_ignored
            );
            assert!(
                tree.entry_for_path("tracked-dir/ancestor-ignored-file1")
                    .unwrap()
                    .is_ignored
            );
            assert!(
                tree.entry_for_path("ignored-dir/ignored-file1")
                    .unwrap()
                    .is_ignored
            );
        });

        std::fs::write(dir.join("tracked-dir/tracked-file2"), "").unwrap();
        std::fs::write(dir.join("tracked-dir/ancestor-ignored-file2"), "").unwrap();
        std::fs::write(dir.join("ignored-dir/ignored-file2"), "").unwrap();
        tree.flush_fs_events(cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            assert!(
                !tree
                    .entry_for_path("tracked-dir/tracked-file2")
                    .unwrap()
                    .is_ignored
            );
            assert!(
                tree.entry_for_path("tracked-dir/ancestor-ignored-file2")
                    .unwrap()
                    .is_ignored
            );
            assert!(
                tree.entry_for_path("ignored-dir/ignored-file2")
                    .unwrap()
                    .is_ignored
            );
            assert!(tree.entry_for_path(".git").unwrap().is_ignored);
        });
    }

    #[gpui::test]
    async fn test_git_repository_for_path(cx: &mut TestAppContext) {
        let root = temp_tree(json!({
            "c.txt": "",
            "dir1": {
                ".git": {},
                "deps": {
                    "dep1": {
                        ".git": {},
                        "src": {
                            "a.txt": ""
                        }
                    }
                },
                "src": {
                    "b.txt": ""
                }
            },
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = cx.read(|cx| Client::new(http_client, cx));
        let tree = Worktree::local(
            client,
            root.path(),
            true,
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        tree.flush_fs_events(cx).await;

        tree.read_with(cx, |tree, _cx| {
            let tree = tree.as_local().unwrap();

            assert!(tree.repo_for("c.txt".as_ref()).is_none());

            let entry = tree.repo_for("dir1/src/b.txt".as_ref()).unwrap();
            assert_eq!(
                entry
                    .work_directory(tree)
                    .map(|directory| directory.as_ref().to_owned()),
                Some(Path::new("dir1").to_owned())
            );

            let entry = tree.repo_for("dir1/deps/dep1/src/a.txt".as_ref()).unwrap();
            assert_eq!(
                entry
                    .work_directory(tree)
                    .map(|directory| directory.as_ref().to_owned()),
                Some(Path::new("dir1/deps/dep1").to_owned())
            );

            let entries = tree.files(false, 0);

            let paths_with_repos = tree
                .entries_with_repos(entries)
                .map(|(entry, repo)| {
                    (
                        entry.path.as_ref(),
                        repo.and_then(|repo| {
                            repo.work_directory(&tree)
                                .map(|work_directory| work_directory.0.to_path_buf())
                        }),
                    )
                })
                .collect::<Vec<_>>();

            assert_eq!(
                paths_with_repos,
                &[
                    (Path::new("c.txt"), None),
                    (
                        Path::new("dir1/deps/dep1/src/a.txt"),
                        Some(Path::new("dir1/deps/dep1").into())
                    ),
                    (Path::new("dir1/src/b.txt"), Some(Path::new("dir1").into())),
                ]
            );
        });

        let repo_update_events = Arc::new(Mutex::new(vec![]));
        tree.update(cx, |_, cx| {
            let repo_update_events = repo_update_events.clone();
            cx.subscribe(&tree, move |_, _, event, _| {
                if let Event::UpdatedGitRepositories(update) = event {
                    repo_update_events.lock().push(update.clone());
                }
            })
            .detach();
        });

        std::fs::write(root.path().join("dir1/.git/random_new_file"), "hello").unwrap();
        tree.flush_fs_events(cx).await;

        assert_eq!(
            repo_update_events.lock()[0]
                .keys()
                .cloned()
                .collect::<Vec<Arc<Path>>>(),
            vec![Path::new("dir1").into()]
        );

        std::fs::remove_dir_all(root.path().join("dir1/.git")).unwrap();
        tree.flush_fs_events(cx).await;

        tree.read_with(cx, |tree, _cx| {
            let tree = tree.as_local().unwrap();

            assert!(tree.repo_for("dir1/src/b.txt".as_ref()).is_none());
        });
    }

    #[gpui::test]
    async fn test_git_status(cx: &mut TestAppContext) {
        #[track_caller]
        fn git_init(path: &Path) -> git2::Repository {
            git2::Repository::init(path).expect("Failed to initialize git repository")
        }

        #[track_caller]
        fn git_add(path: &Path, repo: &git2::Repository) {
            let mut index = repo.index().expect("Failed to get index");
            index.add_path(path).expect("Failed to add a.txt");
            index.write().expect("Failed to write index");
        }

        #[track_caller]
        fn git_remove_index(path: &Path, repo: &git2::Repository) {
            let mut index = repo.index().expect("Failed to get index");
            index.remove_path(path).expect("Failed to add a.txt");
            index.write().expect("Failed to write index");
        }

        #[track_caller]
        fn git_commit(msg: &'static str, repo: &git2::Repository) {
            use git2::Signature;

            let signature = Signature::now("test", "test@zed.dev").unwrap();
            let oid = repo.index().unwrap().write_tree().unwrap();
            let tree = repo.find_tree(oid).unwrap();
            if let Some(head) = repo.head().ok() {
                let parent_obj = head.peel(git2::ObjectType::Commit).unwrap();

                let parent_commit = parent_obj.as_commit().unwrap();

                repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    msg,
                    &tree,
                    &[parent_commit],
                )
                .expect("Failed to commit with parent");
            } else {
                repo.commit(Some("HEAD"), &signature, &signature, msg, &tree, &[])
                    .expect("Failed to commit");
            }
        }

        #[track_caller]
        fn git_stash(repo: &mut git2::Repository) {
            use git2::Signature;

            let signature = Signature::now("test", "test@zed.dev").unwrap();
            repo.stash_save(&signature, "N/A", None)
                .expect("Failed to stash");
        }

        #[track_caller]
        fn git_reset(offset: usize, repo: &git2::Repository) {
            let head = repo.head().expect("Couldn't get repo head");
            let object = head.peel(git2::ObjectType::Commit).unwrap();
            let commit = object.as_commit().unwrap();
            let new_head = commit
                .parents()
                .inspect(|parnet| {
                    parnet.message();
                })
                .skip(offset)
                .next()
                .expect("Not enough history");
            repo.reset(&new_head.as_object(), git2::ResetType::Soft, None)
                .expect("Could not reset");
        }

        #[allow(dead_code)]
        #[track_caller]
        fn git_status(repo: &git2::Repository) -> HashMap<String, git2::Status> {
            repo.statuses(None)
                .unwrap()
                .iter()
                .map(|status| (status.path().unwrap().to_string(), status.status()))
                .collect()
        }

        const IGNORE_RULE: &'static str = "**/target";

        let root = temp_tree(json!({
            "project": {
                "a.txt": "a",
                "b.txt": "bb",
                "c": {
                    "d": {
                        "e.txt": "eee"
                    }
                },
                "f.txt": "ffff",
                "target": {
                    "build_file": "???"
                },
                ".gitignore": IGNORE_RULE
            },

        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = cx.read(|cx| Client::new(http_client, cx));
        let tree = Worktree::local(
            client,
            root.path(),
            true,
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        const A_TXT: &'static str = "a.txt";
        const B_TXT: &'static str = "b.txt";
        const E_TXT: &'static str = "c/d/e.txt";
        const F_TXT: &'static str = "f.txt";
        const DOTGITIGNORE: &'static str = ".gitignore";
        const BUILD_FILE: &'static str = "target/build_file";

        let work_dir = root.path().join("project");
        let mut repo = git_init(work_dir.as_path());
        repo.add_ignore_rule(IGNORE_RULE).unwrap();
        git_add(Path::new(A_TXT), &repo);
        git_add(Path::new(E_TXT), &repo);
        git_add(Path::new(DOTGITIGNORE), &repo);
        git_commit("Initial commit", &repo);

        std::fs::write(work_dir.join(A_TXT), "aa").unwrap();

        tree.flush_fs_events(cx).await;

        // Check that the right git state is observed on startup
        tree.read_with(cx, |tree, _cx| {
            let snapshot = tree.snapshot();
            assert_eq!(snapshot.repository_entries.iter().count(), 1);
            let (dir, repo) = snapshot.repository_entries.iter().next().unwrap();
            assert_eq!(dir.0.as_ref(), Path::new("project"));

            assert_eq!(repo.statuses.iter().count(), 3);
            assert_eq!(
                repo.statuses.get(&Path::new(A_TXT).into()),
                Some(&GitFileStatus::Modified)
            );
            assert_eq!(
                repo.statuses.get(&Path::new(B_TXT).into()),
                Some(&GitFileStatus::Added)
            );
            assert_eq!(
                repo.statuses.get(&Path::new(F_TXT).into()),
                Some(&GitFileStatus::Added)
            );
        });

        git_add(Path::new(A_TXT), &repo);
        git_add(Path::new(B_TXT), &repo);
        git_commit("Committing modified and added", &repo);
        tree.flush_fs_events(cx).await;

        // Check that repo only changes are tracked
        tree.read_with(cx, |tree, _cx| {
            let snapshot = tree.snapshot();
            let (_, repo) = snapshot.repository_entries.iter().next().unwrap();

            assert_eq!(repo.statuses.iter().count(), 1);
            assert_eq!(
                repo.statuses.get(&Path::new(F_TXT).into()),
                Some(&GitFileStatus::Added)
            );
        });

        git_reset(0, &repo);
        git_remove_index(Path::new(B_TXT), &repo);
        git_stash(&mut repo);
        std::fs::write(work_dir.join(E_TXT), "eeee").unwrap();
        std::fs::write(work_dir.join(BUILD_FILE), "this should be ignored").unwrap();
        tree.flush_fs_events(cx).await;

        // Check that more complex repo changes are tracked
        tree.read_with(cx, |tree, _cx| {
            let snapshot = tree.snapshot();
            let (_, repo) = snapshot.repository_entries.iter().next().unwrap();

            assert_eq!(repo.statuses.iter().count(), 3);
            assert_eq!(repo.statuses.get(&Path::new(A_TXT).into()), None);
            assert_eq!(
                repo.statuses.get(&Path::new(B_TXT).into()),
                Some(&GitFileStatus::Added)
            );
            assert_eq!(
                repo.statuses.get(&Path::new(E_TXT).into()),
                Some(&GitFileStatus::Modified)
            );
            assert_eq!(
                repo.statuses.get(&Path::new(F_TXT).into()),
                Some(&GitFileStatus::Added)
            );
        });

        std::fs::remove_file(work_dir.join(B_TXT)).unwrap();
        std::fs::remove_dir_all(work_dir.join("c")).unwrap();
        std::fs::write(
            work_dir.join(DOTGITIGNORE),
            [IGNORE_RULE, "f.txt"].join("\n"),
        )
        .unwrap();

        git_add(Path::new(DOTGITIGNORE), &repo);
        git_commit("Committing modified git ignore", &repo);

        tree.flush_fs_events(cx).await;

        // Check that non-repo behavior is tracked
        tree.read_with(cx, |tree, _cx| {
            let snapshot = tree.snapshot();
            let (_, repo) = snapshot.repository_entries.iter().next().unwrap();

            assert_eq!(repo.statuses.iter().count(), 0);
        });

        let mut renamed_dir_name = "first_directory/second_directory";
        const RENAMED_FILE: &'static str = "rf.txt";

        std::fs::create_dir_all(work_dir.join(renamed_dir_name)).unwrap();
        std::fs::write(
            work_dir.join(renamed_dir_name).join(RENAMED_FILE),
            "new-contents",
        )
        .unwrap();

        tree.flush_fs_events(cx).await;

        tree.read_with(cx, |tree, _cx| {
            let snapshot = tree.snapshot();
            let (_, repo) = snapshot.repository_entries.iter().next().unwrap();

            assert_eq!(repo.statuses.iter().count(), 1);
            assert_eq!(
                repo.statuses
                    .get(&Path::new(renamed_dir_name).join(RENAMED_FILE).into()),
                Some(&GitFileStatus::Added)
            );
        });

        renamed_dir_name = "new_first_directory/second_directory";

        std::fs::rename(
            work_dir.join("first_directory"),
            work_dir.join("new_first_directory"),
        )
        .unwrap();

        tree.flush_fs_events(cx).await;

        tree.read_with(cx, |tree, _cx| {
            let snapshot = tree.snapshot();
            let (_, repo) = snapshot.repository_entries.iter().next().unwrap();

            assert_eq!(repo.statuses.iter().count(), 1);
            assert_eq!(
                repo.statuses
                    .get(&Path::new(renamed_dir_name).join(RENAMED_FILE).into()),
                Some(&GitFileStatus::Added)
            );
        });
    }

    #[gpui::test]
    async fn test_write_file(cx: &mut TestAppContext) {
        let dir = temp_tree(json!({
            ".git": {},
            ".gitignore": "ignored-dir\n",
            "tracked-dir": {},
            "ignored-dir": {}
        }));

        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));

        let tree = Worktree::local(
            client,
            dir.path(),
            true,
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        tree.flush_fs_events(cx).await;

        tree.update(cx, |tree, cx| {
            tree.as_local().unwrap().write_file(
                Path::new("tracked-dir/file.txt"),
                "hello".into(),
                Default::default(),
                cx,
            )
        })
        .await
        .unwrap();
        tree.update(cx, |tree, cx| {
            tree.as_local().unwrap().write_file(
                Path::new("ignored-dir/file.txt"),
                "world".into(),
                Default::default(),
                cx,
            )
        })
        .await
        .unwrap();

        tree.read_with(cx, |tree, _| {
            let tracked = tree.entry_for_path("tracked-dir/file.txt").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/file.txt").unwrap();
            assert!(!tracked.is_ignored);
            assert!(ignored.is_ignored);
        });
    }

    #[gpui::test(iterations = 30)]
    async fn test_create_directory_during_initial_scan(cx: &mut TestAppContext) {
        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "b": {},
                "c": {},
                "d": {},
            }),
        )
        .await;

        let tree = Worktree::local(
            client,
            "/root".as_ref(),
            true,
            fs,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let mut snapshot1 = tree.update(cx, |tree, _| tree.as_local().unwrap().snapshot());

        let entry = tree
            .update(cx, |tree, cx| {
                tree.as_local_mut()
                    .unwrap()
                    .create_entry("a/e".as_ref(), true, cx)
            })
            .await
            .unwrap();
        assert!(entry.is_dir());

        cx.foreground().run_until_parked();
        tree.read_with(cx, |tree, _| {
            assert_eq!(tree.entry_for_path("a/e").unwrap().kind, EntryKind::Dir);
        });

        let snapshot2 = tree.update(cx, |tree, _| tree.as_local().unwrap().snapshot());
        let update = snapshot2.build_update(&snapshot1, 0, 0, true);
        snapshot1.apply_remote_update(update).unwrap();
        assert_eq!(snapshot1.to_vec(true), snapshot2.to_vec(true),);
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_worktree_operations_during_initial_scan(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        let operations = env::var("OPERATIONS")
            .map(|o| o.parse().unwrap())
            .unwrap_or(5);
        let initial_entries = env::var("INITIAL_ENTRIES")
            .map(|o| o.parse().unwrap())
            .unwrap_or(20);

        let root_dir = Path::new("/test");
        let fs = FakeFs::new(cx.background()) as Arc<dyn Fs>;
        fs.as_fake().insert_tree(root_dir, json!({})).await;
        for _ in 0..initial_entries {
            randomly_mutate_fs(&fs, root_dir, 1.0, &mut rng).await;
        }
        log::info!("generated initial tree");

        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));
        let worktree = Worktree::local(
            client.clone(),
            root_dir,
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let mut snapshot = worktree.update(cx, |tree, _| tree.as_local().unwrap().snapshot());

        for _ in 0..operations {
            worktree
                .update(cx, |worktree, cx| {
                    randomly_mutate_worktree(worktree, &mut rng, cx)
                })
                .await
                .log_err();
            worktree.read_with(cx, |tree, _| {
                tree.as_local().unwrap().snapshot.check_invariants()
            });

            if rng.gen_bool(0.6) {
                let new_snapshot =
                    worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
                let update = new_snapshot.build_update(&snapshot, 0, 0, true);
                snapshot.apply_remote_update(update.clone()).unwrap();
                assert_eq!(
                    snapshot.to_vec(true),
                    new_snapshot.to_vec(true),
                    "incorrect snapshot after update {:?}",
                    update
                );
            }
        }

        worktree
            .update(cx, |tree, _| tree.as_local_mut().unwrap().scan_complete())
            .await;
        worktree.read_with(cx, |tree, _| {
            tree.as_local().unwrap().snapshot.check_invariants()
        });

        let new_snapshot = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
        let update = new_snapshot.build_update(&snapshot, 0, 0, true);
        snapshot.apply_remote_update(update.clone()).unwrap();
        assert_eq!(
            snapshot.to_vec(true),
            new_snapshot.to_vec(true),
            "incorrect snapshot after update {:?}",
            update
        );
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_worktree_changes(cx: &mut TestAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|o| o.parse().unwrap())
            .unwrap_or(40);
        let initial_entries = env::var("INITIAL_ENTRIES")
            .map(|o| o.parse().unwrap())
            .unwrap_or(20);

        let root_dir = Path::new("/test");
        let fs = FakeFs::new(cx.background()) as Arc<dyn Fs>;
        fs.as_fake().insert_tree(root_dir, json!({})).await;
        for _ in 0..initial_entries {
            randomly_mutate_fs(&fs, root_dir, 1.0, &mut rng).await;
        }
        log::info!("generated initial tree");

        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));
        let worktree = Worktree::local(
            client.clone(),
            root_dir,
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        worktree
            .update(cx, |tree, _| tree.as_local_mut().unwrap().scan_complete())
            .await;

        // After the initial scan is complete, the `UpdatedEntries` event can
        // be used to follow along with all changes to the worktree's snapshot.
        worktree.update(cx, |tree, cx| {
            let mut paths = tree
                .as_local()
                .unwrap()
                .paths()
                .cloned()
                .collect::<Vec<_>>();

            cx.subscribe(&worktree, move |tree, _, event, _| {
                if let Event::UpdatedEntries(changes) = event {
                    for ((path, _), change_type) in changes.iter() {
                        let path = path.clone();
                        let ix = match paths.binary_search(&path) {
                            Ok(ix) | Err(ix) => ix,
                        };
                        match change_type {
                            PathChange::Added => {
                                assert_ne!(paths.get(ix), Some(&path));
                                paths.insert(ix, path);
                            }

                            PathChange::Removed => {
                                assert_eq!(paths.get(ix), Some(&path));
                                paths.remove(ix);
                            }

                            PathChange::Updated => {
                                assert_eq!(paths.get(ix), Some(&path));
                            }

                            PathChange::AddedOrUpdated => {
                                if paths[ix] != path {
                                    paths.insert(ix, path);
                                }
                            }
                        }
                    }

                    let new_paths = tree.paths().cloned().collect::<Vec<_>>();
                    assert_eq!(paths, new_paths, "incorrect changes: {:?}", changes);
                }
            })
            .detach();
        });

        fs.as_fake().pause_events();
        let mut snapshots = Vec::new();
        let mut mutations_len = operations;
        while mutations_len > 1 {
            if rng.gen_bool(0.2) {
                worktree
                    .update(cx, |worktree, cx| {
                        randomly_mutate_worktree(worktree, &mut rng, cx)
                    })
                    .await
                    .log_err();
            } else {
                randomly_mutate_fs(&fs, root_dir, 1.0, &mut rng).await;
            }

            let buffered_event_count = fs.as_fake().buffered_event_count();
            if buffered_event_count > 0 && rng.gen_bool(0.3) {
                let len = rng.gen_range(0..=buffered_event_count);
                log::info!("flushing {} events", len);
                fs.as_fake().flush_events(len);
            } else {
                randomly_mutate_fs(&fs, root_dir, 0.6, &mut rng).await;
                mutations_len -= 1;
            }

            cx.foreground().run_until_parked();
            if rng.gen_bool(0.2) {
                log::info!("storing snapshot {}", snapshots.len());
                let snapshot =
                    worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
                snapshots.push(snapshot);
            }
        }

        log::info!("quiescing");
        fs.as_fake().flush_events(usize::MAX);
        cx.foreground().run_until_parked();
        let snapshot = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
        snapshot.check_invariants();

        {
            let new_worktree = Worktree::local(
                client.clone(),
                root_dir,
                true,
                fs.clone(),
                Default::default(),
                &mut cx.to_async(),
            )
            .await
            .unwrap();
            new_worktree
                .update(cx, |tree, _| tree.as_local_mut().unwrap().scan_complete())
                .await;
            let new_snapshot =
                new_worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
            assert_eq!(snapshot.to_vec(true), new_snapshot.to_vec(true));
        }

        for (i, mut prev_snapshot) in snapshots.into_iter().enumerate() {
            let include_ignored = rng.gen::<bool>();
            if !include_ignored {
                let mut entries_by_path_edits = Vec::new();
                let mut entries_by_id_edits = Vec::new();
                for entry in prev_snapshot
                    .entries_by_id
                    .cursor::<()>()
                    .filter(|e| e.is_ignored)
                {
                    entries_by_path_edits.push(Edit::Remove(PathKey(entry.path.clone())));
                    entries_by_id_edits.push(Edit::Remove(entry.id));
                }

                prev_snapshot
                    .entries_by_path
                    .edit(entries_by_path_edits, &());
                prev_snapshot.entries_by_id.edit(entries_by_id_edits, &());
            }

            let update = snapshot.build_update(&prev_snapshot, 0, 0, include_ignored);
            prev_snapshot.apply_remote_update(update.clone()).unwrap();
            assert_eq!(
                prev_snapshot.to_vec(include_ignored),
                snapshot.to_vec(include_ignored),
                "wrong update for snapshot {i}. update: {:?}",
                update
            );
        }
    }

    fn randomly_mutate_worktree(
        worktree: &mut Worktree,
        rng: &mut impl Rng,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        log::info!("mutating worktree");
        let worktree = worktree.as_local_mut().unwrap();
        let snapshot = worktree.snapshot();
        let entry = snapshot.entries(false).choose(rng).unwrap();

        match rng.gen_range(0_u32..100) {
            0..=33 if entry.path.as_ref() != Path::new("") => {
                log::info!("deleting entry {:?} ({})", entry.path, entry.id.0);
                worktree.delete_entry(entry.id, cx).unwrap()
            }
            ..=66 if entry.path.as_ref() != Path::new("") => {
                let other_entry = snapshot.entries(false).choose(rng).unwrap();
                let new_parent_path = if other_entry.is_dir() {
                    other_entry.path.clone()
                } else {
                    other_entry.path.parent().unwrap().into()
                };
                let mut new_path = new_parent_path.join(gen_name(rng));
                if new_path.starts_with(&entry.path) {
                    new_path = gen_name(rng).into();
                }

                log::info!(
                    "renaming entry {:?} ({}) to {:?}",
                    entry.path,
                    entry.id.0,
                    new_path
                );
                let task = worktree.rename_entry(entry.id, new_path, cx).unwrap();
                cx.foreground().spawn(async move {
                    task.await?;
                    Ok(())
                })
            }
            _ => {
                let task = if entry.is_dir() {
                    let child_path = entry.path.join(gen_name(rng));
                    let is_dir = rng.gen_bool(0.3);
                    log::info!(
                        "creating {} at {:?}",
                        if is_dir { "dir" } else { "file" },
                        child_path,
                    );
                    worktree.create_entry(child_path, is_dir, cx)
                } else {
                    log::info!("overwriting file {:?} ({})", entry.path, entry.id.0);
                    worktree.write_file(entry.path.clone(), "".into(), Default::default(), cx)
                };
                cx.foreground().spawn(async move {
                    task.await?;
                    Ok(())
                })
            }
        }
    }

    async fn randomly_mutate_fs(
        fs: &Arc<dyn Fs>,
        root_path: &Path,
        insertion_probability: f64,
        rng: &mut impl Rng,
    ) {
        log::info!("mutating fs");
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        for path in fs.as_fake().paths() {
            if path.starts_with(root_path) {
                if fs.is_file(&path).await {
                    files.push(path);
                } else {
                    dirs.push(path);
                }
            }
        }

        if (files.is_empty() && dirs.len() == 1) || rng.gen_bool(insertion_probability) {
            let path = dirs.choose(rng).unwrap();
            let new_path = path.join(gen_name(rng));

            if rng.gen() {
                log::info!(
                    "creating dir {:?}",
                    new_path.strip_prefix(root_path).unwrap()
                );
                fs.create_dir(&new_path).await.unwrap();
            } else {
                log::info!(
                    "creating file {:?}",
                    new_path.strip_prefix(root_path).unwrap()
                );
                fs.create_file(&new_path, Default::default()).await.unwrap();
            }
        } else if rng.gen_bool(0.05) {
            let ignore_dir_path = dirs.choose(rng).unwrap();
            let ignore_path = ignore_dir_path.join(&*GITIGNORE);

            let subdirs = dirs
                .iter()
                .filter(|d| d.starts_with(&ignore_dir_path))
                .cloned()
                .collect::<Vec<_>>();
            let subfiles = files
                .iter()
                .filter(|d| d.starts_with(&ignore_dir_path))
                .cloned()
                .collect::<Vec<_>>();
            let files_to_ignore = {
                let len = rng.gen_range(0..=subfiles.len());
                subfiles.choose_multiple(rng, len)
            };
            let dirs_to_ignore = {
                let len = rng.gen_range(0..subdirs.len());
                subdirs.choose_multiple(rng, len)
            };

            let mut ignore_contents = String::new();
            for path_to_ignore in files_to_ignore.chain(dirs_to_ignore) {
                writeln!(
                    ignore_contents,
                    "{}",
                    path_to_ignore
                        .strip_prefix(&ignore_dir_path)
                        .unwrap()
                        .to_str()
                        .unwrap()
                )
                .unwrap();
            }
            log::info!(
                "creating gitignore {:?} with contents:\n{}",
                ignore_path.strip_prefix(&root_path).unwrap(),
                ignore_contents
            );
            fs.save(
                &ignore_path,
                &ignore_contents.as_str().into(),
                Default::default(),
            )
            .await
            .unwrap();
        } else {
            let old_path = {
                let file_path = files.choose(rng);
                let dir_path = dirs[1..].choose(rng);
                file_path.into_iter().chain(dir_path).choose(rng).unwrap()
            };

            let is_rename = rng.gen();
            if is_rename {
                let new_path_parent = dirs
                    .iter()
                    .filter(|d| !d.starts_with(old_path))
                    .choose(rng)
                    .unwrap();

                let overwrite_existing_dir =
                    !old_path.starts_with(&new_path_parent) && rng.gen_bool(0.3);
                let new_path = if overwrite_existing_dir {
                    fs.remove_dir(
                        &new_path_parent,
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await
                    .unwrap();
                    new_path_parent.to_path_buf()
                } else {
                    new_path_parent.join(gen_name(rng))
                };

                log::info!(
                    "renaming {:?} to {}{:?}",
                    old_path.strip_prefix(&root_path).unwrap(),
                    if overwrite_existing_dir {
                        "overwrite "
                    } else {
                        ""
                    },
                    new_path.strip_prefix(&root_path).unwrap()
                );
                fs.rename(
                    &old_path,
                    &new_path,
                    fs::RenameOptions {
                        overwrite: true,
                        ignore_if_exists: true,
                    },
                )
                .await
                .unwrap();
            } else if fs.is_file(&old_path).await {
                log::info!(
                    "deleting file {:?}",
                    old_path.strip_prefix(&root_path).unwrap()
                );
                fs.remove_file(old_path, Default::default()).await.unwrap();
            } else {
                log::info!(
                    "deleting dir {:?}",
                    old_path.strip_prefix(&root_path).unwrap()
                );
                fs.remove_dir(
                    &old_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();
            }
        }
    }

    fn gen_name(rng: &mut impl Rng) -> String {
        (0..6)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .collect()
    }

    impl LocalSnapshot {
        fn check_invariants(&self) {
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
                    if !entry.is_ignored {
                        assert_eq!(visible_files.next().unwrap().inode, entry.inode);
                    }
                }
            }

            assert!(files.next().is_none());
            assert!(visible_files.next().is_none());

            let mut bfs_paths = Vec::new();
            let mut stack = vec![Path::new("")];
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

            for ignore_parent_abs_path in self.ignores_by_parent_abs_path.keys() {
                let ignore_parent_path =
                    ignore_parent_abs_path.strip_prefix(&self.abs_path).unwrap();
                assert!(self.entry_for_path(&ignore_parent_path).is_some());
                assert!(self
                    .entry_for_path(ignore_parent_path.join(&*GITIGNORE))
                    .is_some());
            }
        }

        fn to_vec(&self, include_ignored: bool) -> Vec<(&Path, u64, bool)> {
            let mut paths = Vec::new();
            for entry in self.entries_by_path.cursor::<()>() {
                if include_ignored || !entry.is_ignored {
                    paths.push((entry.path.as_ref(), entry.inode, entry.is_ignored));
                }
            }
            paths.sort_by(|a, b| a.0.cmp(b.0));
            paths
        }
    }
}
