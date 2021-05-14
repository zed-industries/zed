mod char_bag;
mod fuzzy;
mod ignore;

use crate::{
    editor::{History, Rope},
    sum_tree::{self, Cursor, Edit, SeekBias, SumTree},
};
use ::ignore::gitignore::Gitignore;
use anyhow::{Context, Result};
pub use fuzzy::{match_paths, PathMatch};
use gpui::{scoped_pool, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::{
    prelude::{Sink, Stream},
    watch,
};
use smol::channel::Sender;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    ffi::{CStr, OsStr, OsString},
    fmt, fs,
    future::Future,
    io::{self, Read, Write},
    ops::Deref,
    os::unix::{ffi::OsStrExt, fs::MetadataExt},
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use self::{char_bag::CharBag, ignore::IgnoreStack};

lazy_static! {
    static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
}

#[derive(Clone, Debug)]
enum ScanState {
    Idle,
    Scanning,
    Err(Arc<io::Error>),
}

pub struct Worktree {
    snapshot: Snapshot,
    background_snapshot: Arc<Mutex<Snapshot>>,
    handles: Arc<Mutex<HashMap<Arc<Path>, Weak<Mutex<FileHandleState>>>>>,
    scan_state: (watch::Sender<ScanState>, watch::Receiver<ScanState>),
    _event_stream_handle: fsevent::Handle,
    poll_scheduled: bool,
}

#[derive(Clone, Debug)]
pub struct FileHandle {
    worktree: ModelHandle<Worktree>,
    state: Arc<Mutex<FileHandleState>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FileHandleState {
    path: Arc<Path>,
    is_deleted: bool,
    mtime: SystemTime,
}

impl Worktree {
    pub fn new(path: impl Into<Arc<Path>>, ctx: &mut ModelContext<Self>) -> Self {
        let abs_path = path.into();
        let (scan_state_tx, scan_state_rx) = smol::channel::unbounded();
        let id = ctx.model_id();
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
            scan_state: watch::channel_with(ScanState::Scanning),
            _event_stream_handle: event_stream_handle,
            poll_scheduled: false,
        };

        std::thread::spawn(move || {
            let scanner = BackgroundScanner::new(background_snapshot, handles, scan_state_tx, id);
            scanner.run(event_stream)
        });

        ctx.spawn(|this, mut ctx| {
            let this = this.downgrade();
            async move {
                while let Ok(scan_state) = scan_state_rx.recv().await {
                    let alive = ctx.update(|ctx| {
                        if let Some(handle) = this.upgrade(&ctx) {
                            handle
                                .update(ctx, |this, ctx| this.observe_scan_state(scan_state, ctx));
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

    fn observe_scan_state(&mut self, scan_state: ScanState, ctx: &mut ModelContext<Self>) {
        let _ = self.scan_state.0.blocking_send(scan_state);
        self.poll_entries(ctx);
    }

    fn poll_entries(&mut self, ctx: &mut ModelContext<Self>) {
        self.snapshot = self.background_snapshot.lock().clone();
        ctx.notify();

        if self.is_scanning() && !self.poll_scheduled {
            ctx.spawn(|this, mut ctx| async move {
                this.update(&mut ctx, |this, ctx| {
                    this.poll_scheduled = false;
                    this.poll_entries(ctx);
                })
            })
            .detach();
            self.poll_scheduled = true;
        }
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

    pub fn load_history(
        &self,
        path: &Path,
        ctx: &AppContext,
    ) -> impl Future<Output = Result<History>> {
        let path = path.to_path_buf();
        let abs_path = self.absolutize(&path);
        ctx.background_executor().spawn(async move {
            let mut file = fs::File::open(&abs_path)?;
            let mut base_text = String::new();
            file.read_to_string(&mut base_text)?;
            Ok(History::new(Arc::from(base_text)))
        })
    }

    pub fn save<'a>(&self, path: &Path, content: Rope, ctx: &AppContext) -> Task<Result<()>> {
        let handles = self.handles.clone();
        let path = path.to_path_buf();
        let abs_path = self.absolutize(&path);
        ctx.background_executor().spawn(async move {
            let buffer_size = content.summary().bytes.min(10 * 1024);
            let file = fs::File::create(&abs_path)?;
            let mut writer = io::BufWriter::with_capacity(buffer_size, &file);
            for chunk in content.chunks() {
                writer.write(chunk.as_bytes())?;
            }
            writer.flush()?;

            if let Some(handle) = handles.lock().get(&*path).and_then(Weak::upgrade) {
                let mut handle = handle.lock();
                handle.mtime = file.metadata()?.modified()?;
                handle.is_deleted = false;
            }

            Ok(())
        })
    }
}

impl Entity for Worktree {
    type Event = ();
}

impl Deref for Worktree {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl fmt::Debug for Worktree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.snapshot.fmt(f)
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

    #[cfg(test)]
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
        if cursor.seek(&PathSearch::Exact(path), SeekBias::Left, &()) {
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
        if cursor.seek(&PathSearch::Exact(path.as_ref()), SeekBias::Left, &()) {
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
            let mut new_entries = cursor.slice(&PathSearch::Exact(path), SeekBias::Left, &());
            cursor.seek_forward(&PathSearch::Successor(path), SeekBias::Left, &());
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
    /// Returns this file's path relative to the root of its worktree.
    pub fn path(&self) -> Arc<Path> {
        self.state.lock().path.clone()
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    pub fn file_name<'a>(&'a self, ctx: &'a AppContext) -> Option<OsString> {
        self.state
            .lock()
            .path
            .file_name()
            .or_else(|| self.worktree.read(ctx).abs_path().file_name())
            .map(Into::into)
    }

    pub fn is_deleted(&self) -> bool {
        self.state.lock().is_deleted
    }

    pub fn mtime(&self) -> SystemTime {
        self.state.lock().mtime
    }

    pub fn exists(&self) -> bool {
        !self.is_deleted()
    }

    pub fn load_history(&self, ctx: &AppContext) -> impl Future<Output = Result<History>> {
        self.worktree.read(ctx).load_history(&self.path(), ctx)
    }

    pub fn save<'a>(&self, content: Rope, ctx: &AppContext) -> Task<Result<()>> {
        let worktree = self.worktree.read(ctx);
        worktree.save(&self.path(), content, ctx)
    }

    pub fn worktree_id(&self) -> usize {
        self.worktree.id()
    }

    pub fn entry_id(&self) -> (usize, Arc<Path>) {
        (self.worktree.id(), self.path())
    }

    pub fn observe_from_model<T: Entity>(
        &self,
        ctx: &mut ModelContext<T>,
        mut callback: impl FnMut(&mut T, FileHandle, &mut ModelContext<T>) + 'static,
    ) {
        let mut prev_state = self.state.lock().clone();
        let cur_state = Arc::downgrade(&self.state);
        ctx.observe(&self.worktree, move |observer, worktree, ctx| {
            if let Some(cur_state) = cur_state.upgrade() {
                let cur_state_unlocked = cur_state.lock();
                if *cur_state_unlocked != prev_state {
                    prev_state = cur_state_unlocked.clone();
                    drop(cur_state_unlocked);
                    callback(
                        observer,
                        FileHandle {
                            worktree,
                            state: cur_state,
                        },
                        ctx,
                    );
                }
            }
        });
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
    fn add_summary(&mut self, summary: &'a EntrySummary) {
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
    fn add_summary(&mut self, summary: &'a EntrySummary) {
        *self = Self::Exact(summary.max_path.as_ref());
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FileCount(usize);

impl<'a> sum_tree::Dimension<'a, EntrySummary> for FileCount {
    fn add_summary(&mut self, summary: &'a EntrySummary) {
        self.0 += summary.file_count;
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VisibleFileCount(usize);

impl<'a> sum_tree::Dimension<'a, EntrySummary> for VisibleFileCount {
    fn add_summary(&mut self, summary: &'a EntrySummary) {
        self.0 += summary.visible_file_count;
    }
}

struct BackgroundScanner {
    snapshot: Arc<Mutex<Snapshot>>,
    notify: Sender<ScanState>,
    handles: Arc<Mutex<HashMap<Arc<Path>, Weak<Mutex<FileHandleState>>>>>,
    other_mount_paths: HashSet<PathBuf>,
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
        let mut scanner = Self {
            root_char_bag: Default::default(),
            snapshot,
            notify,
            handles,
            other_mount_paths: Default::default(),
            thread_pool: scoped_pool::Pool::new(16, format!("worktree-{}-scanner", worktree_id)),
        };
        scanner.update_other_mount_paths();
        scanner
    }

    fn update_other_mount_paths(&mut self) {
        let path = self.snapshot.lock().abs_path.clone();
        self.other_mount_paths.clear();
        self.other_mount_paths.extend(
            mounted_volume_paths()
                .into_iter()
                .filter(|mount_path| !path.starts_with(mount_path)),
        );
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

            // Disallow mount points outside the file system containing the root of this worktree
            if self.other_mount_paths.contains(&child_abs_path) {
                continue;
            }

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
        self.update_other_mount_paths();

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
                            state.mtime = metadata.modified().unwrap();
                        }
                    } else if state.path.starts_with(path) {
                        if let Ok(metadata) = fs::metadata(state.path.as_ref()) {
                            state.mtime = metadata.modified().unwrap();
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
    fn file(&self, path: impl AsRef<Path>, app: &mut MutableAppContext) -> Task<FileHandle>;

    #[cfg(test)]
    fn flush_fs_events<'a>(
        &self,
        app: &'a gpui::TestAppContext,
    ) -> futures_core::future::LocalBoxFuture<'a, ()>;
}

impl WorktreeHandle for ModelHandle<Worktree> {
    fn file(&self, path: impl AsRef<Path>, app: &mut MutableAppContext) -> Task<FileHandle> {
        let path = Arc::from(path.as_ref());
        let handle = self.clone();
        let tree = self.read(app);
        let abs_path = tree.absolutize(&path);
        app.spawn(|ctx| async move {
            let mtime = ctx
                .background_executor()
                .spawn(async move {
                    if let Ok(metadata) = fs::metadata(&abs_path) {
                        metadata.modified().unwrap()
                    } else {
                        UNIX_EPOCH
                    }
                })
                .await;
            let state = handle.read_with(&ctx, |tree, _| {
                let mut handles = tree.handles.lock();
                if let Some(state) = handles.get(&path).and_then(Weak::upgrade) {
                    state
                } else {
                    let handle_state = if let Some(entry) = tree.entry_for_path(&path) {
                        FileHandleState {
                            path: entry.path().clone(),
                            is_deleted: false,
                            mtime,
                        }
                    } else {
                        FileHandleState {
                            path: path.clone(),
                            is_deleted: !tree.path_is_pending(path),
                            mtime,
                        }
                    };

                    let state = Arc::new(Mutex::new(handle_state.clone()));
                    handles.insert(handle_state.path, Arc::downgrade(&state));
                    state
                }
            });
            FileHandle {
                worktree: handle.clone(),
                state,
            }
        })
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
        app: &'a gpui::TestAppContext,
    ) -> futures_core::future::LocalBoxFuture<'a, ()> {
        use smol::future::FutureExt;

        let filename = "fs-event-sentinel";
        let root_path = app.read(|ctx| self.read(ctx).abs_path.clone());
        let tree = self.clone();
        async move {
            fs::write(root_path.join(filename), "").unwrap();
            tree.condition_with_duration(Duration::from_secs(5), &app, |tree, _| {
                tree.entry_for_path(filename).is_some()
            })
            .await;

            fs::remove_file(root_path.join(filename)).unwrap();
            tree.condition_with_duration(Duration::from_secs(5), &app, |tree, _| {
                tree.entry_for_path(filename).is_none()
            })
            .await;

            app.read(|ctx| tree.read(ctx).scan_complete()).await;
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
        cursor.seek(&FileCount(start), SeekBias::Right, &());
        Self::All(cursor)
    }

    fn visible(snapshot: &'a Snapshot, start: usize) -> Self {
        let mut cursor = snapshot.entries.cursor();
        cursor.seek(&VisibleFileCount(start), SeekBias::Right, &());
        Self::Visible(cursor)
    }

    fn next_internal(&mut self) {
        match self {
            Self::All(cursor) => {
                let ix = *cursor.start();
                cursor.seek_forward(&FileCount(ix.0 + 1), SeekBias::Right, &());
            }
            Self::Visible(cursor) => {
                let ix = *cursor.start();
                cursor.seek_forward(&VisibleFileCount(ix.0 + 1), SeekBias::Right, &());
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
        cursor.seek(&PathSearch::Exact(parent_path), SeekBias::Right, &());
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
                    .seek_forward(&PathSearch::Successor(item.path()), SeekBias::Left, &());
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn mounted_volume_paths() -> Vec<PathBuf> {
    unsafe {
        let mut stat_ptr: *mut libc::statfs = std::ptr::null_mut();
        let count = libc::getmntinfo(&mut stat_ptr as *mut _, libc::MNT_WAIT);
        if count >= 0 {
            std::slice::from_raw_parts(stat_ptr, count as usize)
                .iter()
                .map(|stat| {
                    PathBuf::from(OsStr::from_bytes(
                        CStr::from_ptr(&stat.f_mntonname[0]).to_bytes(),
                    ))
                })
                .collect()
        } else {
            panic!("failed to run getmntinfo");
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
    use std::time::{SystemTime, UNIX_EPOCH};

    #[gpui::test]
    async fn test_populate_and_search(mut app: gpui::TestAppContext) {
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

        let tree = app.add_model(|ctx| Worktree::new(root_link_path, ctx));

        app.read(|ctx| tree.read(ctx).scan_complete()).await;
        app.read(|ctx| {
            let tree = tree.read(ctx);
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
                ctx.thread_pool().clone(),
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
    async fn test_save_file(mut app: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));

        let tree = app.add_model(|ctx| Worktree::new(dir.path(), ctx));
        app.read(|ctx| tree.read(ctx).scan_complete()).await;
        app.read(|ctx| assert_eq!(tree.read(ctx).file_count(), 1));

        let buffer =
            app.add_model(|ctx| Buffer::new(1, "a line of text.\n".repeat(10 * 1024), ctx));

        let path = tree.update(&mut app, |tree, ctx| {
            let path = tree.files(0).next().unwrap().path().clone();
            assert_eq!(path.file_name().unwrap(), "file1");
            smol::block_on(tree.save(&path, buffer.read(ctx).snapshot(), ctx.as_ref())).unwrap();
            path
        });

        let history = app
            .read(|ctx| tree.read(ctx).load_history(&path, ctx))
            .await
            .unwrap();
        app.read(|ctx| {
            assert_eq!(history.base_text.as_ref(), buffer.read(ctx).text());
        });
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(mut app: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));

        let tree = app.add_model(|ctx| Worktree::new(dir.path().join("file1"), ctx));
        app.read(|ctx| tree.read(ctx).scan_complete()).await;
        app.read(|ctx| assert_eq!(tree.read(ctx).file_count(), 1));

        let buffer =
            app.add_model(|ctx| Buffer::new(1, "a line of text.\n".repeat(10 * 1024), ctx));

        let file = app.update(|ctx| tree.file("", ctx)).await;
        app.update(|ctx| {
            assert_eq!(file.path().file_name(), None);
            smol::block_on(file.save(buffer.read(ctx).snapshot(), ctx.as_ref())).unwrap();
        });

        let history = app.read(|ctx| file.load_history(ctx)).await.unwrap();
        app.read(|ctx| assert_eq!(history.base_text.as_ref(), buffer.read(ctx).text()));
    }

    #[gpui::test]
    async fn test_rescan_simple(mut app: gpui::TestAppContext) {
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

        let tree = app.add_model(|ctx| Worktree::new(dir.path(), ctx));
        let file2 = app.update(|ctx| tree.file("a/file2", ctx)).await;
        let file3 = app.update(|ctx| tree.file("a/file3", ctx)).await;
        let file4 = app.update(|ctx| tree.file("b/c/file4", ctx)).await;
        let file5 = app.update(|ctx| tree.file("b/c/file5", ctx)).await;
        let non_existent_file = app.update(|ctx| tree.file("a/file_x", ctx)).await;

        // After scanning, the worktree knows which files exist and which don't.
        app.read(|ctx| tree.read(ctx).scan_complete()).await;
        assert!(!file2.is_deleted());
        assert!(!file3.is_deleted());
        assert!(!file4.is_deleted());
        assert!(!file5.is_deleted());
        assert!(non_existent_file.is_deleted());

        tree.flush_fs_events(&app).await;
        std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
        std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
        std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
        std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
        tree.flush_fs_events(&app).await;

        app.read(|ctx| {
            assert_eq!(
                tree.read(ctx)
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
    async fn test_rescan_with_gitignore(mut app: gpui::TestAppContext) {
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

        let tree = app.add_model(|ctx| Worktree::new(dir.path(), ctx));
        app.read(|ctx| tree.read(ctx).scan_complete()).await;
        tree.flush_fs_events(&app).await;
        app.read(|ctx| {
            let tree = tree.read(ctx);
            let tracked = tree.entry_for_path("tracked-dir/tracked-file1").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file1").unwrap();
            assert_eq!(tracked.is_ignored(), false);
            assert_eq!(ignored.is_ignored(), true);
        });

        fs::write(dir.path().join("tracked-dir/tracked-file2"), "").unwrap();
        fs::write(dir.path().join("ignored-dir/ignored-file2"), "").unwrap();
        tree.flush_fs_events(&app).await;
        app.read(|ctx| {
            let tree = tree.read(ctx);
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
    fn test_mounted_volume_paths() {
        let paths = mounted_volume_paths();
        assert!(paths.contains(&"/".into()));
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
