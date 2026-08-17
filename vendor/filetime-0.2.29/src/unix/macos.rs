//! Beginning with macOS 10.13, `utimensat` is supported by the OS, so here, we check if the symbol exists
//! and if not, we fallback to `utimes`.

use crate::FileTime;
use libc::{c_char, c_int, timespec};
use std::ffi::{CStr, CString};
use std::os::unix::prelude::*;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::{io, mem};

pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    set_times(p, Some(atime), Some(mtime), true)
}

fn set_times(
    p: &Path,
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
    symlink: bool,
) -> io::Result<()> {
    // Attempt to use the `utimensat` syscall, but if it's not supported by the
    // current kernel then fall back to an older syscall.
    if let Some(func) = utimensat() {
        let flags = if symlink {
            libc::AT_SYMLINK_NOFOLLOW
        } else {
            0
        };

        let p = CString::new(p.as_os_str().as_bytes())?;
        let times = [super::to_timespec(&atime), super::to_timespec(&mtime)];
        let rc = unsafe { func(libc::AT_FDCWD, p.as_ptr(), times.as_ptr(), flags) };
        if rc == 0 {
            return Ok(());
        } else {
            return Err(io::Error::last_os_error());
        }
    }

    super::utimes::set_times(p, atime, mtime, symlink)
}

fn utimensat() -> Option<unsafe extern "C" fn(c_int, *const c_char, *const timespec, c_int) -> c_int>
{
    static ADDR: AtomicUsize = AtomicUsize::new(0);
    unsafe {
        fetch(&ADDR, CStr::from_bytes_with_nul_unchecked(b"utimensat\0"))
            .map(|sym| mem::transmute(sym))
    }
}

fn fetch(cache: &AtomicUsize, name: &CStr) -> Option<usize> {
    match cache.load(SeqCst) {
        0 => {}
        1 => return None,
        n => return Some(n),
    }
    let sym = unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr() as *const _) };
    let (val, ret) = if sym.is_null() {
        (1, None)
    } else {
        (sym as usize, Some(sym as usize))
    };
    cache.store(val, SeqCst);
    return ret;
}
