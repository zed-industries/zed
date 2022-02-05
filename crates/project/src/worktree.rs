use super::{
    fs::{self, Fs},
    ignore::IgnoreStack,
    DiagnosticSummary,
};
use crate::LoadOptions;
use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{anyhow, Result};
use client::{proto, Client, TypedEnvelope};
use clock::ReplicaId;
use collections::HashMap;
use futures::{Stream, StreamExt};
use fuzzy::CharBag;
use gpui::{
    executor, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task,
};
use language::{
    Anchor, Buffer, Completion, DiagnosticEntry, Language, Operation, PointUtf16, Rope,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::{
    prelude::{Sink as _, Stream as _},
    watch,
};
use serde::Deserialize;
use smol::channel::{self, Sender};
use std::{
    any::Any,
    cmp::{self, Ordering},
    convert::{TryFrom, TryInto},
    ffi::{OsStr, OsString},
    fmt,
    future::Future,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, SystemTime},
};
use sum_tree::{Bias, Edit, SeekTarget, SumTree, TreeMap};
use util::ResultExt;

lazy_static! {
    static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct WorktreeId(usize);

pub enum Worktree {
    Local(LocalWorktree),
    Remote(RemoteWorktree),
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    config: WorktreeConfig,
    background_snapshot: Arc<Mutex<LocalSnapshot>>,
    last_scan_state_rx: watch::Receiver<ScanState>,
    _background_scanner_task: Option<Task<()>>,
    poll_task: Option<Task<()>>,
    registration: Registration,
    share: Option<ShareState>,
    diagnostics: HashMap<Arc<Path>, Vec<DiagnosticEntry<PointUtf16>>>,
    diagnostic_summaries: TreeMap<PathKey, DiagnosticSummary>,
    queued_operations: Vec<(u64, Operation)>,
    client: Arc<Client>,
    fs: Arc<dyn Fs>,
    weak: bool,
}

pub struct RemoteWorktree {
    pub(crate) snapshot: Snapshot,
    project_id: u64,
    snapshot_rx: watch::Receiver<Snapshot>,
    client: Arc<Client>,
    updates_tx: postage::mpsc::Sender<proto::UpdateWorktree>,
    replica_id: ReplicaId,
    queued_operations: Vec<(u64, Operation)>,
    diagnostic_summaries: TreeMap<PathKey, DiagnosticSummary>,
    weak: bool,
}

#[derive(Clone)]
pub struct Snapshot {
    id: WorktreeId,
    root_name: String,
    root_char_bag: CharBag,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
}

#[derive(Clone)]
pub struct LocalSnapshot {
    abs_path: Arc<Path>,
    scan_id: usize,
    ignores: HashMap<Arc<Path>, (Arc<Gitignore>, usize)>,
    removed_entry_ids: HashMap<u64, usize>,
    next_entry_id: Arc<AtomicUsize>,
    snapshot: Snapshot,
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

#[derive(Clone, Debug)]
enum ScanState {
    Idle,
    Scanning,
    Err(Arc<anyhow::Error>),
}

#[derive(Debug, Eq, PartialEq)]
enum Registration {
    None,
    Pending,
    Done { project_id: u64 },
}

struct ShareState {
    project_id: u64,
    snapshots_tx: Sender<LocalSnapshot>,
    _maintain_remote_snapshot: Option<Task<()>>,
}

#[derive(Default, Deserialize)]
struct WorktreeConfig {
    collaborators: Vec<String>,
}

pub enum Event {
    UpdatedEntries,
}

impl Entity for Worktree {
    type Event = Event;

    fn release(&mut self, cx: &mut MutableAppContext) {
        if let Some(worktree) = self.as_local_mut() {
            if let Registration::Done { project_id } = worktree.registration {
                let client = worktree.client.clone();
                let unregister_message = proto::UnregisterWorktree {
                    project_id,
                    worktree_id: worktree.id().to_proto(),
                };
                cx.foreground()
                    .spawn(async move {
                        client.send(unregister_message).await?;
                        Ok::<_, anyhow::Error>(())
                    })
                    .detach_and_log_err(cx);
            }
        }
    }
}

impl Worktree {
    pub async fn local(
        client: Arc<Client>,
        path: impl Into<Arc<Path>>,
        weak: bool,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let (tree, scan_states_tx) = LocalWorktree::new(client, path, weak, fs.clone(), cx).await?;
        tree.update(cx, |tree, cx| {
            let tree = tree.as_local_mut().unwrap();
            let abs_path = tree.abs_path().clone();
            let background_snapshot = tree.background_snapshot.clone();
            let background = cx.background().clone();
            tree._background_scanner_task = Some(cx.background().spawn(async move {
                let events = fs.watch(&abs_path, Duration::from_millis(100)).await;
                let scanner =
                    BackgroundScanner::new(background_snapshot, scan_states_tx, fs, background);
                scanner.run(events).await;
            }));
        });
        Ok(tree)
    }

    pub fn remote(
        project_remote_id: u64,
        replica_id: ReplicaId,
        worktree: proto::Worktree,
        client: Arc<Client>,
        cx: &mut MutableAppContext,
    ) -> (ModelHandle<Self>, Task<()>) {
        let remote_id = worktree.id;
        let root_char_bag: CharBag = worktree
            .root_name
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let root_name = worktree.root_name.clone();
        let weak = worktree.weak;
        let snapshot = Snapshot {
            id: WorktreeId(remote_id as usize),
            root_name,
            root_char_bag,
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
        };

        let (updates_tx, mut updates_rx) = postage::mpsc::channel(64);
        let (mut snapshot_tx, snapshot_rx) = watch::channel_with(snapshot.clone());
        let worktree_handle = cx.add_model(|_: &mut ModelContext<Worktree>| {
            Worktree::Remote(RemoteWorktree {
                project_id: project_remote_id,
                replica_id,
                snapshot: snapshot.clone(),
                snapshot_rx: snapshot_rx.clone(),
                updates_tx,
                client: client.clone(),
                queued_operations: Default::default(),
                diagnostic_summaries: TreeMap::from_ordered_entries(
                    worktree.diagnostic_summaries.into_iter().map(|summary| {
                        (
                            PathKey(PathBuf::from(summary.path).into()),
                            DiagnosticSummary {
                                error_count: summary.error_count as usize,
                                warning_count: summary.warning_count as usize,
                                info_count: summary.info_count as usize,
                                hint_count: summary.hint_count as usize,
                            },
                        )
                    }),
                ),
                weak,
            })
        });

        let deserialize_task = cx.spawn({
            let worktree_handle = worktree_handle.clone();
            |cx| async move {
                let (entries_by_path, entries_by_id) = cx
                    .background()
                    .spawn(async move {
                        let mut entries_by_path_edits = Vec::new();
                        let mut entries_by_id_edits = Vec::new();
                        for entry in worktree.entries {
                            match Entry::try_from((&root_char_bag, entry)) {
                                Ok(entry) => {
                                    entries_by_id_edits.push(Edit::Insert(PathEntry {
                                        id: entry.id,
                                        path: entry.path.clone(),
                                        is_ignored: entry.is_ignored,
                                        scan_id: 0,
                                    }));
                                    entries_by_path_edits.push(Edit::Insert(entry));
                                }
                                Err(err) => log::warn!("error for remote worktree entry {:?}", err),
                            }
                        }

                        let mut entries_by_path = SumTree::new();
                        let mut entries_by_id = SumTree::new();
                        entries_by_path.edit(entries_by_path_edits, &());
                        entries_by_id.edit(entries_by_id_edits, &());

                        (entries_by_path, entries_by_id)
                    })
                    .await;

                {
                    let mut snapshot = snapshot_tx.borrow_mut();
                    snapshot.entries_by_path = entries_by_path;
                    snapshot.entries_by_id = entries_by_id;
                }

                cx.background()
                    .spawn(async move {
                        while let Some(update) = updates_rx.recv().await {
                            let mut snapshot = snapshot_tx.borrow().clone();
                            if let Err(error) = snapshot.apply_remote_update(update) {
                                log::error!("error applying worktree update: {}", error);
                            }
                            *snapshot_tx.borrow_mut() = snapshot;
                        }
                    })
                    .detach();

                {
                    let mut snapshot_rx = snapshot_rx.clone();
                    let this = worktree_handle.downgrade();
                    cx.spawn(|mut cx| async move {
                        while let Some(_) = snapshot_rx.recv().await {
                            if let Some(this) = cx.read(|cx| this.upgrade(cx)) {
                                this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
                            } else {
                                break;
                            }
                        }
                    })
                    .detach();
                }
            }
        });
        (worktree_handle, deserialize_task)
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

    pub fn snapshot(&self) -> Snapshot {
        match self {
            Worktree::Local(worktree) => worktree.snapshot().snapshot,
            Worktree::Remote(worktree) => worktree.snapshot(),
        }
    }

    pub fn is_weak(&self) -> bool {
        match self {
            Worktree::Local(worktree) => worktree.weak,
            Worktree::Remote(worktree) => worktree.weak,
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match self {
            Worktree::Local(_) => 0,
            Worktree::Remote(worktree) => worktree.replica_id,
        }
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Arc<Path>, DiagnosticSummary)> + 'a {
        match self {
            Worktree::Local(worktree) => &worktree.diagnostic_summaries,
            Worktree::Remote(worktree) => &worktree.diagnostic_summaries,
        }
        .iter()
        .map(|(path, summary)| (path.0.clone(), summary.clone()))
    }

    fn poll_snapshot(&mut self, cx: &mut ModelContext<Self>) {
        match self {
            Self::Local(worktree) => {
                let is_fake_fs = worktree.fs.is_fake();
                worktree.snapshot = worktree.background_snapshot.lock().clone();
                if worktree.is_scanning() {
                    if worktree.poll_task.is_none() {
                        worktree.poll_task = Some(cx.spawn(|this, mut cx| async move {
                            if is_fake_fs {
                                smol::future::yield_now().await;
                            } else {
                                smol::Timer::after(Duration::from_millis(100)).await;
                            }
                            this.update(&mut cx, |this, cx| {
                                this.as_local_mut().unwrap().poll_task = None;
                                this.poll_snapshot(cx);
                            })
                        }));
                    }
                } else {
                    worktree.poll_task.take();
                    cx.emit(Event::UpdatedEntries);
                }
            }
            Self::Remote(worktree) => {
                worktree.snapshot = worktree.snapshot_rx.borrow().clone();
                cx.emit(Event::UpdatedEntries);
            }
        };

        cx.notify();
    }

    fn send_buffer_update(
        &mut self,
        buffer_id: u64,
        operation: Operation,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some((project_id, rpc)) = match self {
            Worktree::Local(worktree) => worktree
                .share
                .as_ref()
                .map(|share| (share.project_id, worktree.client.clone())),
            Worktree::Remote(worktree) => Some((worktree.project_id, worktree.client.clone())),
        } {
            cx.spawn(|worktree, mut cx| async move {
                if let Err(error) = rpc
                    .request(proto::UpdateBuffer {
                        project_id,
                        buffer_id,
                        operations: vec![language::proto::serialize_operation(&operation)],
                    })
                    .await
                {
                    worktree.update(&mut cx, |worktree, _| {
                        log::error!("error sending buffer operation: {}", error);
                        match worktree {
                            Worktree::Local(t) => &mut t.queued_operations,
                            Worktree::Remote(t) => &mut t.queued_operations,
                        }
                        .push((buffer_id, operation));
                    });
                }
            })
            .detach();
        }
    }
}

impl LocalWorktree {
    async fn new(
        client: Arc<Client>,
        path: impl Into<Arc<Path>>,
        weak: bool,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncAppContext,
    ) -> Result<(ModelHandle<Worktree>, Sender<ScanState>)> {
        let abs_path = path.into();
        let path: Arc<Path> = Arc::from(Path::new(""));
        let next_entry_id = AtomicUsize::new(0);

        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());
        let root_char_bag = root_name.chars().map(|c| c.to_ascii_lowercase()).collect();
        let metadata = fs.metadata(&abs_path).await?;

        let mut config = WorktreeConfig::default();
        if let Ok(zed_toml) = fs.load(&abs_path.join(".zed.toml")).await {
            if let Ok(parsed) = toml::from_str(&zed_toml) {
                config = parsed;
            }
        }

        let (scan_states_tx, scan_states_rx) = smol::channel::unbounded();
        let (mut last_scan_state_tx, last_scan_state_rx) = watch::channel_with(ScanState::Scanning);
        let tree = cx.add_model(move |cx: &mut ModelContext<Worktree>| {
            let mut snapshot = LocalSnapshot {
                abs_path,
                scan_id: 0,
                ignores: Default::default(),
                removed_entry_ids: Default::default(),
                next_entry_id: Arc::new(next_entry_id),
                snapshot: Snapshot {
                    id: WorktreeId::from_usize(cx.model_id()),
                    root_name: root_name.clone(),
                    root_char_bag,
                    entries_by_path: Default::default(),
                    entries_by_id: Default::default(),
                },
            };
            if let Some(metadata) = metadata {
                let entry = Entry::new(
                    path.into(),
                    &metadata,
                    &snapshot.next_entry_id,
                    snapshot.root_char_bag,
                );
                snapshot.insert_entry(entry, fs.as_ref());
            }

            let tree = Self {
                snapshot: snapshot.clone(),
                config,
                background_snapshot: Arc::new(Mutex::new(snapshot)),
                last_scan_state_rx,
                _background_scanner_task: None,
                registration: Registration::None,
                share: None,
                poll_task: None,
                diagnostics: Default::default(),
                diagnostic_summaries: Default::default(),
                queued_operations: Default::default(),
                client,
                fs,
                weak,
            };

            cx.spawn_weak(|this, mut cx| async move {
                while let Ok(scan_state) = scan_states_rx.recv().await {
                    if let Some(handle) = cx.read(|cx| this.upgrade(cx)) {
                        let to_send = handle.update(&mut cx, |this, cx| {
                            last_scan_state_tx.blocking_send(scan_state).ok();
                            this.poll_snapshot(cx);
                            let tree = this.as_local_mut().unwrap();
                            if !tree.is_scanning() {
                                if let Some(share) = tree.share.as_ref() {
                                    return Some((tree.snapshot(), share.snapshots_tx.clone()));
                                }
                            }
                            None
                        });

                        if let Some((snapshot, snapshots_to_send_tx)) = to_send {
                            if let Err(err) = snapshots_to_send_tx.send(snapshot).await {
                                log::error!("error submitting snapshot to send {}", err);
                            }
                        }
                    } else {
                        break;
                    }
                }
            })
            .detach();

            Worktree::Local(tree)
        });

        Ok((tree, scan_states_tx))
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

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

    pub fn authorized_logins(&self) -> Vec<String> {
        self.config.collaborators.clone()
    }

    pub(crate) fn load_buffer_with_options(
        &mut self,
        path: &Path,
        options: LoadOptions,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let path = Arc::from(path);
        cx.spawn(move |this, mut cx| async move {
            let (file, contents) = this
                .update(&mut cx, |t, cx| {
                    t.as_local().unwrap().load_with_options(&path, options, cx)
                })
                .await?;
            Ok(cx.add_model(|cx| Buffer::from_file(0, contents, Box::new(file), cx)))
        })
    }

    pub(crate) fn create_dir(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        let path = Arc::from(path);
        cx.spawn(move |this, mut cx| async move {
            this.update(&mut cx, |t, cx| {
                t.as_local().unwrap().create_dir_task(&path, cx)
            })
            .await?;
            Ok(())
        })
    }

    pub fn diagnostics_for_path(&self, path: &Path) -> Option<Vec<DiagnosticEntry<PointUtf16>>> {
        self.diagnostics.get(path).cloned()
    }

    pub fn update_diagnostics(
        &mut self,
        worktree_path: Arc<Path>,
        diagnostics: Vec<DiagnosticEntry<PointUtf16>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let summary = DiagnosticSummary::new(&diagnostics);
        self.diagnostic_summaries
            .insert(PathKey(worktree_path.clone()), summary.clone());
        self.diagnostics.insert(worktree_path.clone(), diagnostics);

        if let Some(share) = self.share.as_ref() {
            cx.foreground()
                .spawn({
                    let client = self.client.clone();
                    let project_id = share.project_id;
                    let worktree_id = self.id().to_proto();
                    let path = worktree_path.to_string_lossy().to_string();
                    async move {
                        client
                            .send(proto::UpdateDiagnosticSummary {
                                project_id,
                                worktree_id,
                                summary: Some(proto::DiagnosticSummary {
                                    path,
                                    error_count: summary.error_count as u32,
                                    warning_count: summary.warning_count as u32,
                                    info_count: summary.info_count as u32,
                                    hint_count: summary.hint_count as u32,
                                }),
                            })
                            .await
                            .log_err()
                    }
                })
                .detach();
        }

        Ok(())
    }

    pub fn scan_complete(&self) -> impl Future<Output = ()> {
        let mut scan_state_rx = self.last_scan_state_rx.clone();
        async move {
            let mut scan_state = Some(scan_state_rx.borrow().clone());
            while let Some(ScanState::Scanning) = scan_state {
                scan_state = scan_state_rx.recv().await;
            }
        }
    }

    fn is_scanning(&self) -> bool {
        if let ScanState::Scanning = *self.last_scan_state_rx.borrow() {
            true
        } else {
            false
        }
    }

    pub fn snapshot(&self) -> LocalSnapshot {
        self.snapshot.clone()
    }

    fn load_with_options(
        &self,
        path: &Path,
        options: LoadOptions,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<(File, String)>> {
        let handle = cx.handle();
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let background_snapshot = self.background_snapshot.clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let text = fs.load_with_options(&abs_path, options).await?;
            // Eagerly populate the snapshot with an updated entry for the loaded file
            let entry = refresh_entry(fs.as_ref(), &background_snapshot, path, &abs_path).await?;
            this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
            Ok((
                File {
                    entry_id: Some(entry.id),
                    worktree: handle,
                    path: entry.path,
                    mtime: entry.mtime,
                    is_local: true,
                },
                text,
            ))
        })
    }

    fn create_dir_task(&self, path: &Path, cx: &mut ModelContext<Worktree>) -> Task<Result<()>> {
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let fs = self.fs.clone();
        cx.spawn(|_, _| async move {
            fs.create_dir(&abs_path).await?;
            Ok(())
        })
    }

    pub fn save_buffer_as(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<()>> {
        let buffer = buffer_handle.read(cx);
        let text = buffer.as_rope().clone();
        let version = buffer.version();
        let save = self.save(path, text, cx);
        let handle = cx.handle();
        cx.as_mut().spawn(|mut cx| async move {
            let entry = save.await?;
            let file = File {
                entry_id: Some(entry.id),
                worktree: handle,
                path: entry.path,
                mtime: entry.mtime,
                is_local: true,
            };

            buffer_handle.update(&mut cx, |buffer, cx| {
                buffer.did_save(version, file.mtime, Some(Box::new(file)), cx);
            });

            Ok(())
        })
    }

    fn save(
        &self,
        path: impl Into<Arc<Path>>,
        text: Rope,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let path = path.into();
        let abs_path = self.absolutize(&path);
        let background_snapshot = self.background_snapshot.clone();
        let fs = self.fs.clone();
        let save = cx.background().spawn(async move {
            fs.save(&abs_path, &text).await?;
            refresh_entry(fs.as_ref(), &background_snapshot, path.clone(), &abs_path).await
        });

        cx.spawn(|this, mut cx| async move {
            let entry = save.await?;
            this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
            Ok(entry)
        })
    }

    pub fn register(
        &mut self,
        project_id: u64,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<anyhow::Result<()>> {
        if self.registration != Registration::None {
            return Task::ready(Ok(()));
        }

        self.registration = Registration::Pending;
        let client = self.client.clone();
        let register_message = proto::RegisterWorktree {
            project_id,
            worktree_id: self.id().to_proto(),
            root_name: self.root_name().to_string(),
            authorized_logins: self.authorized_logins(),
        };
        cx.spawn(|this, mut cx| async move {
            let response = client.request(register_message).await;
            this.update(&mut cx, |this, _| {
                let worktree = this.as_local_mut().unwrap();
                match response {
                    Ok(_) => {
                        worktree.registration = Registration::Done { project_id };
                        Ok(())
                    }
                    Err(error) => {
                        worktree.registration = Registration::None;
                        Err(error)
                    }
                }
            })
        })
    }

    pub fn share(
        &mut self,
        project_id: u64,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<anyhow::Result<()>> {
        if self.share.is_some() {
            return Task::ready(Ok(()));
        }

        let snapshot = self.snapshot();
        let rpc = self.client.clone();
        let worktree_id = cx.model_id() as u64;
        let (snapshots_to_send_tx, snapshots_to_send_rx) =
            smol::channel::unbounded::<LocalSnapshot>();
        let maintain_remote_snapshot = cx.background().spawn({
            let rpc = rpc.clone();
            let snapshot = snapshot.clone();
            async move {
                let mut prev_snapshot = snapshot;
                while let Ok(snapshot) = snapshots_to_send_rx.recv().await {
                    let message =
                        snapshot.build_update(&prev_snapshot, project_id, worktree_id, false);
                    match rpc.send(message).await {
                        Ok(()) => prev_snapshot = snapshot,
                        Err(err) => log::error!("error sending snapshot diff {}", err),
                    }
                }
            }
        });
        self.share = Some(ShareState {
            project_id,
            snapshots_tx: snapshots_to_send_tx,
            _maintain_remote_snapshot: Some(maintain_remote_snapshot),
        });

        let diagnostic_summaries = self.diagnostic_summaries.clone();
        let weak = self.weak;
        let share_message = cx.background().spawn(async move {
            proto::ShareWorktree {
                project_id,
                worktree: Some(snapshot.to_proto(&diagnostic_summaries, weak)),
            }
        });

        cx.foreground().spawn(async move {
            rpc.request(share_message.await).await?;
            Ok(())
        })
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

    pub fn update_from_remote(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let mut tx = self.updates_tx.clone();
        let payload = envelope.payload.clone();
        cx.background()
            .spawn(async move {
                tx.send(payload).await.expect("receiver runs to completion");
            })
            .detach();

        Ok(())
    }

    pub fn update_diagnostic_summary(
        &mut self,
        path: Arc<Path>,
        summary: &proto::DiagnosticSummary,
    ) {
        self.diagnostic_summaries.insert(
            PathKey(path.clone()),
            DiagnosticSummary {
                error_count: summary.error_count as usize,
                warning_count: summary.warning_count as usize,
                info_count: summary.info_count as usize,
                hint_count: summary.hint_count as usize,
            },
        );
    }
}

impl Snapshot {
    pub fn id(&self) -> WorktreeId {
        self.id
    }

    pub(crate) fn to_proto(
        &self,
        diagnostic_summaries: &TreeMap<PathKey, DiagnosticSummary>,
        weak: bool,
    ) -> proto::Worktree {
        let root_name = self.root_name.clone();
        proto::Worktree {
            id: self.id.0 as u64,
            root_name,
            entries: self
                .entries_by_path
                .iter()
                .filter(|e| !e.is_ignored)
                .map(Into::into)
                .collect(),
            diagnostic_summaries: diagnostic_summaries
                .iter()
                .map(|(path, summary)| summary.to_proto(path.0.clone()))
                .collect(),
            weak,
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
                            removed_entries.push(other_entry.id as u64);
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
                    removed_entries.push(other_entry.id as u64);
                    other_entries.next();
                }
                (None, None) => break,
            }
        }

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries,
        }
    }

    pub(crate) fn apply_remote_update(&mut self, update: proto::UpdateWorktree) -> Result<()> {
        let mut entries_by_path_edits = Vec::new();
        let mut entries_by_id_edits = Vec::new();
        for entry_id in update.removed_entries {
            let entry_id = entry_id as usize;
            let entry = self
                .entry_for_id(entry_id)
                .ok_or_else(|| anyhow!("unknown entry"))?;
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

    pub fn entry_for_id(&self, id: usize) -> Option<&Entry> {
        let entry = self.entries_by_id.get(&id, &())?;
        self.entry_for_path(&entry.path)
    }

    pub fn inode_for_path(&self, path: impl AsRef<Path>) -> Option<u64> {
        self.entry_for_path(path.as_ref()).map(|e| e.inode)
    }
}

impl LocalSnapshot {
    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        if !entry.is_dir() && entry.path.file_name() == Some(&GITIGNORE) {
            let abs_path = self.abs_path.join(&entry.path);
            match build_gitignore(&abs_path, fs) {
                Ok(ignore) => {
                    let ignore_dir_path = entry.path.parent().unwrap();
                    self.ignores
                        .insert(ignore_dir_path.into(), (Arc::new(ignore), self.scan_id));
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
        self.entries_by_path.insert_or_replace(entry.clone(), &());
        let scan_id = self.scan_id;
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
    ) {
        let mut parent_entry = self
            .entries_by_path
            .get(&PathKey(parent_path.clone()), &())
            .unwrap()
            .clone();
        if let Some(ignore) = ignore {
            self.ignores.insert(parent_path, (ignore, self.scan_id));
        }
        if matches!(parent_entry.kind, EntryKind::PendingDir) {
            parent_entry.kind = EntryKind::Dir;
        } else {
            unreachable!();
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
            if let Some((_, scan_id)) = self.ignores.get_mut(path.parent().unwrap()) {
                *scan_id = self.scan_id;
            }
        }
    }

    fn ignore_stack_for_path(&self, path: &Path, is_dir: bool) -> Arc<IgnoreStack> {
        let mut new_ignores = Vec::new();
        for ancestor in path.ancestors().skip(1) {
            if let Some((ignore, _)) = self.ignores.get(ancestor) {
                new_ignores.push((ancestor, Some(ignore.clone())));
            } else {
                new_ignores.push((ancestor, None));
            }
        }

        let mut ignore_stack = IgnoreStack::none();
        for (parent_path, ignore) in new_ignores.into_iter().rev() {
            if ignore_stack.is_path_ignored(&parent_path, true) {
                ignore_stack = IgnoreStack::all();
                break;
            } else if let Some(ignore) = ignore {
                ignore_stack = ignore_stack.append(Arc::from(parent_path), ignore);
            }
        }

        if ignore_stack.is_path_ignored(path, is_dir) {
            ignore_stack = IgnoreStack::all();
        }

        ignore_stack
    }
}

fn build_gitignore(abs_path: &Path, fs: &dyn Fs) -> Result<Gitignore> {
    let contents = smol::block_on(fs.load(&abs_path))?;
    let parent = abs_path.parent().unwrap_or(Path::new("/"));
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
        for entry in self.entries_by_path.cursor::<()>() {
            for _ in entry.path.ancestors().skip(1) {
                write!(f, " ")?;
            }
            writeln!(f, "{:?} (inode: {})", entry.path, entry.inode)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct File {
    pub worktree: ModelHandle<Worktree>,
    pub path: Arc<Path>,
    pub mtime: SystemTime,
    pub(crate) entry_id: Option<usize>,
    pub(crate) is_local: bool,
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
        full_path.push(self.worktree.read(cx).root_name());
        full_path.push(&self.path);
        full_path
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name(&self, cx: &AppContext) -> OsString {
        self.path
            .file_name()
            .map(|name| name.into())
            .unwrap_or_else(|| OsString::from(&self.worktree.read(cx).root_name))
    }

    fn is_deleted(&self) -> bool {
        self.entry_id.is_none()
    }

    fn save(
        &self,
        buffer_id: u64,
        text: Rope,
        version: clock::Global,
        cx: &mut MutableAppContext,
    ) -> Task<Result<(clock::Global, SystemTime)>> {
        self.worktree.update(cx, |worktree, cx| match worktree {
            Worktree::Local(worktree) => {
                let rpc = worktree.client.clone();
                let project_id = worktree.share.as_ref().map(|share| share.project_id);
                let save = worktree.save(self.path.clone(), text, cx);
                cx.background().spawn(async move {
                    let entry = save.await?;
                    if let Some(project_id) = project_id {
                        rpc.send(proto::BufferSaved {
                            project_id,
                            buffer_id,
                            version: (&version).into(),
                            mtime: Some(entry.mtime.into()),
                        })
                        .await?;
                    }
                    Ok((version, entry.mtime))
                })
            }
            Worktree::Remote(worktree) => {
                let rpc = worktree.client.clone();
                let project_id = worktree.project_id;
                cx.foreground().spawn(async move {
                    let response = rpc
                        .request(proto::SaveBuffer {
                            project_id,
                            buffer_id,
                        })
                        .await?;
                    let version = response.version.try_into()?;
                    let mtime = response
                        .mtime
                        .ok_or_else(|| anyhow!("missing mtime"))?
                        .into();
                    Ok((version, mtime))
                })
            }
        })
    }

    fn format_remote(
        &self,
        buffer_id: u64,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree.read(cx);
        let worktree = worktree.as_remote()?;
        let rpc = worktree.client.clone();
        let project_id = worktree.project_id;
        Some(cx.foreground().spawn(async move {
            rpc.request(proto::FormatBuffer {
                project_id,
                buffer_id,
            })
            .await?;
            Ok(())
        }))
    }

    fn completions(
        &self,
        buffer_id: u64,
        position: Anchor,
        language: Option<Arc<Language>>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<Vec<Completion<Anchor>>>> {
        let worktree = self.worktree.read(cx);
        let worktree = if let Some(worktree) = worktree.as_remote() {
            worktree
        } else {
            return Task::ready(Err(anyhow!(
                "remote completions requested on a local worktree"
            )));
        };
        let rpc = worktree.client.clone();
        let project_id = worktree.project_id;
        cx.foreground().spawn(async move {
            let response = rpc
                .request(proto::GetCompletions {
                    project_id,
                    buffer_id,
                    position: Some(language::proto::serialize_anchor(&position)),
                })
                .await?;
            response
                .completions
                .into_iter()
                .map(|completion| {
                    language::proto::deserialize_completion(completion, language.as_ref())
                })
                .collect()
        })
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer_id: u64,
        completion: Completion<Anchor>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<Vec<clock::Local>>> {
        let worktree = self.worktree.read(cx);
        let worktree = if let Some(worktree) = worktree.as_remote() {
            worktree
        } else {
            return Task::ready(Err(anyhow!(
                "remote additional edits application requested on a local worktree"
            )));
        };
        let rpc = worktree.client.clone();
        let project_id = worktree.project_id;
        cx.foreground().spawn(async move {
            let response = rpc
                .request(proto::ApplyCompletionAdditionalEdits {
                    project_id,
                    buffer_id,
                    completion: Some(language::proto::serialize_completion(&completion)),
                })
                .await?;

            Ok(response
                .additional_edits
                .into_iter()
                .map(|edit| clock::Local {
                    replica_id: edit.replica_id as ReplicaId,
                    value: edit.local_timestamp,
                })
                .collect())
        })
    }

    fn buffer_updated(&self, buffer_id: u64, operation: Operation, cx: &mut MutableAppContext) {
        self.worktree.update(cx, |worktree, cx| {
            worktree.send_buffer_update(buffer_id, operation, cx);
        });
    }

    fn buffer_removed(&self, buffer_id: u64, cx: &mut MutableAppContext) {
        self.worktree.update(cx, |worktree, cx| {
            if let Worktree::Remote(worktree) = worktree {
                let project_id = worktree.project_id;
                let rpc = worktree.client.clone();
                cx.background()
                    .spawn(async move {
                        if let Err(error) = rpc
                            .send(proto::CloseBuffer {
                                project_id,
                                buffer_id,
                            })
                            .await
                        {
                            log::error!("error closing remote buffer: {}", error);
                        }
                    })
                    .detach();
            }
        });
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_proto(&self) -> rpc::proto::File {
        rpc::proto::File {
            worktree_id: self.worktree.id() as u64,
            entry_id: self.entry_id.map(|entry_id| entry_id as u64),
            path: self.path.to_string_lossy().into(),
            mtime: Some(self.mtime.into()),
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
        mtime: SystemTime,
        cx: &mut MutableAppContext,
    ) {
        let worktree = self.worktree.read(cx).as_local().unwrap();
        if let Some(project_id) = worktree.share.as_ref().map(|share| share.project_id) {
            let rpc = worktree.client.clone();
            let message = proto::BufferReloaded {
                project_id,
                buffer_id,
                version: version.into(),
                mtime: Some(mtime.into()),
            };
            cx.background()
                .spawn(async move { rpc.send(message).await })
                .detach_and_log_err(cx);
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
            entry_id: proto.entry_id.map(|entry_id| entry_id as usize),
            is_local: false,
        })
    }

    pub fn from_dyn(file: Option<&dyn language::File>) -> Option<&Self> {
        file.and_then(|f| f.as_any().downcast_ref())
    }

    pub fn worktree_id(&self, cx: &AppContext) -> WorktreeId {
        self.worktree.read(cx).id()
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub id: usize,
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub inode: u64,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    pub is_ignored: bool,
}

#[derive(Clone, Debug)]
pub enum EntryKind {
    PendingDir,
    Dir,
    File(CharBag),
}

impl Entry {
    fn new(
        path: Arc<Path>,
        metadata: &fs::Metadata,
        next_entry_id: &AtomicUsize,
        root_char_bag: CharBag,
    ) -> Self {
        Self {
            id: next_entry_id.fetch_add(1, SeqCst),
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
        self.visible_count += rhs.visible_count;
        self.file_count += rhs.file_count;
        self.visible_file_count += rhs.visible_file_count;
    }
}

#[derive(Clone, Debug)]
struct PathEntry {
    id: usize,
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
    type Key = usize;

    fn key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Clone, Debug, Default)]
struct PathEntrySummary {
    max_id: usize,
}

impl sum_tree::Summary for PathEntrySummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.max_id = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, PathEntrySummary> for usize {
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
    snapshot: Arc<Mutex<LocalSnapshot>>,
    notify: Sender<ScanState>,
    executor: Arc<executor::Background>,
}

impl BackgroundScanner {
    fn new(
        snapshot: Arc<Mutex<LocalSnapshot>>,
        notify: Sender<ScanState>,
        fs: Arc<dyn Fs>,
        executor: Arc<executor::Background>,
    ) -> Self {
        Self {
            fs,
            snapshot,
            notify,
            executor,
        }
    }

    fn abs_path(&self) -> Arc<Path> {
        self.snapshot.lock().abs_path.clone()
    }

    fn snapshot(&self) -> LocalSnapshot {
        self.snapshot.lock().clone()
    }

    async fn run(mut self, events_rx: impl Stream<Item = Vec<fsevent::Event>>) {
        if self.notify.send(ScanState::Scanning).await.is_err() {
            return;
        }

        if let Err(err) = self.scan_dirs().await {
            if self
                .notify
                .send(ScanState::Err(Arc::new(err)))
                .await
                .is_err()
            {
                return;
            }
        }

        if self.notify.send(ScanState::Idle).await.is_err() {
            return;
        }

        futures::pin_mut!(events_rx);
        while let Some(events) = events_rx.next().await {
            if self.notify.send(ScanState::Scanning).await.is_err() {
                break;
            }

            if !self.process_events(events).await {
                break;
            }

            if self.notify.send(ScanState::Idle).await.is_err() {
                break;
            }
        }
    }

    async fn scan_dirs(&mut self) -> Result<()> {
        let root_char_bag;
        let next_entry_id;
        let is_dir;
        {
            let snapshot = self.snapshot.lock();
            root_char_bag = snapshot.root_char_bag;
            next_entry_id = snapshot.next_entry_id.clone();
            is_dir = snapshot.root_entry().map_or(false, |e| e.is_dir())
        };

        if is_dir {
            let path: Arc<Path> = Arc::from(Path::new(""));
            let abs_path = self.abs_path();
            let (tx, rx) = channel::unbounded();
            tx.send(ScanJob {
                abs_path: abs_path.to_path_buf(),
                path,
                ignore_stack: IgnoreStack::none(),
                scan_queue: tx.clone(),
            })
            .await
            .unwrap();
            drop(tx);

            self.executor
                .scoped(|scope| {
                    for _ in 0..self.executor.num_cpus() {
                        scope.spawn(async {
                            while let Ok(job) = rx.recv().await {
                                if let Err(err) = self
                                    .scan_dir(root_char_bag, next_entry_id.clone(), &job)
                                    .await
                                {
                                    log::error!("error scanning {:?}: {}", job.abs_path, err);
                                }
                            }
                        });
                    }
                })
                .await;
        }

        Ok(())
    }

    async fn scan_dir(
        &self,
        root_char_bag: CharBag,
        next_entry_id: Arc<AtomicUsize>,
        job: &ScanJob,
    ) -> Result<()> {
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<ScanJob> = Vec::new();
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
            let child_metadata = match self.fs.metadata(&child_abs_path).await? {
                Some(metadata) => metadata,
                None => continue,
            };

            // If we find a .gitignore, add it to the stack of ignores used to determine which paths are ignored
            if child_name == *GITIGNORE {
                match build_gitignore(&child_abs_path, self.fs.as_ref()) {
                    Ok(ignore) => {
                        let ignore = Arc::new(ignore);
                        ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
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
                    entry.is_ignored = ignore_stack.is_path_ignored(&entry.path, entry.is_dir());
                    if entry.is_dir() {
                        new_jobs.next().unwrap().ignore_stack = if entry.is_ignored {
                            IgnoreStack::all()
                        } else {
                            ignore_stack.clone()
                        };
                    }
                }
            }

            let mut child_entry = Entry::new(
                child_path.clone(),
                &child_metadata,
                &next_entry_id,
                root_char_bag,
            );

            if child_metadata.is_dir {
                let is_ignored = ignore_stack.is_path_ignored(&child_path, true);
                child_entry.is_ignored = is_ignored;
                new_entries.push(child_entry);
                new_jobs.push(ScanJob {
                    abs_path: child_abs_path,
                    path: child_path,
                    ignore_stack: if is_ignored {
                        IgnoreStack::all()
                    } else {
                        ignore_stack.clone()
                    },
                    scan_queue: job.scan_queue.clone(),
                });
            } else {
                child_entry.is_ignored = ignore_stack.is_path_ignored(&child_path, false);
                new_entries.push(child_entry);
            };
        }

        self.snapshot
            .lock()
            .populate_dir(job.path.clone(), new_entries, new_ignore);
        for new_job in new_jobs {
            job.scan_queue.send(new_job).await.unwrap();
        }

        Ok(())
    }

    async fn process_events(&mut self, mut events: Vec<fsevent::Event>) -> bool {
        let mut snapshot = self.snapshot();
        snapshot.scan_id += 1;

        let root_abs_path = if let Ok(abs_path) = self.fs.canonicalize(&snapshot.abs_path).await {
            abs_path
        } else {
            return false;
        };
        let root_char_bag = snapshot.root_char_bag;
        let next_entry_id = snapshot.next_entry_id.clone();

        events.sort_unstable_by(|a, b| a.path.cmp(&b.path));
        events.dedup_by(|a, b| a.path.starts_with(&b.path));

        for event in &events {
            match event.path.strip_prefix(&root_abs_path) {
                Ok(path) => snapshot.remove_path(&path),
                Err(_) => {
                    log::error!(
                        "unexpected event {:?} for root path {:?}",
                        event.path,
                        root_abs_path
                    );
                    continue;
                }
            }
        }

        let (scan_queue_tx, scan_queue_rx) = channel::unbounded();
        for event in events {
            let path: Arc<Path> = match event.path.strip_prefix(&root_abs_path) {
                Ok(path) => Arc::from(path.to_path_buf()),
                Err(_) => {
                    log::error!(
                        "unexpected event {:?} for root path {:?}",
                        event.path,
                        root_abs_path
                    );
                    continue;
                }
            };

            match self.fs.metadata(&event.path).await {
                Ok(Some(metadata)) => {
                    let ignore_stack = snapshot.ignore_stack_for_path(&path, metadata.is_dir);
                    let mut fs_entry = Entry::new(
                        path.clone(),
                        &metadata,
                        snapshot.next_entry_id.as_ref(),
                        snapshot.root_char_bag,
                    );
                    fs_entry.is_ignored = ignore_stack.is_all();
                    snapshot.insert_entry(fs_entry, self.fs.as_ref());
                    if metadata.is_dir {
                        scan_queue_tx
                            .send(ScanJob {
                                abs_path: event.path,
                                path,
                                ignore_stack,
                                scan_queue: scan_queue_tx.clone(),
                            })
                            .await
                            .unwrap();
                    }
                }
                Ok(None) => {}
                Err(err) => {
                    // TODO - create a special 'error' entry in the entries tree to mark this
                    log::error!("error reading file on event {:?}", err);
                }
            }
        }

        *self.snapshot.lock() = snapshot;

        // Scan any directories that were created as part of this event batch.
        drop(scan_queue_tx);
        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        while let Ok(job) = scan_queue_rx.recv().await {
                            if let Err(err) = self
                                .scan_dir(root_char_bag, next_entry_id.clone(), &job)
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
        true
    }

    async fn update_ignore_statuses(&self) {
        let mut snapshot = self.snapshot();

        let mut ignores_to_update = Vec::new();
        let mut ignores_to_delete = Vec::new();
        for (parent_path, (_, scan_id)) in &snapshot.ignores {
            if *scan_id == snapshot.scan_id && snapshot.entry_for_path(parent_path).is_some() {
                ignores_to_update.push(parent_path.clone());
            }

            let ignore_path = parent_path.join(&*GITIGNORE);
            if snapshot.entry_for_path(ignore_path).is_none() {
                ignores_to_delete.push(parent_path.clone());
            }
        }

        for parent_path in ignores_to_delete {
            snapshot.ignores.remove(&parent_path);
            self.snapshot.lock().ignores.remove(&parent_path);
        }

        let (ignore_queue_tx, ignore_queue_rx) = channel::unbounded();
        ignores_to_update.sort_unstable();
        let mut ignores_to_update = ignores_to_update.into_iter().peekable();
        while let Some(parent_path) = ignores_to_update.next() {
            while ignores_to_update
                .peek()
                .map_or(false, |p| p.starts_with(&parent_path))
            {
                ignores_to_update.next().unwrap();
            }

            let ignore_stack = snapshot.ignore_stack_for_path(&parent_path, true);
            ignore_queue_tx
                .send(UpdateIgnoreStatusJob {
                    path: parent_path,
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

    async fn update_ignore_status(&self, job: UpdateIgnoreStatusJob, snapshot: &LocalSnapshot) {
        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores.get(&job.path) {
            ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
        }

        let mut entries_by_id_edits = Vec::new();
        let mut entries_by_path_edits = Vec::new();
        for mut entry in snapshot.child_entries(&job.path).cloned() {
            let was_ignored = entry.is_ignored;
            entry.is_ignored = ignore_stack.is_path_ignored(&entry.path, entry.is_dir());
            if entry.is_dir() {
                let child_ignore_stack = if entry.is_ignored {
                    IgnoreStack::all()
                } else {
                    ignore_stack.clone()
                };
                job.ignore_queue
                    .send(UpdateIgnoreStatusJob {
                        path: entry.path.clone(),
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
}

async fn refresh_entry(
    fs: &dyn Fs,
    snapshot: &Mutex<LocalSnapshot>,
    path: Arc<Path>,
    abs_path: &Path,
) -> Result<Entry> {
    let root_char_bag;
    let next_entry_id;
    {
        let snapshot = snapshot.lock();
        root_char_bag = snapshot.root_char_bag;
        next_entry_id = snapshot.next_entry_id.clone();
    }
    let entry = Entry::new(
        path,
        &fs.metadata(abs_path)
            .await?
            .ok_or_else(|| anyhow!("could not read saved file metadata"))?,
        &next_entry_id,
        root_char_bag,
    );
    Ok(snapshot.lock().insert_entry(entry, fs))
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
}

struct UpdateIgnoreStatusJob {
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    ignore_queue: Sender<UpdateIgnoreStatusJob>,
}

pub trait WorktreeHandle {
    #[cfg(test)]
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
    #[cfg(test)]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        use smol::future::FutureExt;

        let filename = "fs-event-sentinel";
        let root_path = cx.read(|cx| self.read(cx).as_local().unwrap().abs_path().clone());
        let tree = self.clone();
        async move {
            std::fs::write(root_path.join(filename), "").unwrap();
            tree.condition(&cx, |tree, _| tree.entry_for_path(filename).is_some())
                .await;

            std::fs::remove_file(root_path.join(filename)).unwrap();
            tree.condition(&cx, |tree, _| tree.entry_for_path(filename).is_none())
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
            id: entry.id as u64,
            is_dir: entry.is_dir(),
            path: entry.path.to_string_lossy().to_string(),
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
                let mut char_bag = root_char_bag.clone();
                char_bag.extend(entry.path.chars().map(|c| c.to_ascii_lowercase()));
                EntryKind::File(char_bag)
            };
            let path: Arc<Path> = Arc::from(Path::new(&entry.path));
            Ok(Entry {
                id: entry.id as usize,
                kind,
                path: path.clone(),
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
    use crate::fs::FakeFs;
    use anyhow::Result;
    use client::test::FakeHttpClient;
    use fs::RealFs;
    use rand::prelude::*;
    use serde_json::json;
    use std::{
        env,
        fmt::Write,
        time::{SystemTime, UNIX_EPOCH},
    };
    use util::test::temp_tree;

    #[gpui::test]
    async fn test_traversal(cx: gpui::TestAppContext) {
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
        let client = Client::new(http_client);

        let tree = Worktree::local(
            client,
            Arc::from(Path::new("/root")),
            false,
            Arc::new(fs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        tree.read_with(&cx, |tree, _| {
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
        })
    }

    #[gpui::test]
    async fn test_rescan_with_gitignore(cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            ".git": {},
            ".gitignore": "ignored-dir\n",
            "tracked-dir": {
                "tracked-file1": "tracked contents",
            },
            "ignored-dir": {
                "ignored-file1": "ignored contents",
            }
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());

        let tree = Worktree::local(
            client,
            dir.path(),
            false,
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            let tracked = tree.entry_for_path("tracked-dir/tracked-file1").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file1").unwrap();
            assert_eq!(tracked.is_ignored, false);
            assert_eq!(ignored.is_ignored, true);
        });

        std::fs::write(dir.path().join("tracked-dir/tracked-file2"), "").unwrap();
        std::fs::write(dir.path().join("ignored-dir/ignored-file2"), "").unwrap();
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            let dot_git = tree.entry_for_path(".git").unwrap();
            let tracked = tree.entry_for_path("tracked-dir/tracked-file2").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file2").unwrap();
            assert_eq!(tracked.is_ignored, false);
            assert_eq!(ignored.is_ignored, true);
            assert_eq!(dot_git.is_ignored, true);
        });
    }

    #[gpui::test(iterations = 100)]
    fn test_random(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|o| o.parse().unwrap())
            .unwrap_or(40);
        let initial_entries = env::var("INITIAL_ENTRIES")
            .map(|o| o.parse().unwrap())
            .unwrap_or(20);

        let root_dir = tempdir::TempDir::new("worktree-test").unwrap();
        for _ in 0..initial_entries {
            randomly_mutate_tree(root_dir.path(), 1.0, &mut rng).unwrap();
        }
        log::info!("Generated initial tree");

        let (notify_tx, _notify_rx) = smol::channel::unbounded();
        let fs = Arc::new(RealFs);
        let next_entry_id = Arc::new(AtomicUsize::new(0));
        let mut initial_snapshot = LocalSnapshot {
            abs_path: root_dir.path().into(),
            scan_id: 0,
            removed_entry_ids: Default::default(),
            ignores: Default::default(),
            next_entry_id: next_entry_id.clone(),
            snapshot: Snapshot {
                id: WorktreeId::from_usize(0),
                entries_by_path: Default::default(),
                entries_by_id: Default::default(),
                root_name: Default::default(),
                root_char_bag: Default::default(),
            },
        };
        initial_snapshot.insert_entry(
            Entry::new(
                Path::new("").into(),
                &smol::block_on(fs.metadata(root_dir.path()))
                    .unwrap()
                    .unwrap(),
                &next_entry_id,
                Default::default(),
            ),
            fs.as_ref(),
        );
        let mut scanner = BackgroundScanner::new(
            Arc::new(Mutex::new(initial_snapshot.clone())),
            notify_tx,
            fs.clone(),
            Arc::new(gpui::executor::Background::new()),
        );
        smol::block_on(scanner.scan_dirs()).unwrap();
        scanner.snapshot().check_invariants();

        let mut events = Vec::new();
        let mut snapshots = Vec::new();
        let mut mutations_len = operations;
        while mutations_len > 1 {
            if !events.is_empty() && rng.gen_bool(0.4) {
                let len = rng.gen_range(0..=events.len());
                let to_deliver = events.drain(0..len).collect::<Vec<_>>();
                log::info!("Delivering events: {:#?}", to_deliver);
                smol::block_on(scanner.process_events(to_deliver));
                scanner.snapshot().check_invariants();
            } else {
                events.extend(randomly_mutate_tree(root_dir.path(), 0.6, &mut rng).unwrap());
                mutations_len -= 1;
            }

            if rng.gen_bool(0.2) {
                snapshots.push(scanner.snapshot());
            }
        }
        log::info!("Quiescing: {:#?}", events);
        smol::block_on(scanner.process_events(events));
        scanner.snapshot().check_invariants();

        let (notify_tx, _notify_rx) = smol::channel::unbounded();
        let mut new_scanner = BackgroundScanner::new(
            Arc::new(Mutex::new(initial_snapshot)),
            notify_tx,
            scanner.fs.clone(),
            scanner.executor.clone(),
        );
        smol::block_on(new_scanner.scan_dirs()).unwrap();
        assert_eq!(
            scanner.snapshot().to_vec(true),
            new_scanner.snapshot().to_vec(true)
        );

        for mut prev_snapshot in snapshots {
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

            let update = scanner
                .snapshot()
                .build_update(&prev_snapshot, 0, 0, include_ignored);
            prev_snapshot.apply_remote_update(update).unwrap();
            assert_eq!(
                prev_snapshot.to_vec(true),
                scanner.snapshot().to_vec(include_ignored)
            );
        }
    }

    fn randomly_mutate_tree(
        root_path: &Path,
        insertion_probability: f64,
        rng: &mut impl Rng,
    ) -> Result<Vec<fsevent::Event>> {
        let root_path = root_path.canonicalize().unwrap();
        let (dirs, files) = read_dir_recursive(root_path.clone());

        let mut events = Vec::new();
        let mut record_event = |path: PathBuf| {
            events.push(fsevent::Event {
                event_id: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                flags: fsevent::StreamFlags::empty(),
                path,
            });
        };

        if (files.is_empty() && dirs.len() == 1) || rng.gen_bool(insertion_probability) {
            let path = dirs.choose(rng).unwrap();
            let new_path = path.join(gen_name(rng));

            if rng.gen() {
                log::info!("Creating dir {:?}", new_path.strip_prefix(root_path)?);
                std::fs::create_dir(&new_path)?;
            } else {
                log::info!("Creating file {:?}", new_path.strip_prefix(root_path)?);
                std::fs::write(&new_path, "")?;
            }
            record_event(new_path);
        } else if rng.gen_bool(0.05) {
            let ignore_dir_path = dirs.choose(rng).unwrap();
            let ignore_path = ignore_dir_path.join(&*GITIGNORE);

            let (subdirs, subfiles) = read_dir_recursive(ignore_dir_path.clone());
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
                write!(
                    ignore_contents,
                    "{}\n",
                    path_to_ignore
                        .strip_prefix(&ignore_dir_path)?
                        .to_str()
                        .unwrap()
                )
                .unwrap();
            }
            log::info!(
                "Creating {:?} with contents:\n{}",
                ignore_path.strip_prefix(&root_path)?,
                ignore_contents
            );
            std::fs::write(&ignore_path, ignore_contents).unwrap();
            record_event(ignore_path);
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
                    std::fs::remove_dir_all(&new_path_parent).ok();
                    new_path_parent.to_path_buf()
                } else {
                    new_path_parent.join(gen_name(rng))
                };

                log::info!(
                    "Renaming {:?} to {}{:?}",
                    old_path.strip_prefix(&root_path)?,
                    if overwrite_existing_dir {
                        "overwrite "
                    } else {
                        ""
                    },
                    new_path.strip_prefix(&root_path)?
                );
                std::fs::rename(&old_path, &new_path)?;
                record_event(old_path.clone());
                record_event(new_path);
            } else if old_path.is_dir() {
                let (dirs, files) = read_dir_recursive(old_path.clone());

                log::info!("Deleting dir {:?}", old_path.strip_prefix(&root_path)?);
                std::fs::remove_dir_all(&old_path).unwrap();
                for file in files {
                    record_event(file);
                }
                for dir in dirs {
                    record_event(dir);
                }
            } else {
                log::info!("Deleting file {:?}", old_path.strip_prefix(&root_path)?);
                std::fs::remove_file(old_path).unwrap();
                record_event(old_path.clone());
            }
        }

        Ok(events)
    }

    fn read_dir_recursive(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let child_entries = std::fs::read_dir(&path).unwrap();
        let mut dirs = vec![path];
        let mut files = Vec::new();
        for child_entry in child_entries {
            let child_path = child_entry.unwrap().path();
            if child_path.is_dir() {
                let (child_dirs, child_files) = read_dir_recursive(child_path);
                dirs.extend(child_dirs);
                files.extend(child_files);
            } else {
                files.push(child_path);
            }
        }
        (dirs, files)
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

            let dfs_paths = self
                .entries_by_path
                .cursor::<()>()
                .map(|e| e.path.as_ref())
                .collect::<Vec<_>>();
            assert_eq!(bfs_paths, dfs_paths);

            for (ignore_parent_path, _) in &self.ignores {
                assert!(self.entry_for_path(ignore_parent_path).is_some());
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
            paths.sort_by(|a, b| a.0.cmp(&b.0));
            paths
        }
    }
}
