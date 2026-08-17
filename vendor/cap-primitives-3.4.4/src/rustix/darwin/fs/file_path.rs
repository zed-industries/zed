//! `get_path` translation code for macOS derived from Rust's
//! library/std/src/sys/unix/fs.rs at revision
//! 108e90ca78f052c0c1c49c42a22c85620be19712.

use crate::rustix::fs::file_path_by_ttyname_or_seaching;
use rustix::fs::getpath;
use std::ffi::OsString;
use std::fs;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStringExt;
use std::path::PathBuf;

pub(crate) fn file_path(file: &fs::File) -> Option<PathBuf> {
    if let Ok(path) = getpath(file) {
        return Some(OsString::from_vec(path.into_bytes()).into());
    }

    file_path_by_ttyname_or_seaching(file)
}
