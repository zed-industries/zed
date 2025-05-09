#[cfg(target_os = "macos")]
mod mac_watcher;

#[cfg(not(target_os = "macos"))]
pub mod fs_watcher;

use anyhow::{Context as _, Result, anyhow};
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use ashpd::desktop::trash;
use gpui::App;
use gpui::BackgroundExecutor;
use gpui::Global;
use gpui::ReadGlobal as _;
use std::borrow::Cow;
use util::command::new_std_command;

#[cfg(unix)]
use std::os::fd::{AsFd, AsRawFd};

#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, MetadataExt};

use async_tar::Archive;
use futures::{AsyncRead, Stream, StreamExt, future::BoxFuture};
use git::repository::{GitRepository, RealGitRepository};
use rope::Rope;
use serde::{Deserialize, Serialize};
use smol::io::AsyncWriteExt;
use std::{
    io::{self, Write},
    path::{Component, Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::TempDir;
use text::LineEnding;

#[cfg(any(test, feature = "test-support"))]
mod fake_git_repo;
#[cfg(any(test, feature = "test-support"))]
use collections::{BTreeMap, btree_map};
#[cfg(any(test, feature = "test-support"))]
use fake_git_repo::FakeGitRepositoryState;
#[cfg(any(test, feature = "test-support"))]
use git::{
    repository::RepoPath,
    status::{FileStatus, StatusCode, TrackedStatus, UnmergedStatus},
};
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PathEventKind {
    Removed,
    Created,
    Changed,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    async fn open_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>>;
    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read + Send + Sync>>;
    async fn load(&self, path: &Path) -> Result<String> {
        Ok(String::from_utf8(self.load_bytes(path).await?)?)
    }
    async fn load_bytes(&self, path: &Path) -> Result<Vec<u8>>;
    async fn atomic_write(&self, path: PathBuf, text: String) -> Result<()>;
    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()>;
    async fn write(&self, path: &Path, content: &[u8]) -> Result<()>;
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

    fn home_dir(&self) -> Option<PathBuf>;
    fn open_repo(&self, abs_dot_git: &Path) -> Option<Arc<dyn GitRepository>>;
    fn git_init(&self, abs_work_directory: &Path, fallback_branch_name: String) -> Result<()>;
    fn is_fake(&self) -> bool;
    async fn is_case_sensitive(&self) -> Result<bool>;

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> Arc<FakeFs> {
        panic!("called as_fake on a real fs");
    }
}

struct GlobalFs(Arc<dyn Fs>);

impl Global for GlobalFs {}

impl dyn Fs {
    /// Returns the global [`Fs`].
    pub fn global(cx: &App) -> Arc<Self> {
        GlobalFs::global(cx).0.clone()
    }

    /// Sets the global [`Fs`].
    pub fn set_global(fs: Arc<Self>, cx: &mut App) {
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
    pub mtime: MTime,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub len: u64,
    pub is_fifo: bool,
}

/// Filesystem modification time. The purpose of this newtype is to discourage use of operations
/// that do not make sense for mtimes. In particular, it is not always valid to compare mtimes using
/// `<` or `>`, as there are many things that can cause the mtime of a file to be earlier than it
/// was. See ["mtime comparison considered harmful" - apenwarr](https://apenwarr.ca/log/20181113).
///
/// Do not derive Ord, PartialOrd, or arithmetic operation traits.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MTime(SystemTime);

impl MTime {
    /// Conversion intended for persistence and testing.
    pub fn from_seconds_and_nanos(secs: u64, nanos: u32) -> Self {
        MTime(UNIX_EPOCH + Duration::new(secs, nanos))
    }

    /// Conversion intended for persistence.
    pub fn to_seconds_and_nanos_for_persistence(self) -> Option<(u64, u32)> {
        self.0
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|duration| (duration.as_secs(), duration.subsec_nanos()))
    }

    /// Returns the value wrapped by this `MTime`, for presentation to the user. The name including
    /// "_for_user" is to discourage misuse - this method should not be used when making decisions
    /// about file dirtiness.
    pub fn timestamp_for_user(self) -> SystemTime {
        self.0
    }

    /// Temporary method to split out the behavior changes from introduction of this newtype.
    pub fn bad_is_greater_than(self, other: MTime) -> bool {
        self.0 > other.0
    }
}

impl From<proto::Timestamp> for MTime {
    fn from(timestamp: proto::Timestamp) -> Self {
        MTime(timestamp.into())
    }
}

impl From<MTime> for proto::Timestamp {
    fn from(mtime: MTime) -> Self {
        mtime.0.into()
    }
}

pub struct RealFs {
    git_binary_path: Option<PathBuf>,
    executor: BackgroundExecutor,
}

pub trait FileHandle: Send + Sync + std::fmt::Debug {
    fn current_path(&self, fs: &Arc<dyn Fs>) -> Result<PathBuf>;
}

impl FileHandle for std::fs::File {
    #[cfg(target_os = "macos")]
    fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
        use std::{
            ffi::{CStr, OsStr},
            os::unix::ffi::OsStrExt,
        };

        let fd = self.as_fd();
        let mut path_buf: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];

        let result = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETPATH, path_buf.as_mut_ptr()) };
        if result == -1 {
            anyhow::bail!("fcntl returned -1".to_string());
        }

        let c_str = unsafe { CStr::from_ptr(path_buf.as_ptr()) };
        let path = PathBuf::from(OsStr::from_bytes(c_str.to_bytes()));
        Ok(path)
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
        let fd = self.as_fd();
        let fd_path = format!("/proc/self/fd/{}", fd.as_raw_fd());
        let new_path = std::fs::read_link(fd_path)?;
        if new_path
            .file_name()
            .is_some_and(|f| f.to_string_lossy().ends_with(" (deleted)"))
        {
            anyhow::bail!("file was deleted")
        };

        Ok(new_path)
    }

    #[cfg(target_os = "windows")]
    fn current_path(&self, _: &Arc<dyn Fs>) -> Result<PathBuf> {
        anyhow::bail!("unimplemented")
    }
}

pub struct RealWatcher {}

impl RealFs {
    pub fn new(git_binary_path: Option<PathBuf>, executor: BackgroundExecutor) -> Self {
        Self {
            git_binary_path,
            executor,
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
                unsafe { NSString::alloc(nil).init_str(string).autorelease() }
            }

            let url: id = msg_send![class!(NSURL), fileURLWithPath: ns_string(path.to_string_lossy().as_ref())];
            let array: id = msg_send![class!(NSArray), arrayWithObject: url];
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];

            let _: id = msg_send![workspace, recycleURLs: array completionHandler: nil];
        }
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        if let Ok(Some(metadata)) = self.metadata(path).await {
            if metadata.is_symlink {
                // TODO: trash_file does not support trashing symlinks yet - https://github.com/bilelmoussaoui/ashpd/issues/255
                return self.remove_file(path, RemoveOptions::default()).await;
            }
        }
        let file = smol::fs::File::open(path).await?;
        match trash::trash_file(&file.as_fd()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Failed to trash file: {}", err);
                // Trashing files can fail if you don't have a trashing dbus service configured.
                // In that case, delete the file directly instead.
                return self.remove_file(path, RemoveOptions::default()).await;
            }
        }
    }

    #[cfg(target_os = "windows")]
    async fn trash_file(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        use util::paths::SanitizedPath;
        use windows::{
            Storage::{StorageDeleteOption, StorageFile},
            core::HSTRING,
        };
        // todo(windows)
        // When new version of `windows-rs` release, make this operation `async`
        let path = SanitizedPath::from(path.canonicalize()?);
        let path_string = path.to_string();
        let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path_string))?.get()?;
        file.DeleteAsync(StorageDeleteOption::Default)?.get()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.trash_file(path, options).await
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    async fn trash_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        self.trash_file(path, options).await
    }

    #[cfg(target_os = "windows")]
    async fn trash_dir(&self, path: &Path, _options: RemoveOptions) -> Result<()> {
        use util::paths::SanitizedPath;
        use windows::{
            Storage::{StorageDeleteOption, StorageFolder},
            core::HSTRING,
        };

        // todo(windows)
        // When new version of `windows-rs` release, make this operation `async`
        let path = SanitizedPath::from(path.canonicalize()?);
        let path_string = path.to_string();
        let folder = StorageFolder::GetFolderFromPathAsync(&HSTRING::from(path_string))?.get()?;
        folder.DeleteAsync(StorageDeleteOption::Default)?.get()?;
        Ok(())
    }

    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read + Send + Sync>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn open_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>> {
        Ok(Arc::new(std::fs::File::open(path)?))
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

    #[cfg(not(target_os = "windows"))]
    async fn atomic_write(&self, path: PathBuf, data: String) -> Result<()> {
        smol::unblock(move || {
            let mut tmp_file = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                // Use the directory of the destination as temp dir to avoid
                // invalid cross-device link error, and XDG_CACHE_DIR for fallback.
                // See https://github.com/zed-industries/zed/pull/8437 for more details.
                tempfile::NamedTempFile::new_in(path.parent().unwrap_or(paths::temp_dir()))
            } else {
                tempfile::NamedTempFile::new()
            }?;
            tmp_file.write_all(data.as_bytes())?;
            tmp_file.persist(path)?;
            Ok::<(), anyhow::Error>(())
        })
        .await?;

        Ok(())
    }

    #[cfg(target_os = "windows")]
    async fn atomic_write(&self, path: PathBuf, data: String) -> Result<()> {
        smol::unblock(move || {
            // If temp dir is set to a different drive than the destination,
            // we receive error:
            //
            // failed to persist temporary file:
            // The system cannot move the file to a different disk drive. (os error 17)
            //
            // This is because `ReplaceFileW` does not support cross volume moves.
            // See the remark section: "The backup file, replaced file, and replacement file must all reside on the same volume."
            // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew#remarks
            //
            // So we use the directory of the destination as a temp dir to avoid it.
            // https://github.com/zed-industries/zed/issues/16571
            let temp_dir = TempDir::new_in(path.parent().unwrap_or(paths::temp_dir()))?;
            let temp_file = {
                let temp_file_path = temp_dir.path().join("temp_file");
                let mut file = std::fs::File::create_new(&temp_file_path)?;
                file.write_all(data.as_bytes())?;
                temp_file_path
            };
            atomic_replace(path.as_path(), temp_file.as_path())?;
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

    async fn write(&self, path: &Path, content: &[u8]) -> Result<()> {
        if let Some(path) = path.parent() {
            self.create_dir(path).await?;
        }
        smol::fs::write(path, content).await?;
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
                };
            }
        };

        let path_buf = path.to_path_buf();
        let path_exists = smol::unblock(move || {
            path_buf
                .try_exists()
                .with_context(|| format!("checking existence for path {path_buf:?}"))
        })
        .await?;
        let is_symlink = symlink_metadata.file_type().is_symlink();
        let metadata = match (is_symlink, path_exists) {
            (true, true) => smol::fs::metadata(path)
                .await
                .with_context(|| "accessing symlink for path {path}")?,
            _ => symlink_metadata,
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
            mtime: MTime(metadata.modified().unwrap()),
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

    #[cfg(not(target_os = "macos"))]
    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    ) {
        use parking_lot::Mutex;
        use util::{ResultExt as _, paths::SanitizedPath};

        let (tx, rx) = smol::channel::unbounded();
        let pending_paths: Arc<Mutex<Vec<PathEvent>>> = Default::default();
        let watcher = Arc::new(fs_watcher::FsWatcher::new(tx, pending_paths.clone()));

        if watcher.add(path).is_err() {
            // If the path doesn't exist yet (e.g. settings.json), watch the parent dir to learn when it's created.
            if let Some(parent) = path.parent() {
                if let Err(e) = watcher.add(parent) {
                    log::warn!("Failed to watch: {e}");
                }
            }
        }

        // Check if path is a symlink and follow the target parent
        if let Some(mut target) = self.read_link(&path).await.ok() {
            // Check if symlink target is relative path, if so make it absolute
            if target.is_relative() {
                if let Some(parent) = path.parent() {
                    target = parent.join(target);
                    if let Ok(canonical) = self.canonicalize(&target).await {
                        target = SanitizedPath::from(canonical).as_path().to_path_buf();
                    }
                }
            }
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

    fn open_repo(&self, dotgit_path: &Path) -> Option<Arc<dyn GitRepository>> {
        Some(Arc::new(RealGitRepository::new(
            dotgit_path,
            self.git_binary_path.clone(),
            self.executor.clone(),
        )?))
    }

    fn git_init(&self, abs_work_directory_path: &Path, fallback_branch_name: String) -> Result<()> {
        let config = new_std_command("git")
            .current_dir(abs_work_directory_path)
            .args(&["config", "--global", "--get", "init.defaultBranch"])
            .output()?;

        let branch_name;

        if config.status.success() && !config.stdout.is_empty() {
            branch_name = String::from_utf8_lossy(&config.stdout);
        } else {
            branch_name = Cow::Borrowed(fallback_branch_name.as_str());
        }

        new_std_command("git")
            .current_dir(abs_work_directory_path)
            .args(&["init", "-b"])
            .arg(branch_name.trim())
            .output()?;

        Ok(())
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

    fn home_dir(&self) -> Option<PathBuf> {
        Some(paths::home_dir().clone())
    }
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
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
    this: std::sync::Weak<Self>,
    // Use an unfair lock to ensure tests are deterministic.
    state: Arc<Mutex<FakeFsState>>,
    executor: gpui::BackgroundExecutor,
}

#[cfg(any(test, feature = "test-support"))]
struct FakeFsState {
    root: Arc<Mutex<FakeFsEntry>>,
    next_inode: u64,
    next_mtime: SystemTime,
    git_event_tx: smol::channel::Sender<PathBuf>,
    event_txs: Vec<(PathBuf, smol::channel::Sender<Vec<PathEvent>>)>,
    events_paused: bool,
    buffered_events: Vec<PathEvent>,
    metadata_call_count: usize,
    read_dir_call_count: usize,
    moves: std::collections::HashMap<u64, PathBuf>,
    home_dir: Option<PathBuf>,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug)]
enum FakeFsEntry {
    File {
        inode: u64,
        mtime: MTime,
        len: u64,
        content: Vec<u8>,
        // The path to the repository state directory, if this is a gitfile.
        git_dir_path: Option<PathBuf>,
    },
    Dir {
        inode: u64,
        mtime: MTime,
        len: u64,
        entries: BTreeMap<String, Arc<Mutex<FakeFsEntry>>>,
        git_repo_state: Option<Arc<Mutex<FakeGitRepositoryState>>>,
    },
    Symlink {
        target: PathBuf,
    },
}

#[cfg(any(test, feature = "test-support"))]
impl FakeFsState {
    fn get_and_increment_mtime(&mut self) -> MTime {
        let mtime = self.next_mtime;
        self.next_mtime += FakeFs::SYSTEMTIME_INTERVAL;
        MTime(mtime)
    }

    fn get_and_increment_inode(&mut self) -> u64 {
        let inode = self.next_inode;
        self.next_inode += 1;
        inode
    }

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
        self.event_txs.retain(|(_, tx)| {
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
    const SYSTEMTIME_INTERVAL: Duration = Duration::from_nanos(100);

    pub fn new(executor: gpui::BackgroundExecutor) -> Arc<Self> {
        let (tx, rx) = smol::channel::bounded::<PathBuf>(10);

        let this = Arc::new_cyclic(|this| Self {
            this: this.clone(),
            executor: executor.clone(),
            state: Arc::new(Mutex::new(FakeFsState {
                root: Arc::new(Mutex::new(FakeFsEntry::Dir {
                    inode: 0,
                    mtime: MTime(UNIX_EPOCH),
                    len: 0,
                    entries: Default::default(),
                    git_repo_state: None,
                })),
                git_event_tx: tx,
                next_mtime: UNIX_EPOCH + Self::SYSTEMTIME_INTERVAL,
                next_inode: 1,
                event_txs: Default::default(),
                buffered_events: Vec::new(),
                events_paused: false,
                read_dir_call_count: 0,
                metadata_call_count: 0,
                moves: Default::default(),
                home_dir: None,
            })),
        });

        executor.spawn({
            let this = this.clone();
            async move {
                while let Ok(git_event) = rx.recv().await {
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

    pub fn get_and_increment_mtime(&self) -> MTime {
        let mut state = self.state.lock();
        state.get_and_increment_mtime()
    }

    pub async fn touch_path(&self, path: impl AsRef<Path>) {
        let mut state = self.state.lock();
        let path = path.as_ref();
        let new_mtime = state.get_and_increment_mtime();
        let new_inode = state.get_and_increment_inode();
        state
            .write_path(path, move |entry| {
                match entry {
                    btree_map::Entry::Vacant(e) => {
                        e.insert(Arc::new(Mutex::new(FakeFsEntry::File {
                            inode: new_inode,
                            mtime: new_mtime,
                            content: Vec::new(),
                            len: 0,
                            git_dir_path: None,
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
        self.write_file_internal(path, content, true).unwrap()
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

    fn write_file_internal(
        &self,
        path: impl AsRef<Path>,
        new_content: Vec<u8>,
        recreate_inode: bool,
    ) -> Result<()> {
        let mut state = self.state.lock();
        let new_inode = state.get_and_increment_inode();
        let new_mtime = state.get_and_increment_mtime();
        let new_len = new_content.len() as u64;
        let mut kind = None;
        state.write_path(path.as_ref(), |entry| {
            match entry {
                btree_map::Entry::Vacant(e) => {
                    kind = Some(PathEventKind::Created);
                    e.insert(Arc::new(Mutex::new(FakeFsEntry::File {
                        inode: new_inode,
                        mtime: new_mtime,
                        len: new_len,
                        content: new_content,
                        git_dir_path: None,
                    })));
                }
                btree_map::Entry::Occupied(mut e) => {
                    kind = Some(PathEventKind::Changed);
                    if let FakeFsEntry::File {
                        inode,
                        mtime,
                        len,
                        content,
                        ..
                    } = &mut *e.get_mut().lock()
                    {
                        *mtime = new_mtime;
                        *content = new_content;
                        *len = new_len;
                        if recreate_inode {
                            *inode = new_inode;
                        }
                    } else {
                        anyhow::bail!("not a file")
                    }
                }
            }
            Ok(())
        })?;
        state.emit_event([(path.as_ref(), kind)]);
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

    pub fn unpause_events_and_flush(&self) {
        self.state.lock().events_paused = false;
        self.flush_events(usize::MAX);
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

    pub fn with_git_state_and_paths<T, F>(
        &self,
        dot_git: &Path,
        emit_git_event: bool,
        f: F,
    ) -> Result<T>
    where
        F: FnOnce(&mut FakeGitRepositoryState, &Path, &Path) -> T,
    {
        let mut state = self.state.lock();
        let entry = state.read_path(dot_git).context("open .git")?;
        let mut entry = entry.lock();

        if let FakeFsEntry::Dir { git_repo_state, .. } = &mut *entry {
            let repo_state = git_repo_state.get_or_insert_with(|| {
                log::debug!("insert git state for {dot_git:?}");
                Arc::new(Mutex::new(FakeGitRepositoryState::new(
                    state.git_event_tx.clone(),
                )))
            });
            let mut repo_state = repo_state.lock();

            let result = f(&mut repo_state, dot_git, dot_git);

            if emit_git_event {
                state.emit_event([(dot_git, None)]);
            }

            Ok(result)
        } else if let FakeFsEntry::File {
            content,
            git_dir_path,
            ..
        } = &mut *entry
        {
            let path = match git_dir_path {
                Some(path) => path,
                None => {
                    let path = std::str::from_utf8(content)
                        .ok()
                        .and_then(|content| content.strip_prefix("gitdir:"))
                        .ok_or_else(|| anyhow!("not a valid gitfile"))?
                        .trim();
                    git_dir_path.insert(normalize_path(&dot_git.parent().unwrap().join(path)))
                }
            }
            .clone();
            drop(entry);
            let Some((git_dir_entry, canonical_path)) = state.try_read_path(&path, true) else {
                anyhow::bail!("pointed-to git dir {path:?} not found")
            };
            let FakeFsEntry::Dir {
                git_repo_state,
                entries,
                ..
            } = &mut *git_dir_entry.lock()
            else {
                anyhow::bail!("gitfile points to a non-directory")
            };
            let common_dir = if let Some(child) = entries.get("commondir") {
                Path::new(
                    std::str::from_utf8(child.lock().file_content("commondir".as_ref())?)
                        .context("commondir content")?,
                )
                .to_owned()
            } else {
                canonical_path.clone()
            };
            let repo_state = git_repo_state.get_or_insert_with(|| {
                Arc::new(Mutex::new(FakeGitRepositoryState::new(
                    state.git_event_tx.clone(),
                )))
            });
            let mut repo_state = repo_state.lock();

            let result = f(&mut repo_state, &canonical_path, &common_dir);

            if emit_git_event {
                state.emit_event([(canonical_path, None)]);
            }

            Ok(result)
        } else {
            Err(anyhow!("not a valid git repository"))
        }
    }

    pub fn with_git_state<T, F>(&self, dot_git: &Path, emit_git_event: bool, f: F) -> Result<T>
    where
        F: FnOnce(&mut FakeGitRepositoryState) -> T,
    {
        self.with_git_state_and_paths(dot_git, emit_git_event, |state, _, _| f(state))
    }

    pub fn set_branch_name(&self, dot_git: &Path, branch: Option<impl Into<String>>) {
        self.with_git_state(dot_git, true, |state| {
            let branch = branch.map(Into::into);
            state.branches.extend(branch.clone());
            state.current_branch_name = branch
        })
        .unwrap();
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
        .unwrap();
    }

    pub fn set_unmerged_paths_for_repo(
        &self,
        dot_git: &Path,
        unmerged_state: &[(RepoPath, UnmergedStatus)],
    ) {
        self.with_git_state(dot_git, true, |state| {
            state.unmerged_paths.clear();
            state.unmerged_paths.extend(
                unmerged_state
                    .iter()
                    .map(|(path, content)| (path.clone(), *content)),
            );
        })
        .unwrap();
    }

    pub fn set_index_for_repo(&self, dot_git: &Path, index_state: &[(RepoPath, String)]) {
        self.with_git_state(dot_git, true, |state| {
            state.index_contents.clear();
            state.index_contents.extend(
                index_state
                    .iter()
                    .map(|(path, content)| (path.clone(), content.clone())),
            );
        })
        .unwrap();
    }

    pub fn set_head_for_repo(&self, dot_git: &Path, head_state: &[(RepoPath, String)]) {
        self.with_git_state(dot_git, true, |state| {
            state.head_contents.clear();
            state.head_contents.extend(
                head_state
                    .iter()
                    .map(|(path, content)| (path.clone(), content.clone())),
            );
        })
        .unwrap();
    }

    pub fn set_git_content_for_repo(
        &self,
        dot_git: &Path,
        head_state: &[(RepoPath, String, Option<String>)],
    ) {
        self.with_git_state(dot_git, true, |state| {
            state.head_contents.clear();
            state.head_contents.extend(
                head_state
                    .iter()
                    .map(|(path, head_content, _)| (path.clone(), head_content.clone())),
            );
            state.index_contents.clear();
            state.index_contents.extend(head_state.iter().map(
                |(path, head_content, index_content)| {
                    (
                        path.clone(),
                        index_content.as_ref().unwrap_or(head_content).clone(),
                    )
                },
            ));
        })
        .unwrap();
    }

    pub fn set_head_and_index_for_repo(
        &self,
        dot_git: &Path,
        contents_by_path: &[(RepoPath, String)],
    ) {
        self.with_git_state(dot_git, true, |state| {
            state.head_contents.clear();
            state.index_contents.clear();
            state.head_contents.extend(contents_by_path.iter().cloned());
            state
                .index_contents
                .extend(contents_by_path.iter().cloned());
        })
        .unwrap();
    }

    pub fn set_blame_for_repo(&self, dot_git: &Path, blames: Vec<(RepoPath, git::blame::Blame)>) {
        self.with_git_state(dot_git, true, |state| {
            state.blames.clear();
            state.blames.extend(blames);
        })
        .unwrap();
    }

    /// Put the given git repository into a state with the given status,
    /// by mutating the head, index, and unmerged state.
    pub fn set_status_for_repo(&self, dot_git: &Path, statuses: &[(&Path, FileStatus)]) {
        let workdir_path = dot_git.parent().unwrap();
        let workdir_contents = self.files_with_contents(&workdir_path);
        self.with_git_state(dot_git, true, |state| {
            state.index_contents.clear();
            state.head_contents.clear();
            state.unmerged_paths.clear();
            for (path, content) in workdir_contents {
                let repo_path: RepoPath = path.strip_prefix(&workdir_path).unwrap().into();
                let status = statuses
                    .iter()
                    .find_map(|(p, status)| (**p == *repo_path.0).then_some(status));
                let mut content = String::from_utf8_lossy(&content).to_string();

                let mut index_content = None;
                let mut head_content = None;
                match status {
                    None => {
                        index_content = Some(content.clone());
                        head_content = Some(content);
                    }
                    Some(FileStatus::Untracked | FileStatus::Ignored) => {}
                    Some(FileStatus::Unmerged(unmerged_status)) => {
                        state
                            .unmerged_paths
                            .insert(repo_path.clone(), *unmerged_status);
                        content.push_str(" (unmerged)");
                        index_content = Some(content.clone());
                        head_content = Some(content);
                    }
                    Some(FileStatus::Tracked(TrackedStatus {
                        index_status,
                        worktree_status,
                    })) => {
                        match worktree_status {
                            StatusCode::Modified => {
                                let mut content = content.clone();
                                content.push_str(" (modified in working copy)");
                                index_content = Some(content);
                            }
                            StatusCode::TypeChanged | StatusCode::Unmodified => {
                                index_content = Some(content.clone());
                            }
                            StatusCode::Added => {}
                            StatusCode::Deleted | StatusCode::Renamed | StatusCode::Copied => {
                                panic!("cannot create these statuses for an existing file");
                            }
                        };
                        match index_status {
                            StatusCode::Modified => {
                                let mut content = index_content.clone().expect(
                                    "file cannot be both modified in index and created in working copy",
                                );
                                content.push_str(" (modified in index)");
                                head_content = Some(content);
                            }
                            StatusCode::TypeChanged | StatusCode::Unmodified => {
                                head_content = Some(index_content.clone().expect("file cannot be both unmodified in index and created in working copy"));
                            }
                            StatusCode::Added => {}
                            StatusCode::Deleted  => {
                                head_content = Some("".into());
                            }
                            StatusCode::Renamed | StatusCode::Copied => {
                                panic!("cannot create these statuses for an existing file");
                            }
                        };
                    }
                };

                if let Some(content) = index_content {
                    state.index_contents.insert(repo_path.clone(), content);
                }
                if let Some(content) = head_content {
                    state.head_contents.insert(repo_path.clone(), content);
                }
            }
        }).unwrap();
    }

    pub fn set_error_message_for_index_write(&self, dot_git: &Path, message: Option<String>) {
        self.with_git_state(dot_git, true, |state| {
            state.simulated_index_write_error_message = message;
        })
        .unwrap();
    }

    pub fn paths(&self, include_dot_git: bool) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let mut queue = collections::VecDeque::new();
        queue.push_back((
            PathBuf::from(util::path!("/")),
            self.state.lock().root.clone(),
        ));
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
        queue.push_back((
            PathBuf::from(util::path!("/")),
            self.state.lock().root.clone(),
        ));
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
        queue.push_back((
            PathBuf::from(util::path!("/")),
            self.state.lock().root.clone(),
        ));
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

    pub fn files_with_contents(&self, prefix: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut result = Vec::new();
        let mut queue = collections::VecDeque::new();
        queue.push_back((
            PathBuf::from(util::path!("/")),
            self.state.lock().root.clone(),
        ));
        while let Some((path, entry)) = queue.pop_front() {
            let e = entry.lock();
            match &*e {
                FakeFsEntry::File { content, .. } => {
                    if path.starts_with(prefix) {
                        result.push((path, content.clone()));
                    }
                }
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

    pub fn watched_paths(&self) -> Vec<PathBuf> {
        let state = self.state.lock();
        state
            .event_txs
            .iter()
            .filter_map(|(path, tx)| Some(path.clone()).filter(|_| !tx.is_closed()))
            .collect()
    }

    /// How many `metadata` calls have been issued.
    pub fn metadata_call_count(&self) -> usize {
        self.state.lock().metadata_call_count
    }

    fn simulate_random_delay(&self) -> impl futures::Future<Output = ()> {
        self.executor.simulate_random_delay()
    }

    pub fn set_home_dir(&self, home_dir: PathBuf) {
        self.state.lock().home_dir = Some(home_dir);
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
struct FakeWatcher {
    tx: smol::channel::Sender<Vec<PathEvent>>,
    original_path: PathBuf,
    fs_state: Arc<Mutex<FakeFsState>>,
    prefixes: Mutex<Vec<PathBuf>>,
}

#[cfg(any(test, feature = "test-support"))]
impl Watcher for FakeWatcher {
    fn add(&self, path: &Path) -> Result<()> {
        if path.starts_with(&self.original_path) {
            return Ok(());
        }
        self.fs_state
            .try_lock()
            .unwrap()
            .event_txs
            .push((path.to_owned(), self.tx.clone()));
        self.prefixes.lock().push(path.to_owned());
        Ok(())
    }

    fn remove(&self, _: &Path) -> Result<()> {
        Ok(())
    }
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug)]
struct FakeHandle {
    inode: u64,
}

#[cfg(any(test, feature = "test-support"))]
impl FileHandle for FakeHandle {
    fn current_path(&self, fs: &Arc<dyn Fs>) -> Result<PathBuf> {
        let fs = fs.as_fake();
        let state = fs.state.lock();
        let Some(target) = state.moves.get(&self.inode) else {
            anyhow::bail!("fake fd not moved")
        };

        if state.try_read_path(&target, false).is_some() {
            return Ok(target.clone());
        }
        anyhow::bail!("fake fd target not found")
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

            let inode = state.get_and_increment_inode();
            let mtime = state.get_and_increment_mtime();
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
        let inode = state.get_and_increment_inode();
        let mtime = state.get_and_increment_mtime();
        let file = Arc::new(Mutex::new(FakeFsEntry::File {
            inode,
            mtime,
            len: 0,
            content: Vec::new(),
            git_dir_path: None,
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
        self.write_file_internal(path, bytes, true)?;
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
                self.write_file_internal(&path, bytes, true)?;
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

        let inode = match *moved_entry.lock() {
            FakeFsEntry::File { inode, .. } => inode,
            FakeFsEntry::Dir { inode, .. } => inode,
            _ => 0,
        };

        state.moves.insert(inode, new_path.clone());

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
        let mtime = state.get_and_increment_mtime();
        let inode = state.get_and_increment_inode();
        let source_entry = state.read_path(&source)?;
        let content = source_entry.lock().file_content(&source)?.clone();
        let mut kind = Some(PathEventKind::Created);
        state.write_path(&target, |e| match e {
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
                    content,
                    git_dir_path: None,
                })))
                .clone(),
            )),
        })?;
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

    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read + Send + Sync>> {
        let bytes = self.load_internal(path).await?;
        Ok(Box::new(io::Cursor::new(bytes)))
    }

    async fn open_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>> {
        self.simulate_random_delay().await;
        let state = self.state.lock();
        let entry = state.read_path(&path)?;
        let entry = entry.lock();
        let inode = match *entry {
            FakeFsEntry::File { inode, .. } => inode,
            FakeFsEntry::Dir { inode, .. } => inode,
            _ => unreachable!(),
        };
        Ok(Arc::new(FakeHandle { inode }))
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
        self.write_file_internal(path, data.into_bytes(), true)?;
        Ok(())
    }

    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        let content = chunks(text, line_ending).collect::<String>();
        if let Some(path) = path.parent() {
            self.create_dir(path).await?;
        }
        self.write_file_internal(path, content.into_bytes(), false)?;
        Ok(())
    }

    async fn write(&self, path: &Path, content: &[u8]) -> Result<()> {
        self.simulate_random_delay().await;
        let path = normalize_path(path);
        if let Some(path) = path.parent() {
            self.create_dir(path).await?;
        }
        self.write_file_internal(path, content.to_vec(), false)?;
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
        let path = path.to_path_buf();
        self.state.lock().event_txs.push((path.clone(), tx.clone()));
        let executor = self.executor.clone();
        let watcher = Arc::new(FakeWatcher {
            tx,
            original_path: path.to_owned(),
            fs_state: self.state.clone(),
            prefixes: Mutex::new(vec![path.to_owned()]),
        });
        (
            Box::pin(futures::StreamExt::filter(rx, {
                let watcher = watcher.clone();
                move |events| {
                    let result = events.iter().any(|evt_path| {
                        let result = watcher
                            .prefixes
                            .lock()
                            .iter()
                            .any(|prefix| evt_path.path.starts_with(prefix));
                        result
                    });
                    let executor = executor.clone();
                    async move {
                        executor.simulate_random_delay().await;
                        result
                    }
                }
            })),
            watcher,
        )
    }

    fn open_repo(&self, abs_dot_git: &Path) -> Option<Arc<dyn GitRepository>> {
        use util::ResultExt as _;

        self.with_git_state_and_paths(
            abs_dot_git,
            false,
            |_, repository_dir_path, common_dir_path| {
                Arc::new(fake_git_repo::FakeGitRepository {
                    fs: self.this.upgrade().unwrap(),
                    executor: self.executor.clone(),
                    dot_git_path: abs_dot_git.to_path_buf(),
                    repository_dir_path: repository_dir_path.to_owned(),
                    common_dir_path: common_dir_path.to_owned(),
                }) as _
            },
        )
        .log_err()
    }

    fn git_init(
        &self,
        abs_work_directory_path: &Path,
        _fallback_branch_name: String,
    ) -> Result<()> {
        smol::block_on(self.create_dir(&abs_work_directory_path.join(".git")))
    }

    fn is_fake(&self) -> bool {
        true
    }

    async fn is_case_sensitive(&self) -> Result<bool> {
        Ok(true)
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> Arc<FakeFs> {
        self.this.upgrade().unwrap()
    }

    fn home_dir(&self) -> Option<PathBuf> {
        self.state.lock().home_dir.clone()
    }
}

fn chunks(rope: &Rope, line_ending: LineEnding) -> impl Iterator<Item = &str> {
    rope.chunks().flat_map(move |chunk| {
        let mut newline = false;
        chunk.lines().flat_map(move |line| {
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

pub async fn copy_recursive<'a>(
    fs: &'a dyn Fs,
    source: &'a Path,
    target: &'a Path,
    options: CopyOptions,
) -> Result<()> {
    for (item, is_dir) in read_dir_items(fs, source).await? {
        let Ok(item_relative_path) = item.strip_prefix(source) else {
            continue;
        };
        let target_item = if item_relative_path == Path::new("") {
            target.to_path_buf()
        } else {
            target.join(item_relative_path)
        };
        if is_dir {
            if !options.overwrite && fs.metadata(&target_item).await.is_ok_and(|m| m.is_some()) {
                if options.ignore_if_exists {
                    continue;
                } else {
                    return Err(anyhow!("{target_item:?} already exists"));
                }
            }
            let _ = fs
                .remove_dir(
                    &target_item,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await;
            fs.create_dir(&target_item).await?;
        } else {
            fs.copy_file(&item, &target_item, options).await?;
        }
    }
    Ok(())
}

/// Recursively reads all of the paths in the given directory.
///
/// Returns a vector of tuples of (path, is_dir).
pub async fn read_dir_items<'a>(fs: &'a dyn Fs, source: &'a Path) -> Result<Vec<(PathBuf, bool)>> {
    let mut items = Vec::new();
    read_recursive(fs, source, &mut items).await?;
    Ok(items)
}

fn read_recursive<'a>(
    fs: &'a dyn Fs,
    source: &'a Path,
    output: &'a mut Vec<(PathBuf, bool)>,
) -> BoxFuture<'a, Result<()>> {
    use futures::future::FutureExt;

    async move {
        let metadata = fs
            .metadata(source)
            .await?
            .ok_or_else(|| anyhow!("path does not exist: {}", source.display()))?;

        if metadata.is_dir {
            output.push((source.to_path_buf(), true));
            let mut children = fs.read_dir(source).await?;
            while let Some(child_path) = children.next().await {
                if let Ok(child_path) = child_path {
                    read_recursive(fs, &child_path, output).await?;
                }
            }
        } else {
            output.push((source.to_path_buf(), false));
        }
        Ok(())
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
            BY_HANDLE_FILE_INFORMATION, FILE_FLAG_BACKUP_SEMANTICS, GetFileInformationByHandle,
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

#[cfg(target_os = "windows")]
fn atomic_replace<P: AsRef<Path>>(
    replaced_file: P,
    replacement_file: P,
) -> windows::core::Result<()> {
    use windows::{
        Win32::Storage::FileSystem::{REPLACE_FILE_FLAGS, ReplaceFileW},
        core::HSTRING,
    };

    // If the file does not exist, create it.
    let _ = std::fs::File::create_new(replaced_file.as_ref());

    unsafe {
        ReplaceFileW(
            &HSTRING::from(replaced_file.as_ref().to_string_lossy().to_string()),
            &HSTRING::from(replacement_file.as_ref().to_string_lossy().to_string()),
            None,
            REPLACE_FILE_FLAGS::default(),
            None,
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::BackgroundExecutor;
    use serde_json::json;
    use util::path;

    #[gpui::test]
    async fn test_fake_fs(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(
            path!("/root"),
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
                PathBuf::from(path!("/root/dir1/a")),
                PathBuf::from(path!("/root/dir1/b")),
                PathBuf::from(path!("/root/dir2/c")),
                PathBuf::from(path!("/root/dir2/dir3/d")),
            ]
        );

        fs.create_symlink(path!("/root/dir2/link-to-dir3").as_ref(), "./dir3".into())
            .await
            .unwrap();

        assert_eq!(
            fs.canonicalize(path!("/root/dir2/link-to-dir3").as_ref())
                .await
                .unwrap(),
            PathBuf::from(path!("/root/dir2/dir3")),
        );
        assert_eq!(
            fs.canonicalize(path!("/root/dir2/link-to-dir3/d").as_ref())
                .await
                .unwrap(),
            PathBuf::from(path!("/root/dir2/dir3/d")),
        );
        assert_eq!(
            fs.load(path!("/root/dir2/link-to-dir3/d").as_ref())
                .await
                .unwrap(),
            "D",
        );
    }

    #[gpui::test]
    async fn test_copy_recursive_with_single_file(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(
            path!("/outer"),
            json!({
                "a": "A",
                "b": "B",
                "inner": {}
            }),
        )
        .await;

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/a")),
                PathBuf::from(path!("/outer/b")),
            ]
        );

        let source = Path::new(path!("/outer/a"));
        let target = Path::new(path!("/outer/a copy"));
        copy_recursive(fs.as_ref(), source, target, Default::default())
            .await
            .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/a")),
                PathBuf::from(path!("/outer/a copy")),
                PathBuf::from(path!("/outer/b")),
            ]
        );

        let source = Path::new(path!("/outer/a"));
        let target = Path::new(path!("/outer/inner/a copy"));
        copy_recursive(fs.as_ref(), source, target, Default::default())
            .await
            .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/a")),
                PathBuf::from(path!("/outer/a copy")),
                PathBuf::from(path!("/outer/b")),
                PathBuf::from(path!("/outer/inner/a copy")),
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_recursive_with_single_dir(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(
            path!("/outer"),
            json!({
                "a": "A",
                "empty": {},
                "non-empty": {
                    "b": "B",
                }
            }),
        )
        .await;

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/a")),
                PathBuf::from(path!("/outer/non-empty/b")),
            ]
        );
        assert_eq!(
            fs.directories(false),
            vec![
                PathBuf::from(path!("/")),
                PathBuf::from(path!("/outer")),
                PathBuf::from(path!("/outer/empty")),
                PathBuf::from(path!("/outer/non-empty")),
            ]
        );

        let source = Path::new(path!("/outer/empty"));
        let target = Path::new(path!("/outer/empty copy"));
        copy_recursive(fs.as_ref(), source, target, Default::default())
            .await
            .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/a")),
                PathBuf::from(path!("/outer/non-empty/b")),
            ]
        );
        assert_eq!(
            fs.directories(false),
            vec![
                PathBuf::from(path!("/")),
                PathBuf::from(path!("/outer")),
                PathBuf::from(path!("/outer/empty")),
                PathBuf::from(path!("/outer/empty copy")),
                PathBuf::from(path!("/outer/non-empty")),
            ]
        );

        let source = Path::new(path!("/outer/non-empty"));
        let target = Path::new(path!("/outer/non-empty copy"));
        copy_recursive(fs.as_ref(), source, target, Default::default())
            .await
            .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/a")),
                PathBuf::from(path!("/outer/non-empty/b")),
                PathBuf::from(path!("/outer/non-empty copy/b")),
            ]
        );
        assert_eq!(
            fs.directories(false),
            vec![
                PathBuf::from(path!("/")),
                PathBuf::from(path!("/outer")),
                PathBuf::from(path!("/outer/empty")),
                PathBuf::from(path!("/outer/empty copy")),
                PathBuf::from(path!("/outer/non-empty")),
                PathBuf::from(path!("/outer/non-empty copy")),
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_recursive(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(
            path!("/outer"),
            json!({
                "inner1": {
                    "a": "A",
                    "b": "B",
                    "inner3": {
                        "d": "D",
                    },
                    "inner4": {}
                },
                "inner2": {
                    "c": "C",
                }
            }),
        )
        .await;

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/inner3/d")),
            ]
        );
        assert_eq!(
            fs.directories(false),
            vec![
                PathBuf::from(path!("/")),
                PathBuf::from(path!("/outer")),
                PathBuf::from(path!("/outer/inner1")),
                PathBuf::from(path!("/outer/inner2")),
                PathBuf::from(path!("/outer/inner1/inner3")),
                PathBuf::from(path!("/outer/inner1/inner4")),
            ]
        );

        let source = Path::new(path!("/outer"));
        let target = Path::new(path!("/outer/inner1/outer"));
        copy_recursive(fs.as_ref(), source, target, Default::default())
            .await
            .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/inner3/d")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner1/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/inner3/d")),
            ]
        );
        assert_eq!(
            fs.directories(false),
            vec![
                PathBuf::from(path!("/")),
                PathBuf::from(path!("/outer")),
                PathBuf::from(path!("/outer/inner1")),
                PathBuf::from(path!("/outer/inner2")),
                PathBuf::from(path!("/outer/inner1/inner3")),
                PathBuf::from(path!("/outer/inner1/inner4")),
                PathBuf::from(path!("/outer/inner1/outer")),
                PathBuf::from(path!("/outer/inner1/outer/inner1")),
                PathBuf::from(path!("/outer/inner1/outer/inner2")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/inner3")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/inner4")),
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_recursive_with_overwriting(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(
            path!("/outer"),
            json!({
                "inner1": {
                    "a": "A",
                    "b": "B",
                    "outer": {
                        "inner1": {
                            "a": "B"
                        }
                    }
                },
                "inner2": {
                    "c": "C",
                }
            }),
        )
        .await;

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/a")),
            ]
        );
        assert_eq!(
            fs.load(path!("/outer/inner1/outer/inner1/a").as_ref())
                .await
                .unwrap(),
            "B",
        );

        let source = Path::new(path!("/outer"));
        let target = Path::new(path!("/outer/inner1/outer"));
        copy_recursive(
            fs.as_ref(),
            source,
            target,
            CopyOptions {
                overwrite: true,
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner1/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/outer/inner1/a")),
            ]
        );
        assert_eq!(
            fs.load(path!("/outer/inner1/outer/inner1/a").as_ref())
                .await
                .unwrap(),
            "A"
        );
    }

    #[gpui::test]
    async fn test_copy_recursive_with_ignoring(executor: BackgroundExecutor) {
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(
            path!("/outer"),
            json!({
                "inner1": {
                    "a": "A",
                    "b": "B",
                    "outer": {
                        "inner1": {
                            "a": "B"
                        }
                    }
                },
                "inner2": {
                    "c": "C",
                }
            }),
        )
        .await;

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/a")),
            ]
        );
        assert_eq!(
            fs.load(path!("/outer/inner1/outer/inner1/a").as_ref())
                .await
                .unwrap(),
            "B",
        );

        let source = Path::new(path!("/outer"));
        let target = Path::new(path!("/outer/inner1/outer"));
        copy_recursive(
            fs.as_ref(),
            source,
            target,
            CopyOptions {
                ignore_if_exists: true,
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(
            fs.files(),
            vec![
                PathBuf::from(path!("/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/a")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/b")),
                PathBuf::from(path!("/outer/inner1/outer/inner2/c")),
                PathBuf::from(path!("/outer/inner1/outer/inner1/outer/inner1/a")),
            ]
        );
        assert_eq!(
            fs.load(path!("/outer/inner1/outer/inner1/a").as_ref())
                .await
                .unwrap(),
            "B"
        );
    }

    #[gpui::test]
    async fn test_realfs_atomic_write(executor: BackgroundExecutor) {
        // With the file handle still open, the file should be replaced
        // https://github.com/zed-industries/zed/issues/30054
        let fs = RealFs {
            git_binary_path: None,
            executor,
        };
        let temp_dir = TempDir::new().unwrap();
        let file_to_be_replaced = temp_dir.path().join("file.txt");
        let mut file = std::fs::File::create_new(&file_to_be_replaced).unwrap();
        file.write_all(b"Hello").unwrap();
        // drop(file);  // We still hold the file handle here
        let content = std::fs::read_to_string(&file_to_be_replaced).unwrap();
        assert_eq!(content, "Hello");
        smol::block_on(fs.atomic_write(file_to_be_replaced.clone(), "World".into())).unwrap();
        let content = std::fs::read_to_string(&file_to_be_replaced).unwrap();
        assert_eq!(content, "World");
    }

    #[gpui::test]
    async fn test_realfs_atomic_write_non_existing_file(executor: BackgroundExecutor) {
        let fs = RealFs {
            git_binary_path: None,
            executor,
        };
        let temp_dir = TempDir::new().unwrap();
        let file_to_be_replaced = temp_dir.path().join("file.txt");
        smol::block_on(fs.atomic_write(file_to_be_replaced.clone(), "Hello".into())).unwrap();
        let content = std::fs::read_to_string(&file_to_be_replaced).unwrap();
        assert_eq!(content, "Hello");
    }
}
