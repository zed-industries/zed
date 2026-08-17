//! On Linux we try to use the more accurate `utimensat` syscall but this isn't
//! always available so we also fall back to `utimes` if we couldn't find
//! `utimensat` at runtime.

use crate::FileTime;
use std::ffi::CString;
use std::io;
use std::os::unix::prelude::*;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    set_times(p, Some(atime), Some(mtime), true)
}

fn set_times(
    p: &Path,
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
    symlink: bool,
) -> io::Result<()> {
    let flags = if symlink {
        libc::AT_SYMLINK_NOFOLLOW
    } else {
        0
    };

    // Same as the `if` statement above.
    static INVALID: AtomicBool = AtomicBool::new(false);
    if !INVALID.load(SeqCst) {
        let p = CString::new(p.as_os_str().as_bytes())?;
        let times = [super::to_timespec(&atime), super::to_timespec(&mtime)];
        let rc = unsafe { libc::utimensat(libc::AT_FDCWD, p.as_ptr(), times.as_ptr(), flags) };
        if rc == 0 {
            return Ok(());
        }
        let err = io::Error::last_os_error();
        if err.raw_os_error() == Some(libc::ENOSYS) {
            INVALID.store(true, SeqCst);
        } else {
            return Err(err);
        }
    }

    super::utimes::set_times(p, atime, mtime, symlink)
}
