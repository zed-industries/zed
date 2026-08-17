use crate::FileTime;
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::prelude::*;
use std::path::Path;

fn get_times(
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
    current: impl FnOnce() -> io::Result<fs::Metadata>,
) -> io::Result<Option<(FileTime, FileTime)>> {
    let pair = match (atime, mtime) {
        (Some(a), Some(b)) => (a, b),
        (None, None) => return Ok(None),
        (Some(a), None) => {
            let meta = current()?;
            (a, FileTime::from_last_modification_time(&meta))
        }
        (None, Some(b)) => {
            let meta = current()?;
            (FileTime::from_last_access_time(&meta), b)
        }
    };
    Ok(Some(pair))
}

#[allow(dead_code)]
pub fn set_symlink_file_times(p: &Path, atime: FileTime, mtime: FileTime) -> io::Result<()> {
    set_times(p, Some(atime), Some(mtime), true)
}

pub fn set_times(
    p: &Path,
    atime: Option<FileTime>,
    mtime: Option<FileTime>,
    symlink: bool,
) -> io::Result<()> {
    let (atime, mtime) = match get_times(atime, mtime, || p.metadata())? {
        Some(pair) => pair,
        None => return Ok(()),
    };
    let p = CString::new(p.as_os_str().as_bytes())?;
    let times = [to_timeval(&atime), to_timeval(&mtime)];
    let rc = unsafe {
        if symlink {
            libc::lutimes(p.as_ptr(), times.as_ptr())
        } else {
            libc::utimes(p.as_ptr(), times.as_ptr())
        }
    };
    return if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    };
}

fn to_timeval(ft: &FileTime) -> libc::timeval {
    libc::timeval {
        tv_sec: ft.seconds() as libc::time_t,
        tv_usec: (ft.nanoseconds() / 1000) as libc::suseconds_t,
    }
}

#[cfg(target_env = "uclibc")]
fn to_timespec(ft: &FileTime) -> libc::timespec {
    libc::timespec {
        tv_sec: ft.seconds() as libc::time_t,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        tv_nsec: (ft.nanoseconds()) as i64,
        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        tv_nsec: (ft.nanoseconds()) as libc::c_long,
    }
}
