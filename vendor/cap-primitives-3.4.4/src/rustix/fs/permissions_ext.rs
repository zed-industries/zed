use crate::fs::Permissions;
use rustix::fs::RawMode;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ImplPermissionsExt {
    #[cfg(not(target_os = "wasi"))]
    mode: RawMode,
}

#[cfg(not(target_os = "wasi"))]
impl ImplPermissionsExt {
    /// Constructs a new instance of `Self` from the given
    /// [`std::fs::Permissions`].
    #[inline]
    pub(crate) fn from_std(std: fs::Permissions) -> Self {
        use std::os::unix::fs::PermissionsExt;
        Self {
            mode: std.mode() as RawMode,
        }
    }

    /// Constructs a new instance of `Permissions` from the given
    /// `RawMode`.
    #[inline]
    pub(crate) const fn from_raw_mode(mode: RawMode) -> Permissions {
        Permissions {
            readonly: Self::readonly(mode),
            ext: Self { mode },
        }
    }

    /// Test whether the given `RawMode` lacks write permissions.
    #[inline]
    pub(crate) const fn readonly(mode: RawMode) -> bool {
        mode & 0o222 == 0
    }

    /// Test whether the given `RawMode` lacks write permissions.
    #[inline]
    pub(crate) fn set_readonly(&mut self, readonly: bool) {
        if readonly {
            // remove write permission for all classes; equivalent to `chmod a-w <file>`
            self.mode &= !0o222;
        } else {
            // add write permission for all classes; equivalent to `chmod a+w <file>`
            self.mode |= 0o222;
        }
    }
}

#[cfg(not(target_os = "wasi"))]
impl crate::fs::PermissionsExt for ImplPermissionsExt {
    fn mode(&self) -> u32 {
        self.mode as u32
    }

    fn set_mode(&mut self, mode: u32) {
        self.mode = mode as RawMode & 0o7777;
    }

    fn from_mode(mode: u32) -> Self {
        Self {
            mode: mode as RawMode & 0o7777,
        }
    }
}

#[cfg(target_os = "wasi")]
impl ImplPermissionsExt {
    pub(crate) fn default() -> Permissions {
        Permissions { readonly: false }
    }
}
