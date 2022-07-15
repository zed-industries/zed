use anyhow::{anyhow, Result};
use fsevent::EventStream;
use futures::{future::BoxFuture, Stream, StreamExt};
use language::LineEnding;
use smol::io::{AsyncReadExt, AsyncWriteExt};
use std::{
    io,
    os::unix::fs::MetadataExt,
    path::{Component, Path, PathBuf},
    pin::Pin,
    time::{Duration, SystemTime},
};
use text::Rope;

#[cfg(any(test, feature = "test-support"))]
use collections::{btree_map, BTreeMap};
#[cfg(any(test, feature = "test-support"))]
use futures::lock::Mutex;
#[cfg(any(test, feature = "test-support"))]
use std::sync::{Arc, Weak};

#[async_trait::async_trait]
pub trait Fs: Send + Sync {
    async fn create_dir(&self, path: &Path) -> Result<()>;
    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()>;
    async fn copy_file(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()>;
    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()>;
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>>;
    async fn load(&self, path: &Path) -> Result<String>;
    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()>;
    async fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
    async fn is_file(&self, path: &Path) -> bool;
    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>>;
    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>>;
    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> Pin<Box<dyn Send + Stream<Item = Vec<fsevent::Event>>>>;
    fn is_fake(&self) -> bool;
    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &FakeFs;
}

#[derive(Copy, Clone, Default)]
pub struct CreateOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone, Default)]
pub struct CopyOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone, Default)]
pub struct RenameOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone, Default)]
pub struct RemoveOptions {
    pub recursive: bool,
    pub ignore_if_not_exists: bool,
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub inode: u64,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    pub is_dir: bool,
}

pub struct RealFs;

#[async_trait::async_trait]
impl Fs for RealFs {
    async fn create_dir(&self, path: &Path) -> Result<()> {
        Ok(smol::fs::create_dir_all(path).await?)
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()> {
        let mut open_options = smol::fs::OpenOptions::new();
        open_options.write(true).create(true);
        if options.overwrite {
            open_options.truncate(true);
        } else if !options.ignore_if_exists {
            open_options.create_new(true);
        }
        open_options.open(path).await?;
        Ok(())
    }

    async fn copy_file(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()> {
        if !options.overwrite && smol::fs::metadata(target).await.is_ok() {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(anyhow!("{target:?} already exists"));
            }
        }

        smol::fs::copy(source, target).await?;
        Ok(())
    }

    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()> {
        if !options.overwrite && smol::fs::metadata(target).await.is_ok() {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(anyhow!("{target:?} already exists"));
            }
        }

        smol::fs::rename(source, target).await?;
        Ok(())
    }

    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        let result = if options.recursive {
            smol::fs::remove_dir_all(path).await
        } else {
            smol::fs::remove_dir(path).await
        };
        match result {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists => {
                Ok(())
            }
            Err(err) => Err(err)?,
        }
    }

    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        match smol::fs::remove_file(path).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists => {
                Ok(())
            }
            Err(err) => Err(err)?,
        }
    }

    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn load(&self, path: &Path) -> Result<String> {
        let mut file = smol::fs::File::open(path).await?;
        let mut text = String::new();
        file.read_to_string(&mut text).await?;
        Ok(text)
    }

    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()> {
        let buffer_size = text.summary().bytes.min(10 * 1024);
        let file = smol::fs::File::create(path).await?;
        let mut writer = smol::io::BufWriter::with_capacity(buffer_size, file);
        for chunk in chunks(text, line_ending) {
            writer.write_all(chunk.as_bytes()).await?;
        }
        writer.flush().await?;
        Ok(())
    }

    async fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        Ok(smol::fs::canonicalize(path).await?)
    }

    async fn is_file(&self, path: &Path) -> bool {
        smol::fs::metadata(path)
            .await
            .map_or(false, |metadata| metadata.is_file())
    }

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>> {
        let symlink_metadata = match smol::fs::symlink_metadata(path).await {
            Ok(metadata) => metadata,
            Err(err) => {
                return match (err.kind(), err.raw_os_error()) {
                    (io::ErrorKind::NotFound, _) => Ok(None),
                    (io::ErrorKind::Other, Some(libc::ENOTDIR)) => Ok(None),
                    _ => Err(anyhow::Error::new(err)),
                }
            }
        };

        let is_symlink = symlink_metadata.file_type().is_symlink();
        let metadata = if is_symlink {
            smol::fs::metadata(path).await?
        } else {
            symlink_metadata
        };
        Ok(Some(Metadata {
            inode: metadata.ino(),
            mtime: metadata.modified().unwrap(),
            is_symlink,
            is_dir: metadata.file_type().is_dir(),
        }))
    }

    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        let result = smol::fs::read_dir(path).await?.map(|entry| match entry {
            Ok(entry) => Ok(entry.path()),
            Err(error) => Err(anyhow!("failed to read dir entry {:?}", error)),
        });
        Ok(Box::pin(result))
    }

    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> Pin<Box<dyn Send + Stream<Item = Vec<fsevent::Event>>>> {
        let (tx, rx) = smol::channel::unbounded();
        let (stream, handle) = EventStream::new(&[path], latency);
        std::mem::forget(handle);
        std::thread::spawn(move || {
            stream.run(move |events| smol::block_on(tx.send(events)).is_ok());
        });
        Box::pin(rx)
    }

    fn is_fake(&self) -> bool {
        false
    }
    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &FakeFs {
        panic!("called `RealFs::as_fake`")
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeFs {
    // Use an unfair lock to ensure tests are deterministic.
    state: Mutex<FakeFsState>,
    executor: Weak<gpui::executor::Background>,
}

#[cfg(any(test, feature = "test-support"))]
struct FakeFsState {
    root: Arc<Mutex<FakeFsEntry>>,
    next_inode: u64,
    event_txs: Vec<smol::channel::Sender<Vec<fsevent::Event>>>,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug)]
enum FakeFsEntry {
    File {
        inode: u64,
        mtime: SystemTime,
        content: String,
    },
    Dir {
        inode: u64,
        mtime: SystemTime,
        entries: BTreeMap<String, Arc<Mutex<FakeFsEntry>>>,
    },
    Symlink {
        target: PathBuf,
    },
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFsState {
    async fn read_path<'a>(&'a self, target: &Path) -> Result<Arc<Mutex<FakeFsEntry>>> {
        Ok(self
            .try_read_path(target)
            .await
            .ok_or_else(|| anyhow!("path does not exist: {}", target.display()))?
            .0)
    }

    async fn try_read_path<'a>(
        &'a self,
        target: &Path,
    ) -> Option<(Arc<Mutex<FakeFsEntry>>, PathBuf)> {
        let mut path = target.to_path_buf();
        let mut real_path = PathBuf::new();
        let mut entry_stack = Vec::new();
        'outer: loop {
            let mut path_components = path.components().collect::<collections::VecDeque<_>>();
            while let Some(component) = path_components.pop_front() {
                match component {
                    Component::Prefix(_) => panic!("prefix paths aren't supported"),
                    Component::RootDir => {
                        entry_stack.clear();
                        entry_stack.push(self.root.clone());
                        real_path.clear();
                        real_path.push("/");
                    }
                    Component::CurDir => {}
                    Component::ParentDir => {
                        entry_stack.pop()?;
                        real_path.pop();
                    }
                    Component::Normal(name) => {
                        let current_entry = entry_stack.last().cloned()?;
                        let current_entry = current_entry.lock().await;
                        if let FakeFsEntry::Dir { entries, .. } = &*current_entry {
                            let entry = entries.get(name.to_str().unwrap()).cloned()?;
                            let _entry = entry.lock().await;
                            if let FakeFsEntry::Symlink { target, .. } = &*_entry {
                                let mut target = target.clone();
                                target.extend(path_components);
                                path = target;
                                continue 'outer;
                            } else {
                                entry_stack.push(entry.clone());
                                real_path.push(name);
                            }
                        } else {
                            return None;
                        }
                    }
                }
            }
            break;
        }
        entry_stack.pop().map(|entry| (entry, real_path))
    }

    async fn write_path<Fn, T>(&self, path: &Path, callback: Fn) -> Result<T>
    where
        Fn: FnOnce(btree_map::Entry<String, Arc<Mutex<FakeFsEntry>>>) -> Result<T>,
    {
        let path = normalize_path(path);
        let filename = path
            .file_name()
            .ok_or_else(|| anyhow!("cannot overwrite the root"))?;
        let parent_path = path.parent().unwrap();

        let parent = self.read_path(parent_path).await?;
        let mut parent = parent.lock().await;
        let new_entry = parent
            .dir_entries(parent_path)?
            .entry(filename.to_str().unwrap().into());
        callback(new_entry)
    }

    fn emit_event<I, T>(&mut self, paths: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<PathBuf>,
    {
        let events = paths
            .into_iter()
            .map(|path| fsevent::Event {
                event_id: 0,
                flags: fsevent::StreamFlags::empty(),
                path: path.into(),
            })
            .collect::<Vec<_>>();

        self.event_txs.retain(|tx| {
            let _ = tx.try_send(events.clone());
            !tx.is_closed()
        });
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFs {
    pub fn new(executor: Arc<gpui::executor::Background>) -> Arc<Self> {
        Arc::new(Self {
            executor: Arc::downgrade(&executor),
            state: Mutex::new(FakeFsState {
                root: Arc::new(Mutex::new(FakeFsEntry::Dir {
                    inode: 0,
                    mtime: SystemTime::now(),
                    entries: Default::default(),
                })),
                next_inode: 1,
                event_txs: Default::default(),
            }),
        })
    }

    pub async fn insert_file(&self, path: impl AsRef<Path>, content: String) {
        let mut state = self.state.lock().await;
        let path = path.as_ref();
        let inode = state.next_inode;
        state.next_inode += 1;
        let file = Arc::new(Mutex::new(FakeFsEntry::File {
            inode,
            mtime: SystemTime::now(),
            content,
        }));
        state
            .write_path(path, move |entry| {
                match entry {
                    btree_map::Entry::Vacant(e) => {
                        e.insert(file);
                    }
                    btree_map::Entry::Occupied(mut e) => {
                        *e.get_mut() = file;
                    }
                }
                Ok(())
            })
            .await
            .unwrap();
        state.emit_event(&[path]);
    }

    pub async fn insert_symlink(&self, path: impl AsRef<Path>, target: PathBuf) {
        let mut state = self.state.lock().await;
        let path = path.as_ref();
        let file = Arc::new(Mutex::new(FakeFsEntry::Symlink { target }));
        state
            .write_path(path.as_ref(), move |e| match e {
                btree_map::Entry::Vacant(e) => {
                    e.insert(file);
                    Ok(())
                }
                btree_map::Entry::Occupied(mut e) => {
                    *e.get_mut() = file;
                    Ok(())
                }
            })
            .await
            .unwrap();
        state.emit_event(&[path]);
    }

    #[must_use]
    pub fn insert_tree<'a>(
        &'a self,
        path: impl 'a + AsRef<Path> + Send,
        tree: serde_json::Value,
    ) -> futures::future::BoxFuture<'a, ()> {
        use futures::FutureExt as _;
        use serde_json::Value::*;

        async move {
            let path = path.as_ref();

            match tree {
                Object(map) => {
                    self.create_dir(path).await.unwrap();
                    for (name, contents) in map {
                        let mut path = PathBuf::from(path);
                        path.push(name);
                        self.insert_tree(&path, contents).await;
                    }
                }
                Null => {
                    self.create_dir(&path).await.unwrap();
                }
                String(contents) => {
                    self.insert_file(&path, contents).await;
                }
                _ => {
                    panic!("JSON object must contain only objects, strings, or null");
                }
            }
        }
        .boxed()
    }

    pub async fn files(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let mut queue = collections::VecDeque::new();
        queue.push_back((PathBuf::from("/"), self.state.lock().await.root.clone()));
        while let Some((path, entry)) = queue.pop_front() {
            let e = entry.lock().await;
            match &*e {
                FakeFsEntry::File { .. } => result.push(path),
                FakeFsEntry::Dir { entries, .. } => {
                    for (name, entry) in entries {
                        queue.push_back((path.join(name), entry.clone()));
                    }
                }
                FakeFsEntry::Symlink { .. } => {}
            }
        }
        result
    }

    async fn simulate_random_delay(&self) {
        self.executor
            .upgrade()
            .expect("executor has been dropped")
            .simulate_random_delay()
            .await;
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFsEntry {
    fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    fn file_content(&self, path: &Path) -> Result<&String> {
        if let Self::File { content, .. } = self {
            Ok(content)
        } else {
            Err(anyhow!("not a file: {}", path.display()))
        }
    }

    fn set_file_content(&mut self, path: &Path, new_content: String) -> Result<()> {
        if let Self::File { content, mtime, .. } = self {
            *mtime = SystemTime::now();
            *content = new_content;
            Ok(())
        } else {
            Err(anyhow!("not a file: {}", path.display()))
        }
    }

    fn dir_entries(
        &mut self,
        path: &Path,
    ) -> Result<&mut BTreeMap<String, Arc<Mutex<FakeFsEntry>>>> {
        if let Self::Dir { entries, .. } = self {
            Ok(entries)
        } else {
            Err(anyhow!("not a directory: {}", path.display()))
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait::async_trait]
impl Fs for FakeFs {
    async fn create_dir(&self, path: &Path) -> Result<()> {
        self.simulate_random_delay().await;
        let mut state = self.state.lock().await;
        let inode = state.next_inode;
        state.next_inode += 1;
        state
            .write_path(path, |entry| {
                entry.or_insert(Arc::new(Mutex::new(FakeFsEntry::Dir {
                    inode,
                    mtime: SystemTime::now(),
                    entries: Default::default(),
                })));
                Ok(())
            })
            .await?;
        state.emit_event(&[path]);
        Ok(())
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()> {
        self.simulate_random_delay().await;
        let mut state = self.state.lock().await;
        let inode = state.next_inode;
        state.next_inode += 1;
        let file = Arc::new(Mutex::new(FakeFsEntry::File {
            inode,
            mtime: SystemTime::now(),
            content: String::new(),
        }));
        state
            .write_path(path, |entry| {
                match entry {
                    btree_map::Entry::Occupied(mut e) => {
                        if options.overwrite {
                            *e.get_mut() = file;
                        } else if !options.ignore_if_exists {
                            return Err(anyhow!("path already exists: {}", path.display()));
                        }
                    }
                    btree_map::Entry::Vacant(e) => {
                        e.insert(file);
                    }
                }
                Ok(())
            })
            .await?;
        state.emit_event(&[path]);
        Ok(())
    }

    async fn rename(&self, old_path: &Path, new_path: &Path, options: RenameOptions) -> Result<()> {
        let old_path = normalize_path(old_path);
        let new_path = normalize_path(new_path);
        let mut state = self.state.lock().await;
        let moved_entry = state
            .write_path(&old_path, |e| {
                if let btree_map::Entry::Occupied(e) = e {
                    Ok(e.remove())
                } else {
                    Err(anyhow!("path does not exist: {}", &old_path.display()))
                }
            })
            .await?;
        state
            .write_path(&new_path, |e| {
                match e {
                    btree_map::Entry::Occupied(mut e) => {
                        if options.overwrite {
                            *e.get_mut() = moved_entry;
                        } else if !options.ignore_if_exists {
                            return Err(anyhow!("path already exists: {}", new_path.display()));
                        }
                    }
                    btree_map::Entry::Vacant(e) => {
                        e.insert(moved_entry);
                    }
                }
                Ok(())
            })
            .await?;
        state.emit_event(&[old_path, new_path]);
        Ok(())
    }

    async fn copy_file(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()> {
        let source = normalize_path(source);
        let target = normalize_path(target);
        let mut state = self.state.lock().await;
        let source_entry = state.read_path(&source).await?;
        let content = source_entry.lock().await.file_content(&source)?.clone();
        let entry = state
            .write_path(&target, |e| match e {
                btree_map::Entry::Occupied(e) => {
                    if options.overwrite {
                        Ok(Some(e.get().clone()))
                    } else if !options.ignore_if_exists {
                        return Err(anyhow!("{target:?} already exists"));
                    } else {
                        Ok(None)
                    }
                }
                btree_map::Entry::Vacant(e) => Ok(Some(
                    e.insert(Arc::new(Mutex::new(FakeFsEntry::File {
                        inode: 0,
                        mtime: SystemTime::now(),
                        content: String::new(),
                    })))
                    .clone(),
                )),
            })
            .await?;
        if let Some(entry) = entry {
            entry.lock().await.set_file_content(&target, content)?;
        }
        state.emit_event(&[target]);
        Ok(())
    }

    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        let path = normalize_path(path);
        let parent_path = path
            .parent()
            .ok_or_else(|| anyhow!("cannot remove the root"))?;
        let base_name = path.file_name().unwrap();

        let state = self.state.lock().await;
        let parent_entry = state.read_path(parent_path).await?;
        let mut parent_entry = parent_entry.lock().await;
        let entry = parent_entry
            .dir_entries(parent_path)?
            .entry(base_name.to_str().unwrap().into());

        match entry {
            btree_map::Entry::Vacant(_) => {
                if !options.ignore_if_not_exists {
                    return Err(anyhow!("{path:?} does not exist"));
                }
            }
            btree_map::Entry::Occupied(e) => {
                {
                    let mut entry = e.get().lock().await;
                    let children = entry.dir_entries(&path)?;
                    if !options.recursive && !children.is_empty() {
                        return Err(anyhow!("{path:?} is not empty"));
                    }
                }
                e.remove();
            }
        }

        Ok(())
    }

    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        let path = normalize_path(path);
        let parent_path = path
            .parent()
            .ok_or_else(|| anyhow!("cannot remove the root"))?;
        let base_name = path.file_name().unwrap();
        let mut state = self.state.lock().await;
        let parent_entry = state.read_path(parent_path).await?;
        let mut parent_entry = parent_entry.lock().await;
        let entry = parent_entry
            .dir_entries(parent_path)?
            .entry(base_name.to_str().unwrap().into());
        match entry {
            btree_map::Entry::Vacant(_) => {
                if !options.ignore_if_not_exists {
                    return Err(anyhow!("{path:?} does not exist"));
                }
            }
            btree_map::Entry::Occupied(e) => {
                e.get().lock().await.file_content(&path)?;
                e.remove();
            }
        }
        state.emit_event(&[path]);
        Ok(())
    }

    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>> {
        let text = self.load(path).await?;
        Ok(Box::new(io::Cursor::new(text)))
    }

    async fn load(&self, path: &Path) -> Result<String> {
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock().await;
        let entry = state.read_path(&path).await?;
        let entry = entry.lock().await;
        entry.file_content(&path).cloned()
    }

    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let content = chunks(text, line_ending).collect();
        self.insert_file(path, content).await;
        Ok(())
    }

    async fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock().await;
        if let Some((_, real_path)) = state.try_read_path(&path).await {
            Ok(real_path)
        } else {
            Err(anyhow!("path does not exist: {}", path.display()))
        }
    }

    async fn is_file(&self, path: &Path) -> bool {
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock().await;
        if let Some((entry, _)) = state.try_read_path(&path).await {
            entry.lock().await.is_file()
        } else {
            false
        }
    }

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let state = self.state.lock().await;
        if let Some((entry, real_path)) = state.try_read_path(&path).await {
            let entry = entry.lock().await;
            let is_symlink = real_path != path;

            Ok(Some(match &*entry {
                FakeFsEntry::File { inode, mtime, .. } => Metadata {
                    inode: *inode,
                    mtime: *mtime,
                    is_dir: false,
                    is_symlink,
                },
                FakeFsEntry::Dir { inode, mtime, .. } => Metadata {
                    inode: *inode,
                    mtime: *mtime,
                    is_dir: true,
                    is_symlink,
                },
                FakeFsEntry::Symlink { .. } => unreachable!(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let state = self.state.lock().await;
        let entry = state.read_path(&path).await?;
        let mut entry = entry.lock().await;
        let children = entry.dir_entries(&path)?;
        let paths = children
            .keys()
            .map(|file_name| Ok(path.join(file_name)))
            .collect::<Vec<_>>();
        Ok(Box::pin(futures::stream::iter(paths)))
    }

    async fn watch(
        &self,
        path: &Path,
        _: Duration,
    ) -> Pin<Box<dyn Send + Stream<Item = Vec<fsevent::Event>>>> {
        let mut state = self.state.lock().await;
        self.simulate_random_delay().await;
        let (tx, rx) = smol::channel::unbounded();
        state.event_txs.push(tx);
        let path = path.to_path_buf();
        let executor = self.executor.clone();
        Box::pin(futures::StreamExt::filter(rx, move |events| {
            let result = events.iter().any(|event| event.path.starts_with(&path));
            let executor = executor.clone();
            async move {
                if let Some(executor) = executor.clone().upgrade() {
                    executor.simulate_random_delay().await;
                }
                result
            }
        }))
    }

    fn is_fake(&self) -> bool {
        true
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &FakeFs {
        self
    }
}

fn chunks(rope: &Rope, line_ending: LineEnding) -> impl Iterator<Item = &str> {
    rope.chunks().flat_map(move |chunk| {
        let mut newline = false;
        chunk.split('\n').flat_map(move |line| {
            let ending = if newline {
                Some(line_ending.as_str())
            } else {
                None
            };
            newline = true;
            ending.into_iter().chain([line])
        })
    })
}

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

pub fn copy_recursive<'a>(
    fs: &'a dyn Fs,
    source: &'a Path,
    target: &'a Path,
    options: CopyOptions,
) -> BoxFuture<'a, Result<()>> {
    use futures::future::FutureExt;

    async move {
        let metadata = fs
            .metadata(source)
            .await?
            .ok_or_else(|| anyhow!("path does not exist: {}", source.display()))?;
        if metadata.is_dir {
            if !options.overwrite && fs.metadata(target).await.is_ok() {
                if options.ignore_if_exists {
                    return Ok(());
                } else {
                    return Err(anyhow!("{target:?} already exists"));
                }
            }

            let _ = fs
                .remove_dir(
                    target,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await;
            fs.create_dir(target).await?;
            let mut children = fs.read_dir(source).await?;
            while let Some(child_path) = children.next().await {
                if let Ok(child_path) = child_path {
                    if let Some(file_name) = child_path.file_name() {
                        let child_target_path = target.join(file_name);
                        copy_recursive(fs, &child_path, &child_target_path, options).await?;
                    }
                }
            }

            Ok(())
        } else {
            fs.copy_file(source, target, options).await
        }
    }
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use serde_json::json;

    #[gpui::test]
    async fn test_fake_fs(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.background());

        fs.insert_tree(
            "/root",
            json!({
                "dir1": {
                    "a": "A",
                    "b": "B"
                },
                "dir2": {
                    "c": "C",
                    "dir3": {
                        "d": "D"
                    }
                }
            }),
        )
        .await;

        assert_eq!(
            fs.files().await,
            vec![
                PathBuf::from("/root/dir1/a"),
                PathBuf::from("/root/dir1/b"),
                PathBuf::from("/root/dir2/c"),
                PathBuf::from("/root/dir2/dir3/d"),
            ]
        );

        fs.insert_symlink("/root/dir2/link-to-dir3", "./dir3".into())
            .await;

        assert_eq!(
            fs.canonicalize("/root/dir2/link-to-dir3".as_ref())
                .await
                .unwrap(),
            PathBuf::from("/root/dir2/dir3"),
        );
        assert_eq!(
            fs.canonicalize("/root/dir2/link-to-dir3/d".as_ref())
                .await
                .unwrap(),
            PathBuf::from("/root/dir2/dir3/d"),
        );
        assert_eq!(
            fs.load("/root/dir2/link-to-dir3/d".as_ref()).await.unwrap(),
            "D",
        );
    }
}
