pub mod call_settings;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(target_os = "macos"))]
mod cross_platform;

#[cfg(not(target_os = "macos"))]
pub use cross_platform::*;
