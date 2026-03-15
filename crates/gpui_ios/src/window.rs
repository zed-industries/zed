use crate::{
    display_link::DisplayLink,
    metal_renderer::{InstanceBufferPool, MetalRenderer},
};
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Bounds, Capslock, DevicePixels, DispatchEventResult, GpuSpecs, Modifiers, PlatformAtlas,
    PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Pixels, Point,
    PromptButton, PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls, WindowParams,
};
use objc::{
    class, declare::ClassDecl, msg_send, runtime::{Class, Object}, sel, sel_impl,
};
use parking_lot::Mutex;
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

// ─── CGRect / UIKit geometry ──────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CGPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CGSize {
    width: f64,
    height: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CGRect {
    origin: CGPoint,
    size: CGSize,
}

// ─── Callbacks ────────────────────────────────────────────────────────────────

#[derive(Default)]
struct IosWindowCallbacks {
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

// ─── IosWindow ────────────────────────────────────────────────────────────────

struct IosWindowInner {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    input_handler: Option<PlatformInputHandler>,
    /// Retained pointer to the UIView we created.
    ui_view: *mut Object,
}

/// iOS window backed by a UIView with a CAMetalLayer sublayer.
///
/// On creation we find the key UIWindow (created by Swift SceneDelegate),
/// attach a full-screen UIView to it, and add the renderer's CAMetalLayer
/// as a sublayer. CADisplayLink drives the frame request callback.
pub struct IosWindow {
    inner: RefCell<IosWindowInner>,
    callbacks: RefCell<IosWindowCallbacks>,
    renderer: RefCell<MetalRenderer>,
    display_link: RefCell<Option<DisplayLink>>,
    display: Rc<dyn PlatformDisplay>,
}

impl IosWindow {
    pub fn new(
        _params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
        instance_buffer_pool: Arc<Mutex<InstanceBufferPool>>,
    ) -> Result<Self> {
        let mut renderer = MetalRenderer::new(instance_buffer_pool);
        let (bounds, scale_factor, ui_view) = Self::create_and_attach_view(&mut renderer)?;

        Ok(Self {
            inner: RefCell::new(IosWindowInner {
                bounds,
                scale_factor,
                input_handler: None,
                ui_view,
            }),
            callbacks: RefCell::new(IosWindowCallbacks::default()),
            renderer: RefCell::new(renderer),
            display_link: RefCell::new(None),
            display,
        })
    }

    /// Creates a UIView filling the key UIWindow, adds the renderer's
    /// CAMetalLayer as a sublayer, and configures the drawable size.
    /// Returns the logical bounds (points), native scale factor, and the UIView pointer.
    fn create_and_attach_view(
        renderer: &mut MetalRenderer,
    ) -> Result<(Bounds<Pixels>, f32, *mut Object)> {
        unsafe {
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let key_window: *mut Object = msg_send![app, keyWindow];
            anyhow::ensure!(
                !key_window.is_null(),
                "no key UIWindow — SceneDelegate must call makeKeyAndVisible before zed_ios_open_window"
            );

            // UIWindow.bounds is CGRectZero immediately after makeKeyAndVisible() in iOS 16+
            // because the layout system hasn't run yet. UIScreen.mainScreen.bounds is
            // orientation-aware (returns correct portrait/landscape logical size) and is
            // available as soon as a UIWindowScene is connected — i.e. before any layout pass.
            // Multiply by `scale` (not `nativeScale`, which is always the hardware native scale
            // and may be portrait-basis only on some iPads) to get device pixels.
            let main_screen: *mut Object = msg_send![class!(UIScreen), mainScreen];
            let screen_bounds: CGRect = msg_send![main_screen, bounds];
            let scale: f32 = msg_send![main_screen, scale];
            let scale = if scale > 0.0 { scale } else { 2.0 };

            let logical_width = screen_bounds.size.width as f32;
            let logical_height = screen_bounds.size.height as f32;
            let device_width = (logical_width * scale).round() as i32;
            let device_height = (logical_height * scale).round() as i32;

            let bounds = Bounds {
                origin: gpui::Point::default(),
                size: gpui::Size {
                    width: gpui::px(logical_width),
                    height: gpui::px(logical_height),
                },
            };

            let logical_frame = CGRect {
                origin: CGPoint::default(),
                size: CGSize {
                    width: logical_width as f64,
                    height: logical_height as f64,
                },
            };

            let view_class = register_metal_view_class();
            let view: *mut Object = msg_send![view_class, alloc];
            let view: *mut Object = msg_send![view, initWithFrame: logical_frame];

            // Stretch to fill the window on rotation or Stage Manager resize.
            let fill_mask: usize = (1 << 1) | (1 << 4); // UIViewAutoresizingFlexibleWidth | Height
            let _: () = msg_send![view, setAutoresizingMask: fill_mask];

            // Add the renderer's CAMetalLayer as a sublayer of the view's root layer.
            let layer_ptr = renderer.layer_ptr();
            let _: () = msg_send![layer_ptr, setFrame: logical_frame];
            let view_layer: *mut Object = msg_send![view, layer];
            let _: () = msg_send![view_layer, addSublayer: layer_ptr];

            let _: () = msg_send![key_window, addSubview: view];

            renderer.update_drawable_size(gpui::size(
                DevicePixels(device_width),
                DevicePixels(device_height),
            ));

            Ok((bounds, scale, view))
        }
    }
}

// ─── raw-window-handle ───────────────────────────────────────────────────────

impl HasWindowHandle for IosWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let inner = self.inner.borrow();
        let ptr =
            NonNull::new(inner.ui_view as *mut c_void).ok_or(HandleError::Unavailable)?;
        let handle = UiKitWindowHandle::new(ptr);
        Ok(unsafe { WindowHandle::borrow_raw(handle.into()) })
    }
}

impl HasDisplayHandle for IosWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = UiKitDisplayHandle::new();
        Ok(unsafe { DisplayHandle::borrow_raw(handle.into()) })
    }
}

// ─── PlatformWindow ──────────────────────────────────────────────────────────

impl PlatformWindow for IosWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.inner.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
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
        unsafe {
            let view = self.inner.borrow().ui_view;
            if view.is_null() {
                return WindowAppearance::Light;
            }
            let trait_collection: *mut Object = msg_send![view, traitCollection];
            // UIUserInterfaceStyleDark == 2
            let style: usize = msg_send![trait_collection, userInterfaceStyle];
            if style == 2 {
                WindowAppearance::Dark
            } else {
                WindowAppearance::Light
            }
        }
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        // Tracked via UIPointerInteraction in Phase 1.3.
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
        // TODO Phase 2: UIAlertController prompt
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
        // UIWindowScene title (shown in App Switcher) — Phase 1.3.
    }

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        true
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        // Store callback in thread-local; CADisplayLink fires on the main thread.
        FRAME_CALLBACK.with(|slot| {
            *slot.borrow_mut() = Some(callback);
        });

        let mut display_link = self.display_link.borrow_mut();
        if display_link.is_none() {
            *display_link = Some(DisplayLink::new(std::ptr::null_mut(), display_link_fired));
        }
        display_link.as_ref().unwrap().start();
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

    fn draw(&self, scene: &Scene) {
        self.renderer.borrow_mut().draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.renderer.borrow().sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
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

// ─── CADisplayLink callback ───────────────────────────────────────────────────

thread_local! {
    /// The `request_frame` callback registered by GPUI, driven by CADisplayLink.
    /// Thread-local is sound: CADisplayLink fires on the main thread, and all
    /// GPUI window callbacks execute on the main thread.
    static FRAME_CALLBACK: RefCell<Option<Box<dyn FnMut(RequestFrameOptions)>>> =
        RefCell::new(None);
}

extern "C" fn display_link_fired(_data: *mut c_void) {
    FRAME_CALLBACK.with(|slot| {
        if let Some(callback) = slot.borrow_mut().as_mut() {
            callback(RequestFrameOptions::default());
        }
    });
}

// ─── UIView subclass ──────────────────────────────────────────────────────────

fn register_metal_view_class() -> &'static Class {
    use std::sync::OnceLock;
    static CLASS: OnceLock<&'static Class> = OnceLock::new();
    CLASS.get_or_init(|| {
        let superclass = class!(UIView);
        let decl =
            ClassDecl::new("ZedMetalView", superclass).expect("ZedMetalView already registered");
        decl.register()
    })
}
