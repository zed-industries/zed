//! iOS Window implementation using UIWindow and UIViewController.
//!
//! iOS windows are fundamentally different from desktop windows:
//! - Always fullscreen (or split-screen on iPad)
//! - No title bar or window chrome
//! - Touch-based input
//! - Safe area insets for notch/home indicator
//!
//! The window is backed by a UIWindow containing a UIViewController
//! whose view hosts the Metal rendering layer.

use super::{IosDisplay, events::*};
use crate::platform::blade;
use crate::{
    AnyWindowHandle, Bounds, DispatchEventResult, GpuSpecs, Modifiers, Pixels, PlatformAtlas,
    PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptButton,
    PromptLevel, RequestFrameOptions, Scene, Size, WindowAppearance, WindowBackgroundAppearance,
    WindowBounds, WindowControlArea, WindowParams,
};
use anyhow::{Result, anyhow};
use core_graphics::{
    base::CGFloat,
    geometry::{CGPoint, CGRect, CGSize},
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{BOOL, Class, NO, Object, Sel, YES},
    sel, sel_impl,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, UiKitDisplayHandle, UiKitWindowHandle};
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    ptr::{self, NonNull},
    rc::Rc,
    sync::Arc,
};

const GPUI_VIEW_IVAR: &str = "gpui_view";
const GPUI_WINDOW_IVAR: &str = "gpui_window_ptr";

static METAL_VIEW_CLASS_REGISTERED: std::sync::Once = std::sync::Once::new();

/// Register a custom UIView subclass that uses CAMetalLayer as its backing layer.
/// This is required for Metal rendering on iOS.
fn register_metal_view_class() -> &'static Class {
    METAL_VIEW_CLASS_REGISTERED.call_once(|| {
        let superclass = class!(UIView);
        let mut decl = ClassDecl::new("GPUIMetalView", superclass).unwrap();

        // Add ivar to store window pointer for touch handling
        decl.add_ivar::<*mut std::ffi::c_void>(GPUI_WINDOW_IVAR);

        // Override layerClass to return CAMetalLayer
        extern "C" fn layer_class(_self: &Class, _sel: Sel) -> *const Class {
            class!(CAMetalLayer) as *const Class
        }

        // Touch handling methods
        extern "C" fn touches_began(
            this: &mut Object,
            _sel: Sel,
            touches: *mut Object,
            event: *mut Object,
        ) {
            handle_touches(this, touches, event);
        }

        extern "C" fn touches_moved(
            this: &mut Object,
            _sel: Sel,
            touches: *mut Object,
            event: *mut Object,
        ) {
            handle_touches(this, touches, event);
        }

        extern "C" fn touches_ended(
            this: &mut Object,
            _sel: Sel,
            touches: *mut Object,
            event: *mut Object,
        ) {
            handle_touches(this, touches, event);
        }

        extern "C" fn touches_cancelled(
            this: &mut Object,
            _sel: Sel,
            touches: *mut Object,
            event: *mut Object,
        ) {
            handle_touches(this, touches, event);
        }

        unsafe {
            // Add class method for layerClass
            decl.add_class_method(
                sel!(layerClass),
                layer_class as extern "C" fn(&Class, Sel) -> *const Class,
            );

            // Add touch handling instance methods
            decl.add_method(
                sel!(touchesBegan:withEvent:),
                touches_began as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(touchesMoved:withEvent:),
                touches_moved as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(touchesEnded:withEvent:),
                touches_ended as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(touchesCancelled:withEvent:),
                touches_cancelled as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object),
            );
        }

        decl.register();
    });

    class!(GPUIMetalView)
}

/// Handle touch events from the GPUIMetalView
fn handle_touches(view: &mut Object, touches: *mut Object, event: *mut Object) {
    unsafe {
        // Get the window pointer from the view's ivar
        let window_ptr: *mut std::ffi::c_void = *view.get_ivar(GPUI_WINDOW_IVAR);
        if window_ptr.is_null() {
            log::warn!("GPUI iOS: Touch event but no window pointer set");
            return;
        }

        let window = &*(window_ptr as *const IosWindow);

        // Get all touches from the set
        let all_touches: *mut Object = msg_send![touches, allObjects];
        let count: usize = msg_send![all_touches, count];

        for i in 0..count {
            let touch: *mut Object = msg_send![all_touches, objectAtIndex: i];
            window.handle_touch(touch, event);
        }
    }
}

/// iOS Window backed by UIWindow + UIViewController.
pub(crate) struct IosWindow {
    /// Handle used by GPUI to identify this window
    handle: AnyWindowHandle,
    /// The UIWindow object
    window: *mut Object,
    /// The UIViewController
    view_controller: *mut Object,
    /// The Metal-backed UIView
    view: *mut Object,
    /// The hidden text input view for keyboard input
    text_input_view: *mut Object,
    /// Current bounds in pixels
    bounds: Cell<Bounds<Pixels>>,
    /// Scale factor
    scale_factor: Cell<f32>,
    /// Appearance (light/dark mode)
    appearance: Cell<WindowAppearance>,
    /// Input handler for text input
    input_handler: RefCell<Option<PlatformInputHandler>>,
    /// Callback for frame requests
    /// Note: pub(super) to allow ffi.rs to access this for the display link callback
    pub(super) request_frame_callback: RefCell<Option<Box<dyn FnMut(RequestFrameOptions)>>>,
    /// Callback for input events
    input_callback: RefCell<Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>>,
    /// Callback for active status changes
    active_status_callback: RefCell<Option<Box<dyn FnMut(bool)>>>,
    /// Callback for hover status changes (not really applicable on iOS)
    hover_status_callback: RefCell<Option<Box<dyn FnMut(bool)>>>,
    /// Callback for resize events
    resize_callback: RefCell<Option<Box<dyn FnMut(Size<Pixels>, f32)>>>,
    /// Callback for move events (not applicable on iOS)
    moved_callback: RefCell<Option<Box<dyn FnMut()>>>,
    /// Callback for should close
    should_close_callback: RefCell<Option<Box<dyn FnMut() -> bool>>>,
    /// Callback for hit test
    hit_test_callback: RefCell<Option<Box<dyn FnMut() -> Option<WindowControlArea>>>>,
    /// Callback for close
    close_callback: RefCell<Option<Box<dyn FnOnce()>>>,
    /// Callback for appearance changes
    appearance_changed_callback: RefCell<Option<Box<dyn FnMut()>>>,
    /// Current mouse position (from touch)
    mouse_position: Cell<Point<Pixels>>,
    /// Current modifiers
    modifiers: Cell<Modifiers>,
    /// Blade renderer for GPU rendering
    renderer: RefCell<blade::Renderer>,
    /// Track if a touch is currently pressed
    touch_pressed: Cell<bool>,
}

// Required for raw_window_handle
unsafe impl Send for IosWindow {}
unsafe impl Sync for IosWindow {}

impl IosWindow {
    pub fn new(
        handle: AnyWindowHandle,
        _params: WindowParams,
        renderer_context: blade::Context,
    ) -> Result<Self> {
        // Create the window on the main screen
        let screen = IosDisplay::main();
        let screen_bounds = screen.bounds();
        let scale_factor = screen.scale();

        unsafe {
            // Create UIWindow
            let screen_obj: *mut Object = msg_send![class!(UIScreen), mainScreen];
            let screen_bounds_cg: CGRect = msg_send![screen_obj, bounds];
            let window: *mut Object = msg_send![class!(UIWindow), alloc];
            let window: *mut Object = msg_send![window, initWithFrame: screen_bounds_cg];

            // Create UIViewController
            let view_controller: *mut Object = msg_send![class!(UIViewController), alloc];
            let view_controller: *mut Object = msg_send![view_controller, init];

            // Create our custom Metal view using the registered class
            let metal_view_class = register_metal_view_class();
            let view: *mut Object = msg_send![metal_view_class, alloc];
            let view: *mut Object = msg_send![view, initWithFrame: screen_bounds_cg];

            // Configure the Metal layer
            let layer: *mut Object = msg_send![view, layer];

            // Get the Metal device using the Metal framework function
            #[link(name = "Metal", kind = "framework")]
            unsafe extern "C" {
                fn MTLCreateSystemDefaultDevice() -> *mut Object;
            }
            let device = MTLCreateSystemDefaultDevice();
            if !device.is_null() {
                let _: () = msg_send![layer, setDevice: device];
            }
            let _: () = msg_send![layer, setPixelFormat: 80_u64]; // MTLPixelFormatBGRA8Unorm
            let _: () = msg_send![layer, setFramebufferOnly: NO];
            let scale: CGFloat = msg_send![screen_obj, scale];
            let _: () = msg_send![layer, setContentsScale: scale];
            let drawable_size = CGSize {
                width: screen_bounds_cg.size.width * scale,
                height: screen_bounds_cg.size.height * scale,
            };
            let _: () = msg_send![layer, setDrawableSize: drawable_size];

            // Enable user interaction on the Metal view for touch handling
            let _: () = msg_send![view, setUserInteractionEnabled: YES];
            let _: () = msg_send![view, setMultipleTouchEnabled: YES];

            // Set the view as the view controller's view
            let _: () = msg_send![view_controller, setView: view];

            // Set the root view controller
            let _: () = msg_send![window, setRootViewController: view_controller];

            // Make the window visible
            let _: () = msg_send![window, makeKeyAndVisible];

            // Create a hidden text input view for keyboard handling
            // This view conforms to UIKeyInput and handles keyboard events
            let text_input_view: *mut Object = msg_send![class!(UIView), alloc];
            let text_input_frame = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize {
                    width: 1.0,
                    height: 1.0,
                },
            };
            let text_input_view: *mut Object =
                msg_send![text_input_view, initWithFrame: text_input_frame];
            // Make it invisible but still able to become first responder
            let _: () = msg_send![text_input_view, setAlpha: 0.01_f64];
            let _: () = msg_send![text_input_view, setUserInteractionEnabled: YES];
            let _: () = msg_send![view, addSubview: text_input_view];

            // Create the blade renderer
            // Note: Blade expects size in pixels (device pixels), not points
            let renderer = blade::new_renderer(
                renderer_context,
                window as *mut c_void,
                view as *mut c_void,
                crate::Size {
                    width: drawable_size.width as f32,
                    height: drawable_size.height as f32,
                },
                false, // not transparent
            );

            let ios_window = Self {
                handle,
                window,
                view_controller,
                view,
                text_input_view,
                bounds: Cell::new(screen_bounds),
                scale_factor: Cell::new(scale_factor),
                appearance: Cell::new(WindowAppearance::Light),
                input_handler: RefCell::new(None),
                request_frame_callback: RefCell::new(None),
                input_callback: RefCell::new(None),
                active_status_callback: RefCell::new(None),
                hover_status_callback: RefCell::new(None),
                resize_callback: RefCell::new(None),
                moved_callback: RefCell::new(None),
                should_close_callback: RefCell::new(None),
                hit_test_callback: RefCell::new(None),
                close_callback: RefCell::new(None),
                appearance_changed_callback: RefCell::new(None),
                mouse_position: Cell::new(Point::default()),
                modifiers: Cell::new(Modifiers::default()),
                renderer: RefCell::new(renderer),
                touch_pressed: Cell::new(false),
            };

            Ok(ios_window)
        }
    }

    /// Register this window with the FFI layer after it's been stored.
    /// This must be called after the window is placed at a stable address
    /// (e.g., in a Box or Arc).
    pub(crate) fn register_with_ffi(&self) {
        super::ffi::register_window(self as *const Self);

        // Set the window pointer on the view so touch events can find us
        unsafe {
            let window_ptr = self as *const Self as *mut std::ffi::c_void;
            (*self.view).set_ivar(GPUI_WINDOW_IVAR, window_ptr);
            log::info!(
                "GPUI iOS: Set window pointer {:p} on view {:p}",
                window_ptr,
                self.view
            );
        }
    }

    /// Handle a touch event from UIKit
    pub fn handle_touch(&self, touch: *mut Object, _event: *mut Object) {
        let position = touch_location_in_view(touch, self.view);
        let phase = touch_phase(touch);
        let tap_count = touch_tap_count(touch);
        let modifiers = self.modifiers.get();

        self.mouse_position.set(position);

        let platform_input = match phase {
            UITouchPhase::Began => {
                self.touch_pressed.set(true);
                touch_began_to_mouse_down(position, tap_count, modifiers)
            }
            UITouchPhase::Moved => {
                touch_moved_to_mouse_move(position, modifiers, Some(crate::MouseButton::Left))
            }
            UITouchPhase::Ended | UITouchPhase::Cancelled => {
                self.touch_pressed.set(false);
                touch_ended_to_mouse_up(position, tap_count, modifiers)
            }
            UITouchPhase::Stationary => return,
        };

        if let Some(callback) = self.input_callback.borrow_mut().as_mut() {
            callback(platform_input);
        }
    }

    /// Get the safe area insets
    pub fn safe_area_insets(&self) -> (f32, f32, f32, f32) {
        unsafe {
            // UIEdgeInsets struct
            #[repr(C)]
            struct UIEdgeInsets {
                top: f64,
                left: f64,
                bottom: f64,
                right: f64,
            }

            let insets: UIEdgeInsets = msg_send![self.view, safeAreaInsets];
            (
                insets.top as f32,
                insets.left as f32,
                insets.bottom as f32,
                insets.right as f32,
            )
        }
    }

    /// Show the software keyboard
    pub fn show_keyboard(&self) {
        log::info!("GPUI iOS: Showing keyboard");
        unsafe {
            // Make the text input view become first responder to show keyboard
            let _: BOOL = msg_send![self.text_input_view, becomeFirstResponder];
        }
    }

    /// Hide the software keyboard
    pub fn hide_keyboard(&self) {
        log::info!("GPUI iOS: Hiding keyboard");
        unsafe {
            // Resign first responder to hide keyboard
            let _: BOOL = msg_send![self.text_input_view, resignFirstResponder];
        }
    }

    /// Handle text input from the software keyboard
    pub fn handle_text_input(&self, text: *mut Object) {
        if text.is_null() {
            return;
        }

        unsafe {
            // Convert NSString to Rust String
            let utf8: *const i8 = msg_send![text, UTF8String];
            if utf8.is_null() {
                return;
            }

            let text_str = std::ffi::CStr::from_ptr(utf8)
                .to_string_lossy()
                .into_owned();

            log::info!("GPUI iOS: Text input: {:?}", text_str);

            // First try the input handler (for text fields)
            if let Some(handler) = self.input_handler.borrow_mut().as_mut() {
                handler.replace_text_in_range(None, &text_str);
                return;
            }

            // Otherwise, send as key events
            for c in text_str.chars() {
                let keystroke = crate::Keystroke {
                    modifiers: Modifiers::default(),
                    key: c.to_string(),
                    key_char: Some(c.to_string()),
                };

                let event = PlatformInput::KeyDown(crate::KeyDownEvent {
                    keystroke,
                    is_held: false,
                    prefer_character_input: true,
                });

                if let Some(callback) = self.input_callback.borrow_mut().as_mut() {
                    callback(event);
                }
            }
        }
    }

    /// Handle a key event from an external keyboard
    pub fn handle_key_event(&self, key_code: u32, modifier_flags: u32, is_key_down: bool) {
        use super::text_input::{
            key_code_to_key_down, key_code_to_key_up, key_code_to_string,
            modifier_flags_to_modifiers,
        };

        let key = key_code_to_string(key_code);
        let modifiers = modifier_flags_to_modifiers(modifier_flags);

        log::info!(
            "GPUI iOS: Key event - key: {:?}, modifiers: {:?}, down: {}",
            key,
            modifiers,
            is_key_down
        );

        let event = if is_key_down {
            key_code_to_key_down(key_code, modifier_flags)
        } else {
            key_code_to_key_up(key_code, modifier_flags)
        };

        if let Some(callback) = self.input_callback.borrow_mut().as_mut() {
            callback(event);
        }
    }

    /// Notify the window of active status changes (foreground/background).
    ///
    /// This is called by the FFI layer when the app transitions between
    /// foreground and background states.
    pub fn notify_active_status_change(&self, is_active: bool) {
        log::info!("GPUI iOS: Window active status changed to: {}", is_active);

        if let Some(callback) = self.active_status_callback.borrow_mut().as_mut() {
            callback(is_active);
        }
    }
}

impl HasWindowHandle for IosWindow {
    fn window_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError>
    {
        let view = NonNull::new(self.view as *mut c_void)
            .ok_or(raw_window_handle::HandleError::Unavailable)?;
        let handle = UiKitWindowHandle::new(view);
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
    }
}

impl HasDisplayHandle for IosWindow {
    fn display_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError>
    {
        let handle = UiKitDisplayHandle::new();
        Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(handle.into()) })
    }
}

impl PlatformWindow for IosWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds.get()
    }

    fn is_maximized(&self) -> bool {
        true // iOS windows are always "maximized"
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.bounds.get())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds.get().size
    }

    fn resize(&mut self, _size: Size<Pixels>) {
        // iOS windows cannot be resized programmatically
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor.get()
    }

    fn appearance(&self) -> WindowAppearance {
        unsafe {
            let trait_collection: *mut Object = msg_send![self.view, traitCollection];
            let style: i64 = msg_send![trait_collection, userInterfaceStyle];
            match style {
                2 => WindowAppearance::Dark,
                _ => WindowAppearance::Light,
            }
        }
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(IosDisplay::main()))
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.mouse_position.get()
    }

    fn modifiers(&self) -> Modifiers {
        self.modifiers.get()
    }

    fn capslock(&self) -> crate::Capslock {
        // Would need to check UIKeyModifierFlags
        crate::Capslock { on: false }
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        *self.input_handler.borrow_mut() = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.input_handler.borrow_mut().take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        // Would use UIAlertController
        let (_tx, rx) = futures::channel::oneshot::channel();

        unsafe {
            // Create UIAlertController
            let title = msg;
            let message = detail.unwrap_or("");

            let alert_style: i64 = 1; // UIAlertControllerStyleAlert

            let title_str: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: title.as_ptr()];
            let message_str: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: message.as_ptr()];

            let alert: *mut Object = msg_send![
                class!(UIAlertController),
                alertControllerWithTitle: title_str
                message: message_str
                preferredStyle: alert_style
            ];

            // Add buttons
            for (_index, button) in answers.iter().enumerate() {
                let button_title: *mut Object = msg_send![
                    class!(NSString),
                    stringWithUTF8String: button.label().as_str().as_ptr()
                ];

                let action_style: i64 = if button.is_cancel() { 1 } else { 0 }; // UIAlertActionStyleCancel or Default

                // Note: In production, this would need a block that calls tx.send(index)
                let action: *mut Object = msg_send![
                    class!(UIAlertAction),
                    actionWithTitle: button_title
                    style: action_style
                    handler: ptr::null::<Object>()
                ];

                let _: () = msg_send![alert, addAction: action];
            }

            // Present the alert
            let _: () = msg_send![
                self.view_controller,
                presentViewController: alert
                animated: YES
                completion: ptr::null::<Object>()
            ];
        }

        Some(rx)
    }

    fn activate(&self) {
        unsafe {
            let _: () = msg_send![self.window, makeKeyAndVisible];
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let key_window: *mut Object = msg_send![app, keyWindow];
            self.window == key_window
        }
    }

    fn is_hovered(&self) -> bool {
        // Hover isn't really applicable on iOS
        false
    }

    fn set_title(&mut self, _title: &str) {
        // iOS apps don't have window titles
    }

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {
        // Could adjust view background color
    }

    fn minimize(&self) {
        // iOS apps cannot be minimized
    }

    fn zoom(&self) {
        // iOS apps cannot be zoomed
    }

    fn toggle_fullscreen(&self) {
        // iOS apps are always fullscreen
    }

    fn is_fullscreen(&self) -> bool {
        true
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        *self.request_frame_callback.borrow_mut() = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        *self.input_callback.borrow_mut() = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        *self.active_status_callback.borrow_mut() = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        *self.hover_status_callback.borrow_mut() = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        *self.resize_callback.borrow_mut() = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        *self.moved_callback.borrow_mut() = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        *self.should_close_callback.borrow_mut() = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        *self.hit_test_callback.borrow_mut() = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        *self.close_callback.borrow_mut() = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        *self.appearance_changed_callback.borrow_mut() = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        self.renderer.borrow_mut().draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.renderer.borrow().sprite_atlas().clone()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        // Would query Metal device capabilities
        None
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {
        // iOS handles IME positioning automatically
    }
}
