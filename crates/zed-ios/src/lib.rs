//! Zed for iPad — iOS static library entry point.
//!
//! This crate produces a static library (.a) that the Swift host app links against.
//! It provides C FFI entry points that the Swift side calls to initialize GPUI,
//! open windows, and manage the application lifecycle.
//!
//! See: docs/ios-port-plan.md for full architecture details.

// Phase 0: Minimal entry point that proves the build pipeline works.
// Phase 1+: Initialize GPUI with IosPlatform, create windows, etc.

/// Main entry point called by AppDelegate.swift after UIApplicationMain.
///
/// # Safety
/// Called from Swift via C FFI. Must be called exactly once on the main thread.
#[no_mangle]
pub extern "C" fn zed_ios_main() {
    // TODO Phase 0: Just prove this gets called without crashing.
    // TODO Phase 1: Initialize GPUI with IosPlatform, start the async executor.
    // TODO Phase 2: Show the connection manager UI.
}

/// Called by SceneDelegate.swift when a new UIWindowScene activates.
///
/// # Safety
/// Called from Swift via C FFI. `scene_id` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn zed_ios_open_window(_scene_id: *const std::ffi::c_char) {
    // TODO Phase 1: Create a new GPUI window backed by the UIWindowScene.
    // TODO Phase 4: State restoration — reconnect to the right host/directory.
}

/// Called by SceneDelegate.swift when a UIWindowScene disconnects.
///
/// # Safety
/// Called from Swift via C FFI. `scene_id` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn zed_ios_close_window(_scene_id: *const std::ffi::c_char) {
    // TODO: Clean up the GPUI window, disconnect if last window.
}

// Submodules — uncomment as implemented:
// pub mod keychain;         // Phase 2.1: SSH key storage via Security.framework
// pub mod network_monitor;  // Phase 2.3: NWPathMonitor connectivity events
// pub mod ssh_transport;    // Phase 2.0: russh-based SSH transport (CRITICAL)
