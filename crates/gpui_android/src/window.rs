use crate::events::ClickState;
use android_activity::AndroidApp;
use anyhow::Context as _;
use gpui::{
    AnyWindowHandle, Bounds, Capslock, Decorations, DevicePixels, DispatchEventResult, GpuSpecs,
    Modifiers, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions,
    ResizeEdge, Scene, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    WindowControlArea, WindowControls, WindowDecorations, WindowParams, px,
};
use gpui_wgpu::{GpuContext, WgpuRenderer, WgpuSurfaceConfig};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

/// Wraps the current `ANativeWindow` so `WgpuRenderer` can create a surface
/// from it via raw-window-handle.
#[derive(Clone, Debug)]
pub(crate) struct RawWindow {
    native_window: android_activity::ndk::native_window::NativeWindow,
}

impl RawWindow {
    fn physical_size(&self) -> Size<DevicePixels> {
        Size {
            width: DevicePixels(self.native_window.width()),
            height: DevicePixels(self.native_window.height()),
        }
    }
}

impl raw_window_handle::HasWindowHandle for RawWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let ptr = std::ptr::NonNull::new(self.native_window.ptr().as_ptr().cast::<std::ffi::c_void>())
            .ok_or(raw_window_handle::HandleError::Unavailable)?;
        let handle = raw_window_handle::AndroidNdkWindowHandle::new(ptr);
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
    }
}

impl raw_window_handle::HasDisplayHandle for RawWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(raw_window_handle::DisplayHandle::android())
    }
}

#[derive(Default)]
pub(crate) struct AndroidWindowCallbacks {
    pub(crate) request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    pub(crate) input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    pub(crate) active_status_change: Option<Box<dyn FnMut(bool)>>,
    pub(crate) hover_status_change: Option<Box<dyn FnMut(bool)>>,
    pub(crate) resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    pub(crate) moved: Option<Box<dyn FnMut()>>,
    pub(crate) should_close: Option<Box<dyn FnMut() -> bool>>,
    pub(crate) close: Option<Box<dyn FnOnce()>>,
    pub(crate) appearance_changed: Option<Box<dyn FnMut()>>,
    pub(crate) hit_test_window_control: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
}

pub(crate) struct AndroidWindowState {
    pub(crate) renderer: WgpuRenderer,
    pub(crate) raw_window: RawWindow,
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) scale_factor: f32,
    pub(crate) title: String,
    pub(crate) input_handler: Option<PlatformInputHandler>,
    pub(crate) is_active: bool,
    pub(crate) mouse_position: Point<Pixels>,
    pub(crate) modifiers: Modifiers,
    pub(crate) capslock: Capslock,
}

pub(crate) struct AndroidWindowInner {
    pub(crate) app: AndroidApp,
    pub(crate) gpu_context: GpuContext,
    pub(crate) state: RefCell<AndroidWindowState>,
    pub(crate) callbacks: RefCell<AndroidWindowCallbacks>,
    pub(crate) click_state: RefCell<ClickState>,
    pub(crate) surface_configured: Cell<bool>,
    pub(crate) appearance: Cell<WindowAppearance>,
    pending_physical_size: Cell<Option<Size<DevicePixels>>>,
}

pub struct AndroidWindow {
    pub(crate) inner: Rc<AndroidWindowInner>,
    display: Rc<dyn PlatformDisplay>,
    #[allow(dead_code)]
    handle: AnyWindowHandle,
}

fn surface_config(size: Size<DevicePixels>) -> WgpuSurfaceConfig {
    WgpuSurfaceConfig {
        size,
        transparent: false,
        // Mailbox avoids blocking in get_current_texture() during Android
        // lifecycle transitions; the renderer falls back to Fifo if unsupported.
        preferred_present_mode: Some(wgpu::PresentMode::Mailbox),
    }
}

pub(crate) fn scale_factor(app: &AndroidApp) -> f32 {
    app.config().density().map_or(2.0, |dpi| dpi as f32 / 160.0)
}

impl AndroidWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        _params: WindowParams,
        app: AndroidApp,
        gpu_context: GpuContext,
        display: Rc<dyn PlatformDisplay>,
        appearance: WindowAppearance,
    ) -> anyhow::Result<Self> {
        let native_window = app
            .native_window()
            .context("no native window: open_window must be called after the first InitWindow")?;
        let raw_window = RawWindow { native_window };
        let physical_size = raw_window.physical_size();
        let scale = scale_factor(&app);

        let renderer = WgpuRenderer::new(
            gpu_context.clone(),
            &raw_window,
            surface_config(physical_size),
            None,
        )?;

        let bounds = Bounds {
            origin: Point::default(),
            size: logical_size(physical_size, scale),
        };

        let state = AndroidWindowState {
            renderer,
            raw_window,
            bounds,
            scale_factor: scale,
            title: String::new(),
            input_handler: None,
            is_active: true,
            mouse_position: Point::default(),
            modifiers: Modifiers::default(),
            capslock: Capslock::default(),
        };

        let inner = Rc::new(AndroidWindowInner {
            app,
            gpu_context,
            state: RefCell::new(state),
            callbacks: RefCell::new(AndroidWindowCallbacks::default()),
            click_state: RefCell::new(ClickState::default()),
            surface_configured: Cell::new(true),
            appearance: Cell::new(appearance),
            pending_physical_size: Cell::new(None),
        });

        Ok(Self {
            inner,
            display,
            handle,
        })
    }
}

fn logical_size(physical: Size<DevicePixels>, scale: f32) -> Size<Pixels> {
    Size {
        width: px(physical.width.0 as f32 / scale),
        height: px(physical.height.0 as f32 / scale),
    }
}

impl AndroidWindowInner {
    /// Called on `MainEvent::InitWindow` after the native window was destroyed
    /// and recreated (backgrounding, rotation). Recreates the wgpu surface on
    /// the same device so cached atlas textures stay valid.
    pub(crate) fn handle_surface_created(&self) {
        let Some(native_window) = self.app.native_window() else {
            log::error!("InitWindow received but native_window() returned None");
            return;
        };
        let raw_window = RawWindow { native_window };
        let physical_size = raw_window.physical_size();
        let Some(instance) = self
            .gpu_context
            .borrow()
            .as_ref()
            .map(|context| context.instance.clone())
        else {
            log::error!("surface recreation requested before the GPU context exists");
            return;
        };

        {
            let mut state = self.state.borrow_mut();
            if let Err(error) =
                state
                    .renderer
                    .replace_surface(&raw_window, surface_config(physical_size), &instance)
            {
                log::error!("failed to replace wgpu surface: {error:#}");
                return;
            }
            state.raw_window = raw_window;
        }
        self.surface_configured.set(true);
        self.update_size();
    }

    /// Called on `MainEvent::TerminateWindow`: the `ANativeWindow` is about to
    /// be destroyed, so rendering must stop until a new surface arrives.
    pub(crate) fn handle_surface_destroyed(&self) {
        self.surface_configured.set(false);
        self.state.borrow_mut().renderer.unconfigure_surface();
    }

    pub(crate) fn update_size(&self) {
        let scale = scale_factor(&self.app);
        let (physical_size, changed) = {
            let mut state = self.state.borrow_mut();
            let physical_size = state.raw_window.physical_size();
            let logical = logical_size(physical_size, scale);
            let changed = state.bounds.size != logical || state.scale_factor != scale;
            state.bounds.size = logical;
            state.scale_factor = scale;
            (physical_size, changed)
        };

        if !changed {
            return;
        }
        self.pending_physical_size.set(Some(physical_size));

        let logical = logical_size(physical_size, scale);
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut callback) = callbacks.resize {
            callback(logical, scale);
        }
    }

    pub(crate) fn request_frame(&self, force_render: bool) {
        if !self.surface_configured.get() {
            return;
        }
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut callback) = callbacks.request_frame {
            callback(RequestFrameOptions {
                require_presentation: true,
                force_render,
            });
        }
    }

    pub(crate) fn set_active(&self, is_active: bool) {
        self.state.borrow_mut().is_active = is_active;
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut callback) = callbacks.active_status_change {
            callback(is_active);
        }
    }

    pub(crate) fn set_appearance(&self, appearance: WindowAppearance) {
        if self.appearance.replace(appearance) == appearance {
            return;
        }
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut callback) = callbacks.appearance_changed {
            callback();
        }
    }

    pub(crate) fn dispatch_input(&self, input: PlatformInput) -> Option<DispatchEventResult> {
        let mut callbacks = self.callbacks.borrow_mut();
        let callback = callbacks.input.as_mut()?;
        Some(callback(input))
    }

    pub(crate) fn with_input_handler<R>(
        &self,
        f: impl FnOnce(&mut PlatformInputHandler) -> R,
    ) -> Option<R> {
        let mut handler = self.state.borrow_mut().input_handler.take()?;
        let result = f(&mut handler);
        self.state.borrow_mut().input_handler = Some(handler);
        Some(result)
    }
}

impl raw_window_handle::HasWindowHandle for AndroidWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw_window = self.inner.state.borrow().raw_window.clone();
        let handle = raw_window.window_handle()?.as_raw();
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle) })
    }
}

impl raw_window_handle::HasDisplayHandle for AndroidWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(raw_window_handle::DisplayHandle::android())
    }
}

impl PlatformWindow for AndroidWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.inner.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        true
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.inner.state.borrow().bounds.size
    }

    fn resize(&mut self, _size: Size<Pixels>) {}

    fn scale_factor(&self) -> f32 {
        self.inner.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        self.inner.appearance.get()
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.inner.state.borrow().mouse_position
    }

    fn modifiers(&self) -> Modifiers {
        self.inner.state.borrow().modifiers
    }

    fn capslock(&self) -> Capslock {
        self.inner.state.borrow().capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.inner.state.borrow_mut().input_handler = Some(input_handler);
        self.inner.app.show_soft_input(true);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.inner.state.borrow_mut().input_handler.take()
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
        self.inner.state.borrow_mut().is_active = true;
    }

    fn is_active(&self) -> bool {
        self.inner.state.borrow().is_active
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.inner.state.borrow_mut().title = title.to_owned();
    }

    fn set_background_appearance(&self, _background: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        true
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.inner.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.inner.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.inner.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.inner.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.inner.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        if !self.inner.surface_configured.get() {
            return;
        }
        let mut state = self.inner.state.borrow_mut();
        if let Some(physical_size) = self.inner.pending_physical_size.take() {
            state.renderer.update_drawable_size(physical_size);
        }
        state.renderer.draw(scene);
    }

    fn completed_frame(&self) {}

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.inner.state.borrow().renderer.sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        self.inner
            .state
            .borrow()
            .renderer
            .supports_dual_source_blending()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        Some(self.inner.state.borrow().renderer.gpu_specs())
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}

    fn request_decorations(&self, _decorations: WindowDecorations) {}

    fn show_window_menu(&self, _position: Point<Pixels>) {}

    fn start_window_move(&self) {}

    fn start_window_resize(&self, _edge: ResizeEdge) {}

    fn window_decorations(&self) -> Decorations {
        Decorations::Server
    }

    fn set_app_id(&mut self, _app_id: &str) {}

    fn window_controls(&self) -> WindowControls {
        WindowControls {
            fullscreen: false,
            maximize: false,
            minimize: false,
            window_menu: false,
        }
    }

    fn set_client_inset(&self, _inset: Pixels) {}
}
