// Module layout mirrors `gpui_linux::linux::*` for muscle memory: each piece
// of the platform owns its own file, the `pub use` re-exports below collapse
// them under `crate::android::*` so the `Platform` impl in `platform.rs` can
// say `super::AndroidWindow` etc.
mod app;
mod bell;
mod clipboard;
mod dispatcher;
mod display;
mod input;
mod intents;
mod jni_glue;
mod keyboard;
mod keystore;
mod platform;
mod window;

pub use app::{android_app, set_android_app};
pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use keyboard::*;
pub use platform::AndroidPlatform;
pub(crate) use window::*;

use std::rc::Rc;

/// Build the default GPUI `Platform` for Android.
///
/// This is what [`gpui_platform::current_platform`] hands back when running
/// on `target_os = "android"`. You normally do **not** call it directly —
/// instead use [`gpui_platform::application`] inside `android_main`.
///
/// # Parameters
///
/// - `headless` is forwarded to [`AndroidPlatform::new`]. When `true`,
///   `open_window` returns an error so consumers can run executors / text
///   systems without trying to allocate a `wgpu::Surface`. The Android port
///   does not have a separate headless backend; the flag is just stashed on
///   the platform struct.
///
/// # Threading
///
/// Returns an [`Rc<dyn Platform>`] (so it must be held on the same thread
/// that constructed it). The returned platform's [`Platform::run`] method
/// blocks the calling thread; that thread becomes GPUI's "main" thread for
/// the rest of the process's life.
///
/// [`gpui_platform::current_platform`]: https://docs.rs/gpui_platform/latest/gpui_platform/fn.current_platform.html
/// [`gpui_platform::application`]: https://docs.rs/gpui_platform/latest/gpui_platform/fn.application.html
/// [`Platform::run`]: gpui::Platform::run
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    Rc::new(AndroidPlatform::new(headless))
}
