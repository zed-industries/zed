use bitflags::bitflags;
use io_lifetimes::{AsFilelike, FromFilelike};
#[cfg(not(any(windows, target_os = "redox")))]
use rustix::fs::{fcntl_getfl, fcntl_setfl, OFlags};
#[cfg(not(windows))]
use std::marker::PhantomData;
use std::{fs, io};
#[cfg(windows)]
use {
    cap_fs_ext::{OpenOptions, Reopen},
    cap_std::fs::OpenOptionsExt,
    io_lifetimes::AsHandle,
    windows_sys::Win32::Storage::FileSystem::FILE_FLAG_WRITE_THROUGH,
    winx::file::{AccessMode, FileModeInformation},
};

/// An opaque representation of the state needed to perform a `set_fd_flags`
/// operation.
#[cfg(windows)]
pub struct SetFdFlags<T> {
    reopened: T,
}

/// An opaque representation of the state needed to perform a `set_fd_flags`
/// operation.
#[cfg(not(windows))]
pub struct SetFdFlags<T> {
    flags: OFlags,
    _phantom: PhantomData<T>,
}

/// Extension trait that can indicate various I/O flags.
pub trait GetSetFdFlags {
    /// Query the "status" flags for the `self` file descriptor.
    fn get_fd_flags(&self) -> io::Result<FdFlags>
    where
        Self: AsFilelike;

    /// Create a new `SetFdFlags` value for use with `set_fd_flags`.
    ///
    /// Some platforms lack the ability to dynamically set the flags and
    /// implement `set_fd_flags` by closing and re-opening the resource and
    /// splitting it into two steps like this simplifies the lifetimes.
    fn new_set_fd_flags(&self, flags: FdFlags) -> io::Result<SetFdFlags<Self>>
    where
        Self: AsFilelike + FromFilelike + Sized;

    /// Set the "status" flags for the `self` file descriptor.
    ///
    /// This requires a `SetFdFlags` obtained from `new_set_fd_flags`.
    fn set_fd_flags(&mut self, set_fd_flags: SetFdFlags<Self>) -> io::Result<()>
    where
        Self: AsFilelike + Sized;
}

bitflags! {
    /// Flag definitions for use with `SetFlags::set_flags`.
    pub struct FdFlags: u32 {
        /// Writes always write to the end of the file.
        const APPEND = 0x01;

        /// Write I/O operations on the file descriptor shall complete as
        /// defined by synchronized I/O *data* integrity completion.
        const DSYNC = 0x02;

        /// I/O operations return `io::ErrorKind::WouldBlock`.
        const NONBLOCK = 0x04;

        /// Read I/O operations on the file descriptor shall complete at the
        /// same level of integrity as specified by the `DSYNC` and `SYNC` flags.
        const RSYNC = 0x08;

        /// Write I/O operations on the file descriptor shall complete as
        /// defined by synchronized I/O *file* integrity completion.
        const SYNC = 0x10;
    }
}

#[cfg(not(windows))]
impl<T> GetSetFdFlags for T {
    fn get_fd_flags(&self) -> io::Result<FdFlags>
    where
        Self: AsFilelike,
    {
        let mut fd_flags = FdFlags::empty();
        let flags = fcntl_getfl(self)?;

        fd_flags.set(FdFlags::APPEND, flags.contains(OFlags::APPEND));
        #[cfg(not(target_os = "freebsd"))]
        fd_flags.set(FdFlags::DSYNC, flags.contains(OFlags::DSYNC));
        fd_flags.set(FdFlags::NONBLOCK, flags.contains(OFlags::NONBLOCK));
        #[cfg(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "freebsd",
            target_os = "fuchsia"
        ))]
        {
            fd_flags.set(FdFlags::SYNC, flags.contains(OFlags::SYNC));
        }
        #[cfg(not(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "freebsd",
            target_os = "fuchsia"
        )))]
        {
            fd_flags.set(FdFlags::RSYNC, flags.contains(OFlags::RSYNC));
            fd_flags.set(FdFlags::SYNC, flags.contains(OFlags::SYNC));
        }

        Ok(fd_flags)
    }

    fn new_set_fd_flags(&self, fd_flags: FdFlags) -> io::Result<SetFdFlags<Self>>
    where
        Self: AsFilelike,
    {
        let mut flags = OFlags::empty();
        flags.set(OFlags::APPEND, fd_flags.contains(FdFlags::APPEND));
        flags.set(OFlags::NONBLOCK, fd_flags.contains(FdFlags::NONBLOCK));

        // Linux, FreeBSD, and others silently ignore these flags in `F_SETFL`.
        if fd_flags.intersects(FdFlags::DSYNC | FdFlags::SYNC | FdFlags::RSYNC) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "setting fd_flags SYNC, DSYNC, and RSYNC is not supported",
            ));
        }

        Ok(SetFdFlags {
            flags,
            _phantom: PhantomData,
        })
    }

    fn set_fd_flags(&mut self, set_fd_flags: SetFdFlags<Self>) -> io::Result<()>
    where
        Self: AsFilelike + Sized,
    {
        Ok(fcntl_setfl(
            &*self.as_filelike_view::<fs::File>(),
            set_fd_flags.flags,
        )?)
    }
}

#[cfg(windows)]
impl<T> GetSetFdFlags for T {
    fn get_fd_flags(&self) -> io::Result<FdFlags>
    where
        Self: AsFilelike,
    {
        let mut fd_flags = FdFlags::empty();
        let handle = self.as_filelike();
        let access_mode = winx::file::query_access_information(handle)?;
        let mode = winx::file::query_mode_information(handle)?;

        // `FILE_APPEND_DATA` with `FILE_WRITE_DATA` means append-mode.
        if access_mode.contains(AccessMode::FILE_APPEND_DATA)
            && !access_mode.contains(AccessMode::FILE_WRITE_DATA)
        {
            fd_flags |= FdFlags::APPEND;
        }

        if mode.contains(FileModeInformation::FILE_WRITE_THROUGH) {
            // Only report `DSYNC`. This is technically the only one of the
            // `O_?SYNC` flags Windows supports.
            fd_flags |= FdFlags::DSYNC;
        }

        Ok(fd_flags)
    }

    fn new_set_fd_flags(&self, fd_flags: FdFlags) -> io::Result<SetFdFlags<Self>>
    where
        Self: AsFilelike + FromFilelike,
    {
        let mut flags = 0;

        if fd_flags.contains(FdFlags::SYNC) || fd_flags.contains(FdFlags::DSYNC) {
            flags |= FILE_FLAG_WRITE_THROUGH;
        }

        let file = self.as_filelike_view::<fs::File>();
        let access_mode = winx::file::query_access_information(file.as_handle())?;
        let new_access_mode = file_access_mode_from_fd_flags(
            fd_flags,
            access_mode.contains(AccessMode::FILE_READ_DATA),
            access_mode.contains(AccessMode::FILE_WRITE_DATA)
                | access_mode.contains(AccessMode::FILE_APPEND_DATA),
        );
        let mut options = OpenOptions::new();
        options.access_mode(new_access_mode.bits());
        options.custom_flags(flags);
        let reopened = Self::from_into_filelike(file.reopen(&options)?);
        Ok(SetFdFlags { reopened })
    }

    fn set_fd_flags(&mut self, set_fd_flags: SetFdFlags<Self>) -> io::Result<()>
    where
        Self: AsFilelike,
    {
        *self = set_fd_flags.reopened;
        Ok(())
    }
}

#[cfg(windows)]
fn file_access_mode_from_fd_flags(fd_flags: FdFlags, read: bool, write: bool) -> AccessMode {
    let mut access_mode = AccessMode::READ_CONTROL;

    // We always need `FILE_WRITE_ATTRIBUTES` so that we can set attributes
    // such as filetimes, etc.
    access_mode.insert(AccessMode::FILE_WRITE_ATTRIBUTES);

    // Note that `GENERIC_READ` and `GENERIC_WRITE` cannot be used to properly
    // support append-only mode. The file-specific flags `FILE_GENERIC_READ`
    // and `FILE_GENERIC_WRITE` are used here instead. These flags have the
    // same semantic meaning for file objects, but allow removal of specific
    // permissions (see below).
    if read {
        access_mode.insert(AccessMode::FILE_GENERIC_READ);
    }
    if write {
        access_mode.insert(AccessMode::FILE_GENERIC_WRITE);
    }

    // For append, grant the handle FILE_APPEND_DATA access but *not*
    // `FILE_WRITE_DATA`. This makes the handle "append only". Changes to the
    // file pointer will be ignored (like POSIX's `O_APPEND` behavior).
    if fd_flags.contains(FdFlags::APPEND) {
        access_mode.insert(AccessMode::FILE_APPEND_DATA);
        access_mode.remove(AccessMode::FILE_WRITE_DATA);
    }

    access_mode
}
