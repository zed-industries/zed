use std::future::Future;

use crate::fs::File;
use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;

/// A builder for opening files with configurable options.
///
/// Files can be opened in [`read`] and/or [`write`] mode.
///
/// The [`append`] option opens files in a special writing mode that moves the file cursor to the
/// end of file before every write operation.
///
/// It is also possible to [`truncate`] the file right after opening, to [`create`] a file if it
/// doesn't exist yet, or to always create a new file with [`create_new`].
///
/// This type is an async version of [`std::fs::OpenOptions`].
///
/// [`read`]: #method.read
/// [`write`]: #method.write
/// [`append`]: #method.append
/// [`truncate`]: #method.truncate
/// [`create`]: #method.create
/// [`create_new`]: #method.create_new
/// [`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
///
/// # Examples
///
/// Open a file for reading:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .open("a.txt")
///     .await?;
/// #
/// # Ok(()) }) }
/// ```
///
/// Open a file for both reading and writing, and create it if it doesn't exist yet:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .write(true)
///     .create(true)
///     .open("a.txt")
///     .await?;
/// #
/// # Ok(()) }) }
/// ```
#[derive(Clone, Debug)]
pub struct OpenOptions(std::fs::OpenOptions);

impl OpenOptions {
    /// Creates a blank set of options.
    ///
    /// All options are initially set to `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .read(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn new() -> OpenOptions {
        OpenOptions(std::fs::OpenOptions::new())
    }

    /// Configures the option for read mode.
    ///
    /// When set to `true`, this option means the file will be readable after opening.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .read(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    /// Configures the option for write mode.
    ///
    /// When set to `true`, this option means the file will be writable after opening.
    ///
    /// If the file already exists, write calls on it will overwrite the previous contents without
    /// truncating it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .write(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }

    /// Configures the option for append mode.
    ///
    /// When set to `true`, this option means the file will be writable after opening and the file
    /// cursor will be moved to the end of file before every write operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .append(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.0.append(append);
        self
    }

    /// Configures the option for truncating the previous file.
    ///
    /// When set to `true`, the file will be truncated to the length of 0 bytes.
    ///
    /// The file must be opened in [`write`] or [`append`] mode for truncation to work.
    ///
    /// [`write`]: #method.write
    /// [`append`]: #method.append
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .write(true)
    ///     .truncate(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.0.truncate(truncate);
        self
    }

    /// Configures the option for creating a new file if it doesn't exist.
    ///
    /// When set to `true`, this option means a new file will be created if it doesn't exist.
    ///
    /// The file must be opened in [`write`] or [`append`] mode for file creation to work.
    ///
    /// [`write`]: #method.write
    /// [`append`]: #method.append
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .write(true)
    ///     .create(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.0.create(create);
        self
    }

    /// Configures the option for creating a new file or failing if it already exists.
    ///
    /// When set to `true`, this option means a new file will be created, or the open operation
    /// will fail if the file already exists.
    ///
    /// The file must be opened in [`write`] or [`append`] mode for file creation to work.
    ///
    /// [`write`]: #method.write
    /// [`append`]: #method.append
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .write(true)
    ///     .create_new(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.0.create_new(create_new);
        self
    }

    /// Opens a file with the configured options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file does not exist and neither [`create`] nor [`create_new`] were set.
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to open the file in the configured mode.
    /// * The file already exists and [`create_new`] was set.
    /// * Invalid combination of options was used, like [`truncate`] was set but [`write`] wasn't,
    ///   or none of [`read`], [`write`], and [`append`] modes was set.
    /// * An OS-level occurred, like too many files are open or the file name is too long.
    /// * Some other I/O error occurred.
    ///
    /// [`read`]: #method.read
    /// [`write`]: #method.write
    /// [`append`]: #method.append
    /// [`truncate`]: #method.truncate
    /// [`create`]: #method.create
    /// [`create_new`]: #method.create_new
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::OpenOptions;
    ///
    /// let file = OpenOptions::new()
    ///     .read(true)
    ///     .open("a.txt")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn open<P: AsRef<Path>>(&self, path: P) -> impl Future<Output = io::Result<File>> {
        let path = path.as_ref().to_owned();
        let options = self.0.clone();
        async move {
            let file = spawn_blocking(move || options.open(path)).await?;
            Ok(File::new(file, true))
        }
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}

cfg_unix! {
    use crate::os::unix::fs::OpenOptionsExt;

    impl OpenOptionsExt for OpenOptions {
        fn mode(&mut self, mode: u32) -> &mut Self {
            self.0.mode(mode);
            self
        }

        fn custom_flags(&mut self, flags: i32) -> &mut Self {
            self.0.custom_flags(flags);
            self
        }
    }
}
