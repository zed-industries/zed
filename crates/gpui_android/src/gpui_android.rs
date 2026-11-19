#![cfg(target_os = "android")]

//! GPUI backend for Android, driven by `android-activity`'s `NativeActivity`
//! glue. The OS owns the activity lifecycle; `AndroidPlatform::run` blocks in
//! the `android_main` thread pumping `AndroidApp::poll_events`. Rendering and
//! text are provided by `gpui_wgpu` (Vulkan/GL + cosmic-text).

mod dispatcher;
mod display;
mod events;
mod insets;
mod keyboard;
mod links;
mod platform;
mod vsync;
mod window;

pub use android_activity::AndroidApp;
pub use platform::{AndroidPlatform, init};

pub fn init_logging() {
    // wgpu and its dependencies are noisy below info.
    let filter = android_logger::FilterBuilder::new().parse("info").build();
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_filter(filter)
            .with_tag("gpui"),
    );
}
