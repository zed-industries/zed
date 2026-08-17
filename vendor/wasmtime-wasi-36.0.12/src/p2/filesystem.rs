use crate::p2::bindings::filesystem::types;
use crate::p2::{InputStream, OutputStream, Pollable, StreamError, StreamResult};
use crate::runtime::{AbortOnDropJoinHandle, spawn_blocking};
use crate::{DirPerms, FilePerms, OpenMode, TrappableError};
use anyhow::anyhow;
use bytes::{Bytes, BytesMut};
use std::io;
use std::mem;
use std::sync::Arc;

pub type FsResult<T> = Result<T, FsError>;

pub type FsError = TrappableError<types::ErrorCode>;

impl From<wasmtime::component::ResourceTableError> for FsError {
    fn from(error: wasmtime::component::ResourceTableError) -> Self {
        Self::trap(error)
    }
}

impl From<io::Error> for FsError {
    fn from(error: io::Error) -> Self {
        types::ErrorCode::from(error).into()
    }
}

pub enum Descriptor {
    File(File),
    Dir(Dir),
}

impl Descriptor {
    pub fn file(&self) -> Result<&File, types::ErrorCode> {
        match self {
            Descriptor::File(f) => Ok(f),
            Descriptor::Dir(_) => Err(types::ErrorCode::BadDescriptor),
        }
    }

    pub fn dir(&self) -> Result<&Dir, types::ErrorCode> {
        match self {
            Descriptor::Dir(d) => Ok(d),
            Descriptor::File(_) => Err(types::ErrorCode::NotDirectory),
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Descriptor::File(_) => true,
            Descriptor::Dir(_) => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Descriptor::File(_) => false,
            Descriptor::Dir(_) => true,
        }
    }
}

#[derive(Clone)]
pub struct File {
    /// The operating system File this struct is mediating access to.
    ///
    /// Wrapped in an Arc because the same underlying file is used for
    /// implementing the stream types. A copy is also needed for
    /// [`spawn_blocking`].
    ///
    /// [`spawn_blocking`]: Self::spawn_blocking
    pub file: Arc<cap_std::fs::File>,
    /// Permissions to enforce on access to the file. These permissions are
    /// specified by a user of the `crate::p2::WasiCtxBuilder`, and are
    /// enforced prior to any enforced by the underlying operating system.
    pub perms: FilePerms,
    /// The mode the file was opened under: bits for reading, and writing.
    /// Required to correctly report the DescriptorFlags, because cap-std
    /// doesn't presently provide a cross-platform equivalent of reading the
    /// oflags back out using fcntl.
    pub open_mode: OpenMode,

    allow_blocking_current_thread: bool,
}

impl File {
    pub fn new(
        file: cap_std::fs::File,
        perms: FilePerms,
        open_mode: OpenMode,
        allow_blocking_current_thread: bool,
    ) -> Self {
        Self {
            file: Arc::new(file),
            perms,
            open_mode,
            allow_blocking_current_thread,
        }
    }

    /// Execute the blocking `body` function.
    ///
    /// Depending on how the WasiCtx was configured, the body may either be:
    /// - Executed directly on the current thread. In this case the `async`
    ///   signature of this method is effectively a lie and the returned
    ///   Future will always be immediately Ready. Or:
    /// - Spawned on a background thread using [`tokio::task::spawn_blocking`]
    ///   and immediately awaited.
    ///
    /// Intentionally blocking the executor thread might seem unorthodox, but is
    /// not actually a problem for specific workloads. See:
    /// - [`crate::p2::WasiCtxBuilder::allow_blocking_current_thread`]
    /// - [Poor performance of wasmtime file I/O maybe because tokio](https://github.com/bytecodealliance/wasmtime/issues/7973)
    /// - [Implement opt-in for enabling WASI to block the current thread](https://github.com/bytecodealliance/wasmtime/pull/8190)
    pub(crate) async fn run_blocking<F, R>(&self, body: F) -> R
    where
        F: FnOnce(&cap_std::fs::File) -> R + Send + 'static,
        R: Send + 'static,
    {
        match self.as_blocking_file() {
            Some(file) => body(file),
            None => self.spawn_blocking(body).await,
        }
    }

    pub(crate) fn spawn_blocking<F, R>(&self, body: F) -> AbortOnDropJoinHandle<R>
    where
        F: FnOnce(&cap_std::fs::File) -> R + Send + 'static,
        R: Send + 'static,
    {
        let f = self.file.clone();
        spawn_blocking(move || body(&f))
    }

    /// Returns `Some` when the current thread is allowed to block in filesystem
    /// operations, and otherwise returns `None` to indicate that
    /// `spawn_blocking` must be used.
    pub(crate) fn as_blocking_file(&self) -> Option<&cap_std::fs::File> {
        if self.allow_blocking_current_thread {
            Some(&self.file)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dir {
    /// The operating system file descriptor this struct is mediating access
    /// to.
    ///
    /// Wrapped in an Arc because a copy is needed for [`spawn_blocking`].
    ///
    /// [`spawn_blocking`]: Self::spawn_blocking
    pub dir: Arc<cap_std::fs::Dir>,
    /// Permissions to enforce on access to this directory. These permissions
    /// are specified by a user of the `crate::p2::WasiCtxBuilder`, and
    /// are enforced prior to any enforced by the underlying operating system.
    ///
    /// These permissions are also enforced on any directories opened under
    /// this directory.
    pub perms: DirPerms,
    /// Permissions to enforce on any files opened under this directory.
    pub file_perms: FilePerms,
    /// The mode the directory was opened under: bits for reading, and writing.
    /// Required to correctly report the DescriptorFlags, because cap-std
    /// doesn't presently provide a cross-platform equivalent of reading the
    /// oflags back out using fcntl.
    pub open_mode: OpenMode,

    allow_blocking_current_thread: bool,
}

impl Dir {
    pub fn new(
        dir: cap_std::fs::Dir,
        perms: DirPerms,
        file_perms: FilePerms,
        open_mode: OpenMode,
        allow_blocking_current_thread: bool,
    ) -> Self {
        Dir {
            dir: Arc::new(dir),
            perms,
            file_perms,
            open_mode,
            allow_blocking_current_thread,
        }
    }

    /// Execute the blocking `body` function.
    ///
    /// Depending on how the WasiCtx was configured, the body may either be:
    /// - Executed directly on the current thread. In this case the `async`
    ///   signature of this method is effectively a lie and the returned
    ///   Future will always be immediately Ready. Or:
    /// - Spawned on a background thread using [`tokio::task::spawn_blocking`]
    ///   and immediately awaited.
    ///
    /// Intentionally blocking the executor thread might seem unorthodox, but is
    /// not actually a problem for specific workloads. See:
    /// - [`crate::p2::WasiCtxBuilder::allow_blocking_current_thread`]
    /// - [Poor performance of wasmtime file I/O maybe because tokio](https://github.com/bytecodealliance/wasmtime/issues/7973)
    /// - [Implement opt-in for enabling WASI to block the current thread](https://github.com/bytecodealliance/wasmtime/pull/8190)
    pub(crate) async fn run_blocking<F, R>(&self, body: F) -> R
    where
        F: FnOnce(&cap_std::fs::Dir) -> R + Send + 'static,
        R: Send + 'static,
    {
        if self.allow_blocking_current_thread {
            body(&self.dir)
        } else {
            let d = self.dir.clone();
            spawn_blocking(move || body(&d)).await
        }
    }
}

pub struct FileInputStream {
    file: File,
    position: u64,
    state: ReadState,
}
enum ReadState {
    Idle,
    Waiting(AbortOnDropJoinHandle<ReadState>),
    DataAvailable(Bytes),
    Error(io::Error),
    Closed,
}
impl FileInputStream {
    pub fn new(file: &File, position: u64) -> Self {
        Self {
            file: file.clone(),
            position,
            state: ReadState::Idle,
        }
    }

    fn blocking_read(file: &cap_std::fs::File, offset: u64, size: usize) -> ReadState {
        use system_interface::fs::FileIoExt;

        let mut buf = BytesMut::zeroed(size.min(crate::MAX_READ_SIZE_ALLOC));
        loop {
            match file.read_at(&mut buf, offset) {
                Ok(0) => return ReadState::Closed,
                Ok(n) => {
                    buf.truncate(n);
                    return ReadState::DataAvailable(buf.freeze());
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {
                    // Try again, continue looping
                }
                Err(e) => return ReadState::Error(e),
            }
        }
    }

    /// Wait for existing background task to finish, without starting any new background reads.
    async fn wait_ready(&mut self) {
        match &mut self.state {
            ReadState::Waiting(task) => {
                self.state = task.await;
            }
            _ => {}
        }
    }
}
#[async_trait::async_trait]
impl InputStream for FileInputStream {
    fn read(&mut self, size: usize) -> StreamResult<Bytes> {
        match &mut self.state {
            ReadState::Idle => {
                if size == 0 {
                    return Ok(Bytes::new());
                }

                let p = self.position;
                self.state = ReadState::Waiting(
                    self.file
                        .spawn_blocking(move |f| Self::blocking_read(f, p, size)),
                );
                Ok(Bytes::new())
            }
            ReadState::DataAvailable(b) => {
                let min_len = b.len().min(size);
                let chunk = b.split_to(min_len);
                if b.len() == 0 {
                    self.state = ReadState::Idle;
                }
                self.position += min_len as u64;
                Ok(chunk)
            }
            ReadState::Waiting(_) => Ok(Bytes::new()),
            ReadState::Error(_) => match mem::replace(&mut self.state, ReadState::Closed) {
                ReadState::Error(e) => Err(StreamError::LastOperationFailed(e.into())),
                _ => unreachable!(),
            },
            ReadState::Closed => Err(StreamError::Closed),
        }
    }
    /// Specialized blocking_* variant to bypass tokio's task spawning & joining
    /// overhead on synchronous file I/O.
    async fn blocking_read(&mut self, size: usize) -> StreamResult<Bytes> {
        self.wait_ready().await;

        // Before we defer to the regular `read`, make sure it has data ready to go:
        if let ReadState::Idle = self.state {
            let p = self.position;
            self.state = self
                .file
                .run_blocking(move |f| Self::blocking_read(f, p, size))
                .await;
        }

        self.read(size)
    }
    async fn cancel(&mut self) {
        match mem::replace(&mut self.state, ReadState::Closed) {
            ReadState::Waiting(task) => {
                // The task was created using `spawn_blocking`, so unless we're
                // lucky enough that the task hasn't started yet, the abort
                // signal won't have any effect and we're forced to wait for it
                // to run to completion.
                // From the guest's point of view, `input-stream::drop` then
                // appears to block. Certainly less than ideal, but arguably still
                // better than letting the guest rack up an unbounded number of
                // background tasks. Also, the guest is only blocked if
                // the stream was dropped mid-read, which we don't expect to
                // occur frequently.
                task.cancel().await;
            }
            _ => {}
        }
    }
}
#[async_trait::async_trait]
impl Pollable for FileInputStream {
    async fn ready(&mut self) {
        if let ReadState::Idle = self.state {
            // The guest hasn't initiated any read, but is nonetheless waiting
            // for data to be available. We'll start a read for them:

            const DEFAULT_READ_SIZE: usize = 4096;
            let p = self.position;
            self.state = ReadState::Waiting(
                self.file
                    .spawn_blocking(move |f| Self::blocking_read(f, p, DEFAULT_READ_SIZE)),
            );
        }

        self.wait_ready().await
    }
}

#[derive(Clone, Copy)]
pub(crate) enum FileOutputMode {
    Position(u64),
    Append,
}

pub(crate) struct FileOutputStream {
    file: File,
    mode: FileOutputMode,
    state: OutputState,
}

enum OutputState {
    Ready,
    /// Allows join future to be awaited in a cancellable manner. Gone variant indicates
    /// no task is currently outstanding.
    Waiting(AbortOnDropJoinHandle<io::Result<usize>>),
    /// The last I/O operation failed with this error.
    Error(io::Error),
    Closed,
}

impl FileOutputStream {
    pub fn write_at(file: &File, position: u64) -> Self {
        Self {
            file: file.clone(),
            mode: FileOutputMode::Position(position),
            state: OutputState::Ready,
        }
    }

    pub fn append(file: &File) -> Self {
        Self {
            file: file.clone(),
            mode: FileOutputMode::Append,
            state: OutputState::Ready,
        }
    }

    fn blocking_write(
        file: &cap_std::fs::File,
        mut buf: Bytes,
        mode: FileOutputMode,
    ) -> io::Result<usize> {
        use system_interface::fs::FileIoExt;

        match mode {
            FileOutputMode::Position(mut p) => {
                let mut total = 0;
                loop {
                    let nwritten = file.write_at(buf.as_ref(), p)?;
                    // afterwards buf contains [nwritten, len):
                    let _ = buf.split_to(nwritten);
                    p += nwritten as u64;
                    total += nwritten;
                    if buf.is_empty() {
                        break;
                    }
                }
                Ok(total)
            }
            FileOutputMode::Append => {
                let mut total = 0;
                loop {
                    let nwritten = file.append(buf.as_ref())?;
                    let _ = buf.split_to(nwritten);
                    total += nwritten;
                    if buf.is_empty() {
                        break;
                    }
                }
                Ok(total)
            }
        }
    }
}

// FIXME: configurable? determine from how much space left in file?
const FILE_WRITE_CAPACITY: usize = 1024 * 1024;

#[async_trait::async_trait]
impl OutputStream for FileOutputStream {
    fn write(&mut self, buf: Bytes) -> Result<(), StreamError> {
        match self.state {
            OutputState::Ready => {}
            OutputState::Closed => return Err(StreamError::Closed),
            OutputState::Waiting(_) | OutputState::Error(_) => {
                // a write is pending - this call was not permitted
                return Err(StreamError::Trap(anyhow!(
                    "write not permitted: check_write not called first"
                )));
            }
        }

        let m = self.mode;
        self.state = OutputState::Waiting(
            self.file
                .spawn_blocking(move |f| Self::blocking_write(f, buf, m)),
        );
        Ok(())
    }
    /// Specialized blocking_* variant to bypass tokio's task spawning & joining
    /// overhead on synchronous file I/O.
    async fn blocking_write_and_flush(&mut self, buf: Bytes) -> StreamResult<()> {
        self.ready().await;

        match self.state {
            OutputState::Ready => {}
            OutputState::Closed => return Err(StreamError::Closed),
            OutputState::Error(_) => match mem::replace(&mut self.state, OutputState::Closed) {
                OutputState::Error(e) => return Err(StreamError::LastOperationFailed(e.into())),
                _ => unreachable!(),
            },
            OutputState::Waiting(_) => unreachable!("we've just waited for readiness"),
        }

        let m = self.mode;
        match self
            .file
            .run_blocking(move |f| Self::blocking_write(f, buf, m))
            .await
        {
            Ok(nwritten) => {
                if let FileOutputMode::Position(p) = &mut self.mode {
                    *p += nwritten as u64;
                }
                self.state = OutputState::Ready;
                Ok(())
            }
            Err(e) => {
                self.state = OutputState::Closed;
                Err(StreamError::LastOperationFailed(e.into()))
            }
        }
    }
    fn flush(&mut self) -> Result<(), StreamError> {
        match self.state {
            // Only userland buffering of file writes is in the blocking task,
            // so there's nothing extra that needs to be done to request a
            // flush.
            OutputState::Ready | OutputState::Waiting(_) => Ok(()),
            OutputState::Closed => Err(StreamError::Closed),
            OutputState::Error(_) => match mem::replace(&mut self.state, OutputState::Closed) {
                OutputState::Error(e) => Err(StreamError::LastOperationFailed(e.into())),
                _ => unreachable!(),
            },
        }
    }
    fn check_write(&mut self) -> Result<usize, StreamError> {
        match self.state {
            OutputState::Ready => Ok(FILE_WRITE_CAPACITY),
            OutputState::Closed => Err(StreamError::Closed),
            OutputState::Error(_) => match mem::replace(&mut self.state, OutputState::Closed) {
                OutputState::Error(e) => Err(StreamError::LastOperationFailed(e.into())),
                _ => unreachable!(),
            },
            OutputState::Waiting(_) => Ok(0),
        }
    }
    async fn cancel(&mut self) {
        match mem::replace(&mut self.state, OutputState::Closed) {
            OutputState::Waiting(task) => {
                // The task was created using `spawn_blocking`, so unless we're
                // lucky enough that the task hasn't started yet, the abort
                // signal won't have any effect and we're forced to wait for it
                // to run to completion.
                // From the guest's point of view, `output-stream::drop` then
                // appears to block. Certainly less than ideal, but arguably still
                // better than letting the guest rack up an unbounded number of
                // background tasks. Also, the guest is only blocked if
                // the stream was dropped mid-write, which we don't expect to
                // occur frequently.
                task.cancel().await;
            }
            _ => {}
        }
    }
}

#[async_trait::async_trait]
impl Pollable for FileOutputStream {
    async fn ready(&mut self) {
        if let OutputState::Waiting(task) = &mut self.state {
            self.state = match task.await {
                Ok(nwritten) => {
                    if let FileOutputMode::Position(p) = &mut self.mode {
                        *p += nwritten as u64;
                    }
                    OutputState::Ready
                }
                Err(e) => OutputState::Error(e),
            };
        }
    }
}

pub struct ReaddirIterator(
    std::sync::Mutex<Box<dyn Iterator<Item = FsResult<types::DirectoryEntry>> + Send + 'static>>,
);

impl ReaddirIterator {
    pub(crate) fn new(
        i: impl Iterator<Item = FsResult<types::DirectoryEntry>> + Send + 'static,
    ) -> Self {
        ReaddirIterator(std::sync::Mutex::new(Box::new(i)))
    }
    pub(crate) fn next(&self) -> FsResult<Option<types::DirectoryEntry>> {
        self.0.lock().unwrap().next().transpose()
    }
}

impl IntoIterator for ReaddirIterator {
    type Item = FsResult<types::DirectoryEntry>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + Send>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_inner().unwrap()
    }
}
