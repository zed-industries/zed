use std::future::Future;

use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;

/// A builder for creating directories with configurable options.
///
/// For Unix-specific options, import the [`os::unix::fs::DirBuilderExt`] trait.
///
/// This type is an async version of [`std::fs::DirBuilder`].
///
/// [`os::unix::fs::DirBuilderExt`]: ../os/unix/fs/trait.DirBuilderExt.html
/// [`std::fs::DirBuilder`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
#[derive(Debug, Default)]
pub struct DirBuilder {
    /// Set to `true` if non-existent parent directories should be created.
    recursive: bool,

    /// Unix mode for newly created directories.
    #[cfg(unix)]
    mode: Option<u32>,
}

impl DirBuilder {
    /// Creates a blank set of options.
    ///
    /// The [`recursive`] option is initially set to `false`.
    ///
    /// [`recursive`]: #method.recursive
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::fs::DirBuilder;
    ///
    /// let builder = DirBuilder::new();
    /// ```
    pub fn new() -> DirBuilder {
        #[cfg(not(unix))]
        let builder = DirBuilder { recursive: false };

        #[cfg(unix)]
        let builder = DirBuilder {
            recursive: false,
            mode: None,
        };

        builder
    }

    /// Sets the option for recursive mode.
    ///
    /// When set to `true`, this option means all parent directories should be created recursively
    /// if they don't exist. Parents are created with the same permissions as the final directory.
    ///
    /// This option is initially set to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::fs::DirBuilder;
    ///
    /// let mut builder = DirBuilder::new();
    /// builder.recursive(true);
    /// ```
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive = recursive;
        self
    }

    /// Creates a directory with the configured options.
    ///
    /// It is considered an error if the directory already exists unless recursive mode is enabled.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` already points to an existing file or directory.
    /// * The current process lacks permissions to create the directory or its missing parents.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::DirBuilder;
    ///
    /// DirBuilder::new()
    ///     .recursive(true)
    ///     .create("./some/directory")
    ///     .await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn create<P: AsRef<Path>>(&self, path: P) -> impl Future<Output = io::Result<()>> {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(self.recursive);

        #[cfg(unix)]
        {
            if let Some(mode) = self.mode {
                std::os::unix::fs::DirBuilderExt::mode(&mut builder, mode);
            }
        }

        let path = path.as_ref().to_owned();
        async move { spawn_blocking(move || builder.create(path)).await }
    }
}

cfg_unix! {
    use crate::os::unix::fs::DirBuilderExt;

    impl DirBuilderExt for DirBuilder {
        fn mode(&mut self, mode: u32) -> &mut Self {
            self.mode = Some(mode);
            self
        }
    }
}
