#[cfg(not(windows))]
use crate::fs::ImplPermissionsExt;
#[cfg(unix)]
use rustix::fs::RawMode;
use std::{fs, io};

/// Representation of the various permissions on a file.
///
/// This corresponds to [`std::fs::Permissions`].
///
/// <details>
/// We need to define our own version because the libstd `Permissions` doesn't
/// have a public constructor that we can use.
/// </details>
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Permissions {
    pub(crate) readonly: bool,

    #[cfg(any(unix, target_os = "vxworks"))]
    pub(crate) ext: ImplPermissionsExt,
}

impl Permissions {
    /// Constructs a new instance of `Self` from the given
    /// `std::fs::Permissions`.
    #[inline]
    pub fn from_std(std: fs::Permissions) -> Self {
        Self {
            readonly: std.readonly(),

            #[cfg(any(unix, target_os = "vxworks"))]
            ext: ImplPermissionsExt::from_std(std),
        }
    }

    /// Consumes `self` and produces a new instance of `std::fs::Permissions`.
    ///
    /// <details>
    /// The `file` parameter works around the fact that we can't construct a
    /// `Permissions` object ourselves on Windows.
    /// </details>
    #[inline]
    pub fn into_std(self, file: &fs::File) -> io::Result<fs::Permissions> {
        self._into_std(file)
    }

    #[cfg(unix)]
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    fn _into_std(self, _file: &fs::File) -> io::Result<fs::Permissions> {
        use std::os::unix::fs::PermissionsExt;
        Ok(fs::Permissions::from_mode(crate::fs::PermissionsExt::mode(
            &self.ext,
        )))
    }

    #[cfg(target_os = "wasi")]
    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    fn _into_std(self, file: &fs::File) -> io::Result<fs::Permissions> {
        let mut permissions = file.metadata()?.permissions();
        permissions.set_readonly(self.readonly());
        Ok(permissions)
    }

    #[cfg(windows)]
    #[inline]
    fn _into_std(self, file: &fs::File) -> io::Result<fs::Permissions> {
        let mut permissions = file.metadata()?.permissions();
        permissions.set_readonly(self.readonly());
        Ok(permissions)
    }

    /// Returns `true` if these permissions describe a readonly (unwritable)
    /// file.
    ///
    /// This corresponds to [`std::fs::Permissions::readonly`].
    #[inline]
    pub const fn readonly(&self) -> bool {
        self.readonly
    }

    /// Modifies the readonly flag for this set of permissions.
    ///
    /// This corresponds to [`std::fs::Permissions::set_readonly`].
    #[inline]
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly;

        #[cfg(any(unix, target_os = "vxworks"))]
        self.ext.set_readonly(readonly);
    }
}

/// Unix-specific extensions to [`Permissions`].
#[cfg(unix)]
pub trait PermissionsExt {
    /// Returns the underlying raw `st_mode` bits that contain the standard
    /// Unix permissions for this file.
    fn mode(&self) -> u32;

    /// Sets the underlying raw bits for this set of permissions.
    fn set_mode(&mut self, mode: u32);

    /// Creates a new instance of `Permissions` from the given set of Unix
    /// permission bits.
    fn from_mode(mode: u32) -> Self;
}

#[cfg(unix)]
impl PermissionsExt for Permissions {
    #[inline]
    fn mode(&self) -> u32 {
        crate::fs::PermissionsExt::mode(&self.ext)
    }

    #[inline]
    fn set_mode(&mut self, mode: u32) {
        crate::fs::PermissionsExt::set_mode(&mut self.ext, mode)
    }

    #[inline]
    fn from_mode(mode: u32) -> Self {
        Self {
            readonly: ImplPermissionsExt::readonly(mode as RawMode),
            ext: crate::fs::PermissionsExt::from_mode(mode),
        }
    }
}

#[cfg(target_os = "vxworks")]
impl PermissionsExt for Permissions {
    #[inline]
    fn mode(&self) -> u32 {
        self.ext.mode()
    }

    #[inline]
    fn set_mode(&mut self, mode: u32) {
        self.ext.set_mode(mode)
    }

    #[inline]
    fn from_mode(mode: u32) -> Self {
        Self {
            readonly: ImplPermissionsExt::readonly(mode),
            ext: ImplPermissionsExt::from(mode),
        }
    }
}
