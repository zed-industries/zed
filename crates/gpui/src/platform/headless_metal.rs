//! Headless Metal platform for visual tests without AppKit dependencies.
//!
//! This module provides a platform implementation that can render GPUI scenes
//! to images using Metal, without requiring a window, display, or the main thread.
//!
//! # Thread Safety
//!
//! Unlike `VisualTestAppContext`, this platform does not use AppKit and can
//! safely run on any thread, including Rust test worker threads.

#![cfg(all(target_os = "macos", any(test, feature = "test-support")))]

use crate::{
    AnyWindowHandle, App, AppCell, AppContext, AssetSource, AtlasKey, AtlasTextureId, AtlasTile,
    BackgroundExecutor, Bounds, ClipboardItem, CursorStyle, DevicePixels, DispatchEventResult,
    DummyKeyboardMapper, Entity, ForegroundExecutor, GpuSpecs, Keymap, NoopTextSystem, Pixels,
    Platform, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Point,
    PromptButton, PromptLevel, Render, RequestFrameOptions, Scene, Size, Task, TestDispatcher,
    TestDisplay, TextSystem, TileId, Window, WindowAppearance, WindowBackgroundAppearance,
    WindowBounds, WindowControlArea, WindowHandle, WindowOptions, WindowParams, app::GpuiMode,
};
use anyhow::Result;
use collections::HashMap;
use futures::channel::oneshot;
use image::RgbaImage;
use parking_lot::Mutex;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{self, Arc},
    time::Duration,
};

use super::mac::metal_renderer::{InstanceBufferPool, MetalRenderer};

#[cfg(feature = "font-kit")]
use super::mac::MacTextSystem;

// ─────────────────────────────────────────────────────────────────────────────
// Headless Metal Platform
// ─────────────────────────────────────────────────────────────────────────────

/// A platform that uses Metal for rendering without any AppKit dependencies.
///
/// This allows visual tests to run on any thread, not just the main thread.
pub struct HeadlessMetalPlatform {
    dispatcher: TestDispatcher,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    active_display: Rc<dyn PlatformDisplay>,
    active_cursor: Mutex<CursorStyle>,
    clipboard: Mutex<Option<ClipboardItem>>,
    find_pasteboard: Mutex<Option<ClipboardItem>>,
    renderer_context: Arc<Mutex<InstanceBufferPool>>,
    weak: RefCell<Weak<Self>>,
}

impl HeadlessMetalPlatform {
    /// Creates a new HeadlessMetalPlatform with the given random seed.
    pub fn new(seed: u64) -> Rc<Self> {
        let dispatcher = TestDispatcher::new(seed);
        let arc_dispatcher = Arc::new(dispatcher.clone());

        let background_executor = BackgroundExecutor::new(arc_dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(arc_dispatcher);

        #[cfg(feature = "font-kit")]
        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(MacTextSystem::new());

        #[cfg(not(feature = "font-kit"))]
        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(NoopTextSystem::new());
        let active_display = Rc::new(TestDisplay::new());
        let renderer_context = Arc::new(Mutex::new(InstanceBufferPool::default()));

        Rc::new_cyclic(|weak| Self {
            dispatcher,
            background_executor,
            foreground_executor,
            text_system,
            active_display,
            active_cursor: Mutex::new(CursorStyle::Arrow),
            clipboard: Mutex::new(None),
            find_pasteboard: Mutex::new(None),
            renderer_context,
            weak: RefCell::new(weak.clone()),
        })
    }

    /// Returns a reference to the TestDispatcher for controlling task scheduling.
    pub fn dispatcher(&self) -> &TestDispatcher {
        &self.dispatcher
    }

    /// Runs all pending tasks until there's nothing left to do.
    pub fn run_until_parked(&self) {
        self.dispatcher.run_until_parked();
    }

    /// Advances the simulated clock by the given duration.
    pub fn advance_clock(&self, duration: Duration) {
        self.dispatcher.advance_clock(duration);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Headless Metal App Context
// ─────────────────────────────────────────────────────────────────────────────

/// App context for headless Metal rendering tests.
///
/// Unlike `VisualTestAppContext`, this can run on any thread because it doesn't
/// use AppKit. It provides real Metal rendering for accurate screenshots.
pub struct HeadlessMetalAppContext {
    /// The underlying app cell
    pub app: Rc<AppCell>,
    /// The background executor for running async tasks
    pub background_executor: BackgroundExecutor,
    /// The foreground executor for running tasks on the main thread
    pub foreground_executor: ForegroundExecutor,
    /// The test dispatcher for deterministic task scheduling
    dispatcher: TestDispatcher,
    platform: Rc<HeadlessMetalPlatform>,
    text_system: Arc<TextSystem>,
}

impl HeadlessMetalAppContext {
    /// Creates a new headless Metal app context.
    pub fn new() -> Self {
        Self::with_asset_source(Arc::new(()))
    }

    /// Creates a new headless Metal app context with a custom asset source.
    pub fn with_asset_source(asset_source: Arc<dyn AssetSource>) -> Self {
        let seed = std::env::var("SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let platform = HeadlessMetalPlatform::new(seed);
        let dispatcher = platform.dispatcher().clone();
        let background_executor = platform.background_executor();
        let foreground_executor = platform.foreground_executor();
        let text_system = Arc::new(TextSystem::new(platform.text_system()));

        let http_client = http_client::FakeHttpClient::with_404_response();
        let mut app = App::new_app(platform.clone(), asset_source, http_client);
        app.borrow_mut().mode = GpuiMode::test();

        Self {
            app,
            background_executor,
            foreground_executor,
            dispatcher,
            platform,
            text_system,
        }
    }

    /// Opens a window for headless rendering.
    pub fn open_window<V: Render + 'static>(
        &mut self,
        size: Size<Pixels>,
        build_root: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
    ) -> Result<WindowHandle<V>> {
        use crate::{point, px};

        let bounds = Bounds {
            origin: point(px(0.0), px(0.0)),
            size,
        };

        let mut cx = self.app.borrow_mut();
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                focus: false,
                show: false,
                ..Default::default()
            },
            build_root,
        )
    }

    /// Runs all pending tasks until parked.
    pub fn run_until_parked(&self) {
        self.dispatcher.run_until_parked();
    }

    /// Advances the simulated clock.
    pub fn advance_clock(&self, duration: Duration) {
        self.dispatcher.advance_clock(duration);
    }

    /// Enables parking mode, allowing blocking on real I/O (e.g., async asset loading).
    ///
    /// When parking is allowed, `block_on` calls will actually wait for I/O completion
    /// instead of panicking. This is useful for visual tests that need to load embedded
    /// assets like images.
    pub fn allow_parking(&self) {
        self.dispatcher.allow_parking();
    }

    /// Disables parking mode, returning to deterministic test execution.
    ///
    /// Call this after assets have loaded to avoid 100ms sleep intervals when
    /// `run_until_parked()` finds no work to do.
    pub fn forbid_parking(&self) {
        self.dispatcher.forbid_parking();
    }

    /// Updates app state.
    pub fn update<R>(&mut self, f: impl FnOnce(&mut App) -> R) -> R {
        let mut app = self.app.borrow_mut();
        f(&mut app)
    }

    /// Updates a window and calls draw to render.
    pub fn update_window<R>(
        &mut self,
        window: AnyWindowHandle,
        f: impl FnOnce(crate::AnyView, &mut Window, &mut App) -> R,
    ) -> Result<R> {
        let mut app = self.app.borrow_mut();
        app.update_window(window, f)
    }

    /// Captures a screenshot from a window.
    pub fn capture_screenshot(&mut self, window: AnyWindowHandle) -> Result<RgbaImage> {
        let mut app = self.app.borrow_mut();
        app.update_window(window, |_, window, _| window.render_to_image())
            .map_err(|e| anyhow::anyhow!("Failed to update window: {:?}", e))?
    }

    /// Returns the text system.
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    /// Returns the background executor.
    pub fn background_executor(&self) -> &BackgroundExecutor {
        &self.background_executor
    }

    /// Returns the foreground executor.
    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        &self.foreground_executor
    }
}

impl Default for HeadlessMetalAppContext {
    fn default() -> Self {
        Self::new()
    }
}

struct HeadlessKeyboardLayout;

impl PlatformKeyboardLayout for HeadlessKeyboardLayout {
    fn id(&self) -> &str {
        "headless.keyboard"
    }

    fn name(&self) -> &str {
        "Headless Keyboard"
    }
}

impl Platform for HeadlessMetalPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(HeadlessKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper {})
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {}

    fn run(&self, _on_finish_launching: Box<dyn 'static + FnOnce()>) {
        panic!("HeadlessMetalPlatform::run should not be called - use run_until_parked() instead")
    }

    fn quit(&self) {}

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.active_display.clone()]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.active_display.clone())
    }

    #[cfg(feature = "screen-capture")]
    fn is_screen_capture_supported(&self) -> bool {
        false
    }

    #[cfg(feature = "screen-capture")]
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Rc<dyn crate::ScreenCaptureSource>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(Vec::new())).ok();
        rx
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let window = HeadlessMetalWindow::new(
            handle,
            options,
            self.weak.borrow().clone(),
            self.active_display.clone(),
            self.renderer_context.clone(),
        );
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: crate::PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(None)).ok();
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(None)).ok();
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        true
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn crate::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn crate::Action) -> bool>) {}

    fn app_path(&self) -> Result<PathBuf> {
        Ok(PathBuf::from("/dev/null"))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!("not supported in headless mode"))
    }

    fn set_menus(&self, _menus: Vec<crate::Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<crate::MenuItem>, _keymap: &Keymap) {}

    fn add_recent_document(&self, _path: &Path) {}

    fn set_cursor_style(&self, style: CursorStyle) {
        *self.active_cursor.lock() = style;
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        *self.clipboard.lock() = Some(item);
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.clipboard.lock().clone()
    }

    fn read_from_find_pasteboard(&self) -> Option<ClipboardItem> {
        self.find_pasteboard.lock().clone()
    }

    fn write_to_find_pasteboard(&self, item: ClipboardItem) {
        *self.find_pasteboard.lock() = Some(item);
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Headless Metal Window
// ─────────────────────────────────────────────────────────────────────────────

struct HeadlessMetalWindowState {
    bounds: Bounds<Pixels>,
    handle: AnyWindowHandle,
    display: Rc<dyn PlatformDisplay>,
    #[allow(dead_code)]
    platform: Weak<HeadlessMetalPlatform>,
    renderer: MetalRenderer,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    last_image: Option<RgbaImage>,
    title: Option<String>,
    edited: bool,
    input_handler: Option<PlatformInputHandler>,
    should_close_handler: Option<Box<dyn FnMut() -> bool>>,
    input_callback: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    hover_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    hit_test_window_control_callback: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
}

/// A headless window that renders to Metal textures without requiring a display.
pub struct HeadlessMetalWindow(Mutex<HeadlessMetalWindowState>);

impl HeadlessMetalWindow {
    fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        platform: Weak<HeadlessMetalPlatform>,
        display: Rc<dyn PlatformDisplay>,
        renderer_context: Arc<Mutex<InstanceBufferPool>>,
    ) -> Self {
        let renderer = MetalRenderer::new_headless(renderer_context);
        let sprite_atlas = renderer.sprite_atlas().clone();

        Self(Mutex::new(HeadlessMetalWindowState {
            bounds: params.bounds,
            handle,
            display,
            platform,
            renderer,
            sprite_atlas,
            last_image: None,
            title: None,
            edited: false,
            input_handler: None,
            should_close_handler: None,
            input_callback: None,
            active_status_change_callback: None,
            hover_status_change_callback: None,
            resize_callback: None,
            moved_callback: None,
            hit_test_window_control_callback: None,
        }))
    }

    /// Returns the last rendered image.
    pub fn last_rendered_image(&self) -> Result<RgbaImage> {
        let state = self.0.lock();
        state
            .last_image
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No scene has been drawn yet"))
    }
}

impl HasWindowHandle for HeadlessMetalWindow {
    fn window_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError>
    {
        Err(raw_window_handle::HandleError::Unavailable)
    }
}

impl HasDisplayHandle for HeadlessMetalWindow {
    fn display_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError>
    {
        Err(raw_window_handle::HandleError::Unavailable)
    }
}

impl PlatformWindow for HeadlessMetalWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.0.lock().bounds
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds().size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.0.lock().bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        2.0 // Retina
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.0.lock().display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> crate::Modifiers {
        crate::Modifiers::default()
    }

    fn capslock(&self) -> crate::Capslock {
        crate::Capslock::default()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.lock().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.lock().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {}

    fn is_active(&self) -> bool {
        false
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn set_title(&mut self, title: &str) {
        self.0.lock().title = Some(title.to_string());
    }

    fn set_app_id(&mut self, _app_id: &str) {}

    fn set_edited(&mut self, edited: bool) {
        self.0.lock().edited = edited;
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}

    fn show_character_palette(&self) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        false
    }

    fn on_request_frame(&self, _callback: Box<dyn FnMut(RequestFrameOptions)>) {}

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.0.lock().input_callback = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().active_status_change_callback = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().hover_status_change_callback = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.lock().resize_callback = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().moved_callback = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.lock().should_close_handler = Some(callback);
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.0.lock().hit_test_window_control_callback = Some(callback);
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn draw(&self, scene: &Scene) {
        let mut state = self.0.lock();
        let size = state.bounds.size;
        let scale_factor = 2.0; // Retina
        let device_size = size.to_device_pixels(scale_factor);

        // Render immediately and store the result
        match state.renderer.render_scene_to_image(scene, device_size) {
            Ok(image) => {
                state.last_image = Some(image);
            }
            Err(e) => {
                log::error!("Failed to render scene to image: {}", e);
            }
        }
    }

    fn render_to_image(&self, scene: &Scene) -> Result<RgbaImage> {
        let mut state = self.0.lock();
        let size = state.bounds.size;
        let scale_factor = 2.0; // Retina
        let device_size = size.to_device_pixels(scale_factor);
        state.renderer.render_scene_to_image(scene, device_size)
    }

    fn sprite_atlas(&self) -> sync::Arc<dyn PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }

    fn show_window_menu(&self, _position: Point<Pixels>) {}

    fn start_window_move(&self) {}

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Headless Atlas (for sprite storage)
// ─────────────────────────────────────────────────────────────────────────────

struct HeadlessAtlasState {
    next_id: u32,
    tiles: HashMap<AtlasKey, AtlasTile>,
}

struct HeadlessAtlas(Mutex<HeadlessAtlasState>);

impl HeadlessAtlas {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(Mutex::new(HeadlessAtlasState {
            next_id: 0,
            tiles: HashMap::default(),
        }))
    }
}

impl PlatformAtlas for HeadlessAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>> {
        let mut state = self.0.lock();
        if let Some(tile) = state.tiles.get(key) {
            return Ok(Some(tile.clone()));
        }

        let Some((size, _data)) = build()? else {
            return Ok(None);
        };

        let id = state.next_id;
        state.next_id += 1;

        let tile = AtlasTile {
            texture_id: AtlasTextureId {
                index: 0,
                kind: crate::AtlasTextureKind::Polychrome,
            },
            tile_id: TileId(id),
            padding: 0,
            bounds: Bounds {
                origin: Point::default(),
                size,
            },
        };

        state.tiles.insert(key.clone(), tile.clone());
        Ok(Some(tile))
    }

    fn remove(&self, key: &AtlasKey) {
        self.0.lock().tiles.remove(key);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headless_platform_creation() {
        // This should not panic or require main thread
        let platform = HeadlessMetalPlatform::new(42);
        assert!(!platform.displays().is_empty());
    }

    #[test]
    fn test_headless_clipboard() {
        let platform = HeadlessMetalPlatform::new(42);

        // Write to clipboard
        let item = ClipboardItem::new_string("test".to_string());
        platform.write_to_clipboard(item);

        // Read from clipboard
        let read = platform.read_from_clipboard();
        assert!(read.is_some());
    }

    #[test]
    fn test_headless_app_context_creation() {
        // This should not panic or require main thread
        let _cx = HeadlessMetalAppContext::new();
    }
}
