//! A clean testing API for GPUI applications.
//!
//! `TestApp` provides a simpler alternative to `TestAppContext` with:
//! - Automatic effect flushing after updates
//! - Clean window creation and inspection
//! - Input simulation helpers
//!
//! # Example
//! ```ignore
//! #[test]
//! fn test_my_view() {
//!     let mut app = TestApp::new();
//!
//!     let mut window = app.open_window(|window, cx| {
//!         MyView::new(window, cx)
//!     });
//!
//!     window.update(|view, window, cx| {
//!         view.do_something(cx);
//!     });
//!
//!     // Check rendered state
//!     assert_eq!(window.title(), Some("Expected Title"));
//! }
//! ```

use crate::{
    AnyWindowHandle, App, AppCell, AppContext, AsyncApp, BackgroundExecutor, BorrowAppContext,
    Bounds, ClipboardItem, Context, Entity, ForegroundExecutor, Global, InputEvent, Keystroke,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Platform, Point, Render,
    SceneSnapshot, Size, Task, TestDispatcher, TestPlatform, TextSystem, Window, WindowBounds,
    WindowHandle, WindowOptions, app::GpuiMode,
};
use rand::{SeedableRng, rngs::StdRng};
use std::{future::Future, rc::Rc, sync::Arc, time::Duration};

/// A test application context with a clean API.
///
/// Unlike `TestAppContext`, `TestApp` automatically flushes effects after
/// each update and provides simpler window management.
pub struct TestApp {
    app: Rc<AppCell>,
    platform: Rc<TestPlatform>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    #[allow(dead_code)]
    dispatcher: TestDispatcher,
    text_system: Arc<TextSystem>,
}

impl TestApp {
    /// Create a new test application.
    pub fn new() -> Self {
        Self::with_seed(0)
    }

    /// Create a new test application with a specific random seed.
    pub fn with_seed(seed: u64) -> Self {
        let dispatcher = TestDispatcher::new(seed);
        let arc_dispatcher = Arc::new(dispatcher.clone());
        let background_executor = BackgroundExecutor::new(arc_dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(arc_dispatcher);
        let platform = TestPlatform::new(background_executor.clone(), foreground_executor.clone());
        let asset_source = Arc::new(());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let text_system = Arc::new(TextSystem::new(platform.text_system()));

        let mut app = App::new_app(platform.clone(), asset_source, http_client);
        app.borrow_mut().mode = GpuiMode::test();

        Self {
            app,
            platform,
            background_executor,
            foreground_executor,
            dispatcher,
            text_system,
        }
    }

    /// Run a closure with mutable access to the App context.
    /// Automatically runs until parked after the closure completes.
    pub fn update<R>(&mut self, f: impl FnOnce(&mut App) -> R) -> R {
        let result = {
            let mut app = self.app.borrow_mut();
            app.update(f)
        };
        self.run_until_parked();
        result
    }

    /// Run a closure with read-only access to the App context.
    pub fn read<R>(&self, f: impl FnOnce(&App) -> R) -> R {
        let app = self.app.borrow();
        f(&app)
    }

    /// Create a new entity in the app.
    pub fn new_entity<T: 'static>(
        &mut self,
        build: impl FnOnce(&mut Context<T>) -> T,
    ) -> Entity<T> {
        self.update(|cx| cx.new(build))
    }

    /// Update an entity.
    pub fn update_entity<T: 'static, R>(
        &mut self,
        entity: &Entity<T>,
        f: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> R {
        self.update(|cx| entity.update(cx, f))
    }

    /// Read an entity.
    pub fn read_entity<T: 'static, R>(
        &self,
        entity: &Entity<T>,
        f: impl FnOnce(&T, &App) -> R,
    ) -> R {
        self.read(|cx| f(entity.read(cx), cx))
    }

    /// Open a test window with the given root view.
    pub fn open_window<V: Render + 'static>(
        &mut self,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> TestWindow<V> {
        let bounds = self.read(|cx| Bounds::maximized(None, cx));
        let handle = self.update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| build_view(window, cx)),
            )
            .unwrap()
        });

        TestWindow {
            handle,
            app: self.app.clone(),
            platform: self.platform.clone(),
            background_executor: self.background_executor.clone(),
        }
    }

    /// Open a test window with specific options.
    pub fn open_window_with_options<V: Render + 'static>(
        &mut self,
        options: WindowOptions,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> TestWindow<V> {
        let handle = self.update(|cx| {
            cx.open_window(options, |window, cx| cx.new(|cx| build_view(window, cx)))
                .unwrap()
        });

        TestWindow {
            handle,
            app: self.app.clone(),
            platform: self.platform.clone(),
            background_executor: self.background_executor.clone(),
        }
    }

    /// Run pending tasks until there's nothing left to do.
    pub fn run_until_parked(&self) {
        self.background_executor.run_until_parked();
    }

    /// Advance the simulated clock by the given duration.
    pub fn advance_clock(&self, duration: Duration) {
        self.background_executor.advance_clock(duration);
    }

    /// Spawn a future on the foreground executor.
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncApp) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn(f(self.to_async()))
    }

    /// Spawn a future on the background executor.
    pub fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.background_executor.spawn(future)
    }

    /// Get an async handle to the app.
    pub fn to_async(&self) -> AsyncApp {
        AsyncApp {
            app: Rc::downgrade(&self.app),
            background_executor: self.background_executor.clone(),
            foreground_executor: self.foreground_executor.clone(),
        }
    }

    /// Get the background executor.
    pub fn background_executor(&self) -> &BackgroundExecutor {
        &self.background_executor
    }

    /// Get the foreground executor.
    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        &self.foreground_executor
    }

    /// Get the text system.
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    /// Check if a global of the given type exists.
    pub fn has_global<G: Global>(&self) -> bool {
        self.read(|cx| cx.has_global::<G>())
    }

    /// Set a global value.
    pub fn set_global<G: Global>(&mut self, global: G) {
        self.update(|cx| cx.set_global(global));
    }

    /// Read a global value.
    pub fn read_global<G: Global, R>(&self, f: impl FnOnce(&G, &App) -> R) -> R {
        self.read(|cx| f(cx.global(), cx))
    }

    /// Update a global value.
    pub fn update_global<G: Global, R>(&mut self, f: impl FnOnce(&mut G, &mut App) -> R) -> R {
        self.update(|cx| cx.update_global(f))
    }

    // Platform simulation methods

    /// Write text to the simulated clipboard.
    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform.write_to_clipboard(item);
    }

    /// Read from the simulated clipboard.
    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform.read_from_clipboard()
    }

    /// Get URLs that have been opened via `cx.open_url()`.
    pub fn opened_url(&self) -> Option<String> {
        self.platform.opened_url.borrow().clone()
    }

    /// Check if a file path prompt is pending.
    pub fn did_prompt_for_new_path(&self) -> bool {
        self.platform.did_prompt_for_new_path()
    }

    /// Simulate answering a path selection dialog.
    pub fn simulate_new_path_selection(
        &self,
        select: impl FnOnce(&std::path::Path) -> Option<std::path::PathBuf>,
    ) {
        self.platform.simulate_new_path_selection(select);
    }

    /// Check if a prompt dialog is pending.
    pub fn has_pending_prompt(&self) -> bool {
        self.platform.has_pending_prompt()
    }

    /// Simulate answering a prompt dialog.
    pub fn simulate_prompt_answer(&self, button: &str) {
        self.platform.simulate_prompt_answer(button);
    }

    /// Get all open windows.
    pub fn windows(&self) -> Vec<AnyWindowHandle> {
        self.read(|cx| cx.windows())
    }
}

impl Default for TestApp {
    fn default() -> Self {
        Self::new()
    }
}

/// A test window with inspection and simulation capabilities.
pub struct TestWindow<V> {
    handle: WindowHandle<V>,
    app: Rc<AppCell>,
    platform: Rc<TestPlatform>,
    background_executor: BackgroundExecutor,
}

impl<V: 'static + Render> TestWindow<V> {
    /// Get the window handle.
    pub fn handle(&self) -> WindowHandle<V> {
        self.handle
    }

    /// Get the root view entity.
    pub fn root(&self) -> Entity<V> {
        let mut app = self.app.borrow_mut();
        let any_handle: AnyWindowHandle = self.handle.into();
        app.update_window(any_handle, |root_view, _, _| {
            root_view.downcast::<V>().expect("root view type mismatch")
        })
        .expect("window not found")
    }

    /// Update the root view.
    /// Automatically draws the window after the update to ensure the scene is current.
    pub fn update<R>(&mut self, f: impl FnOnce(&mut V, &mut Window, &mut Context<V>) -> R) -> R {
        let result = {
            let mut app = self.app.borrow_mut();
            let any_handle: AnyWindowHandle = self.handle.into();
            app.update_window(any_handle, |root_view, window, cx| {
                let view = root_view.downcast::<V>().expect("root view type mismatch");
                view.update(cx, |view, cx| f(view, window, cx))
            })
            .expect("window not found")
        };
        self.background_executor.run_until_parked();
        self.draw();
        result
    }

    /// Read the root view.
    pub fn read<R>(&self, f: impl FnOnce(&V, &App) -> R) -> R {
        let app = self.app.borrow();
        let view = self
            .app
            .borrow()
            .windows
            .get(self.handle.window_id())
            .and_then(|w| w.as_ref())
            .and_then(|w| w.root.clone())
            .and_then(|r| r.downcast::<V>().ok())
            .expect("window or root view not found");
        f(view.read(&app), &app)
    }

    /// Get the window title.
    pub fn title(&self) -> Option<String> {
        let app = self.app.borrow();
        app.read_window(&self.handle, |_, _cx| {
            // TODO: expose title through Window API
            None
        })
        .unwrap()
    }

    /// Simulate a keystroke.
    /// Automatically draws the window after the keystroke.
    pub fn simulate_keystroke(&mut self, keystroke: &str) {
        let keystroke = Keystroke::parse(keystroke).unwrap();
        {
            let mut app = self.app.borrow_mut();
            let any_handle: AnyWindowHandle = self.handle.into();
            app.update_window(any_handle, |_, window, cx| {
                window.dispatch_keystroke(keystroke, cx);
            })
            .unwrap();
        }
        self.background_executor.run_until_parked();
        self.draw();
    }

    /// Simulate multiple keystrokes (space-separated).
    pub fn simulate_keystrokes(&mut self, keystrokes: &str) {
        for keystroke in keystrokes.split(' ') {
            self.simulate_keystroke(keystroke);
        }
    }

    /// Simulate typing text.
    pub fn simulate_input(&mut self, input: &str) {
        for char in input.chars() {
            self.simulate_keystroke(&char.to_string());
        }
    }

    /// Simulate a mouse move.
    pub fn simulate_mouse_move(&mut self, position: Point<Pixels>) {
        self.simulate_event(MouseMoveEvent {
            position,
            modifiers: Default::default(),
            pressed_button: None,
        });
    }

    /// Simulate a mouse down event.
    pub fn simulate_mouse_down(&mut self, position: Point<Pixels>, button: MouseButton) {
        self.simulate_event(MouseDownEvent {
            position,
            button,
            modifiers: Default::default(),
            click_count: 1,
            first_mouse: false,
        });
    }

    /// Simulate a mouse up event.
    pub fn simulate_mouse_up(&mut self, position: Point<Pixels>, button: MouseButton) {
        self.simulate_event(MouseUpEvent {
            position,
            button,
            modifiers: Default::default(),
            click_count: 1,
        });
    }

    /// Simulate a click at the given position.
    pub fn simulate_click(&mut self, position: Point<Pixels>, button: MouseButton) {
        self.simulate_mouse_down(position, button);
        self.simulate_mouse_up(position, button);
    }

    /// Simulate a scroll event.
    pub fn simulate_scroll(&mut self, position: Point<Pixels>, delta: Point<Pixels>) {
        self.simulate_event(crate::ScrollWheelEvent {
            position,
            delta: crate::ScrollDelta::Pixels(delta),
            modifiers: Default::default(),
            touch_phase: crate::TouchPhase::Moved,
        });
    }

    /// Simulate an input event.
    /// Automatically draws the window after the event.
    pub fn simulate_event<E: InputEvent>(&mut self, event: E) {
        let platform_input = event.to_platform_input();
        {
            let mut app = self.app.borrow_mut();
            let any_handle: AnyWindowHandle = self.handle.into();
            app.update_window(any_handle, |_, window, cx| {
                window.dispatch_event(platform_input, cx);
            })
            .unwrap();
        }
        self.background_executor.run_until_parked();
        self.draw();
    }

    /// Simulate resizing the window.
    /// Automatically draws the window after the resize.
    pub fn simulate_resize(&mut self, size: Size<Pixels>) {
        let window_id = self.handle.window_id();
        let mut app = self.app.borrow_mut();
        if let Some(Some(window)) = app.windows.get_mut(window_id) {
            if let Some(test_window) = window.platform_window.as_test() {
                test_window.simulate_resize(size);
            }
        }
        drop(app);
        self.background_executor.run_until_parked();
        self.draw();
    }

    /// Force a redraw of the window.
    pub fn draw(&mut self) {
        let mut app = self.app.borrow_mut();
        let any_handle: AnyWindowHandle = self.handle.into();
        app.update_window(any_handle, |_, window, cx| {
            window.draw(cx).clear();
        })
        .unwrap();
    }

    /// Get a snapshot of the rendered scene for inspection.
    /// The scene is automatically kept up to date after `update()` and `simulate_*()` calls.
    pub fn scene_snapshot(&self) -> SceneSnapshot {
        let app = self.app.borrow();
        let window = app
            .windows
            .get(self.handle.window_id())
            .and_then(|w| w.as_ref())
            .expect("window not found");
        window.rendered_frame.scene.snapshot()
    }

    /// Get the named diagnostic quads recorded during imperative paint, without inspecting the
    /// rest of the scene snapshot.
    ///
    /// This is useful for tests that want a stable, semantic view of layout/paint geometry without
    /// coupling to the low-level quad/glyph output.
    pub fn diagnostic_quads(&self) -> Vec<crate::scene::test_scene::DiagnosticQuad> {
        self.scene_snapshot().diagnostic_quads
    }
}

impl<V> Clone for TestWindow<V> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            app: self.app.clone(),
            platform: self.platform.clone(),
            background_executor: self.background_executor.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FocusHandle, Focusable, div, prelude::*};

    struct Counter {
        count: usize,
        focus_handle: FocusHandle,
    }

    impl Counter {
        fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
            let focus_handle = cx.focus_handle();
            Self {
                count: 0,
                focus_handle,
            }
        }

        fn increment(&mut self, _cx: &mut Context<Self>) {
            self.count += 1;
        }
    }

    impl Focusable for Counter {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl Render for Counter {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(format!("Count: {}", self.count))
        }
    }

    #[test]
    fn test_basic_usage() {
        let mut app = TestApp::new();

        let mut window = app.open_window(Counter::new);

        window.update(|counter, _window, cx| {
            counter.increment(cx);
        });

        window.read(|counter, _| {
            assert_eq!(counter.count, 1);
        });
    }

    #[test]
    fn test_entity_creation() {
        let mut app = TestApp::new();

        let entity = app.new_entity(|cx| Counter {
            count: 42,
            focus_handle: cx.focus_handle(),
        });

        app.read_entity(&entity, |counter, _| {
            assert_eq!(counter.count, 42);
        });

        app.update_entity(&entity, |counter, _cx| {
            counter.count += 1;
        });

        app.read_entity(&entity, |counter, _| {
            assert_eq!(counter.count, 43);
        });
    }

    #[test]
    fn test_globals() {
        let mut app = TestApp::new();

        struct MyGlobal(String);
        impl Global for MyGlobal {}

        assert!(!app.has_global::<MyGlobal>());

        app.set_global(MyGlobal("hello".into()));

        assert!(app.has_global::<MyGlobal>());

        app.read_global::<MyGlobal, _>(|global, _| {
            assert_eq!(global.0, "hello");
        });

        app.update_global::<MyGlobal, _>(|global, _| {
            global.0 = "world".into();
        });

        app.read_global::<MyGlobal, _>(|global, _| {
            assert_eq!(global.0, "world");
        });
    }
}
