//! GPUI platform implementation for Android.
//!
//! Bootstrap pattern (in your app's `lib.rs`):
//!
//! ```ignore
//! #[no_mangle]
//! fn android_main(app: android_activity::AndroidApp) {
//!     gpui_android::set_android_app(app);
//!     gpui_platform::application().run(|cx| {
//!         // open windows, etc.
//!     });
//! }
//! ```
//!
//! The crate body is gated behind `cfg(target_os = "android")`, so on
//! non-Android targets it's an empty lib that the workspace can still build.
#![cfg(target_os = "android")]

mod android;

pub use android::{AndroidPlatform, current_platform, set_android_app};

/// Re-export of `android_activity::AndroidApp` so consumers don't have to add
/// `android-activity` to their `Cargo.toml` directly when bootstrapping
/// `android_main`.
pub use android_activity::AndroidApp;
