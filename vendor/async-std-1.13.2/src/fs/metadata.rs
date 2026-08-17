use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;

/// Reads metadata for a path.
///
/// This function will traverse symbolic links to read metadata for the target file or directory.
/// If you want to read metadata without following symbolic links, use [`symlink_metadata`]
/// instead.
///
/// This function is an async version of [`std::fs::metadata`].
///
/// [`symlink_metadata`]: fn.symlink_metadata.html
/// [`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file or directory.
/// * The current process lacks permissions to read metadata for the path.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// let perm = fs::metadata("a.txt").await?.permissions();
/// #
/// # Ok(()) }) }
/// ```
pub async fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || std::fs::metadata(path)).await
}

cfg_not_docs! {
    pub use std::fs::Metadata;
}

cfg_docs! {
    use std::time::SystemTime;

    use crate::fs::{FileType, Permissions};

    /// Metadata for a file or directory.
    ///
    /// Metadata is returned by [`metadata`] and [`symlink_metadata`].
    ///
    /// This type is a re-export of [`std::fs::Metadata`].
    ///
    /// [`metadata`]: fn.metadata.html
    /// [`symlink_metadata`]: fn.symlink_metadata.html
    /// [`is_dir`]: #method.is_dir
    /// [`is_file`]: #method.is_file
    /// [`std::fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
    #[derive(Clone, Debug)]
    pub struct Metadata {
        _private: (),
    }

    impl Metadata {
        /// Returns the file type from this metadata.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{:?}", metadata.file_type());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn file_type(&self) -> FileType {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns `true` if this metadata is for a regular directory.
        ///
        /// If this metadata is for a symbolic link, this method returns `false`.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata(".").await?;
        /// println!("{:?}", metadata.is_dir());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn is_dir(&self) -> bool {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns `true` if this metadata is for a regular file.
        ///
        /// If this metadata is for a symbolic link, this method returns `false`.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{:?}", metadata.is_file());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn is_file(&self) -> bool {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns the file size in bytes.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{}", metadata.len());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn len(&self) -> u64 {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns the permissions from this metadata.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{:?}", metadata.permissions());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn permissions(&self) -> Permissions {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns the last modification time.
        ///
        /// # Errors
        ///
        /// This data may not be available on all platforms, in which case an error will be
        /// returned.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{:?}", metadata.modified());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn modified(&self) -> io::Result<SystemTime> {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns the last access time.
        ///
        /// # Errors
        ///
        /// This data may not be available on all platforms, in which case an error will be
        /// returned.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{:?}", metadata.accessed());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn accessed(&self) -> io::Result<SystemTime> {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Returns the creation time.
        ///
        /// # Errors
        ///
        /// This data may not be available on all platforms, in which case an error will be
        /// returned.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        /// #
        /// use async_std::fs;
        ///
        /// let metadata = fs::metadata("a.txt").await?;
        /// println!("{:?}", metadata.created());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn created(&self) -> io::Result<SystemTime> {
            unreachable!("this impl only appears in the rendered docs")
        }
    }
}
