//! GPUI platform implementation for Android.
//!
//! This crate is empty on non-Android targets; the workspace can still build it
//! everywhere because all Android-specific dependencies are gated behind
//! `cfg(target_os = "android")` in `Cargo.toml`.
#![cfg(target_os = "android")]

mod android;

pub use android::{AndroidPlatform, current_platform, set_native_window};
