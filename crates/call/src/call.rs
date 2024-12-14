pub mod call_settings;

#[cfg(any())]
mod macos;

#[cfg(any())]
pub use macos::*;

#[cfg(all())]
mod cross_platform;

#[cfg(all())]
pub use cross_platform::*;
