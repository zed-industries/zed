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
mod clipboard;
mod cursor;
mod dispatcher;
mod display;
mod executor;
mod executor_runtime;
mod gestures;
mod gpu;
mod input;
mod input_handler;
mod keyboard;
#[cfg(all(target_os = "linux", feature = "wayland"))]
pub mod layer_shell;
mod menu;
mod notification;
mod platform;
mod platform_scheduler;
mod platform_window;
pub mod popup;
pub mod profiler;
mod prompt;
#[cfg(any(
    test,
    target_os = "windows",
    target_os = "linux",
    target_family = "wasm",
    feature = "test-support",
    feature = "bench-support"
))]
#[expect(missing_docs)]
pub mod queue;
#[cfg(all(
    feature = "screen-capture",
    any(target_os = "windows", target_os = "linux", target_os = "freebsd",)
))]
pub mod scap_screen_capture;
mod screen_capture;
#[cfg(any(test, feature = "test-support"))]
mod test_dispatcher;
mod text_input;
mod window;
mod window_id;

pub use app::*;
pub use clipboard::*;
pub use cursor::*;
pub use dispatcher::*;
pub use display::*;
pub use executor::*;
pub use executor_runtime::*;
pub use gestures::*;
pub use gpu::*;
pub use gpui_shared_string::*;
pub use gpui_types::*;
pub use input::*;
pub use input_handler::*;
pub use keyboard::*;
pub use menu::*;
pub use notification::*;
pub use platform::*;
pub use platform_scheduler::*;
pub use platform_window::*;
pub use profiler::{
    ForegroundRunnableCounter, foreground_runnable_counter, foreground_runnable_finished,
};
pub use prompt::*;
#[cfg(any(
    test,
    target_os = "windows",
    target_os = "linux",
    target_family = "wasm",
    feature = "test-support",
    feature = "bench-support"
))]
pub use queue::*;
pub use screen_capture::*;
#[cfg(any(test, feature = "test-support"))]
pub use test_dispatcher::*;
pub use text_input::*;
pub use window::*;
pub use window_id::*;
