pub mod call_settings;

#[cfg(any(
    all(target_os = "macos", feature = "livekit-macos"),
    all(
        not(target_os = "macos"),
        feature = "livekit-macos",
        not(feature = "livekit-cross-platform")
    )
))]
mod macos;

#[cfg(any(
    all(target_os = "macos", feature = "livekit-macos"),
    all(
        not(target_os = "macos"),
        feature = "livekit-macos",
        not(feature = "livekit-cross-platform")
    )
))]
pub use macos::*;

#[cfg(any(
    all(
        target_os = "macos",
        feature = "livekit-cross-platform",
        not(feature = "livekit-macos"),
    ),
    all(not(target_os = "macos"), feature = "livekit-cross-platform"),
))]
mod cross_platform;

#[cfg(any(
    all(
        target_os = "macos",
        feature = "livekit-cross-platform",
        not(feature = "livekit-macos"),
    ),
    all(not(target_os = "macos"), feature = "livekit-cross-platform"),
))]
pub use cross_platform::*;
