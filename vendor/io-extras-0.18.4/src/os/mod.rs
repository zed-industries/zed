//! OS-specific functionality.

#[cfg(not(windows))]
pub mod rustix;
#[cfg(windows)]
pub mod windows;
