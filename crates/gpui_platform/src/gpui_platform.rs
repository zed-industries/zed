//! Convenience crate that re-exports GPUI's platform traits and the
//! `current_platform` constructor so consumers don't need `#[cfg]` gating.

pub use gpui::Platform;

use std::rc::Rc;

/// Returns a background executor for the current platform.
pub fn background_executor() -> gpui::BackgroundExecutor {
    current_platform(true).background_executor()
}

pub fn application() -> gpui::Application {
    gpui::Application::with_platform(current_platform(false))
}

pub fn headless() -> gpui::Application {
    gpui::Application::with_platform(current_platform(true))
}

/// Unlike `application`, this function returns a single-threaded web application.
#[cfg(target_family = "wasm")]
pub fn single_threaded_web() -> gpui::Application {
    gpui::Application::with_platform(Rc::new(gpui_web::WebPlatform::new(false)))
}

/// Initializes panic hooks and logging for the web platform.
/// Call this before running the application in a wasm_bindgen entrypoint.
#[cfg(target_family = "wasm")]
pub fn web_init() {
    console_error_panic_hook::set_once();
    gpui_web::init_logging();
}

/// Returns the default [`Platform`] for the current OS.
pub fn current_platform(headless: bool) -> Rc<dyn Platform> {
    #[cfg(target_os = "macos")]
    {
        Rc::new(gpui_macos::MacPlatform::new(headless))
    }

    #[cfg(target_os = "windows")]
    {
        Rc::new(
            gpui_windows::WindowsPlatform::new(headless)
                .expect("failed to initialize Windows platform"),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        gpui_linux::current_platform(headless)
    }

    #[cfg(target_family = "wasm")]
    {
        let _ = headless;
        Rc::new(gpui_web::WebPlatform::new(true))
    }
}

/// Returns a new [`HeadlessRenderer`] for the current platform, if available.
#[cfg(feature = "test-support")]
pub fn current_headless_renderer() -> Option<Box<dyn gpui::PlatformHeadlessRenderer>> {
    #[cfg(target_os = "macos")]
    {
        Some(Box::new(
            gpui_macos::metal_renderer::MetalHeadlessRenderer::new(),
        ))
    }

    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

#[cfg(all(test, target_os = "macos", feature = "test-support"))]
mod pixel_tests {
    use super::*;
    use gpui::{
        AppContext, Context, HeadlessAppContext, IntoElement, ParentElement, Render, Styled,
        Window, div, px, size,
    };
    use std::sync::Arc;

    struct RoundedClipView;

    impl Render for RoundedClipView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().bg(gpui::black()).child(
                div()
                    .size(px(100.))
                    .rounded(px(30.))
                    .overflow_hidden()
                    .child(div().size_full().bg(gpui::red())),
            )
        }
    }

    #[test]
    fn rounded_overflow_hidden_clips_rendered_pixels() {
        let text_system = Arc::new(gpui::NoopTextSystem);
        let mut cx =
            HeadlessAppContext::with_platform(text_system, Arc::new(()), current_headless_renderer);
        let window = cx
            .open_window(size(px(200.), px(200.)), |_, cx| {
                cx.new(|_| RoundedClipView)
            })
            .unwrap();
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear();
        })
        .unwrap();
        let image = cx.capture_screenshot(window.into()).unwrap();

        let scale = image.width() as f32 / 200.;
        let pixel = |x: f32, y: f32| image.get_pixel((x * scale) as u32, (y * scale) as u32).0;
        let is_red = |p: [u8; 4]| p[0] > 200 && p[1] < 50 && p[2] < 50;

        // Center of the rounded container is the child's red fill.
        assert!(is_red(pixel(50., 50.)), "center: {:?}", pixel(50., 50.));
        // Edge midpoints are inside the rounded rect.
        assert!(is_red(pixel(50., 2.)), "top edge: {:?}", pixel(50., 2.));
        // Corner pixels are outside the 30px arcs and must be clipped to the black
        // background.
        for (x, y) in [(2., 2.), (98., 2.), (98., 98.), (2., 98.)] {
            let p = pixel(x, y);
            assert!(!is_red(p), "corner ({x}, {y}) should be clipped: {p:?}");
        }
        // Points inside the arcs are red.
        for (x, y) in [(12., 12.), (88., 12.), (88., 88.), (12., 88.)] {
            let p = pixel(x, y);
            assert!(is_red(p), "inside arc ({x}, {y}) should be red: {p:?}");
        }
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use gpui::{AppContext, Empty, VisualTestAppContext};
    use std::cell::RefCell;
    use std::time::Duration;

    // Note: All VisualTestAppContext tests are ignored by default because they require
    // the macOS main thread. Standard Rust tests run on worker threads, which causes
    // SIGABRT when interacting with macOS AppKit/Cocoa APIs.
    //
    // To run these tests, use:
    // cargo test -p gpui visual_test_context -- --ignored --test-threads=1

    #[test]
    #[ignore] // Requires macOS main thread
    fn test_foreground_tasks_run_with_run_until_parked() {
        let mut cx = VisualTestAppContext::new(current_platform(false));

        let task_ran = Rc::new(RefCell::new(false));

        // Spawn a foreground task via the App's spawn method
        // This should use our TestDispatcher, not the MacDispatcher
        {
            let task_ran = task_ran.clone();
            cx.update(|cx| {
                cx.spawn(async move |_| {
                    *task_ran.borrow_mut() = true;
                })
                .detach();
            });
        }

        // The task should not have run yet
        assert!(!*task_ran.borrow());

        // Run until parked should execute the foreground task
        cx.run_until_parked();

        // Now the task should have run
        assert!(*task_ran.borrow());
    }

    #[test]
    #[ignore] // Requires macOS main thread
    fn test_advance_clock_triggers_delayed_tasks() {
        let mut cx = VisualTestAppContext::new(current_platform(false));

        let task_ran = Rc::new(RefCell::new(false));

        // Spawn a task that waits for a timer
        {
            let task_ran = task_ran.clone();
            let executor = cx.background_executor.clone();
            cx.update(|cx| {
                cx.spawn(async move |_| {
                    executor.timer(Duration::from_millis(500)).await;
                    *task_ran.borrow_mut() = true;
                })
                .detach();
            });
        }

        // Run until parked - the task should be waiting on the timer
        cx.run_until_parked();
        assert!(!*task_ran.borrow());

        // Advance clock past the timer duration
        cx.advance_clock(Duration::from_millis(600));

        // Now the task should have completed
        assert!(*task_ran.borrow());
    }

    #[test]
    #[ignore] // Requires macOS main thread - window creation fails on test threads
    fn test_window_spawn_uses_test_dispatcher() {
        let mut cx = VisualTestAppContext::new(current_platform(false));

        let task_ran = Rc::new(RefCell::new(false));

        let window = cx
            .open_offscreen_window_default(|_, cx| cx.new(|_| Empty))
            .expect("Failed to open window");

        // Spawn a task via window.spawn - this is the critical test case
        // for tooltip behavior, as tooltips use window.spawn for delayed show
        {
            let task_ran = task_ran.clone();
            cx.update_window(window.into(), |_, window, cx| {
                window
                    .spawn(cx, async move |_| {
                        *task_ran.borrow_mut() = true;
                    })
                    .detach();
            })
            .ok();
        }

        // The task should not have run yet
        assert!(!*task_ran.borrow());

        // Run until parked should execute the foreground task spawned via window
        cx.run_until_parked();

        // Now the task should have run
        assert!(*task_ran.borrow());
    }
}
