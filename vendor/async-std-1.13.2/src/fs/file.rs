use std::cell::UnsafeCell;
use std::cmp;
use std::fmt;
use std::io::{Read as _, Seek as _, Write as _};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::fs::{Metadata, Permissions};
use crate::future;
use crate::io::{self, Read, Seek, SeekFrom, Write};
use crate::path::Path;
use crate::prelude::*;
use crate::task::{spawn_blocking, Context, Poll, Waker};
use crate::utils::Context as _;

const ARC_TRY_UNWRAP_EXPECT: &str = "cannot acquire ownership of the file handle after drop";

/// An open file on the filesystem.
///
/// Depending on what options the file was opened with, this type can be used for reading and/or
/// writing.
///
/// Files are automatically closed when they get dropped and any errors detected on closing are
/// ignored. Use the [`sync_all`] method before dropping a file if such errors need to be handled.
///
/// This type is an async version of [`std::fs::File`].
///
/// [`sync_all`]: #method.sync_all
/// [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
///
/// # Examples
///
/// Create a new file and write some bytes to it:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs::File;
/// use async_std::prelude::*;
///
/// let mut file = File::create("a.txt").await?;
/// file.write_all(b"Hello, world!").await?;
/// #
/// # Ok(()) }) }
/// ```
///
/// Read the contents of a file into a vector of bytes:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs::File;
/// use async_std::prelude::*;
///
/// let mut file = File::open("a.txt").await?;
/// let mut contents = Vec::new();
/// file.read_to_end(&mut contents).await?;
/// #
/// # Ok(()) }) }
/// ```
#[derive(Clone)]
pub struct File {
    /// A reference to the inner file.
    file: Arc<std::fs::File>,

    /// The state of the file protected by an async lock.
    lock: Lock<State>,
}

impl File {
    /// Creates an async file handle.
    pub(crate) fn new(file: std::fs::File, is_flushed: bool) -> File {
        let file = Arc::new(file);

        File {
            file: file.clone(),
            lock: Lock::new(State {
                file,
                mode: Mode::Idle,
                cache: Vec::new(),
                is_flushed,
                last_read_err: None,
                last_write_err: None,
            }),
        }
    }

    /// Opens a file in read-only mode.
    ///
    /// See the [`OpenOptions::open`] function for more options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * Some other I/O error occurred.
    ///
    /// For more details, see the list of errors documented by [`OpenOptions::open`].
    ///
    /// [`OpenOptions::open`]: struct.OpenOptions.html#method.open
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    ///
    /// let file = File::open("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref().to_owned();
        let file = spawn_blocking(move || {
            std::fs::File::open(&path).context(|| format!("could not open `{}`", path.display()))
        })
        .await?;
        Ok(File::new(file, true))
    }

    /// Opens a file in write-only mode.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    ///
    /// See the [`OpenOptions::open`] function for more options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to write to the file.
    /// * Some other I/O error occurred.
    ///
    /// For more details, see the list of errors documented by [`OpenOptions::open`].
    ///
    /// [`OpenOptions::open`]: struct.OpenOptions.html#method.open
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    ///
    /// let file = File::create("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
        let path = path.as_ref().to_owned();
        let file = spawn_blocking(move || {
            std::fs::File::create(&path)
        })
        .await?;
        Ok(File::new(file, true))
    }

    /// Synchronizes OS-internal buffered contents and metadata to disk.
    ///
    /// This function will ensure that all in-memory data reaches the filesystem.
    ///
    /// This can be used to handle errors that would otherwise only be caught when the file is
    /// closed. When a file is dropped, errors in synchronizing this in-memory data are ignored.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::prelude::*;
    ///
    /// let mut file = File::create("a.txt").await?;
    /// file.write_all(b"Hello, world!").await?;
    /// file.sync_all().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn sync_all(&self) -> io::Result<()> {
        // Flush the write cache before calling `sync_all()`.
        let state = future::poll_fn(|cx| {
            let state = futures_core::ready!(self.lock.poll_lock(cx));
            state.poll_flush(cx)
        })
        .await?;

        spawn_blocking(move || state.file.sync_all()).await
    }

    /// Synchronizes OS-internal buffered contents to disk.
    ///
    /// This is similar to [`sync_all`], except that file metadata may not be synchronized.
    ///
    /// This is intended for use cases that must synchronize the contents of the file, but don't
    /// need the file metadata synchronized to disk.
    ///
    /// Note that some platforms may simply implement this in terms of [`sync_all`].
    ///
    /// [`sync_all`]: #method.sync_all
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::prelude::*;
    ///
    /// let mut file = File::create("a.txt").await?;
    /// file.write_all(b"Hello, world!").await?;
    /// file.sync_data().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn sync_data(&self) -> io::Result<()> {
        // Flush the write cache before calling `sync_data()`.
        let state = future::poll_fn(|cx| {
            let state = futures_core::ready!(self.lock.poll_lock(cx));
            state.poll_flush(cx)
        })
        .await?;

        spawn_blocking(move || state.file.sync_data()).await
    }

    /// Truncates or extends the file.
    ///
    /// If `size` is less than the current file size, then the file will be truncated. If it is
    /// greater than the current file size, then the file will be extended to `size` and have all
    /// intermediate data filled with zeros.
    ///
    /// The file's cursor stays at the same position, even if the cursor ends up being past the end
    /// of the file after this operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    ///
    /// let file = File::create("a.txt").await?;
    /// file.set_len(10).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        // Invalidate the read cache and flush the write cache before calling `set_len()`.
        let state = future::poll_fn(|cx| {
            let state = futures_core::ready!(self.lock.poll_lock(cx));
            let state = futures_core::ready!(state.poll_unread(cx))?;
            state.poll_flush(cx)
        })
        .await?;

        spawn_blocking(move || state.file.set_len(size)).await
    }

    /// Reads the file's metadata.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    ///
    /// let file = File::open("a.txt").await?;
    /// let metadata = file.metadata().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn metadata(&self) -> io::Result<Metadata> {
        let file = self.file.clone();
        spawn_blocking(move || file.metadata()).await
    }

    /// Changes the permissions on the file.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The current process lacks permissions to change attributes on the file.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    ///
    /// let file = File::create("a.txt").await?;
    ///
    /// let mut perms = file.metadata().await?.permissions();
    /// perms.set_readonly(true);
    /// file.set_permissions(perms).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        let file = self.file.clone();
        spawn_blocking(move || file.set_permissions(perm)).await
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // We need to flush the file on drop. Unfortunately, that is not possible to do in a
        // non-blocking fashion, but our only other option here is losing data remaining in the
        // write cache. Good task schedulers should be resilient to occasional blocking hiccups in
        // file destructors so we don't expect this to be a common problem in practice.
        let _ = futures_lite::future::block_on(self.flush());
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.file.fmt(f)
    }
}

impl Read for File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }
}

impl Read for &File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let state = futures_core::ready!(self.lock.poll_lock(cx));
        state.poll_read(cx, buf)
    }
}

impl Write for File {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}

impl Write for &File {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let state = futures_core::ready!(self.lock.poll_lock(cx));
        state.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let state = futures_core::ready!(self.lock.poll_lock(cx));
        state.poll_flush(cx).map(|res| res.map(drop))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let state = futures_core::ready!(self.lock.poll_lock(cx));
        state.poll_close(cx)
    }
}

impl Seek for File {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        Pin::new(&mut &*self).poll_seek(cx, pos)
    }
}

impl Seek for &File {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let state = futures_core::ready!(self.lock.poll_lock(cx));
        state.poll_seek(cx, pos)
    }
}

impl From<std::fs::File> for File {
    fn from(file: std::fs::File) -> File {
        File::new(file, false)
    }
}

cfg_unix! {
    use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

    impl File {
        fn into_std_file(self) -> std::fs::File {
            let file = self.file.clone();
            drop(self);
            Arc::try_unwrap(file)
                .expect(ARC_TRY_UNWRAP_EXPECT)
        }
    }

    impl AsRawFd for File {
        fn as_raw_fd(&self) -> RawFd {
            self.file.as_raw_fd()
        }
    }

    impl FromRawFd for File {
        unsafe fn from_raw_fd(fd: RawFd) -> File {
            std::fs::File::from_raw_fd(fd).into()
        }
    }

    impl IntoRawFd for File {
        fn into_raw_fd(self) -> RawFd {
            self.into_std_file().into_raw_fd()
        }
    }

    cfg_io_safety! {
        use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

        impl AsFd for File {
            fn as_fd(&self) -> BorrowedFd<'_> {
                self.file.as_fd()
            }
        }

        impl From<OwnedFd> for File {
            fn from(fd: OwnedFd) -> Self {
                std::fs::File::from(fd).into()
            }
        }

        impl From<File> for OwnedFd {
            fn from(val: File) -> OwnedFd {
                val.into_std_file().into()
            }
        }
    }
}

cfg_windows! {
    use crate::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};

    impl AsRawHandle for File {
        fn as_raw_handle(&self) -> RawHandle {
            self.file.as_raw_handle()
        }
    }

    impl FromRawHandle for File {
        unsafe fn from_raw_handle(handle: RawHandle) -> File {
            std::fs::File::from_raw_handle(handle).into()
        }
    }

    impl IntoRawHandle for File {
        fn into_raw_handle(self) -> RawHandle {
            let file = self.file.clone();
            drop(self);
            Arc::try_unwrap(file)
                .expect(ARC_TRY_UNWRAP_EXPECT)
                .into_raw_handle()
        }
    }

    cfg_io_safety! {
        use crate::os::windows::io::{AsHandle, BorrowedHandle, OwnedHandle};

        impl AsHandle for File {
            fn as_handle(&self) -> BorrowedHandle<'_> {
                self.file.as_handle()
            }
        }

        impl From<OwnedHandle> for File {
            fn from(handle: OwnedHandle) -> Self {
                std::fs::File::from(handle).into()
            }
        }

        impl From<File> for OwnedHandle {
            fn from(val: File) -> OwnedHandle {
                let file = val.file.clone();
                drop(val);
                Arc::try_unwrap(file)
                    .expect(ARC_TRY_UNWRAP_EXPECT)
                    .into()
            }
        }
    }
}

/// An async mutex with non-borrowing lock guards.
struct Lock<T>(Arc<LockState<T>>);

unsafe impl<T: Send> Send for Lock<T> {}
unsafe impl<T: Send> Sync for Lock<T> {}

impl<T> Clone for Lock<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

/// The state of a lock.
struct LockState<T> {
    /// Set to `true` when locked.
    locked: AtomicBool,

    /// The inner value.
    value: UnsafeCell<T>,

    /// A list of tasks interested in acquiring the lock.
    wakers: Mutex<Vec<Waker>>,
}

impl<T> Lock<T> {
    /// Creates a new lock initialized with `value`.
    fn new(value: T) -> Lock<T> {
        Lock(Arc::new(LockState {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
            wakers: Mutex::new(Vec::new()),
        }))
    }

    /// Attempts to acquire the lock.
    fn poll_lock(&self, cx: &mut Context<'_>) -> Poll<LockGuard<T>> {
        // Try acquiring the lock.
        if self.0.locked.swap(true, Ordering::Acquire) {
            // Lock the list of wakers.
            let mut list = self.0.wakers.lock().unwrap();

            // Try acquiring the lock again.
            if self.0.locked.swap(true, Ordering::Acquire) {
                // If failed again, add the current task to the list and return.
                if list.iter().all(|w| !w.will_wake(cx.waker())) {
                    list.push(cx.waker().clone());
                }
                return Poll::Pending;
            }
        }

        // The lock was successfully acquired.
        Poll::Ready(LockGuard(Some(self.0.clone())))
    }
}

/// A lock guard.
///
/// When dropped, ownership of the inner value is returned back to the lock.
/// The inner value is always Some, except when the lock is dropped, where we
/// set it to None. See comment in drop().
struct LockGuard<T>(Option<Arc<LockState<T>>>);

unsafe impl<T: Send> Send for LockGuard<T> {}
unsafe impl<T: Sync> Sync for LockGuard<T> {}

impl<T> LockGuard<T> {
    /// Registers a task interested in acquiring the lock.
    ///
    /// When this lock guard gets dropped, all registered tasks will be woken up.
    fn register(&self, cx: &Context<'_>) {
        let mut list = self.0.as_ref().unwrap().wakers.lock().unwrap();

        if list.iter().all(|w| !w.will_wake(cx.waker())) {
            list.push(cx.waker().clone());
        }
    }
}

impl<T> Drop for LockGuard<T> {
    fn drop(&mut self) {
        // Set the Option to None and take its value so we can drop the Arc
        // before we wake up the tasks.
        let lock = self.0.take().unwrap();

        // Prepare to wake up all registered tasks interested in acquiring the lock.
        let wakers: Vec<_> = lock.wakers.lock().unwrap().drain(..).collect();

        // Release the lock.
        lock.locked.store(false, Ordering::Release);

        // Drop the Arc _before_ waking up the tasks, to avoid races. See
        // reproducer and discussion in https://github.com/async-rs/async-std/issues/1001.
        drop(lock);

        // Wake up all registered tasks interested in acquiring the lock.
        for w in wakers {
            w.wake();
        }
    }
}

impl<T> Deref for LockGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: Safe because the lock is held when this method is called. And
        // the inner value is always Some since it is only set to None in
        // drop().
        unsafe { &*self.0.as_ref().unwrap().value.get() }
    }
}

impl<T> DerefMut for LockGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: Safe because the lock is held when this method is called. And
        // the inner value is always Some since it is only set to None in
        // drop().
        unsafe { &mut *self.0.as_ref().unwrap().value.get() }
    }
}

/// Modes a file can be in.
///
/// The file can either be in idle mode, reading mode, or writing mode.
enum Mode {
    /// The cache is empty.
    Idle,

    /// The cache contains data read from the inner file.
    ///
    /// The `usize` represents how many bytes from the beginning of cache have been consumed.
    Reading(usize),

    /// The cache contains data that needs to be written to the inner file.
    Writing,
}

/// The current state of a file.
///
/// The `File` struct protects this state behind a lock.
///
/// Filesystem operations that get spawned as blocking tasks will acquire the lock, take ownership
/// of the state and return it back once the operation completes.
struct State {
    /// The inner file.
    file: Arc<std::fs::File>,

    /// The current mode (idle, reading, or writing).
    mode: Mode,

    /// The read/write cache.
    ///
    /// If in reading mode, the cache contains a chunk of data that has been read from the file.
    /// If in writing mode, the cache contains data that will eventually be written to the file.
    cache: Vec<u8>,

    /// Set to `true` if the file is flushed.
    ///
    /// When a file is flushed, the write cache and the inner file's buffer are empty.
    is_flushed: bool,

    /// The last read error that came from an async operation.
    last_read_err: Option<io::Error>,

    /// The last write error that came from an async operation.
    last_write_err: Option<io::Error>,
}

impl LockGuard<State> {
    /// Seeks to a new position in the file.
    fn poll_seek(mut self, cx: &mut Context<'_>, pos: SeekFrom) -> Poll<io::Result<u64>> {
        // If this operation doesn't move the cursor, then poll the current position inside the
        // file. This call should not block because it doesn't touch the actual file on disk.
        if pos == SeekFrom::Current(0) {
            // Poll the internal file cursor.
            let internal = (&*self.file).seek(SeekFrom::Current(0))?;

            // Factor in the difference caused by caching.
            let actual = match self.mode {
                Mode::Idle => internal,
                Mode::Reading(start) => internal - self.cache.len() as u64 + start as u64,
                Mode::Writing => internal + self.cache.len() as u64,
            };
            return Poll::Ready(Ok(actual));
        }

        // If the file is in reading mode and the cache will stay valid after seeking, then adjust
        // the current position in the read cache without invaliding it.
        if let Mode::Reading(start) = self.mode {
            if let SeekFrom::Current(diff) = pos {
                if let Some(new) = (start as i64).checked_add(diff) {
                    if 0 <= new && new <= self.cache.len() as i64 {
                        // Poll the internal file cursor.
                        let internal = (&*self.file).seek(SeekFrom::Current(0))?;

                        // Adjust the current position in the read cache.
                        self.mode = Mode::Reading(new as usize);

                        // Factor in the difference caused by caching.
                        return Poll::Ready(Ok(internal - self.cache.len() as u64 + new as u64));
                    }
                }
            }
        }

        // Invalidate the read cache and flush the write cache before calling `seek()`.
        self = futures_core::ready!(self.poll_unread(cx))?;
        self = futures_core::ready!(self.poll_flush(cx))?;

        // Seek to the new position. This call should not block because it only changes the
        // internal offset into the file and doesn't touch the actual file on disk.
        Poll::Ready((&*self.file).seek(pos))
    }

    /// Reads some bytes from the file into a buffer.
    fn poll_read(mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        // If an async operation has left a read error, return it now.
        if let Some(err) = self.last_read_err.take() {
            return Poll::Ready(Err(err));
        }

        match self.mode {
            Mode::Idle => {}
            Mode::Reading(0) if self.cache.is_empty() => {
                // If the cache is empty in reading mode, the last operation didn't read any bytes,
                // which indicates that it reached the end of the file. In this case we need to
                // reset the mode to idle so that next time we try to read again, since the file
                // may grow after the first EOF.
                self.mode = Mode::Idle;
                return Poll::Ready(Ok(0));
            }
            Mode::Reading(start) => {
                // How many bytes in the cache are available for reading.
                let available = self.cache.len() - start;

                if available > 0 {
                    // Copy data from the cache into the buffer.
                    let n = cmp::min(available, buf.len());
                    buf[..n].copy_from_slice(&self.cache[start..(start + n)]);

                    // Move the read cursor forward.
                    self.mode = Mode::Reading(start + n);

                    return Poll::Ready(Ok(n));
                }
            }
            Mode::Writing => {
                // If we're in writing mode, flush the write cache.
                self = futures_core::ready!(self.poll_flush(cx))?;
            }
        }

        // Make the cache as long as `buf`.
        if self.cache.len() < buf.len() {
            let diff = buf.len() - self.cache.len();
            self.cache.reserve(diff);
        }
        unsafe {
            self.cache.set_len(buf.len());
        }

        // Register current task's interest in the file lock.
        self.register(cx);

        // Start a read operation asynchronously.
        spawn_blocking(move || {
            // Read some data from the file into the cache.
            let res = {
                let State { file, cache, .. } = &mut *self;
                (&**file).read(cache)
            };

            match res {
                Ok(n) => {
                    // Update cache length and switch to reading mode, starting from index 0.
                    unsafe {
                        self.cache.set_len(n);
                    }
                    self.mode = Mode::Reading(0);
                }
                Err(err) => {
                    // Save the error and switch to idle mode.
                    self.cache.clear();
                    self.mode = Mode::Idle;
                    self.last_read_err = Some(err);
                }
            }
        });

        Poll::Pending
    }

    /// Invalidates the read cache.
    ///
    /// This method will also move the internal file's cursor backwards by the number of unconsumed
    /// bytes in the read cache.
    fn poll_unread(mut self, _: &mut Context<'_>) -> Poll<io::Result<Self>> {
        match self.mode {
            Mode::Idle | Mode::Writing => Poll::Ready(Ok(self)),
            Mode::Reading(start) => {
                // The number of unconsumed bytes in the read cache.
                let n = self.cache.len() - start;

                if n > 0 {
                    // Seek `n` bytes backwards. This call should not block because it only changes
                    // the internal offset into the file and doesn't touch the actual file on disk.
                    //
                    // We ignore errors here because special files like `/dev/random` are not
                    // seekable.
                    let _ = (&*self.file).seek(SeekFrom::Current(-(n as i64)));
                }

                // Switch to idle mode.
                self.cache.clear();
                self.mode = Mode::Idle;

                Poll::Ready(Ok(self))
            }
        }
    }

    /// Writes some data from a buffer into the file.
    fn poll_write(mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        // If an async operation has left a write error, return it now.
        if let Some(err) = self.last_write_err.take() {
            return Poll::Ready(Err(err));
        }

        // If we're in reading mode, invalidate the read buffer.
        self = futures_core::ready!(self.poll_unread(cx))?;

        // If necessary, grow the cache to have as much capacity as `buf`.
        if self.cache.capacity() < buf.len() {
            let diff = buf.len() - self.cache.capacity();
            self.cache.reserve(diff);
        }

        // How many bytes can be written into the cache before filling up.
        let available = self.cache.capacity() - self.cache.len();

        // If there is space available in the cache or if the buffer is empty, we can write data
        // into the cache.
        if available > 0 || buf.is_empty() {
            let n = cmp::min(available, buf.len());
            let start = self.cache.len();

            // Copy data from the buffer into the cache.
            unsafe {
                self.cache.set_len(start + n);
            }
            self.cache[start..start + n].copy_from_slice(&buf[..n]);

            // Mark the file as not flushed and switch to writing mode.
            self.is_flushed = false;
            self.mode = Mode::Writing;
            Poll::Ready(Ok(n))
        } else {
            // Drain the write cache because it's full.
            futures_core::ready!(self.poll_drain(cx))?;
            Poll::Pending
        }
    }

    /// Drains the write cache.
    fn poll_drain(mut self, cx: &mut Context<'_>) -> Poll<io::Result<Self>> {
        // If an async operation has left a write error, return it now.
        if let Some(err) = self.last_write_err.take() {
            return Poll::Ready(Err(err));
        }

        match self.mode {
            Mode::Idle | Mode::Reading(..) => Poll::Ready(Ok(self)),
            Mode::Writing => {
                // Register current task's interest in the file lock.
                self.register(cx);

                // Start a write operation asynchronously.
                spawn_blocking(move || {
                    match (&*self.file).write_all(&self.cache) {
                        Ok(_) => {
                            // Switch to idle mode.
                            self.cache.clear();
                            self.mode = Mode::Idle;
                        }
                        Err(err) => {
                            // Save the error.
                            self.last_write_err = Some(err);
                        }
                    };
                });

                Poll::Pending
            }
        }
    }

    /// Flushes the write cache into the file.
    fn poll_flush(mut self, cx: &mut Context<'_>) -> Poll<io::Result<Self>> {
        // If the file is already in flushed state, return.
        if self.is_flushed {
            return Poll::Ready(Ok(self));
        }

        // If there is data in the write cache, drain it.
        self = futures_core::ready!(self.poll_drain(cx))?;

        // Register current task's interest in the file lock.
        self.register(cx);

        // Start a flush operation asynchronously.
        spawn_blocking(move || {
            match (&*self.file).flush() {
                Ok(()) => {
                    // Mark the file as flushed.
                    self.is_flushed = true;
                }
                Err(err) => {
                    // Save the error.
                    self.last_write_err = Some(err);
                }
            }
        });

        Poll::Pending
    }

    // This function does nothing because we're not sure about `AsyncWrite::poll_close()`'s exact
    // semantics nor whether it will stay in the `AsyncWrite` trait.
    fn poll_close(self, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_file_drop() {
        crate::task::block_on(async move {
            File::open(file!()).await.unwrap();
        });
    }

    #[test]
    fn async_file_clone() {
        crate::task::block_on(async move {
            let file = File::open(file!()).await.unwrap();
            let mut clone = file.clone();
            let len = crate::task::spawn_blocking(move || {
                let mut buf = Vec::new();
                crate::task::block_on(async move {
                    clone.read_to_end(&mut buf).await.unwrap();
                    drop(clone);
                    buf.len()
                })
            }).await;
            assert_eq!(len as u64, file.metadata().await.unwrap().len());
        });
    }

    #[test]
    fn async_file_create_error() {
        let file_name = Path::new("/tmp/does_not_exist/test");
        let expect = std::fs::File::create(file_name).unwrap_err();

        crate::task::block_on(async move {
            let actual = File::create(file_name).await.unwrap_err();
            assert_eq!(format!("{}", expect), format!("{}", actual));
        })
    }

    #[test]
    fn file_eof_is_not_permanent() -> crate::io::Result<()> {
        let tempdir = tempfile::Builder::new()
            .prefix("async-std-file-eof-test")
            .tempdir()?;
        let path = tempdir.path().join("testfile");

        crate::task::block_on(async {
            let mut file_w = File::create(&path).await?;
            let mut file_r = File::open(&path).await?;

            file_w.write_all(b"data").await?;
            file_w.flush().await?;

            let mut buf = [0u8; 4];
            let mut len = file_r.read(&mut buf).await?;
            assert_eq!(len, 4);
            assert_eq!(&buf, b"data");

            len = file_r.read(&mut buf).await?;
            assert_eq!(len, 0);

            file_w.write_all(b"more").await?;
            file_w.flush().await?;

            len = file_r.read(&mut buf).await?;
            assert_eq!(len, 4);
            assert_eq!(&buf, b"more");

            len = file_r.read(&mut buf).await?;
            assert_eq!(len, 0);

            Ok(())
        })
    }
}
