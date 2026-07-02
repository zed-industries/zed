#![cfg(target_os = "android")]

//! GPUI backend for Android, driven by `android-activity`'s `NativeActivity`
//! glue. The OS owns the activity lifecycle; `AndroidPlatform::run` blocks in
//! the `android_main` thread pumping `AndroidApp::poll_events`. Rendering and
//! text are provided by `gpui_wgpu` (Vulkan/GL + cosmic-text).

mod dispatcher;
mod display;
mod events;
mod keyboard;
mod platform;
mod window;

pub use android_activity::AndroidApp;
pub use platform::{AndroidPlatform, init};

pub fn init_logging() {
    // Debug for the sync stack, info elsewhere (wgpu and friends are noisy).
    let filter = android_logger::FilterBuilder::new()
        .parse("info,deltadb=debug,delta_remote=debug")
        .build();
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_filter(filter)
            .with_tag("gpui"),
    );
}
