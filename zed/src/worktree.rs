mod ignore;

use self::ignore::IgnoreStack;
use crate::{
    editor::{self, Buffer, History, Operation, Rope},
    fs::{self, Fs},
    fuzzy,
    fuzzy::CharBag,
    language::LanguageRegistry,
    rpc::{self, proto},
    sum_tree::{self, Cursor, Edit, SumTree},
    time::{self, ReplicaId},
    util::Bias,
};
use ::ignore::gitignore::Gitignore;
use anyhow::{anyhow, Result};
use futures::{Stream, StreamExt};
pub use fuzzy::{match_paths, PathMatch};
use gpui::{
    executor, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task, WeakModelHandle,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::{
    prelude::{Sink as _, Stream as _},
    watch,
};
use smol::channel::{self, Sender};
use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    convert::{TryFrom, TryInto},
    ffi::{OsStr, OsString},
    fmt,
    future::Future,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, SystemTime},
};
use zrpc::{ForegroundRouter, PeerId, TypedEnvelope};

lazy_static! {
    static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
}

pub fn init(cx: &mut MutableAppContext, rpc: &rpc::Client, router: &mut ForegroundRouter) {
    rpc.on_message(router, remote::add_peer, cx);
    rpc.on_message(router, remote::remove_peer, cx);
    rpc.on_message(router, remote::update_worktree, cx);
    rpc.on_message(router, remote::open_buffer, cx);
    rpc.on_message(router, remote::close_buffer, cx);
    rpc.on_message(router, remote::update_buffer, cx);
    rpc.on_message(router, remote::buffer_saved, cx);
    rpc.on_message(router, remote::save_buffer, cx);
}

#[derive(Clone, Debug)]
enum ScanState {
    Idle,
    Scanning,
    Err(Arc<anyhow::Error>),
}

pub enum Worktree {
    Local(LocalWorktree),
    Remote(RemoteWorktree),
}

impl Entity for Worktree {
    type Event = ();

    fn release(&mut self, cx: &mut MutableAppContext) {
        let rpc = match self {
            Self::Local(tree) => tree.rpc.clone(),
            Self::Remote(tree) => Some((tree.rpc.clone(), tree.remote_id)),
        };

        if let Some((rpc, worktree_id)) = rpc {
            cx.spawn(|_| async move {
                rpc.state
                    .write()
                    .await
                    .shared_worktrees
                    .remove(&worktree_id);
                if let Err(err) = rpc.send(proto::CloseWorktree { worktree_id }).await {
                    log::error!("error closing worktree {}: {}", worktree_id, err);
                }
            })
            .detach();
        }
    }
}

impl Worktree {
    pub async fn open_local(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let (tree, scan_states_tx) = LocalWorktree::new(path, languages, fs.clone(), cx).await?;
        tree.update(cx, |tree, cx| {
            let tree = tree.as_local_mut().unwrap();
            let abs_path = tree.snapshot.abs_path.clone();
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

    pub async fn open_remote(
        rpc: rpc::Client,
        id: u64,
        access_token: String,
        languages: Arc<LanguageRegistry>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let response = rpc
            .request(proto::OpenWorktree {
                worktree_id: id,
                access_token,
            })
            .await?;

        Worktree::remote(response, rpc, languages, cx).await
    }

    async fn remote(
        open_response: proto::OpenWorktreeResponse,
        rpc: rpc::Client,
        languages: Arc<LanguageRegistry>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let worktree = open_response
            .worktree
            .ok_or_else(|| anyhow!("empty worktree"))?;

        let remote_id = open_response.worktree_id;
        let replica_id = open_response.replica_id as ReplicaId;
        let peers = open_response.peers;
        let root_char_bag: CharBag = worktree
            .root_name
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let root_name = worktree.root_name.clone();
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

        let worktree = cx.update(|cx| {
            cx.add_model(|cx: &mut ModelContext<Worktree>| {
                let snapshot = Snapshot {
                    id: cx.model_id(),
                    scan_id: 0,
                    abs_path: Path::new("").into(),
                    root_name,
                    root_char_bag,
                    ignores: Default::default(),
                    entries_by_path,
                    entries_by_id,
                    removed_entry_ids: Default::default(),
                    next_entry_id: Default::default(),
                };

                let (updates_tx, mut updates_rx) = postage::mpsc::channel(64);
                let (mut snapshot_tx, snapshot_rx) = watch::channel_with(snapshot.clone());

                cx.background()
                    .spawn(async move {
                        while let Some(update) = updates_rx.recv().await {
                            let mut snapshot = snapshot_tx.borrow().clone();
                            if let Err(error) = snapshot.apply_update(update) {
                                log::error!("error applying worktree update: {}", error);
                            }
                            *snapshot_tx.borrow_mut() = snapshot;
                        }
                    })
                    .detach();

                {
                    let mut snapshot_rx = snapshot_rx.clone();
                    cx.spawn_weak(|this, mut cx| async move {
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

                Worktree::Remote(RemoteWorktree {
                    remote_id,
                    replica_id,
                    snapshot,
                    snapshot_rx,
                    updates_tx,
                    rpc: rpc.clone(),
                    open_buffers: Default::default(),
                    peers: peers
                        .into_iter()
                        .map(|p| (PeerId(p.peer_id), p.replica_id as ReplicaId))
                        .collect(),
                    languages,
                })
            })
        });
        rpc.state
            .write()
            .await
            .shared_worktrees
            .insert(open_response.worktree_id, worktree.downgrade());

        Ok(worktree)
    }

    pub fn as_local(&self) -> Option<&LocalWorktree> {
        if let Worktree::Local(worktree) = self {
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

    pub fn snapshot(&self) -> Snapshot {
        match self {
            Worktree::Local(worktree) => worktree.snapshot(),
            Worktree::Remote(worktree) => worktree.snapshot(),
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match self {
            Worktree::Local(_) => 0,
            Worktree::Remote(worktree) => worktree.replica_id,
        }
    }

    pub fn add_peer(
        &mut self,
        envelope: TypedEnvelope<proto::AddPeer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        match self {
            Worktree::Local(worktree) => worktree.add_peer(envelope, cx),
            Worktree::Remote(worktree) => worktree.add_peer(envelope, cx),
        }
    }

    pub fn remove_peer(
        &mut self,
        envelope: TypedEnvelope<proto::RemovePeer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        match self {
            Worktree::Local(worktree) => worktree.remove_peer(envelope, cx),
            Worktree::Remote(worktree) => worktree.remove_peer(envelope, cx),
        }
    }

    pub fn peers(&self) -> &HashMap<PeerId, ReplicaId> {
        match self {
            Worktree::Local(worktree) => &worktree.peers,
            Worktree::Remote(worktree) => &worktree.peers,
        }
    }

    pub fn open_buffer(
        &mut self,
        path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        match self {
            Worktree::Local(worktree) => worktree.open_buffer(path.as_ref(), cx),
            Worktree::Remote(worktree) => worktree.open_buffer(path.as_ref(), cx),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn has_open_buffer(&self, path: impl AsRef<Path>, cx: &AppContext) -> bool {
        let mut open_buffers: Box<dyn Iterator<Item = _>> = match self {
            Worktree::Local(worktree) => Box::new(worktree.open_buffers.values()),
            Worktree::Remote(worktree) => {
                Box::new(worktree.open_buffers.values().filter_map(|buf| {
                    if let RemoteBuffer::Loaded(buf) = buf {
                        Some(buf)
                    } else {
                        None
                    }
                }))
            }
        };

        let path = path.as_ref();
        open_buffers
            .find(|buffer| {
                if let Some(file) = buffer.upgrade(cx).and_then(|buffer| buffer.read(cx).file()) {
                    file.path.as_ref() == path
                } else {
                    false
                }
            })
            .is_some()
    }

    pub fn update_buffer(
        &mut self,
        envelope: proto::UpdateBuffer,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let buffer_id = envelope.buffer_id as usize;
        let ops = envelope
            .operations
            .into_iter()
            .map(|op| op.try_into())
            .collect::<anyhow::Result<Vec<_>>>()?;

        match self {
            Worktree::Local(worktree) => {
                let buffer = worktree
                    .open_buffers
                    .get(&buffer_id)
                    .and_then(|buf| buf.upgrade(&cx))
                    .ok_or_else(|| {
                        anyhow!("invalid buffer {} in update buffer message", buffer_id)
                    })?;
                buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
            }
            Worktree::Remote(worktree) => match worktree.open_buffers.get_mut(&buffer_id) {
                Some(RemoteBuffer::Operations(pending_ops)) => pending_ops.extend(ops),
                Some(RemoteBuffer::Loaded(buffer)) => {
                    if let Some(buffer) = buffer.upgrade(&cx) {
                        buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                    } else {
                        worktree
                            .open_buffers
                            .insert(buffer_id, RemoteBuffer::Operations(ops));
                    }
                }
                None => {
                    worktree
                        .open_buffers
                        .insert(buffer_id, RemoteBuffer::Operations(ops));
                }
            },
        }

        Ok(())
    }

    pub fn buffer_saved(
        &mut self,
        message: proto::BufferSaved,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let Worktree::Remote(worktree) = self {
            if let Some(buffer) = worktree
                .open_buffers
                .get(&(message.buffer_id as usize))
                .and_then(|buf| buf.upgrade(&cx))
            {
                buffer.update(cx, |buffer, cx| {
                    let version = message.version.try_into()?;
                    let mtime = message
                        .mtime
                        .ok_or_else(|| anyhow!("missing mtime"))?
                        .into();
                    buffer.did_save(version, mtime, cx);
                    Result::<_, anyhow::Error>::Ok(())
                })?;
            }
            Ok(())
        } else {
            Err(anyhow!(
                "invalid buffer {} in buffer saved message",
                message.buffer_id
            ))
        }
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
                    self.update_open_buffers(cx);
                }
            }
            Self::Remote(worktree) => {
                worktree.snapshot = worktree.snapshot_rx.borrow().clone();
                self.update_open_buffers(cx);
            }
        };

        cx.notify();
    }

    fn update_open_buffers(&mut self, cx: &mut ModelContext<Self>) {
        let open_buffers: Box<dyn Iterator<Item = _>> = match &self {
            Self::Local(worktree) => Box::new(worktree.open_buffers.iter()),
            Self::Remote(worktree) => {
                Box::new(worktree.open_buffers.iter().filter_map(|(id, buf)| {
                    if let RemoteBuffer::Loaded(buf) = buf {
                        Some((id, buf))
                    } else {
                        None
                    }
                }))
            }
        };

        let mut buffers_to_delete = Vec::new();
        for (buffer_id, buffer) in open_buffers {
            if let Some(buffer) = buffer.upgrade(&cx) {
                buffer.update(cx, |buffer, cx| {
                    let buffer_is_clean = !buffer.is_dirty();

                    if let Some(file) = buffer.file_mut() {
                        let mut file_changed = false;

                        if let Some(entry) = file
                            .entry_id
                            .and_then(|entry_id| self.entry_for_id(entry_id))
                        {
                            if entry.path != file.path {
                                file.path = entry.path.clone();
                                file_changed = true;
                            }

                            if entry.mtime != file.mtime {
                                file.mtime = entry.mtime;
                                file_changed = true;
                                if let Some(worktree) = self.as_local() {
                                    if buffer_is_clean {
                                        let abs_path = worktree.absolutize(&file.path);
                                        refresh_buffer(abs_path, &worktree.fs, cx);
                                    }
                                }
                            }
                        } else if let Some(entry) = self.entry_for_path(&file.path) {
                            file.entry_id = Some(entry.id);
                            file.mtime = entry.mtime;
                            if let Some(worktree) = self.as_local() {
                                if buffer_is_clean {
                                    let abs_path = worktree.absolutize(&file.path);
                                    refresh_buffer(abs_path, &worktree.fs, cx);
                                }
                            }
                            file_changed = true;
                        } else if !file.is_deleted() {
                            if buffer_is_clean {
                                cx.emit(editor::buffer::Event::Dirtied);
                            }
                            file.entry_id = None;
                            file_changed = true;
                        }

                        if file_changed {
                            cx.emit(editor::buffer::Event::FileHandleChanged);
                        }
                    }
                });
            } else {
                buffers_to_delete.push(*buffer_id);
            }
        }

        for buffer_id in buffers_to_delete {
            match self {
                Self::Local(worktree) => {
                    worktree.open_buffers.remove(&buffer_id);
                }
                Self::Remote(worktree) => {
                    worktree.open_buffers.remove(&buffer_id);
                }
            }
        }
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

pub struct LocalWorktree {
    snapshot: Snapshot,
    background_snapshot: Arc<Mutex<Snapshot>>,
    snapshots_to_send_tx: Option<Sender<Snapshot>>,
    last_scan_state_rx: watch::Receiver<ScanState>,
    _background_scanner_task: Option<Task<()>>,
    poll_task: Option<Task<()>>,
    rpc: Option<(rpc::Client, u64)>,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
    shared_buffers: HashMap<PeerId, HashMap<u64, ModelHandle<Buffer>>>,
    peers: HashMap<PeerId, ReplicaId>,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
}

impl LocalWorktree {
    async fn new(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
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

        let (scan_states_tx, scan_states_rx) = smol::channel::unbounded();
        let (mut last_scan_state_tx, last_scan_state_rx) = watch::channel_with(ScanState::Scanning);
        let tree = cx.add_model(move |cx: &mut ModelContext<Worktree>| {
            let mut snapshot = Snapshot {
                id: cx.model_id(),
                scan_id: 0,
                abs_path,
                root_name,
                root_char_bag,
                ignores: Default::default(),
                entries_by_path: Default::default(),
                entries_by_id: Default::default(),
                removed_entry_ids: Default::default(),
                next_entry_id: Arc::new(next_entry_id),
            };
            if let Some(metadata) = metadata {
                snapshot.insert_entry(Entry::new(
                    path.into(),
                    &metadata,
                    &snapshot.next_entry_id,
                    snapshot.root_char_bag,
                ));
            }

            let tree = Self {
                snapshot: snapshot.clone(),
                background_snapshot: Arc::new(Mutex::new(snapshot)),
                snapshots_to_send_tx: None,
                last_scan_state_rx,
                _background_scanner_task: None,
                poll_task: None,
                open_buffers: Default::default(),
                shared_buffers: Default::default(),
                peers: Default::default(),
                rpc: None,
                languages,
                fs,
            };

            cx.spawn_weak(|this, mut cx| async move {
                while let Ok(scan_state) = scan_states_rx.recv().await {
                    if let Some(handle) = cx.read(|cx| this.upgrade(&cx)) {
                        let to_send = handle.update(&mut cx, |this, cx| {
                            last_scan_state_tx.blocking_send(scan_state).ok();
                            this.poll_snapshot(cx);
                            let tree = this.as_local_mut().unwrap();
                            if !tree.is_scanning() {
                                if let Some(snapshots_to_send_tx) =
                                    tree.snapshots_to_send_tx.clone()
                                {
                                    Some((tree.snapshot(), snapshots_to_send_tx))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
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

    pub fn open_buffer(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let handle = cx.handle();

        // If there is already a buffer for the given path, then return it.
        let mut existing_buffer = None;
        self.open_buffers.retain(|_buffer_id, buffer| {
            if let Some(buffer) = buffer.upgrade(cx.as_ref()) {
                if let Some(file) = buffer.read(cx.as_ref()).file() {
                    if file.worktree_id() == handle.id() && file.path.as_ref() == path {
                        existing_buffer = Some(buffer);
                    }
                }
                true
            } else {
                false
            }
        });

        let languages = self.languages.clone();
        let path = Arc::from(path);
        cx.spawn(|this, mut cx| async move {
            if let Some(existing_buffer) = existing_buffer {
                Ok(existing_buffer)
            } else {
                let (file, contents) = this
                    .update(&mut cx, |this, cx| this.as_local().unwrap().load(&path, cx))
                    .await?;
                let language = languages.select_language(&path).cloned();
                let buffer = cx.add_model(|cx| {
                    Buffer::from_history(0, History::new(contents.into()), Some(file), language, cx)
                });
                this.update(&mut cx, |this, _| {
                    let this = this
                        .as_local_mut()
                        .ok_or_else(|| anyhow!("must be a local worktree"))?;
                    this.open_buffers.insert(buffer.id(), buffer.downgrade());
                    Ok(buffer)
                })
            }
        })
    }

    pub fn open_remote_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::OpenBuffer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<proto::OpenBufferResponse>> {
        let peer_id = envelope.original_sender_id();
        let path = Path::new(&envelope.payload.path);

        let buffer = self.open_buffer(path, cx);

        cx.spawn(|this, mut cx| async move {
            let buffer = buffer.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .shared_buffers
                    .entry(peer_id?)
                    .or_default()
                    .insert(buffer.id() as u64, buffer.clone());

                Ok(proto::OpenBufferResponse {
                    buffer: Some(buffer.update(cx.as_mut(), |buffer, cx| buffer.to_proto(cx))),
                })
            })
        })
    }

    pub fn close_remote_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        _: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&envelope.original_sender_id()?) {
            shared_buffers.remove(&envelope.payload.buffer_id);
        }

        Ok(())
    }

    pub fn add_peer(
        &mut self,
        envelope: TypedEnvelope<proto::AddPeer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let peer = envelope.payload.peer.ok_or_else(|| anyhow!("empty peer"))?;
        self.peers
            .insert(PeerId(peer.peer_id), peer.replica_id as ReplicaId);
        cx.notify();
        Ok(())
    }

    pub fn remove_peer(
        &mut self,
        envelope: TypedEnvelope<proto::RemovePeer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let peer_id = PeerId(envelope.payload.peer_id);
        let replica_id = self
            .peers
            .remove(&peer_id)
            .ok_or_else(|| anyhow!("unknown peer {:?}", peer_id))?;
        self.shared_buffers.remove(&peer_id);
        for (_, buffer) in &self.open_buffers {
            if let Some(buffer) = buffer.upgrade(&cx) {
                buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
            }
        }
        cx.notify();
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

    pub fn snapshot(&self) -> Snapshot {
        self.snapshot.clone()
    }

    pub fn abs_path(&self) -> &Path {
        self.snapshot.abs_path.as_ref()
    }

    pub fn contains_abs_path(&self, path: &Path) -> bool {
        path.starts_with(&self.snapshot.abs_path)
    }

    fn absolutize(&self, path: &Path) -> PathBuf {
        if path.file_name().is_some() {
            self.snapshot.abs_path.join(path)
        } else {
            self.snapshot.abs_path.to_path_buf()
        }
    }

    fn load(&self, path: &Path, cx: &mut ModelContext<Worktree>) -> Task<Result<(File, String)>> {
        let handle = cx.handle();
        let path = Arc::from(path);
        let abs_path = self.absolutize(&path);
        let background_snapshot = self.background_snapshot.clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let text = fs.load(&abs_path).await?;
            // Eagerly populate the snapshot with an updated entry for the loaded file
            let entry = refresh_entry(fs.as_ref(), &background_snapshot, path, &abs_path).await?;
            this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
            Ok((File::new(entry.id, handle, entry.path, entry.mtime), text))
        })
    }

    pub fn save_buffer_as(
        &self,
        buffer: ModelHandle<Buffer>,
        path: impl Into<Arc<Path>>,
        text: Rope,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<File>> {
        let save = self.save(path, text, cx);
        cx.spawn(|this, mut cx| async move {
            let entry = save.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .open_buffers
                    .insert(buffer.id(), buffer.downgrade());
                Ok(File::new(entry.id, cx.handle(), entry.path, entry.mtime))
            })
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

    pub fn share(
        &mut self,
        rpc: rpc::Client,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<anyhow::Result<(u64, String)>> {
        let snapshot = self.snapshot();
        let share_request = self.share_request(cx);
        let handle = cx.handle();
        cx.spawn(|this, mut cx| async move {
            let share_request = share_request.await;
            let share_response = rpc.request(share_request).await?;

            rpc.state
                .write()
                .await
                .shared_worktrees
                .insert(share_response.worktree_id, handle.downgrade());

            log::info!("sharing worktree {:?}", share_response);
            let (snapshots_to_send_tx, snapshots_to_send_rx) =
                smol::channel::unbounded::<Snapshot>();

            cx.background()
                .spawn({
                    let rpc = rpc.clone();
                    let worktree_id = share_response.worktree_id;
                    async move {
                        let mut prev_snapshot = snapshot;
                        while let Ok(snapshot) = snapshots_to_send_rx.recv().await {
                            let message = snapshot.build_update(&prev_snapshot, worktree_id);
                            match rpc.send(message).await {
                                Ok(()) => prev_snapshot = snapshot,
                                Err(err) => log::error!("error sending snapshot diff {}", err),
                            }
                        }
                    }
                })
                .detach();

            this.update(&mut cx, |worktree, _| {
                let worktree = worktree.as_local_mut().unwrap();
                worktree.rpc = Some((rpc, share_response.worktree_id));
                worktree.snapshots_to_send_tx = Some(snapshots_to_send_tx);
            });

            Ok((share_response.worktree_id, share_response.access_token))
        })
    }

    fn share_request(&self, cx: &mut ModelContext<Worktree>) -> Task<proto::ShareWorktree> {
        let snapshot = self.snapshot();
        let root_name = self.root_name.clone();
        cx.background().spawn(async move {
            let entries = snapshot
                .entries_by_path
                .cursor::<(), ()>()
                .map(Into::into)
                .collect();
            proto::ShareWorktree {
                worktree: Some(proto::Worktree { root_name, entries }),
            }
        })
    }
}

pub fn refresh_buffer(abs_path: PathBuf, fs: &Arc<dyn Fs>, cx: &mut ModelContext<Buffer>) {
    let fs = fs.clone();
    cx.spawn(|buffer, mut cx| async move {
        let new_text = fs.load(&abs_path).await;
        match new_text {
            Err(error) => log::error!("error refreshing buffer after file changed: {}", error),
            Ok(new_text) => {
                buffer
                    .update(&mut cx, |buffer, cx| {
                        buffer.set_text_from_disk(new_text.into(), cx)
                    })
                    .await;
            }
        }
    })
    .detach()
}

impl Deref for LocalWorktree {
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

pub struct RemoteWorktree {
    remote_id: u64,
    snapshot: Snapshot,
    snapshot_rx: watch::Receiver<Snapshot>,
    rpc: rpc::Client,
    updates_tx: postage::mpsc::Sender<proto::UpdateWorktree>,
    replica_id: ReplicaId,
    open_buffers: HashMap<usize, RemoteBuffer>,
    peers: HashMap<PeerId, ReplicaId>,
    languages: Arc<LanguageRegistry>,
}

impl RemoteWorktree {
    pub fn open_buffer(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let handle = cx.handle();
        let mut existing_buffer = None;
        self.open_buffers.retain(|_buffer_id, buffer| {
            if let Some(buffer) = buffer.upgrade(cx.as_ref()) {
                if let Some(file) = buffer.read(cx.as_ref()).file() {
                    if file.worktree_id() == handle.id() && file.path.as_ref() == path {
                        existing_buffer = Some(buffer);
                    }
                }
                true
            } else {
                false
            }
        });

        let rpc = self.rpc.clone();
        let languages = self.languages.clone();
        let replica_id = self.replica_id;
        let remote_worktree_id = self.remote_id;
        let path = path.to_string_lossy().to_string();
        cx.spawn(|this, mut cx| async move {
            if let Some(existing_buffer) = existing_buffer {
                Ok(existing_buffer)
            } else {
                let entry = this
                    .read_with(&cx, |tree, _| tree.entry_for_path(&path).cloned())
                    .ok_or_else(|| anyhow!("file does not exist"))?;
                let file = File::new(entry.id, handle, entry.path, entry.mtime);
                let language = languages.select_language(&path).cloned();
                let response = rpc
                    .request(proto::OpenBuffer {
                        worktree_id: remote_worktree_id as u64,
                        path,
                    })
                    .await?;
                let remote_buffer = response.buffer.ok_or_else(|| anyhow!("empty buffer"))?;
                let buffer_id = remote_buffer.id as usize;
                let buffer = cx.add_model(|cx| {
                    Buffer::from_proto(replica_id, remote_buffer, Some(file), language, cx).unwrap()
                });
                this.update(&mut cx, |this, cx| {
                    let this = this.as_remote_mut().unwrap();
                    if let Some(RemoteBuffer::Operations(pending_ops)) = this
                        .open_buffers
                        .insert(buffer_id, RemoteBuffer::Loaded(buffer.downgrade()))
                    {
                        buffer.update(cx, |buf, cx| buf.apply_ops(pending_ops, cx))?;
                    }
                    Result::<_, anyhow::Error>::Ok(())
                })?;
                Ok(buffer)
            }
        })
    }

    fn snapshot(&self) -> Snapshot {
        self.snapshot.clone()
    }

    pub fn add_peer(
        &mut self,
        envelope: TypedEnvelope<proto::AddPeer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let peer = envelope.payload.peer.ok_or_else(|| anyhow!("empty peer"))?;
        self.peers
            .insert(PeerId(peer.peer_id), peer.replica_id as ReplicaId);
        cx.notify();
        Ok(())
    }

    pub fn remove_peer(
        &mut self,
        envelope: TypedEnvelope<proto::RemovePeer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let peer_id = PeerId(envelope.payload.peer_id);
        let replica_id = self
            .peers
            .remove(&peer_id)
            .ok_or_else(|| anyhow!("unknown peer {:?}", peer_id))?;
        for (_, buffer) in &self.open_buffers {
            if let Some(buffer) = buffer.upgrade(&cx) {
                buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
            }
        }
        cx.notify();
        Ok(())
    }
}

enum RemoteBuffer {
    Operations(Vec<Operation>),
    Loaded(WeakModelHandle<Buffer>),
}

impl RemoteBuffer {
    fn upgrade(&self, cx: impl AsRef<AppContext>) -> Option<ModelHandle<Buffer>> {
        match self {
            Self::Operations(_) => None,
            Self::Loaded(buffer) => buffer.upgrade(cx),
        }
    }
}

#[derive(Clone)]
pub struct Snapshot {
    id: usize,
    scan_id: usize,
    abs_path: Arc<Path>,
    root_name: String,
    root_char_bag: CharBag,
    ignores: HashMap<Arc<Path>, (Arc<Gitignore>, usize)>,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
    removed_entry_ids: HashMap<u64, usize>,
    next_entry_id: Arc<AtomicUsize>,
}

impl Snapshot {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn build_update(&self, other: &Self, worktree_id: u64) -> proto::UpdateWorktree {
        let mut updated_entries = Vec::new();
        let mut removed_entries = Vec::new();
        let mut self_entries = self.entries_by_id.cursor::<(), ()>().peekable();
        let mut other_entries = other.entries_by_id.cursor::<(), ()>().peekable();
        loop {
            match (self_entries.peek(), other_entries.peek()) {
                (Some(self_entry), Some(other_entry)) => match self_entry.id.cmp(&other_entry.id) {
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
                },
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
            updated_entries,
            removed_entries,
            worktree_id,
        }
    }

    fn apply_update(&mut self, update: proto::UpdateWorktree) -> Result<()> {
        self.scan_id += 1;
        let scan_id = self.scan_id;

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
                scan_id,
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

    pub fn files(&self, start: usize) -> FileIter {
        FileIter::all(self, start)
    }

    pub fn paths(&self) -> impl Iterator<Item = &Arc<Path>> {
        let empty_path = Path::new("");
        self.entries_by_path
            .cursor::<(), ()>()
            .filter(move |entry| entry.path.as_ref() != empty_path)
            .map(|entry| &entry.path)
    }

    pub fn visible_files(&self, start: usize) -> FileIter {
        FileIter::visible(self, start)
    }

    fn child_entries<'a>(&'a self, path: &'a Path) -> ChildEntriesIter<'a> {
        ChildEntriesIter::new(path, self)
    }

    pub fn root_entry(&self) -> Option<&Entry> {
        self.entry_for_path("")
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    fn entry_for_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let mut cursor = self.entries_by_path.cursor::<_, ()>();
        if cursor.seek(&PathSearch::Exact(path.as_ref()), Bias::Left, &()) {
            cursor.item()
        } else {
            None
        }
    }

    fn entry_for_id(&self, id: usize) -> Option<&Entry> {
        let entry = self.entries_by_id.get(&id, &())?;
        self.entry_for_path(&entry.path)
    }

    pub fn inode_for_path(&self, path: impl AsRef<Path>) -> Option<u64> {
        self.entry_for_path(path.as_ref()).map(|e| e.inode)
    }

    fn insert_entry(&mut self, mut entry: Entry) -> Entry {
        if !entry.is_dir() && entry.path.file_name() == Some(&GITIGNORE) {
            let (ignore, err) = Gitignore::new(self.abs_path.join(&entry.path));
            if let Some(err) = err {
                log::error!("error in ignore file {:?} - {:?}", &entry.path, err);
            }

            let ignore_dir_path = entry.path.parent().unwrap();
            self.ignores
                .insert(ignore_dir_path.into(), (Arc::new(ignore), self.scan_id));
        }

        self.reuse_entry_id(&mut entry);
        self.entries_by_path.insert_or_replace(entry.clone(), &());
        self.entries_by_id.insert_or_replace(
            PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                scan_id: self.scan_id,
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
        let removed_entry_ids;
        {
            let mut cursor = self.entries_by_path.cursor::<_, ()>();
            new_entries = cursor.slice(&PathSearch::Exact(path), Bias::Left, &());
            removed_entry_ids = cursor.slice(&PathSearch::Successor(path), Bias::Left, &());
            new_entries.push_tree(cursor.suffix(&()), &());
        }
        self.entries_by_path = new_entries;

        let mut entries_by_id_edits = Vec::new();
        for entry in removed_entry_ids.cursor::<(), ()>() {
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

impl fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.entries_by_path.cursor::<(), ()>() {
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
    entry_id: Option<usize>,
    worktree: ModelHandle<Worktree>,
    pub path: Arc<Path>,
    pub mtime: SystemTime,
}

impl File {
    pub fn new(
        entry_id: usize,
        worktree: ModelHandle<Worktree>,
        path: Arc<Path>,
        mtime: SystemTime,
    ) -> Self {
        Self {
            entry_id: Some(entry_id),
            worktree,
            path,
            mtime,
        }
    }

    pub fn buffer_updated(&self, buffer_id: u64, operation: Operation, cx: &mut MutableAppContext) {
        self.worktree.update(cx, |worktree, cx| {
            if let Some((rpc, remote_id)) = match worktree {
                Worktree::Local(worktree) => worktree.rpc.clone(),
                Worktree::Remote(worktree) => Some((worktree.rpc.clone(), worktree.remote_id)),
            } {
                cx.spawn(|_, _| async move {
                    if let Err(error) = rpc
                        .send(proto::UpdateBuffer {
                            worktree_id: remote_id,
                            buffer_id,
                            operations: Some(operation).iter().map(Into::into).collect(),
                        })
                        .await
                    {
                        log::error!("error sending buffer operation: {}", error);
                    }
                })
                .detach();
            }
        });
    }

    pub fn buffer_removed(&self, buffer_id: u64, cx: &mut MutableAppContext) {
        self.worktree.update(cx, |worktree, cx| {
            if let Worktree::Remote(worktree) = worktree {
                let worktree_id = worktree.remote_id;
                let rpc = worktree.rpc.clone();
                cx.background()
                    .spawn(async move {
                        if let Err(error) = rpc
                            .send(proto::CloseBuffer {
                                worktree_id,
                                buffer_id,
                            })
                            .await
                        {
                            log::error!("error closing remote buffer: {}", error);
                        };
                    })
                    .detach();
            }
        });
    }

    /// Returns this file's path relative to the root of its worktree.
    pub fn path(&self) -> Arc<Path> {
        self.path.clone()
    }

    pub fn abs_path(&self, cx: &AppContext) -> PathBuf {
        self.worktree.read(cx).abs_path.join(&self.path)
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    pub fn file_name<'a>(&'a self, cx: &'a AppContext) -> Option<OsString> {
        self.path
            .file_name()
            .or_else(|| Some(OsStr::new(self.worktree.read(cx).root_name())))
            .map(Into::into)
    }

    pub fn is_deleted(&self) -> bool {
        self.entry_id.is_none()
    }

    pub fn exists(&self) -> bool {
        !self.is_deleted()
    }

    pub fn save(
        &self,
        buffer_id: u64,
        text: Rope,
        version: time::Global,
        cx: &mut MutableAppContext,
    ) -> Task<Result<(time::Global, SystemTime)>> {
        self.worktree.update(cx, |worktree, cx| match worktree {
            Worktree::Local(worktree) => {
                let rpc = worktree.rpc.clone();
                let save = worktree.save(self.path.clone(), text, cx);
                cx.spawn(|_, _| async move {
                    let entry = save.await?;
                    if let Some((rpc, worktree_id)) = rpc {
                        rpc.send(proto::BufferSaved {
                            worktree_id,
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
                let rpc = worktree.rpc.clone();
                let worktree_id = worktree.remote_id;
                cx.spawn(|_, _| async move {
                    let response = rpc
                        .request(proto::SaveBuffer {
                            worktree_id,
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

    pub fn worktree_id(&self) -> usize {
        self.worktree.id()
    }

    pub fn entry_id(&self) -> (usize, Arc<Path>) {
        (self.worktree.id(), self.path.clone())
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
        let file_count;
        let visible_file_count;
        if self.is_file() {
            file_count = 1;
            if self.is_ignored {
                visible_file_count = 0;
            } else {
                visible_file_count = 1;
            }
        } else {
            file_count = 0;
            visible_file_count = 0;
        }

        EntrySummary {
            max_path: self.path.clone(),
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
    file_count: usize,
    visible_file_count: usize,
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(Path::new("")),
            file_count: 0,
            visible_file_count: 0,
        }
    }
}

impl sum_tree::Summary for EntrySummary {
    type Context = ();

    fn add_summary(&mut self, rhs: &Self, _: &()) {
        self.max_path = rhs.max_path.clone();
        self.file_count += rhs.file_count;
        self.visible_file_count += rhs.visible_file_count;
    }
}

#[derive(Clone, Debug)]
struct PathEntry {
    id: usize,
    path: Arc<Path>,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PathSearch<'a> {
    Exact(&'a Path),
    Successor(&'a Path),
}

impl<'a> Ord for PathSearch<'a> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Self::Exact(a), Self::Exact(b)) => a.cmp(b),
            (Self::Successor(a), Self::Exact(b)) => {
                if b.starts_with(a) {
                    cmp::Ordering::Greater
                } else {
                    a.cmp(b)
                }
            }
            _ => unreachable!("not sure we need the other two cases"),
        }
    }
}

impl<'a> PartialOrd for PathSearch<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Default for PathSearch<'a> {
    fn default() -> Self {
        Self::Exact(Path::new("").into())
    }
}

impl<'a: 'b, 'b> sum_tree::Dimension<'a, EntrySummary> for PathSearch<'b> {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        *self = Self::Exact(summary.max_path.as_ref());
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FileCount(usize);

impl<'a> sum_tree::Dimension<'a, EntrySummary> for FileCount {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.0 += summary.file_count;
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VisibleFileCount(usize);

impl<'a> sum_tree::Dimension<'a, EntrySummary> for VisibleFileCount {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.0 += summary.visible_file_count;
    }
}

struct BackgroundScanner {
    fs: Arc<dyn Fs>,
    snapshot: Arc<Mutex<Snapshot>>,
    notify: Sender<ScanState>,
    executor: Arc<executor::Background>,
}

impl BackgroundScanner {
    fn new(
        snapshot: Arc<Mutex<Snapshot>>,
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

    fn snapshot(&self) -> Snapshot {
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
                let (ignore, err) = Gitignore::new(&child_abs_path);
                if let Some(err) = err {
                    log::error!("error in ignore file {:?} - {:?}", child_name, err);
                }
                let ignore = Arc::new(ignore);
                ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
                new_ignore = Some(ignore);

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
                    snapshot.insert_entry(fs_entry);
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

    async fn update_ignore_status(&self, job: UpdateIgnoreStatusJob, snapshot: &Snapshot) {
        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores.get(&job.path) {
            ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
        }

        let mut edits = Vec::new();
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
                edits.push(Edit::Insert(entry));
            }
        }
        self.snapshot.lock().entries_by_path.edit(edits, &());
    }
}

async fn refresh_entry(
    fs: &dyn Fs,
    snapshot: &Mutex<Snapshot>,
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
    Ok(snapshot.lock().insert_entry(entry))
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
        let root_path = cx.read(|cx| self.read(cx).abs_path.clone());
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

pub enum FileIter<'a> {
    All(Cursor<'a, Entry, FileCount, ()>),
    Visible(Cursor<'a, Entry, VisibleFileCount, ()>),
}

impl<'a> FileIter<'a> {
    fn all(snapshot: &'a Snapshot, start: usize) -> Self {
        let mut cursor = snapshot.entries_by_path.cursor();
        cursor.seek(&FileCount(start), Bias::Right, &());
        Self::All(cursor)
    }

    fn visible(snapshot: &'a Snapshot, start: usize) -> Self {
        let mut cursor = snapshot.entries_by_path.cursor();
        cursor.seek(&VisibleFileCount(start), Bias::Right, &());
        Self::Visible(cursor)
    }

    fn next_internal(&mut self) {
        match self {
            Self::All(cursor) => {
                let ix = *cursor.seek_start();
                cursor.seek_forward(&FileCount(ix.0 + 1), Bias::Right, &());
            }
            Self::Visible(cursor) => {
                let ix = *cursor.seek_start();
                cursor.seek_forward(&VisibleFileCount(ix.0 + 1), Bias::Right, &());
            }
        }
    }

    fn item(&self) -> Option<&'a Entry> {
        match self {
            Self::All(cursor) => cursor.item(),
            Self::Visible(cursor) => cursor.item(),
        }
    }
}

impl<'a> Iterator for FileIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.item() {
            self.next_internal();
            Some(entry)
        } else {
            None
        }
    }
}

struct ChildEntriesIter<'a> {
    parent_path: &'a Path,
    cursor: Cursor<'a, Entry, PathSearch<'a>, ()>,
}

impl<'a> ChildEntriesIter<'a> {
    fn new(parent_path: &'a Path, snapshot: &'a Snapshot) -> Self {
        let mut cursor = snapshot.entries_by_path.cursor();
        cursor.seek(&PathSearch::Exact(parent_path), Bias::Right, &());
        Self {
            parent_path,
            cursor,
        }
    }
}

impl<'a> Iterator for ChildEntriesIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.cursor.item() {
            if item.path.starts_with(self.parent_path) {
                self.cursor
                    .seek_forward(&PathSearch::Successor(&item.path), Bias::Left, &());
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
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

mod remote {
    use super::*;

    pub async fn add_peer(
        envelope: TypedEnvelope<proto::AddPeer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        rpc.state
            .read()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?
            .update(cx, |worktree, cx| worktree.add_peer(envelope, cx))
    }

    pub async fn remove_peer(
        envelope: TypedEnvelope<proto::RemovePeer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        rpc.state
            .read()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?
            .update(cx, |worktree, cx| worktree.remove_peer(envelope, cx))
    }

    pub async fn update_worktree(
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        rpc.state
            .read()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?
            .update(cx, |worktree, _| {
                if let Some(worktree) = worktree.as_remote_mut() {
                    let mut tx = worktree.updates_tx.clone();
                    Ok(async move {
                        tx.send(envelope.payload)
                            .await
                            .expect("receiver runs to completion");
                    })
                } else {
                    Err(anyhow!(
                        "invalid update message for local worktree {}",
                        envelope.payload.worktree_id
                    ))
                }
            })?
            .await;

        Ok(())
    }

    pub async fn open_buffer(
        envelope: TypedEnvelope<proto::OpenBuffer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        let receipt = envelope.receipt();
        let worktree = rpc
            .state
            .read()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?;

        let response = worktree
            .update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .open_remote_buffer(envelope, cx)
            })
            .await?;

        rpc.respond(receipt, response).await?;

        Ok(())
    }

    pub async fn close_buffer(
        envelope: TypedEnvelope<proto::CloseBuffer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        let worktree = rpc
            .state
            .read()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?;

        worktree.update(cx, |worktree, cx| {
            worktree
                .as_local_mut()
                .unwrap()
                .close_remote_buffer(envelope, cx)
        })
    }

    pub async fn update_buffer(
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        let message = envelope.payload;
        rpc.state
            .read()
            .await
            .shared_worktree(message.worktree_id, cx)?
            .update(cx, |tree, cx| tree.update_buffer(message, cx))?;
        Ok(())
    }

    pub async fn save_buffer(
        envelope: TypedEnvelope<proto::SaveBuffer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        let state = rpc.state.read().await;
        let worktree = state.shared_worktree(envelope.payload.worktree_id, cx)?;
        let sender_id = envelope.original_sender_id()?;
        let buffer = worktree.read_with(cx, |tree, _| {
            tree.as_local()
                .unwrap()
                .shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))
        })?;
        let (version, mtime) = buffer.update(cx, |buffer, cx| buffer.save(cx))?.await?;
        rpc.respond(
            envelope.receipt(),
            proto::BufferSaved {
                worktree_id: envelope.payload.worktree_id,
                buffer_id: envelope.payload.buffer_id,
                version: (&version).into(),
                mtime: Some(mtime.into()),
            },
        )
        .await?;
        Ok(())
    }

    pub async fn buffer_saved(
        envelope: TypedEnvelope<proto::BufferSaved>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        rpc.state
            .read()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?
            .update(cx, |worktree, cx| {
                worktree.buffer_saved(envelope.payload, cx)
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use anyhow::Result;
    use fs::RealFs;
    use rand::prelude::*;
    use serde_json::json;
    use std::time::UNIX_EPOCH;
    use std::{env, fmt::Write, os::unix, time::SystemTime};

    #[gpui::test]
    async fn test_populate_and_search(cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "apple": "",
                "banana": {
                    "carrot": {
                        "date": "",
                        "endive": "",
                    }
                },
                "fennel": {
                    "grape": "",
                }
            }
        }));

        let root_link_path = dir.path().join("root_link");
        unix::fs::symlink(&dir.path().join("root"), &root_link_path).unwrap();
        unix::fs::symlink(
            &dir.path().join("root/fennel"),
            &dir.path().join("root/finnochio"),
        )
        .unwrap();

        let tree = Worktree::open_local(
            root_link_path,
            Default::default(),
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        let snapshots = [cx.read(|cx| {
            let tree = tree.read(cx);
            assert_eq!(tree.file_count(), 5);
            assert_eq!(
                tree.inode_for_path("fennel/grape"),
                tree.inode_for_path("finnochio/grape")
            );
            tree.snapshot()
        })];
        let cancel_flag = Default::default();
        let results = cx
            .read(|cx| {
                match_paths(
                    &snapshots,
                    "bna",
                    false,
                    false,
                    10,
                    &cancel_flag,
                    cx.background().clone(),
                )
            })
            .await;
        assert_eq!(
            results
                .into_iter()
                .map(|result| result.path)
                .collect::<Vec<Arc<Path>>>(),
            vec![
                PathBuf::from("banana/carrot/date").into(),
                PathBuf::from("banana/carrot/endive").into(),
            ]
        );
    }

    #[gpui::test]
    async fn test_search_worktree_without_files(cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "dir1": {},
                "dir2": {
                    "dir3": {}
                }
            }
        }));
        let tree = Worktree::open_local(
            dir.path(),
            Default::default(),
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        let snapshots = [cx.read(|cx| {
            let tree = tree.read(cx);
            assert_eq!(tree.file_count(), 0);
            tree.snapshot()
        })];
        let cancel_flag = Default::default();
        let results = cx
            .read(|cx| {
                match_paths(
                    &snapshots,
                    "dir",
                    false,
                    false,
                    10,
                    &cancel_flag,
                    cx.background().clone(),
                )
            })
            .await;
        assert_eq!(
            results
                .into_iter()
                .map(|result| result.path)
                .collect::<Vec<Arc<Path>>>(),
            vec![]
        );
    }

    #[gpui::test]
    async fn test_save_file(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));
        let tree = Worktree::open_local(
            dir.path(),
            app_state.languages.clone(),
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        let buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();
        let save = buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
            buffer.save(cx).unwrap()
        });
        save.await.unwrap();

        let new_text = std::fs::read_to_string(dir.path().join("file1")).unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));
        let file_path = dir.path().join("file1");

        let tree = Worktree::open_local(
            file_path.clone(),
            app_state.languages.clone(),
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| assert_eq!(tree.read(cx).file_count(), 1));

        let buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("", cx))
            .await
            .unwrap();
        let save = buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
            buffer.save(cx).unwrap()
        });
        save.await.unwrap();

        let new_text = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_rescan_and_remote_updates(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "a": {
                "file1": "",
                "file2": "",
                "file3": "",
            },
            "b": {
                "c": {
                    "file4": "",
                    "file5": "",
                }
            }
        }));

        let tree = Worktree::open_local(
            dir.path(),
            Default::default(),
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
            let buffer = tree.update(cx, |tree, cx| tree.open_buffer(path, cx));
            async move { buffer.await.unwrap() }
        };
        let id_for_path = |path: &'static str, cx: &gpui::TestAppContext| {
            tree.read_with(cx, |tree, _| {
                tree.entry_for_path(path)
                    .expect(&format!("no entry for path {}", path))
                    .id
            })
        };

        let buffer2 = buffer_for_path("a/file2", &mut cx).await;
        let buffer3 = buffer_for_path("a/file3", &mut cx).await;
        let buffer4 = buffer_for_path("b/c/file4", &mut cx).await;
        let buffer5 = buffer_for_path("b/c/file5", &mut cx).await;

        let file2_id = id_for_path("a/file2", &cx);
        let file3_id = id_for_path("a/file3", &cx);
        let file4_id = id_for_path("b/c/file4", &cx);

        // Wait for the initial scan.
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Create a remote copy of this worktree.
        let initial_snapshot = tree.read_with(&cx, |tree, _| tree.snapshot());
        let worktree_id = 1;
        let share_request = tree
            .update(&mut cx, |tree, cx| {
                tree.as_local().unwrap().share_request(cx)
            })
            .await;
        let remote = Worktree::remote(
            proto::OpenWorktreeResponse {
                worktree_id,
                worktree: share_request.worktree,
                replica_id: 1,
                peers: Vec::new(),
            },
            rpc::Client::new(Default::default()),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| {
            assert!(!buffer2.read(cx).is_dirty());
            assert!(!buffer3.read(cx).is_dirty());
            assert!(!buffer4.read(cx).is_dirty());
            assert!(!buffer5.read(cx).is_dirty());
        });

        // Rename and delete files and directories.
        tree.flush_fs_events(&cx).await;
        std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
        std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
        std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
        std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
        tree.flush_fs_events(&cx).await;

        let expected_paths = vec![
            "a",
            "a/file1",
            "a/file2.new",
            "b",
            "d",
            "d/file3",
            "d/file4",
        ];

        cx.read(|app| {
            assert_eq!(
                tree.read(app)
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                expected_paths
            );

            assert_eq!(id_for_path("a/file2.new", &cx), file2_id);
            assert_eq!(id_for_path("d/file3", &cx), file3_id);
            assert_eq!(id_for_path("d/file4", &cx), file4_id);

            assert_eq!(
                buffer2.read(app).file().unwrap().path().as_ref(),
                Path::new("a/file2.new")
            );
            assert_eq!(
                buffer3.read(app).file().unwrap().path().as_ref(),
                Path::new("d/file3")
            );
            assert_eq!(
                buffer4.read(app).file().unwrap().path().as_ref(),
                Path::new("d/file4")
            );
            assert_eq!(
                buffer5.read(app).file().unwrap().path().as_ref(),
                Path::new("b/c/file5")
            );

            assert!(!buffer2.read(app).file().unwrap().is_deleted());
            assert!(!buffer3.read(app).file().unwrap().is_deleted());
            assert!(!buffer4.read(app).file().unwrap().is_deleted());
            assert!(buffer5.read(app).file().unwrap().is_deleted());
        });

        // Update the remote worktree. Check that it becomes consistent with the
        // local worktree.
        remote.update(&mut cx, |remote, cx| {
            let update_message = tree
                .read(cx)
                .snapshot()
                .build_update(&initial_snapshot, worktree_id);
            remote
                .as_remote_mut()
                .unwrap()
                .snapshot
                .apply_update(update_message)
                .unwrap();

            assert_eq!(
                remote
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                expected_paths
            );
        });
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

        let tree = Worktree::open_local(
            dir.path(),
            Default::default(),
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
        let mut initial_snapshot = Snapshot {
            id: 0,
            scan_id: 0,
            abs_path: root_dir.path().into(),
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
            removed_entry_ids: Default::default(),
            ignores: Default::default(),
            root_name: Default::default(),
            root_char_bag: Default::default(),
            next_entry_id: next_entry_id.clone(),
        };
        initial_snapshot.insert_entry(Entry::new(
            Path::new("").into(),
            &smol::block_on(fs.metadata(root_dir.path()))
                .unwrap()
                .unwrap(),
            &next_entry_id,
            Default::default(),
        ));
        let mut scanner = BackgroundScanner::new(
            Arc::new(Mutex::new(initial_snapshot.clone())),
            notify_tx,
            fs.clone(),
            Arc::new(gpui::executor::Background::new()),
        );
        smol::block_on(scanner.scan_dirs()).unwrap();
        scanner.snapshot().check_invariants();

        let mut events = Vec::new();
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
        assert_eq!(scanner.snapshot().to_vec(), new_scanner.snapshot().to_vec());
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

    impl Snapshot {
        fn check_invariants(&self) {
            let mut files = self.files(0);
            let mut visible_files = self.visible_files(0);
            for entry in self.entries_by_path.cursor::<(), ()>() {
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
                .cursor::<(), ()>()
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

        fn to_vec(&self) -> Vec<(&Path, u64, bool)> {
            let mut paths = Vec::new();
            for entry in self.entries_by_path.cursor::<(), ()>() {
                paths.push((entry.path.as_ref(), entry.inode, entry.is_ignored));
            }
            paths.sort_by(|a, b| a.0.cmp(&b.0));
            paths
        }
    }
}
