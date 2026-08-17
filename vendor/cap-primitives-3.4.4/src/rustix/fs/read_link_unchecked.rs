use rustix::fs::readlinkat;
use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// *Unsandboxed* function similar to `read_link`, but which does not perform
/// sandboxing.
pub(crate) fn read_link_unchecked(
    start: &fs::File,
    path: &Path,
    reuse: PathBuf,
) -> io::Result<PathBuf> {
    Ok(readlinkat(start, path, reuse.into_os_string().into_vec())
        .map(|path| OsString::from_vec(path.into_bytes()).into())?)
}
