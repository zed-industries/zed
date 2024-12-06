pub mod call_settings;

#[cfg(all(target_os = "macos", not(feature = "livekit-cross-platform")))]
mod macos;

#[cfg(all(target_os = "macos", not(feature = "livekit-cross-platform")))]
pub use macos::*;

#[cfg(feature = "livekit-cross-platform")]
mod cross_platform;

#[cfg(feature = "livekit-cross-platform")]
pub use cross_platform::*;

#[cfg(all(not(target_os = "macos"), not(feature = "livekit-cross-platform")))]
compile_error!("Linux/Windows builds require `--features call/livekit-cross-platform`");
