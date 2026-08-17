use crate::SystemTimeSpec;
use io_lifetimes::AsFilelike;
#[cfg(not(windows))]
use rustix::{
    fs::{futimens, utimensat, AtFlags, Timestamps, CWD},
    fs::{UTIME_NOW, UTIME_OMIT},
    time::Timespec,
};
use std::path::Path;
use std::time::SystemTime;
use std::{fs, io};
#[cfg(windows)]
use {
    std::{
        os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
        ptr,
        time::Duration,
    },
    windows_sys::Win32::Foundation::{ERROR_NOT_SUPPORTED, FILETIME, HANDLE},
    windows_sys::Win32::Storage::FileSystem::{
        SetFileTime, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
    },
};

/// Set the last access timestamp of a file or other filesystem object.
#[inline]
pub fn set_atime<P: AsRef<Path>>(path: P, atime: SystemTimeSpec) -> io::Result<()> {
    set_times(path, Some(atime), None)
}

/// Set the last modification timestamp of a file or other filesystem object.
#[inline]
pub fn set_mtime<P: AsRef<Path>>(path: P, mtime: SystemTimeSpec) -> io::Result<()> {
    set_times(path, None, Some(mtime))
}

/// Set the last access and last modification timestamps of a file or other
/// filesystem object.
#[inline]
pub fn set_times<P: AsRef<Path>>(
    path: P,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let path = path.as_ref();
    _set_times(path, atime, mtime)
}

#[cfg(not(windows))]
fn _set_times(
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let times = Timestamps {
        last_access: to_timespec(atime)?,
        last_modification: to_timespec(mtime)?,
    };
    Ok(utimensat(CWD, path, &times, AtFlags::empty())?)
}

#[cfg(windows)]
fn _set_times(
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let custom_flags = FILE_FLAG_BACKUP_SEMANTICS;

    match fs::OpenOptions::new()
        .write(true)
        .custom_flags(custom_flags)
        .open(path)
    {
        Ok(file) => return _set_file_times(&file, atime, mtime),
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => return Err(err),
        },
    }

    match fs::OpenOptions::new()
        .read(true)
        .custom_flags(custom_flags)
        .open(path)
    {
        Ok(file) => return _set_file_times(&file, atime, mtime),
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => return Err(err),
        },
    }

    Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32))
}

/// Like `set_times`, but never follows symlinks.
#[inline]
pub fn set_symlink_times<P: AsRef<Path>>(
    path: P,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let path = path.as_ref();
    _set_symlink_times(path, atime, mtime)
}

/// Like `set_times`, but never follows symlinks.
#[cfg(not(windows))]
fn _set_symlink_times(
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let times = Timestamps {
        last_access: to_timespec(atime)?,
        last_modification: to_timespec(mtime)?,
    };
    Ok(utimensat(CWD, path, &times, AtFlags::SYMLINK_NOFOLLOW)?)
}

/// Like `set_times`, but never follows symlinks.
#[cfg(windows)]
fn _set_symlink_times(
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let custom_flags = FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT;

    match fs::OpenOptions::new()
        .write(true)
        .custom_flags(custom_flags)
        .open(path)
    {
        Ok(file) => return _set_file_times(&file, atime, mtime),
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => return Err(err),
        },
    }

    match fs::OpenOptions::new()
        .read(true)
        .custom_flags(custom_flags)
        .open(path)
    {
        Ok(file) => return _set_file_times(&file, atime, mtime),
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => (),
            _ => return Err(err),
        },
    }

    Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32))
}

/// An extension trait for `std::fs::File`, `cap_std::fs::File`, and similar
/// types.
pub trait SetTimes {
    /// Set the last access and last modification timestamps of an open file
    /// handle.
    ///
    /// This corresponds to [`filetime::set_file_handle_times`].
    ///
    /// [`filetime::set_file_handle_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_handle_times.html
    fn set_times(
        &self,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;
}

impl<T: AsFilelike> SetTimes for T {
    #[inline]
    fn set_times(
        &self,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        _set_file_times(&self.as_filelike_view::<fs::File>(), atime, mtime)
    }
}

#[cfg(not(windows))]
fn _set_file_times(
    file: &fs::File,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let times = Timestamps {
        last_access: to_timespec(atime)?,
        last_modification: to_timespec(mtime)?,
    };
    Ok(futimens(file, &times)?)
}

#[cfg(not(windows))]
#[allow(clippy::useless_conversion)]
pub(crate) fn to_timespec(ft: Option<SystemTimeSpec>) -> io::Result<Timespec> {
    Ok(match ft {
        None => Timespec {
            tv_sec: 0,
            tv_nsec: UTIME_OMIT.into(),
        },
        Some(SystemTimeSpec::SymbolicNow) => Timespec {
            tv_sec: 0,
            tv_nsec: UTIME_NOW.into(),
        },
        Some(SystemTimeSpec::Absolute(ft)) => {
            let duration = ft.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let nanoseconds = duration.subsec_nanos();
            assert_ne!(i64::from(nanoseconds), i64::from(UTIME_OMIT));
            assert_ne!(i64::from(nanoseconds), i64::from(UTIME_NOW));
            Timespec {
                tv_sec: duration
                    .as_secs()
                    .try_into()
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
                tv_nsec: nanoseconds.try_into().unwrap(),
            }
        }
    })
}

#[cfg(windows)]
fn _set_file_times(
    file: &fs::File,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    let mut now = None;

    let atime = match atime {
        None => None,
        Some(SystemTimeSpec::SymbolicNow) => {
            let right_now = SystemTime::now();
            now = Some(right_now);
            Some(right_now)
        }
        Some(SystemTimeSpec::Absolute(time)) => Some(time),
    };
    let mtime = match mtime {
        None => None,
        Some(SystemTimeSpec::SymbolicNow) => {
            if let Some(prev_now) = now {
                Some(prev_now)
            } else {
                Some(SystemTime::now())
            }
        }
        Some(SystemTimeSpec::Absolute(time)) => Some(time),
    };

    let atime = atime.map(to_filetime).transpose()?;
    let mtime = mtime.map(to_filetime).transpose()?;
    if unsafe {
        SetFileTime(
            file.as_raw_handle() as HANDLE,
            ptr::null(),
            atime.as_ref().map(|r| r as *const _).unwrap_or(ptr::null()),
            mtime.as_ref().map(|r| r as *const _).unwrap_or(ptr::null()),
        )
    } != 0
    {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(windows)]
fn to_filetime(ft: SystemTime) -> io::Result<FILETIME> {
    // To convert a `SystemTime` to absolute seconds and nanoseconds, we need
    // a reference point. The `UNIX_EPOCH` is the only reference point provided
    // by the standard library. But we know that Windows' time stamps are
    // relative to January 1, 1601 so adjust by the difference between that and
    // the Unix epoch.
    let epoch = SystemTime::UNIX_EPOCH - Duration::from_secs(11644473600);
    let ft = ft
        .duration_since(epoch)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let intervals = ft.as_secs() * (1_000_000_000 / 100) + u64::from(ft.subsec_nanos() / 100);

    // On Windows, a zero time is silently ignored, so issue an error instead.
    if intervals == 0 {
        return Err(io::Error::from_raw_os_error(ERROR_NOT_SUPPORTED as i32));
    }

    Ok(FILETIME {
        dwLowDateTime: intervals as u32,
        dwHighDateTime: (intervals >> 32) as u32,
    })
}
