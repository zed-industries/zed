use crate::fs::{
    open_dir_for_reading, open_dir_for_reading_unchecked, open_entry_impl, read_dir_unchecked,
    remove_dir_unchecked, remove_file_unchecked, stat_unchecked, DirEntryInner, FollowSymlinks,
    Metadata, OpenOptions, ReadDir,
};
use io_extras::os::rustix::{AsRawFd, FromRawFd, RawFd};
use io_lifetimes::AsFd;
use rustix::fd::OwnedFd;
use rustix::fs::Dir;
use std::ffi::OsStr;
use std::mem::ManuallyDrop;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::path::{Component, Path};
use std::sync::{Arc, Mutex};
use std::{fmt, fs, io};

pub(crate) struct ReadDirInner {
    raw_fd: RawFd,

    // `Dir` doesn't implement `AsFd`, because libc `fdopendir` has UB if the
    // file descriptor is used in almost any way, so we hold a separate
    // `OwnedFd` that we can do `as_fd()` on.
    rustix: Arc<Mutex<(Dir, OwnedFd)>>,
}

impl ReadDirInner {
    pub(crate) fn new(start: &fs::File, path: &Path, follow: FollowSymlinks) -> io::Result<Self> {
        let fd = open_dir_for_reading(start, path, follow)?;
        let dir = Dir::read_from(fd.as_fd())?;
        Ok(Self {
            raw_fd: fd.as_fd().as_raw_fd(),
            rustix: Arc::new(Mutex::new((dir, OwnedFd::from(fd)))),
        })
    }

    pub(crate) fn read_base_dir(start: &fs::File) -> io::Result<Self> {
        // Open ".", to obtain a new independent file descriptor. Don't use
        // `dup` since in that case the resulting file descriptor would share
        // a current position with the original, and `read_dir` calls after
        // the first `read_dir` call wouldn't start from the beginning.
        let fd =
            open_dir_for_reading_unchecked(start, Component::CurDir.as_ref(), FollowSymlinks::No)?;
        let dir = Dir::read_from(fd.as_fd())?;
        Ok(Self {
            raw_fd: fd.as_fd().as_raw_fd(),
            rustix: Arc::new(Mutex::new((dir, fd.into()))),
        })
    }

    pub(crate) fn new_unchecked(
        start: &fs::File,
        path: &Path,
        follow: FollowSymlinks,
    ) -> io::Result<Self> {
        let fd = open_dir_for_reading_unchecked(start, path, follow)?;
        let dir = Dir::read_from(fd.as_fd())?;
        Ok(Self {
            raw_fd: fd.as_fd().as_raw_fd(),
            rustix: Arc::new(Mutex::new((dir, fd.into()))),
        })
    }

    pub(super) fn open(&self, file_name: &OsStr, options: &OpenOptions) -> io::Result<fs::File> {
        open_entry_impl(&self.as_file_view(), file_name, options)
    }

    pub(super) fn metadata(&self, file_name: &OsStr) -> io::Result<Metadata> {
        stat_unchecked(&self.as_file_view(), file_name.as_ref(), FollowSymlinks::No)
    }

    pub(super) fn remove_file(&self, file_name: &OsStr) -> io::Result<()> {
        remove_file_unchecked(&self.as_file_view(), file_name.as_ref())
    }

    pub(super) fn remove_dir(&self, file_name: &OsStr) -> io::Result<()> {
        remove_dir_unchecked(&self.as_file_view(), file_name.as_ref())
    }

    pub(super) fn self_metadata(&self) -> io::Result<Metadata> {
        Metadata::from_file(&self.as_file_view())
    }

    pub(super) fn read_dir(
        &self,
        file_name: &OsStr,
        follow: FollowSymlinks,
    ) -> io::Result<ReadDir> {
        read_dir_unchecked(&self.as_file_view(), file_name.as_ref(), follow)
    }

    #[allow(unsafe_code)]
    fn as_file_view(&self) -> ManuallyDrop<fs::File> {
        // Safety: `self.rustix` owns the file descriptor. We just hold a
        // copy outside so that we can read it without taking a lock.
        ManuallyDrop::new(unsafe { fs::File::from_raw_fd(self.raw_fd) })
    }
}

impl Iterator for ReadDirInner {
    type Item = io::Result<DirEntryInner>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.rustix.lock().unwrap().0.read()?;
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => return Some(Err(e.into())),
            };
            let file_name = entry.file_name().to_bytes();
            if file_name != Component::CurDir.as_os_str().as_bytes()
                && file_name != Component::ParentDir.as_os_str().as_bytes()
            {
                let clone = Arc::clone(&self.rustix);
                return Some(Ok(DirEntryInner {
                    rustix: entry,
                    read_dir: Self {
                        raw_fd: self.raw_fd,
                        rustix: clone,
                    },
                }));
            }
        }
    }
}

impl fmt::Debug for ReadDirInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("ReadDir");
        b.field("raw_fd", &self.raw_fd);
        b.finish()
    }
}
