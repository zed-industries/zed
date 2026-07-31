use crate::{display::OpenHarmonyDisplay, events::SurfaceInfo};
use anyhow::{Context as _, anyhow};
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, Bounds, DispatchEventResult, Pixels, PlatformInput, PlatformInputHandler,
    PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions, Scene, Size,
    WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowInsets,
    WindowParams, GpuSpecs, point, px,
};
use gpui_wgpu::{CompositorGpuHint, GpuContext, WgpuRenderer, WgpuSurfaceConfig, wgpu};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, OhosDisplayHandle,
    OhosNdkWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle,
};
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    rc::Rc,
    sync::Arc,
};

pub(crate) struct OpenHarmonyWindow {
    handle: AnyWindowHandle,
    native_window: NonNull<c_void>,
    display: Rc<OpenHarmonyDisplay>,
    bounds: RefCell<Bounds<Pixels>>,
    scale_factor: Cell<f32>,
    is_active: Cell<bool>,
    is_hovered: Cell<bool>,
    mouse_position: Cell<Point<Pixels>>,
    modifiers: Cell<gpui::Modifiers>,
    capslock: Cell<gpui::Capslock>,
    appearance: Cell<WindowAppearance>,
    background_appearance: Cell<WindowBackgroundAppearance>,
    renderer: RefCell<WgpuRenderer>,
    callbacks: RefCell<WindowCallbacks>,
    input_handler: RefCell<Option<PlatformInputHandler>>,
}

pub(crate) struct OpenHarmonyWindowHandle(pub(crate) Rc<OpenHarmonyWindow>);

impl Deref for OpenHarmonyWindowHandle {
    type Target = OpenHarmonyWindow;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HasWindowHandle for OpenHarmonyWindowHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.0.window_handle()
    }
}

impl HasDisplayHandle for OpenHarmonyWindowHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.0.display_handle()
    }
}

#[derive(Default)]
struct WindowCallbacks {
    request_frame: Option<Box<dyn FnMut(RequestFrameOptions) + 'static>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult + 'static>>,
    active: Option<Box<dyn FnMut(bool) + 'static>>,
    hover: Option<Box<dyn FnMut(bool) + 'static>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32) + 'static>>,
    moved: Option<Box<dyn FnMut() + 'static>>,
    should_close: Option<Box<dyn FnMut() -> bool + 'static>>,
    close: Option<Box<dyn FnOnce() + 'static>>,
    appearance_changed: Option<Box<dyn FnMut() + 'static>>,
    hit_test: Option<Box<dyn FnMut() -> Option<WindowControlArea> + 'static>>,
    insets_changed: Option<Box<dyn FnMut(WindowInsets) + 'static>>,
}

impl OpenHarmonyWindow {
    pub fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        surface_info: &SurfaceInfo,
        display: Rc<OpenHarmonyDisplay>,
        gpu_context: GpuContext,
    ) -> anyhow::Result<Self> {
        let scale_factor = surface_info.scale_factor;
        let bounds = params.bounds.map(|v| v / scale_factor);
        let device_size = bounds.size.to_device_pixels(scale_factor);

        let native_window = surface_info
            .native_window
            .as_nonnull()
            .ok_or_else(|| anyhow!("invalid native window"))?;

        let raw_window = OpenHarmonyRawWindow {
            native_window,
            _marker: PhantomData,
        };

        let renderer = WgpuRenderer::new(
            gpu_context,
            &raw_window,
            WgpuSurfaceConfig {
                size: device_size,
                transparent: false,
                preferred_present_mode: Some(wgpu::PresentMode::Fifo),
            },
            None::<CompositorGpuHint>,
        )
        .context("failed to create wgpu renderer")?;

        Ok(Self {
            handle,
            native_window,
            display,
            bounds: RefCell::new(bounds),
            scale_factor: Cell::new(scale_factor),
            is_active: Cell::new(false),
            is_hovered: Cell::new(false),
            mouse_position: Cell::new(point(px(0.), px(0.))),
            modifiers: Cell::new(gpui::Modifiers::default()),
            capslock: Cell::new(gpui::Capslock::default()),
            appearance: Cell::new(WindowAppearance::Light),
            background_appearance: Cell::new(WindowBackgroundAppearance::default()),
            renderer: RefCell::new(renderer),
            callbacks: RefCell::new(WindowCallbacks::default()),
            input_handler: RefCell::new(None),
        })
    }

    pub fn handle(&self) -> AnyWindowHandle {
        self.handle
    }

    pub fn native_window(&self) -> NonNull<c_void> {
        self.native_window
    }

    pub fn set_size(&self, size: Size<Pixels>) {
        self.bounds.borrow_mut().size = size;
        let device_size = size.to_device_pixels(self.scale_factor.get());
        self.renderer.borrow_mut().update_drawable_size(device_size);
        if let Some(callback) = self.callbacks.borrow_mut().resize.as_mut() {
            callback(size, self.scale_factor.get());
        }
        if let Some(callback) = self.callbacks.borrow_mut().moved.as_mut() {
            callback();
        }
    }

    pub fn set_scale_factor(&self, scale_factor: f32) {
        self.scale_factor.set(scale_factor);
        self.set_size(self.bounds.borrow().size);
    }

    pub fn set_active(&self, active: bool) {
        self.is_active.set(active);
        if let Some(callback) = self.callbacks.borrow_mut().active.as_mut() {
            callback(active);
        }
    }

    pub fn set_hovered(&self, hovered: bool) {
        self.is_hovered.set(hovered);
        if let Some(callback) = self.callbacks.borrow_mut().hover.as_mut() {
            callback(hovered);
        }
    }

    pub fn set_mouse_position(&self, position: Point<Pixels>) {
        self.mouse_position.set(position);
    }

    pub fn dispatch_input(&self, input: PlatformInput) -> DispatchEventResult {
        if let Some(handler) = self.callbacks.borrow_mut().input.as_mut() {
            handler(input)
        } else {
            DispatchEventResult::default()
        }
    }

    pub fn request_frame(&self) {
        if let Some(callback) = self.callbacks.borrow_mut().request_frame.as_mut() {
            callback(RequestFrameOptions {
                require_presentation: true,
                force_render: false,
            });
        }
    }

    pub fn on_surface_destroyed(&self) {
        self.renderer.borrow_mut().destroy();
    }

    pub fn set_appearance(&self, appearance: WindowAppearance) {
        self.appearance.set(appearance);
        if let Some(callback) = self.callbacks.borrow_mut().appearance_changed.as_mut() {
            callback();
        }
    }
}

impl PlatformWindow for OpenHarmonyWindowHandle {
    fn bounds(&self) -> Bounds<Pixels> {
        *self.0.bounds.borrow()
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds().size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.0.set_size(size);
    }

    fn scale_factor(&self) -> f32 {
        self.0.scale_factor.get()
    }

    fn appearance(&self) -> WindowAppearance {
        self.0.appearance.get()
    }

    fn display(&self) -> Option<Rc<dyn gpui::PlatformDisplay>> {
        Some(self.0.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.0.mouse_position.get()
    }

    fn modifiers(&self) -> gpui::Modifiers {
        self.0.modifiers.get()
    }

    fn capslock(&self) -> gpui::Capslock {
        self.0.capslock.get()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.input_handler.replace(Some(input_handler));
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.input_handler.take()
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

    fn activate(&self) {
        self.0.set_active(true);
    }

    fn is_active(&self) -> bool {
        self.0.is_active.get()
    }

    fn is_hovered(&self) -> bool {
        self.0.is_hovered.get()
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        self.0.background_appearance.get()
    }

    fn set_title(&mut self, _title: &str) {}

    fn set_background_appearance(&self, background_appearance: WindowBackgroundAppearance) {
        self.0.background_appearance.set(background_appearance);
    }

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        false
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions) + 'static>) {
        self.0.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult + 'static>) {
        self.0.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool) + 'static>) {
        self.0.callbacks.borrow_mut().active = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool) + 'static>) {
        self.0.callbacks.borrow_mut().hover = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32) + 'static>) {
        self.0.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut() + 'static>) {
        self.0.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool + 'static>) {
        self.0.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_hit_test_window_control(
        &self,
        callback: Box<dyn FnMut() -> Option<WindowControlArea> + 'static>,
    ) {
        self.0.callbacks.borrow_mut().hit_test = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce() + 'static>) {
        self.0.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut() + 'static>) {
        self.0.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn on_insets_changed(&self, callback: Box<dyn FnMut(WindowInsets) + 'static>) {
        self.0.callbacks.borrow_mut().insets_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        self.0.renderer.borrow_mut().draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn gpui::PlatformAtlas> {
        self.0.renderer.borrow().sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        true
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn insets(&self) -> WindowInsets {
        WindowInsets::default()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        Some(self.0.renderer.borrow().gpu_specs())
    }
}

impl HasWindowHandle for OpenHarmonyWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = OhosNdkWindowHandle::new(self.native_window);
        let raw = RawWindowHandle::OhosNdk(handle);
        Ok(unsafe { WindowHandle::borrow_raw(raw) })
    }
}

impl HasDisplayHandle for OpenHarmonyWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = OhosDisplayHandle::new();
        let raw = RawDisplayHandle::Ohos(handle);
        Ok(unsafe { DisplayHandle::borrow_raw(raw) })
    }
}

#[derive(Clone, Debug)]
struct OpenHarmonyRawWindow {
    native_window: NonNull<c_void>,
    _marker: PhantomData<*mut c_void>,
}

unsafe impl Send for OpenHarmonyRawWindow {}
unsafe impl Sync for OpenHarmonyRawWindow {}

impl HasWindowHandle for OpenHarmonyRawWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = OhosNdkWindowHandle::new(self.native_window);
        let raw = RawWindowHandle::OhosNdk(handle);
        Ok(unsafe { WindowHandle::borrow_raw(raw) })
    }
}

impl HasDisplayHandle for OpenHarmonyRawWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = OhosDisplayHandle::new();
        let raw = RawDisplayHandle::Ohos(handle);
        Ok(unsafe { DisplayHandle::borrow_raw(raw) })
    }
}
