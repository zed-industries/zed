mod app;
mod clipboard;
mod dispatcher;
mod display;
mod input;
mod keyboard;
mod platform;
mod window;

pub use app::set_android_app;
pub(crate) use app::*;
pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use keyboard::*;
pub use platform::AndroidPlatform;
pub(crate) use window::*;

use std::rc::Rc;

/// Returns the default platform implementation for Android.
///
/// `headless` controls whether the platform initialises rendering resources.
/// The Android port currently has no separate headless backend, so this argument
/// is forwarded to [`AndroidPlatform`] for future use.
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    Rc::new(AndroidPlatform::new(headless))
}
