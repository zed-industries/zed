//! UIKit-backed implementation details for the iOS GPUI platform.

pub(crate) mod cg_types;
mod dispatcher;
mod display;
mod events;
pub mod ffi;
mod platform;
mod text_input;
mod text_system;
pub mod util;
mod window;

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub use platform::*;
pub(crate) use text_system::*;
pub use window::set_status_bar_style;
pub(crate) use window::*;

/// Returns the native platform implementation for iOS.
pub fn current_platform(_headless: bool) -> std::rc::Rc<dyn gpui::Platform> {
    std::rc::Rc::new(IosPlatform::new())
}
