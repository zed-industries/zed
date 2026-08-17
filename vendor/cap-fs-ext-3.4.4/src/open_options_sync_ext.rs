/// Extension trait for `cap_primitives::fs::OpenOptions` which adds
/// `sync`, `dsync`, `rsync`, and `nonblock` functions for controlling various
/// I/O modes for the opened file.
pub trait OpenOptionsSyncExt {
    /// Requests write operations complete as defined by synchronized I/O file
    /// integrity completion.
    fn sync(&mut self, enable: bool) -> &mut Self;

    /// Requests write operations complete as defined by synchronized I/O data
    /// integrity completion.
    fn dsync(&mut self, enable: bool) -> &mut Self;

    /// Requests read operations complete as defined by the level of integrity
    /// specified by `sync` and `dsync`.
    fn rsync(&mut self, enable: bool) -> &mut Self;

    /// Requests that I/O operations fail with `std::io::ErrorKind::WouldBlock`
    /// if they would otherwise block.
    ///
    /// This option is commonly not implemented for regular files, so blocking
    /// may still occur.
    fn nonblock(&mut self, enable: bool) -> &mut Self;
}

impl OpenOptionsSyncExt for cap_primitives::fs::OpenOptions {
    #[inline]
    fn sync(&mut self, enable: bool) -> &mut Self {
        // `sync` functionality is implemented within `cap_primitives`;
        // we're just exposing it here since `OpenOptions` is re-exported by
        // `cap_std` etc. and `sync` isn't in `std`.
        self._cap_fs_ext_sync(enable)
    }

    #[inline]
    fn dsync(&mut self, enable: bool) -> &mut Self {
        // `dsync` functionality is implemented within `cap_primitives`;
        // we're just exposing it here since `OpenOptions` is re-exported by
        // `cap_std` etc. and `dsync` isn't in `std`.
        self._cap_fs_ext_dsync(enable)
    }

    #[inline]
    fn rsync(&mut self, enable: bool) -> &mut Self {
        // `rsync` functionality is implemented within `cap_primitives`;
        // we're just exposing it here since `OpenOptions` is re-exported by
        // `cap_std` etc. and `rsync` isn't in `std`.
        self._cap_fs_ext_rsync(enable)
    }

    #[inline]
    fn nonblock(&mut self, enable: bool) -> &mut Self {
        // `nonblock` functionality is implemented within `cap_primitives`;
        // we're just exposing it here since `OpenOptions` is re-exported by
        // `cap_std` etc. and `nonblock` isn't in `std`.
        self._cap_fs_ext_nonblock(enable)
    }
}
