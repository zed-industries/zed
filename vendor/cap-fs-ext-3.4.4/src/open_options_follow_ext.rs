use crate::FollowSymlinks;

/// Extension trait for `cap_primitives::fs::OpenOptions` which adds
/// `follow`, a function for controlling whether a symlink in the last
/// component of a path is followed.
pub trait OpenOptionsFollowExt {
    /// Sets the option for following symlinks in the last component of a path.
    ///
    /// This option, when set to `FollowSymlinks::Yes`, will indicate that a
    /// symbolic link in the last component of a path will be followed. When
    /// set to `FollowSymlinks::No`, it will indicate that attempting to
    /// resolve a path which ends in a symbolic link will fail.
    fn follow(&mut self, follow: FollowSymlinks) -> &mut Self;
}

impl OpenOptionsFollowExt for cap_primitives::fs::OpenOptions {
    #[inline]
    fn follow(&mut self, follow: FollowSymlinks) -> &mut Self {
        // `follow` functionality is implemented within `cap_primitives`; we're
        // just exposing it here since `OpenOptions` is re-exported by
        // `cap_std` etc. and `follow` isn't in `std`.
        self._cap_fs_ext_follow(follow)
    }
}
