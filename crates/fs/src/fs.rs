#[cfg(target_os = "macos")]
mod mac_watcher;

#[cfg(target_os = "linux")]
pub mod linux_watcher;

use anyhow::{anyhow, Result};
use git::GitHostingProviderRegistry;

#[cfg(target_os = "linux")]
use ashpd::desktop::trash;
#[cfg(target_os = "linux")]
use std::{fs::File, os::fd::AsFd};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

use async_tar::Archive;
use futures::{future::BoxFuture, AsyncRead, Stream, StreamExt};
use git::repository::{GitRepository, RealGitRepository};
use gpui::{AppContext, Global, ReadGlobal};
use rope::Rope;
use smol::io::AsyncWriteExt;
use std::{
    io::{self, Write},
    path::{Component, Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tempfile::{NamedTempFile, TempDir};
use text::LineEnding;
use util::ResultExt;

#[cfg(any(test, feature = "test-support"))]
use collections::{btree_map, BTreeMap};
#[cfg(any(test, feature = "test-support"))]
use git::repository::{FakeGitRepositoryState, GitFileStatus};
#[cfg(any(test, feature = "test-support"))]
use parking_lot::Mutex;
#[cfg(any(test, feature = "test-support"))]
use smol::io::AsyncReadExt;
#[cfg(any(test, feature = "test-support"))]
use std::ffi::OsStr;

pub trait Watcher: Send + Sync {
    fn add(&self, path: &Path) -> Result<()>;
    fn remove(&self, path: &Path) -> Result<()>;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathEventKind {
    Removed,
    Created,
    Changed,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathEvent {
    pub path: PathBuf,
    pub kind: Option<PathEventKind>,
}

impl From<PathEvent> for PathBuf {
    fn from(event: PathEvent) -> Self {
        event.path
    }
}

#[async_trait::async_trait]
pub trait Fs: Send + Sync {
    async fn create_dir(&self, path: &Path) -> Result<()>;
    async fn create_symlink(&self, path: &Path, target: PathBuf) -> Result<()>;
    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()>;
    async fn create_file_with(
        &self,
        path: &Path,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()>;
    async fn extract_tar_file(
        &self,
        path: &Path,
        content: Archive<Pin<&mut (dyn AsyncRead + Send)>>,
    ) -> Result<()>;
    async fn copy_file(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()>;
    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()>;
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.remove_dir(path, options).await
    }
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn trash_file(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.remove_file(path, options).await
    }
    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>>;
    async fn load(&self, path: &Path) -> Result<String> {
        Ok(String::from_utf8(self.load_bytes(path).await?)?)
    }
    async fn load_bytes(&self, path: &Path) -> Result<Vec<u8>>;
    async fn atomic_write(&self, path: PathBuf, text: String) -> Result<()>;
    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()>;
    async fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
    async fn is_file(&self, path: &Path) -> bool;
    async fn is_dir(&self, path: &Path) -> bool;
    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>>;
    async fn read_link(&self, path: &Path) -> Result<PathBuf>;
    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>>;

    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    );

    fn open_repo(&self, abs_dot_git: &Path) -> Option<Arc<dyn GitRepository>>;
    fn is_fake(&self) -> bool;
    async fn is_case_sensitive(&self) -> Result<bool>;

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &FakeFs {
        panic!("called as_fake on a real fs");
    }
}

struct GlobalFs(Arc<dyn Fs>);

impl Global for GlobalFs {}

impl dyn Fs {
    /// Returns the global [`Fs`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalFs::global(cx).0.clone()
    }

    /// Sets the global [`Fs`].
    pub fn set_global(fs: Arc<Self>, cx: &mut AppContext) {
        cx.set_global(GlobalFs(fs));
    }
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

#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    pub inode: u64,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub len: u64,
    pub is_fifo: bool,
}

#[derive(Default)]
pub struct RealFs {
    git_hosting_provider_registry: Arc<GitHostingProviderRegistry>,
    git_binary_path: Option<PathBuf>,
}

pub struct RealWatcher {}

impl RealFs {
    pub fn new(
        git_hosting_provider_registry: Arc<GitHostingProviderRegistry>,
        git_binary_path: Option<PathBuf>,
    ) -> Self {
        Self {
            git_hosting_provider_registry,
            git_binary_path,
        }
    }
}

#[async_trait::async_trait]
impl Fs for RealFs {
    async fn create_dir(&self, path: &Path) -> Result<()> {
        Ok(smol::fs::create_dir_all(path).await?)
    }

    async fn create_symlink(&self, path: &Path, target: PathBuf) -> Result<()> {
        #[cfg(unix)]
        smol::fs::unix::symlink(target, path).await?;

        #[cfg(windows)]
        if smol::fs::metadata(&target).await?.is_dir() {
            smol::fs::windows::symlink_dir(target, path).await?
        } else {
            smol::fs::windows::symlink_file(target, path).await?
        }

        Ok(())
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

    async fn create_file_with(
        &self,
        path: &Path,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()> {
        let mut file = smol::fs::File::create(&path).await?;
        futures::io::copy(content, &mut file).await?;
        Ok(())
    }

    async fn extract_tar_file(
        &self,
        path: &Path,
        content: Archive<Pin<&mut (dyn AsyncRead + Send)>>,
    ) -> Result<()> {
        content.unpack(path).await?;
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
        #[cfg(windows)]
        if let Ok(Some(metadata)) = self.metadata(path).await {
            if metadata.is_symlink && metadata.is_dir {
                self.remove_dir(
                    path,
                    RemoveOptions {
                        recursive: false,
                        ignore_if_not_exists: true,
                    },
                )
                .await?;
                return Ok(());
            }
        }

        match smol::fs::remove_file(path).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists => {
                Ok(())
            }
            Err(err) => Err(err)?,
        }
    }

    #[cfg(target_os = "macos")]
    async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        use cocoa::{
            base::{id, nil},
            foundation::{NSAutoreleasePool, NSString},
        };
        use objc::{class, msg_send, sel, sel_impl};

        unsafe {
            unsafe fn ns_string(string: &str) -> id {
                NSString::alloc(nil).init_str(string).autorelease()
            }

            let url: id = msg_send![class!(NSURL), fileURLWithPath: ns_string(path.to_string_lossy().as_ref())];
            let array: id = msg_send![class!(NSArray), arrayWithObject: url];
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];

            let _: id = msg_send![workspace, recycleURLs: array completionHandler: nil];
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        let file = File::open(path)?;
        match trash::trash_file(&file.as_fd()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }

    #[cfg(target_os = "windows")]
    async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        use windows::{
            core::HSTRING,
            Storage::{StorageDeleteOption, StorageFile},
        };
        // todo(windows)
        // When new version of `windows-rs` release, make this operation `async`
        let path = path.canonicalize()?.to_string_lossy().to_string();
        let path_str = path.trim_start_matches("\\\\?\\");
        if path_str.is_empty() {
            anyhow::bail!("File path is empty!");
        }
        let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path_str))?.get()?;
        file.DeleteAsync(StorageDeleteOption::Default)?.get()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.trash_file(path, options).await
    }

    #[cfg(target_os = "linux")]
    async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.trash_file(path, options).await
    }

    #[cfg(target_os = "windows")]
    async fn trash_dir(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        use windows::{
            core::HSTRING,
            Storage::{StorageDeleteOption, StorageFolder},
        };

        let path = path.canonicalize()?.to_string_lossy().to_string();
        let path_str = path.trim_start_matches("\\\\?\\");
        if path_str.is_empty() {
            anyhow::bail!("Folder path is empty!");
        }
        // todo(windows)
        // When new version of `windows-rs` release, make this operation `async`
        let folder = StorageFolder::GetFolderFromPathAsync(&HSTRING::from(path_str))?.get()?;
        folder.DeleteAsync(StorageDeleteOption::Default)?.get()?;
        Ok(())
    }

    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn load(&self, path: &Path) -> Result<String> {
        let path = path.to_path_buf();
        let text = smol::unblock(|| std::fs::read_to_string(path)).await?;
        Ok(text)
    }
    async fn load_bytes(&self, path: &Path) -> Result<Vec<u8>> {
        let path = path.to_path_buf();
        let bytes = smol::unblock(|| std::fs::read(path)).await?;
        Ok(bytes)
    }

    async fn atomic_write(&self, path: PathBuf, data: String) -> Result<()> {
        smol::unblock(move || {
            let mut tmp_file = if cfg!(target_os = "linux") {
                // Use the directory of the destination as temp dir to avoid
                // invalid cross-device link error, and XDG_CACHE_DIR for fallback.
                // See https://github.com/zed-industries/zed/pull/8437 for more details.
                NamedTempFile::new_in(path.parent().unwrap_or(paths::temp_dir()))
            } else if cfg!(target_os = "windows") {
                // If temp dir is set to a different drive than the destination,
                // we receive error:
                //
                // failed to persist temporary file:
                // The system cannot move the file to a different disk drive. (os error 17)
                //
                // So we use the directory of the destination as a temp dir to avoid it.
                // https://github.com/zed-industries/zed/issues/16571
                NamedTempFile::new_in(path.parent().unwrap_or(paths::temp_dir()))
            } else {
                NamedTempFile::new()
            }?;
            tmp_file.write_all(data.as_bytes())?;
            tmp_file.persist(path)?;
            Ok::<(), anyhow::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()> {
        let buffer_size = text.summary().len.min(10 * 1024);
        if let Some(path) = path.parent() {
            self.create_dir(path).await?;
        }
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

    async fn is_dir(&self, path: &Path) -> bool {
        smol::fs::metadata(path)
            .await
            .map_or(false, |metadata| metadata.is_dir())
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

        #[cfg(unix)]
        let inode = metadata.ino();

        #[cfg(windows)]
        let inode = file_id(path).await?;

        #[cfg(windows)]
        let is_fifo = false;

        #[cfg(unix)]
        let is_fifo = metadata.file_type().is_fifo();

        Ok(Some(Metadata {
            inode,
            mtime: metadata.modified().unwrap(),
            len: metadata.len(),
            is_symlink,
            is_dir: metadata.file_type().is_dir(),
            is_fifo,
        }))
    }

    async fn read_link(&self, path: &Path) -> Result<PathBuf> {
        let path = smol::fs::read_link(path).await?;
        Ok(path)
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

    #[cfg(target_os = "macos")]
    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    ) {
        use fsevent::StreamFlags;

        let (events_tx, events_rx) = smol::channel::unbounded();
        let handles = Arc::new(parking_lot::Mutex::new(collections::BTreeMap::default()));
        let watcher = Arc::new(mac_watcher::MacWatcher::new(
            events_tx,
            Arc::downgrade(&handles),
            latency,
        ));
        watcher.add(path).expect("handles can't be dropped");

        (
            Box::pin(
                events_rx
                    .map(|events| {
                        events
                            .into_iter()
                            .map(|event| {
                                let kind = if event.flags.contains(StreamFlags::ITEM_REMOVED) {
                                    Some(PathEventKind::Removed)
                                } else if event.flags.contains(StreamFlags::ITEM_CREATED) {
                                    Some(PathEventKind::Created)
                                } else if event.flags.contains(StreamFlags::ITEM_MODIFIED) {
                                    Some(PathEventKind::Changed)
                                } else {
                                    None
                                };
                                PathEvent {
                                    path: event.path,
                                    kind,
                                }
                            })
                            .collect()
                    })
                    .chain(futures::stream::once(async move {
                        drop(handles);
                        vec![]
                    })),
            ),
            watcher,
        )
    }

    #[cfg(target_os = "linux")]
    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    ) {
        use parking_lot::Mutex;

        let (tx, rx) = smol::channel::unbounded();
        let pending_paths: Arc<Mutex<Vec<PathEvent>>> = Default::default();
        let watcher = Arc::new(linux_watcher::LinuxWatcher::new(tx, pending_paths.clone()));

        watcher.add(&path).ok(); // Ignore "file doesn't exist error" and rely on parent watcher.
        if let Some(parent) = path.parent() {
            // watch the parent dir so we can tell when settings.json is created
            watcher.add(parent).log_err();
        }

        // Check if path is a symlink and follow the target parent
        if let Some(target) = self.read_link(&path).await.ok() {
            watcher.add(&target).ok();
            if let Some(parent) = target.parent() {
                watcher.add(parent).log_err();
            }
        }

        (
            Box::pin(rx.filter_map({
                let watcher = watcher.clone();
                move |_| {
                    let _ = watcher.clone();
                    let pending_paths = pending_paths.clone();
                    async move {
                        smol::Timer::after(latency).await;
                        let paths = std::mem::take(&mut *pending_paths.lock());
                        (!paths.is_empty()).then_some(paths)
                    }
                }
            })),
            watcher,
        )
    }

    #[cfg(target_os = "windows")]
    async fn watch(
        &self,
        path: &Path,
        _latency: Duration,
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    ) {
        use notify::{EventKind, Watcher};

        let (tx, rx) = smol::channel::unbounded();

        let mut file_watcher = notify::recommended_watcher({
            let tx = tx.clone();
            move |event: Result<notify::Event, _>| {
                if let Some(event) = event.log_err() {
                    let kind = match event.kind {
                        EventKind::Create(_) => Some(PathEventKind::Created),
                        EventKind::Modify(_) => Some(PathEventKind::Changed),
                        EventKind::Remove(_) => Some(PathEventKind::Removed),
                        _ => None,
                    };

                    tx.try_send(
                        event
                            .paths
                            .into_iter()
                            .map(|path| PathEvent { path, kind })
                            .collect::<Vec<_>>(),
                    )
                    .ok();
                }
            }
        })
        .expect("Could not start file watcher");

        file_watcher
            .watch(path, notify::RecursiveMode::Recursive)
            .log_err();

        (
            Box::pin(rx.chain(futures::stream::once(async move {
                drop(file_watcher);
                vec![]
            }))),
            Arc::new(RealWatcher {}),
        )
    }

    fn open_repo(&self, dotgit_path: &Path) -> Option<Arc<dyn GitRepository>> {
        let repo = git2::Repository::open(dotgit_path).log_err()?;
        Some(Arc::new(RealGitRepository::new(
            repo,
            self.git_binary_path.clone(),
            self.git_hosting_provider_registry.clone(),
        )))
    }

    fn is_fake(&self) -> bool {
        false
    }

    /// Checks whether the file system is case sensitive by attempting to create two files
    /// that have the same name except for the casing.
    ///
    /// It creates both files in a temporary directory it removes at the end.
    async fn is_case_sensitive(&self) -> Result<bool> {
        let temp_dir = TempDir::new()?;
        let test_file_1 = temp_dir.path().join("case_sensitivity_test.tmp");
        let test_file_2 = temp_dir.path().join("CASE_SENSITIVITY_TEST.TMP");

        let create_opts = CreateOptions {
            overwrite: false,
            ignore_if_exists: false,
        };

        // Create file1
        self.create_file(&test_file_1, create_opts).await?;

        // Now check whether it's possible to create file2
        let case_sensitive = match self.create_file(&test_file_2, create_opts).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if let Some(io_error) = e.downcast_ref::<io::Error>() {
                    if io_error.kind() == io::ErrorKind::AlreadyExists {
                        Ok(false)
                    } else {
                        Err(e)
                    }
                } else {
                    Err(e)
                }
            }
        };

        temp_dir.close()?;
        case_sensitive
    }
}

#[cfg(not(target_os = "linux"))]
impl Watcher for RealWatcher {
    fn add(&self, _: &Path) -> Result<()> {
        Ok(())
    }

    fn remove(&self, _: &Path) -> Result<()> {
        Ok(())
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeFs {
    // Use an unfair lock to ensure tests are deterministic.
    state: Mutex<FakeFsState>,
    executor: gpui::BackgroundExecutor,
}

#[cfg(any(test, feature = "test-support"))]
struct FakeFsState {
    root: Arc<Mutex<FakeFsEntry>>,
    next_inode: u64,
    next_mtime: SystemTime,
    git_event_tx: smol::channel::Sender<PathBuf>,
    event_txs: Vec<smol::channel::Sender<Vec<PathEvent>>>,
    events_paused: bool,
    buffered_events: Vec<PathEvent>,
    metadata_call_count: usize,
    read_dir_call_count: usize,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug)]
enum FakeFsEntry {
    File {
        inode: u64,
        mtime: SystemTime,
        len: u64,
        content: Vec<u8>,
    },
    Dir {
        inode: u64,
        mtime: SystemTime,
        len: u64,
        entries: BTreeMap<String, Arc<Mutex<FakeFsEntry>>>,
        git_repo_state: Option<Arc<Mutex<git::repository::FakeGitRepositoryState>>>,
    },
    Symlink {
        target: PathBuf,
    },
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFsState {
    fn read_path(&self, target: &Path) -> Result<Arc<Mutex<FakeFsEntry>>> {
        Ok(self
            .try_read_path(target, true)
            .ok_or_else(|| {
                anyhow!(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("not found: {}", target.display())
                ))
            })?
            .0)
    }

    fn try_read_path(
        &self,
        target: &Path,
        follow_symlink: bool,
    ) -> Option<(Arc<Mutex<FakeFsEntry>>, PathBuf)> {
        let mut path = target.to_path_buf();
        let mut canonical_path = PathBuf::new();
        let mut entry_stack = Vec::new();
        'outer: loop {
            let mut path_components = path.components().peekable();
            let mut prefix = None;
            while let Some(component) = path_components.next() {
                match component {
                    Component::Prefix(prefix_component) => prefix = Some(prefix_component),
                    Component::RootDir => {
                        entry_stack.clear();
                        entry_stack.push(self.root.clone());
                        canonical_path.clear();
                        match prefix {
                            Some(prefix_component) => {
                                canonical_path = PathBuf::from(prefix_component.as_os_str());
                                // Prefixes like `C:\\` are represented without their trailing slash, so we have to re-add it.
                                canonical_path.push(std::path::MAIN_SEPARATOR_STR);
                            }
                            None => canonical_path = PathBuf::from(std::path::MAIN_SEPARATOR_STR),
                        }
                    }
                    Component::CurDir => {}
                    Component::ParentDir => {
                        entry_stack.pop()?;
                        canonical_path.pop();
                    }
                    Component::Normal(name) => {
                        let current_entry = entry_stack.last().cloned()?;
                        let current_entry = current_entry.lock();
                        if let FakeFsEntry::Dir { entries, .. } = &*current_entry {
                            let entry = entries.get(name.to_str().unwrap()).cloned()?;
                            if path_components.peek().is_some() || follow_symlink {
                                let entry = entry.lock();
                                if let FakeFsEntry::Symlink { target, .. } = &*entry {
                                    let mut target = target.clone();
                                    target.extend(path_components);
                                    path = target;
                                    continue 'outer;
                                }
                            }
                            entry_stack.push(entry.clone());
                            canonical_path = canonical_path.join(name);
                        } else {
                            return None;
                        }
                    }
                }
            }
            break;
        }
        Some((entry_stack.pop()?, canonical_path))
    }

    fn write_path<Fn, T>(&self, path: &Path, callback: Fn) -> Result<T>
    where
        Fn: FnOnce(btree_map::Entry<String, Arc<Mutex<FakeFsEntry>>>) -> Result<T>,
    {
        let path = normalize_path(path);
        let filename = path
            .file_name()
            .ok_or_else(|| anyhow!("cannot overwrite the root"))?;
        let parent_path = path.parent().unwrap();

        let parent = self.read_path(parent_path)?;
        let mut parent = parent.lock();
        let new_entry = parent
            .dir_entries(parent_path)?
            .entry(filename.to_str().unwrap().into());
        callback(new_entry)
    }

    fn emit_event<I, T>(&mut self, paths: I)
    where
        I: IntoIterator<Item = (T, Option<PathEventKind>)>,
        T: Into<PathBuf>,
    {
        self.buffered_events
            .extend(paths.into_iter().map(|(path, kind)| PathEvent {
                path: path.into(),
                kind,
            }));

        if !self.events_paused {
            self.flush_events(self.buffered_events.len());
        }
    }

    fn flush_events(&mut self, mut count: usize) {
        count = count.min(self.buffered_events.len());
        let events = self.buffered_events.drain(0..count).collect::<Vec<_>>();
        self.event_txs.retain(|tx| {
            let _ = tx.try_send(events.clone());
            !tx.is_closed()
        });
    }
}

#[cfg(any(test, feature = "test-support"))]
pub static FS_DOT_GIT: std::sync::LazyLock<&'static OsStr> =
    std::sync::LazyLock::new(|| OsStr::new(".git"));

#[cfg(any(test, feature = "test-support"))]
impl FakeFs {
    /// We need to use something large enough for Windows and Unix to consider this a new file.
    /// https://doc.rust-lang.org/nightly/std/time/struct.SystemTime.html#platform-specific-behavior
    const SYSTEMTIME_INTERVAL: u64 = 100;

    pub fn new(executor: gpui::BackgroundExecutor) -> Arc<Self> {
        let (tx, mut rx) = smol::channel::bounded::<PathBuf>(10);

        let this = Arc::new(Self {
            executor: executor.clone(),
            state: Mutex::new(FakeFsState {
                root: Arc::new(Mutex::new(FakeFsEntry::Dir {
                    inode: 0,
                    mtime: SystemTime::UNIX_EPOCH,
                    len: 0,
                    entries: Default::default(),
                    git_repo_state: None,
                })),
                git_event_tx: tx,
                next_mtime: SystemTime::UNIX_EPOCH,
                next_inode: 1,
                event_txs: Default::default(),
                buffered_events: Vec::new(),
                events_paused: false,
                read_dir_call_count: 0,
                metadata_call_count: 0,
            }),
        });

        executor.spawn({
            let this = this.clone();
            async move {
                while let Some(git_event) = rx.next().await {
                    if let Some(mut state) = this.state.try_lock() {
                        state.emit_event([(git_event, None)]);
                    } else {
                        panic!("Failed to lock file system state, this execution would have caused a test hang");
                    }
                }
            }
        }).detach();

        this
    }

    pub fn set_next_mtime(&self, next_mtime: SystemTime) {
        let mut state = self.state.lock();
        state.next_mtime = next_mtime;
    }

    pub async fn touch_path(&self, path: impl AsRef<Path>) {
        let mut state = self.state.lock();
        let path = path.as_ref();
        let new_mtime = state.next_mtime;
        let new_inode = state.next_inode;
        state.next_inode += 1;
        state.next_mtime += Duration::from_nanos(Self::SYSTEMTIME_INTERVAL);
        state
            .write_path(path, move |entry| {
                match entry {
                    btree_map::Entry::Vacant(e) => {
                        e.insert(Arc::new(Mutex::new(FakeFsEntry::File {
                            inode: new_inode,
                            mtime: new_mtime,
                            content: Vec::new(),
                            len: 0,
                        })));
                    }
                    btree_map::Entry::Occupied(mut e) => match &mut *e.get_mut().lock() {
                        FakeFsEntry::File { mtime, .. } => *mtime = new_mtime,
                        FakeFsEntry::Dir { mtime, .. } => *mtime = new_mtime,
                        FakeFsEntry::Symlink { .. } => {}
                    },
                }
                Ok(())
            })
            .unwrap();
        state.emit_event([(path.to_path_buf(), None)]);
    }

    pub async fn insert_file(&self, path: impl AsRef<Path>, content: Vec<u8>) {
        self.write_file_internal(path, content).unwrap()
    }

    pub async fn insert_symlink(&self, path: impl AsRef<Path>, target: PathBuf) {
        let mut state = self.state.lock();
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
            .unwrap();
        state.emit_event([(path, None)]);
    }

    fn write_file_internal(&self, path: impl AsRef<Path>, content: Vec<u8>) -> Result<()> {
        let mut state = self.state.lock();
        let path = path.as_ref();
        let inode = state.next_inode;
        let mtime = state.next_mtime;
        state.next_inode += 1;
        state.next_mtime += Duration::from_nanos(Self::SYSTEMTIME_INTERVAL);
        let file = Arc::new(Mutex::new(FakeFsEntry::File {
            inode,
            mtime,
            len: content.len() as u64,
            content,
        }));
        let mut kind = None;
        state.write_path(path, {
            let kind = &mut kind;
            move |entry| {
                match entry {
                    btree_map::Entry::Vacant(e) => {
                        *kind = Some(PathEventKind::Created);
                        e.insert(file);
                    }
                    btree_map::Entry::Occupied(mut e) => {
                        *kind = Some(PathEventKind::Changed);
                        *e.get_mut() = file;
                    }
                }
                Ok(())
            }
        })?;
        state.emit_event([(path, kind)]);
        Ok(())
    }

    pub fn read_file_sync(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let path = path.as_ref();
        let path = normalize_path(path);
        let state = self.state.lock();
        let entry = state.read_path(&path)?;
        let entry = entry.lock();
        entry.file_content(&path).cloned()
    }

    async fn load_internal(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let path = path.as_ref();
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock();
        let entry = state.read_path(&path)?;
        let entry = entry.lock();
        entry.file_content(&path).cloned()
    }

    pub fn pause_events(&self) {
        self.state.lock().events_paused = true;
    }

    pub fn buffered_event_count(&self) -> usize {
        self.state.lock().buffered_events.len()
    }

    pub fn flush_events(&self, count: usize) {
        self.state.lock().flush_events(count);
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
                    self.create_dir(path).await.unwrap();
                }
                String(contents) => {
                    self.insert_file(&path, contents.into_bytes()).await;
                }
                _ => {
                    panic!("JSON object must contain only objects, strings, or null");
                }
            }
        }
        .boxed()
    }

    pub fn insert_tree_from_real_fs<'a>(
        &'a self,
        path: impl 'a + AsRef<Path> + Send,
        src_path: impl 'a + AsRef<Path> + Send,
    ) -> futures::future::BoxFuture<'a, ()> {
        use futures::FutureExt as _;

        async move {
            let path = path.as_ref();
            if std::fs::metadata(&src_path).unwrap().is_file() {
                let contents = std::fs::read(src_path).unwrap();
                self.insert_file(path, contents).await;
            } else {
                self.create_dir(path).await.unwrap();
                for entry in std::fs::read_dir(&src_path).unwrap() {
                    let entry = entry.unwrap();
                    self.insert_tree_from_real_fs(path.join(entry.file_name()), entry.path())
                        .await;
                }
            }
        }
        .boxed()
    }

    pub fn with_git_state<F>(&self, dot_git: &Path, emit_git_event: bool, f: F)
    where
        F: FnOnce(&mut FakeGitRepositoryState),
    {
        let mut state = self.state.lock();
        let entry = state.read_path(dot_git).unwrap();
        let mut entry = entry.lock();

        if let FakeFsEntry::Dir { git_repo_state, .. } = &mut *entry {
            let repo_state = git_repo_state.get_or_insert_with(|| {
                Arc::new(Mutex::new(FakeGitRepositoryState::new(
                    dot_git.to_path_buf(),
                    state.git_event_tx.clone(),
                )))
            });
            let mut repo_state = repo_state.lock();

            f(&mut repo_state);

            if emit_git_event {
                state.emit_event([(dot_git, None)]);
            }
        } else {
            panic!("not a directory");
        }
    }

    pub fn set_branch_name(&self, dot_git: &Path, branch: Option<impl Into<String>>) {
        self.with_git_state(dot_git, true, |state| {
            let branch = branch.map(Into::into);
            state.branches.extend(branch.clone());
            state.current_branch_name = branch.map(Into::into)
        })
    }

    pub fn insert_branches(&self, dot_git: &Path, branches: &[&str]) {
        self.with_git_state(dot_git, true, |state| {
            if let Some(first) = branches.first() {
                if state.current_branch_name.is_none() {
                    state.current_branch_name = Some(first.to_string())
                }
            }
            state
                .branches
                .extend(branches.iter().map(ToString::to_string));
        })
    }

    pub fn set_index_for_repo(&self, dot_git: &Path, head_state: &[(&Path, String)]) {
        self.with_git_state(dot_git, true, |state| {
            state.index_contents.clear();
            state.index_contents.extend(
                head_state
                    .iter()
                    .map(|(path, content)| (path.to_path_buf(), content.clone())),
            );
        });
    }

    pub fn set_blame_for_repo(&self, dot_git: &Path, blames: Vec<(&Path, git::blame::Blame)>) {
        self.with_git_state(dot_git, true, |state| {
            state.blames.clear();
            state.blames.extend(
                blames
                    .into_iter()
                    .map(|(path, blame)| (path.to_path_buf(), blame)),
            );
        });
    }

    pub fn set_status_for_repo_via_working_copy_change(
        &self,
        dot_git: &Path,
        statuses: &[(&Path, GitFileStatus)],
    ) {
        self.with_git_state(dot_git, false, |state| {
            state.worktree_statuses.clear();
            state.worktree_statuses.extend(
                statuses
                    .iter()
                    .map(|(path, content)| ((**path).into(), *content)),
            );
        });
        self.state.lock().emit_event(
            statuses
                .iter()
                .map(|(path, _)| (dot_git.parent().unwrap().join(path), None)),
        );
    }

    pub fn set_status_for_repo_via_git_operation(
        &self,
        dot_git: &Path,
        statuses: &[(&Path, GitFileStatus)],
    ) {
        self.with_git_state(dot_git, true, |state| {
            state.worktree_statuses.clear();
            state.worktree_statuses.extend(
                statuses
                    .iter()
                    .map(|(path, content)| ((**path).into(), *content)),
            );
        });
    }

    pub fn paths(&self, include_dot_git: bool) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let mut queue = collections::VecDeque::new();
        queue.push_back((PathBuf::from("/"), self.state.lock().root.clone()));
        while let Some((path, entry)) = queue.pop_front() {
            if let FakeFsEntry::Dir { entries, .. } = &*entry.lock() {
                for (name, entry) in entries {
                    queue.push_back((path.join(name), entry.clone()));
                }
            }
            if include_dot_git
                || !path
                    .components()
                    .any(|component| component.as_os_str() == *FS_DOT_GIT)
            {
                result.push(path);
            }
        }
        result
    }

    pub fn directories(&self, include_dot_git: bool) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let mut queue = collections::VecDeque::new();
        queue.push_back((PathBuf::from("/"), self.state.lock().root.clone()));
        while let Some((path, entry)) = queue.pop_front() {
            if let FakeFsEntry::Dir { entries, .. } = &*entry.lock() {
                for (name, entry) in entries {
                    queue.push_back((path.join(name), entry.clone()));
                }
                if include_dot_git
                    || !path
                        .components()
                        .any(|component| component.as_os_str() == *FS_DOT_GIT)
                {
                    result.push(path);
                }
            }
        }
        result
    }

    pub fn files(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let mut queue = collections::VecDeque::new();
        queue.push_back((PathBuf::from("/"), self.state.lock().root.clone()));
        while let Some((path, entry)) = queue.pop_front() {
            let e = entry.lock();
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

    /// How many `read_dir` calls have been issued.
    pub fn read_dir_call_count(&self) -> usize {
        self.state.lock().read_dir_call_count
    }

    /// How many `metadata` calls have been issued.
    pub fn metadata_call_count(&self) -> usize {
        self.state.lock().metadata_call_count
    }

    fn simulate_random_delay(&self) -> impl futures::Future<Output = ()> {
        self.executor.simulate_random_delay()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFsEntry {
    fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink { .. })
    }

    fn file_content(&self, path: &Path) -> Result<&Vec<u8>> {
        if let Self::File { content, .. } = self {
            Ok(content)
        } else {
            Err(anyhow!("not a file: {}", path.display()))
        }
    }

    fn set_file_content(&mut self, path: &Path, new_content: Vec<u8>) -> Result<()> {
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
struct FakeWatcher {}

#[cfg(any(test, feature = "test-support"))]
impl Watcher for FakeWatcher {
    fn add(&self, _: &Path) -> Result<()> {
        Ok(())
    }

    fn remove(&self, _: &Path) -> Result<()> {
        Ok(())
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait::async_trait]
impl Fs for FakeFs {
    async fn create_dir(&self, path: &Path) -> Result<()> {
        self.simulate_random_delay().await;

        let mut created_dirs = Vec::new();
        let mut cur_path = PathBuf::new();
        for component in path.components() {
            let should_skip = matches!(component, Component::Prefix(..) | Component::RootDir);
            cur_path.push(component);
            if should_skip {
                continue;
            }
            let mut state = self.state.lock();

            let inode = state.next_inode;
            let mtime = state.next_mtime;
            state.next_mtime += Duration::from_nanos(Self::SYSTEMTIME_INTERVAL);
            state.next_inode += 1;
            state.write_path(&cur_path, |entry| {
                entry.or_insert_with(|| {
                    created_dirs.push((cur_path.clone(), Some(PathEventKind::Created)));
                    Arc::new(Mutex::new(FakeFsEntry::Dir {
                        inode,
                        mtime,
                        len: 0,
                        entries: Default::default(),
                        git_repo_state: None,
                    }))
                });
                Ok(())
            })?
        }

        self.state.lock().emit_event(created_dirs);
        Ok(())
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()> {
        self.simulate_random_delay().await;
        let mut state = self.state.lock();
        let inode = state.next_inode;
        let mtime = state.next_mtime;
        state.next_mtime += Duration::from_nanos(Self::SYSTEMTIME_INTERVAL);
        state.next_inode += 1;
        let file = Arc::new(Mutex::new(FakeFsEntry::File {
            inode,
            mtime,
            len: 0,
            content: Vec::new(),
        }));
        let mut kind = Some(PathEventKind::Created);
        state.write_path(path, |entry| {
            match entry {
                btree_map::Entry::Occupied(mut e) => {
                    if options.overwrite {
                        kind = Some(PathEventKind::Changed);
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
        })?;
        state.emit_event([(path, kind)]);
        Ok(())
    }

    async fn create_symlink(&self, path: &Path, target: PathBuf) -> Result<()> {
        let mut state = self.state.lock();
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
            .unwrap();
        state.emit_event([(path, None)]);

        Ok(())
    }

    async fn create_file_with(
        &self,
        path: &Path,
        mut content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()> {
        let mut bytes = Vec::new();
        content.read_to_end(&mut bytes).await?;
        self.write_file_internal(path, bytes)?;
        Ok(())
    }

    async fn extract_tar_file(
        &self,
        path: &Path,
        content: Archive<Pin<&mut (dyn AsyncRead + Send)>>,
    ) -> Result<()> {
        let mut entries = content.entries()?;
        while let Some(entry) = entries.next().await {
            let mut entry = entry?;
            if entry.header().entry_type().is_file() {
                let path = path.join(entry.path()?.as_ref());
                let mut bytes = Vec::new();
                entry.read_to_end(&mut bytes).await?;
                self.create_dir(path.parent().unwrap()).await?;
                self.write_file_internal(&path, bytes)?;
            }
        }
        Ok(())
    }

    async fn rename(&self, old_path: &Path, new_path: &Path, options: RenameOptions) -> Result<()> {
        self.simulate_random_delay().await;

        let old_path = normalize_path(old_path);
        let new_path = normalize_path(new_path);

        let mut state = self.state.lock();
        let moved_entry = state.write_path(&old_path, |e| {
            if let btree_map::Entry::Occupied(e) = e {
                Ok(e.get().clone())
            } else {
                Err(anyhow!("path does not exist: {}", &old_path.display()))
            }
        })?;

        state.write_path(&new_path, |e| {
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
        })?;

        state
            .write_path(&old_path, |e| {
                if let btree_map::Entry::Occupied(e) = e {
                    Ok(e.remove())
                } else {
                    unreachable!()
                }
            })
            .unwrap();

        state.emit_event([
            (old_path, Some(PathEventKind::Removed)),
            (new_path, Some(PathEventKind::Created)),
        ]);
        Ok(())
    }

    async fn copy_file(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()> {
        self.simulate_random_delay().await;

        let source = normalize_path(source);
        let target = normalize_path(target);
        let mut state = self.state.lock();
        let mtime = state.next_mtime;
        let inode = util::post_inc(&mut state.next_inode);
        state.next_mtime += Duration::from_nanos(Self::SYSTEMTIME_INTERVAL);
        let source_entry = state.read_path(&source)?;
        let content = source_entry.lock().file_content(&source)?.clone();
        let mut kind = Some(PathEventKind::Created);
        let entry = state.write_path(&target, |e| match e {
            btree_map::Entry::Occupied(e) => {
                if options.overwrite {
                    kind = Some(PathEventKind::Changed);
                    Ok(Some(e.get().clone()))
                } else if !options.ignore_if_exists {
                    return Err(anyhow!("{target:?} already exists"));
                } else {
                    Ok(None)
                }
            }
            btree_map::Entry::Vacant(e) => Ok(Some(
                e.insert(Arc::new(Mutex::new(FakeFsEntry::File {
                    inode,
                    mtime,
                    len: content.len() as u64,
                    content: Vec::new(),
                })))
                .clone(),
            )),
        })?;
        if let Some(entry) = entry {
            entry.lock().set_file_content(&target, content)?;
        }
        state.emit_event([(target, kind)]);
        Ok(())
    }

    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.simulate_random_delay().await;

        let path = normalize_path(path);
        let parent_path = path
            .parent()
            .ok_or_else(|| anyhow!("cannot remove the root"))?;
        let base_name = path.file_name().unwrap();

        let mut state = self.state.lock();
        let parent_entry = state.read_path(parent_path)?;
        let mut parent_entry = parent_entry.lock();
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
                    let mut entry = e.get().lock();
                    let children = entry.dir_entries(&path)?;
                    if !options.recursive && !children.is_empty() {
                        return Err(anyhow!("{path:?} is not empty"));
                    }
                }
                e.remove();
            }
        }
        state.emit_event([(path, Some(PathEventKind::Removed))]);
        Ok(())
    }

    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.simulate_random_delay().await;

        let path = normalize_path(path);
        let parent_path = path
            .parent()
            .ok_or_else(|| anyhow!("cannot remove the root"))?;
        let base_name = path.file_name().unwrap();
        let mut state = self.state.lock();
        let parent_entry = state.read_path(parent_path)?;
        let mut parent_entry = parent_entry.lock();
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
                e.get().lock().file_content(&path)?;
                e.remove();
            }
        }
        state.emit_event([(path, Some(PathEventKind::Removed))]);
        Ok(())
    }

    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read>> {
        let bytes = self.load_internal(path).await?;
        Ok(Box::new(io::Cursor::new(bytes)))
    }

    async fn load(&self, path: &Path) -> Result<String> {
        let content = self.load_internal(path).await?;
        Ok(String::from_utf8(content.clone())?)
    }

    async fn load_bytes(&self, path: &Path) -> Result<Vec<u8>> {
        self.load_internal(path).await
    }

    async fn atomic_write(&self, path: PathBuf, data: String) -> Result<()> {
        self.simulate_random_delay().await;
        let path = normalize_path(path.as_path());
        self.write_file_internal(path, data.into_bytes())?;
        Ok(())
    }

    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let content = chunks(text, line_ending).collect::<String>();
        if let Some(path) = path.parent() {
            self.create_dir(path).await?;
        }
        self.write_file_internal(path, content.into_bytes())?;
        Ok(())
    }

    async fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock();
        if let Some((_, canonical_path)) = state.try_read_path(&path, true) {
            Ok(canonical_path)
        } else {
            Err(anyhow!("path does not exist: {}", path.display()))
        }
    }

    async fn is_file(&self, path: &Path) -> bool {
        let path = normalize_path(path);
        self.simulate_random_delay().await;
        let state = self.state.lock();
        if let Some((entry, _)) = state.try_read_path(&path, true) {
            entry.lock().is_file()
        } else {
            false
        }
    }

    async fn is_dir(&self, path: &Path) -> bool {
        self.metadata(path)
            .await
            .is_ok_and(|metadata| metadata.is_some_and(|metadata| metadata.is_dir))
    }

    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let mut state = self.state.lock();
        state.metadata_call_count += 1;
        if let Some((mut entry, _)) = state.try_read_path(&path, false) {
            let is_symlink = entry.lock().is_symlink();
            if is_symlink {
                if let Some(e) = state.try_read_path(&path, true).map(|e| e.0) {
                    entry = e;
                } else {
                    return Ok(None);
                }
            }

            let entry = entry.lock();
            Ok(Some(match &*entry {
                FakeFsEntry::File {
                    inode, mtime, len, ..
                } => Metadata {
                    inode: *inode,
                    mtime: *mtime,
                    len: *len,
                    is_dir: false,
                    is_symlink,
                    is_fifo: false,
                },
                FakeFsEntry::Dir {
                    inode, mtime, len, ..
                } => Metadata {
                    inode: *inode,
                    mtime: *mtime,
                    len: *len,
                    is_dir: true,
                    is_symlink,
                    is_fifo: false,
                },
                FakeFsEntry::Symlink { .. } => unreachable!(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn read_link(&self, path: &Path) -> Result<PathBuf> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let state = self.state.lock();
        if let Some((entry, _)) = state.try_read_path(&path, false) {
            let entry = entry.lock();
            if let FakeFsEntry::Symlink { target } = &*entry {
                Ok(target.clone())
            } else {
                Err(anyhow!("not a symlink: {}", path.display()))
            }
        } else {
            Err(anyhow!("path does not exist: {}", path.display()))
        }
    }

    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let mut state = self.state.lock();
        state.read_dir_call_count += 1;
        let entry = state.read_path(&path)?;
        let mut entry = entry.lock();
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
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    ) {
        self.simulate_random_delay().await;
        let (tx, rx) = smol::channel::unbounded();
        self.state.lock().event_txs.push(tx);
        let path = path.to_path_buf();
        let executor = self.executor.clone();
        (
            Box::pin(futures::StreamExt::filter(rx, move |events| {
                let result = events
                    .iter()
                    .any(|evt_path| evt_path.path.starts_with(&path));
                let executor = executor.clone();
                async move {
                    executor.simulate_random_delay().await;
                    result
                }
            })),
            Arc::new(FakeWatcher {}),
        )
    }

    fn open_repo(&self, abs_dot_git: &Path) -> Option<Arc<dyn GitRepository>> {
        let state = self.state.lock();
        let entry = state.read_path(abs_dot_git).unwrap();
        let mut entry = entry.lock();
        if let FakeFsEntry::Dir { git_repo_state, .. } = &mut *entry {
            let state = git_repo_state
                .get_or_insert_with(|| {
                    Arc::new(Mutex::new(FakeGitRepositoryState::new(
                        abs_dot_git.to_path_buf(),
                        state.git_event_tx.clone(),
                    )))
                })
                .clone();
            Some(git::repository::FakeGitRepository::open(state))
        } else {
            None
        }
    }

    fn is_fake(&self) -> bool {
        true
    }

    async fn is_case_sensitive(&self) -> Result<bool> {
        Ok(true)
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
            if !options.overwrite && fs.metadata(target).await.is_ok_and(|m| m.is_some()) {
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

// todo(windows)
// can we get file id not open the file twice?
// https://github.com/rust-lang/rust/issues/63010
#[cfg(target_os = "windows")]
async fn file_id(path: impl AsRef<Path>) -> Result<u64> {
    use std::os::windows::io::AsRawHandle;

    use smol::fs::windows::OpenOptionsExt;
    use windows::Win32::{
        Foundation::HANDLE,
        Storage::FileSystem::{
            GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION, FILE_FLAG_BACKUP_SEMANTICS,
        },
    };

    let file = smol::fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS.0)
        .open(path)
        .await?;

    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    // https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle
    // This function supports Windows XP+
    smol::unblock(move || {
        unsafe { GetFileInformationByHandle(HANDLE(file.as_raw_handle() as _), &mut info)? };

        Ok(((info.nFileIndexHigh as u64) << 32) | (info.nFileIndexLow as u64))
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::BackgroundExecutor;
    use serde_json::json;

    #[gpui::test]
    async fn test_fake_fs(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
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
            fs.files(),
            vec![
                PathBuf::from("/root/dir1/a"),
                PathBuf::from("/root/dir1/b"),
                PathBuf::from("/root/dir2/c"),
                PathBuf::from("/root/dir2/dir3/d"),
            ]
        );

        fs.create_symlink("/root/dir2/link-to-dir3".as_ref(), "./dir3".into())
            .await
            .unwrap();

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
