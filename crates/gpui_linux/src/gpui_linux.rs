#![cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

pub use linux::current_platform;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub use linux::switchable_platform;
