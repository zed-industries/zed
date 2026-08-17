//-
// Copyright 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::io;
use std::process::{Child, Output};
#[cfg(feature = "timeout")]
use std::time::Duration;

#[cfg(feature = "timeout")]
use wait_timeout::ChildExt;

/// Wraps `std::process::ExitStatus`. Historically, this was due to the
/// `wait_timeout` crate having its own `ExitStatus` type.
///
/// Method documentation is copied from the [Rust std
/// docs](https://doc.rust-lang.org/stable/std/process/struct.ExitStatus.html)
/// and the [`wait_timeout`
/// docs](https://docs.rs/wait-timeout/0.1.5/wait_timeout/struct.ExitStatus.html).
#[derive(Clone, Copy)]
pub struct ExitStatusWrapper(ExitStatusEnum);

#[derive(Debug, Clone, Copy)]
enum ExitStatusEnum {
    Std(::std::process::ExitStatus),
}

impl ExitStatusWrapper {
    fn std(es: ::std::process::ExitStatus) -> Self {
        ExitStatusWrapper(ExitStatusEnum::Std(es))
    }

    /// Was termination successful? Signal termination is not considered a
    /// success, and success is defined as a zero exit status.
    pub fn success(&self) -> bool {
        match self.0 {
            ExitStatusEnum::Std(es) => es.success(),
        }
    }

    /// Returns the exit code of the process, if any.
    ///
    /// On Unix, this will return `None` if the process was terminated by a
    /// signal; `std::os::unix` provides an extension trait for extracting the
    /// signal and other details from the `ExitStatus`.
    pub fn code(&self) -> Option<i32> {
        match self.0 {
            ExitStatusEnum::Std(es) => es.code(),
        }
    }

    /// Returns the Unix signal which terminated this process.
    ///
    /// Note that on Windows this will always return None and on Unix this will
    /// return None if the process successfully exited otherwise.
    ///
    /// For simplicity and to match `wait_timeout`, this method is always
    /// present even on systems that do not support it.
    #[cfg(not(target_os = "windows"))]
    pub fn unix_signal(&self) -> Option<i32> {
        use std::os::unix::process::ExitStatusExt;

        match self.0 {
            ExitStatusEnum::Std(es) => es.signal(),
        }
    }

    /// Returns the Unix signal which terminated this process.
    ///
    /// Note that on Windows this will always return None and on Unix this will
    /// return None if the process successfully exited otherwise.
    ///
    /// For simplicity and to match `wait_timeout`, this method is always
    /// present even on systems that do not support it.
    #[cfg(target_os = "windows")]
    pub fn unix_signal(&self) -> Option<i32> {
        None
    }
}

impl fmt::Debug for ExitStatusWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ExitStatusEnum::Std(ref es) => fmt::Debug::fmt(es, f),
        }
    }
}

impl fmt::Display for ExitStatusWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ExitStatusEnum::Std(ref es) => fmt::Display::fmt(es, f),
        }
    }
}

/// Wraps a `std::process::Child` to coordinate state between `std` and
/// `wait_timeout`.
///
/// This is necessary because the completion of a call to
/// `wait_timeout::ChildExt::wait_timeout` leaves the `Child` in an
/// inconsistent state, as it does not know the child has exited, and on Unix
/// may end up referencing another process.
///
/// Documentation for this struct's methods is largely copied from the [Rust
/// std docs](https://doc.rust-lang.org/stable/std/process/struct.Child.html).
#[derive(Debug)]
pub struct ChildWrapper {
    child: Child,
    exit_status: Option<ExitStatusWrapper>,
}

impl ChildWrapper {
    pub(crate) fn new(child: Child) -> Self {
        ChildWrapper { child, exit_status: None }
    }

    /// Return a reference to the inner `std::process::Child`.
    ///
    /// Use care on the returned object, as it does not necessarily reference
    /// the correct process unless you know the child process has not exited
    /// and no wait calls have succeeded.
    pub fn inner(&self) -> &Child {
        &self.child
    }

    /// Return a mutable reference to the inner `std::process::Child`.
    ///
    /// Use care on the returned object, as it does not necessarily reference
    /// the correct process unless you know the child process has not exited
    /// and no wait calls have succeeded.
    pub fn inner_mut(&mut self) -> &mut Child {
        &mut self.child
    }

    /// Forces the child to exit. This is equivalent to sending a SIGKILL on
    /// unix platforms.
    ///
    /// If the process has already been reaped by this handle, returns a
    /// `NotFound` error.
    pub fn kill(&mut self) -> io::Result<()> {
        if self.exit_status.is_none() {
            self.child.kill()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Process already reaped"))
        }
    }

    /// Returns the OS-assigned processor identifier associated with this child.
    ///
    /// This succeeds even if the child has already been reaped. In this case,
    /// the process id may reference no process at all or even an unrelated
    /// process.
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    /// Waits for the child to exit completely, returning the status that it
    /// exited with. This function will continue to have the same return value
    /// after it has been called at least once.
    ///
    /// The stdin handle to the child process, if any, will be closed before
    /// waiting. This helps avoid deadlock: it ensures that the child does not
    /// block waiting for input from the parent, while the parent waits for the
    /// child to exit.
    ///
    /// If the child process has already been reaped, returns its exit status
    /// without blocking.
    pub fn wait(&mut self) -> io::Result<ExitStatusWrapper> {
        if let Some(status) = self.exit_status {
            Ok(status)
        } else {
            let status = ExitStatusWrapper::std(self.child.wait()?);
            self.exit_status = Some(status);
            Ok(status)
        }
    }

    /// Attempts to collect the exit status of the child if it has already exited.
    ///
    /// This function will not block the calling thread and will only
    /// advisorily check to see if the child process has exited or not. If the
    /// child has exited then on Unix the process id is reaped. This function
    /// is guaranteed to repeatedly return a successful exit status so long as
    /// the child has already exited.
    ///
    /// If the child has exited, then `Ok(Some(status))` is returned. If the
    /// exit status is not available at this time then `Ok(None)` is returned.
    /// If an error occurs, then that error is returned.
    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatusWrapper>> {
        if let Some(status) = self.exit_status {
            Ok(Some(status))
        } else {
            let status = self.child.try_wait()?.map(ExitStatusWrapper::std);
            self.exit_status = status;
            Ok(status)
        }
    }

    /// Simultaneously waits for the child to exit and collect all remaining
    /// output on the stdout/stderr handles, returning an `Output` instance.
    ///
    /// The stdin handle to the child process, if any, will be closed before
    /// waiting. This helps avoid deadlock: it ensures that the child does not
    /// block waiting for input from the parent, while the parent waits for the
    /// child to exit.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent. (In
    /// the context of `rusty_fork`, they are by default redirected to a file.)
    /// In order to capture the output into this `Result<Output>` it is
    /// necessary to create new pipes between parent and child. Use
    /// `stdout(Stdio::piped())` or `stderr(Stdio::piped())`, respectively.
    ///
    /// If the process has already been reaped, returns a `NotFound` error.
    pub fn wait_with_output(self) -> io::Result<Output> {
        if self.exit_status.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound, "Process already reaped"));
        }

        self.child.wait_with_output()
    }

    /// Wait for the child to exit, but only up to the given maximum duration.
    ///
    /// If the process has already been reaped, returns its exit status
    /// immediately. Otherwise, if the process terminates within the duration,
    /// returns `Ok(Sone(..))`, or `Ok(None)` otherwise.
    ///
    /// This is only present if the "timeout" feature is enabled.
    #[cfg(feature = "timeout")]
    pub fn wait_timeout(&mut self, dur: Duration)
                        -> io::Result<Option<ExitStatusWrapper>> {
        if let Some(status) = self.exit_status {
            Ok(Some(status))
        } else {
            let status = self.child.wait_timeout(dur)?.map(ExitStatusWrapper::std);
            self.exit_status = status;
            Ok(status)
        }
    }
}
