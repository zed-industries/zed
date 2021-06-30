mod char_bag;
mod fuzzy;
mod ignore;

use self::{char_bag::CharBag, ignore::IgnoreStack};
use crate::{
    editor::{self, Buffer, History, Operation, Rope},
    language::LanguageRegistry,
    rpc::{self, proto},
    sum_tree::{self, Cursor, Edit, SumTree},
    time::ReplicaId,
    util::Bias,
};
use ::ignore::gitignore::Gitignore;
use anyhow::{anyhow, Context, Result};
use atomic::Ordering::SeqCst;
pub use fuzzy::{match_paths, PathMatch};
use gpui::{
    scoped_pool, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task, WeakModelHandle,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::{
    prelude::{Sink, Stream},
    watch,
};
use smol::{
    channel::Sender,
    io::{AsyncReadExt, AsyncWriteExt},
};
use std::{
    cmp,
    collections::HashMap,
    convert::TryInto,
    ffi::{OsStr, OsString},
    fmt, fs,
    future::Future,
    io,
    ops::Deref,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{
        atomic::{self, AtomicUsize},
        Arc,
    },
    time::{Duration, SystemTime},
};
use zed_rpc::{PeerId, TypedEnvelope};

lazy_static! {
    static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
}

pub fn init(cx: &mut MutableAppContext, rpc: rpc::Client) {
    rpc.on_message(remote::open_buffer, cx);
    rpc.on_message(remote::close_buffer, cx);
    rpc.on_message(remote::update_buffer, cx);
    rpc.on_message(remote::remove_guest, cx);
}

#[derive(Clone, Debug)]
enum ScanState {
    Idle,
    Scanning,
    Err(Arc<io::Error>),
}

pub enum Worktree {
    Local(LocalWorktree),
    Remote(RemoteWorktree),
}

impl Entity for Worktree {
    type Event = ();
}

impl Worktree {
    pub fn local(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        Worktree::Local(LocalWorktree::new(path, languages, cx))
    }

    pub async fn remote(
        rpc: rpc::Client,
        id: u64,
        access_token: String,
        languages: Arc<LanguageRegistry>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let open_worktree_response = rpc
            .request(proto::OpenWorktree {
                worktree_id: id,
                access_token,
            })
            .await?;
        let worktree_message = open_worktree_response
            .worktree
            .ok_or_else(|| anyhow!("empty worktree"))?;
        let replica_id = open_worktree_response
            .replica_id
            .ok_or_else(|| anyhow!("empty replica id"))?;
        let worktree = cx.update(|cx| {
            cx.add_model(|cx| {
                Worktree::Remote(RemoteWorktree::new(
                    id,
                    worktree_message,
                    rpc.clone(),
                    replica_id as ReplicaId,
                    languages,
                    cx,
                ))
            })
        });
        rpc.state
            .lock()
            .await
            .shared_worktrees
            .insert(id, worktree.downgrade());
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
            Worktree::Remote(worktree) => worktree.snapshot.clone(),
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

    pub fn has_open_buffer(&self, path: impl AsRef<Path>, cx: &AppContext) -> bool {
        let open_buffers = match self {
            Worktree::Local(worktree) => &worktree.open_buffers,
            Worktree::Remote(worktree) => &worktree.open_buffers,
        };

        let path = path.as_ref();
        open_buffers
            .values()
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
        let open_buffers = match self {
            Worktree::Local(worktree) => &worktree.open_buffers,
            Worktree::Remote(worktree) => &worktree.open_buffers,
        };
        let buffer = open_buffers
            .get(&(envelope.buffer_id as usize))
            .and_then(|buf| buf.upgrade(&cx))
            .ok_or_else(|| {
                anyhow!(
                    "invalid buffer {} in update buffer message",
                    envelope.buffer_id
                )
            })?;
        let ops = envelope
            .operations
            .into_iter()
            .map(|op| op.try_into())
            .collect::<anyhow::Result<Vec<_>>>()?;
        buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
        Ok(())
    }

    fn save(
        &self,
        path: &Path,
        text: Rope,
        cx: &mut ModelContext<Self>,
    ) -> impl Future<Output = Result<()>> {
        match self {
            Worktree::Local(worktree) => {
                let save = worktree.save(path, text, cx);
                async move {
                    save.await?;
                    Ok(())
                }
            }
            Worktree::Remote(_) => todo!(),
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
    scan_state: (watch::Sender<ScanState>, watch::Receiver<ScanState>),
    _event_stream_handle: fsevent::Handle,
    poll_scheduled: bool,
    rpc: Option<(rpc::Client, u64)>,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
    shared_buffers: HashMap<PeerId, HashMap<u64, ModelHandle<Buffer>>>,
    languages: Arc<LanguageRegistry>,
}

impl LocalWorktree {
    fn new(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        let abs_path = path.into();
        let (scan_state_tx, scan_state_rx) = smol::channel::unbounded();
        let id = cx.model_id();
        let snapshot = Snapshot {
            id,
            scan_id: 0,
            abs_path,
            root_name: Default::default(),
            root_char_bag: Default::default(),
            ignores: Default::default(),
            entries: Default::default(),
            paths_by_id: Default::default(),
            removed_entry_ids: Default::default(),
            next_entry_id: Default::default(),
        };
        let (event_stream, event_stream_handle) =
            fsevent::EventStream::new(&[snapshot.abs_path.as_ref()], Duration::from_millis(100));

        let background_snapshot = Arc::new(Mutex::new(snapshot.clone()));

        let tree = Self {
            snapshot,
            background_snapshot: background_snapshot.clone(),
            scan_state: watch::channel_with(ScanState::Scanning),
            _event_stream_handle: event_stream_handle,
            poll_scheduled: false,
            open_buffers: Default::default(),
            shared_buffers: Default::default(),
            rpc: None,
            languages,
        };

        std::thread::spawn(move || {
            let scanner = BackgroundScanner::new(background_snapshot, scan_state_tx, id);
            scanner.run(event_stream)
        });

        cx.spawn(|this, mut cx| {
            let this = this.downgrade();
            async move {
                while let Ok(scan_state) = scan_state_rx.recv().await {
                    let alive = cx.update(|cx| {
                        if let Some(handle) = this.upgrade(&cx) {
                            handle.update(cx, |this, cx| {
                                if let Worktree::Local(worktree) = this {
                                    worktree.observe_scan_state(scan_state, cx)
                                } else {
                                    unreachable!()
                                }
                            });
                            true
                        } else {
                            false
                        }
                    });

                    if !alive {
                        break;
                    }
                }
            }
        })
        .detach();

        tree
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
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&envelope.original_sender_id()?) {
            shared_buffers.remove(&envelope.payload.buffer_id);
        }

        Ok(())
    }

    pub fn remove_guest(
        &mut self,
        envelope: TypedEnvelope<proto::RemoveGuest>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        self.shared_buffers.remove(&envelope.original_sender_id()?);
        Ok(())
    }

    pub fn scan_complete(&self) -> impl Future<Output = ()> {
        let mut scan_state_rx = self.scan_state.1.clone();
        async move {
            let mut scan_state = Some(scan_state_rx.borrow().clone());
            while let Some(ScanState::Scanning) = scan_state {
                scan_state = scan_state_rx.recv().await;
            }
        }
    }

    fn observe_scan_state(&mut self, scan_state: ScanState, cx: &mut ModelContext<Worktree>) {
        self.scan_state.0.blocking_send(scan_state).ok();
        self.poll_snapshot(cx);
    }

    fn poll_snapshot(&mut self, cx: &mut ModelContext<Worktree>) {
        self.snapshot = self.background_snapshot.lock().clone();
        if self.is_scanning() {
            if !self.poll_scheduled {
                cx.spawn(|this, mut cx| async move {
                    smol::Timer::after(Duration::from_millis(100)).await;
                    this.update(&mut cx, |this, cx| {
                        let worktree = this.as_local_mut().unwrap();
                        worktree.poll_scheduled = false;
                        worktree.poll_snapshot(cx);
                    })
                })
                .detach();
                self.poll_scheduled = true;
            }
        } else {
            let mut buffers_to_delete = Vec::new();
            for (buffer_id, buffer) in &self.open_buffers {
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
                                    if buffer_is_clean {
                                        let abs_path = self.absolutize(&file.path);
                                        refresh_buffer(abs_path, cx);
                                    }
                                }
                            } else if let Some(entry) = self.entry_for_path(&file.path) {
                                file.entry_id = Some(entry.id);
                                file.mtime = entry.mtime;
                                if buffer_is_clean {
                                    let abs_path = self.absolutize(&file.path);
                                    refresh_buffer(abs_path, cx);
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
                self.open_buffers.remove(&buffer_id);
            }
        }

        cx.notify();
    }

    fn is_scanning(&self) -> bool {
        if let ScanState::Scanning = *self.scan_state.1.borrow() {
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
        cx.spawn(|this, mut cx| async move {
            let mut file = smol::fs::File::open(&abs_path).await?;
            let mut text = String::new();
            file.read_to_string(&mut text).await?;
            // Eagerly populate the snapshot with an updated entry for the loaded file
            let entry = refresh_entry(&background_snapshot, path, &abs_path)?;
            this.update(&mut cx, |this, cx| {
                let this = this.as_local_mut().unwrap();
                this.poll_snapshot(cx);
            });
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

        let save = cx.background().spawn(async move {
            let buffer_size = text.summary().bytes.min(10 * 1024);
            let file = smol::fs::File::create(&abs_path).await?;
            let mut writer = smol::io::BufWriter::with_capacity(buffer_size, file);
            for chunk in text.chunks() {
                writer.write_all(chunk.as_bytes()).await?;
            }
            writer.flush().await?;
            refresh_entry(&background_snapshot, path.clone(), &abs_path)
        });

        cx.spawn(|this, mut cx| async move {
            let entry = save.await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut().unwrap().poll_snapshot(cx);
            });
            Ok(entry)
        })
    }

    pub fn share(
        &mut self,
        rpc: rpc::Client,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<anyhow::Result<(u64, String)>> {
        let root_name = self.root_name.clone();
        let snapshot = self.snapshot();
        let handle = cx.handle();
        cx.spawn(|this, mut cx| async move {
            let entries = cx
                .background()
                .spawn(async move {
                    snapshot
                        .entries
                        .cursor::<(), ()>()
                        .map(|entry| proto::Entry {
                            id: entry.id as u64,
                            is_dir: entry.is_dir(),
                            path: entry.path.to_string_lossy().to_string(),
                            inode: entry.inode,
                            mtime: Some(entry.mtime.into()),
                            is_symlink: entry.is_symlink,
                            is_ignored: entry.is_ignored,
                        })
                        .collect()
                })
                .await;

            let share_response = rpc
                .request(proto::ShareWorktree {
                    worktree: Some(proto::Worktree { root_name, entries }),
                })
                .await?;

            rpc.state
                .lock()
                .await
                .shared_worktrees
                .insert(share_response.worktree_id, handle.downgrade());

            log::info!("sharing worktree {:?}", share_response);

            this.update(&mut cx, |worktree, _| {
                worktree.as_local_mut().unwrap().rpc = Some((rpc, share_response.worktree_id));
            });
            Ok((share_response.worktree_id, share_response.access_token))
        })
    }
}

pub fn refresh_buffer(abs_path: PathBuf, cx: &mut ModelContext<Buffer>) {
    cx.spawn(|buffer, mut cx| async move {
        let new_text = cx
            .background()
            .spawn(async move {
                let mut file = smol::fs::File::open(&abs_path).await?;
                let mut text = String::new();
                file.read_to_string(&mut text).await?;
                Ok::<_, anyhow::Error>(text.into())
            })
            .await;

        match new_text {
            Err(error) => log::error!("error refreshing buffer after file changed: {}", error),
            Ok(new_text) => {
                buffer
                    .update(&mut cx, |buffer, cx| {
                        buffer.set_text_from_disk(new_text, cx)
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
    rpc: rpc::Client,
    replica_id: ReplicaId,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
    languages: Arc<LanguageRegistry>,
}

impl RemoteWorktree {
    fn new(
        remote_id: u64,
        worktree: proto::Worktree,
        rpc: rpc::Client,
        replica_id: ReplicaId,
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        let root_char_bag: CharBag = worktree
            .root_name
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let mut entries = SumTree::new();
        let mut paths_by_id = rpds::HashTrieMapSync::default();
        for entry in worktree.entries {
            if let Some(mtime) = entry.mtime {
                let kind = if entry.is_dir {
                    EntryKind::Dir
                } else {
                    let mut char_bag = root_char_bag.clone();
                    char_bag.extend(entry.path.chars().map(|c| c.to_ascii_lowercase()));
                    EntryKind::File(char_bag)
                };
                let path: Arc<Path> = Arc::from(Path::new(&entry.path));
                entries.push(
                    Entry {
                        id: entry.id as usize,
                        kind,
                        path: path.clone(),
                        inode: entry.inode,
                        mtime: mtime.into(),
                        is_symlink: entry.is_symlink,
                        is_ignored: entry.is_ignored,
                    },
                    &(),
                );
                paths_by_id.insert_mut(entry.id as usize, path);
            } else {
                log::warn!("missing mtime in remote worktree entry {:?}", entry.path);
            }
        }
        let snapshot = Snapshot {
            id: cx.model_id(),
            scan_id: 0,
            abs_path: Path::new("").into(),
            root_name: worktree.root_name,
            root_char_bag,
            ignores: Default::default(),
            entries,
            paths_by_id,
            removed_entry_ids: Default::default(),
            next_entry_id: Default::default(),
        };
        Self {
            remote_id,
            snapshot,
            rpc,
            replica_id,
            open_buffers: Default::default(),
            languages,
        }
    }

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
                let buffer_id = remote_buffer.id;
                let buffer = cx.add_model(|cx| {
                    Buffer::from_proto(replica_id, remote_buffer, Some(file), language, cx).unwrap()
                });
                this.update(&mut cx, |this, _| {
                    let this = this.as_remote_mut().unwrap();
                    this.open_buffers
                        .insert(buffer_id as usize, buffer.downgrade());
                });
                Ok(buffer)
            }
        })
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
    entries: SumTree<Entry>,
    paths_by_id: rpds::HashTrieMapSync<usize, Arc<Path>>,
    removed_entry_ids: HashMap<u64, usize>,
    next_entry_id: Arc<AtomicUsize>,
}

impl Snapshot {
    pub fn file_count(&self) -> usize {
        self.entries.summary().file_count
    }

    pub fn visible_file_count(&self) -> usize {
        self.entries.summary().visible_file_count
    }

    pub fn files(&self, start: usize) -> FileIter {
        FileIter::all(self, start)
    }

    pub fn paths(&self) -> impl Iterator<Item = &Arc<Path>> {
        let empty_path = Path::new("");
        self.entries
            .cursor::<(), ()>()
            .filter(move |entry| entry.path.as_ref() != empty_path)
            .map(|entry| entry.path())
    }

    pub fn visible_files(&self, start: usize) -> FileIter {
        FileIter::visible(self, start)
    }

    fn child_entries<'a>(&'a self, path: &'a Path) -> ChildEntriesIter<'a> {
        ChildEntriesIter::new(path, self)
    }

    pub fn root_entry(&self) -> &Entry {
        self.entry_for_path("").unwrap()
    }

    /// Returns the filename of the snapshot's root, plus a trailing slash if the snapshot's root is
    /// a directory.
    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    fn entry_for_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let mut cursor = self.entries.cursor::<_, ()>();
        if cursor.seek(&PathSearch::Exact(path.as_ref()), Bias::Left, &()) {
            cursor.item()
        } else {
            None
        }
    }

    fn entry_for_id(&self, id: usize) -> Option<&Entry> {
        let path = self.paths_by_id.get(&id)?;
        self.entry_for_path(path)
    }

    pub fn inode_for_path(&self, path: impl AsRef<Path>) -> Option<u64> {
        self.entry_for_path(path.as_ref()).map(|e| e.inode())
    }

    fn insert_entry(&mut self, mut entry: Entry) -> Entry {
        if !entry.is_dir() && entry.path().file_name() == Some(&GITIGNORE) {
            let (ignore, err) = Gitignore::new(self.abs_path.join(entry.path()));
            if let Some(err) = err {
                log::error!("error in ignore file {:?} - {:?}", entry.path(), err);
            }

            let ignore_dir_path = entry.path().parent().unwrap();
            self.ignores
                .insert(ignore_dir_path.into(), (Arc::new(ignore), self.scan_id));
        }

        self.reuse_entry_id(&mut entry);
        self.entries.insert_or_replace(entry.clone(), &());
        self.paths_by_id.insert_mut(entry.id, entry.path.clone());
        entry
    }

    fn populate_dir(
        &mut self,
        parent_path: Arc<Path>,
        entries: impl IntoIterator<Item = Entry>,
        ignore: Option<Arc<Gitignore>>,
    ) {
        let mut edits = Vec::new();

        let mut parent_entry = self
            .entries
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
        edits.push(Edit::Insert(parent_entry));

        for mut entry in entries {
            self.reuse_entry_id(&mut entry);
            self.paths_by_id.insert_mut(entry.id, entry.path.clone());
            edits.push(Edit::Insert(entry));
        }
        self.entries.edit(edits, &());
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
            let mut cursor = self.entries.cursor::<_, ()>();
            new_entries = cursor.slice(&PathSearch::Exact(path), Bias::Left, &());
            removed_entry_ids = cursor.slice(&PathSearch::Successor(path), Bias::Left, &());
            new_entries.push_tree(cursor.suffix(&()), &());
        }
        self.entries = new_entries;
        for entry in removed_entry_ids.cursor::<(), ()>() {
            let removed_entry_id = self
                .removed_entry_ids
                .entry(entry.inode)
                .or_insert(entry.id);
            *removed_entry_id = cmp::max(*removed_entry_id, entry.id);
            self.paths_by_id.remove_mut(&entry.id);
        }

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
        for entry in self.entries.cursor::<(), ()>() {
            for _ in entry.path().ancestors().skip(1) {
                write!(f, " ")?;
            }
            writeln!(f, "{:?} (inode: {})", entry.path(), entry.inode())?;
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
                cx.background()
                    .spawn(async move {
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

    pub fn save(&self, text: Rope, cx: &mut MutableAppContext) -> impl Future<Output = Result<()>> {
        self.worktree
            .update(cx, |worktree, cx| worktree.save(&self.path, text, cx))
    }

    pub fn worktree_id(&self) -> usize {
        self.worktree.id()
    }

    pub fn entry_id(&self) -> (usize, Arc<Path>) {
        (self.worktree.id(), self.path())
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    id: usize,
    kind: EntryKind,
    path: Arc<Path>,
    inode: u64,
    mtime: SystemTime,
    is_symlink: bool,
    is_ignored: bool,
}

#[derive(Clone, Debug)]
pub enum EntryKind {
    PendingDir,
    Dir,
    File(CharBag),
}

impl Entry {
    pub fn path(&self) -> &Arc<Path> {
        &self.path
    }

    pub fn inode(&self) -> u64 {
        self.inode
    }

    pub fn is_ignored(&self) -> bool {
        self.is_ignored
    }

    fn is_dir(&self) -> bool {
        matches!(self.kind, EntryKind::Dir | EntryKind::PendingDir)
    }

    fn is_file(&self) -> bool {
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
            max_path: self.path().clone(),
            file_count,
            visible_file_count,
        }
    }
}

impl sum_tree::KeyedItem for Entry {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        PathKey(self.path().clone())
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
    snapshot: Arc<Mutex<Snapshot>>,
    notify: Sender<ScanState>,
    thread_pool: scoped_pool::Pool,
}

impl BackgroundScanner {
    fn new(snapshot: Arc<Mutex<Snapshot>>, notify: Sender<ScanState>, worktree_id: usize) -> Self {
        Self {
            snapshot,
            notify,
            thread_pool: scoped_pool::Pool::new(16, format!("worktree-{}-scanner", worktree_id)),
        }
    }

    fn abs_path(&self) -> Arc<Path> {
        self.snapshot.lock().abs_path.clone()
    }

    fn snapshot(&self) -> Snapshot {
        self.snapshot.lock().clone()
    }

    fn run(mut self, event_stream: fsevent::EventStream) {
        if smol::block_on(self.notify.send(ScanState::Scanning)).is_err() {
            return;
        }

        if let Err(err) = self.scan_dirs() {
            if smol::block_on(self.notify.send(ScanState::Err(Arc::new(err)))).is_err() {
                return;
            }
        }

        if smol::block_on(self.notify.send(ScanState::Idle)).is_err() {
            return;
        }

        event_stream.run(move |events| {
            if smol::block_on(self.notify.send(ScanState::Scanning)).is_err() {
                return false;
            }

            if !self.process_events(events) {
                return false;
            }

            if smol::block_on(self.notify.send(ScanState::Idle)).is_err() {
                return false;
            }

            true
        });
    }

    fn scan_dirs(&mut self) -> io::Result<()> {
        self.snapshot.lock().scan_id += 1;

        let path: Arc<Path> = Arc::from(Path::new(""));
        let abs_path = self.abs_path();
        let metadata = fs::metadata(&abs_path)?;
        let inode = metadata.ino();
        let is_symlink = fs::symlink_metadata(&abs_path)?.file_type().is_symlink();
        let is_dir = metadata.file_type().is_dir();
        let mtime = metadata.modified()?;

        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let mut root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());
        if is_dir {
            root_name.push('/');
        }

        let root_char_bag = root_name.chars().map(|c| c.to_ascii_lowercase()).collect();
        let next_entry_id;
        {
            let mut snapshot = self.snapshot.lock();
            snapshot.root_name = root_name;
            snapshot.root_char_bag = root_char_bag;
            next_entry_id = snapshot.next_entry_id.clone();
        }

        if is_dir {
            self.snapshot.lock().insert_entry(Entry {
                id: next_entry_id.fetch_add(1, SeqCst),
                kind: EntryKind::PendingDir,
                path: path.clone(),
                inode,
                mtime,
                is_symlink,
                is_ignored: false,
            });

            let (tx, rx) = crossbeam_channel::unbounded();
            tx.send(ScanJob {
                abs_path: abs_path.to_path_buf(),
                path,
                ignore_stack: IgnoreStack::none(),
                scan_queue: tx.clone(),
            })
            .unwrap();
            drop(tx);

            self.thread_pool.scoped(|pool| {
                for _ in 0..self.thread_pool.thread_count() {
                    pool.execute(|| {
                        while let Ok(job) = rx.recv() {
                            if let Err(err) =
                                self.scan_dir(root_char_bag, next_entry_id.clone(), &job)
                            {
                                log::error!("error scanning {:?}: {}", job.abs_path, err);
                            }
                        }
                    });
                }
            });
        } else {
            self.snapshot.lock().insert_entry(Entry {
                id: next_entry_id.fetch_add(1, SeqCst),
                kind: EntryKind::File(char_bag_for_path(root_char_bag, &path)),
                path,
                inode,
                mtime,
                is_symlink,
                is_ignored: false,
            });
        }

        Ok(())
    }

    fn scan_dir(
        &self,
        root_char_bag: CharBag,
        next_entry_id: Arc<AtomicUsize>,
        job: &ScanJob,
    ) -> io::Result<()> {
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<ScanJob> = Vec::new();
        let mut ignore_stack = job.ignore_stack.clone();
        let mut new_ignore = None;

        for child_entry in fs::read_dir(&job.abs_path)? {
            let child_entry = child_entry?;
            let child_name = child_entry.file_name();
            let child_abs_path = job.abs_path.join(&child_name);
            let child_path: Arc<Path> = job.path.join(&child_name).into();
            let child_is_symlink = child_entry.metadata()?.file_type().is_symlink();
            let child_metadata = if let Ok(metadata) = fs::metadata(&child_abs_path) {
                metadata
            } else {
                log::error!("could not get metadata for path {:?}", child_abs_path);
                continue;
            };

            let child_inode = child_metadata.ino();
            let child_mtime = child_metadata.modified()?;

            // If we find a .gitignore, add it to the stack of ignores used to determine which paths are ignored
            if child_name == *GITIGNORE {
                let (ignore, err) = Gitignore::new(&child_abs_path);
                if let Some(err) = err {
                    log::error!("error in ignore file {:?} - {:?}", child_path, err);
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

            if child_metadata.is_dir() {
                let is_ignored = ignore_stack.is_path_ignored(&child_path, true);
                new_entries.push(Entry {
                    id: next_entry_id.fetch_add(1, SeqCst),
                    kind: EntryKind::PendingDir,
                    path: child_path.clone(),
                    inode: child_inode,
                    mtime: child_mtime,
                    is_symlink: child_is_symlink,
                    is_ignored,
                });
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
                let is_ignored = ignore_stack.is_path_ignored(&child_path, false);
                new_entries.push(Entry {
                    id: next_entry_id.fetch_add(1, SeqCst),
                    kind: EntryKind::File(char_bag_for_path(root_char_bag, &child_path)),
                    path: child_path,
                    inode: child_inode,
                    mtime: child_mtime,
                    is_symlink: child_is_symlink,
                    is_ignored,
                });
            };
        }

        self.snapshot
            .lock()
            .populate_dir(job.path.clone(), new_entries, new_ignore);
        for new_job in new_jobs {
            job.scan_queue.send(new_job).unwrap();
        }

        Ok(())
    }

    fn process_events(&mut self, mut events: Vec<fsevent::Event>) -> bool {
        let mut snapshot = self.snapshot();
        snapshot.scan_id += 1;

        let root_abs_path = if let Ok(abs_path) = snapshot.abs_path.canonicalize() {
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

        let (scan_queue_tx, scan_queue_rx) = crossbeam_channel::unbounded();
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

            match fs_entry_for_path(
                snapshot.root_char_bag,
                &next_entry_id,
                path.clone(),
                &event.path,
            ) {
                Ok(Some(mut fs_entry)) => {
                    let is_dir = fs_entry.is_dir();
                    let ignore_stack = snapshot.ignore_stack_for_path(&path, is_dir);
                    fs_entry.is_ignored = ignore_stack.is_all();
                    snapshot.insert_entry(fs_entry);
                    if is_dir {
                        scan_queue_tx
                            .send(ScanJob {
                                abs_path: event.path,
                                path,
                                ignore_stack,
                                scan_queue: scan_queue_tx.clone(),
                            })
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
        self.thread_pool.scoped(|pool| {
            for _ in 0..self.thread_pool.thread_count() {
                pool.execute(|| {
                    while let Ok(job) = scan_queue_rx.recv() {
                        if let Err(err) = self.scan_dir(root_char_bag, next_entry_id.clone(), &job)
                        {
                            log::error!("error scanning {:?}: {}", job.abs_path, err);
                        }
                    }
                });
            }
        });

        // Attempt to detect renames only over a single batch of file-system events.
        self.snapshot.lock().removed_entry_ids.clear();

        self.update_ignore_statuses();
        true
    }

    fn update_ignore_statuses(&self) {
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

        let (ignore_queue_tx, ignore_queue_rx) = crossbeam_channel::unbounded();
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
                .unwrap();
        }
        drop(ignore_queue_tx);

        self.thread_pool.scoped(|scope| {
            for _ in 0..self.thread_pool.thread_count() {
                scope.execute(|| {
                    while let Ok(job) = ignore_queue_rx.recv() {
                        self.update_ignore_status(job, &snapshot);
                    }
                });
            }
        });
    }

    fn update_ignore_status(&self, job: UpdateIgnoreStatusJob, snapshot: &Snapshot) {
        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores.get(&job.path) {
            ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
        }

        let mut edits = Vec::new();
        for mut entry in snapshot.child_entries(&job.path).cloned() {
            let was_ignored = entry.is_ignored;
            entry.is_ignored = ignore_stack.is_path_ignored(entry.path(), entry.is_dir());
            if entry.is_dir() {
                let child_ignore_stack = if entry.is_ignored {
                    IgnoreStack::all()
                } else {
                    ignore_stack.clone()
                };
                job.ignore_queue
                    .send(UpdateIgnoreStatusJob {
                        path: entry.path().clone(),
                        ignore_stack: child_ignore_stack,
                        ignore_queue: job.ignore_queue.clone(),
                    })
                    .unwrap();
            }

            if entry.is_ignored != was_ignored {
                edits.push(Edit::Insert(entry));
            }
        }
        self.snapshot.lock().entries.edit(edits, &());
    }
}

fn refresh_entry(snapshot: &Mutex<Snapshot>, path: Arc<Path>, abs_path: &Path) -> Result<Entry> {
    let root_char_bag;
    let next_entry_id;
    {
        let snapshot = snapshot.lock();
        root_char_bag = snapshot.root_char_bag;
        next_entry_id = snapshot.next_entry_id.clone();
    }
    let entry = fs_entry_for_path(root_char_bag, &next_entry_id, path, abs_path)?
        .ok_or_else(|| anyhow!("could not read saved file metadata"))?;
    Ok(snapshot.lock().insert_entry(entry))
}

fn fs_entry_for_path(
    root_char_bag: CharBag,
    next_entry_id: &AtomicUsize,
    path: Arc<Path>,
    abs_path: &Path,
) -> Result<Option<Entry>> {
    let metadata = match fs::metadata(&abs_path) {
        Err(err) => {
            return match (err.kind(), err.raw_os_error()) {
                (io::ErrorKind::NotFound, _) => Ok(None),
                (io::ErrorKind::Other, Some(libc::ENOTDIR)) => Ok(None),
                _ => Err(anyhow::Error::new(err)),
            }
        }
        Ok(metadata) => metadata,
    };
    let inode = metadata.ino();
    let mtime = metadata.modified()?;
    let is_symlink = fs::symlink_metadata(&abs_path)
        .context("failed to read symlink metadata")?
        .file_type()
        .is_symlink();

    let entry = Entry {
        id: next_entry_id.fetch_add(1, SeqCst),
        kind: if metadata.file_type().is_dir() {
            EntryKind::PendingDir
        } else {
            EntryKind::File(char_bag_for_path(root_char_bag, &path))
        },
        path: Arc::from(path),
        inode,
        mtime,
        is_symlink,
        is_ignored: false,
    };

    Ok(Some(entry))
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
    scan_queue: crossbeam_channel::Sender<ScanJob>,
}

struct UpdateIgnoreStatusJob {
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    ignore_queue: crossbeam_channel::Sender<UpdateIgnoreStatusJob>,
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
            fs::write(root_path.join(filename), "").unwrap();
            tree.condition(&cx, |tree, _| tree.entry_for_path(filename).is_some())
                .await;

            fs::remove_file(root_path.join(filename)).unwrap();
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
        let mut cursor = snapshot.entries.cursor();
        cursor.seek(&FileCount(start), Bias::Right, &());
        Self::All(cursor)
    }

    fn visible(snapshot: &'a Snapshot, start: usize) -> Self {
        let mut cursor = snapshot.entries.cursor();
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
        let mut cursor = snapshot.entries.cursor();
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
            if item.path().starts_with(self.parent_path) {
                self.cursor
                    .seek_forward(&PathSearch::Successor(item.path()), Bias::Left, &());
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }
}

mod remote {
    use super::*;
    use crate::rpc::TypedEnvelope;

    pub async fn open_buffer(
        envelope: TypedEnvelope<proto::OpenBuffer>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        let receipt = envelope.receipt();
        let worktree = rpc
            .state
            .lock()
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
            .lock()
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
        let mut state = rpc.state.lock().await;
        match state.shared_worktree(message.worktree_id, cx) {
            Ok(worktree) => {
                if let Err(error) = worktree.update(cx, |tree, cx| tree.update_buffer(message, cx))
                {
                    log::error!("error applying operations to buffer: {}", error);
                }
            }
            Err(error) => log::error!("{}", error),
        }

        Ok(())
    }

    pub async fn remove_guest(
        envelope: TypedEnvelope<proto::RemoveGuest>,
        rpc: &rpc::Client,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<()> {
        rpc.state
            .lock()
            .await
            .shared_worktree(envelope.payload.worktree_id, cx)?
            .update(cx, |worktree, cx| match worktree {
                Worktree::Local(worktree) => worktree.remove_guest(envelope, cx),
                Worktree::Remote(_) => todo!(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use anyhow::Result;
    use rand::prelude::*;
    use serde_json::json;
    use std::time::UNIX_EPOCH;
    use std::{env, fmt::Write, os::unix, time::SystemTime};

    #[gpui::test]
    async fn test_populate_and_search(mut cx: gpui::TestAppContext) {
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

        let tree = cx.add_model(|cx| Worktree::local(root_link_path, Default::default(), cx));

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            assert_eq!(tree.file_count(), 5);

            assert_eq!(
                tree.inode_for_path("fennel/grape"),
                tree.inode_for_path("finnochio/grape")
            );

            let results = match_paths(
                Some(tree.snapshot()).iter(),
                "bna",
                false,
                false,
                false,
                10,
                Default::default(),
                cx.thread_pool().clone(),
            )
            .into_iter()
            .map(|result| result.path)
            .collect::<Vec<Arc<Path>>>();
            assert_eq!(
                results,
                vec![
                    PathBuf::from("banana/carrot/date").into(),
                    PathBuf::from("banana/carrot/endive").into(),
                ]
            );
        })
    }

    #[gpui::test]
    async fn test_save_file(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));
        let tree = cx.add_model(|cx| Worktree::local(dir.path(), app_state.languages, cx));
        let buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();
        let save = buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
            buffer.save(cx).unwrap()
        });
        save.await.unwrap();

        let new_text = fs::read_to_string(dir.path().join("file1")).unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));
        let file_path = dir.path().join("file1");

        let tree = cx.add_model(|cx| Worktree::local(file_path.clone(), app_state.languages, cx));
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

        let new_text = fs::read_to_string(file_path).unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_rescan_simple(mut cx: gpui::TestAppContext) {
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

        let tree = cx.add_model(|cx| Worktree::local(dir.path(), Default::default(), cx));

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

        // After scanning, the worktree knows which files exist and which don't.
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        cx.read(|cx| {
            assert!(!buffer2.read(cx).is_dirty());
            assert!(!buffer3.read(cx).is_dirty());
            assert!(!buffer4.read(cx).is_dirty());
            assert!(!buffer5.read(cx).is_dirty());
        });

        tree.flush_fs_events(&cx).await;
        std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
        std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
        std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
        std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
        tree.flush_fs_events(&cx).await;

        cx.read(|app| {
            assert_eq!(
                tree.read(app)
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                vec![
                    "a",
                    "a/file1",
                    "a/file2.new",
                    "b",
                    "d",
                    "d/file3",
                    "d/file4"
                ]
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
    }

    #[gpui::test]
    async fn test_rescan_with_gitignore(mut cx: gpui::TestAppContext) {
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

        let tree = cx.add_model(|cx| Worktree::local(dir.path(), Default::default(), cx));
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            let tracked = tree.entry_for_path("tracked-dir/tracked-file1").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file1").unwrap();
            assert_eq!(tracked.is_ignored(), false);
            assert_eq!(ignored.is_ignored(), true);
        });

        fs::write(dir.path().join("tracked-dir/tracked-file2"), "").unwrap();
        fs::write(dir.path().join("ignored-dir/ignored-file2"), "").unwrap();
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            let dot_git = tree.entry_for_path(".git").unwrap();
            let tracked = tree.entry_for_path("tracked-dir/tracked-file2").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file2").unwrap();
            assert_eq!(tracked.is_ignored(), false);
            assert_eq!(ignored.is_ignored(), true);
            assert_eq!(dot_git.is_ignored(), true);
        });
    }

    #[test]
    fn test_random() {
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().unwrap())
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|o| o.parse().unwrap())
            .unwrap_or(40);
        let initial_entries = env::var("INITIAL_ENTRIES")
            .map(|o| o.parse().unwrap())
            .unwrap_or(20);
        let seeds = if let Ok(seed) = env::var("SEED").map(|s| s.parse().unwrap()) {
            seed..seed + 1
        } else {
            0..iterations
        };

        for seed in seeds {
            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);

            let root_dir = tempdir::TempDir::new(&format!("test-{}", seed)).unwrap();
            for _ in 0..initial_entries {
                randomly_mutate_tree(root_dir.path(), 1.0, &mut rng).unwrap();
            }
            log::info!("Generated initial tree");

            let (notify_tx, _notify_rx) = smol::channel::unbounded();
            let mut scanner = BackgroundScanner::new(
                Arc::new(Mutex::new(Snapshot {
                    id: 0,
                    scan_id: 0,
                    abs_path: root_dir.path().into(),
                    entries: Default::default(),
                    paths_by_id: Default::default(),
                    removed_entry_ids: Default::default(),
                    ignores: Default::default(),
                    root_name: Default::default(),
                    root_char_bag: Default::default(),
                    next_entry_id: Default::default(),
                })),
                notify_tx,
                0,
            );
            scanner.scan_dirs().unwrap();
            scanner.snapshot().check_invariants();

            let mut events = Vec::new();
            let mut mutations_len = operations;
            while mutations_len > 1 {
                if !events.is_empty() && rng.gen_bool(0.4) {
                    let len = rng.gen_range(0..=events.len());
                    let to_deliver = events.drain(0..len).collect::<Vec<_>>();
                    log::info!("Delivering events: {:#?}", to_deliver);
                    scanner.process_events(to_deliver);
                    scanner.snapshot().check_invariants();
                } else {
                    events.extend(randomly_mutate_tree(root_dir.path(), 0.6, &mut rng).unwrap());
                    mutations_len -= 1;
                }
            }
            log::info!("Quiescing: {:#?}", events);
            scanner.process_events(events);
            scanner.snapshot().check_invariants();

            let (notify_tx, _notify_rx) = smol::channel::unbounded();
            let mut new_scanner = BackgroundScanner::new(
                Arc::new(Mutex::new(Snapshot {
                    id: 0,
                    scan_id: 0,
                    abs_path: root_dir.path().into(),
                    entries: Default::default(),
                    paths_by_id: Default::default(),
                    removed_entry_ids: Default::default(),
                    ignores: Default::default(),
                    root_name: Default::default(),
                    root_char_bag: Default::default(),
                    next_entry_id: Default::default(),
                })),
                notify_tx,
                1,
            );
            new_scanner.scan_dirs().unwrap();
            assert_eq!(scanner.snapshot().to_vec(), new_scanner.snapshot().to_vec());
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
                fs::create_dir(&new_path)?;
            } else {
                log::info!("Creating file {:?}", new_path.strip_prefix(root_path)?);
                fs::write(&new_path, "")?;
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
            fs::write(&ignore_path, ignore_contents).unwrap();
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
                    fs::remove_dir_all(&new_path_parent).ok();
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
                fs::rename(&old_path, &new_path)?;
                record_event(old_path.clone());
                record_event(new_path);
            } else if old_path.is_dir() {
                let (dirs, files) = read_dir_recursive(old_path.clone());

                log::info!("Deleting dir {:?}", old_path.strip_prefix(&root_path)?);
                fs::remove_dir_all(&old_path).unwrap();
                for file in files {
                    record_event(file);
                }
                for dir in dirs {
                    record_event(dir);
                }
            } else {
                log::info!("Deleting file {:?}", old_path.strip_prefix(&root_path)?);
                fs::remove_file(old_path).unwrap();
                record_event(old_path.clone());
            }
        }

        Ok(events)
    }

    fn read_dir_recursive(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let child_entries = fs::read_dir(&path).unwrap();
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
            for entry in self.entries.cursor::<(), ()>() {
                if entry.is_file() {
                    assert_eq!(files.next().unwrap().inode(), entry.inode);
                    if !entry.is_ignored {
                        assert_eq!(visible_files.next().unwrap().inode(), entry.inode);
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
                    stack.insert(ix, child_entry.path());
                }
            }

            let dfs_paths = self
                .entries
                .cursor::<(), ()>()
                .map(|e| e.path().as_ref())
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
            for entry in self.entries.cursor::<(), ()>() {
                paths.push((entry.path().as_ref(), entry.inode(), entry.is_ignored()));
            }
            paths.sort_by(|a, b| a.0.cmp(&b.0));
            paths
        }
    }
}
