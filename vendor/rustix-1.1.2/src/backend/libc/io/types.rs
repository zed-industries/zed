use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::io::fcntl_getfd
    /// [`fcntl_setfd`]: crate::io::fcntl_setfd
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FdFlags: u32 {
        /// `FD_CLOEXEC`
        const CLOEXEC = bitcast!(c::FD_CLOEXEC);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(all(linux_kernel, not(target_os = "android")))]
bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ReadWriteFlags: u32 {
        /// `RWF_DSYNC` (since Linux 4.7)
        const DSYNC = libc::RWF_DSYNC as u32;
        /// `RWF_HIPRI` (since Linux 4.6)
        const HIPRI = libc::RWF_HIPRI as u32;
        /// `RWF_SYNC` (since Linux 4.7)
        const SYNC = libc::RWF_SYNC as u32;
        /// `RWF_NOWAIT` (since Linux 4.14)
        const NOWAIT = libc::RWF_NOWAIT as u32;
        /// `RWF_APPEND` (since Linux 4.16)
        const APPEND = libc::RWF_APPEND as u32;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct DupFlags: u32 {
        /// `O_CLOEXEC`
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "android",
            target_os = "redox",
        )))] // Android 5.0 has dup3, but libc doesn't have bindings
        const CLOEXEC = bitcast!(c::O_CLOEXEC);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}
