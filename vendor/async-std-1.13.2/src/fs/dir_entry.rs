use std::ffi::OsString;
use std::fmt;
use std::sync::Arc;

use crate::fs::{FileType, Metadata};
use crate::io;
use crate::path::PathBuf;
use crate::task::spawn_blocking;

/// An entry in a directory.
///
/// A stream of entries in a directory is returned by [`read_dir`].
///
/// This type is an async version of [`std::fs::DirEntry`].
///
/// [`read_dir`]: fn.read_dir.html
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
pub struct DirEntry(Arc<std::fs::DirEntry>);

impl DirEntry {
    /// Creates an asynchronous `DirEntry` from a synchronous one.
    pub(crate) fn new(inner: std::fs::DirEntry) -> DirEntry {
        DirEntry(Arc::new(inner))
    }

    /// Returns the full path to this entry.
    ///
    /// The full path is created by joining the original path passed to [`read_dir`] with the name
    /// of this entry.
    ///
    /// [`read_dir`]: fn.read_dir.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_std::prelude::*;
    ///
    /// let mut dir = fs::read_dir(".").await?;
    ///
    /// while let Some(res) = dir.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.path());
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn path(&self) -> PathBuf {
        self.0.path().into()
    }

    /// Reads the metadata for this entry.
    ///
    /// This function will traverse symbolic links to read the metadata.
    ///
    /// If you want to read metadata without following symbolic links, use [`symlink_metadata`]
    /// instead.
    ///
    /// [`symlink_metadata`]: fn.symlink_metadata.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read the metadata.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_std::prelude::*;
    ///
    /// let mut dir = fs::read_dir(".").await?;
    ///
    /// while let Some(res) = dir.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.metadata().await?);
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn metadata(&self) -> io::Result<Metadata> {
        let inner = self.0.clone();
        spawn_blocking(move || inner.metadata()).await
    }

    /// Reads the file type for this entry.
    ///
    /// This function will not traverse symbolic links if this entry points at one.
    ///
    /// If you want to read metadata with following symbolic links, use [`metadata`] instead.
    ///
    /// [`metadata`]: #method.metadata
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read this entry's metadata.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_std::prelude::*;
    ///
    /// let mut dir = fs::read_dir(".").await?;
    ///
    /// while let Some(res) = dir.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.file_type().await?);
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn file_type(&self) -> io::Result<FileType> {
        let inner = self.0.clone();
        spawn_blocking(move || inner.file_type()).await
    }

    /// Returns the bare name of this entry without the leading path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_std::prelude::*;
    ///
    /// let mut dir = fs::read_dir(".").await?;
    ///
    /// while let Some(res) = dir.next().await {
    ///     let entry = res?;
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DirEntry").field(&self.path()).finish()
    }
}

impl Clone for DirEntry {
    fn clone(&self) -> Self {
        DirEntry(self.0.clone())
    }
}

cfg_unix! {
    use crate::os::unix::fs::DirEntryExt;

    impl DirEntryExt for DirEntry {
        fn ino(&self) -> u64 {
            self.0.ino()
        }
    }
}
