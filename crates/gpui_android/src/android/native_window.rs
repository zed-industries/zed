//! Holds the currently-active Android `NativeWindow` so [`AndroidPlatform`] can
//! create [`AndroidWindow`]s on demand. The JNI/Java bridge is expected to
//! call [`set_native_window`] from the `surfaceCreated` / `onResume`
//! callbacks, and to set it to `None` on `surfaceDestroyed` / `onPause`.
//!
//! Holding the [`ndk::native_window::NativeWindow`] keeps the underlying
//! `ANativeWindow` reference alive (the type is a ref-counted handle), so the
//! GPU surface stays valid until the next callback explicitly drops it.

use ndk::native_window::NativeWindow;
use parking_lot::RwLock;
use std::sync::OnceLock;

static NATIVE_WINDOW: OnceLock<RwLock<Option<NativeWindow>>> = OnceLock::new();

fn slot() -> &'static RwLock<Option<NativeWindow>> {
    NATIVE_WINDOW.get_or_init(|| RwLock::new(None))
}

/// Register (or clear) the live `NativeWindow` for the GPUI Android platform.
/// Pass `None` from `surfaceDestroyed` / `onPause` so subsequent attempts to
/// open a window fail fast instead of crashing on a stale handle.
pub fn set_native_window(window: Option<NativeWindow>) {
    *slot().write() = window;
}

/// Borrow a clone of the current native window if one is registered.
pub(crate) fn current_native_window() -> Option<NativeWindow> {
    slot().read().as_ref().cloned()
}
