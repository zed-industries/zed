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

mod keyboard;

pub use keyboard::*;
