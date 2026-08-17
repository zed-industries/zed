use crate::fs::file_path_by_searching;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use rustix::termios::ttyname;
#[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
use std::ffi::OsString;
use std::fs;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

pub(crate) fn file_path_by_ttyname_or_seaching(file: &fs::File) -> Option<PathBuf> {
    // If it happens to be a tty, we can look up its name.
    #[cfg(not(any(target_os = "wasi", target_os = "fuchsia")))]
    if let Ok(name) = ttyname(file, Vec::new()) {
        return Some(OsString::from_vec(name.into_bytes()).into());
    }

    file_path_by_searching(file)
}
