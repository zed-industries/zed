use anyhow::{anyhow, Result};
use fsevent::EventStream;
use futures::{Stream, StreamExt};
use postage::prelude::Sink as _;
use smol::io::{AsyncReadExt, AsyncWriteExt};
use std::{
    io,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    pin::Pin,
    time::{Duration, SystemTime},
};
use text::Rope;

#[async_trait::async_trait]
pub trait Fs: Send + Sync {
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
        let (mut tx, rx) = postage::mpsc::channel(64);
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
    events_tx: postage::broadcast::Sender<Vec<fsevent::Event>>,
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

    async fn emit_event(&mut self, paths: &[&Path]) {
        let events = paths
            .iter()
            .map(|path| fsevent::Event {
                event_id: 0,
                flags: fsevent::StreamFlags::empty(),
                path: path.to_path_buf(),
            })
            .collect();

        let _ = self.events_tx.send(events).await;
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeFs {
    // Use an unfair lock to ensure tests are deterministic.
    state: futures::lock::Mutex<FakeFsState>,
    executor: std::sync::Arc<gpui::executor::Background>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFs {
    pub fn new(executor: std::sync::Arc<gpui::executor::Background>) -> Self {
        let (events_tx, _) = postage::broadcast::channel(2048);
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
        Self {
            executor,
            state: futures::lock::Mutex::new(FakeFsState {
                entries,
                next_inode: 1,
                events_tx,
            }),
        }
    }

    pub async fn insert_dir(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut state = self.state.lock().await;
        let path = path.as_ref();
        state.validate_path(path)?;

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
        Ok(())
    }

    pub async fn insert_file(&self, path: impl AsRef<Path>, content: String) -> Result<()> {
        let mut state = self.state.lock().await;
        let path = path.as_ref();
        state.validate_path(path)?;

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
        Ok(())
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
                    self.insert_dir(path).await.unwrap();
                    for (name, contents) in map {
                        let mut path = PathBuf::from(path);
                        path.push(name);
                        self.insert_tree(&path, contents).await;
                    }
                }
                Null => {
                    self.insert_dir(&path).await.unwrap();
                }
                String(contents) => {
                    self.insert_file(&path, contents).await.unwrap();
                }
                _ => {
                    panic!("JSON object must contain only objects, strings, or null");
                }
            }
        }
        .boxed()
    }

    pub async fn remove(&self, path: &Path) -> Result<()> {
        let mut state = self.state.lock().await;
        state.validate_path(path)?;
        state.entries.retain(|path, _| !path.starts_with(path));
        state.emit_event(&[path]).await;
        Ok(())
    }

    pub async fn rename(&self, source: &Path, target: &Path) -> Result<()> {
        let mut state = self.state.lock().await;
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

            state.emit_event(&[source, target]).await;
            Ok(())
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait::async_trait]
impl Fs for FakeFs {
    async fn load(&self, path: &Path) -> Result<String> {
        self.executor.simulate_random_delay().await;
        let state = self.state.lock().await;
        let text = state
            .entries
            .get(path)
            .and_then(|e| e.content.as_ref())
            .ok_or_else(|| anyhow!("file {:?} does not exist", path))?;
        Ok(text.clone())
    }

    async fn save(&self, path: &Path, text: &Rope) -> Result<()> {
        self.executor.simulate_random_delay().await;
        let mut state = self.state.lock().await;
        state.validate_path(path)?;
        if let Some(entry) = state.entries.get_mut(path) {
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
        self.executor.simulate_random_delay().await;
        Ok(path.to_path_buf())
    }

    async fn is_file(&self, path: &Path) -> bool {
        self.executor.simulate_random_delay().await;
        let state = self.state.lock().await;
        state
            .entries
            .get(path)
            .map_or(false, |entry| !entry.metadata.is_dir)
    }

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>> {
        self.executor.simulate_random_delay().await;
        let state = self.state.lock().await;
        Ok(state.entries.get(path).map(|entry| entry.metadata.clone()))
    }

    async fn read_dir(
        &self,
        abs_path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        use futures::{future, stream};
        self.executor.simulate_random_delay().await;
        let state = self.state.lock().await;
        let abs_path = abs_path.to_path_buf();
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
        let state = self.state.lock().await;
        self.executor.simulate_random_delay().await;
        let rx = state.events_tx.subscribe();
        let path = path.to_path_buf();
        Box::pin(futures::StreamExt::filter(rx, move |events| {
            let result = events.iter().any(|event| event.path.starts_with(&path));
            async move { result }
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
