#[cfg(feature = "fs_utf8")]
use camino::Utf8Path;
#[cfg(all(windows, feature = "async_std", feature = "fs_utf8"))]
use cap_primitives::fs::stat;
#[cfg(not(windows))]
use cap_primitives::fs::symlink;
use cap_primitives::fs::{
    access, open_dir_nofollow, set_symlink_permissions, set_times, set_times_nofollow,
    FollowSymlinks, Permissions,
};
#[cfg(windows)]
use cap_primitives::fs::{symlink_dir, symlink_file};
use io_lifetimes::AsFilelike;
use std::io;
use std::path::Path;
#[cfg(feature = "async_std")]
use {async_std::task::spawn_blocking, async_trait::async_trait};

pub use cap_primitives::fs::{AccessType, SystemTimeSpec};

/// Extension trait for `Dir`.
pub trait DirExt {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_atime.html
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_times.html
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    /// This function does not follow symlink.
    ///
    /// This corresponds to [`filetime::set_symlink_file_times`].
    ///
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_symlink_file_times.html
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], except that it's
    /// supported on non-Unix platforms as well, and it's not guaranteed to be
    /// atomic.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a file.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new directory symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a directory.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Similar to `cap_std::fs::Dir::open_dir`, but fails if the path names a
    /// symlink.
    fn open_dir_nofollow<P: AsRef<Path>>(&self, path: P) -> io::Result<Self>
    where
        Self: Sized;

    /// Removes a file or symlink from a filesystem.
    ///
    /// Removal of symlinks has different behavior under Windows - if a symlink
    /// points to a directory, it cannot be removed with the `remove_file`
    /// operation. This method will remove files and all symlinks.
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object.
    fn access<P: AsRef<Path>>(&self, path: P, type_: AccessType) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object, without
    /// following symbolic links.
    fn access_symlink<P: AsRef<Path>>(&self, path: P, type_: AccessType) -> io::Result<()>;

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    fn set_symlink_permissions<P: AsRef<Path>>(&self, path: P, perm: Permissions)
        -> io::Result<()>;
}

/// Extension trait for `Dir`, async.
///
/// The path parameters include `Send` for the `async_trait` macro.
#[cfg(feature = "async_std")]
#[async_trait]
pub trait AsyncDirExt {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_atime.html
    async fn set_atime<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        atime: SystemTimeSpec,
    ) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    async fn set_mtime<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        mtime: SystemTimeSpec,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_times.html
    async fn set_times<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    /// This function does not follow symlink.
    ///
    /// This corresponds to [`filetime::set_symlink_file_times`].
    ///
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_symlink_file_times.html
    async fn set_symlink_times<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], except that
    /// it's supported on non-Unix platforms as well, and it's not guaranteed
    /// to be atomic.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    async fn symlink<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a file.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    async fn symlink_file<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Creates a new directory symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a directory.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    async fn symlink_dir<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Similar to `cap_std::fs::Dir::open_dir`, but fails if the path names a
    /// symlink.
    async fn open_dir_nofollow<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
    ) -> io::Result<Self>
    where
        Self: Sized;

    /// Removes a file or symlink from a filesystem.
    ///
    /// Removal of symlinks has different behavior under Windows - if a symlink
    /// points to a directory, it cannot be removed with the `remove_file`
    /// operation. This method will remove files and all symlinks.
    async fn remove_file_or_symlink<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
    ) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object.
    async fn access<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object, without
    /// following symbolic links.
    async fn access_symlink<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()>;

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    async fn set_symlink_permissions<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()>;
}

/// `fs_utf8` version of `DirExt`.
#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
pub trait DirExtUtf8 {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_atime.html
    fn set_atime<P: AsRef<Utf8Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    fn set_mtime<P: AsRef<Utf8Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_times.html
    fn set_times<P: AsRef<Utf8Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    /// This function does not follow symlink.
    ///
    /// This corresponds to [`filetime::set_symlink_file_times`].
    ///
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_symlink_file_times.html
    fn set_symlink_times<P: AsRef<Utf8Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], except that it's
    /// supported on non-Unix platforms as well, and it's not guaranteed to be
    /// atomic.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    fn symlink<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(&self, src: P, dst: Q) -> io::Result<()>;

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a file.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    fn symlink_file<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Creates a new directory symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a directory.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    fn symlink_dir<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(&self, src: P, dst: Q)
        -> io::Result<()>;

    /// Similar to `cap_std::fs::Dir::open_dir`, but fails if the path names a
    /// symlink.
    fn open_dir_nofollow<P: AsRef<Utf8Path>>(&self, path: P) -> io::Result<Self>
    where
        Self: Sized;

    /// Removes a file or symlink from a filesystem.
    ///
    /// This is similar to [`std::fs::remove_file`], except that it also works
    /// on symlinks to directories on Windows, similar to how `unlink` works
    /// on symlinks to directories on Posix-ish platforms.
    fn remove_file_or_symlink<P: AsRef<Utf8Path>>(&self, path: P) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object.
    fn access<P: AsRef<Utf8Path>>(&self, path: P, type_: AccessType) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object, without
    /// following symbolic links.
    fn access_symlink<P: AsRef<Utf8Path>>(&self, path: P, type_: AccessType) -> io::Result<()>;

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    fn set_symlink_permissions<P: AsRef<Utf8Path>>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()>;
}

/// `fs_utf8` version of `DirExt`.
///
/// The path parameters include `Send` for the `async_trait` macro.
#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
#[async_trait]
pub trait AsyncDirExtUtf8 {
    /// Set the last access time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_atime`].
    ///
    /// [`filetime::set_file_atime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_atime.html
    async fn set_atime<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        atime: SystemTimeSpec,
    ) -> io::Result<()>;

    /// Set the last modification time for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_mtime`].
    ///
    /// [`filetime::set_file_mtime`]: https://docs.rs/filetime/latest/filetime/fn.set_file_mtime.html
    async fn set_mtime<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        mtime: SystemTimeSpec,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    ///
    /// This corresponds to [`filetime::set_file_times`].
    ///
    /// [`filetime::set_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_file_times.html
    async fn set_times<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Set the last access and modification times for a file on a filesystem.
    /// This function does not follow symlink.
    ///
    /// This corresponds to [`filetime::set_symlink_file_times`].
    ///
    /// [`filetime::set_symlink_file_times`]: https://docs.rs/filetime/latest/filetime/fn.set_symlink_file_times.html
    async fn set_symlink_times<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()>;

    /// Creates a new symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], except that
    /// it's supported on non-Unix platforms as well, and it's not guaranteed
    /// to be atomic.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    async fn symlink<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a file.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    async fn symlink_file<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Creates a new directory symbolic link on a filesystem.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], except that
    /// it's supported on non-Windows platforms as well, and it's not
    /// guaranteed to fail if the target is not a directory.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    async fn symlink_dir<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()>;

    /// Similar to `cap_std::fs::Dir::open_dir`, but fails if the path names a
    /// symlink.
    async fn open_dir_nofollow<P: AsRef<Utf8Path> + Send>(&self, path: P) -> io::Result<Self>
    where
        Self: Sized;

    /// Removes a file or symlink from a filesystem.
    ///
    /// Removal of symlinks has different behavior under Windows - if a symlink
    /// points to a directory, it cannot be removed with the `remove_file`
    /// operation. This method will remove files and all symlinks.
    async fn remove_file_or_symlink<P: AsRef<Utf8Path> + Send>(&self, path: P) -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object.
    async fn access<P: AsRef<Utf8Path> + Send>(&self, path: P, type_: AccessType)
        -> io::Result<()>;

    /// Test for accessibility or existence of a filesystem object, without
    /// following symbolic links.
    async fn access_symlink<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()>;

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    async fn set_symlink_permissions<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()>;
}

#[cfg(feature = "std")]
impl DirExt for cap_std::fs::Dir {
    #[inline]
    fn set_atime<P: AsRef<Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            Some(atime),
            None,
        )
    }

    #[inline]
    fn set_mtime<P: AsRef<Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        set_times(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            None,
            Some(mtime),
        )
    }

    #[inline]
    fn set_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            atime,
            mtime,
        )
    }

    #[inline]
    fn set_symlink_times<P: AsRef<Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        set_times_nofollow(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            atime,
            mtime,
        )
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink(
            src.as_ref(),
            &self.as_filelike_view::<std::fs::File>(),
            dst.as_ref(),
        )
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        self.symlink(src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            self.symlink_dir(src, dst)
        } else {
            self.symlink_file(src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_file(
            src.as_ref(),
            &self.as_filelike_view::<std::fs::File>(),
            dst.as_ref(),
        )
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        symlink_dir(
            src.as_ref(),
            &self.as_filelike_view::<std::fs::File>(),
            dst.as_ref(),
        )
    }

    #[inline]
    fn open_dir_nofollow<P: AsRef<Path>>(&self, path: P) -> io::Result<Self> {
        match open_dir_nofollow(&self.as_filelike_view::<std::fs::File>(), path.as_ref()) {
            Ok(file) => Ok(Self::from_std_file(file)),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        use crate::OpenOptionsFollowExt;
        use cap_primitives::fs::_WindowsByHandle;
        use cap_std::fs::{OpenOptions, OpenOptionsExt};
        use windows_sys::Win32::Storage::FileSystem::{
            DELETE, FILE_ATTRIBUTE_DIRECTORY, FILE_FLAG_BACKUP_SEMANTICS,
            FILE_FLAG_OPEN_REPARSE_POINT,
        };
        let path = path.as_ref();

        let mut opts = OpenOptions::new();
        opts.access_mode(DELETE);
        opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
        opts.follow(FollowSymlinks::No);
        let file = self.open_with(path, &opts)?;

        let meta = file.metadata()?;
        if meta.file_type().is_symlink()
            && meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY
        {
            self.remove_dir(path)?;
        } else {
            self.remove_file(path)?;
        }

        // Drop the file after calling `remove_file` or `remove_dir`, since
        // Windows doesn't actually remove the file until after the last open
        // handle is closed, and this protects us from race conditions where
        // other processes replace the file out from underneath us.
        drop(file);

        Ok(())
    }

    /// Test for accessibility or existence of a filesystem object.
    fn access<P: AsRef<Path>>(&self, path: P, type_: AccessType) -> io::Result<()> {
        access(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            type_,
            FollowSymlinks::Yes,
        )
    }

    /// Test for accessibility or existence of a filesystem object.
    fn access_symlink<P: AsRef<Path>>(&self, path: P, type_: AccessType) -> io::Result<()> {
        access(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            type_,
            FollowSymlinks::No,
        )
    }

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    fn set_symlink_permissions<P: AsRef<Path>>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        set_symlink_permissions(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref(),
            perm,
        )
    }
}

#[cfg(feature = "async_std")]
#[async_trait]
impl AsyncDirExt for cap_async_std::fs::Dir {
    #[inline]
    async fn set_atime<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        atime: SystemTimeSpec,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            set_times(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                Some(atime),
                None,
            )
        })
        .await
    }

    #[inline]
    async fn set_mtime<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        mtime: SystemTimeSpec,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            set_times(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                None,
                Some(mtime),
            )
        })
        .await
    }

    #[inline]
    async fn set_times<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            set_times(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                atime,
                mtime,
            )
        })
        .await
    }

    #[inline]
    async fn set_symlink_times<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            set_times_nofollow(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                atime,
                mtime,
            )
        })
        .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn symlink<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn symlink_file<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn symlink_dir<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    #[cfg(windows)]
    #[inline]
    async fn symlink<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        if self.metadata(&src).await?.is_dir() {
            spawn_blocking(move || {
                symlink_dir(
                    src.as_ref(),
                    &clone.as_filelike_view::<std::fs::File>(),
                    dst.as_ref(),
                )
            })
            .await
        } else {
            spawn_blocking(move || {
                symlink_file(
                    src.as_ref(),
                    &clone.as_filelike_view::<std::fs::File>(),
                    dst.as_ref(),
                )
            })
            .await
        }
    }

    #[cfg(windows)]
    #[inline]
    async fn symlink_file<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink_file(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    #[cfg(windows)]
    #[inline]
    async fn symlink_dir<
        P: AsRef<async_std::path::Path> + Send,
        Q: AsRef<async_std::path::Path> + Send,
    >(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = src.as_ref().to_path_buf();
        let dst = dst.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            symlink_dir(
                src.as_ref(),
                &clone.as_filelike_view::<std::fs::File>(),
                dst.as_ref(),
            )
        })
        .await
    }

    #[inline]
    async fn open_dir_nofollow<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
    ) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            match open_dir_nofollow(&clone.as_filelike_view::<std::fs::File>(), path.as_ref()) {
                Ok(file) => Ok(Self::from_std_file(file.into())),
                Err(e) => Err(e),
            }
        })
        .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn remove_file_or_symlink<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
    ) -> io::Result<()> {
        self.remove_file(path).await
    }

    #[cfg(windows)]
    #[inline]
    async fn remove_file_or_symlink<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
    ) -> io::Result<()> {
        use crate::OpenOptionsFollowExt;
        use cap_primitives::fs::_WindowsByHandle;
        use cap_std::fs::{OpenOptions, OpenOptionsExt};
        use windows_sys::Win32::Storage::FileSystem::{
            DELETE, FILE_ATTRIBUTE_DIRECTORY, FILE_FLAG_BACKUP_SEMANTICS,
            FILE_FLAG_OPEN_REPARSE_POINT,
        };
        let path = path.as_ref();

        let mut opts = OpenOptions::new();
        opts.access_mode(DELETE);
        opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
        opts.follow(FollowSymlinks::No);
        let file = self.open_with(path, &opts).await?;

        let meta = file.metadata().await?;
        if meta.file_type().is_symlink()
            && meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY
        {
            self.remove_dir(path).await?;
        } else {
            self.remove_file(path).await?;
        }

        // Drop the file after calling `remove_file` or `remove_dir`, since
        // Windows doesn't actually remove the file until after the last open
        // handle is closed, and this protects us from race conditions where
        // other processes replace the file out from underneath us.
        drop(file);

        Ok(())
    }

    /// Test for accessibility or existence of a filesystem object.
    async fn access<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            access(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                type_,
                FollowSymlinks::Yes,
            )
        })
        .await
    }

    /// Test for accessibility or existence of a filesystem object, without
    /// following symbolic links.
    async fn access_symlink<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            access(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                type_,
                FollowSymlinks::No,
            )
        })
        .await
    }

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    async fn set_symlink_permissions<P: AsRef<async_std::path::Path> + Send>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let clone = self.clone();
        spawn_blocking(move || {
            set_symlink_permissions(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                perm,
            )
        })
        .await
    }
}

#[cfg(all(feature = "std", feature = "fs_utf8"))]
impl DirExtUtf8 for cap_std::fs_utf8::Dir {
    #[inline]
    fn set_atime<P: AsRef<Utf8Path>>(&self, path: P, atime: SystemTimeSpec) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        set_times(
            &self.as_filelike_view::<std::fs::File>(),
            &path,
            Some(atime),
            None,
        )
    }

    #[inline]
    fn set_mtime<P: AsRef<Utf8Path>>(&self, path: P, mtime: SystemTimeSpec) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        set_times(
            &self.as_filelike_view::<std::fs::File>(),
            &path,
            None,
            Some(mtime),
        )
    }

    #[inline]
    fn set_times<P: AsRef<Utf8Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        set_times(
            &self.as_filelike_view::<std::fs::File>(),
            &path,
            atime,
            mtime,
        )
    }

    #[inline]
    fn set_symlink_times<P: AsRef<Utf8Path>>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        set_times_nofollow(
            &self.as_filelike_view::<std::fs::File>(),
            &path,
            atime,
            mtime,
        )
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_file<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(not(windows))]
    #[inline]
    fn symlink_dir<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        Self::symlink(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if self.metadata(src.as_ref())?.is_dir() {
            Self::symlink_dir(self, src, dst)
        } else {
            Self::symlink_file(self, src, dst)
        }
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_file<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        Self::symlink_file(self, src, dst)
    }

    #[cfg(windows)]
    #[inline]
    fn symlink_dir<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        Self::symlink_dir(self, src, dst)
    }

    #[inline]
    fn open_dir_nofollow<P: AsRef<Utf8Path>>(&self, path: P) -> io::Result<Self> {
        match open_dir_nofollow(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref().as_ref(),
        ) {
            Ok(file) => Ok(Self::from_std_file(file)),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Utf8Path>>(&self, path: P) -> io::Result<()> {
        self.remove_file(path.as_ref())
    }

    #[cfg(windows)]
    #[inline]
    fn remove_file_or_symlink<P: AsRef<Utf8Path>>(&self, path: P) -> io::Result<()> {
        use crate::OpenOptionsFollowExt;
        use cap_primitives::fs::_WindowsByHandle;
        use cap_std::fs::{OpenOptions, OpenOptionsExt};
        use windows_sys::Win32::Storage::FileSystem::{
            DELETE, FILE_ATTRIBUTE_DIRECTORY, FILE_FLAG_BACKUP_SEMANTICS,
            FILE_FLAG_OPEN_REPARSE_POINT,
        };
        let path = path.as_ref();

        let mut opts = OpenOptions::new();
        opts.access_mode(DELETE);
        opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
        opts.follow(FollowSymlinks::No);
        let file = self.open_with(path, &opts)?;

        let meta = file.metadata()?;
        if meta.file_type().is_symlink()
            && meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY
        {
            self.remove_dir(path)?;
        } else {
            self.remove_file(path)?;
        }

        // Drop the file after calling `remove_file` or `remove_dir`, since
        // Windows doesn't actually remove the file until after the last open
        // handle is closed, and this protects us from race conditions where
        // other processes replace the file out from underneath us.
        drop(file);

        Ok(())
    }

    /// Test for accessibility or existence of a filesystem object.
    fn access<P: AsRef<Utf8Path>>(&self, path: P, type_: AccessType) -> io::Result<()> {
        access(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref().as_ref(),
            type_,
            FollowSymlinks::Yes,
        )
    }

    /// Test for accessibility or existence of a filesystem object.
    fn access_symlink<P: AsRef<Utf8Path>>(&self, path: P, type_: AccessType) -> io::Result<()> {
        access(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref().as_ref(),
            type_,
            FollowSymlinks::No,
        )
    }

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    fn set_symlink_permissions<P: AsRef<Utf8Path>>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        set_symlink_permissions(
            &self.as_filelike_view::<std::fs::File>(),
            path.as_ref().as_ref(),
            perm,
        )
    }
}

#[cfg(all(feature = "async_std", feature = "fs_utf8"))]
#[async_trait]
impl AsyncDirExtUtf8 for cap_async_std::fs_utf8::Dir {
    #[inline]
    async fn set_atime<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        atime: SystemTimeSpec,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            set_times(
                &clone.as_filelike_view::<std::fs::File>(),
                &path,
                Some(atime),
                None,
            )
        })
        .await
    }

    #[inline]
    async fn set_mtime<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        mtime: SystemTimeSpec,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            set_times(
                &clone.as_filelike_view::<std::fs::File>(),
                &path,
                None,
                Some(mtime),
            )
        })
        .await
    }

    #[inline]
    async fn set_times<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            set_times(
                &clone.as_filelike_view::<std::fs::File>(),
                &path,
                atime,
                mtime,
            )
        })
        .await
    }

    #[inline]
    async fn set_symlink_times<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            set_times_nofollow(
                &clone.as_filelike_view::<std::fs::File>(),
                &path,
                atime,
                mtime,
            )
        })
        .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn symlink<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = from_utf8(src.as_ref())?;
        let dst = from_utf8(dst.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || symlink(&src, &clone.as_filelike_view::<std::fs::File>(), &dst))
            .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn symlink_file<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        self.symlink(src, dst).await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn symlink_dir<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        self.symlink(src, dst).await
    }

    #[cfg(windows)]
    #[inline]
    async fn symlink<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = from_utf8(src.as_ref())?;
        let src_ = src.clone();
        let dst = from_utf8(dst.as_ref())?;
        let clone = self.clone();
        // Call `stat` directly to avoid `async_trait` capturing `self`.
        let metadata = spawn_blocking(move || {
            stat(
                &clone.as_filelike_view::<std::fs::File>(),
                &src_,
                FollowSymlinks::Yes,
            )
        })
        .await?;
        let clone = self.clone();
        if metadata.is_dir() {
            spawn_blocking(move || {
                symlink_dir(&src, &clone.as_filelike_view::<std::fs::File>(), &dst)
            })
            .await
        } else {
            spawn_blocking(move || {
                symlink_file(&src, &clone.as_filelike_view::<std::fs::File>(), &dst)
            })
            .await
        }
    }

    #[cfg(windows)]
    #[inline]
    async fn symlink_file<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = from_utf8(src.as_ref())?;
        let dst = from_utf8(dst.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || symlink_file(&src, &clone.as_filelike_view::<std::fs::File>(), &dst))
            .await
    }

    #[cfg(windows)]
    #[inline]
    async fn symlink_dir<P: AsRef<Utf8Path> + Send, Q: AsRef<Utf8Path> + Send>(
        &self,
        src: P,
        dst: Q,
    ) -> io::Result<()> {
        let src = from_utf8(src.as_ref())?;
        let dst = from_utf8(dst.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || symlink_dir(&src, &clone.as_filelike_view::<std::fs::File>(), &dst))
            .await
    }

    #[inline]
    async fn open_dir_nofollow<P: AsRef<Utf8Path> + Send>(&self, path: P) -> io::Result<Self> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            match open_dir_nofollow(&clone.as_filelike_view::<std::fs::File>(), path.as_ref()) {
                Ok(file) => Ok(Self::from_std_file(file.into())),
                Err(e) => Err(e),
            }
        })
        .await
    }

    #[cfg(not(windows))]
    #[inline]
    async fn remove_file_or_symlink<P: AsRef<Utf8Path> + Send>(&self, path: P) -> io::Result<()> {
        self.remove_file(path).await
    }

    #[cfg(windows)]
    #[inline]
    async fn remove_file_or_symlink<P: AsRef<Utf8Path> + Send>(&self, path: P) -> io::Result<()> {
        use crate::{FollowSymlinks, OpenOptionsFollowExt};
        use cap_primitives::fs::_WindowsByHandle;
        use cap_std::fs::{OpenOptions, OpenOptionsExt};
        use windows_sys::Win32::Storage::FileSystem::{
            DELETE, FILE_ATTRIBUTE_DIRECTORY, FILE_FLAG_BACKUP_SEMANTICS,
            FILE_FLAG_OPEN_REPARSE_POINT,
        };
        let path = path.as_ref();

        let mut opts = OpenOptions::new();
        opts.access_mode(DELETE);
        opts.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS);
        opts.follow(FollowSymlinks::No);
        let file = self.open_with(path, &opts).await?;

        let meta = file.metadata().await?;
        if meta.file_type().is_symlink()
            && meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY
        {
            self.remove_dir(path).await?;
        } else {
            self.remove_file(path).await?;
        }

        // Drop the file after calling `remove_file` or `remove_dir`, since
        // Windows doesn't actually remove the file until after the last open
        // handle is closed, and this protects us from race conditions where
        // other processes replace the file out from underneath us.
        drop(file);

        Ok(())
    }

    async fn access<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            access(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                type_,
                FollowSymlinks::Yes,
            )
        })
        .await
    }

    async fn access_symlink<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        type_: AccessType,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            access(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                type_,
                FollowSymlinks::No,
            )
        })
        .await
    }

    /// Changes the permissions found on a file or a directory, without following
    /// symbolic links.
    async fn set_symlink_permissions<P: AsRef<Utf8Path> + Send>(
        &self,
        path: P,
        perm: Permissions,
    ) -> io::Result<()> {
        let path = from_utf8(path.as_ref())?;
        let clone = self.clone();
        spawn_blocking(move || {
            set_symlink_permissions(
                &clone.as_filelike_view::<std::fs::File>(),
                path.as_ref(),
                perm,
            )
        })
        .await
    }
}

#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
#[cfg(not(feature = "arf_strings"))]
fn from_utf8<'a>(path: &'a Utf8Path) -> io::Result<&'a std::path::Path> {
    Ok(path.as_std_path())
}

#[cfg(all(any(feature = "std", feature = "async_std"), feature = "fs_utf8"))]
#[cfg(feature = "arf_strings")]
fn from_utf8<'a>(path: &'a Utf8Path) -> io::Result<std::path::PathBuf> {
    #[cfg(not(windows))]
    let path = {
        #[cfg(unix)]
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        #[cfg(target_os = "wasi")]
        use std::{ffi::OsString, os::wasi::ffi::OsStringExt};

        let string = arf_strings::str_to_host(path.as_str())?;
        OsString::from_vec(string.into_bytes())
    };

    #[cfg(windows)]
    let path = arf_strings::str_to_host(path.as_str())?;

    Ok(path.into())
}
