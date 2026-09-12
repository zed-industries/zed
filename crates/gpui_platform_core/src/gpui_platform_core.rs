//! Abstract platform traits shared by `gpui` and its platform backends.
//!
//! This crate is the contract between the GPUI facade and the concrete
//! platform implementations (`gpui_macos`, `gpui_linux`, `gpui_windows`,
//! `gpui_web`). It depends only on `gpui_types` and other low-level leaf
//! crates, never on `gpui`, so a backend can implement these traits without
//! depending on the entire framework.
//!
//! The `gpui` crate re-exports everything here, so consumers keep using
//! `gpui::PlatformKeyboardMapper` and friends.

#![warn(missing_docs)]

mod app;
mod cursor;
mod dispatcher;
mod display;
mod executor;
mod executor_runtime;
mod keyboard;
mod notification;
mod platform_scheduler;
#[cfg(feature = "profiler")]
pub mod profiler;
mod prompt;
mod screen_capture;
#[cfg(any(test, feature = "test-support"))]
mod test_dispatcher;
mod window;

pub use app::*;
pub use cursor::*;
pub use dispatcher::*;
pub use display::*;
pub use executor::*;
pub use executor_runtime::*;
pub use keyboard::*;
pub use notification::*;
pub use platform_scheduler::*;
#[cfg(feature = "profiler")]
pub use profiler::*;
pub use prompt::*;
pub use screen_capture::*;
#[cfg(any(test, feature = "test-support"))]
pub use test_dispatcher::*;
pub use window::*;
