use crate::FileTime;
use std::ffi::CString;
use std::io;
use std::os::unix::prelude::*;
use std::path::Path;

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
        if cfg!(target_os = "emscripten") {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "emscripten does not support utimensat for symlinks",
            ));
        }
        libc::AT_SYMLINK_NOFOLLOW
    } else {
        0
    };

    let p = CString::new(p.as_os_str().as_bytes())?;
    let times = [super::to_timespec(&atime), super::to_timespec(&mtime)];
    let rc = unsafe { libc::utimensat(libc::AT_FDCWD, p.as_ptr(), times.as_ptr(), flags) };
    if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
