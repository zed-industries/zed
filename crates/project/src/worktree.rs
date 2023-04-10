use crate::{
    copy_recursive, ignore::IgnoreStack, DiagnosticSummary, ProjectEntryId, RemoveOptions,
};
use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{anyhow, Context, Result};
use client::{proto, Client};
use clock::ReplicaId;
use collections::{HashMap, VecDeque};
use fs::{repository::GitRepository, Fs, LineEnding};
use futures::{
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    select_biased, Stream, StreamExt,
};
use fuzzy::CharBag;
use git::{DOT_GIT, GITIGNORE};
use gpui::{
    executor, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task,
};
use language::{
    proto::{
        deserialize_fingerprint, deserialize_version, serialize_fingerprint, serialize_line_ending,
        serialize_version,
    },
    Buffer, DiagnosticEntry, File as _, PointUtf16, Rope, RopeFingerprint, Unclipped,
};
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
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    task::Poll,
    time::{Duration, SystemTime},
};
use sum_tree::{Bias, Edit, SeekTarget, SumTree, TreeMap, TreeSet};
use util::{paths::HOME, ResultExt, TryFutureExt};

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
    diagnostics: HashMap<Arc<Path>, Vec<DiagnosticEntry<Unclipped<PointUtf16>>>>,
    diagnostic_summaries: TreeMap<PathKey, DiagnosticSummary>,
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
    diagnostic_summaries: TreeMap<PathKey, DiagnosticSummary>,
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
    scan_id: usize,
    completed_scan_id: usize,
}

#[derive(Clone)]
pub struct GitRepositoryEntry {
    pub(crate) repo: Arc<Mutex<dyn GitRepository>>,

    pub(crate) scan_id: usize,
    // Path to folder containing the .git file or directory
    pub(crate) content_path: Arc<Path>,
    // Path to the actual .git folder.
    // Note: if .git is a file, this points to the folder indicated by the .git file
    pub(crate) git_dir_path: Arc<Path>,
}

impl std::fmt::Debug for GitRepositoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRepositoryEntry")
            .field("content_path", &self.content_path)
            .field("git_dir_path", &self.git_dir_path)
            .finish()
    }
}

#[derive(Debug)]
pub struct LocalSnapshot {
    ignores_by_parent_abs_path: HashMap<Arc<Path>, (Arc<Gitignore>, usize)>,
    git_repositories: Vec<GitRepositoryEntry>,
    removed_entry_ids: HashMap<u64, ProjectEntryId>,
    next_entry_id: Arc<AtomicUsize>,
    snapshot: Snapshot,
}

impl Clone for LocalSnapshot {
    fn clone(&self) -> Self {
        Self {
            ignores_by_parent_abs_path: self.ignores_by_parent_abs_path.clone(),
            git_repositories: self.git_repositories.iter().cloned().collect(),
            removed_entry_ids: self.removed_entry_ids.clone(),
            next_entry_id: self.next_entry_id.clone(),
            snapshot: self.snapshot.clone(),
        }
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
    /// The worktree is performing its initial scan of the filesystem.
    Initializing {
        snapshot: LocalSnapshot,
        barrier: Option<barrier::Sender>,
    },
    Initialized {
        snapshot: LocalSnapshot,
    },
    /// The worktree is updating in response to filesystem events.
    Updating,
    Updated {
        snapshot: LocalSnapshot,
        changes: HashMap<Arc<Path>, PathChange>,
        barrier: Option<barrier::Sender>,
    },
}

struct ShareState {
    project_id: u64,
    snapshots_tx: watch::Sender<LocalSnapshot>,
    resume_updates: watch::Sender<()>,
    _maintain_remote_snapshot: Task<Option<()>>,
}

pub enum Event {
    UpdatedEntries(HashMap<Arc<Path>, PathChange>),
    UpdatedGitRepositories(Vec<GitRepositoryEntry>),
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
                git_repositories: Default::default(),
                removed_entry_ids: Default::default(),
                next_entry_id,
                snapshot: Snapshot {
                    id: WorktreeId::from_usize(cx.model_id()),
                    abs_path: abs_path.clone(),
                    root_name: root_name.clone(),
                    root_char_bag: root_name.chars().map(|c| c.to_ascii_lowercase()).collect(),
                    entries_by_path: Default::default(),
                    entries_by_id: Default::default(),
                    scan_id: 0,
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
                        this.as_local_mut()
                            .unwrap()
                            .background_scanner_updated(state, cx);
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
                    BackgroundScanner::new(snapshot, scan_states_tx, fs, background)
                        .run(events, path_changes_rx)
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
        cx: &mut MutableAppContext,
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
                scan_id: 0,
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
    ) -> impl Iterator<Item = (Arc<Path>, DiagnosticSummary)> + '_ {
        match self {
            Worktree::Local(worktree) => &worktree.diagnostic_summaries,
            Worktree::Remote(worktree) => &worktree.diagnostic_summaries,
        }
        .iter()
        .map(|(path, summary)| (path.0.clone(), *summary))
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
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let path = Arc::from(path);
        cx.spawn(move |this, mut cx| async move {
            let (file, contents, diff_base) = this
                .update(&mut cx, |t, cx| t.as_local().unwrap().load(&path, cx))
                .await?;
            Ok(cx.add_model(|cx| {
                let mut buffer = Buffer::from_file(0, contents, diff_base, Arc::new(file), cx);
                buffer.git_diff_recalc(cx);
                buffer
            }))
        })
    }

    pub fn diagnostics_for_path(
        &self,
        path: &Path,
    ) -> Option<Vec<DiagnosticEntry<Unclipped<PointUtf16>>>> {
        self.diagnostics.get(path).cloned()
    }

    pub fn update_diagnostics(
        &mut self,
        language_server_id: usize,
        worktree_path: Arc<Path>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        _: &mut ModelContext<Worktree>,
    ) -> Result<bool> {
        self.diagnostics.remove(&worktree_path);
        let old_summary = self
            .diagnostic_summaries
            .remove(&PathKey(worktree_path.clone()))
            .unwrap_or_default();
        let new_summary = DiagnosticSummary::new(language_server_id, &diagnostics);
        if !new_summary.is_empty() {
            self.diagnostic_summaries
                .insert(PathKey(worktree_path.clone()), new_summary);
            self.diagnostics.insert(worktree_path.clone(), diagnostics);
        }

        let updated = !old_summary.is_empty() || !new_summary.is_empty();
        if updated {
            if let Some(share) = self.share.as_ref() {
                self.client
                    .send(proto::UpdateDiagnosticSummary {
                        project_id: share.project_id,
                        worktree_id: self.id().to_proto(),
                        summary: Some(proto::DiagnosticSummary {
                            path: worktree_path.to_string_lossy().to_string(),
                            language_server_id: language_server_id as u64,
                            error_count: new_summary.error_count as u32,
                            warning_count: new_summary.warning_count as u32,
                        }),
                    })
                    .log_err();
            }
        }

        Ok(updated)
    }

    fn background_scanner_updated(
        &mut self,
        scan_state: ScanState,
        cx: &mut ModelContext<Worktree>,
    ) {
        match scan_state {
            ScanState::Initializing { snapshot, barrier } => {
                *self.is_scanning.0.borrow_mut() = true;
                self.set_snapshot(snapshot, cx);
                drop(barrier);
            }
            ScanState::Initialized { snapshot } => {
                *self.is_scanning.0.borrow_mut() = false;
                self.set_snapshot(snapshot, cx);
            }
            ScanState::Updating => {
                *self.is_scanning.0.borrow_mut() = true;
            }
            ScanState::Updated {
                snapshot,
                changes,
                barrier,
            } => {
                *self.is_scanning.0.borrow_mut() = false;
                cx.emit(Event::UpdatedEntries(changes));
                self.set_snapshot(snapshot, cx);
                drop(barrier);
            }
        }
        cx.notify();
    }

    fn set_snapshot(&mut self, new_snapshot: LocalSnapshot, cx: &mut ModelContext<Worktree>) {
        let updated_repos = Self::changed_repos(
            &self.snapshot.git_repositories,
            &new_snapshot.git_repositories,
        );
        self.snapshot = new_snapshot;

        if let Some(share) = self.share.as_mut() {
            *share.snapshots_tx.borrow_mut() = self.snapshot.clone();
        }

        if !updated_repos.is_empty() {
            cx.emit(Event::UpdatedGitRepositories(updated_repos));
        }
    }

    fn changed_repos(
        old_repos: &[GitRepositoryEntry],
        new_repos: &[GitRepositoryEntry],
    ) -> Vec<GitRepositoryEntry> {
        fn diff<'a>(
            a: &'a [GitRepositoryEntry],
            b: &'a [GitRepositoryEntry],
            updated: &mut HashMap<&'a Path, GitRepositoryEntry>,
        ) {
            for a_repo in a {
                let matched = b.iter().find(|b_repo| {
                    a_repo.git_dir_path == b_repo.git_dir_path && a_repo.scan_id == b_repo.scan_id
                });

                if matched.is_none() {
                    updated.insert(a_repo.git_dir_path.as_ref(), a_repo.clone());
                }
            }
        }

        let mut updated = HashMap::<&Path, GitRepositoryEntry>::default();

        diff(old_repos, new_repos, &mut updated);
        diff(new_repos, old_repos, &mut updated);

        updated.into_values().collect()
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

        cx.spawn(|this, mut cx| async move {
            let text = fs.load(&abs_path).await?;

            let diff_base = if let Some(repo) = snapshot.repo_for(&path) {
                if let Ok(repo_relative) = path.strip_prefix(repo.content_path) {
                    let repo_relative = repo_relative.to_owned();
                    cx.background()
                        .spawn(async move { repo.repo.lock().load_index_text(&repo_relative) })
                        .await
                } else {
                    None
                }
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

            for (path, summary) in self.diagnostic_summaries.iter() {
                if let Err(e) = self.client.send(proto::UpdateDiagnosticSummary {
                    project_id,
                    worktree_id,
                    summary: Some(summary.to_proto(&path.0)),
                }) {
                    return Task::ready(Err(e));
                }
            }

            let _maintain_remote_snapshot = cx.background().spawn({
                let client = self.client.clone();
                async move {
                    let mut share_tx = Some(share_tx);
                    let mut prev_snapshot = LocalSnapshot {
                        ignores_by_parent_abs_path: Default::default(),
                        git_repositories: Default::default(),
                        removed_entry_ids: Default::default(),
                        next_entry_id: Default::default(),
                        snapshot: Snapshot {
                            id: WorktreeId(worktree_id as usize),
                            abs_path: Path::new("").into(),
                            root_name: Default::default(),
                            root_char_bag: Default::default(),
                            entries_by_path: Default::default(),
                            entries_by_id: Default::default(),
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
            let version = deserialize_version(response.version);
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
        let summary = DiagnosticSummary {
            language_server_id: summary.language_server_id as usize,
            error_count: summary.error_count as usize,
            warning_count: summary.warning_count as usize,
        };
        if summary.is_empty() {
            self.diagnostic_summaries.remove(&PathKey(path));
        } else {
            self.diagnostic_summaries.insert(PathKey(path), summary);
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

    pub(crate) fn apply_remote_update(&mut self, update: proto::UpdateWorktree) -> Result<()> {
        let mut entries_by_path_edits = Vec::new();
        let mut entries_by_id_edits = Vec::new();
        for entry_id in update.removed_entries {
            let entry = self
                .entry_for_id(ProjectEntryId::from_proto(entry_id))
                .ok_or_else(|| anyhow!("unknown entry {}", entry_id))?;
            entries_by_path_edits.push(Edit::Remove(PathKey(entry.path.clone())));
            entries_by_id_edits.push(Edit::Remove(entry.id));
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

    pub fn root_entry(&self) -> Option<&Entry> {
        self.entry_for_path("")
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn scan_started(&mut self) {
        self.scan_id += 1;
    }

    pub fn scan_completed(&mut self) {
        self.completed_scan_id = self.scan_id;
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
    // Gives the most specific git repository for a given path
    pub(crate) fn repo_for(&self, path: &Path) -> Option<GitRepositoryEntry> {
        self.git_repositories
            .iter()
            .rev() //git_repository is ordered lexicographically
            .find(|repo| repo.manages(path))
            .cloned()
    }

    pub(crate) fn repo_with_dot_git_containing(
        &mut self,
        path: &Path,
    ) -> Option<&mut GitRepositoryEntry> {
        // Git repositories cannot be nested, so we don't need to reverse the order
        self.git_repositories
            .iter_mut()
            .find(|repo| repo.in_dot_git(path))
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

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            abs_path: self.abs_path().to_string_lossy().into(),
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries,
            scan_id: self.scan_id as u64,
            is_last_update: self.completed_scan_id == self.scan_id,
        }
    }

    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        if entry.is_file() && entry.path.file_name() == Some(&GITIGNORE) {
            let abs_path = self.abs_path.join(&entry.path);
            match smol::block_on(build_gitignore(&abs_path, fs)) {
                Ok(ignore) => {
                    self.ignores_by_parent_abs_path.insert(
                        abs_path.parent().unwrap().into(),
                        (Arc::new(ignore), self.scan_id),
                    );
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
        self.entries_by_path.insert_or_replace(entry.clone(), &());
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

        if let Some(ignore) = ignore {
            self.ignores_by_parent_abs_path.insert(
                self.abs_path.join(&parent_path).into(),
                (ignore, self.scan_id),
            );
        }
        if matches!(parent_entry.kind, EntryKind::PendingDir) {
            parent_entry.kind = EntryKind::Dir;
        } else {
            unreachable!();
        }

        if parent_path.file_name() == Some(&DOT_GIT) {
            let abs_path = self.abs_path.join(&parent_path);
            let content_path: Arc<Path> = parent_path.parent().unwrap().into();
            if let Err(ix) = self
                .git_repositories
                .binary_search_by_key(&&content_path, |repo| &repo.content_path)
            {
                if let Some(repo) = fs.open_repo(abs_path.as_path()) {
                    self.git_repositories.insert(
                        ix,
                        GitRepositoryEntry {
                            repo,
                            scan_id: 0,
                            content_path,
                            git_dir_path: parent_path,
                        },
                    );
                }
            }
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
            if let Some((_, scan_id)) = self
                .ignores_by_parent_abs_path
                .get_mut(abs_parent_path.as_path())
            {
                *scan_id = self.snapshot.scan_id;
            }
        } else if path.file_name() == Some(&DOT_GIT) {
            let parent_path = path.parent().unwrap();
            if let Ok(ix) = self
                .git_repositories
                .binary_search_by_key(&parent_path, |repo| repo.git_dir_path.as_ref())
            {
                self.git_repositories[ix].scan_id = self.snapshot.scan_id;
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

    pub fn git_repo_entries(&self) -> &[GitRepositoryEntry] {
        &self.git_repositories
    }
}

impl GitRepositoryEntry {
    // Note that these paths should be relative to the worktree root.
    pub(crate) fn manages(&self, path: &Path) -> bool {
        path.starts_with(self.content_path.as_ref())
    }

    // Note that this path should be relative to the worktree root.
    pub(crate) fn in_dot_git(&self, path: &Path) -> bool {
        path.starts_with(self.git_dir_path.as_ref())
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
        cx: &mut MutableAppContext,
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
    fs: Arc<dyn Fs>,
    snapshot: Mutex<LocalSnapshot>,
    notify: UnboundedSender<ScanState>,
    executor: Arc<executor::Background>,
}

impl BackgroundScanner {
    fn new(
        snapshot: LocalSnapshot,
        notify: UnboundedSender<ScanState>,
        fs: Arc<dyn Fs>,
        executor: Arc<executor::Background>,
    ) -> Self {
        Self {
            fs,
            snapshot: Mutex::new(snapshot),
            notify,
            executor,
        }
    }

    fn abs_path(&self) -> Arc<Path> {
        self.snapshot.lock().abs_path.clone()
    }

    async fn run(
        self,
        events_rx: impl Stream<Item = Vec<fsevent::Event>>,
        mut changed_paths: channel::Receiver<(Vec<PathBuf>, barrier::Sender)>,
    ) {
        use futures::FutureExt as _;

        // Retrieve the basic properties of the root node.
        let root_char_bag;
        let root_abs_path;
        let root_inode;
        let root_is_dir;
        let next_entry_id;
        {
            let mut snapshot = self.snapshot.lock();
            snapshot.scan_started();
            root_char_bag = snapshot.root_char_bag;
            root_abs_path = snapshot.abs_path.clone();
            root_inode = snapshot.root_entry().map(|e| e.inode);
            root_is_dir = snapshot.root_entry().map_or(false, |e| e.is_dir());
            next_entry_id = snapshot.next_entry_id.clone();
        }

        // Populate ignores above the root.
        let ignore_stack;
        for ancestor in root_abs_path.ancestors().skip(1) {
            if let Ok(ignore) = build_gitignore(&ancestor.join(&*GITIGNORE), self.fs.as_ref()).await
            {
                self.snapshot
                    .lock()
                    .ignores_by_parent_abs_path
                    .insert(ancestor.into(), (ignore.into(), 0));
            }
        }
        {
            let mut snapshot = self.snapshot.lock();
            ignore_stack = snapshot.ignore_stack_for_abs_path(&root_abs_path, true);
            if ignore_stack.is_all() {
                if let Some(mut root_entry) = snapshot.root_entry().cloned() {
                    root_entry.is_ignored = true;
                    snapshot.insert_entry(root_entry, self.fs.as_ref());
                }
            }
        };

        if root_is_dir {
            let mut ancestor_inodes = TreeSet::default();
            if let Some(root_inode) = root_inode {
                ancestor_inodes.insert(root_inode);
            }

            let (tx, rx) = channel::unbounded();
            self.executor
                .block(tx.send(ScanJob {
                    abs_path: root_abs_path.to_path_buf(),
                    path: Arc::from(Path::new("")),
                    ignore_stack,
                    ancestor_inodes,
                    scan_queue: tx.clone(),
                }))
                .unwrap();
            drop(tx);

            let progress_update_count = AtomicUsize::new(0);
            self.executor
                .scoped(|scope| {
                    for _ in 0..self.executor.num_cpus() {
                        scope.spawn(async {
                            let mut last_progress_update_count = 0;
                            let progress_update_timer = self.pause_between_progress_updates().fuse();
                            futures::pin_mut!(progress_update_timer);
                            loop {
                                select_biased! {
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
                                                if self
                                                    .notify
                                                    .unbounded_send(ScanState::Initializing {
                                                        snapshot: self.snapshot.lock().clone(),
                                                        barrier: None,
                                                    })
                                                    .is_err()
                                                {
                                                    break;
                                                }
                                            }
                                            Err(current_count) => last_progress_update_count = current_count,
                                        }
                                        progress_update_timer.set(self.pause_between_progress_updates().fuse());
                                    }

                                    // Refresh any paths requested by the main thread.
                                    job = changed_paths.recv().fuse() => {
                                        let Ok((abs_paths, barrier)) = job else { break };
                                        self.update_entries_for_paths(abs_paths, None).await;
                                        if self
                                            .notify
                                            .unbounded_send(ScanState::Initializing {
                                                snapshot: self.snapshot.lock().clone(),
                                                barrier: Some(barrier),
                                            })
                                            .is_err()
                                        {
                                            break;
                                        }
                                    }

                                    // Recursively load directories from the file system.
                                    job = rx.recv().fuse() => {
                                        let Ok(job) = job else { break };
                                        if let Err(err) = self
                                            .scan_dir(root_char_bag, next_entry_id.clone(), &job)
                                            .await
                                        {
                                            log::error!("error scanning {:?}: {}", job.abs_path, err);
                                        }
                                    }
                                }
                            }
                        });
                    }
                })
                .await;
        }

        self.snapshot.lock().scan_completed();

        if self
            .notify
            .unbounded_send(ScanState::Initialized {
                snapshot: self.snapshot.lock().clone(),
            })
            .is_err()
        {
            return;
        }

        // Process any events that occurred while performing the initial scan. These
        // events can't be reported as precisely, because there is no snapshot of the
        // worktree before they occurred.
        futures::pin_mut!(events_rx);
        if let Poll::Ready(Some(mut events)) = futures::poll!(events_rx.next()) {
            while let Poll::Ready(Some(additional_events)) = futures::poll!(events_rx.next()) {
                events.extend(additional_events);
            }
            let abs_paths = events.into_iter().map(|e| e.path).collect();
            if self.notify.unbounded_send(ScanState::Updating).is_err() {
                return;
            }
            if let Some(changes) = self.process_events(abs_paths, true).await {
                if self
                    .notify
                    .unbounded_send(ScanState::Updated {
                        snapshot: self.snapshot.lock().clone(),
                        changes,
                        barrier: None,
                    })
                    .is_err()
                {
                    return;
                }
            } else {
                return;
            }
        }

        // Continue processing events until the worktree is dropped.
        loop {
            let barrier;
            let abs_paths;
            select_biased! {
                request = changed_paths.next().fuse() => {
                    let Some((paths, b)) = request else { break };
                    abs_paths = paths;
                    barrier = Some(b);
                }
                events = events_rx.next().fuse() => {
                    let Some(events) = events else { break };
                    abs_paths = events.into_iter().map(|e| e.path).collect();
                    barrier = None;
                }
            }

            if self.notify.unbounded_send(ScanState::Updating).is_err() {
                return;
            }
            if let Some(changes) = self.process_events(abs_paths, false).await {
                if self
                    .notify
                    .unbounded_send(ScanState::Updated {
                        snapshot: self.snapshot.lock().clone(),
                        changes,
                        barrier,
                    })
                    .is_err()
                {
                    return;
                }
            } else {
                return;
            }
        }
    }

    async fn pause_between_progress_updates(&self) {
        #[cfg(any(test, feature = "test-support"))]
        if self.fs.is_fake() {
            return self.executor.simulate_random_delay().await;
        }
        smol::Timer::after(Duration::from_millis(100)).await;
    }

    async fn scan_dir(
        &self,
        root_char_bag: CharBag,
        next_entry_id: Arc<AtomicUsize>,
        job: &ScanJob,
    ) -> Result<()> {
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();
        let mut ignore_stack = job.ignore_stack.clone();
        let mut new_ignore = None;

        let mut child_paths = self.fs.read_dir(&job.abs_path).await?;
        while let Some(child_abs_path) = child_paths.next().await {
            let child_abs_path = match child_abs_path {
                Ok(child_abs_path) => child_abs_path,
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
                        ignore_stack =
                            ignore_stack.append(job.abs_path.as_path().into(), ignore.clone());
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
                    let entry_abs_path = self.abs_path().join(&entry.path);
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

    async fn process_events(
        &self,
        abs_paths: Vec<PathBuf>,
        received_before_initialized: bool,
    ) -> Option<HashMap<Arc<Path>, PathChange>> {
        let (scan_queue_tx, scan_queue_rx) = channel::unbounded();

        let prev_snapshot = {
            let mut snapshot = self.snapshot.lock();
            snapshot.scan_started();
            snapshot.clone()
        };

        let event_paths = self
            .update_entries_for_paths(abs_paths, Some(scan_queue_tx))
            .await?;

        // Scan any directories that were created as part of this event batch.
        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        while let Ok(job) = scan_queue_rx.recv().await {
                            if let Err(err) = self
                                .scan_dir(
                                    prev_snapshot.root_char_bag,
                                    prev_snapshot.next_entry_id.clone(),
                                    &job,
                                )
                                .await
                            {
                                log::error!("error scanning {:?}: {}", job.abs_path, err);
                            }
                        }
                    });
                }
            })
            .await;

        // Attempt to detect renames only over a single batch of file-system events.
        self.snapshot.lock().removed_entry_ids.clear();

        self.update_ignore_statuses().await;
        self.update_git_repositories();
        let changes = self.build_change_set(
            prev_snapshot.snapshot,
            event_paths,
            received_before_initialized,
        );
        self.snapshot.lock().scan_completed();
        Some(changes)
    }

    async fn update_entries_for_paths(
        &self,
        mut abs_paths: Vec<PathBuf>,
        scan_queue_tx: Option<Sender<ScanJob>>,
    ) -> Option<Vec<Arc<Path>>> {
        abs_paths.sort_unstable();
        abs_paths.dedup_by(|a, b| a.starts_with(&b));

        let root_abs_path = self.snapshot.lock().abs_path.clone();
        let root_canonical_path = self.fs.canonicalize(&root_abs_path).await.ok()?;
        let metadata = futures::future::join_all(
            abs_paths
                .iter()
                .map(|abs_path| self.fs.metadata(&abs_path))
                .collect::<Vec<_>>(),
        )
        .await;

        let mut snapshot = self.snapshot.lock();
        if scan_queue_tx.is_some() {
            for abs_path in &abs_paths {
                if let Ok(path) = abs_path.strip_prefix(&root_canonical_path) {
                    snapshot.remove_path(path);
                }
            }
        }

        let mut event_paths = Vec::with_capacity(abs_paths.len());
        for (abs_path, metadata) in abs_paths.into_iter().zip(metadata.into_iter()) {
            let path: Arc<Path> = match abs_path.strip_prefix(&root_canonical_path) {
                Ok(path) => Arc::from(path.to_path_buf()),
                Err(_) => {
                    log::error!(
                        "unexpected event {:?} for root path {:?}",
                        abs_path,
                        root_canonical_path
                    );
                    continue;
                }
            };
            event_paths.push(path.clone());
            let abs_path = root_abs_path.join(&path);

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

                    let scan_id = snapshot.scan_id;
                    if let Some(repo) = snapshot.repo_with_dot_git_containing(&path) {
                        repo.repo.lock().reload_index();
                        repo.scan_id = scan_id;
                    }

                    if let Some(scan_queue_tx) = &scan_queue_tx {
                        let mut ancestor_inodes = snapshot.ancestor_inodes_for_path(&path);
                        if metadata.is_dir && !ancestor_inodes.contains(&metadata.inode) {
                            ancestor_inodes.insert(metadata.inode);
                            self.executor
                                .block(scan_queue_tx.send(ScanJob {
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
                Ok(None) => {}
                Err(err) => {
                    // TODO - create a special 'error' entry in the entries tree to mark this
                    log::error!("error reading file on event {:?}", err);
                }
            }
        }

        Some(event_paths)
    }

    async fn update_ignore_statuses(&self) {
        let mut snapshot = self.snapshot.lock().clone();
        let mut ignores_to_update = Vec::new();
        let mut ignores_to_delete = Vec::new();
        for (parent_abs_path, (_, scan_id)) in &snapshot.ignores_by_parent_abs_path {
            if let Ok(parent_path) = parent_abs_path.strip_prefix(&snapshot.abs_path) {
                if *scan_id == snapshot.scan_id && snapshot.entry_for_path(parent_path).is_some() {
                    ignores_to_update.push(parent_abs_path.clone());
                }

                let ignore_path = parent_path.join(&*GITIGNORE);
                if snapshot.entry_for_path(ignore_path).is_none() {
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
            ignore_queue_tx
                .send(UpdateIgnoreStatusJob {
                    abs_path: parent_abs_path,
                    ignore_stack,
                    ignore_queue: ignore_queue_tx.clone(),
                })
                .await
                .unwrap();
        }
        drop(ignore_queue_tx);

        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        while let Ok(job) = ignore_queue_rx.recv().await {
                            self.update_ignore_status(job, &snapshot).await;
                        }
                    });
                }
            })
            .await;
    }

    fn update_git_repositories(&self) {
        let mut snapshot = self.snapshot.lock();
        let mut git_repositories = mem::take(&mut snapshot.git_repositories);
        git_repositories.retain(|repo| snapshot.entry_for_path(&repo.git_dir_path).is_some());
        snapshot.git_repositories = git_repositories;
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
            let abs_path = self.abs_path().join(&entry.path);
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
        old_snapshot: Snapshot,
        event_paths: Vec<Arc<Path>>,
        received_before_initialized: bool,
    ) -> HashMap<Arc<Path>, PathChange> {
        use PathChange::{Added, AddedOrUpdated, Removed, Updated};

        let new_snapshot = self.snapshot.lock();
        let mut changes = HashMap::default();
        let mut old_paths = old_snapshot.entries_by_path.cursor::<PathKey>();
        let mut new_paths = new_snapshot.entries_by_path.cursor::<PathKey>();

        for path in event_paths {
            let path = PathKey(path);
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
                                changes.insert(old_entry.path.clone(), Removed);
                                old_paths.next(&());
                            }
                            Ordering::Equal => {
                                if received_before_initialized {
                                    // If the worktree was not fully initialized when this event was generated,
                                    // we can't know whether this entry was added during the scan or whether
                                    // it was merely updated.
                                    changes.insert(old_entry.path.clone(), AddedOrUpdated);
                                } else if old_entry.mtime != new_entry.mtime {
                                    changes.insert(old_entry.path.clone(), Updated);
                                }
                                old_paths.next(&());
                                new_paths.next(&());
                            }
                            Ordering::Greater => {
                                changes.insert(new_entry.path.clone(), Added);
                                new_paths.next(&());
                            }
                        }
                    }
                    (Some(old_entry), None) => {
                        changes.insert(old_entry.path.clone(), Removed);
                        old_paths.next(&());
                    }
                    (None, Some(new_entry)) => {
                        changes.insert(new_entry.path.clone(), Added);
                        new_paths.next(&());
                    }
                    (None, None) => break,
                }
            }
        }
        changes
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
    abs_path: PathBuf,
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
        self.advance_to_offset(self.offset() + 1)
    }

    pub fn advance_to_offset(&mut self, offset: usize) -> bool {
        self.cursor.seek_forward(
            &TraversalTarget::Count {
                count: offset,
                include_dirs: self.include_dirs,
                include_ignored: self.include_ignored,
            },
            Bias::Right,
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

    pub fn offset(&self) -> usize {
        self.cursor
            .start()
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
    use fs::repository::FakeGitRepository;
    use fs::{FakeFs, RealFs};
    use gpui::{executor::Deterministic, TestAppContext};
    use rand::prelude::*;
    use serde_json::json;
    use std::{env, fmt::Write};
    use util::http::FakeHttpClient;

    use util::test::temp_tree;

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
            "c.txt": "",
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

            let repo = tree.repo_for("dir1/src/b.txt".as_ref()).unwrap();
            assert_eq!(repo.content_path.as_ref(), Path::new("dir1"));
            assert_eq!(repo.git_dir_path.as_ref(), Path::new("dir1/.git"));

            let repo = tree.repo_for("dir1/deps/dep1/src/a.txt".as_ref()).unwrap();
            assert_eq!(repo.content_path.as_ref(), Path::new("dir1/deps/dep1"));
            assert_eq!(repo.git_dir_path.as_ref(), Path::new("dir1/deps/dep1/.git"),);
        });

        let original_scan_id = tree.read_with(cx, |tree, _cx| {
            let tree = tree.as_local().unwrap();
            tree.repo_for("dir1/src/b.txt".as_ref()).unwrap().scan_id
        });

        std::fs::write(root.path().join("dir1/.git/random_new_file"), "hello").unwrap();
        tree.flush_fs_events(cx).await;

        tree.read_with(cx, |tree, _cx| {
            let tree = tree.as_local().unwrap();
            let new_scan_id = tree.repo_for("dir1/src/b.txt".as_ref()).unwrap().scan_id;
            assert_ne!(
                original_scan_id, new_scan_id,
                "original {original_scan_id}, new {new_scan_id}"
            );
        });

        std::fs::remove_dir_all(root.path().join("dir1/.git")).unwrap();
        tree.flush_fs_events(cx).await;

        tree.read_with(cx, |tree, _cx| {
            let tree = tree.as_local().unwrap();

            assert!(tree.repo_for("dir1/src/b.txt".as_ref()).is_none());
        });
    }

    #[test]
    fn test_changed_repos() {
        fn fake_entry(git_dir_path: impl AsRef<Path>, scan_id: usize) -> GitRepositoryEntry {
            GitRepositoryEntry {
                repo: Arc::new(Mutex::new(FakeGitRepository::default())),
                scan_id,
                content_path: git_dir_path.as_ref().parent().unwrap().into(),
                git_dir_path: git_dir_path.as_ref().into(),
            }
        }

        let prev_repos: Vec<GitRepositoryEntry> = vec![
            fake_entry("/.git", 0),
            fake_entry("/a/.git", 0),
            fake_entry("/a/b/.git", 0),
        ];

        let new_repos: Vec<GitRepositoryEntry> = vec![
            fake_entry("/a/.git", 1),
            fake_entry("/a/b/.git", 0),
            fake_entry("/a/c/.git", 0),
        ];

        let res = LocalWorktree::changed_repos(&prev_repos, &new_repos);

        // Deletion retained
        assert!(res
            .iter()
            .find(|repo| repo.git_dir_path.as_ref() == Path::new("/.git") && repo.scan_id == 0)
            .is_some());

        // Update retained
        assert!(res
            .iter()
            .find(|repo| repo.git_dir_path.as_ref() == Path::new("/a/.git") && repo.scan_id == 1)
            .is_some());

        // Addition retained
        assert!(res
            .iter()
            .find(|repo| repo.git_dir_path.as_ref() == Path::new("/a/c/.git") && repo.scan_id == 0)
            .is_some());

        // Nochange, not retained
        assert!(res
            .iter()
            .find(|repo| repo.git_dir_path.as_ref() == Path::new("/a/b/.git") && repo.scan_id == 0)
            .is_none());
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
    async fn test_create_directory(cx: &mut TestAppContext) {
        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/a",
            json!({
                "b": {},
                "c": {},
                "d": {},
            }),
        )
        .await;

        let tree = Worktree::local(
            client,
            "/a".as_ref(),
            true,
            fs,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

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
            randomly_mutate_tree(&fs, root_dir, 1.0, &mut rng).await;
        }
        log::info!("generated initial tree");

        let next_entry_id = Arc::new(AtomicUsize::default());
        let client = cx.read(|cx| Client::new(FakeHttpClient::with_404_response(), cx));
        let worktree = Worktree::local(
            client.clone(),
            root_dir,
            true,
            fs.clone(),
            next_entry_id.clone(),
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
                    for (path, change_type) in changes.iter() {
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

        let mut snapshots = Vec::new();
        let mut mutations_len = operations;
        while mutations_len > 1 {
            randomly_mutate_tree(&fs, root_dir, 1.0, &mut rng).await;
            let buffered_event_count = fs.as_fake().buffered_event_count().await;
            if buffered_event_count > 0 && rng.gen_bool(0.3) {
                let len = rng.gen_range(0..=buffered_event_count);
                log::info!("flushing {} events", len);
                fs.as_fake().flush_events(len).await;
            } else {
                randomly_mutate_tree(&fs, root_dir, 0.6, &mut rng).await;
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
        fs.as_fake().flush_events(usize::MAX).await;
        cx.foreground().run_until_parked();
        let snapshot = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
        snapshot.check_invariants();

        {
            let new_worktree = Worktree::local(
                client.clone(),
                root_dir,
                true,
                fs.clone(),
                next_entry_id,
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

    async fn randomly_mutate_tree(
        fs: &Arc<dyn Fs>,
        root_path: &Path,
        insertion_probability: f64,
        rng: &mut impl Rng,
    ) {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        for path in fs.as_fake().paths().await {
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
