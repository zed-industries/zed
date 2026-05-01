mod dispatcher;
mod display;
mod keyboard;
mod native_window;
mod platform;
mod window;

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use keyboard::*;
pub use native_window::set_native_window;
pub(crate) use native_window::*;
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
