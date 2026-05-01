use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use anyhow::Result;
use gpui::{
    AnyWindowHandle, AtlasKey, AtlasTile, Bounds, Capslock, DevicePixels, DispatchEventResult,
    GpuSpecs, Modifiers, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions,
    Scene, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowParams,
};

use super::AndroidDisplay;

/// Stub atlas used by [`AndroidWindow`] until the real wgpu surface lifecycle
/// is wired through JNI. The Android window cannot be opened yet (see
/// [`super::platform::AndroidPlatform::open_window`]); this exists only so the
/// trait impl below compiles and the surface area is locked in.
pub(crate) struct NoopAtlas;

impl PlatformAtlas for NoopAtlas {
    fn get_or_insert_with<'a>(
        &self,
        _key: &AtlasKey,
        _build: &mut dyn FnMut()
            -> anyhow::Result<Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>>,
    ) -> anyhow::Result<Option<AtlasTile>> {
        anyhow::bail!("AndroidWindow atlas is not initialized; rendering is not yet wired up")
    }

    fn remove(&self, _key: &AtlasKey) {}
}

#[derive(Default)]
struct WindowCallbacks {
    request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    hover_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
    hit_test_window_control: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
}

struct WindowState {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    title: String,
    input_handler: Option<PlatformInputHandler>,
    is_active: bool,
    is_hovered: bool,
    is_fullscreen: bool,
    mouse_position: Point<Pixels>,
    modifiers: Modifiers,
    capslock: Capslock,
}

pub(crate) struct AndroidWindow {
    handle: AnyWindowHandle,
    display: Rc<dyn PlatformDisplay>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    state: RefCell<WindowState>,
    callbacks: RefCell<WindowCallbacks>,
    is_subpixel: Cell<bool>,
}

impl AndroidWindow {
    /// Construct an `AndroidWindow` backed by the supplied display. The native
    /// surface and wgpu renderer will be plumbed through here once the JNI
    /// bridge is implemented.
    #[allow(dead_code)]
    pub(crate) fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
    ) -> Self {
        let bounds = params.bounds;
        Self {
            handle,
            display,
            sprite_atlas: Arc::new(NoopAtlas),
            state: RefCell::new(WindowState {
                bounds,
                scale_factor: 1.0,
                title: String::new(),
                input_handler: None,
                is_active: true,
                is_hovered: false,
                is_fullscreen: true, // mobile windows are effectively always fullscreen
                mouse_position: Point::default(),
                modifiers: Modifiers::default(),
                capslock: Capslock::default(),
            }),
            callbacks: RefCell::new(WindowCallbacks::default()),
            is_subpixel: Cell::new(false),
        }
    }

    /// Construct from an `AndroidDisplay`, the only display type we provide.
    #[allow(dead_code)]
    pub(crate) fn from_display(
        handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<AndroidDisplay>,
    ) -> Self {
        Self::new(handle, params, display)
    }
}

impl raw_window_handle::HasWindowHandle for AndroidWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::Unavailable)
    }
}

impl raw_window_handle::HasDisplayHandle for AndroidWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Err(raw_window_handle::HandleError::Unavailable)
    }
}

impl PlatformWindow for AndroidWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        true
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.state.borrow().bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.state.borrow_mut().bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        self.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.state.borrow().mouse_position
    }

    fn modifiers(&self) -> Modifiers {
        self.state.borrow().modifiers
    }

    fn capslock(&self) -> Capslock {
        self.state.borrow().capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.state.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.state.borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {
        self.state.borrow_mut().is_active = true;
    }

    fn is_active(&self) -> bool {
        self.state.borrow().is_active
    }

    fn is_hovered(&self) -> bool {
        self.state.borrow().is_hovered
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.state.borrow_mut().title = title.to_owned();
    }

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        self.state.borrow().is_fullscreen
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn draw(&self, _scene: &Scene) {
        // Rendering is not wired up yet on Android. Once the JNI surface
        // lifecycle is in place we will route the scene to a `WgpuRenderer`.
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.sprite_atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        self.is_subpixel.get()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}
}

impl AndroidWindow {
    /// Returns the handle this window was opened with. Useful for tests.
    #[allow(dead_code)]
    pub(crate) fn handle(&self) -> AnyWindowHandle {
        self.handle
    }
}

