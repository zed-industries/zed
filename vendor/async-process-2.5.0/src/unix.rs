//! Unix-specific extensions.

use std::ffi::OsStr;
use std::io;
use std::os::unix::process::CommandExt as _;

use crate::Command;

/// Unix-specific extensions to the [`Command`] builder.
///
/// This trait is sealed: it cannot be implemented outside `async-process`.
/// This is so that future additional methods are not breaking changes.
pub trait CommandExt: crate::sealed::Sealed {
    /// Sets the child process's user ID. This translates to a
    /// `setuid` call in the child process. Failure in the `setuid`
    /// call will cause the spawn to fail.
    fn uid(&mut self, id: u32) -> &mut Command;

    /// Similar to `uid`, but sets the group ID of the child process. This has
    /// the same semantics as the `uid` field.
    fn gid(&mut self, id: u32) -> &mut Command;

    /// Performs all the required setup by this `Command`, followed by calling
    /// the `execvp` syscall.
    ///
    /// On success this function will not return, and otherwise it will return
    /// an error indicating why the exec (or another part of the setup of the
    /// `Command`) failed.
    ///
    /// `exec` not returning has the same implications as calling
    /// [`std::process::exit`] – no destructors on the current stack or any other
    /// thread’s stack will be run. Therefore, it is recommended to only call
    /// `exec` at a point where it is fine to not run any destructors. Note,
    /// that the `execvp` syscall independently guarantees that all memory is
    /// freed and all file descriptors with the `CLOEXEC` option (set by default
    /// on all file descriptors opened by the standard library) are closed.
    ///
    /// This function, unlike `spawn`, will **not** `fork` the process to create
    /// a new child. Like spawn, however, the default behavior for the stdio
    /// descriptors will be to inherited from the current process.
    ///
    /// # Notes
    ///
    /// The process may be in a "broken state" if this function returns in
    /// error. For example the working directory, environment variables, signal
    /// handling settings, various user/group information, or aspects of stdio
    /// file descriptors may have changed. If a "transactional spawn" is
    /// required to gracefully handle errors it is recommended to use the
    /// cross-platform `spawn` instead.
    fn exec(&mut self) -> io::Error;

    /// Set executable argument
    ///
    /// Set the first process argument, `argv[0]`, to something other than the
    /// default executable path.
    fn arg0<S>(&mut self, arg: S) -> &mut Command
    where
        S: AsRef<OsStr>;
}

impl crate::sealed::Sealed for Command {}
impl CommandExt for Command {
    fn uid(&mut self, id: u32) -> &mut Command {
        self.inner.uid(id);
        self
    }

    fn gid(&mut self, id: u32) -> &mut Command {
        self.inner.gid(id);
        self
    }

    fn exec(&mut self) -> io::Error {
        self.inner.exec()
    }

    fn arg0<S>(&mut self, arg: S) -> &mut Command
    where
        S: AsRef<OsStr>,
    {
        self.inner.arg0(arg);
        self
    }
}
