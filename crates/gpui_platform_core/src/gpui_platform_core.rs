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
mod atlas;
mod bounds_tree;
mod clipboard;
mod cursor;
mod dispatcher;
mod display;
mod executor;
mod executor_runtime;
mod font_fallbacks;
mod font_features;
mod gestures;
mod gpu;
mod input;
mod input_handler;
mod keyboard;
#[cfg(all(target_os = "linux", feature = "wayland"))]
pub mod layer_shell;
mod notification;
mod platform_scheduler;
pub mod popup;
#[cfg(feature = "profiler")]
pub mod profiler;
mod prompt;
mod render;
mod scene;
mod screen_capture;
#[cfg(any(test, feature = "test-support"))]
mod test_dispatcher;
mod text_input;
mod text_system;
mod window;
mod window_id;

pub use app::*;
pub use atlas::*;
pub use clipboard::*;
pub use cursor::*;
pub use dispatcher::*;
pub use display::*;
pub use executor::*;
pub use executor_runtime::*;
pub use font_fallbacks::*;
pub use font_features::*;
pub use gestures::*;
pub use gpu::*;
pub use input::*;
pub use input_handler::*;
pub use keyboard::*;
pub use notification::*;
pub use platform_scheduler::*;
#[cfg(feature = "profiler")]
pub use profiler::*;
pub use prompt::*;
pub use render::*;
pub use scene::*;
pub use screen_capture::*;
#[cfg(any(test, feature = "test-support"))]
pub use test_dispatcher::*;
pub use text_input::*;
pub use text_system::*;
pub use window::*;
pub use window_id::*;
