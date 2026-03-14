use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Bounds, Capslock, DispatchEventResult, GpuSpecs, Modifiers,
    Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow,
    Point, PromptButton, PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls, WindowParams,
};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, UiKitDisplayHandle,
    UiKitWindowHandle, WindowHandle,
};
use std::{
    cell::RefCell,
    ffi::c_void,
    ptr::NonNull,
    rc::Rc,
    sync::Arc,
};
/// Callbacks registered by GPUI's window layer.
#[derive(Default)]
struct IosWindowCallbacks {
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
#[allow(dead_code)]
struct IosWindowInner {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    input_handler: Option<PlatformInputHandler>,
    /// Raw pointer to the UIView that hosts the CAMetalLayer.
    /// Set to non-null in Phase 1.3 when SceneDelegate calls back into Rust.
    ui_view: Option<NonNull<c_void>>,
    /// Raw pointer to the UIWindow containing the view.
    ui_window: Option<NonNull<c_void>>,
}
/// iOS window backed by a UIView with a CAMetalLayer.
///
/// Phase 1 status: struct compiles and satisfies the `PlatformWindow` trait.
/// The UIKit integration (creating a real UIView, wiring CADisplayLink, etc.)
/// is completed in Phase 1.3 when SceneDelegate FFI is wired up.
pub struct IosWindow {
    inner: RefCell<IosWindowInner>,
    callbacks: RefCell<IosWindowCallbacks>,
    display: Rc<dyn PlatformDisplay>,
}
impl IosWindow {
    pub fn new(_params: WindowParams, display: Rc<dyn PlatformDisplay>) -> Self {
        let bounds = display.bounds();
        Self {
            inner: RefCell::new(IosWindowInner {
                bounds,
                scale_factor: 2.0,
                input_handler: None,
                ui_view: None,
                ui_window: None,
            }),
            callbacks: RefCell::new(IosWindowCallbacks::default()),
            display,
        }
    }
}
impl HasWindowHandle for IosWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let inner = self.inner.borrow();
        // When ui_window is set (Phase 1.3), we return the real UIWindow pointer.
        // Until then, we return an error so callers know the window isn't ready.
        let ui_window = inner
            .ui_window
            .ok_or(HandleError::Unavailable)?;
        let handle = UiKitWindowHandle::new(ui_window);
        Ok(unsafe { WindowHandle::borrow_raw(handle.into()) })
    }
}
impl HasDisplayHandle for IosWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = UiKitDisplayHandle::new();
        Ok(unsafe { DisplayHandle::borrow_raw(handle.into()) })
    }
}
impl PlatformWindow for IosWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.inner.borrow().bounds
    }
    fn is_maximized(&self) -> bool {
        // iOS always fills the scene; no traditional window maximization.
        true
    }
    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.inner.borrow().bounds)
    }
    fn content_size(&self) -> Size<Pixels> {
        self.inner.borrow().bounds.size
    }
    fn resize(&mut self, size: Size<Pixels>) {
        self.inner.borrow_mut().bounds.size = size;
    }
    fn scale_factor(&self) -> f32 {
        self.inner.borrow().scale_factor
    }
    fn appearance(&self) -> WindowAppearance {
        // TODO Phase 1.3: query UITraitCollection.userInterfaceStyle
        WindowAppearance::Light
    }
    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }
    fn mouse_position(&self) -> Point<Pixels> {
        // Pointer position is tracked via UIPointerInteraction in Phase 1.3.
        Point::default()
    }
    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }
    fn capslock(&self) -> Capslock {
        Capslock::default()
    }
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.inner.borrow_mut().input_handler = Some(input_handler);
    }
    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.inner.borrow_mut().input_handler.take()
    }
    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<oneshot::Receiver<usize>> {
        // TODO Phase 2: implement UIAlertController prompt
        None
    }
    fn activate(&self) {}
    fn is_active(&self) -> bool {
        true
    }
    fn is_hovered(&self) -> bool {
        false
    }
    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }
    fn set_title(&mut self, _title: &str) {
        // UIWindowScene title (shown in the App Switcher) — Phase 1.3
    }
    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}
    fn minimize(&self) {}
    fn zoom(&self) {}
    fn toggle_fullscreen(&self) {
        // iOS is always full-scene; no traditional fullscreen toggle.
    }
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
    fn on_hit_test_window_control(
        &self,
        callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
        self.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }
    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.callbacks.borrow_mut().close = Some(callback);
    }
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().appearance_changed = Some(callback);
    }
    fn draw(&self, _scene: &Scene) {
        // TODO Phase 1.3: submit scene to Metal renderer
    }
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        // TODO Phase 1.3: return real Metal atlas
        unimplemented!("Metal sprite atlas not yet implemented for iOS")
    }
    fn is_subpixel_rendering_supported(&self) -> bool {
        // iOS LCD subpixel rendering is not supported (all displays are OLED/retina).
        false
    }
    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }
    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {
        // TODO Phase 1.3: reposition software keyboard via UITextInput
    }
    fn window_controls(&self) -> WindowControls {
        WindowControls {
            fullscreen: false,
            maximize: false,
            minimize: false,
            window_menu: false,
        }
    }
}
