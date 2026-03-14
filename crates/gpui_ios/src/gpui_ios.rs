#![cfg(target_os = "ios")]
//! iOS platform implementation for GPUI.
//!
//! Implements `Platform`, `PlatformWindow`, and `PlatformDisplay` for iPadOS
//! using UIKit + Metal + CoreText. The run loop is driven by UIKit's main
//! runloop; GCD is used for background dispatch.
//!
//! Phase 1 status: dispatcher and display are functional stubs; window and
//! renderer are not yet wired to UIKit. Metal rendering and UITextInput are
//! Phase 1.3 work.

mod dispatcher;
mod display;
mod keyboard;
mod platform;
mod window;

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use keyboard::*;
pub(crate) use window::*;

pub use platform::IosPlatform;
