use crate::fs::{FollowSymlinks, ImplOpenOptionsExt};

/// Options and flags which can be used to configure how a file is opened.
///
/// This corresponds to [`std::fs::OpenOptions`].
///
/// This `OpenOptions` has no `open` method. To open a file with an
/// `OptionOptions`, first obtain a [`Dir`] containing the path, and then call
/// [`Dir::open_with`].
///
/// [`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html
/// [`Dir::open_with`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html#method.open_with
///
/// <details>
/// We need to define our own version because the libstd `OpenOptions` doesn't
/// have public accessors that we can use.
/// </details>
#[derive(Debug, Clone)]
pub struct OpenOptions {
    pub(crate) read: bool,
    pub(crate) write: bool,
    pub(crate) append: bool,
    pub(crate) truncate: bool,
    pub(crate) create: bool,
    pub(crate) create_new: bool,
    pub(crate) dir_required: bool,
    pub(crate) maybe_dir: bool,
    pub(crate) sync: bool,
    pub(crate) dsync: bool,
    pub(crate) rsync: bool,
    pub(crate) nonblock: bool,
    pub(crate) readdir_required: bool,
    pub(crate) follow: FollowSymlinks,

    #[cfg(any(unix, windows, target_os = "vxworks"))]
    pub(crate) ext: ImplOpenOptionsExt,
}

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// This corresponds to [`std::fs::OpenOptions::new`].
    #[allow(clippy::new_without_default)]
    #[inline]
    pub const fn new() -> Self {
        Self {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            dir_required: false,
            maybe_dir: false,
            sync: false,
            dsync: false,
            rsync: false,
            nonblock: false,
            readdir_required: false,
            follow: FollowSymlinks::Yes,

            #[cfg(any(unix, windows, target_os = "vxworks"))]
            ext: ImplOpenOptionsExt::new(),
        }
    }

    /// Sets the option for read access.
    ///
    /// This corresponds to [`std::fs::OpenOptions::read`].
    #[inline]
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    ///
    /// This corresponds to [`std::fs::OpenOptions::write`].
    #[inline]
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    /// Sets the option for the append mode.
    ///
    /// This corresponds to [`std::fs::OpenOptions::append`].
    #[inline]
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    /// Sets the option for truncating a previous file.
    ///
    /// This corresponds to [`std::fs::OpenOptions::truncate`].
    #[inline]
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    /// Sets the option to create a new file.
    ///
    /// This corresponds to [`std::fs::OpenOptions::create`].
    #[inline]
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    /// Sets the option to always create a new file.
    ///
    /// This corresponds to [`std::fs::OpenOptions::create_new`].
    #[inline]
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new = create_new;
        self
    }

    /// Sets the option to enable or suppress following of symlinks.
    #[inline]
    pub(crate) fn follow(&mut self, follow: FollowSymlinks) -> &mut Self {
        self.follow = follow;
        self
    }

    /// Sets the option to enable an error if the opened object is not a
    /// directory.
    #[inline]
    pub(crate) fn dir_required(&mut self, dir_required: bool) -> &mut Self {
        self.dir_required = dir_required;
        self
    }

    /// Sets the option to disable an error if the opened object is a
    /// directory.
    #[inline]
    pub(crate) fn maybe_dir(&mut self, maybe_dir: bool) -> &mut Self {
        self.maybe_dir = maybe_dir;
        self
    }

    /// Requests write operations complete as defined by synchronized I/O file
    /// integrity completion.
    #[inline]
    pub(crate) fn sync(&mut self, enable: bool) -> &mut Self {
        self.sync = enable;
        self
    }

    /// Requests write operations complete as defined by synchronized I/O data
    /// integrity completion.
    #[inline]
    pub(crate) fn dsync(&mut self, enable: bool) -> &mut Self {
        self.dsync = enable;
        self
    }

    /// Requests read operations complete as defined by the level of integrity
    /// specified by `sync` and `dsync`.
    #[inline]
    pub(crate) fn rsync(&mut self, enable: bool) -> &mut Self {
        self.rsync = enable;
        self
    }

    /// Requests that I/O operations fail with `std::io::ErrorKind::WouldBlock`
    /// if they would otherwise block.
    ///
    /// This option is commonly not implemented for regular files, so blocking
    /// may still occur.
    #[inline]
    pub(crate) fn nonblock(&mut self, enable: bool) -> &mut Self {
        self.nonblock = enable;
        self
    }

    /// Sets the option to request the ability to read directory entries.
    #[inline]
    pub(crate) fn readdir_required(&mut self, readdir_required: bool) -> &mut Self {
        self.readdir_required = readdir_required;
        self
    }

    /// Wrapper to allow `follow` to be exposed by the `cap-fs-ext` crate.
    ///
    /// This is hidden from the main API since this functionality isn't present
    /// in `std`. Use `cap_fs_ext::OpenOptionsFollowExt` instead of calling
    /// this directly.
    #[doc(hidden)]
    #[inline]
    pub fn _cap_fs_ext_follow(&mut self, follow: FollowSymlinks) -> &mut Self {
        self.follow(follow)
    }

    /// Wrapper to allow `maybe_dir` to be exposed by the `cap-fs-ext` crate.
    ///
    /// This is hidden from the main API since this functionality isn't present
    /// in `std`. Use `cap_fs_ext::OpenOptionsMaybeDirExt` instead of
    /// calling this directly.
    #[doc(hidden)]
    #[inline]
    pub fn _cap_fs_ext_maybe_dir(&mut self, maybe_dir: bool) -> &mut Self {
        self.maybe_dir(maybe_dir)
    }

    /// Wrapper to allow `sync` to be exposed by the `cap-fs-ext` crate.
    ///
    /// This is hidden from the main API since this functionality isn't present
    /// in `std`. Use `cap_fs_ext::OpenOptionsSyncExt` instead of
    /// calling this directly.
    #[doc(hidden)]
    #[inline]
    pub fn _cap_fs_ext_sync(&mut self, enable: bool) -> &mut Self {
        self.sync(enable)
    }

    /// Wrapper to allow `dsync` to be exposed by the `cap-fs-ext` crate.
    ///
    /// This is hidden from the main API since this functionality isn't present
    /// in `std`. Use `cap_fs_ext::OpenOptionsSyncExt` instead of
    /// calling this directly.
    #[doc(hidden)]
    #[inline]
    pub fn _cap_fs_ext_dsync(&mut self, enable: bool) -> &mut Self {
        self.dsync(enable)
    }

    /// Wrapper to allow `rsync` to be exposed by the `cap-fs-ext` crate.
    ///
    /// This is hidden from the main API since this functionality isn't present
    /// in `std`. Use `cap_fs_ext::OpenOptionsSyncExt` instead of
    /// calling this directly.
    #[doc(hidden)]
    #[inline]
    pub fn _cap_fs_ext_rsync(&mut self, enable: bool) -> &mut Self {
        self.rsync(enable)
    }

    /// Wrapper to allow `nonblock` to be exposed by the `cap-fs-ext` crate.
    ///
    /// This is hidden from the main API since this functionality isn't present
    /// in `std`. Use `cap_fs_ext::OpenOptionsSyncExt` instead of
    /// calling this directly.
    #[doc(hidden)]
    #[inline]
    pub fn _cap_fs_ext_nonblock(&mut self, enable: bool) -> &mut Self {
        self.nonblock(enable)
    }
}

/// Unix-specific extensions to [`fs::OpenOptions`].
#[cfg(unix)]
pub trait OpenOptionsExt {
    /// Sets the mode bits that a new file will be created with.
    fn mode(&mut self, mode: u32) -> &mut Self;

    /// Pass custom flags to the `flags` argument of `open`.
    fn custom_flags(&mut self, flags: i32) -> &mut Self;
}

/// WASI-specific extensions to [`fs::OpenOptions`].
#[cfg(target_os = "wasi")]
pub trait OpenOptionsExt {
    /// Pass custom `dirflags` argument to `path_open`.
    fn lookup_flags(&mut self, flags: u32) -> &mut Self;

    /// Indicates whether `OpenOptions` must open a directory or not.
    fn directory(&mut self, dir: bool) -> &mut Self;

    /// Indicates whether `__WASI_FDFLAG_DSYNC` is passed in the `fs_flags`
    /// field of `path_open`.
    fn dsync(&mut self, dsync: bool) -> &mut Self;

    /// Indicates whether `__WASI_FDFLAG_NONBLOCK` is passed in the `fs_flags`
    /// field of `path_open`.
    fn nonblock(&mut self, nonblock: bool) -> &mut Self;

    /// Indicates whether `__WASI_FDFLAG_RSYNC` is passed in the `fs_flags`
    /// field of `path_open`.
    fn rsync(&mut self, rsync: bool) -> &mut Self;

    /// Indicates whether `__WASI_FDFLAG_SYNC` is passed in the `fs_flags`
    /// field of `path_open`.
    fn sync(&mut self, sync: bool) -> &mut Self;

    /// Indicates the value that should be passed in for the `fs_rights_base`
    /// parameter of `path_open`.
    fn fs_rights_base(&mut self, rights: u64) -> &mut Self;

    /// Indicates the value that should be passed in for the
    /// `fs_rights_inheriting` parameter of `path_open`.
    fn fs_rights_inheriting(&mut self, rights: u64) -> &mut Self;

    /// Open a file or directory.
    fn open_at<P: AsRef<std::path::Path>>(
        &self,
        file: &std::fs::File,
        path: P,
    ) -> std::io::Result<std::fs::File>;
}

/// Windows-specific extensions to [`fs::OpenOptions`].
#[cfg(windows)]
pub trait OpenOptionsExt {
    /// Overrides the `dwDesiredAccess` argument to the call to [`CreateFile`]
    /// with the specified value.
    fn access_mode(&mut self, access: u32) -> &mut Self;

    /// Overrides the `dwShareMode` argument to the call to [`CreateFile`] with
    /// the specified value.
    fn share_mode(&mut self, val: u32) -> &mut Self;

    /// Sets extra flags for the `dwFileFlags` argument to the call to
    /// [`CreateFile2`] to the specified value (or combines it with
    /// `attributes` and `security_qos_flags` to set the `dwFlagsAndAttributes`
    /// for [`CreateFile`]).
    ///
    /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    /// [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2
    fn custom_flags(&mut self, flags: u32) -> &mut Self;

    /// Sets the `dwFileAttributes` argument to the call to [`CreateFile2`] to
    /// the specified value (or combines it with `custom_flags` and
    /// `security_qos_flags` to set the `dwFlagsAndAttributes` for
    /// [`CreateFile`]).
    ///
    /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    /// [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2
    fn attributes(&mut self, val: u32) -> &mut Self;

    /// Sets the `dwSecurityQosFlags` argument to the call to [`CreateFile2`] to
    /// the specified value (or combines it with `custom_flags` and `attributes`
    /// to set the `dwFlagsAndAttributes` for [`CreateFile`]).
    ///
    /// [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    /// [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2
    /// [Impersonation Levels]:
    ///     https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level
    fn security_qos_flags(&mut self, flags: u32) -> &mut Self;
}

#[cfg(unix)]
impl OpenOptionsExt for OpenOptions {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.ext.mode(mode);
        self
    }

    #[inline]
    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.ext.custom_flags(flags);
        self
    }
}

#[cfg(target_os = "wasi")]
impl OpenOptionsExt for OpenOptions {
    fn lookup_flags(&mut self, _: u32) -> &mut Self {
        todo!()
    }

    fn directory(&mut self, dir_required: bool) -> &mut Self {
        self.dir_required = dir_required;
        self
    }

    fn dsync(&mut self, _: bool) -> &mut Self {
        todo!()
    }

    fn nonblock(&mut self, _: bool) -> &mut Self {
        todo!()
    }

    fn rsync(&mut self, _: bool) -> &mut Self {
        todo!()
    }

    fn sync(&mut self, _: bool) -> &mut Self {
        todo!()
    }

    fn fs_rights_base(&mut self, _: u64) -> &mut Self {
        todo!()
    }

    fn fs_rights_inheriting(&mut self, _: u64) -> &mut Self {
        todo!()
    }

    fn open_at<P>(&self, dirfd: &std::fs::File, path: P) -> Result<std::fs::File, std::io::Error>
    where
        P: AsRef<std::path::Path>,
    {
        crate::fs::open(dirfd, path.as_ref(), self)
    }
}

#[cfg(target_os = "vxworks")]
impl OpenOptionsExt for OpenOptions {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.ext.mode(mode);
        self
    }

    #[inline]
    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.ext.custom_flags(flags);
        self
    }
}

#[cfg(windows)]
impl OpenOptionsExt for OpenOptions {
    #[inline]
    fn access_mode(&mut self, access: u32) -> &mut Self {
        self.ext.access_mode(access);
        self
    }

    /// To prevent race conditions on Windows, handles for directories must be
    /// opened without `FILE_SHARE_DELETE`.
    #[inline]
    fn share_mode(&mut self, val: u32) -> &mut Self {
        self.ext.share_mode(val);
        self
    }

    #[inline]
    fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.ext.custom_flags(flags);
        self
    }

    #[inline]
    fn attributes(&mut self, val: u32) -> &mut Self {
        self.ext.attributes(val);
        self
    }

    #[inline]
    fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.ext.security_qos_flags(flags);
        self
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for OpenOptions {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        use arbitrary::Arbitrary;
        let (read, write) = match u.int_in_range(0..=2)? {
            0 => (true, false),
            1 => (false, true),
            2 => (true, true),
            _ => panic!(),
        };
        // TODO: `OpenOptionsExt` options.
        Ok(Self::new()
            .read(read)
            .write(write)
            .create(<bool as Arbitrary>::arbitrary(u)?)
            .append(<bool as Arbitrary>::arbitrary(u)?)
            .truncate(<bool as Arbitrary>::arbitrary(u)?)
            .create(<bool as Arbitrary>::arbitrary(u)?)
            .create_new(<bool as Arbitrary>::arbitrary(u)?)
            .dir_required(<bool as Arbitrary>::arbitrary(u)?)
            .maybe_dir(<bool as Arbitrary>::arbitrary(u)?)
            .sync(<bool as Arbitrary>::arbitrary(u)?)
            .dsync(<bool as Arbitrary>::arbitrary(u)?)
            .rsync(<bool as Arbitrary>::arbitrary(u)?)
            .nonblock(<bool as Arbitrary>::arbitrary(u)?)
            .readdir_required(<bool as Arbitrary>::arbitrary(u)?)
            .follow(<FollowSymlinks as Arbitrary>::arbitrary(u)?)
            .clone())
    }
}
