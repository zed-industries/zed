use crate::fs::{
    FileType, FollowSymlinks, Metadata, MetadataExt, OpenOptions, ReadDir, ReadDirInner,
};
use rustix::fs::DirEntry;
use std::ffi::{OsStr, OsString};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::{fmt, fs, io};

pub(crate) struct DirEntryInner {
    pub(super) rustix: DirEntry,
    pub(super) read_dir: ReadDirInner,
}

impl DirEntryInner {
    #[inline]
    pub(crate) fn open(&self, options: &OpenOptions) -> io::Result<fs::File> {
        self.read_dir.open(self.file_name_bytes(), options)
    }

    #[inline]
    pub(crate) fn metadata(&self) -> io::Result<Metadata> {
        self.read_dir.metadata(self.file_name_bytes())
    }

    #[inline]
    pub(crate) fn remove_file(&self) -> io::Result<()> {
        self.read_dir.remove_file(self.file_name_bytes())
    }

    #[inline]
    pub(crate) fn read_dir(&self, follow: FollowSymlinks) -> io::Result<ReadDir> {
        self.read_dir.read_dir(self.file_name_bytes(), follow)
    }

    #[inline]
    pub(crate) fn remove_dir(&self) -> io::Result<()> {
        self.read_dir.remove_dir(self.file_name_bytes())
    }

    #[cfg(not(any(target_os = "illumos", target_os = "solaris")))]
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn file_type(&self) -> io::Result<FileType> {
        use crate::fs::ImplFileTypeExt;

        Ok(match self.rustix.file_type() {
            rustix::fs::FileType::Directory => FileType::dir(),
            rustix::fs::FileType::RegularFile => FileType::file(),
            rustix::fs::FileType::Symlink => FileType::ext(ImplFileTypeExt::symlink()),
            #[cfg(not(target_os = "wasi"))]
            rustix::fs::FileType::Fifo => FileType::ext(ImplFileTypeExt::fifo()),
            #[cfg(not(target_os = "wasi"))]
            rustix::fs::FileType::Socket => FileType::ext(ImplFileTypeExt::socket()),
            rustix::fs::FileType::CharacterDevice => FileType::ext(ImplFileTypeExt::char_device()),
            rustix::fs::FileType::BlockDevice => FileType::ext(ImplFileTypeExt::block_device()),
            rustix::fs::FileType::Unknown => FileType::unknown(),
        })
    }

    #[cfg(any(target_os = "illumos", target_os = "solaris"))]
    #[inline]
    pub(crate) fn file_type(&self) -> io::Result<FileType> {
        // These platforms don't have the file type on the dirent, so we must lstat the file.
        self.metadata().map(|m| m.file_type())
    }

    #[inline]
    pub(crate) fn file_name(&self) -> OsString {
        self.file_name_bytes().to_os_string()
    }

    #[inline]
    pub(crate) fn ino(&self) -> u64 {
        self.rustix.ino()
    }

    #[inline]
    pub(crate) fn is_same_file(&self, metadata: &Metadata) -> io::Result<bool> {
        let self_md = self.metadata()?;
        Ok(self_md.ino() == metadata.ino() && self_md.dev() == metadata.dev())
    }

    fn file_name_bytes(&self) -> &OsStr {
        OsStr::from_bytes(self.rustix.file_name().to_bytes())
    }
}

impl fmt::Debug for DirEntryInner {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.file_name()).finish()
    }
}
