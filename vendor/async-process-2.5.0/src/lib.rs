//! Async interface for working with processes.
//!
//! This crate is an async version of [`std::process`].
//!
//! # Implementation
//!
//! A background thread named "async-process" is lazily created on first use, which waits for
//! spawned child processes to exit and then calls the `wait()` syscall to clean up the "zombie"
//! processes. This is unlike the `process` API in the standard library, where dropping a running
//! `Child` leaks its resources.
//!
//! This crate uses [`async-io`] for async I/O on Unix-like systems and [`blocking`] for async I/O
//! on Windows.
//!
//! [`async-io`]: https://docs.rs/async-io
//! [`blocking`]: https://docs.rs/blocking
//!
//! # Examples
//!
//! Spawn a process and collect its output:
//!
//! ```no_run
//! # futures_lite::future::block_on(async {
//! use async_process::Command;
//!
//! let out = Command::new("echo").arg("hello").arg("world").output().await?;
//! assert_eq!(out.stdout, b"hello world\n");
//! # std::io::Result::Ok(()) });
//! ```
//!
//! Read the output line-by-line as it gets produced:
//!
//! ```no_run
//! # futures_lite::future::block_on(async {
//! use async_process::{Command, Stdio};
//! use futures_lite::{io::BufReader, prelude::*};
//!
//! let mut child = Command::new("find")
//!     .arg(".")
//!     .stdout(Stdio::piped())
//!     .spawn()?;
//!
//! let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
//!
//! while let Some(line) = lines.next().await {
//!     println!("{}", line?);
//! }
//! # std::io::Result::Ok(()) });
//! ```

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]

use std::convert::Infallible;
use std::ffi::OsStr;
use std::fmt;
use std::path::Path;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll};
use std::thread;

#[cfg(unix)]
use async_io::Async;
#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};

#[cfg(windows)]
use blocking::Unblock;

use futures_lite::{future, io, prelude::*};

#[doc(no_inline)]
pub use std::process::{ExitStatus, Output, Stdio};

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

mod reaper;

mod sealed {
    pub trait Sealed {}
}

#[cfg(test)]
static DRIVER_THREAD_SPAWNED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// The zombie process reaper.
///
/// This structure reaps zombie processes and emits the `SIGCHLD` signal.
struct Reaper {
    /// Underlying system reaper.
    sys: reaper::Reaper,

    /// The number of tasks polling the SIGCHLD event.
    ///
    /// If this is zero, the `async-process` thread must be spawned.
    drivers: AtomicUsize,

    /// Number of live `Child` instances currently running.
    ///
    /// This is used to prevent the reaper thread from being spawned right as the program closes,
    /// when the reaper thread isn't needed. This represents the number of active processes.
    child_count: AtomicUsize,
}

impl Reaper {
    /// Get the singleton instance of the reaper.
    fn get() -> &'static Self {
        static REAPER: OnceLock<Reaper> = OnceLock::new();

        REAPER.get_or_init(|| Reaper {
            sys: reaper::Reaper::new(),
            drivers: AtomicUsize::new(0),
            child_count: AtomicUsize::new(0),
        })
    }

    /// Ensure that the reaper is driven.
    ///
    /// If there are no active `driver()` callers, this will spawn the `async-process` thread.
    #[inline]
    fn ensure_driven(&'static self) {
        if self
            .drivers
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::Acquire)
            .is_ok()
        {
            self.start_driver_thread();
        }
    }

    /// Start the `async-process` thread.
    #[cold]
    fn start_driver_thread(&'static self) {
        #[cfg(test)]
        DRIVER_THREAD_SPAWNED
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .unwrap_or_else(|_| unreachable!("Driver thread already spawned"));

        thread::Builder::new()
            .name("async-process".to_string())
            .spawn(move || {
                let driver = async move {
                    // No need to bump self.drivers, it was already bumped in ensure_driven.
                    let guard = self.sys.lock().await;
                    self.sys.reap(guard).await
                };

                #[cfg(unix)]
                async_io::block_on(driver);

                #[cfg(not(unix))]
                future::block_on(driver);
            })
            .expect("cannot spawn async-process thread");
    }

    /// Register a process with this reaper.
    fn register(&'static self, child: std::process::Child) -> io::Result<reaper::ChildGuard> {
        self.ensure_driven();
        self.sys.register(child)
    }

    /// Register a process spawned outside of `std` with this reaper.
    #[cfg(target_os = "macos")]
    fn adopt(&'static self, pid: rustix::process::Pid) -> reaper::ChildGuard {
        self.ensure_driven();
        self.sys.adopt(pid)
    }
}

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        // Wraps a sync I/O type into an async I/O type.
        fn wrap<T>(io: T) -> io::Result<Unblock<T>> {
            Ok(Unblock::new(io))
        }
    } else if #[cfg(unix)] {
        /// Wrap a file descriptor into a non-blocking I/O type.
        fn wrap<T: std::os::unix::io::AsFd>(io: T) -> io::Result<Async<T>> {
            Async::new(io)
        }
    }
}

/// A guard that can kill child processes, or push them into the zombie list.
struct ChildGuard {
    inner: reaper::ChildGuard,
    reap_on_drop: bool,
    kill_on_drop: bool,
    reaper: &'static Reaper,
}

// When the last reference to the child process is dropped, push it into the zombie list.
impl Drop for ChildGuard {
    fn drop(&mut self) {
        if self.kill_on_drop {
            self.inner.kill().ok();
        }
        if self.reap_on_drop {
            self.inner.reap(&self.reaper.sys);
        }

        // Decrement number of children.
        self.reaper.child_count.fetch_sub(1, Ordering::Acquire);
    }
}

/// A spawned child process.
///
/// The process can be in running or exited state. Use [`status()`][`Child::status()`] or
/// [`output()`][`Child::output()`] to wait for it to exit.
///
/// If the [`Child`] is dropped, the process keeps running in the background.
///
/// # Examples
///
/// Spawn a process and wait for it to complete:
///
/// ```no_run
/// # futures_lite::future::block_on(async {
/// use async_process::Command;
///
/// Command::new("cp").arg("a.txt").arg("b.txt").status().await?;
/// # std::io::Result::Ok(()) });
/// ```
pub struct Child {
    /// The handle for writing to the child's standard input (stdin), if it has been captured.
    pub stdin: Option<ChildStdin>,

    /// The handle for reading from the child's standard output (stdout), if it has been captured.
    pub stdout: Option<ChildStdout>,

    /// The handle for reading from the child's standard error (stderr), if it has been captured.
    pub stderr: Option<ChildStderr>,

    /// The inner child process handle.
    child: Arc<Mutex<ChildGuard>>,
}

impl Child {
    /// Wraps the inner child process handle and registers it in the global process list.
    ///
    /// The "async-process" thread waits for processes in the global list and cleans up the
    /// resources when they exit.
    fn new(cmd: &mut Command) -> io::Result<Child> {
        // Make sure the reaper exists before we spawn the child process.
        let reaper = Reaper::get();
        let mut child = cmd.inner.spawn()?;

        // Convert sync I/O types into async I/O types.
        let stdin = child.stdin.take().map(wrap).transpose()?.map(ChildStdin);
        let stdout = child.stdout.take().map(wrap).transpose()?.map(ChildStdout);
        let stderr = child.stderr.take().map(wrap).transpose()?.map(ChildStderr);

        // Bump the child count.
        reaper.child_count.fetch_add(1, Ordering::Relaxed);

        // Register the child process in the global list.
        let inner = reaper.register(child)?;

        Ok(Child {
            stdin,
            stdout,
            stderr,
            child: Arc::new(Mutex::new(ChildGuard {
                inner,
                reap_on_drop: cmd.reap_on_drop,
                kill_on_drop: cmd.kill_on_drop,
                reaper,
            })),
        })
    }

    /// Adopts a process that was spawned by means other than [`Command`], registering it in the
    /// global process list by its OS-assigned process identifier.
    ///
    /// The returned [`Child`] has no stdio handles; the caller is responsible for any pipes
    /// connected to the process. If `reap_on_drop` is `true`, dropping the returned [`Child`]
    /// hands the process to the reaper so that it does not become a zombie. If `kill_on_drop` is
    /// `true`, dropping the returned [`Child`] also kills the process.
    ///
    /// # Safety
    ///
    /// - `pid` must identify a direct child of the calling process that has not yet been waited
    ///   on.
    /// - After this call, the process must not be waited on by any other means (such as `waitpid`
    ///   or `std::process::Child::wait`), and no other [`Child`] may be created from the same
    ///   `pid`. Otherwise, the pid may be reused by an unrelated process, which this handle could
    ///   then kill or wait on.
    #[cfg(target_os = "macos")]
    pub unsafe fn adopt_raw_pid(
        pid: u32,
        reap_on_drop: bool,
        kill_on_drop: bool,
    ) -> io::Result<Child> {
        let pid = i32::try_from(pid)
            .ok()
            .filter(|pid| *pid > 0)
            .and_then(rustix::process::Pid::from_raw)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid pid"))?;

        let reaper = Reaper::get();
        reaper.child_count.fetch_add(1, Ordering::Relaxed);
        let inner = reaper.adopt(pid);

        Ok(Child {
            stdin: None,
            stdout: None,
            stderr: None,
            child: Arc::new(Mutex::new(ChildGuard {
                inner,
                reap_on_drop,
                kill_on_drop,
                reaper,
            })),
        })
    }

    /// Returns the OS-assigned process identifier associated with this child.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    ///
    /// let mut child = Command::new("ls").spawn()?;
    /// println!("id: {}", child.id());
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn id(&self) -> u32 {
        self.child.lock().unwrap().inner.id()
    }

    /// Forces the child process to exit.
    ///
    /// If the child has already exited, an [`InvalidInput`] error is returned.
    ///
    /// This is equivalent to sending a SIGKILL on Unix platforms.
    ///
    /// [`InvalidInput`]: `std::io::ErrorKind::InvalidInput`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    ///
    /// let mut child = Command::new("yes").spawn()?;
    /// child.kill()?;
    /// println!("exit status: {}", child.status().await?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn kill(&mut self) -> io::Result<()> {
        self.child.lock().unwrap().inner.kill()
    }

    /// Returns the exit status if the process has exited.
    ///
    /// Unlike [`status()`][`Child::status()`], this method will not drop the stdin handle.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    ///
    /// let mut child = Command::new("ls").spawn()?;
    ///
    /// match child.try_status()? {
    ///     None => println!("still running"),
    ///     Some(status) => println!("exited with: {}", status),
    /// }
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn try_status(&mut self) -> io::Result<Option<ExitStatus>> {
        self.child.lock().unwrap().inner.try_wait()
    }

    /// Drops the stdin handle and waits for the process to exit.
    ///
    /// Closing the stdin of the process helps avoid deadlocks. It ensures that the process does
    /// not block waiting for input from the parent process while the parent waits for the child to
    /// exit.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::{Command, Stdio};
    ///
    /// let mut child = Command::new("cp")
    ///     .arg("a.txt")
    ///     .arg("b.txt")
    ///     .spawn()?;
    ///
    /// println!("exit status: {}", child.status().await?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn status(&mut self) -> impl Future<Output = io::Result<ExitStatus>> {
        self.stdin.take();
        let child = self.child.clone();

        async move { Reaper::get().sys.status(&child).await }
    }

    /// Drops the stdin handle and collects the output of the process.
    ///
    /// Closing the stdin of the process helps avoid deadlocks. It ensures that the process does
    /// not block waiting for input from the parent process while the parent waits for the child to
    /// exit.
    ///
    /// In order to capture the output of the process, [`Command::stdout()`] and
    /// [`Command::stderr()`] must be configured with [`Stdio::piped()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::{Command, Stdio};
    ///
    /// let child = Command::new("ls")
    ///     .stdout(Stdio::piped())
    ///     .stderr(Stdio::piped())
    ///     .spawn()?;
    ///
    /// let out = child.output().await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn output(mut self) -> impl Future<Output = io::Result<Output>> {
        // A future that waits for the exit status.
        let status = self.status();

        // A future that collects stdout.
        let stdout = self.stdout.take();
        let stdout = async move {
            let mut v = Vec::new();
            if let Some(mut s) = stdout {
                s.read_to_end(&mut v).await?;
            }
            io::Result::Ok(v)
        };

        // A future that collects stderr.
        let stderr = self.stderr.take();
        let stderr = async move {
            let mut v = Vec::new();
            if let Some(mut s) = stderr {
                s.read_to_end(&mut v).await?;
            }
            io::Result::Ok(v)
        };

        async move {
            let (stdout, stderr) = future::try_zip(stdout, stderr).await?;
            let status = status.await?;
            Ok(Output {
                status,
                stdout,
                stderr,
            })
        }
    }
}

impl fmt::Debug for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Child")
            .field("stdin", &self.stdin)
            .field("stdout", &self.stdout)
            .field("stderr", &self.stderr)
            .finish()
    }
}

/// A handle to a child process's standard input (stdin).
///
/// When a [`ChildStdin`] is dropped, the underlying handle gets closed. If the child process was
/// previously blocked on input, it becomes unblocked after dropping.
#[derive(Debug)]
pub struct ChildStdin(
    #[cfg(windows)] Unblock<std::process::ChildStdin>,
    #[cfg(unix)] Async<std::process::ChildStdin>,
);

impl ChildStdin {
    /// Convert async_process::ChildStdin into std::process::Stdio.
    ///
    /// You can use it to associate to the next process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    /// use std::process::Stdio;
    ///
    /// let mut ls_child = Command::new("ls").stdin(Stdio::piped()).spawn()?;
    /// let stdio:Stdio = ls_child.stdin.take().unwrap().into_stdio().await?;
    ///
    /// let mut echo_child = Command::new("echo").arg("./").stdout(stdio).spawn()?;
    ///
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn into_stdio(self) -> io::Result<std::process::Stdio> {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                Ok(self.0.into_inner().await.into())
            } else if #[cfg(unix)] {
                let child_stdin = self.0.into_inner()?;
                blocking_fd(rustix::fd::AsFd::as_fd(&child_stdin))?;
                Ok(child_stdin.into())
            }
        }
    }
}

impl io::AsyncWrite for ChildStdin {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

#[cfg(unix)]
impl AsRawFd for ChildStdin {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for ChildStdin {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<ChildStdin> for OwnedFd {
    type Error = io::Error;

    fn try_from(value: ChildStdin) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

// TODO(notgull): Add mirroring AsRawHandle impls for all of the child handles
//
// at the moment this is pretty hard to do because of how they're wrapped in
// Unblock, meaning that we can't always access the underlying handle. async-fs
// gets around this by putting the handle in an Arc, but there's still some decision
// to be made about how to handle this (no pun intended)

/// A handle to a child process's standard output (stdout).
///
/// When a [`ChildStdout`] is dropped, the underlying handle gets closed.
#[derive(Debug)]
pub struct ChildStdout(
    #[cfg(windows)] Unblock<std::process::ChildStdout>,
    #[cfg(unix)] Async<std::process::ChildStdout>,
);

impl ChildStdout {
    /// Convert async_process::ChildStdout into std::process::Stdio.
    ///
    /// You can use it to associate to the next process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    /// use std::process::Stdio;
    /// use std::io::Read;
    /// use futures_lite::AsyncReadExt;
    ///
    /// let mut ls_child = Command::new("ls").stdout(Stdio::piped()).spawn()?;
    /// let stdio:Stdio = ls_child.stdout.take().unwrap().into_stdio().await?;
    ///
    /// let mut echo_child = Command::new("echo").stdin(stdio).stdout(Stdio::piped()).spawn()?;
    /// let mut buf = vec![];
    /// echo_child.stdout.take().unwrap().read(&mut buf).await;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn into_stdio(self) -> io::Result<std::process::Stdio> {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                Ok(self.0.into_inner().await.into())
            } else if #[cfg(unix)] {
                let child_stdout = self.0.into_inner()?;
                blocking_fd(rustix::fd::AsFd::as_fd(&child_stdout))?;
                Ok(child_stdout.into())
            }
        }
    }
}

impl io::AsyncRead for ChildStdout {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

#[cfg(unix)]
impl AsRawFd for ChildStdout {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for ChildStdout {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<ChildStdout> for OwnedFd {
    type Error = io::Error;

    fn try_from(value: ChildStdout) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

/// A handle to a child process's standard error (stderr).
///
/// When a [`ChildStderr`] is dropped, the underlying handle gets closed.
#[derive(Debug)]
pub struct ChildStderr(
    #[cfg(windows)] Unblock<std::process::ChildStderr>,
    #[cfg(unix)] Async<std::process::ChildStderr>,
);

impl ChildStderr {
    /// Convert async_process::ChildStderr into std::process::Stdio.
    ///
    /// You can use it to associate to the next process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    /// use std::process::Stdio;
    ///
    /// let mut ls_child = Command::new("ls").arg("x").stderr(Stdio::piped()).spawn()?;
    /// let stdio:Stdio = ls_child.stderr.take().unwrap().into_stdio().await?;
    ///
    /// let mut echo_child = Command::new("echo").stdin(stdio).spawn()?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn into_stdio(self) -> io::Result<std::process::Stdio> {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                Ok(self.0.into_inner().await.into())
            } else if #[cfg(unix)] {
                let child_stderr = self.0.into_inner()?;
                blocking_fd(rustix::fd::AsFd::as_fd(&child_stderr))?;
                Ok(child_stderr.into())
            }
        }
    }
}

impl io::AsyncRead for ChildStderr {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

#[cfg(unix)]
impl AsRawFd for ChildStderr {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for ChildStderr {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<ChildStderr> for OwnedFd {
    type Error = io::Error;

    fn try_from(value: ChildStderr) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

/// Runs the driver for the asynchronous processes.
///
/// This future takes control of global structures related to driving [`Child`]ren and reaping
/// zombie processes. These responsibilities include listening for the `SIGCHLD` signal and
/// making sure zombie processes are successfully waited on.
///
/// If multiple tasks run `driver()` at once, only one will actually drive the reaper; the other
/// ones will just sleep. If a task that is driving the reaper is dropped, a previously sleeping
/// task will take over. If all tasks driving the reaper are dropped, the "async-process" thread
/// will be spawned. The "async-process" thread just blocks on this future and will automatically
/// be spawned if no tasks are driving the reaper once a [`Child`] is created.
///
/// This future will never complete. It is intended to be ran on a background task in your
/// executor of choice.
///
/// # Examples
///
/// ```no_run
/// use async_executor::Executor;
/// use async_process::{driver, Command};
///
/// # futures_lite::future::block_on(async {
/// // Create an executor and run on it.
/// let ex = Executor::new();
/// ex.run(async {
///     // Run the driver future in the background.
///     ex.spawn(driver()).detach();
///
///     // Run a command.
///     Command::new("ls").output().await.ok();
/// }).await;
/// # });
/// ```
#[allow(clippy::manual_async_fn)]
#[inline]
pub fn driver() -> impl Future<Output = Infallible> + Send + 'static {
    async {
        // Get the reaper.
        let reaper = Reaper::get();

        // Make sure the reaper knows we're driving it.
        reaper.drivers.fetch_add(1, Ordering::SeqCst);

        // Decrement the driver count when this future is dropped.
        let _guard = CallOnDrop(|| {
            let prev_count = reaper.drivers.fetch_sub(1, Ordering::SeqCst);

            // If this was the last driver, and there are still resources actively using the
            // reaper, make sure that there is a thread driving the reaper.
            if prev_count == 1
                && (reaper.child_count.load(Ordering::SeqCst) > 0 || reaper.sys.has_zombies())
            {
                reaper.ensure_driven();
            }
        });

        // Acquire the reaper lock and start polling the SIGCHLD event.
        let guard = reaper.sys.lock().await;
        reaper.sys.reap(guard).await
    }
}

/// A builder for spawning processes.
///
/// # Examples
///
/// ```no_run
/// # futures_lite::future::block_on(async {
/// use async_process::Command;
///
/// let output = if cfg!(target_os = "windows") {
///     Command::new("cmd").args(&["/C", "echo hello"]).output().await?
/// } else {
///     Command::new("sh").arg("-c").arg("echo hello").output().await?
/// };
/// # std::io::Result::Ok(()) });
/// ```
pub struct Command {
    inner: std::process::Command,
    stdin: bool,
    stdout: bool,
    stderr: bool,
    reap_on_drop: bool,
    kill_on_drop: bool,
}

impl Command {
    /// Constructs a new [`Command`] for launching `program`.
    ///
    /// The initial configuration (the working directory and environment variables) is inherited
    /// from the current process.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// ```
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Self::from(std::process::Command::new(program))
    }

    /// Adds a single argument to pass to the program.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("echo");
    /// cmd.arg("hello");
    /// cmd.arg("world");
    /// ```
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.inner.arg(arg);
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("echo");
    /// cmd.args(&["hello", "world"]);
    /// ```
    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    /// Configures an environment variable for the new process.
    ///
    /// Note that environment variable names are case-insensitive (but case-preserving) on Windows,
    /// and case-sensitive on all other platforms.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.env("PATH", "/bin");
    /// ```
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.inner.env(key, val);
        self
    }

    /// Configures multiple environment variables for the new process.
    ///
    /// Note that environment variable names are case-insensitive (but case-preserving) on Windows,
    /// and case-sensitive on all other platforms.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.envs(vec![("PATH", "/bin"), ("TERM", "xterm-256color")]);
    /// ```
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Command
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.inner.envs(vars);
        self
    }

    /// Gets an iterator of the arguments that will be passed to the program.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("echo");
    /// cmd.arg("first").arg("second");
    /// let args: Vec<&OsStr> = cmd.get_args().collect();
    /// assert_eq!(args, &["first", "second"]);
    /// ```
    pub fn get_args(&self) -> std::process::CommandArgs<'_> {
        self.inner.get_args()
    }

    /// Gets an iterator of the environment variables explicitly set for the child process.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.env("TERM", "dumb").env_remove("TZ");
    /// let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
    /// assert_eq!(envs, &[
    ///     (OsStr::new("TERM"), Some(OsStr::new("dumb"))),
    ///     (OsStr::new("TZ"), None)
    /// ]);
    /// ```
    pub fn get_envs(&self) -> std::process::CommandEnvs<'_> {
        self.inner.get_envs()
    }

    /// Gets the working directory for the child process.
    ///
    /// This returns [`None`] if the working directory will not be changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// assert_eq!(cmd.get_current_dir(), None);
    /// cmd.current_dir("/bin");
    /// assert_eq!(cmd.get_current_dir(), Some(Path::new("/bin")));
    /// ```
    pub fn get_current_dir(&self) -> Option<&Path> {
        self.inner.get_current_dir()
    }

    /// Gets the path to the program that was given to [`Command::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let cmd = Command::new("echo");
    /// assert_eq!(cmd.get_program(), "echo");
    /// ```
    pub fn get_program(&self) -> &OsStr {
        self.inner.get_program()
    }

    /// Removes an environment variable mapping.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.env_remove("PATH");
    /// ```
    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Command {
        self.inner.env_remove(key);
        self
    }

    /// Removes all environment variable mappings.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.env_clear();
    /// ```
    pub fn env_clear(&mut self) -> &mut Command {
        self.inner.env_clear();
        self
    }

    /// Configures the working directory for the new process.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::Command;
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.current_dir("/");
    /// ```
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command {
        self.inner.current_dir(dir);
        self
    }

    /// Configures the standard input (stdin) for the new process.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::{Command, Stdio};
    ///
    /// let mut cmd = Command::new("cat");
    /// cmd.stdin(Stdio::null());
    /// ```
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdin = true;
        self.inner.stdin(cfg);
        self
    }

    /// Configures the standard output (stdout) for the new process.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::{Command, Stdio};
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.stdout(Stdio::piped());
    /// ```
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdout = true;
        self.inner.stdout(cfg);
        self
    }

    /// Configures the standard error (stderr) for the new process.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::{Command, Stdio};
    ///
    /// let mut cmd = Command::new("ls");
    /// cmd.stderr(Stdio::piped());
    /// ```
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stderr = true;
        self.inner.stderr(cfg);
        self
    }

    /// Configures whether to reap the zombie process when [`Child`] is dropped.
    ///
    /// When the process finishes, it becomes a "zombie" and some resources associated with it
    /// remain until [`Child::try_status()`], [`Child::status()`], or [`Child::output()`] collects
    /// its exit code.
    ///
    /// If its exit code is never collected, the resources may leak forever. This crate has a
    /// background thread named "async-process" that collects such "zombie" processes and then
    /// "reaps" them, thus preventing the resource leaks.
    ///
    /// The default value of this option is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::{Command, Stdio};
    ///
    /// let mut cmd = Command::new("cat");
    /// cmd.reap_on_drop(false);
    /// ```
    pub fn reap_on_drop(&mut self, reap_on_drop: bool) -> &mut Command {
        self.reap_on_drop = reap_on_drop;
        self
    }

    /// Configures whether to kill the process when [`Child`] is dropped.
    ///
    /// The default value of this option is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_process::{Command, Stdio};
    ///
    /// let mut cmd = Command::new("cat");
    /// cmd.kill_on_drop(true);
    /// ```
    pub fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Command {
        self.kill_on_drop = kill_on_drop;
        self
    }

    /// Executes the command and returns the [`Child`] handle to it.
    ///
    /// If not configured, stdin, stdout and stderr will be set to [`Stdio::inherit()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    ///
    /// let child = Command::new("ls").spawn()?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn spawn(&mut self) -> io::Result<Child> {
        if !self.stdin {
            self.inner.stdin(Stdio::inherit());
        }
        if !self.stdout {
            self.inner.stdout(Stdio::inherit());
        }
        if !self.stderr {
            self.inner.stderr(Stdio::inherit());
        }

        Child::new(self)
    }

    /// Executes the command, waits for it to exit, and returns the exit status.
    ///
    /// If not configured, stdin, stdout and stderr will be set to [`Stdio::inherit()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    ///
    /// let status = Command::new("cp")
    ///     .arg("a.txt")
    ///     .arg("b.txt")
    ///     .status()
    ///     .await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn status(&mut self) -> impl Future<Output = io::Result<ExitStatus>> {
        let child = self.spawn();
        async { child?.status().await }
    }

    /// Executes the command and collects its output.
    ///
    /// If not configured, stdin will be set to [`Stdio::null()`], and stdout and stderr will be
    /// set to [`Stdio::piped()`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # futures_lite::future::block_on(async {
    /// use async_process::Command;
    ///
    /// let output = Command::new("cat")
    ///     .arg("a.txt")
    ///     .output()
    ///     .await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn output(&mut self) -> impl Future<Output = io::Result<Output>> {
        if !self.stdin {
            self.inner.stdin(Stdio::null());
        }
        if !self.stdout {
            self.inner.stdout(Stdio::piped());
        }
        if !self.stderr {
            self.inner.stderr(Stdio::piped());
        }

        let child = Child::new(self);
        async { child?.output().await }
    }
}

impl From<std::process::Command> for Command {
    fn from(inner: std::process::Command) -> Self {
        Self {
            inner,
            stdin: false,
            stdout: false,
            stderr: false,
            reap_on_drop: true,
            kill_on_drop: false,
        }
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("Command")
                .field("inner", &self.inner)
                .field("stdin", &self.stdin)
                .field("stdout", &self.stdout)
                .field("stderr", &self.stderr)
                .field("reap_on_drop", &self.reap_on_drop)
                .field("kill_on_drop", &self.kill_on_drop)
                .finish()
        } else {
            // Stdlib outputs command-line in Debug for Command. This does the
            // same, if not in "alternate" (long pretty-printed) mode.
            // This is useful for logs, for example.
            fmt::Debug::fmt(&self.inner, f)
        }
    }
}

/// Moves `Fd` out of non-blocking mode.
#[cfg(unix)]
fn blocking_fd(fd: rustix::fd::BorrowedFd<'_>) -> io::Result<()> {
    cfg_if::cfg_if! {
        // ioctl(FIONBIO) sets the flag atomically, but we use this only on Linux
        // for now, as with the standard library, because it seems to behave
        // differently depending on the platform.
        // https://github.com/rust-lang/rust/commit/efeb42be2837842d1beb47b51bb693c7474aba3d
        // https://github.com/libuv/libuv/blob/e9d91fccfc3e5ff772d5da90e1c4a24061198ca0/src/unix/poll.c#L78-L80
        // https://github.com/tokio-rs/mio/commit/0db49f6d5caf54b12176821363d154384357e70a
        if #[cfg(target_os = "linux")] {
            rustix::io::ioctl_fionbio(fd, false)?;
        } else {
            let previous = rustix::fs::fcntl_getfl(fd)?;
            let new = previous & !rustix::fs::OFlags::NONBLOCK;
            if new != previous {
                rustix::fs::fcntl_setfl(fd, new)?;
            }
        }
    }
    Ok(())
}

struct CallOnDrop<F: FnMut()>(F);

impl<F: FnMut()> Drop for CallOnDrop<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn polled_driver() {
        use super::{driver, Command};
        use futures_lite::future;
        use futures_lite::prelude::*;

        let is_thread_spawned =
            || super::DRIVER_THREAD_SPAWNED.load(std::sync::atomic::Ordering::SeqCst);

        #[cfg(unix)]
        fn command() -> Command {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg("echo hello");
            cmd
        }

        #[cfg(windows)]
        fn command() -> Command {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg("echo hello");
            cmd
        }

        #[cfg(unix)]
        const OUTPUT: &[u8] = b"hello\n";
        #[cfg(windows)]
        const OUTPUT: &[u8] = b"hello\r\n";

        future::block_on(async {
            // Thread should not be spawned off the bat.
            assert!(!is_thread_spawned());

            // Spawn a driver.
            let mut driver1 = Box::pin(driver());
            future::poll_once(&mut driver1).await;
            assert!(!is_thread_spawned());

            // We should be able to run the driver in parallel with a process future.
            async {
                (&mut driver1).await;
            }
            .or(async {
                let output = command().output().await.unwrap();
                assert_eq!(output.stdout, OUTPUT);
            })
            .await;
            assert!(!is_thread_spawned());

            // Spawn a second driver.
            let mut driver2 = Box::pin(driver());
            future::poll_once(&mut driver2).await;
            assert!(!is_thread_spawned());

            // Poll both drivers in parallel.
            async {
                (&mut driver1).await;
            }
            .or(async {
                (&mut driver2).await;
            })
            .or(async {
                let output = command().output().await.unwrap();
                assert_eq!(output.stdout, OUTPUT);
            })
            .await;
            assert!(!is_thread_spawned());

            // Once one is dropped, the other should take over.
            drop(driver1);
            assert!(!is_thread_spawned());

            // Poll driver2 in parallel with a process future.
            async {
                (&mut driver2).await;
            }
            .or(async {
                let output = command().output().await.unwrap();
                assert_eq!(output.stdout, OUTPUT);
            })
            .await;
            assert!(!is_thread_spawned());

            // Once driver2 is dropped, the thread should not be spawned, as there are no active
            // child processes..
            drop(driver2);
            assert!(!is_thread_spawned());

            // We should now be able to poll the process future independently, it will spawn the
            // thread.
            let output = command().output().await.unwrap();
            assert_eq!(output.stdout, OUTPUT);
            assert!(is_thread_spawned());
        });
    }
}
