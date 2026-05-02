//! Holds the [`AndroidApp`] handle the JNI side passes into [`android_main`].
//! Cloning an `AndroidApp` is cheap (it's a refcounted handle), so we copy it
//! into the global slot and hand clones out to anyone that needs to interact
//! with the activity (the platform's run loop, the window for surface access,
//! and so on).

use android_activity::AndroidApp;
use parking_lot::RwLock;
use std::sync::OnceLock;

static ANDROID_APP: OnceLock<RwLock<Option<AndroidApp>>> = OnceLock::new();

fn slot() -> &'static RwLock<Option<AndroidApp>> {
    ANDROID_APP.get_or_init(|| RwLock::new(None))
}

/// Register the `AndroidApp` instance for this process. Must be called from
/// `#[no_mangle] fn android_main(app: AndroidApp)` before constructing the
/// GPUI [`AndroidPlatform`](super::AndroidPlatform).
pub fn set_android_app(app: AndroidApp) {
    *slot().write() = Some(app);
}

/// Borrow a clone of the registered `AndroidApp`. Returns `None` if
/// [`set_android_app`] was never called (e.g. a non-Android-bootstrap test).
pub(crate) fn android_app() -> Option<AndroidApp> {
    slot().read().clone()
}
