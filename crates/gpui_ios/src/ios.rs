//! UIKit-backed implementation details for the iOS GPUI platform.

mod application;
mod display;
mod events;
mod platform;
mod text_input;
mod text_system;
mod window;

pub(crate) use display::*;
pub use platform::*;
pub(crate) use text_system::*;
pub use window::set_status_bar_style;
pub(crate) use window::*;

/// Returns the native platform implementation for iOS.
pub fn current_platform(_headless: bool) -> std::rc::Rc<dyn gpui::Platform> {
    std::rc::Rc::new(IosPlatform::new())
}
