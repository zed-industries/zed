//! Convenience crate that re-exports GPUI's platform traits and the
//! `current_platform` constructor so consumers don't need `#[cfg]` gating.

pub use gpui::Platform;

use std::rc::Rc;

/// Returns a background executor for the current platform.
pub fn background_executor() -> gpui::BackgroundExecutor {
    current_platform(true).background_executor()
}

pub fn application() -> gpui::Application {
    #[cfg(target_family = "wasm")]
    {
        application_with_web_backend(gpui_web::WebBackendPreference::Auto)
    }

    #[cfg(not(target_family = "wasm"))]
    gpui::Application::with_platform(current_platform(false))
}

pub fn headless() -> gpui::Application {
    gpui::Application::with_platform(current_platform(true))
}

#[cfg(target_family = "wasm")]
pub use gpui_web::WebBackendPreference;

#[cfg(target_family = "wasm")]
pub fn application_with_web_backend(backend_preference: WebBackendPreference) -> gpui::Application {
    let platform = Rc::new(gpui_web::WebPlatform::new_with_backend(
        true,
        backend_preference,
    ));
    let http_client = std::sync::Arc::new(platform.fetch_http_client());
    gpui::Application::with_platform(platform).with_http_client(http_client)
}

/// Unlike `application`, this function returns a single-threaded web application.
#[cfg(target_family = "wasm")]
pub fn single_threaded_web() -> gpui::Application {
    let platform = Rc::new(gpui_web::WebPlatform::new(false));
    let http_client = std::sync::Arc::new(platform.fetch_http_client());
    gpui::Application::with_platform(platform).with_http_client(http_client)
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

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use gpui::{AppContext, Empty, VisualTestAppContext};
    use std::cell::RefCell;
    use std::time::Duration;

    #[cfg(all(feature = "test-support", feature = "font-kit"))]
    #[test]
    #[ignore = "requires a Metal device"]
    fn retained_scene_matches_full_refresh_pixels() {
        use gpui::{
            Context, Entity, HeadlessAppContext, IntoElement, ParentElement, Render, Styled,
            Window, div, px, rgb, size,
        };

        struct Tile(u32);
        impl Render for Tile {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .w(px(140.))
                    .h(px(90.))
                    .rounded_lg()
                    .border_2()
                    .border_color(rgb(0xffffff))
                    .bg(rgb(self.0))
                    .opacity(0.8)
                    .text_color(rgb(0xffffff))
                    .child("Retained λ")
                    .child(
                        gpui::canvas(
                            |_, _, _| (),
                            |bounds, _, window, _| {
                                let mut path = gpui::PathBuilder::fill();
                                path.move_to(bounds.origin);
                                path.line_to(bounds.bottom_right());
                                path.line_to(bounds.bottom_left());
                                path.close();
                                window.paint_path(path.build().expect("triangle"), rgb(0xffbb22));
                            },
                        )
                        .w(px(80.))
                        .h(px(30.)),
                    )
            }
        }
        struct Root(Vec<Entity<Tile>>);
        impl Render for Root {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .bg(rgb(0x182030))
                    .flex()
                    .gap_3()
                    .overflow_hidden()
                    .children(self.0.iter().cloned())
            }
        }
        let mut cx = HeadlessAppContext::with_platform(
            current_platform(true).text_system(),
            std::sync::Arc::new(()),
            current_headless_renderer,
        );
        let first = cx.new(|_| Tile(0x883344));
        let second = cx.new(|_| Tile(0x338844));
        let window = cx
            .open_window(size(px(400.), px(200.)), |_, cx| {
                cx.new(|_| Root(vec![first.clone(), second]))
            })
            .expect("offscreen window");
        cx.run_until_parked();
        let mut reused = 0;
        for step in 0..8 {
            first.update(&mut cx, |tile, cx| {
                tile.0 += 0x030201;
                cx.notify();
            });
            window
                .update(&mut cx, |root, window, cx| {
                    if step == 3 {
                        root.0.reverse();
                        cx.notify();
                    }
                    if step == 5 {
                        window.resize(size(px(220.), px(140.)));
                    }
                    if step == 7 {
                        root.0.retain(|tile| tile != &first);
                        cx.notify();
                    }
                })
                .expect("update tiles");
            cx.run_until_parked();
            cx.update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                if let Some(stats) = window.retained_node_stats() {
                    reused += stats.reused_subtrees;
                }
            })
            .expect("draw incremental scene");
            let incremental = cx
                .capture_screenshot(window.into())
                .expect("Metal readback");
            assert!(
                incremental
                    .pixels()
                    .any(|pixel| pixel != incremental.get_pixel(0, 0)),
                "nontrivial GPU output"
            );
            cx.update_window(window.into(), |_, window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
            })
            .expect("draw reference scene");
            let reference = cx
                .capture_screenshot(window.into())
                .expect("reference Metal readback");
            assert!(incremental == reference, "GPU pixels differ at step {step}");
        }
        cx.update_window(window.into(), |_, window, _| {
            if window.retained_node_stats().is_some() {
                assert!(reused > 0, "pixel oracle must exercise retained reuse");
            }
        })
        .expect("verify reuse");
    }

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
