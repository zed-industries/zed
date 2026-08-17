#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! This crate provides the [`open`] function, which opens a file or link with the default program
//! configured on the system:
//!
//! ```no_run
//! # fn main() -> Result<(), ::opener::OpenError> {
//! // open a website
//! opener::open("https://www.rust-lang.org")?;
//!
//! // open a file
//! opener::open("../Cargo.toml")?;
//! # Ok(())
//! # }
//! ```
//!
//! An [`open_browser`] function is also provided, for when you intend on opening a file or link in
//! a browser, specifically. This function works like the [`open`] function, but explicitly allows
//! overriding the browser launched by setting the `$BROWSER` environment variable.
//!
//! # Crate features
//!
//! - **reveal** - Enables usage of the [`reveal`] function. On Linux this will pull in the
//!   [`dbus`](https://docs.rs/dbus) crate. If the **dbus-vendored** feature is enabled (which it
//!   is by default), dbus will use static linking. Otherwise it will be dynamically linked, and
//!   some system dependencies will be required to build `dbus` and also to run any built binaries.
//!   Refer to the [`dbus` docs](https://github.com/diwic/dbus-rs#requirements) for the specifics.
//! - **dbus-vendored** - See the **reveal** feature.

#![warn(
    rust_2018_idioms,
    deprecated_in_future,
    macro_use_extern_crate,
    missing_debug_implementations,
    unused_qualifications
)]

#[cfg(all(feature = "reveal", target_os = "linux"))]
mod freedesktop;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod linux_and_more;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use crate::linux_and_more as sys;
#[cfg(target_os = "macos")]
use crate::macos as sys;
#[cfg(target_os = "windows")]
use crate::windows as sys;

use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter};
use std::process::{Command, ExitStatus, Stdio};
use std::{env, io};

/// Opens a file or link with the system default program.
///
/// Note that a path like "rustup.rs" could potentially refer to either a file or a website. If you
/// want to open the website, you should add the "http://" prefix, for example.
///
/// Also note that a result of `Ok(())` just means a way of opening the path was found, and no error
/// occurred as a direct result of opening the path. Errors beyond that point aren't caught. For
/// example, `Ok(())` would be returned even if a file was opened with a program that can't read the
/// file, or a dead link was opened in a browser.
///
/// ## Platform Implementation Details
///
/// - On Windows the `ShellExecuteW` Windows API function is used.
/// - On Mac the system `open` command is used.
/// - On Windows Subsystem for Linux (WSL), the system `wslview` from [`wslu`] is used if available,
/// otherwise the system `xdg-open` is used, if available.
/// - On non-WSL Linux and other platforms,
/// the system `xdg-open` script is used if available, otherwise an `xdg-open` script embedded in
/// this library is used.
///
/// [`wslu`]: https://github.com/wslutilities/wslu/
pub fn open<P>(path: P) -> Result<(), OpenError>
where
    P: AsRef<OsStr>,
{
    sys::open(path.as_ref())
}

/// Opens a file or link with the system default program, using the `BROWSER` environment variable
/// when set.
///
/// If the `BROWSER` environment variable is set, the program specified by it is used to open the
/// path. If not, behavior is identical to [`open()`].
pub fn open_browser<P>(path: P) -> Result<(), OpenError>
where
    P: AsRef<OsStr>,
{
    let mut path = path.as_ref();
    if let Ok(browser_var) = env::var("BROWSER") {
        let windows_path;
        if is_wsl() && browser_var.ends_with(".exe") {
            if let Some(windows_path_2) = wsl_to_windows_path(path) {
                windows_path = windows_path_2;
                path = &windows_path;
            }
        };

        Command::new(&browser_var)
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| OpenError::Spawn {
                cmds: browser_var,
                source: err,
            })?;

        Ok(())
    } else {
        sys::open(path)
    }
}

/// Opens the default file explorer and reveals a file or folder in its containing folder.
///
/// ## Errors
/// This function may or may not return an error if the path does not exist.
///
/// ## Platform Implementation Details
/// - On Windows and Windows Subsystem for Linux (WSL) the `explorer.exe /select, <path>` command is used.
/// - On Mac the system `open -R` command is used.
/// - On non-WSL Linux the [`file-manager-interface`] or the [`org.freedesktop.portal.OpenURI`] DBus Interface is used if available,
///   falling back to opening the containing folder with [`open`].
/// - On other platforms, the containing folder is shown with [`open`].
///
/// [`org.freedesktop.portal.OpenURI`]: https://flatpak.github.io/xdg-desktop-portal/#gdbus-org.freedesktop.portal.OpenURI
/// [`file-manager-interface`]: https://freedesktop.org/wiki/Specifications/file-manager-interface/
#[cfg(feature = "reveal")]
pub fn reveal<P>(path: P) -> Result<(), OpenError>
where
    P: AsRef<std::path::Path>,
{
    sys::reveal(path.as_ref())
}

/// An error type representing the failure to open a path. Possibly returned by the [`open`]
/// function.
#[non_exhaustive]
#[derive(Debug)]
pub enum OpenError {
    /// An IO error occurred.
    Io(io::Error),

    /// There was an error spawning command(s).
    Spawn {
        /// The command(s) that failed to spawn.
        cmds: String,

        /// The underlying error.
        source: io::Error,
    },

    /// A command exited with a non-zero exit status.
    ExitStatus {
        /// A string that identifies the command.
        cmd: &'static str,

        /// The failed process's exit status.
        status: ExitStatus,

        /// Anything the process wrote to stderr.
        stderr: String,
    },
}

impl Display for OpenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpenError::Io(_) => {
                write!(f, "IO error")?;
            }
            OpenError::Spawn { cmds, source: _ } => {
                write!(f, "error spawning command(s) '{cmds}'")?;
            }
            OpenError::ExitStatus {
                cmd,
                status,
                stderr,
            } => {
                write!(f, "command '{cmd}' did not execute successfully; {status}")?;

                let stderr = stderr.trim();
                if !stderr.is_empty() {
                    write!(f, "\ncommand stderr:\n{stderr}")?;
                }
            }
        }

        Ok(())
    }
}

impl Error for OpenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OpenError::Io(inner) => Some(inner),
            OpenError::Spawn { cmds: _, source } => Some(source),
            OpenError::ExitStatus { .. } => None,
        }
    }
}

#[cfg(target_os = "linux")]
fn is_wsl() -> bool {
    sys::is_wsl()
}

#[cfg(not(target_os = "linux"))]
fn is_wsl() -> bool {
    false
}

#[cfg(target_os = "linux")]
fn wsl_to_windows_path(path: &OsStr) -> Option<OsString> {
    use bstr::ByteSlice;
    use std::os::unix::ffi::OsStringExt;

    let output = Command::new("wslpath")
        .arg("-w")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(OsString::from_vec(output.stdout.trim_end().to_vec()))
}

#[cfg(not(target_os = "linux"))]
fn wsl_to_windows_path(_path: &OsStr) -> Option<OsString> {
    unreachable!()
}

#[cfg(not(target_os = "windows"))]
fn wait_child(child: &mut std::process::Child, cmd_name: &'static str) -> Result<(), OpenError> {
    use std::io::Read;

    let exit_status = child.wait().map_err(OpenError::Io)?;
    if exit_status.success() {
        Ok(())
    } else {
        let mut stderr_output = String::new();
        if let Some(stderr) = child.stderr.as_mut() {
            stderr.read_to_string(&mut stderr_output).ok();
        }

        Err(OpenError::ExitStatus {
            cmd: cmd_name,
            status: exit_status,
            stderr: stderr_output,
        })
    }
}
