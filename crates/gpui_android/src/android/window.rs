use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use android_activity::input::{ImeOptions, InputType, TextInputAction, TextInputState};
use anyhow::{Context as _, Result};
use gpui::{
    AnyWindowHandle, AtlasKey, AtlasTile, Bounds, Capslock, DevicePixels, DispatchEventResult,
    GpuSpecs, Modifiers, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions,
    Scene, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowParams, px,
};

/// Stand-in atlas returned by [`AndroidWindow::sprite_atlas`] when no surface
/// is alive — i.e. before `attach_surface`. GPUI calls `sprite_atlas` very
/// early in window setup (synchronously, on the foreground thread, before
/// the activity loop has had a chance to deliver `MainEvent::InitWindow`),
/// so we MUST hand back something. Real glyph uploads will only succeed once
/// the wgpu surface is up and the renderer's atlas takes over.
struct NoopAtlas;

impl PlatformAtlas for NoopAtlas {
    fn get_or_insert_with<'a>(
        &self,
        _key: &AtlasKey,
        _build: &mut dyn FnMut() -> anyhow::Result<
            Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>,
        >,
    ) -> anyhow::Result<Option<AtlasTile>> {
        Ok(None)
    }

    fn remove(&self, _key: &AtlasKey) {}
}
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
    /// Last `TextInputState` we *received* from the IME. Used to suppress
    /// echo: when we apply an edit and Android re-broadcasts the resulting
    /// state, the broadcast matches `last_ime_state` and we drop it.
    last_ime_state: Option<TextInputStateSnapshot>,
}

#[derive(Clone, PartialEq, Eq)]
struct TextInputStateSnapshot {
    text: String,
    selection_start: usize,
    selection_end: usize,
    compose_start: Option<usize>,
    compose_end: Option<usize>,
}

impl From<&TextInputState> for TextInputStateSnapshot {
    fn from(state: &TextInputState) -> Self {
        Self {
            text: state.text.clone(),
            selection_start: state.selection.start,
            selection_end: state.selection.end,
            compose_start: state.compose_region.as_ref().map(|s| s.start),
            compose_end: state.compose_region.as_ref().map(|s| s.end),
        }
    }
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
                // Hand callers a NoopAtlas until `attach_surface` swaps in
                // the real wgpu-backed atlas. This avoids the panic that
                // happens when GPUI's window setup queries the atlas before
                // the activity loop has delivered `MainEvent::InitWindow`.
                sprite_atlas: Some(Arc::new(NoopAtlas) as Arc<dyn PlatformAtlas>),
                gpu_specs: None,
                last_ime_state: None,
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
        // Fall back to a NoopAtlas so any subsequent `sprite_atlas()` call
        // (e.g. during a redraw that arrives between TerminateWindow and
        // process shutdown) hands callers a real `Arc<dyn PlatformAtlas>`
        // instead of `None`/panicking.
        state.sprite_atlas = Some(Arc::new(NoopAtlas) as Arc<dyn PlatformAtlas>);
        drop(state);
        *self.surface.lock() = None;
    }

    /// Notify the window of a new content rect (the area not covered by
    /// system bars or the IME). For now this is plumbed onto the bounds
    /// reported via `content_size()`; once GPUI gains a first-class
    /// "safe area inset" concept it should be exposed there directly.
    pub(crate) fn update_content_rect(
        &self,
        rect: android_activity::Rect,
        scale_factor: f32,
    ) {
        let logical_origin = Point {
            x: px(rect.left as f32 / scale_factor),
            y: px(rect.top as f32 / scale_factor),
        };
        let logical_size = Size {
            width: px(((rect.right - rect.left).max(0)) as f32 / scale_factor),
            height: px(((rect.bottom - rect.top).max(0)) as f32 / scale_factor),
        };
        {
            let mut state = self.state.borrow_mut();
            state.bounds = Bounds {
                origin: logical_origin,
                size: logical_size,
            };
        }
        // GPUI's resize handler can re-enter the window (e.g. via
        // `scale_factor()`), so the state borrow above must drop before
        // invoking it. Same pattern in every other dispatch_* method.
        invoke_callback(self, |callbacks| {
            if let Some(resize) = callbacks.resize.as_mut() {
                resize(logical_size, scale_factor);
            }
        });
    }

    /// Update the window's logical bounds + physical pixel size from a
    /// configuration change (rotation, fold, font-scale).
    pub(crate) fn update_size(&self, new_size: Size<DevicePixels>, scale_factor: f32) {
        let logical = Size {
            width: px(new_size.width.0 as f32 / scale_factor),
            height: px(new_size.height.0 as f32 / scale_factor),
        };
        {
            let mut state = self.state.borrow_mut();
            state.physical_size = new_size;
            state.scale_factor = scale_factor;
            state.bounds = Bounds {
                origin: Point::default(),
                size: logical,
            };
            if let Some(renderer) = state.renderer.as_mut() {
                renderer.update_drawable_size(new_size);
            }
        }
        invoke_callback(self, |callbacks| {
            if let Some(resize) = callbacks.resize.as_mut() {
                resize(logical, scale_factor);
            }
        });
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
    /// edits through `replace_text_in_range`. The whole-document replace is
    /// correct because the IME already mirrors the editor's state; a future
    /// pass should diff the two for efficiency on large documents.
    ///
    /// **Echo prevention.** The IME re-broadcasts `TextInputState` every
    /// time a side updates it — including after our own `set_text_input_state`
    /// pushes. We snapshot the state we apply and ignore re-deliveries that
    /// match it bit-for-bit, mirroring the makepad pattern.
    pub(crate) fn dispatch_text_event(&self, state: TextInputState) {
        let snapshot = TextInputStateSnapshot::from(&state);
        // Stash the snapshot, then take the input handler out of the cell so
        // the user-supplied PlatformInputHandler can re-enter `state.borrow*`
        // safely (the input handler internally schedules onto the foreground
        // executor, which can call back into our methods).
        let mut handler = {
            let mut state_borrow = self.state.borrow_mut();
            if state_borrow.last_ime_state.as_ref() == Some(&snapshot) {
                return;
            }
            state_borrow.last_ime_state = Some(snapshot);
            state_borrow.input_handler.take()
        };
        let Some(handler_ref) = handler.as_mut() else {
            return;
        };
        if let Some(compose) = state.compose_region {
            handler_ref.replace_and_mark_text_in_range(
                None,
                &state.text,
                Some(compose.start..compose.end),
            );
        } else {
            handler_ref.replace_text_in_range(None, &state.text);
        }
        // Restore the handler.
        self.state.borrow_mut().input_handler = handler;
    }

    pub(crate) fn dispatch_request_frame(&self, options: RequestFrameOptions) {
        invoke_callback(self, |callbacks| {
            if let Some(handler) = callbacks.request_frame.as_mut() {
                handler(options);
            }
        });
    }

    pub(crate) fn dispatch_active_status(&self, active: bool) {
        self.state.borrow_mut().is_active = active;
        invoke_callback(self, |callbacks| {
            if let Some(handler) = callbacks.active_status_change.as_mut() {
                handler(active);
            }
        });
    }

    pub(crate) fn set_appearance(&self, appearance: WindowAppearance) {
        self.state.borrow_mut().appearance = appearance;
        invoke_callback(self, |callbacks| {
            if let Some(handler) = callbacks.appearance_changed.as_mut() {
                handler();
            }
        });
    }
}

/// Invoke a callback on the window's `WindowCallbacks` while making sure no
/// `state` borrow is live. The user-supplied callback may re-enter the
/// window's `PlatformWindow` methods, which all read `self.state`; without
/// this scoping pattern those re-entries would panic with
/// "RefCell already mutably borrowed".
fn invoke_callback(window: &AndroidWindow, f: impl FnOnce(&mut WindowCallbacks)) {
    debug_assert!(window.state.try_borrow().is_ok());
    let mut callbacks = window.callbacks.borrow_mut();
    f(&mut callbacks);
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
        // Register a sane default `EditorInfo` and pop the soft keyboard so
        // the user can actually type. Apps can override this later by calling
        // `set_input_handler` again after re-configuring via JNI.
        if let Some(app) = super::android_app() {
            app.set_ime_editor_info(
                InputType::TYPE_CLASS_TEXT
                    | InputType::TYPE_TEXT_FLAG_MULTI_LINE
                    | InputType::TYPE_TEXT_FLAG_NO_SUGGESTIONS,
                TextInputAction::None,
                ImeOptions::IME_FLAG_NO_FULLSCREEN,
            );
            app.show_soft_input(false);
        }
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        if let Some(app) = super::android_app() {
            app.hide_soft_input(false);
        }
        self.state.borrow_mut().last_ime_state = None;
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
        // `sprite_atlas` is initialised in the `WindowState` constructor with
        // a `NoopAtlas`, so this `unwrap_or_else` arm should never fire.
        // Keep it as a safety net — a panic here would tear down the JVM
        // process during early window bring-up.
        self.state
            .borrow()
            .sprite_atlas
            .clone()
            .unwrap_or_else(|| Arc::new(NoopAtlas) as Arc<dyn PlatformAtlas>)
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

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {
        // The IME's candidate-window position is normally pushed via
        // `View.updateCursorAnchorInfo`, but android-activity's
        // GameActivity has no Rust-visible View handle. Phase 2 (the
        // android-view client) will implement this for real.
    }

    fn play_system_bell(&self) {
        if let Some(app) = super::android_app() {
            super::bell::ring(&app);
        }
    }
}
