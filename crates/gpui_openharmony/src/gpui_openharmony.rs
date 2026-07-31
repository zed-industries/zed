#![cfg_attr(target_env = "ohos", allow(dead_code))]
#![allow(clippy::too_many_arguments)]

#[cfg(target_env = "ohos")]
mod dispatcher;
#[cfg(target_env = "ohos")]
mod display;
#[cfg(target_env = "ohos")]
mod events;
#[cfg(target_env = "ohos")]
mod keyboard;
#[cfg(target_env = "ohos")]
mod clipboard;
#[cfg(target_env = "ohos")]
mod display_info;
#[cfg(target_env = "ohos")]
mod platform;
#[cfg(target_env = "ohos")]
mod text_system;
#[cfg(target_env = "ohos")]
mod window;

#[cfg(target_env = "ohos")]
pub use events::{
    NativeWindow, OpenHarmonyHostEvent, OpenHarmonyKeyEvent, OpenHarmonyTouchEvent,
    OpenHarmonyTouchPhase, SurfaceInfo,
};
#[cfg(target_env = "ohos")]
pub use platform::{OpenHarmonyPlatform, current_platform};
