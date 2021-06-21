mod char_bag;
mod fuzzy;
mod ignore;

use self::{char_bag::CharBag, ignore::IgnoreStack};
use crate::{
    editor::{History, Rope},
    rpc::{self, proto, ConnectionId},
    sum_tree::{self, Cursor, Edit, SumTree},
    util::Bias,
};
use ::ignore::gitignore::Gitignore;
use anyhow::{anyhow, Context, Result};
pub use fuzzy::{match_paths, PathMatch};
use gpui::{scoped_pool, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::{
    prelude::{Sink, Stream},
    watch,
};
use smol::{channel::Sender, lock::Mutex as AsyncMutex};
use std::{
    cmp,
    collections::HashMap,
    ffi::{OsStr, OsString},
    fmt, fs,
    future::Future,
    hash::Hash,
    io::{self, Read, Write},
    ops::Deref,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering::SeqCst},
        Arc, Weak,
    },
    time::{Duration, UNIX_EPOCH},
};

lazy_static! {
    static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
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
    pub fn local(path: impl Into<Arc<Path>>, cx: &mut ModelContext<Worktree>) -> Self {
        Worktree::Local(LocalWorktree::new(path, cx))
    }

    pub fn remote(
        id: usize,
        worktree: proto::Worktree,
        rpc: rpc::Client,
        connection_id: ConnectionId,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        Worktree::Remote(RemoteWorktree::new(id, worktree, rpc, connection_id, cx))
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

    pub fn snapshot(&self) -> Snapshot {
        match self {
            Worktree::Local(worktree) => worktree.snapshot(),
            Worktree::Remote(worktree) => worktree.snapshot.clone(),
        }
    }

    pub fn save(
        &self,
        path: &Path,
        content: Rope,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        match self {
            Worktree::Local(worktree) => worktree.save(path, content, cx),
            Worktree::Remote(worktree) => todo!(),
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
    handles: Arc<Mutex<HashMap<Arc<Path>, Weak<Mutex<FileHandleState>>>>>,
    next_handle_id: AtomicU64,
    scan_state: (watch::Sender<ScanState>, watch::Receiver<ScanState>),
    _event_stream_handle: fsevent::Handle,
    polling_snapshot: bool,
    rpc: Option<rpc::Client>,
}

#[derive(Clone)]
pub struct FileHandle {
    worktree: ModelHandle<Worktree>,
    state: Arc<Mutex<FileHandleState>>,
}

#[derive(Clone)]
struct FileHandleState {
    path: Arc<Path>,
    is_deleted: bool,
    mtime: Duration,
    worktree_id: usize,
    id: u64,
    rpc: Option<(ConnectionId, rpc::Client)>,
}

impl Drop for FileHandleState {
    fn drop(&mut self) {
        if let Some((connection_id, rpc)) = self.rpc.take() {
            let id = self.id;
            let worktree_id = self.worktree_id as u64;
            smol::spawn(async move {
                if let Err(error) = rpc
                    .send(connection_id, proto::CloseFile { worktree_id, id })
                    .await
                {
                    log::warn!("error closing file {}: {}", id, error);
                }
            })
            .detach();
        }
    }
}

impl LocalWorktree {
    fn new(path: impl Into<Arc<Path>>, cx: &mut ModelContext<Worktree>) -> Self {
        let abs_path = path.into();
        let (scan_state_tx, scan_state_rx) = smol::channel::unbounded();
        let id = cx.model_id();
        let snapshot = Snapshot {
            id,
            scan_id: 0,
            abs_path,
            root_name: Default::default(),
            ignores: Default::default(),
            entries: Default::default(),
        };
        let (event_stream, event_stream_handle) =
            fsevent::EventStream::new(&[snapshot.abs_path.as_ref()], Duration::from_millis(100));

        let background_snapshot = Arc::new(Mutex::new(snapshot.clone()));
        let handles = Arc::new(Mutex::new(Default::default()));

        let tree = Self {
            snapshot,
            background_snapshot: background_snapshot.clone(),
            handles: handles.clone(),
            next_handle_id: Default::default(),
            scan_state: watch::channel_with(ScanState::Scanning),
            _event_stream_handle: event_stream_handle,
            polling_snapshot: false,
            rpc: None,
        };

        std::thread::spawn(move || {
            let scanner = BackgroundScanner::new(background_snapshot, handles, scan_state_tx, id);
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
        let _ = self.scan_state.0.blocking_send(scan_state);
        if !self.polling_snapshot {
            self.poll_snapshot(cx);
        }
    }

    fn poll_snapshot(&mut self, cx: &mut ModelContext<Worktree>) {
        let poll_again = self.is_scanning();
        if poll_again {
            self.polling_snapshot = true;
        }

        // let prev_snapshot = self.snapshot.clone();
        let background_snapshot = self.background_snapshot.clone();
        let next_snapshot = cx.background_executor().spawn(async move {
            let next_snapshot = background_snapshot.lock().clone();
            // TODO: Diff with next and prev snapshots
            next_snapshot
        });

        cx.spawn(|this, mut cx| async move {
            let next_snapshot = next_snapshot.await;
            this.update(&mut cx, |this, cx| {
                let worktree = this.as_local_mut().unwrap();
                worktree.snapshot = next_snapshot;
                cx.notify();
            });

            if poll_again {
                smol::Timer::after(Duration::from_millis(100)).await;
                this.update(&mut cx, |this, cx| {
                    let worktree = this.as_local_mut().unwrap();
                    worktree.polling_snapshot = false;
                    worktree.poll_snapshot(cx);
                })
            }
        })
        .detach();
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

    pub fn save(&self, path: &Path, content: Rope, cx: &AppContext) -> Task<Result<()>> {
        let handles = self.handles.clone();
        let path = path.to_path_buf();
        let abs_path = self.absolutize(&path);
        cx.background_executor().spawn(async move {
            let buffer_size = content.summary().bytes.min(10 * 1024);
            let file = fs::File::create(&abs_path)?;
            let mut writer = io::BufWriter::with_capacity(buffer_size, &file);
            for chunk in content.chunks() {
                writer.write(chunk.as_bytes())?;
            }
            writer.flush()?;

            if let Some(handle) = handles.lock().get(&*path).and_then(Weak::upgrade) {
                let mut handle = handle.lock();
                handle.mtime = file.metadata()?.modified()?.duration_since(UNIX_EPOCH)?;
                handle.is_deleted = false;
            }

            Ok(())
        })
    }

    pub fn share(
        &mut self,
        client: rpc::Client,
        connection_id: ConnectionId,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<anyhow::Result<(u64, String)>> {
        self.rpc = Some(client.clone());
        let root_name = self.root_name.clone();
        let snapshot = self.snapshot();
        let handle = cx.handle();
        cx.spawn(|_this, cx| async move {
            let entries = cx
                .background_executor()
                .spawn(async move {
                    snapshot
                        .entries
                        .cursor::<(), ()>()
                        .map(|entry| proto::Entry {
                            is_dir: entry.is_dir(),
                            path: entry.path.to_string_lossy().to_string(),
                            inode: entry.inode,
                            is_symlink: entry.is_symlink,
                            is_ignored: entry.is_ignored,
                        })
                        .collect()
                })
                .await;

            let share_response = client
                .request(
                    connection_id,
                    proto::ShareWorktree {
                        worktree: Some(proto::Worktree { root_name, entries }),
                    },
                )
                .await?;

            client
                .state
                .lock()
                .await
                .shared_worktrees
                .insert(share_response.worktree_id, handle);

            log::info!("sharing worktree {:?}", share_response);
            Ok((share_response.worktree_id, share_response.access_token))
        })
    }
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
    remote_id: usize,
    snapshot: Snapshot,
    handles: Arc<Mutex<HashMap<Arc<Path>, Arc<AsyncMutex<Weak<Mutex<FileHandleState>>>>>>>,
    rpc: rpc::Client,
    connection_id: ConnectionId,
}

impl RemoteWorktree {
    fn new(
        remote_id: usize,
        worktree: proto::Worktree,
        rpc: rpc::Client,
        connection_id: ConnectionId,
        cx: &mut ModelContext<Worktree>,
    ) -> Self {
        let root_char_bag: CharBag = worktree
            .root_name
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let mut entries = SumTree::new();
        entries.extend(
            worktree.entries.into_iter().map(|entry| {
                let kind = if entry.is_dir {
                    EntryKind::Dir
                } else {
                    let mut char_bag = root_char_bag.clone();
                    char_bag.extend(entry.path.chars().map(|c| c.to_ascii_lowercase()));
                    EntryKind::File(char_bag)
                };
                Entry {
                    kind,
                    path: Path::new(&entry.path).into(),
                    inode: entry.inode,
                    is_symlink: entry.is_symlink,
                    is_ignored: entry.is_ignored,
                }
            }),
            &(),
        );
        let snapshot = Snapshot {
            id: cx.model_id(),
            scan_id: 0,
            abs_path: Path::new("").into(),
            root_name: worktree.root_name,
            ignores: Default::default(),
            entries,
        };
        Self {
            remote_id,
            snapshot,
            handles: Default::default(),
            rpc,
            connection_id,
        }
    }
}

#[derive(Clone)]
pub struct Snapshot {
    id: usize,
    scan_id: usize,
    abs_path: Arc<Path>,
    root_name: String,
    ignores: HashMap<Arc<Path>, (Arc<Gitignore>, usize)>,
    entries: SumTree<Entry>,
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
        self.entries
            .cursor::<(), ()>()
            .skip(1)
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

    fn path_is_pending(&self, path: impl AsRef<Path>) -> bool {
        if self.entries.is_empty() {
            return true;
        }
        let path = path.as_ref();
        let mut cursor = self.entries.cursor::<_, ()>();
        if cursor.seek(&PathSearch::Exact(path), Bias::Left, &()) {
            let entry = cursor.item().unwrap();
            if entry.path.as_ref() == path {
                return matches!(entry.kind, EntryKind::PendingDir);
            }
        }
        if let Some(entry) = cursor.prev_item() {
            matches!(entry.kind, EntryKind::PendingDir) && path.starts_with(entry.path.as_ref())
        } else {
            false
        }
    }

    fn entry_for_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let mut cursor = self.entries.cursor::<_, ()>();
        if cursor.seek(&PathSearch::Exact(path.as_ref()), Bias::Left, &()) {
            cursor.item()
        } else {
            None
        }
    }

    pub fn inode_for_path(&self, path: impl AsRef<Path>) -> Option<u64> {
        self.entry_for_path(path.as_ref()).map(|e| e.inode())
    }

    fn insert_entry(&mut self, entry: Entry) {
        if !entry.is_dir() && entry.path().file_name() == Some(&GITIGNORE) {
            let (ignore, err) = Gitignore::new(self.abs_path.join(entry.path()));
            if let Some(err) = err {
                log::error!("error in ignore file {:?} - {:?}", entry.path(), err);
            }

            let ignore_dir_path = entry.path().parent().unwrap();
            self.ignores
                .insert(ignore_dir_path.into(), (Arc::new(ignore), self.scan_id));
        }
        self.entries.insert(entry, &());
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

        for entry in entries {
            edits.push(Edit::Insert(entry));
        }
        self.entries.edit(edits, &());
    }

    fn remove_path(&mut self, path: &Path) {
        let new_entries = {
            let mut cursor = self.entries.cursor::<_, ()>();
            let mut new_entries = cursor.slice(&PathSearch::Exact(path), Bias::Left, &());
            cursor.seek_forward(&PathSearch::Successor(path), Bias::Left, &());
            new_entries.push_tree(cursor.suffix(&()), &());
            new_entries
        };
        self.entries = new_entries;

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

impl FileHandle {
    pub fn id(&self) -> u64 {
        self.state.lock().id
    }

    /// Returns this file's path relative to the root of its worktree.
    pub fn path(&self) -> Arc<Path> {
        self.state.lock().path.clone()
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    pub fn file_name<'a>(&'a self, cx: &'a AppContext) -> Option<OsString> {
        self.state
            .lock()
            .path
            .file_name()
            .or_else(|| Some(OsStr::new(self.worktree.read(cx).root_name())))
            .map(Into::into)
    }

    pub fn is_deleted(&self) -> bool {
        self.state.lock().is_deleted
    }

    pub fn mtime(&self) -> Duration {
        self.state.lock().mtime
    }

    pub fn exists(&self) -> bool {
        !self.is_deleted()
    }

    pub fn load_history(&self, cx: &AppContext) -> Task<Result<History>> {
        match self.worktree.read(cx) {
            Worktree::Local(worktree) => {
                let path = self.state.lock().path.to_path_buf();
                let abs_path = worktree.absolutize(&path);
                cx.background_executor().spawn(async move {
                    let mut file = fs::File::open(&abs_path)?;
                    let mut base_text = String::new();
                    file.read_to_string(&mut base_text)?;
                    Ok(History::new(Arc::from(base_text)))
                })
            }
            Worktree::Remote(worktree) => {
                let state = self.state.lock();
                let id = state.id;
                let worktree_id = worktree.remote_id as u64;
                let (connection_id, rpc) = state.rpc.clone().unwrap();
                cx.background_executor().spawn(async move {
                    let response = rpc
                        .request(connection_id, proto::OpenBuffer { worktree_id, id })
                        .await?;
                    let buffer = response
                        .buffer
                        .ok_or_else(|| anyhow!("buffer must be present"))?;
                    let history = History::new(buffer.content.into());
                    Ok(history)
                })
            }
        }
    }

    pub fn save(&self, content: Rope, cx: &AppContext) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx);
        worktree.save(&self.path(), content, cx)
    }

    pub fn worktree_id(&self) -> usize {
        self.worktree.id()
    }

    pub fn entry_id(&self) -> (usize, Arc<Path>) {
        (self.worktree.id(), self.path())
    }

    pub fn observe_from_model<T: Entity>(
        &self,
        cx: &mut ModelContext<T>,
        mut callback: impl FnMut(&mut T, FileHandle, &mut ModelContext<T>) + 'static,
    ) {
        let mut prev_state = self.state.lock().clone();
        let cur_state = Arc::downgrade(&self.state);
        cx.observe(&self.worktree, move |observer, worktree, cx| {
            if let Some(cur_state) = cur_state.upgrade() {
                let cur_state_unlocked = cur_state.lock();
                if cur_state_unlocked.mtime != prev_state.mtime
                    || cur_state_unlocked.path != prev_state.path
                {
                    prev_state = cur_state_unlocked.clone();
                    drop(cur_state_unlocked);
                    callback(
                        observer,
                        FileHandle {
                            worktree,
                            state: cur_state,
                        },
                        cx,
                    );
                }
            }
        });
    }
}

impl PartialEq for FileHandle {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.state, &other.state) {
            true
        } else {
            let self_state = self.state.lock();
            let other_state = other.state.lock();
            self_state.worktree_id == other_state.worktree_id && self_state.id == other_state.id
        }
    }
}

impl Eq for FileHandle {}

impl Hash for FileHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.lock().id.hash(state);
        self.worktree.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    kind: EntryKind,
    path: Arc<Path>,
    inode: u64,
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
            _ => todo!("not sure we need the other two cases"),
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
    handles: Arc<Mutex<HashMap<Arc<Path>, Weak<Mutex<FileHandleState>>>>>,
    thread_pool: scoped_pool::Pool,
    root_char_bag: CharBag,
}

impl BackgroundScanner {
    fn new(
        snapshot: Arc<Mutex<Snapshot>>,
        handles: Arc<Mutex<HashMap<Arc<Path>, Weak<Mutex<FileHandleState>>>>>,
        notify: Sender<ScanState>,
        worktree_id: usize,
    ) -> Self {
        Self {
            root_char_bag: Default::default(),
            snapshot,
            notify,
            handles,
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

        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let mut root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());
        if is_dir {
            root_name.push('/');
        }
        self.root_char_bag = root_name.chars().map(|c| c.to_ascii_lowercase()).collect();
        self.snapshot.lock().root_name = root_name;

        if is_dir {
            self.snapshot.lock().insert_entry(Entry {
                kind: EntryKind::PendingDir,
                path: path.clone(),
                inode,
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
                            if let Err(err) = self.scan_dir(&job) {
                                log::error!("error scanning {:?}: {}", job.abs_path, err);
                            }
                        }
                    });
                }
            });
        } else {
            self.snapshot.lock().insert_entry(Entry {
                kind: EntryKind::File(self.char_bag(&path)),
                path,
                inode,
                is_symlink,
                is_ignored: false,
            });
        }

        self.mark_deleted_file_handles();
        Ok(())
    }

    fn scan_dir(&self, job: &ScanJob) -> io::Result<()> {
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
                    kind: EntryKind::PendingDir,
                    path: child_path.clone(),
                    inode: child_inode,
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
                    kind: EntryKind::File(self.char_bag(&child_path)),
                    path: child_path,
                    inode: child_inode,
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

        let mut renamed_paths: HashMap<u64, PathBuf> = HashMap::new();
        let mut handles = self.handles.lock();
        let mut updated_handles = HashMap::new();
        for event in &events {
            let path = if let Ok(path) = event.path.strip_prefix(&root_abs_path) {
                path
            } else {
                continue;
            };

            let metadata = fs::metadata(&event.path);
            if event.flags.contains(fsevent::StreamFlags::ITEM_RENAMED) {
                if let Some(inode) = snapshot.inode_for_path(path) {
                    renamed_paths.insert(inode, path.to_path_buf());
                } else if let Ok(metadata) = &metadata {
                    let new_path = path;
                    if let Some(old_path) = renamed_paths.get(&metadata.ino()) {
                        handles.retain(|handle_path, handle_state| {
                            if let Ok(path_suffix) = handle_path.strip_prefix(&old_path) {
                                let new_handle_path: Arc<Path> =
                                    if path_suffix.file_name().is_some() {
                                        new_path.join(path_suffix)
                                    } else {
                                        new_path.to_path_buf()
                                    }
                                    .into();
                                if let Some(handle_state) = Weak::upgrade(&handle_state) {
                                    let mut state = handle_state.lock();
                                    state.path = new_handle_path.clone();
                                    updated_handles
                                        .insert(new_handle_path, Arc::downgrade(&handle_state));
                                }
                                false
                            } else {
                                true
                            }
                        });
                        handles.extend(updated_handles.drain());
                    }
                }
            }

            for state in handles.values_mut() {
                if let Some(state) = Weak::upgrade(&state) {
                    let mut state = state.lock();
                    if state.path.as_ref() == path {
                        if let Ok(metadata) = &metadata {
                            state.mtime = metadata
                                .modified()
                                .unwrap()
                                .duration_since(UNIX_EPOCH)
                                .unwrap();
                        }
                    } else if state.path.starts_with(path) {
                        if let Ok(metadata) = fs::metadata(state.path.as_ref()) {
                            state.mtime = metadata
                                .modified()
                                .unwrap()
                                .duration_since(UNIX_EPOCH)
                                .unwrap();
                        }
                    }
                }
            }
        }
        drop(handles);

        events.sort_unstable_by(|a, b| a.path.cmp(&b.path));
        let mut abs_paths = events.into_iter().map(|e| e.path).peekable();
        let (scan_queue_tx, scan_queue_rx) = crossbeam_channel::unbounded();

        while let Some(abs_path) = abs_paths.next() {
            let path = match abs_path.strip_prefix(&root_abs_path) {
                Ok(path) => Arc::from(path.to_path_buf()),
                Err(_) => {
                    log::error!(
                        "unexpected event {:?} for root path {:?}",
                        abs_path,
                        root_abs_path
                    );
                    continue;
                }
            };

            while abs_paths.peek().map_or(false, |p| p.starts_with(&abs_path)) {
                abs_paths.next();
            }

            snapshot.remove_path(&path);

            match self.fs_entry_for_path(path.clone(), &abs_path) {
                Ok(Some(mut fs_entry)) => {
                    let is_dir = fs_entry.is_dir();
                    let ignore_stack = snapshot.ignore_stack_for_path(&path, is_dir);
                    fs_entry.is_ignored = ignore_stack.is_all();
                    snapshot.insert_entry(fs_entry);
                    if is_dir {
                        scan_queue_tx
                            .send(ScanJob {
                                abs_path,
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
                        if let Err(err) = self.scan_dir(&job) {
                            log::error!("error scanning {:?}: {}", job.abs_path, err);
                        }
                    }
                });
            }
        });

        self.update_ignore_statuses();
        self.mark_deleted_file_handles();
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

    fn mark_deleted_file_handles(&self) {
        let mut handles = self.handles.lock();
        let snapshot = self.snapshot.lock();
        handles.retain(|path, handle_state| {
            if let Some(handle_state) = Weak::upgrade(&handle_state) {
                let mut handle_state = handle_state.lock();
                handle_state.is_deleted = snapshot.entry_for_path(&path).is_none();
                true
            } else {
                false
            }
        });
    }

    fn fs_entry_for_path(&self, path: Arc<Path>, abs_path: &Path) -> Result<Option<Entry>> {
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
        let is_symlink = fs::symlink_metadata(&abs_path)
            .context("failed to read symlink metadata")?
            .file_type()
            .is_symlink();

        let entry = Entry {
            kind: if metadata.file_type().is_dir() {
                EntryKind::PendingDir
            } else {
                EntryKind::File(self.char_bag(&path))
            },
            path,
            inode,
            is_symlink,
            is_ignored: false,
        };

        Ok(Some(entry))
    }

    fn char_bag(&self, path: &Path) -> CharBag {
        let mut result = self.root_char_bag;
        result.extend(
            path.to_string_lossy()
                .chars()
                .map(|c| c.to_ascii_lowercase()),
        );
        result
    }
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
    fn file(&self, path: impl AsRef<Path>, cx: &mut MutableAppContext) -> Task<Result<FileHandle>>;

    #[cfg(test)]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;
}

impl WorktreeHandle for ModelHandle<Worktree> {
    fn file(&self, path: impl AsRef<Path>, cx: &mut MutableAppContext) -> Task<Result<FileHandle>> {
        let path = Arc::from(path.as_ref());
        let handle = self.clone();
        let tree = self.read(cx);
        match tree {
            Worktree::Local(tree) => {
                let worktree_id = handle.id();
                let abs_path = tree.absolutize(&path);
                cx.spawn(|cx| async move {
                    let mtime = cx
                        .background_executor()
                        .spawn(async move { fs::metadata(&abs_path) })
                        .await?
                        .modified()?
                        .duration_since(UNIX_EPOCH)?;
                    let state = handle.read_with(&cx, |tree, _| {
                        let mut handles = tree.as_local().unwrap().handles.lock();
                        handles
                            .get(&path)
                            .and_then(Weak::upgrade)
                            .unwrap_or_else(|| {
                                let id =
                                    tree.as_local().unwrap().next_handle_id.fetch_add(1, SeqCst);
                                let handle_state = if let Some(entry) = tree.entry_for_path(&path) {
                                    FileHandleState {
                                        path: entry.path().clone(),
                                        is_deleted: false,
                                        mtime,
                                        worktree_id,
                                        id,
                                        rpc: None,
                                    }
                                } else {
                                    FileHandleState {
                                        path: path.clone(),
                                        is_deleted: !tree.path_is_pending(&path),
                                        mtime,
                                        worktree_id,
                                        id,
                                        rpc: None,
                                    }
                                };

                                let state = Arc::new(Mutex::new(handle_state.clone()));
                                handles.insert(path, Arc::downgrade(&state));
                                state
                            })
                    });
                    Ok(FileHandle {
                        worktree: handle.clone(),
                        state,
                    })
                })
            }
            Worktree::Remote(tree) => {
                let remote_worktree_id = tree.remote_id;
                let connection_id = tree.connection_id;
                let rpc = tree.rpc.clone();
                let handles = tree.handles.clone();
                cx.spawn(|cx| async move {
                    let state = handles
                        .lock()
                        .entry(path.clone())
                        .or_insert_with(|| Arc::new(AsyncMutex::new(Weak::new())))
                        .clone();

                    let mut state = state.lock().await;
                    if let Some(state) = Weak::upgrade(&state) {
                        Ok(FileHandle {
                            worktree: handle,
                            state,
                        })
                    } else {
                        let response = rpc
                            .request(
                                connection_id,
                                proto::OpenFile {
                                    worktree_id: remote_worktree_id as u64,
                                    path: path.to_string_lossy().to_string(),
                                },
                            )
                            .await?;
                        let is_deleted = handle.read_with(&cx, |tree, _| {
                            tree.entry_for_path(&path).is_none() && !tree.path_is_pending(&path)
                        });
                        let new_state = Arc::new(Mutex::new(FileHandleState {
                            path,
                            is_deleted,
                            mtime: Duration::from_secs(response.mtime),
                            worktree_id: remote_worktree_id,
                            id: response.id,
                            rpc: Some((connection_id, rpc)),
                        }));
                        *state = Arc::downgrade(&new_state);
                        Ok(FileHandle {
                            worktree: handle,
                            state: new_state,
                        })
                    }
                })
            }
        }
    }

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
    All(Cursor<'a, Entry, FileCount, FileCount>),
    Visible(Cursor<'a, Entry, VisibleFileCount, VisibleFileCount>),
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
                let ix = *cursor.start();
                cursor.seek_forward(&FileCount(ix.0 + 1), Bias::Right, &());
            }
            Self::Visible(cursor) => {
                let ix = *cursor.start();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::Buffer;
    use crate::test::*;
    use anyhow::Result;
    use rand::prelude::*;
    use serde_json::json;
    use std::env;
    use std::fmt::Write;
    use std::os::unix;
    use std::time::SystemTime;

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

        let tree = cx.add_model(|cx| Worktree::local(root_link_path, cx));

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
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));

        let tree = cx.add_model(|cx| Worktree::local(dir.path(), cx));
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| assert_eq!(tree.read(cx).file_count(), 1));

        let buffer = cx.add_model(|cx| Buffer::new(1, "a line of text.\n".repeat(10 * 1024), cx));

        let path = tree.update(&mut cx, |tree, cx| {
            let path = tree.files(0).next().unwrap().path().clone();
            assert_eq!(path.file_name().unwrap(), "file1");
            smol::block_on(tree.save(&path, buffer.read(cx).snapshot().text(), cx.as_ref()))
                .unwrap();
            path
        });

        let file = cx.update(|cx| tree.file(&path, cx)).await.unwrap();
        let history = cx.read(|cx| file.load_history(cx)).await.unwrap();
        cx.read(|cx| {
            assert_eq!(history.base_text.as_ref(), buffer.read(cx).text());
        });
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));

        let tree = cx.add_model(|cx| Worktree::local(dir.path().join("file1"), cx));
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| assert_eq!(tree.read(cx).file_count(), 1));

        let buffer = cx.add_model(|cx| Buffer::new(1, "a line of text.\n".repeat(10 * 1024), cx));

        let file = cx.update(|cx| tree.file("", cx)).await.unwrap();
        cx.update(|cx| {
            assert_eq!(file.path().file_name(), None);
            smol::block_on(file.save(buffer.read(cx).snapshot().text(), cx.as_ref())).unwrap();
        });

        let history = cx.read(|cx| file.load_history(cx)).await.unwrap();
        cx.read(|cx| assert_eq!(history.base_text.as_ref(), buffer.read(cx).text()));
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

        let tree = cx.add_model(|cx| Worktree::local(dir.path(), cx));
        let file2 = cx.update(|cx| tree.file("a/file2", cx)).await.unwrap();
        let file3 = cx.update(|cx| tree.file("a/file3", cx)).await.unwrap();
        let file4 = cx.update(|cx| tree.file("b/c/file4", cx)).await.unwrap();
        let file5 = cx.update(|cx| tree.file("b/c/file5", cx)).await.unwrap();
        let non_existent_file = cx.update(|cx| tree.file("a/file_x", cx)).await.unwrap();

        // After scanning, the worktree knows which files exist and which don't.
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        assert!(!file2.is_deleted());
        assert!(!file3.is_deleted());
        assert!(!file4.is_deleted());
        assert!(!file5.is_deleted());
        assert!(non_existent_file.is_deleted());

        tree.flush_fs_events(&cx).await;
        std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
        std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
        std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
        std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
        tree.flush_fs_events(&cx).await;

        cx.read(|cx| {
            assert_eq!(
                tree.read(cx)
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

            assert_eq!(file2.path().to_str().unwrap(), "a/file2.new");
            assert_eq!(file4.path().as_ref(), Path::new("d/file4"));
            assert_eq!(file5.path().as_ref(), Path::new("d/file5"));
            assert!(!file2.is_deleted());
            assert!(!file4.is_deleted());
            assert!(file5.is_deleted());

            // Right now, this rename isn't detected because the target path
            // no longer exists on the file system by the time we process the
            // rename event.
            assert_eq!(file3.path().as_ref(), Path::new("a/file3"));
            assert!(file3.is_deleted());
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

        let tree = cx.add_model(|cx| Worktree::local(dir.path(), cx));
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
    fn test_path_is_pending() {
        let mut snapshot = Snapshot {
            id: 0,
            scan_id: 0,
            abs_path: Path::new("").into(),
            entries: Default::default(),
            ignores: Default::default(),
            root_name: Default::default(),
        };

        snapshot.entries.edit(
            vec![
                Edit::Insert(Entry {
                    path: Path::new("b").into(),
                    kind: EntryKind::Dir,
                    inode: 0,
                    is_ignored: false,
                    is_symlink: false,
                }),
                Edit::Insert(Entry {
                    path: Path::new("b/a").into(),
                    kind: EntryKind::Dir,
                    inode: 0,
                    is_ignored: false,
                    is_symlink: false,
                }),
                Edit::Insert(Entry {
                    path: Path::new("b/c").into(),
                    kind: EntryKind::PendingDir,
                    inode: 0,
                    is_ignored: false,
                    is_symlink: false,
                }),
                Edit::Insert(Entry {
                    path: Path::new("b/e").into(),
                    kind: EntryKind::Dir,
                    inode: 0,
                    is_ignored: false,
                    is_symlink: false,
                }),
            ],
            &(),
        );

        assert!(!snapshot.path_is_pending("b/a"));
        assert!(!snapshot.path_is_pending("b/b"));
        assert!(snapshot.path_is_pending("b/c"));
        assert!(snapshot.path_is_pending("b/c/x"));
        assert!(!snapshot.path_is_pending("b/d"));
        assert!(!snapshot.path_is_pending("b/e"));
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
                    ignores: Default::default(),
                    root_name: Default::default(),
                })),
                Arc::new(Mutex::new(Default::default())),
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
                    ignores: Default::default(),
                    root_name: Default::default(),
                })),
                Arc::new(Mutex::new(Default::default())),
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
