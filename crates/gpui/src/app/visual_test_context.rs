use crate::{
    Action, AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, Bounds,
    ClipboardItem, Context, Entity, ForegroundExecutor, Global, InputEvent, Keystroke, Modifiers,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Platform, Point, Render,
    Result, Size, Task, TextSystem, Window, WindowBounds, WindowHandle, WindowOptions,
    app::GpuiMode, current_platform,
};
use anyhow::anyhow;
use image::RgbaImage;
use std::{future::Future, rc::Rc, sync::Arc, time::Duration};

/// A test context that uses real macOS rendering instead of mocked rendering.
/// This is used for visual tests that need to capture actual screenshots.
///
/// Unlike `TestAppContext` which uses `TestPlatform` with mocked rendering,
/// `VisualTestAppContext` uses the real `MacPlatform` to produce actual rendered output.
///
/// Windows created through this context are positioned off-screen (at coordinates like -10000, -10000)
/// so they are invisible to the user but still fully rendered by the compositor.
#[derive(Clone)]
pub struct VisualTestAppContext {
    /// The underlying app cell
    pub app: Rc<AppCell>,
    /// The background executor for running async tasks
    pub background_executor: BackgroundExecutor,
    /// The foreground executor for running tasks on the main thread
    pub foreground_executor: ForegroundExecutor,
    platform: Rc<dyn Platform>,
    text_system: Arc<TextSystem>,
}

impl VisualTestAppContext {
    /// Creates a new `VisualTestAppContext` with real macOS platform rendering.
    ///
    /// This initializes the real macOS platform (not the test platform), which means:
    /// - Windows are actually rendered by Metal/the compositor
    /// - Screenshots can be captured via ScreenCaptureKit
    /// - All platform APIs work as they do in production
    pub fn new() -> Self {
        let platform = current_platform(false);
        let background_executor = platform.background_executor();
        let foreground_executor = platform.foreground_executor();
        let text_system = Arc::new(TextSystem::new(platform.text_system()));

        let asset_source = Arc::new(());
        let http_client = http_client::FakeHttpClient::with_404_response();

        let mut app = App::new_app(platform.clone(), asset_source, http_client);
        app.borrow_mut().mode = GpuiMode::test();

        Self {
            app,
            background_executor,
            foreground_executor,
            platform,
            text_system,
        }
    }

    /// Opens a window positioned off-screen for invisible rendering.
    ///
    /// The window is positioned at (-10000, -10000) so it's not visible on any display,
    /// but it's still fully rendered by the compositor and can be captured via ScreenCaptureKit.
    ///
    /// # Arguments
    /// * `size` - The size of the window to create
    /// * `build_root` - A closure that builds the root view for the window
    pub fn open_offscreen_window<V: Render + 'static>(
        &mut self,
        size: Size<Pixels>,
        build_root: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
    ) -> Result<WindowHandle<V>> {
        use crate::{point, px};

        let bounds = Bounds {
            origin: point(px(-10000.0), px(-10000.0)),
            size,
        };

        let mut cx = self.app.borrow_mut();
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                focus: false,
                show: true,
                ..Default::default()
            },
            build_root,
        )
    }

    /// Opens an off-screen window with default size (1280x800).
    pub fn open_offscreen_window_default<V: Render + 'static>(
        &mut self,
        build_root: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
    ) -> Result<WindowHandle<V>> {
        use crate::{px, size};
        self.open_offscreen_window(size(px(1280.0), px(800.0)), build_root)
    }

    /// Returns whether screen capture is supported on this platform.
    pub fn is_screen_capture_supported(&self) -> bool {
        self.platform.is_screen_capture_supported()
    }

    /// Returns the text system used by this context.
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    /// Returns the background executor.
    pub fn executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    /// Returns the foreground executor.
    pub fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    /// Runs pending background tasks until there's nothing left to do.
    pub fn run_until_parked(&self) {
        self.background_executor.run_until_parked();
    }

    /// Updates the app state.
    pub fn update<R>(&mut self, f: impl FnOnce(&mut App) -> R) -> R {
        let mut app = self.app.borrow_mut();
        f(&mut app)
    }

    /// Reads from the app state.
    pub fn read<R>(&self, f: impl FnOnce(&App) -> R) -> R {
        let app = self.app.borrow();
        f(&app)
    }

    /// Updates a window.
    pub fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        let mut lock = self.app.borrow_mut();
        lock.update_window(window, f)
    }

    /// Spawns a task on the foreground executor.
    pub fn spawn<F, R>(&self, f: F) -> Task<R>
    where
        F: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn(f)
    }

    /// Checks if a global of type G exists.
    pub fn has_global<G: Global>(&self) -> bool {
        let app = self.app.borrow();
        app.has_global::<G>()
    }

    /// Reads a global value.
    pub fn read_global<G: Global, R>(&self, f: impl FnOnce(&G, &App) -> R) -> R {
        let app = self.app.borrow();
        f(app.global::<G>(), &app)
    }

    /// Sets a global value.
    pub fn set_global<G: Global>(&mut self, global: G) {
        let mut app = self.app.borrow_mut();
        app.set_global(global);
    }

    /// Updates a global value.
    pub fn update_global<G: Global, R>(&mut self, f: impl FnOnce(&mut G, &mut App) -> R) -> R {
        let mut lock = self.app.borrow_mut();
        lock.update(|cx| {
            let mut global = cx.lease_global::<G>();
            let result = f(&mut global, cx);
            cx.end_global_lease(global);
            result
        })
    }

    /// Simulates a sequence of keystrokes on the given window.
    ///
    /// Keystrokes are specified as a space-separated string, e.g., "cmd-p escape".
    pub fn simulate_keystrokes(&mut self, window: AnyWindowHandle, keystrokes: &str) {
        for keystroke_text in keystrokes.split_whitespace() {
            let keystroke = Keystroke::parse(keystroke_text)
                .unwrap_or_else(|_| panic!("Invalid keystroke: {}", keystroke_text));
            self.dispatch_keystroke(window, keystroke);
        }
        self.run_until_parked();
    }

    /// Dispatches a single keystroke to a window.
    pub fn dispatch_keystroke(&mut self, window: AnyWindowHandle, keystroke: Keystroke) {
        self.update_window(window, |_, window, cx| {
            window.dispatch_keystroke(keystroke, cx);
        })
        .ok();
    }

    /// Simulates typing text input on the given window.
    pub fn simulate_input(&mut self, window: AnyWindowHandle, input: &str) {
        for char in input.chars() {
            let key = char.to_string();
            let keystroke = Keystroke {
                modifiers: Modifiers::default(),
                key: key.clone(),
                key_char: Some(key),
            };
            self.dispatch_keystroke(window, keystroke);
        }
        self.run_until_parked();
    }

    /// Simulates a mouse move event.
    pub fn simulate_mouse_move(
        &mut self,
        window: AnyWindowHandle,
        position: Point<Pixels>,
        button: impl Into<Option<MouseButton>>,
        modifiers: Modifiers,
    ) {
        self.simulate_event(
            window,
            MouseMoveEvent {
                position,
                modifiers,
                pressed_button: button.into(),
            },
        );
    }

    /// Simulates a mouse down event.
    pub fn simulate_mouse_down(
        &mut self,
        window: AnyWindowHandle,
        position: Point<Pixels>,
        button: MouseButton,
        modifiers: Modifiers,
    ) {
        self.simulate_event(
            window,
            MouseDownEvent {
                position,
                modifiers,
                button,
                click_count: 1,
                first_mouse: false,
            },
        );
    }

    /// Simulates a mouse up event.
    pub fn simulate_mouse_up(
        &mut self,
        window: AnyWindowHandle,
        position: Point<Pixels>,
        button: MouseButton,
        modifiers: Modifiers,
    ) {
        self.simulate_event(
            window,
            MouseUpEvent {
                position,
                modifiers,
                button,
                click_count: 1,
            },
        );
    }

    /// Simulates a click (mouse down followed by mouse up).
    pub fn simulate_click(
        &mut self,
        window: AnyWindowHandle,
        position: Point<Pixels>,
        modifiers: Modifiers,
    ) {
        self.simulate_mouse_down(window, position, MouseButton::Left, modifiers);
        self.simulate_mouse_up(window, position, MouseButton::Left, modifiers);
    }

    /// Simulates an input event on the given window.
    pub fn simulate_event<E: InputEvent>(&mut self, window: AnyWindowHandle, event: E) {
        self.update_window(window, |_, window, cx| {
            window.dispatch_event(event.to_platform_input(), cx);
        })
        .ok();
        self.run_until_parked();
    }

    /// Dispatches an action to the given window.
    pub fn dispatch_action(&mut self, window: AnyWindowHandle, action: impl Action) {
        self.update_window(window, |_, window, cx| {
            window.dispatch_action(action.boxed_clone(), cx);
        })
        .ok();
        self.run_until_parked();
    }

    /// Writes to the clipboard.
    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform.write_to_clipboard(item);
    }

    /// Reads from the clipboard.
    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform.read_from_clipboard()
    }

    /// Waits for a condition to become true, with a timeout.
    pub async fn wait_for<T: 'static>(
        &mut self,
        entity: &Entity<T>,
        predicate: impl Fn(&T) -> bool,
        timeout: Duration,
    ) -> Result<()> {
        let start = std::time::Instant::now();
        loop {
            {
                let app = self.app.borrow();
                if predicate(entity.read(&app)) {
                    return Ok(());
                }
            }

            if start.elapsed() > timeout {
                return Err(anyhow!("Timed out waiting for condition"));
            }

            self.run_until_parked();
            self.background_executor
                .timer(Duration::from_millis(10))
                .await;
        }
    }

    /// Captures a screenshot of the specified window using direct texture capture.
    ///
    /// This renders the scene to a Metal texture and reads the pixels directly,
    /// which does not require the window to be visible on screen.
    #[cfg(any(test, feature = "test-support"))]
    pub fn capture_screenshot(&mut self, window: AnyWindowHandle) -> Result<RgbaImage> {
        self.update_window(window, |_, window, _cx| window.render_to_image())?
    }

    /// Waits for animations to complete by waiting a couple of frames.
    pub async fn wait_for_animations(&self) {
        self.background_executor
            .timer(Duration::from_millis(32))
            .await;
        self.run_until_parked();
    }
}

impl Default for VisualTestAppContext {
    fn default() -> Self {
        Self::new()
    }
}

impl AppContext for VisualTestAppContext {
    type Result<T> = T;

    fn new<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>> {
        let mut app = self.app.borrow_mut();
        app.new(build_entity)
    }

    fn reserve_entity<T: 'static>(&mut self) -> Self::Result<crate::Reservation<T>> {
        let mut app = self.app.borrow_mut();
        app.reserve_entity()
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: crate::Reservation<T>,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>> {
        let mut app = self.app.borrow_mut();
        app.insert_entity(reservation, build_entity)
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Self::Result<R> {
        let mut app = self.app.borrow_mut();
        app.update_entity(handle, update)
    }

    fn as_mut<'a, T>(&'a mut self, _: &Entity<T>) -> Self::Result<crate::GpuiBorrow<'a, T>>
    where
        T: 'static,
    {
        panic!("Cannot use as_mut with a visual test app context. Try calling update() first")
    }

    fn read_entity<T, R>(
        &self,
        handle: &Entity<T>,
        read: impl FnOnce(&T, &App) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let app = self.app.borrow();
        app.read_entity(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        let mut lock = self.app.borrow_mut();
        lock.update_window(window, f)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let app = self.app.borrow();
        app.read_window(window, read)
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.background_executor.spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> Self::Result<R>
    where
        G: Global,
    {
        let app = self.app.borrow();
        callback(app.global::<G>(), &app)
    }
}
