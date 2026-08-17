#[cfg(target_os = "wasi")]
use crate::fs::OpenOptionsExt;
use crate::fs::{DirBuilder, File, Metadata, OpenOptions, ReadDir};
#[cfg(feature = "fs_utf8")]
use crate::fs_utf8::Dir as DirUtf8;
#[cfg(unix)]
use crate::os::unix::net::{UnixDatagram, UnixListener, UnixStream};
#[cfg(not(target_os = "wasi"))]
use cap_primitives::fs::set_permissions;
use cap_primitives::fs::{
    canonicalize, copy, create_dir, hard_link, open, open_ambient_dir, open_dir, open_parent_dir,
    read_base_dir, read_dir, read_link, read_link_contents, remove_dir, remove_dir_all,
    remove_file, remove_open_dir, remove_open_dir_all, rename, stat, DirOptions, FollowSymlinks,
    Permissions,
};
use cap_primitives::AmbientAuthority;
use io_lifetimes::AsFilelike;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, OwnedHandle};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::{fmt, fs};
#[cfg(not(windows))]
use {
    cap_primitives::fs::{symlink, symlink_contents},
    io_extras::os::rustix::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
};
#[cfg(windows)]
use {
    cap_primitives::fs::{symlink_dir, symlink_file},
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, IntoRawHandleOrSocket,
        OwnedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle},
};

/// A reference to an open directory on a filesystem.
///
/// This does not directly correspond to anything in `std`, however its methods
/// correspond to the [functions in `std::fs`] and the constructor methods for
/// [`std::fs::File`].
///
/// Unlike `std::fs`, this API's `canonicalize` returns a relative path since
/// absolute paths don't interoperate well with the capability model.
///
/// [functions in `std::fs`]: https://doc.rust-lang.org/std/fs/index.html#functions
pub struct Dir {
    std_file: fs::File,
}

impl Dir {
    /// Constructs a new instance of `Self` from the given [`std::fs::File`].
    ///
    /// To prevent race conditions on Windows, the file must be opened without
    /// `FILE_SHARE_DELETE`.
    ///
    /// This grants access the resources the `std::fs::File` instance already
    /// has access to.
    #[inline]
    pub fn from_std_file(std_file: fs::File) -> Self {
        Self { std_file }
    }

    /// Consumes `self` and returns a [`std::fs::File`].
    #[inline]
    pub fn into_std_file(self) -> fs::File {
        self.std_file
    }

    /// Attempts to open a file in read-only mode.
    ///
    /// This corresponds to [`std::fs::File::open`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self.open_with(path, OpenOptions::new().read(true))
    }

    /// Opens a file at `path` with the options specified by `options`.
    ///
    /// This corresponds to [`std::fs::OpenOptions::open`].
    ///
    /// Instead of being a method on `OpenOptions`, this is a method on `Dir`,
    /// and it only accesses paths relative to `self`.
    #[inline]
    pub fn open_with<P: AsRef<Path>>(&self, path: P, options: &OpenOptions) -> io::Result<File> {
        self._open_with(path.as_ref(), options)
    }

    #[cfg(not(target_os = "wasi"))]
    #[inline]
    fn _open_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        let dir = open(&self.std_file, path, options)?;
        Ok(File::from_std(dir))
    }

    #[cfg(target_os = "wasi")]
    #[inline]
    fn _open_with(&self, path: &Path, options: &OpenOptions) -> io::Result<File> {
        let dir = options.open_at(&self.std_file, path)?;
        Ok(File::from_std(dir))
    }

    /// Attempts to open a directory.
    #[inline]
    pub fn open_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<Self> {
        let dir = open_dir(&self.std_file, path.as_ref())?;
        Ok(Self::from_std_file(dir))
    }

    /// Attempts to open a directory, verifying UTF-8.
    ///
    /// This is equivalent to [`crate::fs_utf8::Dir::open_dir`].
    #[inline]
    #[cfg(feature = "fs_utf8")]
    pub fn open_dir_utf8<P: AsRef<camino::Utf8Path>>(&self, path: P) -> io::Result<DirUtf8> {
        let path = crate::fs_utf8::from_utf8(path.as_ref())?;
        self.open_dir(path).map(DirUtf8::from_cap_std)
    }

    /// Creates a new, empty directory at the provided path.
    ///
    /// This corresponds to [`std::fs::create_dir`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self._create_dir_one(path.as_ref(), &DirOptions::new())
    }

    /// Recursively create a directory and all of its parent components if they
    /// are missing.
    ///
    /// This corresponds to [`std::fs::create_dir_all`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        self._create_dir_all(path.as_ref(), &DirOptions::new())
    }

    /// Creates the specified directory with the options configured in this
    /// builder.
    ///
    /// This corresponds to [`std::fs::DirBuilder::create`].
    #[cfg(not(target_os = "wasi"))]
    #[inline]
    pub fn create_dir_with<P: AsRef<Path>>(
        &self,
        path: P,
        dir_builder: &DirBuilder,
    ) -> io::Result<()> {
        let options = dir_builder.options();
        if dir_builder.is_recursive() {
            self._create_dir_all(path.as_ref(), options)
        } else {
            self._create_dir_one(path.as_ref(), options)
        }
    }

    fn _create_dir_one(&self, path: &Path, dir_options: &DirOptions) -> io::Result<()> {
        create_dir(&self.std_file, path, dir_options)
    }

    fn _create_dir_all(&self, path: &Path, dir_options: &DirOptions) -> io::Result<()> {
        if path == Path::new("") {
            return Ok(());
        }

        match self._create_dir_one(path, dir_options) {
            Ok(()) => return Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) if self.is_dir(path) => return Ok(()),
            Err(e) => return Err(e),
        }
        match path.parent() {
            Some(p) => self._create_dir_all(p, dir_options)?,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to create whole tree",
                ))
            }
        }
        match self._create_dir_one(path, dir_options) {
            Ok(()) => Ok(()),
            Err(_) if self.is_dir(path) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Opens a file in write-only mode.
    ///
    /// This corresponds to [`std::fs::File::create`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self.open_with(
            path,
            OpenOptions::new().write(true).create(true).truncate(true),
        )
    }

    /// Returns the canonical form of a path with all intermediate components
    /// normalized and symbolic links resolved.
    ///
    /// This corresponds to [`std::fs::canonicalize`], but instead of returning
    /// an absolute path, returns a path relative to the directory
    /// represented by `self`.
    #[inline]
    pub fn canonicalize<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        canonicalize(&self.std_file, path.as_ref())
    }

    /// Copies the contents of one file to another. This function will also
    /// copy the permission bits of the original file to the destination
    /// file.
    ///
    /// This corresponds to [`std::fs::copy`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to_dir: &Self,
        to: Q,
    ) -> io::Result<u64> {
        copy(&self.std_file, from.as_ref(), &to_dir.std_file, to.as_ref())
    }

    /// Creates a new hard link on a filesystem.
    ///
    /// This corresponds to [`std::fs::hard_link`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        src: P,
        dst_dir: &Self,
        dst: Q,
    ) -> io::Result<()> {
        hard_link(
            &self.std_file,
            src.as_ref(),
            &dst_dir.std_file,
            dst.as_ref(),
        )
    }

    /// Given a path, query the file system to get information about a file,
    /// directory, etc.
    ///
    /// This corresponds to [`std::fs::metadata`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        stat(&self.std_file, path.as_ref(), FollowSymlinks::Yes)
    }

    /// Queries metadata about the underlying directory.
    ///
    /// This is similar to [`std::fs::File::metadata`], but for `Dir` rather
    /// than for `File`.
    #[inline]
    pub fn dir_metadata(&self) -> io::Result<Metadata> {
        Metadata::from_file(&self.std_file)
    }

    /// Returns an iterator over the entries within `self`.
    #[inline]
    pub fn entries(&self) -> io::Result<ReadDir> {
        read_base_dir(&self.std_file).map(|inner| ReadDir { inner })
    }

    /// Returns an iterator over UTF-8 entries within `self`.
    ///
    /// Equivalent to [`crate::fs_utf8::Dir::read_dir`].
    #[inline]
    #[cfg(feature = "fs_utf8")]
    pub fn entries_utf8(&self) -> io::Result<crate::fs_utf8::ReadDir> {
        self.entries().map(crate::fs_utf8::ReadDir::from_cap_std)
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// This corresponds to [`std::fs::read_dir`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<ReadDir> {
        read_dir(&self.std_file, path.as_ref()).map(|inner| ReadDir { inner })
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This corresponds to [`std::fs::read`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn read<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<u8>> {
        let mut file = self.open(path)?;
        let mut bytes = Vec::with_capacity(initial_buffer_size(&file));
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This corresponds to [`std::fs::read_link`], but only accesses paths
    /// relative to `self`.  Unlike [`read_link_contents`], this method considers it an error if
    /// the link's target is an absolute path.
    #[inline]
    pub fn read_link<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        read_link(&self.std_file, path.as_ref())
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This corresponds to [`std::fs::read_link`]. but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn read_link_contents<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        read_link_contents(&self.std_file, path.as_ref())
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This corresponds to [`std::fs::read_to_string`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        let mut s = String::new();
        self.open(path)?.read_to_string(&mut s)?;
        Ok(s)
    }

    /// Removes an empty directory.
    ///
    /// This corresponds to [`std::fs::remove_dir`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn remove_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        remove_dir(&self.std_file, path.as_ref())
    }

    /// Removes a directory at this path, after removing all its contents. Use
    /// carefully!
    ///
    /// This corresponds to [`std::fs::remove_dir_all`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        remove_dir_all(&self.std_file, path.as_ref())
    }

    /// Remove the directory referenced by `self` and consume `self`.
    ///
    /// Even though this implementation works in terms of handles as much as
    /// possible, removal is not guaranteed to be atomic with respect to a
    /// concurrent rename of the directory.
    #[inline]
    pub fn remove_open_dir(self) -> io::Result<()> {
        remove_open_dir(self.std_file)
    }

    /// Removes the directory referenced by `self`, after removing all its
    /// contents, and consume `self`. Use carefully!
    ///
    /// Even though this implementation works in terms of handles as much as
    /// possible, removal is not guaranteed to be atomic with respect to a
    /// concurrent rename of the directory.
    #[inline]
    pub fn remove_open_dir_all(self) -> io::Result<()> {
        remove_open_dir_all(self.std_file)
    }

    /// Removes a file from a filesystem.
    ///
    /// This corresponds to [`std::fs::remove_file`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn remove_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        remove_file(&self.std_file, path.as_ref())
    }

    /// Rename a file or directory to a new name, replacing the original file
    /// if to already exists.
    ///
    /// This corresponds to [`std::fs::rename`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to_dir: &Self,
        to: Q,
    ) -> io::Result<()> {
        rename(&self.std_file, from.as_ref(), &to_dir.std_file, to.as_ref())
    }

    /// Changes the permissions found on a file or a directory.
    ///
    /// This corresponds to [`std::fs::set_permissions`], but only accesses
    /// paths relative to `self`. Also, on some platforms, this function
    /// may fail if the file or directory cannot be opened for reading or
    /// writing first.
    #[cfg(not(target_os = "wasi"))]
    #[inline]
    pub fn set_permissions<P: AsRef<Path>>(&self, path: P, perm: Permissions) -> io::Result<()> {
        set_permissions(&self.std_file, path.as_ref(), perm)
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// This corresponds to [`std::fs::symlink_metadata`], but only accesses
    /// paths relative to `self`.
    #[inline]
    pub fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<Metadata> {
        stat(&self.std_file, path.as_ref(), FollowSymlinks::No)
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This corresponds to [`std::fs::write`], but only accesses paths
    /// relative to `self`.
    #[inline]
    pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> io::Result<()> {
        let mut file = self.create(path)?;
        file.write_all(contents.as_ref())
    }

    /// Creates a new symbolic link on a filesystem.
    ///
    /// The `original` argument provides the target of the symlink. The `link`
    /// argument provides the name of the created symlink.
    ///
    /// Despite the argument ordering, `original` is not resolved relative to
    /// `self` here. `link` is resolved relative to `self`, and `original` is
    /// not resolved within this function.
    ///
    /// The `link` path is resolved when the symlink is dereferenced, relative
    /// to the directory that contains it.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], but only accesses
    /// paths relative to `self`.
    ///
    /// Unlike [`symlink_contents`] this method will return an error if `original` is an absolute
    /// path.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    #[cfg(not(windows))]
    #[inline]
    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, original: P, link: Q) -> io::Result<()> {
        symlink(original.as_ref(), &self.std_file, link.as_ref())
    }

    /// Creates a new symbolic link on a filesystem.
    ///
    /// The `original` argument provides the target of the symlink. The `link`
    /// argument provides the name of the created symlink.
    ///
    /// Despite the argument ordering, `original` is not resolved relative to
    /// `self` here. `link` is resolved relative to `self`, and `original` is
    /// not resolved within this function.
    ///
    /// The `link` path is resolved when the symlink is dereferenced, relative
    /// to the directory that contains it.
    ///
    /// This corresponds to [`std::os::unix::fs::symlink`], but only accesses
    /// paths relative to `self`.
    ///
    /// [`std::os::unix::fs::symlink`]: https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html
    #[cfg(not(windows))]
    #[inline]
    pub fn symlink_contents<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        original: P,
        link: Q,
    ) -> io::Result<()> {
        symlink_contents(original.as_ref(), &self.std_file, link.as_ref())
    }

    /// Creates a new file symbolic link on a filesystem.
    ///
    /// The `original` argument provides the target of the symlink. The `link`
    /// argument provides the name of the created symlink.
    ///
    /// Despite the argument ordering, `original` is not resolved relative to
    /// `self` here. `link` is resolved relative to `self`, and `original` is
    /// not resolved within this function.
    ///
    /// The `link` path is resolved when the symlink is dereferenced, relative
    /// to the directory that contains it.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_file`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_file.html
    #[cfg(windows)]
    #[inline]
    pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        original: P,
        link: Q,
    ) -> io::Result<()> {
        symlink_file(original.as_ref(), &self.std_file, link.as_ref())
    }

    /// Creates a new directory symlink on a filesystem.
    ///
    /// The `original` argument provides the target of the symlink. The `link`
    /// argument provides the name of the created symlink.
    ///
    /// Despite the argument ordering, `original` is not resolved relative to
    /// `self` here. `link` is resolved relative to `self`, and `original` is
    /// not resolved within this function.
    ///
    /// The `link` path is resolved when the symlink is dereferenced, relative
    /// to the directory that contains it.
    ///
    /// This corresponds to [`std::os::windows::fs::symlink_dir`], but only
    /// accesses paths relative to `self`.
    ///
    /// [`std::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
    #[cfg(windows)]
    #[inline]
    pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        original: P,
        link: Q,
    ) -> io::Result<()> {
        symlink_dir(original.as_ref(), &self.std_file, link.as_ref())
    }

    /// Creates a new `UnixListener` bound to the specified socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixListener::bind`], but
    /// only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`std::os::unix::net::UnixListener::bind`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.bind
    #[doc(alias = "bind")]
    #[cfg(unix)]
    #[inline]
    pub fn bind_unix_listener<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixListener> {
        todo!(
            "Dir::bind_unix_listener({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Connects to the socket named by path.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::connect`], but
    /// only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`std::os::unix::net::UnixStream::connect`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.connect
    #[doc(alias = "connect")]
    #[cfg(unix)]
    #[inline]
    pub fn connect_unix_stream<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixStream> {
        todo!(
            "Dir::connect_unix_stream({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Creates a Unix datagram socket bound to the given path.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::bind`], but
    /// only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`std::os::unix::net::UnixDatagram::bind`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.bind
    #[doc(alias = "bind")]
    #[cfg(unix)]
    #[inline]
    pub fn bind_unix_datagram<P: AsRef<Path>>(&self, path: P) -> io::Result<UnixDatagram> {
        todo!(
            "Dir::bind_unix_datagram({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Connects the socket to the specified address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::connect`], but
    /// only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`std::os::unix::net::UnixDatagram::connect`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.connect
    #[doc(alias = "connect")]
    #[cfg(unix)]
    #[inline]
    pub fn connect_unix_datagram<P: AsRef<Path>>(
        &self,
        _unix_datagram: &UnixDatagram,
        path: P,
    ) -> io::Result<()> {
        todo!(
            "Dir::connect_unix_datagram({:?}, {})",
            self.std_file,
            path.as_ref().display()
        )
    }

    /// Sends data on the socket to the specified address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::send_to`], but
    /// only accesses paths relative to `self`.
    ///
    /// XXX: This function is not yet implemented.
    ///
    /// [`std::os::unix::net::UnixDatagram::send_to`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.send_to
    #[doc(alias = "send_to")]
    #[cfg(unix)]
    #[inline]
    pub fn send_to_unix_datagram_addr<P: AsRef<Path>>(
        &self,
        _unix_datagram: &UnixDatagram,
        buf: &[u8],
        path: P,
    ) -> io::Result<usize> {
        todo!(
            "Dir::send_to_unix_datagram_addr({:?}, {:?}, {})",
            self.std_file,
            buf,
            path.as_ref().display()
        )
    }

    /// Creates a new `Dir` instance that shares the same underlying file
    /// handle as the existing `Dir` instance.
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let dir = self.std_file.try_clone()?;
        Ok(Self::from_std_file(dir))
    }

    /// Returns `true` if the path points at an existing entity.
    ///
    /// This corresponds to [`std::path::Path::exists`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).is_ok()
    }

    /// Returns `true` if the path points at an existing entity.
    ///
    /// This corresponds to [`std::fs::try_exists`], but only
    /// accesses paths relative to `self`.
    ///
    /// # API correspondence with `std`
    ///
    /// This API is not yet stable in `std`, but is likely to be. For more
    /// information, see the [tracker issue](https://github.com/rust-lang/rust/issues/83186).
    #[inline]
    pub fn try_exists<P: AsRef<Path>>(&self, path: P) -> io::Result<bool> {
        match self.metadata(path) {
            Ok(_) => Ok(true),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }

    /// Returns `true` if the path exists on disk and is pointing at a regular
    /// file.
    ///
    /// This corresponds to [`std::path::Path::is_file`], but only
    /// accesses paths relative to `self`.
    #[inline]
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).map(|m| m.is_file()).unwrap_or(false)
    }

    /// Checks if `path` is a directory.
    ///
    /// This is similar to [`std::path::Path::is_dir`] in that it checks if
    /// `path` relative to `Dir` is a directory. This function will
    /// traverse symbolic links to query information about the destination
    /// file. In case of broken symbolic links, this will return `false`.
    #[inline]
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path).map(|m| m.is_dir()).unwrap_or(false)
    }

    /// Constructs a new instance of `Self` by opening the given path as a
    /// directory using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub fn open_ambient_dir<P: AsRef<Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<Self> {
        let dir = open_ambient_dir(path.as_ref(), ambient_authority)?;
        Ok(Self::from_std_file(dir))
    }

    /// Constructs a new instance of `Self` by opening the parent directory
    /// (aka "..") of `self`, using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function accesses a directory outside of the `self` subtree.
    #[inline]
    pub fn open_parent_dir(&self, ambient_authority: AmbientAuthority) -> io::Result<Self> {
        let dir = open_parent_dir(&self.std_file, ambient_authority)?;
        Ok(Self::from_std_file(dir))
    }

    /// Recursively create a directory and all of its parent components if they
    /// are missing, using the host process' ambient authority.
    ///
    /// # Ambient Authority
    ///
    /// This function is not sandboxed and may access any path that the host
    /// process has access to.
    #[inline]
    pub fn create_ambient_dir_all<P: AsRef<Path>>(
        path: P,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<()> {
        let _ = ambient_authority;
        fs::create_dir_all(path)
    }

    /// Construct a new instance of `Self` from existing directory file
    /// descriptor.
    ///
    /// This can be useful when interacting with other libraries and or C/C++
    /// code which has invoked `openat(..., O_DIRECTORY)` external to this
    /// crate.
    pub fn reopen_dir<Filelike: AsFilelike>(dir: &Filelike) -> io::Result<Self> {
        cap_primitives::fs::open_dir(
            &dir.as_filelike_view::<std::fs::File>(),
            std::path::Component::CurDir.as_ref(),
        )
        .map(Self::from_std_file)
    }
}

// Safety: `FilelikeViewType` is implemented for `std::fs::File`.
unsafe impl io_lifetimes::views::FilelikeViewType for Dir {}

#[cfg(not(windows))]
impl FromRawFd for Dir {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std_file(fs::File::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for Dir {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std_file(fs::File::from(fd))
    }
}

#[cfg(windows)]
impl FromRawHandle for Dir {
    /// To prevent race conditions on Windows, the handle must be opened
    /// without `FILE_SHARE_DELETE`.
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_std_file(fs::File::from_raw_handle(handle))
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for Dir {
    #[inline]
    fn from(handle: OwnedHandle) -> Self {
        Self::from_std_file(fs::File::from(handle))
    }
}

#[cfg(not(windows))]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std_file.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for Dir {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std_file.as_fd()
    }
}

#[cfg(windows)]
impl AsRawHandle for Dir {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.std_file.as_raw_handle()
    }
}

#[cfg(windows)]
impl AsHandle for Dir {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.std_file.as_handle()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for Dir {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std_file.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for Dir {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std_file.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for Dir {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std_file.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl From<Dir> for OwnedFd {
    #[inline]
    fn from(dir: Dir) -> OwnedFd {
        dir.std_file.into()
    }
}

#[cfg(windows)]
impl IntoRawHandle for Dir {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.std_file.into_raw_handle()
    }
}

#[cfg(windows)]
impl From<Dir> for OwnedHandle {
    #[inline]
    fn from(dir: Dir) -> OwnedHandle {
        dir.std_file.into()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for Dir {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std_file.into_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl From<Dir> for OwnedHandleOrSocket {
    #[inline]
    fn from(dir: Dir) -> Self {
        dir.std_file.into()
    }
}

/// Indicates how large a buffer to pre-allocate before reading the entire
/// file.
///
/// Derived from the function of the same name in Rust's library/std/src/fs.rs
/// at revision 108e90ca78f052c0c1c49c42a22c85620be19712.
fn initial_buffer_size(file: &File) -> usize {
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file. Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0)
}

impl fmt::Debug for Dir {
    // Like libstd's version, but doesn't print the path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("Dir");
        #[cfg(not(windows))]
        b.field("fd", &self.std_file.as_raw_fd());
        #[cfg(windows)]
        b.field("handle", &self.std_file.as_raw_handle());
        b.finish()
    }
}
