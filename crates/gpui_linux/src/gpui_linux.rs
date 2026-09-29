#![cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

pub use linux::{LinuxDisplayModes, current_platform, linux_platform};
