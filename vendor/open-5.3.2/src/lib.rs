//! Use this library to open a given path or URL in some application, obeying the current user's desktop configuration.
//!
//! It is expected that `http:` and `https:` URLs will open in a web browser, although desktop configuration may override this.
//! The choice of application for opening other path types is highly system-dependent.
//!
//! # Usage
//!
//! Open the given URL in the default web browser, without blocking.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! open::that("http://rust-lang.org")?;
//! # Ok(())
//! # }
//! ```
//!
//! Alternatively, specify the program to be used to open the path or URL.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! open::with("http://rust-lang.org", "firefox")?;
//! # Ok(())
//! # }
//! ```
//!
//! Or obtain the commands to launch a file or path without running them.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let cmds = open::commands("http://rust-lang.org")[0].status()?;
//! # Ok(())
//! # }
//! ```
//!
//! It's also possible to obtain a command that can be used to open a path in an application.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let status = open::with_command("http://rust-lang.org", "firefox").status()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Notes
//!
//! ## Nonblocking operation
//!
//! The functions provided are nonblocking as it will return even though the
//! launched child process is still running. Note that depending on the operating
//! system, spawning launch helpers, which this library does under the hood,
//! might still take 100's of milliseconds.
//!
//! **Beware that on some platforms and circumstances, the launcher may block**.
//! In this case, please use the [`commands()`] or [`with_command()`] accordingly
//! to `spawn()` it without blocking.
//!
//! ## Error handling
//!
//! As an operating system program is used, the open operation can fail.
//! Therefore, you are advised to check the result and behave
//! accordingly, e.g. by letting the user know that the open operation failed.
//!
//! ```no_run
//! let path = "http://rust-lang.org";
//!
//! match open::that(path) {
//!     Ok(()) => println!("Opened '{}' successfully.", path),
//!     Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
//! }
//! ```
//!
//! ## UNIX Desktop
//!
//! The choice of application for opening a given URL or path on a UNIX desktop is highly dependent on the user's GUI framework (if any) and its configuration.
//! `open::that()` tries a sequence of opener commands to open the specified path.
//! The configuration of these openers is dependent on the window system.
//!
// On a console, without a window manager, results will likely be poor. The openers expect to be able to open in a new or existing window, something that consoles lack.
//!
//! On Windows WSL, `wslview` is tried first, then `xdg-open`. In other UNIX environments, `xdg-open` is tried first. If this fails, a sequence of other openers is tried.
//!
//! Currently the `BROWSER` environment variable is ignored even for `http:` and `https:` URLs unless the opener being used happens to respect it.
//!
//! It cannot be overemphasized how fragile this all is in UNIX environments. It is common for the various MIME tables to incorrectly specify the application "owning" a given filetype.
//! It is common for openers to behave strangely. Use with caution, as this crate merely inherits a particular platforms shortcomings.

#[cfg(target_os = "windows")]
use windows as os;

#[cfg(target_os = "macos")]
use macos as os;

#[cfg(target_os = "ios")]
use ios as os;

#[cfg(target_os = "visionos")]
use ios as os;

#[cfg(target_os = "haiku")]
use haiku as os;

#[cfg(target_os = "redox")]
use redox as os;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "aix",
    target_os = "hurd"
))]
use unix as os;

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "ios",
    target_os = "visionos",
    target_os = "macos",
    target_os = "windows",
    target_os = "haiku",
    target_os = "redox",
    target_os = "aix",
    target_os = "hurd"
)))]
compile_error!("open is not supported on this platform");

use std::{
    ffi::OsStr,
    io,
    process::{Command, Stdio},
    thread,
};

/// Open path with the default application without blocking.
///
/// # Examples
///
/// ```no_run
/// let path = "http://rust-lang.org";
///
/// match open::that(path) {
///     Ok(()) => println!("Opened '{}' successfully.", path),
///     Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
/// }
/// ```
///
/// # Errors
///
/// A [`std::io::Error`] is returned on failure. Because different operating systems
/// handle errors differently it is recommend to not match on a certain error.
///
/// # Beware
///
/// Sometimes, depending on the platform and system configuration, launchers *can* block.
/// If you want to be sure they don't, use [`that_in_background()`] or [`that_detached`] instead.
pub fn that(path: impl AsRef<OsStr>) -> io::Result<()> {
    let mut last_err = None;
    for mut cmd in commands(path) {
        match cmd.status_without_output() {
            Ok(status) => {
                return Ok(status).into_result(&cmd);
            }
            Err(err) => last_err = Some(err),
        }
    }
    Err(last_err.expect("no launcher worked, at least one error"))
}

/// Open path with the given application.
///
/// This function may block if the application or launcher doesn't detach itself.
/// In that case, consider using [`with_in_background()`] or [`with_command()].
///
/// # Examples
///
/// ```no_run
/// let path = "http://rust-lang.org";
/// let app = "firefox";
///
/// match open::with(path, app) {
///     Ok(()) => println!("Opened '{}' successfully.", path),
///     Err(err) => panic!("An error occurred when opening '{}': {}", path, err),
/// }
/// ```
///
/// # Errors
///
/// A [`std::io::Error`] is returned on failure. Because different operating systems
/// handle errors differently it is recommend to not match on a certain error.
pub fn with(path: impl AsRef<OsStr>, app: impl Into<String>) -> io::Result<()> {
    let mut cmd = with_command(path, app);
    cmd.status_without_output().into_result(&cmd)
}

/// Get multiple commands that open `path` with the default application.
///
/// Each command represents a launcher to try.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let path = "http://rust-lang.org";
/// assert!(open::commands(path)[0].status()?.success());
/// # Ok(())
/// # }
/// ```
pub fn commands(path: impl AsRef<OsStr>) -> Vec<Command> {
    os::commands(path)
}

/// Get a command that uses `app` to open `path`.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let path = "http://rust-lang.org";
/// assert!(open::with_command(path, "app").status()?.success());
/// # Ok(())
/// # }
/// ```
pub fn with_command(path: impl AsRef<OsStr>, app: impl Into<String>) -> Command {
    os::with_command(path, app)
}

/// Open path with the default application in a new thread to assure it's non-blocking.
///
/// See documentation of [`that()`] for more details.
pub fn that_in_background(path: impl AsRef<OsStr>) -> thread::JoinHandle<io::Result<()>> {
    let path = path.as_ref().to_os_string();
    thread::spawn(|| that(path))
}

/// Open path with the given application in a new thread, which is useful if
/// the program ends up to be blocking. Otherwise, prefer [`with()`] for
/// straightforward error handling.
///
/// See documentation of [`with()`] for more details.
pub fn with_in_background<T: AsRef<OsStr>>(
    path: T,
    app: impl Into<String>,
) -> thread::JoinHandle<io::Result<()>> {
    let path = path.as_ref().to_os_string();
    let app = app.into();
    thread::spawn(|| with(path, app))
}

/// Open path with the default application using a detached process. which is useful if
/// the program ends up to be blocking or want to out-live your app
///
/// See documentation of [`that()`] for more details.
pub fn that_detached(path: impl AsRef<OsStr>) -> io::Result<()> {
    #[cfg(any(not(feature = "shellexecute-on-windows"), not(windows)))]
    {
        let mut last_err = None;
        for mut cmd in commands(path) {
            match cmd.spawn_detached() {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => last_err = Some(err),
            }
        }
        Err(last_err.expect("no launcher worked, at least one error"))
    }

    #[cfg(all(windows, feature = "shellexecute-on-windows"))]
    {
        windows::that_detached(path)
    }
}

/// Open path with the given application using a detached process, which is useful if
/// the program ends up to be blocking or want to out-live your app. Otherwise, prefer [`with()`] for
/// straightforward error handling.
///
/// See documentation of [`with()`] for more details.
pub fn with_detached<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> io::Result<()> {
    #[cfg(any(not(feature = "shellexecute-on-windows"), not(windows)))]
    {
        let mut cmd = with_command(path, app);
        cmd.spawn_detached()
    }

    #[cfg(all(windows, feature = "shellexecute-on-windows"))]
    {
        windows::with_detached(path, app)
    }
}

trait IntoResult<T> {
    fn into_result(self, cmd: &Command) -> T;
}

impl IntoResult<io::Result<()>> for io::Result<std::process::ExitStatus> {
    fn into_result(self, cmd: &Command) -> io::Result<()> {
        match self {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Launcher {cmd:?} failed with {:?}", status),
            )),
            Err(err) => Err(err),
        }
    }
}

trait CommandExt {
    fn status_without_output(&mut self) -> io::Result<std::process::ExitStatus>;
    fn spawn_detached(&mut self) -> io::Result<()>;
}

impl CommandExt for Command {
    fn status_without_output(&mut self) -> io::Result<std::process::ExitStatus> {
        self.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    }

    fn spawn_detached(&mut self) -> io::Result<()> {
        // This is pretty much lifted from the implementation in Alacritty:
        // https://github.com/alacritty/alacritty/blob/b9c886872d1202fc9302f68a0bedbb17daa35335/alacritty/src/daemon.rs

        self.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(unix)]
        unsafe {
            use std::os::unix::process::CommandExt as _;

            self.pre_exec(move || {
                match libc::fork() {
                    -1 => return Err(io::Error::last_os_error()),
                    0 => (),
                    _ => libc::_exit(0),
                }

                if libc::setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            });
        }
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            self.creation_flags(CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW);
        }

        self.spawn().map(|_| ())
    }
}

#[cfg(windows)]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "visionos")]
mod ios;

#[cfg(target_os = "haiku")]
mod haiku;

#[cfg(target_os = "redox")]
mod redox;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "aix",
    target_os = "hurd"
))]
mod unix;
