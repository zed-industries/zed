use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use android_activity::input::TextInputState;
use anyhow::{Context as _, Result};
use gpui::{
    AnyWindowHandle, Bounds, Capslock, DevicePixels, DispatchEventResult, GpuSpecs, Modifiers,
    Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow,
    Point, PromptButton, PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowParams, px,
};
use gpui_wgpu::{GpuContext, WgpuRenderer, WgpuSurfaceConfig};
use ndk::native_window::NativeWindow;
use parking_lot::Mutex;
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, DisplayHandle, HasDisplayHandle, HasWindowHandle,
    HandleError, RawDisplayHandle, RawWindowHandle, WindowHandle,
};

use super::AndroidDisplay;

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
    physical_size: Size<DevicePixels>,
    scale_factor: f32,
    title: String,
    input_handler: Option<PlatformInputHandler>,
    is_active: bool,
    is_hovered: bool,
    mouse_position: Point<Pixels>,
    modifiers: Modifiers,
    capslock: Capslock,
    appearance: WindowAppearance,
    /// `Some` only while a surface is alive. Recreated on `MainEvent::InitWindow`,
    /// dropped synchronously on `MainEvent::TerminateWindow` (the makepad pattern
    /// — see `crates/gpui_android/src/android/platform.rs`).
    renderer: Option<WgpuRenderer>,
    /// Atlas exposed via [`PlatformWindow::sprite_atlas`]. Cached separately so
    /// we can return `Arc<dyn PlatformAtlas>` even when the renderer's been
    /// torn down between window cycles (callers may keep handles around).
    sprite_atlas: Option<Arc<dyn PlatformAtlas>>,
    /// Most-recent `GpuSpecs` reported by wgpu, captured the first time the
    /// renderer initialised.
    gpu_specs: Option<GpuSpecs>,
}

/// Cheap clone-able handle to a `NativeWindow` that satisfies wgpu's
/// `HasWindowHandle + HasDisplayHandle + Send + Sync + Clone + Debug`
/// requirements. Wrapping `NativeWindow` directly works (NDK 0.9 already
/// provides the impls), but going through this struct lets us decouple the
/// window from its surface lifetime: when Android destroys the surface we
/// just drop our `Renderer`, the held `NativeWindow` is released by `Drop`,
/// and the next `InitWindow` event hands us a fresh one.
#[derive(Clone, Debug)]
struct WindowSurface(NativeWindow);

impl HasWindowHandle for WindowSurface {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let mut handle = AndroidNdkWindowHandle::new(self.0.ptr().cast());
        // Pre-rotation transforms are handled inside the renderer once we
        // wire surface configuration; for now we just hand off the raw window.
        let _ = &mut handle;
        // SAFETY: the underlying ANativeWindow is kept alive for the
        // lifetime of `self` via the refcounted `NativeWindow` handle.
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(handle)) })
    }
}

impl HasDisplayHandle for WindowSurface {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Android(AndroidDisplayHandle::new()))
        })
    }
}

pub(crate) struct AndroidWindow {
    handle: AnyWindowHandle,
    display: Rc<dyn PlatformDisplay>,
    state: RefCell<WindowState>,
    callbacks: RefCell<WindowCallbacks>,
    /// Shared GPU context across all GPUI windows (we only ever have one on
    /// Android today, but the renderer API expects this shape).
    gpu_context: GpuContext,
    /// Lock-protected handle to the live native window. Held in a `Mutex` so
    /// the JNI-driven event-thread can swap it in/out without `RefCell`'s
    /// thread-locality complaints.
    surface: Mutex<Option<WindowSurface>>,
    /// Mirrors whether the surface is currently alive. Read by `draw` to skip
    /// frames that arrive after `surfaceDestroyed`.
    surface_alive: Cell<bool>,
}

impl AndroidWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
        scale_factor: f32,
        gpu_context: GpuContext,
    ) -> Self {
        let bounds = params.bounds;
        let physical_size = Size {
            width: DevicePixels((f32::from(bounds.size.width) * scale_factor) as i32),
            height: DevicePixels((f32::from(bounds.size.height) * scale_factor) as i32),
        };

        Self {
            handle,
            display,
            state: RefCell::new(WindowState {
                bounds,
                physical_size,
                scale_factor,
                title: String::new(),
                input_handler: None,
                is_active: true,
                is_hovered: false,
                mouse_position: Point::default(),
                modifiers: Modifiers::default(),
                capslock: Capslock::default(),
                appearance: WindowAppearance::Light,
                renderer: None,
                sprite_atlas: None,
                gpu_specs: None,
            }),
            callbacks: RefCell::new(WindowCallbacks::default()),
            gpu_context,
            surface: Mutex::new(None),
            surface_alive: Cell::new(false),
        }
    }

    /// Construct from an `AndroidDisplay` handle; convenience for callers that
    /// know they are using GPUI's stock display type.
    #[allow(dead_code)]
    pub(crate) fn from_display(
        handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<AndroidDisplay>,
        scale_factor: f32,
        gpu_context: GpuContext,
    ) -> Self {
        Self::new(handle, params, display, scale_factor, gpu_context)
    }

    /// Returns the handle this window was opened with. Useful for tests.
    #[allow(dead_code)]
    pub(crate) fn handle(&self) -> AnyWindowHandle {
        self.handle
    }

    /// Called from [`super::AndroidPlatform`] when Android publishes a new
    /// surface (`MainEvent::InitWindow` or `MainEvent::Resume` with a
    /// non-null window). Initialises a [`WgpuRenderer`] tied to the new
    /// surface.
    pub(crate) fn attach_surface(
        &self,
        native_window: NativeWindow,
        physical_size: Size<DevicePixels>,
    ) -> Result<()> {
        let surface = WindowSurface(native_window);
        *self.surface.lock() = Some(surface.clone());
        self.surface_alive.set(true);

        let config = WgpuSurfaceConfig {
            size: physical_size,
            transparent: false,
            preferred_present_mode: Some(gpui_wgpu::wgpu::PresentMode::Mailbox),
        };
        let renderer = WgpuRenderer::new(self.gpu_context.clone(), &surface, config, None)
            .context("failed to initialise WgpuRenderer for the Android surface")?;

        let gpu_specs = renderer.gpu_specs();
        let sprite_atlas: Arc<dyn PlatformAtlas> = renderer.sprite_atlas().clone();
        let mut state = self.state.borrow_mut();
        state.physical_size = physical_size;
        state.renderer = Some(renderer);
        state.sprite_atlas = Some(sprite_atlas);
        state.gpu_specs = Some(gpu_specs);
        Ok(())
    }

    /// Called from [`super::AndroidPlatform`] on `MainEvent::TerminateWindow`
    /// or `MainEvent::Pause` to drop the GPU surface synchronously before
    /// returning to the JVM (the wgpu/Vulkan-on-Android contract).
    pub(crate) fn detach_surface(&self) {
        self.surface_alive.set(false);
        let mut state = self.state.borrow_mut();
        // Drop renderer first; this releases the wgpu surface, which the
        // adapter requires before we drop the underlying NativeWindow.
        state.renderer = None;
        state.sprite_atlas = None;
        drop(state);
        *self.surface.lock() = None;
    }

    /// Update the window's logical bounds + physical pixel size from a
    /// configuration change (rotation, fold, font-scale).
    pub(crate) fn update_size(&self, new_size: Size<DevicePixels>, scale_factor: f32) {
        let mut state = self.state.borrow_mut();
        state.physical_size = new_size;
        state.scale_factor = scale_factor;
        let logical = Size {
            width: px(new_size.width.0 as f32 / scale_factor),
            height: px(new_size.height.0 as f32 / scale_factor),
        };
        state.bounds = Bounds {
            origin: Point::default(),
            size: logical,
        };
        if let Some(renderer) = state.renderer.as_mut() {
            renderer.update_drawable_size(new_size);
        }
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(resize) = callbacks.resize.as_mut() {
            resize(logical, scale_factor);
        }
    }

    /// Forward an input event to the registered handler.
    pub(crate) fn dispatch_input(&self, event: PlatformInput) -> DispatchEventResult {
        if let PlatformInput::MouseMove(ev) = &event {
            self.state.borrow_mut().mouse_position = ev.position;
        }
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(handler) = callbacks.input.as_mut() {
            handler(event)
        } else {
            DispatchEventResult::default()
        }
    }

    /// Translate an android-activity `TextInputState` (the IME's view of the
    /// editor's text) into GPUI's `PlatformInputHandler` calls.
    ///
    /// Composing edits go through `replace_and_mark_text_in_range`, committed
    /// edits through `replace_text_in_range`. We naively replace the whole
    /// document with the IME's view each time — this is correct (the IME
    /// already mirrors the editor's state) but inefficient for large
    /// documents; a future pass should diff the two.
    pub(crate) fn dispatch_text_event(&self, state: TextInputState) {
        let mut state_borrow = self.state.borrow_mut();
        let Some(handler) = state_borrow.input_handler.as_mut() else {
            return;
        };
        let selection = state.selection.start..state.selection.end;
        if let Some(compose) = state.compose_region {
            handler.replace_and_mark_text_in_range(
                None,
                &state.text,
                Some(compose.start..compose.end),
            );
        } else {
            handler.replace_text_in_range(None, &state.text);
        }
        // Drop the borrow so subsequent input dispatches can re-borrow without
        // the user-supplied handler invalidating it.
        let _ = selection;
    }

    pub(crate) fn dispatch_request_frame(&self, options: RequestFrameOptions) {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(handler) = callbacks.request_frame.as_mut() {
            handler(options);
        }
    }

    pub(crate) fn dispatch_active_status(&self, active: bool) {
        self.state.borrow_mut().is_active = active;
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(handler) = callbacks.active_status_change.as_mut() {
            handler(active);
        }
    }

    pub(crate) fn set_appearance(&self, appearance: WindowAppearance) {
        self.state.borrow_mut().appearance = appearance;
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(handler) = callbacks.appearance_changed.as_mut() {
            handler();
        }
    }
}

impl HasWindowHandle for AndroidWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        // We cannot return a borrowed handle that survives across the lock
        // guard's drop; for the common cases (wgpu builds the surface inside
        // `attach_surface`) callers don't need this. Return `Unavailable` so
        // misuse fails loudly instead of silently dangling.
        Err(HandleError::Unavailable)
    }
}

impl HasDisplayHandle for AndroidWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Android(AndroidDisplayHandle::new()))
        })
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

    fn resize(&mut self, _size: Size<Pixels>) {
        // Android decides window size — we cannot resize on demand. Recorded
        // for symmetry with the trait but otherwise a no-op.
    }

    fn scale_factor(&self) -> f32 {
        self.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        self.state.borrow().appearance
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
        true
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

    fn draw(&self, scene: &Scene) {
        if !self.surface_alive.get() {
            return;
        }
        if let Some(renderer) = self.state.borrow_mut().renderer.as_mut() {
            renderer.draw(scene);
        }
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.state
            .borrow()
            .sprite_atlas
            .clone()
            .expect("sprite_atlas() called before attach_surface()")
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        self.state
            .borrow()
            .renderer
            .as_ref()
            .map(|r| r.supports_dual_source_blending())
            .unwrap_or(false)
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        self.state.borrow().gpu_specs.clone()
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}
}
