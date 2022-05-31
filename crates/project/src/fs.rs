use anyhow::{anyhow, Result};
use fsevent::EventStream;
use futures::{Stream, StreamExt};
use smol::io::{AsyncReadExt, AsyncWriteExt};
use std::{
    io,
    os::unix::fs::MetadataExt,
    path::{Component, Path, PathBuf},
    pin::Pin,
    time::{Duration, SystemTime},
};
use text::Rope;

#[async_trait::async_trait]
pub trait Fs: Send + Sync {
    async fn create_dir(&self, path: &Path) -> Result<()>;
    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()>;
    async fn copy(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()>;
    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()>;
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>>;
    async fn load(&self, path: &Path) -> Result<String>;
    async fn save(&self, path: &Path, text: &Rope) -> Result<()>;
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

    async fn copy(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()> {
        if !options.overwrite && smol::fs::metadata(target).await.is_ok() {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(anyhow!("{target:?} already exists"));
            }
        }

        let metadata = smol::fs::metadata(source).await?;
        let _ = smol::fs::remove_dir_all(target).await;
        if metadata.is_dir() {
            self.create_dir(target).await?;
            let mut children = smol::fs::read_dir(source).await?;
            while let Some(child) = children.next().await {
                if let Ok(child) = child {
                    let child_source_path = child.path();
                    let child_target_path = target.join(child.file_name());
                    self.copy(&child_source_path, &child_target_path, options)
                        .await?;
                }
            }
        } else {
            smol::fs::copy(source, target).await?;
        }

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
#[derive(Clone, Debug)]
struct FakeFsEntry {
    metadata: Metadata,
    content: Option<String>,
}

#[cfg(any(test, feature = "test-support"))]
struct FakeFsState {
    entries: std::collections::BTreeMap<PathBuf, FakeFsEntry>,
    next_inode: u64,
    event_txs: Vec<smol::channel::Sender<Vec<fsevent::Event>>>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFsState {
    fn validate_path(&self, path: &Path) -> Result<()> {
        if path.is_absolute()
            && path
                .parent()
                .and_then(|path| self.entries.get(path))
                .map_or(false, |e| e.metadata.is_dir)
        {
            Ok(())
        } else {
            Err(anyhow!("invalid path {:?}", path))
        }
    }

    async fn emit_event<I, T>(&mut self, paths: I)
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
pub struct FakeFs {
    // Use an unfair lock to ensure tests are deterministic.
    state: futures::lock::Mutex<FakeFsState>,
    executor: std::sync::Weak<gpui::executor::Background>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFs {
    pub fn new(executor: std::sync::Arc<gpui::executor::Background>) -> std::sync::Arc<Self> {
        let mut entries = std::collections::BTreeMap::new();
        entries.insert(
            Path::new("/").to_path_buf(),
            FakeFsEntry {
                metadata: Metadata {
                    inode: 0,
                    mtime: SystemTime::now(),
                    is_dir: true,
                    is_symlink: false,
                },
                content: None,
            },
        );
        std::sync::Arc::new(Self {
            executor: std::sync::Arc::downgrade(&executor),
            state: futures::lock::Mutex::new(FakeFsState {
                entries,
                next_inode: 1,
                event_txs: Default::default(),
            }),
        })
    }

    pub async fn insert_dir(&self, path: impl AsRef<Path>) {
        let mut state = self.state.lock().await;
        let path = path.as_ref();
        state.validate_path(path).unwrap();

        let inode = state.next_inode;
        state.next_inode += 1;
        state.entries.insert(
            path.to_path_buf(),
            FakeFsEntry {
                metadata: Metadata {
                    inode,
                    mtime: SystemTime::now(),
                    is_dir: true,
                    is_symlink: false,
                },
                content: None,
            },
        );
        state.emit_event(&[path]).await;
    }

    pub async fn insert_file(&self, path: impl AsRef<Path>, content: String) {
        let mut state = self.state.lock().await;
        let path = path.as_ref();
        state.validate_path(path).unwrap();

        let inode = state.next_inode;
        state.next_inode += 1;
        state.entries.insert(
            path.to_path_buf(),
            FakeFsEntry {
                metadata: Metadata {
                    inode,
                    mtime: SystemTime::now(),
                    is_dir: false,
                    is_symlink: false,
                },
                content: Some(content),
            },
        );
        state.emit_event(&[path]).await;
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
                    self.insert_dir(path).await;
                    for (name, contents) in map {
                        let mut path = PathBuf::from(path);
                        path.push(name);
                        self.insert_tree(&path, contents).await;
                    }
                }
                Null => {
                    self.insert_dir(&path).await;
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
        self.state
            .lock()
            .await
            .entries
            .iter()
            .filter_map(|(path, entry)| entry.content.as_ref().map(|_| path.clone()))
            .collect()
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
#[async_trait::async_trait]
impl Fs for FakeFs {
    async fn create_dir(&self, path: &Path) -> Result<()> {
        self.simulate_random_delay().await;
        let state = &mut *self.state.lock().await;
        let path = normalize_path(path);
        let mut ancestor_path = PathBuf::new();
        let mut created_dir_paths = Vec::new();
        for component in path.components() {
            ancestor_path.push(component);
            let entry = state
                .entries
                .entry(ancestor_path.clone())
                .or_insert_with(|| {
                    let inode = state.next_inode;
                    state.next_inode += 1;
                    created_dir_paths.push(ancestor_path.clone());
                    FakeFsEntry {
                        metadata: Metadata {
                            inode,
                            mtime: SystemTime::now(),
                            is_dir: true,
                            is_symlink: false,
                        },
                        content: None,
                    }
                });
            if !entry.metadata.is_dir {
                return Err(anyhow!(
                    "cannot create directory because {:?} is a file",
                    ancestor_path
                ));
            }
        }
        state.emit_event(&created_dir_paths).await;

        Ok(())
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()> {
        self.simulate_random_delay().await;
        let mut state = self.state.lock().await;
        let path = normalize_path(path);
        state.validate_path(&path)?;
        if let Some(entry) = state.entries.get_mut(&path) {
            if entry.metadata.is_dir || entry.metadata.is_symlink {
                return Err(anyhow!(
                    "cannot create file because {:?} is a dir or a symlink",
                    path
                ));
            }

            if options.overwrite {
                entry.metadata.mtime = SystemTime::now();
                entry.content = Some(Default::default());
            } else if !options.ignore_if_exists {
                return Err(anyhow!(
                    "cannot create file because {:?} already exists",
                    &path
                ));
            }
        } else {
            let inode = state.next_inode;
            state.next_inode += 1;
            let entry = FakeFsEntry {
                metadata: Metadata {
                    inode,
                    mtime: SystemTime::now(),
                    is_dir: false,
                    is_symlink: false,
                },
                content: Some(Default::default()),
            };
            state.entries.insert(path.to_path_buf(), entry);
        }
        state.emit_event(&[path]).await;

        Ok(())
    }

    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()> {
        let source = normalize_path(source);
        let target = normalize_path(target);

        let mut state = self.state.lock().await;
        state.validate_path(&source)?;
        state.validate_path(&target)?;

        if !options.overwrite && state.entries.contains_key(&target) {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(anyhow!("{target:?} already exists"));
            }
        }

        let mut removed = Vec::new();
        state.entries.retain(|path, entry| {
            if let Ok(relative_path) = path.strip_prefix(&source) {
                removed.push((relative_path.to_path_buf(), entry.clone()));
                false
            } else {
                true
            }
        });

        for (relative_path, entry) in removed {
            let new_path = normalize_path(&target.join(relative_path));
            state.entries.insert(new_path, entry);
        }

        state.emit_event(&[source, target]).await;
        Ok(())
    }

    async fn copy(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()> {
        let source = normalize_path(source);
        let target = normalize_path(target);

        let mut state = self.state.lock().await;
        state.validate_path(&source)?;
        state.validate_path(&target)?;

        if !options.overwrite && state.entries.contains_key(&target) {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(anyhow!("{target:?} already exists"));
            }
        }

        let mut new_entries = Vec::new();
        for (path, entry) in &state.entries {
            if let Ok(relative_path) = path.strip_prefix(&source) {
                new_entries.push((relative_path.to_path_buf(), entry.clone()));
            }
        }

        let mut events = Vec::new();
        for (relative_path, entry) in new_entries {
            let new_path = normalize_path(&target.join(relative_path));
            events.push(new_path.clone());
            state.entries.insert(new_path, entry);
        }

        state.emit_event(&events).await;
        Ok(())
    }

    async fn remove_dir(&self, dir_path: &Path, options: RemoveOptions) -> Result<()> {
        let dir_path = normalize_path(dir_path);
        let mut state = self.state.lock().await;
        state.validate_path(&dir_path)?;
        if let Some(entry) = state.entries.get(&dir_path) {
            if !entry.metadata.is_dir {
                return Err(anyhow!(
                    "cannot remove {dir_path:?} because it is not a dir"
                ));
            }

            if !options.recursive {
                let descendants = state
                    .entries
                    .keys()
                    .filter(|path| path.starts_with(path))
                    .count();
                if descendants > 1 {
                    return Err(anyhow!("{dir_path:?} is not empty"));
                }
            }

            state.entries.retain(|path, _| !path.starts_with(&dir_path));
            state.emit_event(&[dir_path]).await;
        } else if !options.ignore_if_not_exists {
            return Err(anyhow!("{dir_path:?} does not exist"));
        }

        Ok(())
    }

    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        let path = normalize_path(path);
        let mut state = self.state.lock().await;
        state.validate_path(&path)?;
        if let Some(entry) = state.entries.get(&path) {
            if entry.metadata.is_dir {
                return Err(anyhow!("cannot remove {path:?} because it is not a file"));
            }

            state.entries.remove(&path);
            state.emit_event(&[path]).await;
        } else if !options.ignore_if_not_exists {
            return Err(anyhow!("{path:?} does not exist"));
        }
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
        let text = state
            .entries
            .get(&path)
            .and_then(|e| e.content.as_ref())
            .ok_or_else(|| anyhow!("file {:?} does not exist", path))?;
        Ok(text.clone())
    }

    async fn save(&self, path: &Path, text: &Rope) -> Result<()> {
        self.simulate_random_delay().await;
        let mut state = self.state.lock().await;
        let path = normalize_path(path);
        state.validate_path(&path)?;
        if let Some(entry) = state.entries.get_mut(&path) {
            if entry.metadata.is_dir {
                Err(anyhow!("cannot overwrite a directory with a file"))
            } else {
                entry.content = Some(text.chunks().collect());
                entry.metadata.mtime = SystemTime::now();
                state.emit_event(&[path]).await;
                Ok(())
            }
        } else {
            let inode = state.next_inode;
            state.next_inode += 1;
            let entry = FakeFsEntry {
                metadata: Metadata {
                    inode,
                    mtime: SystemTime::now(),
                    is_dir: false,
                    is_symlink: false,
                },
                content: Some(text.chunks().collect()),
            };
            state.entries.insert(path.to_path_buf(), entry);
            state.emit_event(&[path]).await;
            Ok(())
        }
    }

    async fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        self.simulate_random_delay().await;
        Ok(normalize_path(path))
    }

    async fn is_file(&self, path: &Path) -> bool {
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock().await;
        state
            .entries
            .get(&path)
            .map_or(false, |entry| !entry.metadata.is_dir)
    }

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>> {
        self.simulate_random_delay().await;
        let state = self.state.lock().await;
        let path = normalize_path(path);
        Ok(state.entries.get(&path).map(|entry| entry.metadata.clone()))
    }

    async fn read_dir(
        &self,
        abs_path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        use futures::{future, stream};
        self.simulate_random_delay().await;
        let state = self.state.lock().await;
        let abs_path = normalize_path(abs_path);
        Ok(Box::pin(stream::iter(state.entries.clone()).filter_map(
            move |(child_path, _)| {
                future::ready(if child_path.parent() == Some(&abs_path) {
                    Some(Ok(child_path))
                } else {
                    None
                })
            },
        )))
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
