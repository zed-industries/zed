//! Zed for iPad — iOS static library entry point.
//!
//! This crate produces a static library (.a) that the Swift host app links against.
//! It provides C FFI entry points that the Swift side calls to initialize GPUI,
//! open windows, and manage the application lifecycle.
//!
//! See: docs/ios-port-plan.md for full architecture details.

#[cfg(target_os = "ios")]
mod ios {
    use gpui::{
        Application, ApplicationKeepAlive, App, AppContext as _, Context, Render,
        Window, WindowOptions, div, IntoElement, SharedString, prelude::*,
    };
    use gpui_ios::IosPlatform;
    use std::{cell::RefCell, rc::Rc};

    thread_local! {
        /// Keeps the GPUI application alive for the process lifetime.
        /// On iOS, Application::run() returns immediately (UIKit owns the run loop),
        /// so we must hold this handle or the App is immediately dropped.
        static APP_KEEPALIVE: RefCell<Option<ApplicationKeepAlive>> = RefCell::new(None);
    }

    /// Minimal smoke-test view that renders a line of text.
    struct TextSmokeView;

    impl Render for TextSmokeView {
        fn render(
            &mut self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> impl IntoElement {
            div()
                .size_full()
                .bg(gpui::rgb(0xff0000))
                .flex()
                .items_center()
                .justify_center()
                .text_color(gpui::rgb(0xffffff))
                .child(SharedString::from("Hello from Zed on iPad!"))
        }
    }

    pub fn ios_main() {
        let platform = Rc::new(IosPlatform::new());
        let app = Application::with_platform(platform);

        // Keep the app alive — Application::run() returns immediately on iOS
        // because UIKit owns the run loop.
        let keepalive = app.keep_alive();
        APP_KEEPALIVE.with(|cell| *cell.borrow_mut() = Some(keepalive));

        app.run(|_cx: &mut App| {
            // Window is opened from zed_ios_open_window when the UIWindowScene activates.
        });
    }

    pub fn ios_open_window() {
        APP_KEEPALIVE.with(|cell| {
            let borrowed = cell.borrow();
            if let Some(keepalive) = borrowed.as_ref() {
                keepalive.update(|cx| {
                    if let Err(err) = cx.open_window(WindowOptions::default(), |_window, cx| {
                        cx.new(|_| TextSmokeView)
                    }) {
                        log::error!("[zed-ios] open_window failed: {err:?}");
                    }
                });
            } else {
                log::error!("[zed-ios] APP_KEEPALIVE is None — zed_ios_main must be called first");
            }
        });
    }
}

/// Main entry point called by AppDelegate.swift after UIApplicationMain.
///
/// # Safety
/// Called from Swift via C FFI. Must be called exactly once on the main thread.
#[unsafe(no_mangle)]
pub extern "C" fn zed_ios_main() {
    #[cfg(target_os = "ios")]
    ios::ios_main();
}

/// Called by SceneDelegate.swift when a new UIWindowScene activates.
///
/// # Safety
/// Called from Swift via C FFI. `scene_id` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_ios_open_window(_scene_id: *const std::ffi::c_char) {
    #[cfg(target_os = "ios")]
    ios::ios_open_window();
}

/// Called by SceneDelegate.swift when a UIWindowScene disconnects.
///
/// # Safety
/// Called from Swift via C FFI. `scene_id` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_ios_close_window(_scene_id: *const std::ffi::c_char) {
    // TODO: Clean up the GPUI window, disconnect if last window.
}

// Submodules — uncomment as implemented:
// pub mod keychain;         // Phase 2.1: SSH key storage via Security.framework
// pub mod network_monitor;  // Phase 2.3: NWPathMonitor connectivity events
// pub mod ssh_transport;    // Phase 2.0: russh-based SSH transport (CRITICAL)
