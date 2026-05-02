//! GPUI platform implementation for Android.
//!
//! `gpui_android` is the Android equivalent of [`gpui_macos`], [`gpui_linux`]
//! and [`gpui_windows`]: it implements GPUI's [`Platform`], [`PlatformWindow`],
//! [`PlatformDispatcher`], [`PlatformDisplay`], [`PlatformAtlas`],
//! [`PlatformKeyboardLayout`] and [`PlatformTextSystem`] traits on top of
//! [`android-activity`](https://crates.io/crates/android-activity)'s
//! `GameActivity` backend, [`wgpu`] over Vulkan, and `cosmic-text` driven
//! through `/system/fonts`.
//!
//! # Architecture
//!
//! - **Bootstrap.** Your app's `cdylib` exposes
//!   `#[unsafe(no_mangle)] fn android_main(app: AndroidApp)` (the symbol the
//!   `android-activity` `GameActivity` glue spawns once the JVM thread is
//!   ready). Inside it you call [`set_android_app`] to register the activity
//!   handle, then run [`gpui_platform::application`] as on every other
//!   platform.
//! - **Surface lifecycle.** Android can revoke the underlying
//!   `ANativeWindow` at any time (rotation, fold, app switch). The platform
//!   listens for `MainEvent::InitWindow` / `TerminateWindow`, attaches /
//!   detaches the [`gpui_wgpu::WgpuRenderer`] synchronously, and gates every
//!   `draw` call on a `surface_alive` flag so a redraw arriving after the
//!   surface is gone cannot crash the GPU.
//! - **Input routing.** Touch events are mapped to
//!   [`PlatformInput::MouseDown`], [`MouseMove`] and [`MouseUp`] with
//!   [`MouseButton::Left`], so any GPUI element that already responds to a
//!   click handler responds to a finger tap. Hardware keys translate via a
//!   keycode table; soft-keyboard input flows through the IME's
//!   `TextInputState` straight into the focused [`PlatformInputHandler`].
//! - **Fonts.** `fontdb` 0.23 has no Android branch in
//!   `load_system_fonts()`, so [`AndroidPlatform`] manually walks
//!   `/system/fonts/`, `/data/fonts/` and `/product/fonts/` at startup and
//!   registers every TrueType/OpenType file with `cosmic-text`.
//! - **JNI.** Clipboard, system intents (`open_url`, `reveal_path`,
//!   credentials in `AndroidKeyStore`) all go through raw JNI calls, no
//!   Java/Kotlin shim required beyond a 10-line `GameActivity` subclass that
//!   `loadLibrary("…")`s your `.so`.
//!
//! # Crate is empty on non-Android targets
//!
//! The crate body is gated behind `cfg(target_os = "android")`, so on macOS,
//! Linux, Windows and wasm builds it produces an empty rlib. That keeps the
//! workspace buildable everywhere without conditional dependencies in
//! consumers.
//!
//! # Quick start (in your app's `lib.rs`)
//!
//! ```ignore
//! #[unsafe(no_mangle)]
//! fn android_main(app: gpui_android::AndroidApp) {
//!     android_logger::init_once(
//!         android_logger::Config::default()
//!             .with_max_level(log::LevelFilter::Info),
//!     );
//!     gpui_android::set_android_app(app);
//!
//!     gpui_platform::application().run(|cx: &mut gpui::App| {
//!         cx.open_window(
//!             gpui::WindowOptions::default(),
//!             |_, cx| cx.new(|_| MyRoot::new()),
//!         )
//!         .unwrap();
//!         cx.activate(true);
//!     });
//! }
//! ```
//!
//! Pair that with `crate-type = ["cdylib"]` in `Cargo.toml`, a tiny
//! `GameActivity` subclass that runs `System.loadLibrary("your_lib")`, and
//! the manifest entries described in `crates/gpui_android/SETUP.md`.
//!
//! [`Platform`]: gpui::Platform
//! [`PlatformWindow`]: gpui::PlatformWindow
//! [`PlatformDispatcher`]: gpui::PlatformDispatcher
//! [`PlatformDisplay`]: gpui::PlatformDisplay
//! [`PlatformAtlas`]: gpui::PlatformAtlas
//! [`PlatformKeyboardLayout`]: gpui::PlatformKeyboardLayout
//! [`PlatformTextSystem`]: gpui::PlatformTextSystem
//! [`PlatformInputHandler`]: gpui::PlatformInputHandler
//! [`PlatformInput::MouseDown`]: gpui::PlatformInput::MouseDown
//! [`MouseButton::Left`]: gpui::MouseButton::Left
//! [`MouseMove`]: gpui::PlatformInput::MouseMove
//! [`MouseUp`]: gpui::PlatformInput::MouseUp
//! [`gpui_macos`]: https://docs.rs/gpui_macos
//! [`gpui_linux`]: https://docs.rs/gpui_linux
//! [`gpui_windows`]: https://docs.rs/gpui_windows
//! [`gpui_platform::application`]: https://docs.rs/gpui_platform/latest/gpui_platform/fn.application.html
#![cfg(target_os = "android")]
#![warn(missing_docs)]

mod android;

pub use android::{AndroidPlatform, android_app, current_platform, set_android_app};

/// Re-export of [`android_activity::AndroidApp`] so applications don't have
/// to add `android-activity` to their `Cargo.toml` directly when wiring up
/// `android_main`.
///
/// `AndroidApp` is a cheaply-clonable handle to the activity's native
/// state: lifecycle events, input queue, clipboard interop, IME state and
/// (most importantly) the `NativeWindow` to render into. After
/// [`set_android_app`] has been called, [`android_app`] hands you a clone
/// from anywhere in the process.
pub use android_activity::AndroidApp;
