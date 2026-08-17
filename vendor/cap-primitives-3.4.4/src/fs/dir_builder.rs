use crate::fs::DirOptions;
use std::fmt;

/// A builder used to create directories in various manners.
///
/// This corresponds to [`std::fs::DirBuilder`].
///
/// Unlike `std::fs::DirBuilder`, this API has no `DirBuilder::create`, because
/// creating directories requires a capability. Use [`Dir::create_dir_with`]
/// instead.
///
/// [`Dir::create_dir_with`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html#method.create_dir_with
///
/// <details>
/// We need to define our own version because the libstd `DirBuilder` doesn't
/// have public accessors that we can use.
/// </details>
pub struct DirBuilder {
    pub(crate) recursive: bool,
    pub(crate) options: DirOptions,
}

impl DirBuilder {
    /// Creates a new set of options with default mode/security settings for
    /// all platforms and also non-recursive.
    ///
    /// This corresponds to [`std::fs::DirBuilder::new`].
    #[allow(clippy::new_without_default)]
    #[inline]
    pub const fn new() -> Self {
        Self {
            recursive: false,
            options: DirOptions::new(),
        }
    }

    /// Indicates that directories should be created recursively, creating all
    /// parent directories.
    ///
    /// This corresponds to [`std::fs::DirBuilder::recursive`].
    #[inline]
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive = recursive;
        self
    }

    /// Return the `DirOptions` contained in this `DirBuilder`.
    #[inline]
    pub const fn options(&self) -> &DirOptions {
        &self.options
    }

    /// Return the value of the `recursive` flag.
    #[inline]
    pub const fn is_recursive(&self) -> bool {
        self.recursive
    }
}

/// Unix-specific extensions to [`fs::DirBuilder`].
#[cfg(unix)]
pub trait DirBuilderExt {
    /// Sets the mode to create new directories with. This option defaults to
    /// 0o777.
    fn mode(&mut self, mode: u32) -> &mut Self;
}

#[cfg(unix)]
impl DirBuilderExt for DirBuilder {
    #[inline]
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.options.mode(mode);
        self
    }
}

impl fmt::Debug for DirBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = f.debug_struct("DirBuilder");
        b.field("recursive", &self.recursive);
        b.field("options", &self.options);
        b.finish()
    }
}
