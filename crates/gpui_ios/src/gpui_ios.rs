#![cfg(target_os = "ios")]
//! iOS platform implementation for GPUI.
//!
//! Implements `Platform`, `PlatformWindow`, and `PlatformDisplay` for iPadOS
//! using UIKit + Metal. The run loop is owned by UIKit; GPUI hooks in via
//! CADisplayLink and GCD.
//!
//! Phase 1 status: Metal renderer and CADisplayLink are wired up.
//! UITextInput, CoreText text system, and input events are Phase 1.3 work.

mod dispatcher;
mod display;
mod display_link;
mod keyboard;
pub(crate) mod metal_atlas;
pub(crate) mod metal_renderer;
mod platform;
mod window;

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use keyboard::*;
pub(crate) use window::IosWindow;

pub use platform::{IosPlatform, start_rendering};
