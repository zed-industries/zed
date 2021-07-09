mod char_bag;
mod fuzzy;
mod ignore;

use self::{char_bag::CharBag, ignore::IgnoreStack};
use crate::{
    editor::{self, Buffer, History, Operation, Rope},
    language::LanguageRegistry,
    rpc::{self, proto},
    sum_tree::{self, Cursor, Edit, SumTree},
    time::{self, ReplicaId},
    util::Bias,
};
use ::ignore::gitignore::Gitignore;
use anyhow::{anyhow, Context, Result};
use atomic::Ordering::SeqCst;
use futures::{future, stream, Stream, StreamExt};
pub use fuzzy::{match_paths, PathMatch};
use gpui::{
    executor, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task, WeakModelHandle,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::{
    broadcast,
    prelude::{Sink as _, Stream as _},
    watch,
};
use smol::{
    channel::{self, Sender},
    io::{AsyncReadExt, AsyncWriteExt},
    lock::RwLock,
};
use std::{
    cmp::{self, Ordering},
    collections::{BTreeMap, HashMap},
    convert::{TryFrom, TryInto},
    ffi::{OsStr, OsString},
    fmt,
    future::Future,
    io,
    ops::Deref,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    pin::Pin,
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
    rpc.on_message(remote::add_peer, cx);
    rpc.on_message(remote::remove_peer, cx);
    rpc.on_message(remote::update_worktree, cx);
    rpc.on_message(remote::open_buffer, cx);
    rpc.on_message(remote::close_buffer, cx);
    rpc.on_message(remote::update_buffer, cx);
    rpc.on_message(remote::buffer_saved, cx);
    rpc.on_message(remote::save_buffer, cx);
}

#[async_trait::async_trait]
trait Fs: Send + Sync {
    async fn entry(
        &self,
        root_char_bag: CharBag,
        next_entry_id: &AtomicUsize,
        path: Arc<Path>,
        abs_path: &Path,
    ) -> Result<Option<Entry>>;
    async fn child_entries<'a>(
        &self,
        root_char_bag: CharBag,
        next_entry_id: &'a AtomicUsize,
        path: &'a Path,
        abs_path: &'a Path,
    ) -> Result<Pin<Box<dyn 'a + Stream<Item = Result<Entry>> + Send>>>;
    async fn load(&self, path: &Path) -> Result<String>;
    async fn save(&self, path: &Path, text: &Rope) -> Result<()>;
}

struct OsFs;

#[async_trait::async_trait]
impl Fs for OsFs {
    async fn entry(
        &self,
        root_char_bag: CharBag,
        next_entry_id: &AtomicUsize,
        path: Arc<Path>,
        abs_path: &Path,
    ) -> Result<Option<Entry>> {
        let metadata = match smol::fs::metadata(&abs_path).await {
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
        let is_symlink = smol::fs::symlink_metadata(&abs_path)
            .await
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

    async fn child_entries<'a>(
        &self,
        root_char_bag: CharBag,
        next_entry_id: &'a AtomicUsize,
        path: &'a Path,
        abs_path: &'a Path,
    ) -> Result<Pin<Box<dyn 'a + Stream<Item = Result<Entry>> + Send>>> {
        let entries = smol::fs::read_dir(abs_path).await?;
        Ok(entries
            .then(move |entry| async move {
                let child_entry = entry?;
                let child_name = child_entry.file_name();
                let child_path: Arc<Path> = path.join(&child_name).into();
                let child_abs_path = abs_path.join(&child_name);
                let child_is_symlink = child_entry.metadata().await?.file_type().is_symlink();
                let child_metadata = smol::fs::metadata(child_abs_path).await?;
                let child_inode = child_metadata.ino();
                let child_mtime = child_metadata.modified()?;
                Ok(Entry {
                    id: next_entry_id.fetch_add(1, SeqCst),
                    kind: if child_metadata.file_type().is_dir() {
                        EntryKind::PendingDir
                    } else {
                        EntryKind::File(char_bag_for_path(root_char_bag, &child_path))
                    },
                    path: child_path,
                    inode: child_inode,
                    mtime: child_mtime,
                    is_symlink: child_is_symlink,
                    is_ignored: false,
                })
            })
            .boxed())
    }

    async fn load(&self, path: &Path) -> Result<String> {
        let mut file = smol::fs::File::open(path).await?;
        let mut text = String::new();
        file.read_to_string(&mut text).await?;
        Ok(text)
    }

    async fn save(&self, path: &Path, text: &Rope) -> Result<()> {
        let buffer_size = text.summary().bytes.min(10 * 1024);
        let file = smol::fs::File::create(path).await?;
        let mut writer = smol::io::BufWriter::with_capacity(buffer_size, file);
        for chunk in text.chunks() {
            writer.write_all(chunk.as_bytes()).await?;
        }
        writer.flush().await?;
        Ok(())
    }
}

#[derive(Clone)]
struct InMemoryEntry {
    inode: u64,
    mtime: SystemTime,
    is_dir: bool,
    is_symlink: bool,
    content: Option<String>,
}

struct InMemoryFsState {
    entries: BTreeMap<PathBuf, InMemoryEntry>,
    next_inode: u64,
    events_tx: broadcast::Sender<fsevent::Event>,
}

impl InMemoryFsState {
    fn validate_path(&self, path: &Path) -> Result<()> {
        if path.is_absolute()
            && path
                .parent()
                .and_then(|path| self.entries.get(path))
                .map_or(false, |e| e.is_dir)
        {
            Ok(())
        } else {
            Err(anyhow!("invalid path {:?}", path))
        }
    }

    async fn emit_event(&mut self, path: &Path) {
        let _ = self
            .events_tx
            .send(fsevent::Event {
                event_id: 0,
                flags: fsevent::StreamFlags::empty(),
                path: path.to_path_buf(),
            })
            .await;
    }
}

pub struct InMemoryFs {
    state: RwLock<InMemoryFsState>,
}

impl InMemoryFs {
    pub fn new() -> Self {
        let (events_tx, _) = broadcast::channel(2048);
        let mut entries = BTreeMap::new();
        entries.insert(
            Path::new("/").to_path_buf(),
            InMemoryEntry {
                inode: 0,
                mtime: SystemTime::now(),
                is_dir: true,
                is_symlink: false,
                content: None,
            },
        );
        Self {
            state: RwLock::new(InMemoryFsState {
                entries,
                next_inode: 1,
                events_tx,
            }),
        }
    }

    pub async fn insert_dir(&self, path: &Path) -> Result<()> {
        let mut state = self.state.write().await;
        state.validate_path(path)?;

        let inode = state.next_inode;
        state.next_inode += 1;
        state.entries.insert(
            path.to_path_buf(),
            InMemoryEntry {
                inode,
                mtime: SystemTime::now(),
                is_dir: true,
                is_symlink: false,
                content: None,
            },
        );
        state.emit_event(path).await;
        Ok(())
    }

    pub async fn remove(&self, path: &Path) -> Result<()> {
        let mut state = self.state.write().await;
        state.validate_path(path)?;
        state.entries.retain(|path, _| !path.starts_with(path));
        state.emit_event(&path).await;
        Ok(())
    }

    pub async fn rename(&self, source: &Path, target: &Path) -> Result<()> {
        let mut state = self.state.write().await;
        state.validate_path(source)?;
        state.validate_path(target)?;
        if state.entries.contains_key(target) {
            Err(anyhow!("target path already exists"))
        } else {
            let mut removed = Vec::new();
            state.entries.retain(|path, entry| {
                if let Ok(relative_path) = path.strip_prefix(source) {
                    removed.push((relative_path.to_path_buf(), entry.clone()));
                    false
                } else {
                    true
                }
            });

            for (relative_path, entry) in removed {
                let new_path = target.join(relative_path);
                state.entries.insert(new_path, entry);
            }

            Ok(())
        }
    }

    pub async fn events(&self) -> broadcast::Receiver<fsevent::Event> {
        self.state.read().await.events_tx.subscribe()
    }
}

#[async_trait::async_trait]
impl Fs for InMemoryFs {
    async fn entry(
        &self,
        root_char_bag: CharBag,
        next_entry_id: &AtomicUsize,
        path: Arc<Path>,
        abs_path: &Path,
    ) -> Result<Option<Entry>> {
        let state = self.state.read().await;
        if let Some(entry) = state.entries.get(abs_path) {
            Ok(Some(Entry {
                id: next_entry_id.fetch_add(1, SeqCst),
                kind: if entry.is_dir {
                    EntryKind::PendingDir
                } else {
                    EntryKind::File(char_bag_for_path(root_char_bag, &path))
                },
                path: Arc::from(path),
                inode: entry.inode,
                mtime: entry.mtime,
                is_symlink: entry.is_symlink,
                is_ignored: false,
            }))
        } else {
            Ok(None)
        }
    }

    async fn child_entries<'a>(
        &self,
        root_char_bag: CharBag,
        next_entry_id: &'a AtomicUsize,
        path: &'a Path,
        abs_path: &'a Path,
    ) -> Result<Pin<Box<dyn 'a + Stream<Item = Result<Entry>> + Send>>> {
        let state = self.state.read().await;
        Ok(stream::iter(state.entries.clone())
            .filter(move |(child_path, _)| future::ready(child_path.parent() == Some(abs_path)))
            .then(move |(child_abs_path, child_entry)| async move {
                smol::future::yield_now().await;
                let child_path = Arc::from(path.join(child_abs_path.file_name().unwrap()));
                Ok(Entry {
                    id: next_entry_id.fetch_add(1, SeqCst),
                    kind: if child_entry.is_dir {
                        EntryKind::PendingDir
                    } else {
                        EntryKind::File(char_bag_for_path(root_char_bag, &child_path))
                    },
                    path: child_path,
                    inode: child_entry.inode,
                    mtime: child_entry.mtime,
                    is_symlink: child_entry.is_symlink,
                    is_ignored: false,
                })
            })
            .boxed())
    }

    async fn load(&self, path: &Path) -> Result<String> {
        let state = self.state.read().await;
        let text = state
            .entries
            .get(path)
            .and_then(|e| e.content.as_ref())
            .ok_or_else(|| anyhow!("file {:?} does not exist", path))?;
        Ok(text.clone())
    }

    async fn save(&self, path: &Path, text: &Rope) -> Result<()> {
        let mut state = self.state.write().await;
        state.validate_path(path)?;
        if let Some(entry) = state.entries.get_mut(path) {
            if entry.is_dir {
                Err(anyhow!("cannot overwrite a directory with a file"))
            } else {
                entry.content = Some(text.chunks().collect());
                entry.mtime = SystemTime::now();
                state.emit_event(path).await;
                Ok(())
            }
        } else {
            let inode = state.next_inode;
            state.next_inode += 1;
            state.entries.insert(
                path.to_path_buf(),
                InMemoryEntry {
                    inode,
                    mtime: SystemTime::now(),
                    is_dir: false,
                    is_symlink: false,
                    content: Some(text.chunks().collect()),
                },
            );
            state.emit_event(path).await;
            Ok(())
        }
    }
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
    pub fn local(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        let fs = Arc::new(OsFs);
        let (mut tree, scan_states_tx) = LocalWorktree::new(path, languages, fs.clone(), cx);
        let (event_stream, event_stream_handle) = fsevent::EventStream::new(
            &[tree.snapshot.abs_path.as_ref()],
            Duration::from_millis(100),
        );
        let background_snapshot = tree.background_snapshot.clone();
        std::thread::spawn(move || {
            let scanner = BackgroundScanner::new(
                background_snapshot,
                scan_states_tx,
                fs,
                Arc::new(executor::Background::new()),
            );
            scanner.run(event_stream);
        });
        tree._event_stream_handle = Some(event_stream_handle);
        Worktree::Local(tree)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<InMemoryFs>,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        let (tree, scan_states_tx) = LocalWorktree::new(path, languages, fs.clone(), cx);
        let background_snapshot = tree.background_snapshot.clone();
        let fs = fs.clone();
        let background = cx.background().clone();
        cx.background()
            .spawn(async move {
                let events_rx = fs.events().await;
                let scanner =
                    BackgroundScanner::new(background_snapshot, scan_states_tx, fs, background);
                scanner.run_test(events_rx).await;
            })
            .detach();
        Worktree::Local(tree)
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
            .and_then(|buf| buf.upgrade(&cx));

        let buffer = if let Some(buffer) = buffer {
            buffer
        } else {
            return if matches!(self, Worktree::Local(_)) {
                Err(anyhow!(
                    "invalid buffer {} in update buffer message",
                    envelope.buffer_id
                ))
            } else {
                Ok(())
            };
        };

        let ops = envelope
            .operations
            .into_iter()
            .map(|op| op.try_into())
            .collect::<anyhow::Result<Vec<_>>>()?;
        buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
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

    fn poll_snapshot(&mut self, cx: &mut ModelContext<Worktree>) {
        let update_buffers = match self {
            Self::Local(worktree) => {
                worktree.snapshot = worktree.background_snapshot.lock().clone();
                if worktree.is_scanning() {
                    if !worktree.poll_scheduled {
                        cx.spawn(|this, mut cx| async move {
                            smol::Timer::after(Duration::from_millis(100)).await;
                            this.update(&mut cx, |this, cx| {
                                this.as_local_mut().unwrap().poll_scheduled = false;
                                this.poll_snapshot(cx);
                            })
                        })
                        .detach();
                        worktree.poll_scheduled = true;
                    }
                    false
                } else {
                    true
                }
            }
            Self::Remote(worktree) => {
                worktree.snapshot = worktree.snapshot_rx.borrow().clone();
                true
            }
        };

        if update_buffers {
            let mut buffers_to_delete = Vec::new();
            for (buffer_id, buffer) in self.open_buffers() {
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
                                            refresh_buffer(abs_path, cx);
                                        }
                                    }
                                }
                            } else if let Some(entry) = self.entry_for_path(&file.path) {
                                file.entry_id = Some(entry.id);
                                file.mtime = entry.mtime;
                                if let Some(worktree) = self.as_local() {
                                    if buffer_is_clean {
                                        let abs_path = worktree.absolutize(&file.path);
                                        refresh_buffer(abs_path, cx);
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
                self.open_buffers_mut().remove(&buffer_id);
            }
        }

        cx.notify();
    }

    fn open_buffers(&self) -> &HashMap<usize, WeakModelHandle<Buffer>> {
        match self {
            Self::Local(worktree) => &worktree.open_buffers,
            Self::Remote(worktree) => &worktree.open_buffers,
        }
    }

    fn open_buffers_mut(&mut self) -> &mut HashMap<usize, WeakModelHandle<Buffer>> {
        match self {
            Self::Local(worktree) => &mut worktree.open_buffers,
            Self::Remote(worktree) => &mut worktree.open_buffers,
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
    _event_stream_handle: Option<fsevent::Handle>,
    poll_scheduled: bool,
    rpc: Option<(rpc::Client, u64)>,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
    shared_buffers: HashMap<PeerId, HashMap<u64, ModelHandle<Buffer>>>,
    peers: HashMap<PeerId, ReplicaId>,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
}

impl LocalWorktree {
    fn new(
        path: impl Into<Arc<Path>>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Worktree>,
    ) -> (Self, Sender<ScanState>) {
        let abs_path = path.into();
        let (scan_states_tx, scan_states_rx) = smol::channel::unbounded();
        let (mut last_scan_state_tx, last_scan_state_rx) = watch::channel_with(ScanState::Scanning);
        let id = cx.model_id();
        let snapshot = Snapshot {
            id,
            scan_id: 0,
            abs_path,
            root_name: Default::default(),
            root_char_bag: Default::default(),
            ignores: Default::default(),
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
            removed_entry_ids: Default::default(),
            next_entry_id: Default::default(),
        };

        let tree = Self {
            snapshot: snapshot.clone(),
            background_snapshot: Arc::new(Mutex::new(snapshot)),
            snapshots_to_send_tx: None,
            last_scan_state_rx,
            _event_stream_handle: None,
            poll_scheduled: false,
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
                    handle.update(&mut cx, |this, cx| {
                        last_scan_state_tx.blocking_send(scan_state).ok();
                        this.poll_snapshot(cx);
                        let tree = this.as_local_mut().unwrap();
                        if !tree.is_scanning() {
                            if let Some(snapshots_to_send_tx) = tree.snapshots_to_send_tx.clone() {
                                if let Err(err) =
                                    smol::block_on(snapshots_to_send_tx.send(tree.snapshot()))
                                {
                                    log::error!("error submitting snapshot to send {}", err);
                                }
                            }
                        }
                    });
                } else {
                    break;
                }
            }
        })
        .detach();

        (tree, scan_states_tx)
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
    snapshot_rx: watch::Receiver<Snapshot>,
    rpc: rpc::Client,
    updates_tx: postage::mpsc::Sender<proto::UpdateWorktree>,
    replica_id: ReplicaId,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
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

    fn run(mut self, event_stream: fsevent::EventStream) {
        if smol::block_on(self.notify.send(ScanState::Scanning)).is_err() {
            return;
        }

        if let Err(err) = smol::block_on(self.scan_dirs()) {
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

            if !smol::block_on(self.process_events(events)) {
                return false;
            }

            if smol::block_on(self.notify.send(ScanState::Idle)).is_err() {
                return false;
            }

            true
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    async fn run_test(mut self, mut events_rx: broadcast::Receiver<fsevent::Event>) {
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

        while let Some(event) = events_rx.recv().await {
            let mut events = vec![event];
            while let Ok(event) = events_rx.try_recv() {
                events.push(event);
            }

            if self.notify.send(ScanState::Scanning).await.is_err() {
                break;
            }

            if self.process_events(events).await {
                break;
            }

            if self.notify.send(ScanState::Idle).await.is_err() {
                break;
            }
        }
    }

    async fn scan_dirs(&mut self) -> Result<()> {
        let next_entry_id;
        {
            let mut snapshot = self.snapshot.lock();
            snapshot.scan_id += 1;
            next_entry_id = snapshot.next_entry_id.clone();
        }

        let path: Arc<Path> = Arc::from(Path::new(""));
        let abs_path = self.abs_path();

        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let mut root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());
        let root_char_bag = root_name.chars().map(|c| c.to_ascii_lowercase()).collect();
        let entry = self
            .fs
            .entry(root_char_bag, &next_entry_id, path.clone(), &abs_path)
            .await?
            .ok_or_else(|| anyhow!("root entry does not exist"))?;
        let is_dir = entry.is_dir();
        if is_dir {
            root_name.push('/');
        }

        {
            let mut snapshot = self.snapshot.lock();
            snapshot.root_name = root_name;
            snapshot.root_char_bag = root_char_bag;
        }

        self.snapshot.lock().insert_entry(entry);
        if is_dir {
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
                    for _ in 0..self.executor.threads() {
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

        let mut child_entries = self
            .fs
            .child_entries(
                root_char_bag,
                next_entry_id.as_ref(),
                &job.path,
                &job.abs_path,
            )
            .await?;
        while let Some(child_entry) = child_entries.next().await {
            let mut child_entry = match child_entry {
                Ok(child_entry) => child_entry,
                Err(error) => {
                    log::error!("error processing entry {:?}", error);
                    continue;
                }
            };
            let child_name = child_entry.path.file_name().unwrap();
            let child_abs_path = job.abs_path.join(&child_name);
            let child_path = child_entry.path.clone();

            // If we find a .gitignore, add it to the stack of ignores used to determine which paths are ignored
            if child_name == *GITIGNORE {
                let (ignore, err) = Gitignore::new(&child_abs_path);
                if let Some(err) = err {
                    log::error!("error in ignore file {:?} - {:?}", child_entry.path, err);
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

            if child_entry.is_dir() {
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

            match self
                .fs
                .entry(
                    snapshot.root_char_bag,
                    &next_entry_id,
                    path.clone(),
                    &event.path,
                )
                .await
            {
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
                for _ in 0..self.executor.threads() {
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
                for _ in 0..self.executor.threads() {
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
    let entry = fs
        .entry(root_char_bag, &next_entry_id, path, abs_path)
        .await?
        .ok_or_else(|| anyhow!("could not read saved file metadata"))?;
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

        std::fs::write(dir.path().join("tracked-dir/tracked-file2"), "").unwrap();
        std::fs::write(dir.path().join("ignored-dir/ignored-file2"), "").unwrap();
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
                    entries_by_path: Default::default(),
                    entries_by_id: Default::default(),
                    removed_entry_ids: Default::default(),
                    ignores: Default::default(),
                    root_name: Default::default(),
                    root_char_bag: Default::default(),
                    next_entry_id: Default::default(),
                })),
                notify_tx,
                Arc::new(OsFs),
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
                Arc::new(Mutex::new(Snapshot {
                    id: 0,
                    scan_id: 0,
                    abs_path: root_dir.path().into(),
                    entries_by_path: Default::default(),
                    entries_by_id: Default::default(),
                    removed_entry_ids: Default::default(),
                    ignores: Default::default(),
                    root_name: Default::default(),
                    root_char_bag: Default::default(),
                    next_entry_id: Default::default(),
                })),
                notify_tx,
                scanner.fs.clone(),
                scanner.executor.clone(),
            );
            smol::block_on(new_scanner.scan_dirs()).unwrap();
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
                .entries_by_path
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
            for entry in self.entries_by_path.cursor::<(), ()>() {
                paths.push((entry.path().as_ref(), entry.inode(), entry.is_ignored()));
            }
            paths.sort_by(|a, b| a.0.cmp(&b.0));
            paths
        }
    }
}
