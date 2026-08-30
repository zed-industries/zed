//! iOS Window implementation using UIWindow and UIViewController.
//!
//! iOS windows are fundamentally different from desktop windows:
//! - Always fullscreen (or split-screen on iPad)
//! - No title bar or window chrome
//! - Touch-based input
//! - Safe area insets for notch/home indicator
//!
//! The window is backed by a UIWindow containing a UIViewController
//! whose view hosts a CAMetalLayer.

use super::IosDisplay;
use super::events::*;
use gpui::{
    AnyWindowHandle, Bounds, Capslock, DevicePixels, DispatchEventResult, Edges, GpuSpecs,
    Modifiers, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions, Scene, Size,
    TextInputStateChange, TouchEvent, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    WindowControlArea, WindowInsets, WindowParams, px, size,
};
use gpui_apple::metal_renderer::{Context as MetalContext, MetalRenderer};
use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel};
use objc2::{class, msg_send, sel};

use super::cg_types::ObjcCGRect;
use parking_lot::Mutex;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, UiKitDisplayHandle, UiKitWindowHandle};
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    ptr::{self, NonNull},
    rc::Rc,
    sync::Arc,
};

const GPUI_WINDOW_IVAR: &str = "gpui_window_ptr";

static METAL_VIEW_CLASS_REGISTERED: std::sync::Once = std::sync::Once::new();
static VC_CLASS_REGISTERED: std::sync::Once = std::sync::Once::new();
static TEXT_INPUT_VIEW_CLASS_REGISTERED: std::sync::Once = std::sync::Once::new();
static KEYBOARD_OBSERVERS_REGISTERED: std::sync::Once = std::sync::Once::new();

/// Global storage for the current status bar style.
/// 0 = default (dark content), 1 = light content.
/// Accessed from the main thread only.
static STATUS_BAR_STYLE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Register a custom UIViewController subclass that allows overriding
/// `preferredStatusBarStyle` at runtime.
fn register_view_controller_class() -> &'static AnyClass {
    VC_CLASS_REGISTERED.call_once(|| {
        let superclass = class!(UIViewController);
        let Some(mut decl) = ClassBuilder::new(c"GPUIViewController", superclass) else {
            return;
        };

        // Override preferredStatusBarStyle
        extern "C" fn preferred_status_bar_style(_this: *mut AnyObject, _sel: Sel) -> isize {
            let style = STATUS_BAR_STYLE.load(std::sync::atomic::Ordering::Relaxed);
            if style == 1 {
                1 // UIStatusBarStyleLightContent
            } else {
                3 // UIStatusBarStyleDarkContent (iOS 13+)
            }
        }

        // Override viewDidLayoutSubviews — called by UIKit on rotation,
        // split-screen changes, and any other layout pass.
        extern "C" fn view_did_layout_subviews(this: *mut AnyObject, _sel: Sel) {
            // Call super
            unsafe {
                let superclass = class!(UIViewController);
                let _: () = msg_send![super(this, superclass), viewDidLayoutSubviews];
            }

            // Notify all registered GPUI windows about the layout change.
            if let Some(wrapper) = super::ffi::IOS_WINDOW_LIST.get() {
                unsafe {
                    let windows = &*wrapper.0.get();
                    for &window_ptr in windows.iter() {
                        if !window_ptr.is_null() {
                            let window = &*window_ptr;
                            window.handle_layout_change();
                        }
                    }
                }
            }
        }

        // Run edge-to-edge: hide the status bar and let the home indicator
        // fade so the editor gets the full screen.
        extern "C" fn prefers_status_bar_hidden(_this: *mut AnyObject, _sel: Sel) -> Bool {
            Bool::YES
        }

        extern "C" fn prefers_home_indicator_auto_hidden(_this: *mut AnyObject, _sel: Sel) -> Bool {
            Bool::YES
        }

        // UIRectEdgeAll: require a second swipe for system gestures at the
        // screen edges, matching other full-screen apps.
        extern "C" fn preferred_screen_edges_deferring_system_gestures(
            _this: *mut AnyObject,
            _sel: Sel,
        ) -> usize {
            15
        }

        unsafe {
            decl.add_method(
                sel!(preferredStatusBarStyle),
                preferred_status_bar_style as extern "C" fn(*mut AnyObject, Sel) -> isize,
            );
            decl.add_method(
                sel!(viewDidLayoutSubviews),
                view_did_layout_subviews as extern "C" fn(*mut AnyObject, Sel),
            );
            decl.add_method(
                sel!(prefersStatusBarHidden),
                prefers_status_bar_hidden as extern "C" fn(*mut AnyObject, Sel) -> Bool,
            );
            decl.add_method(
                sel!(prefersHomeIndicatorAutoHidden),
                prefers_home_indicator_auto_hidden as extern "C" fn(*mut AnyObject, Sel) -> Bool,
            );
            decl.add_method(
                sel!(preferredScreenEdgesDeferringSystemGestures),
                preferred_screen_edges_deferring_system_gestures
                    as extern "C" fn(*mut AnyObject, Sel) -> usize,
            );
        }

        decl.register();
    });

    class!(GPUIViewController)
}

/// Set the iOS status bar content style (light or dark text/icons).
///
/// This updates the stored style and asks the root view controller
/// to re-query `preferredStatusBarStyle`.
pub fn set_status_bar_style(style: crate::StatusBarContentStyle) {
    use crate::StatusBarContentStyle;

    let value = match style {
        StatusBarContentStyle::Light => 1,
        StatusBarContentStyle::Dark => 0,
    };
    STATUS_BAR_STYLE.store(value, std::sync::atomic::Ordering::Relaxed);

    // Ask UIKit to re-query the status bar style
    unsafe {
        if let Some(wrapper) = super::ffi::IOS_WINDOW_LIST.get() {
            let windows = &*wrapper.0.get();
            if let Some(&window_ptr) = windows.last() {
                if !window_ptr.is_null() {
                    let window = &*window_ptr;
                    let vc = window.view_controller;
                    if !vc.is_null() {
                        let _: () = msg_send![vc, setNeedsStatusBarAppearanceUpdate];
                    }
                }
            }
        }
    }
}

/// Register a custom UIView subclass that uses CAMetalLayer as its backing layer.
/// This is required for Metal rendering on iOS.
fn register_metal_view_class() -> &'static AnyClass {
    METAL_VIEW_CLASS_REGISTERED.call_once(|| {
        let superclass = class!(UIView);
        let Some(mut decl) = ClassBuilder::new(c"GPUIMetalView", superclass) else {
            return;
        };

        // Add ivar to store window pointer for touch handling
        decl.add_ivar::<*mut std::ffi::c_void>(c"gpui_window_ptr");

        // Override layerClass to return CAMetalLayer
        extern "C" fn layer_class(_self: *const AnyClass, _sel: Sel) -> *const AnyClass {
            class!(CAMetalLayer) as *const AnyClass
        }

        // Touch handling methods
        extern "C" fn touches_began(
            this: *mut AnyObject,
            _sel: Sel,
            touches: *mut AnyObject,
            event: *mut AnyObject,
        ) {
            handle_touches(this, touches, event);
        }

        extern "C" fn touches_moved(
            this: *mut AnyObject,
            _sel: Sel,
            touches: *mut AnyObject,
            event: *mut AnyObject,
        ) {
            handle_touches(this, touches, event);
        }

        extern "C" fn touches_ended(
            this: *mut AnyObject,
            _sel: Sel,
            touches: *mut AnyObject,
            event: *mut AnyObject,
        ) {
            handle_touches(this, touches, event);
        }

        extern "C" fn touches_cancelled(
            this: *mut AnyObject,
            _sel: Sel,
            touches: *mut AnyObject,
            event: *mut AnyObject,
        ) {
            handle_touches(this, touches, event);
        }

        unsafe {
            // Add class method for layerClass
            decl.add_class_method(
                sel!(layerClass),
                layer_class as extern "C" fn(*const AnyClass, Sel) -> *const AnyClass,
            );

            // Add touch handling instance methods
            decl.add_method(
                sel!(touchesBegan:withEvent:),
                touches_began as extern "C" fn(*mut AnyObject, Sel, *mut AnyObject, *mut AnyObject),
            );
            decl.add_method(
                sel!(touchesMoved:withEvent:),
                touches_moved as extern "C" fn(*mut AnyObject, Sel, *mut AnyObject, *mut AnyObject),
            );
            decl.add_method(
                sel!(touchesEnded:withEvent:),
                touches_ended as extern "C" fn(*mut AnyObject, Sel, *mut AnyObject, *mut AnyObject),
            );
            decl.add_method(
                sel!(touchesCancelled:withEvent:),
                touches_cancelled
                    as extern "C" fn(*mut AnyObject, Sel, *mut AnyObject, *mut AnyObject),
            );
        }

        decl.register();
    });

    class!(GPUIMetalView)
}

/// Register a custom UIView subclass that implements UIKeyInput protocol.
///
/// iOS requires the first-responder view to conform to `UIKeyInput` in order
/// for the software keyboard to actually route typed characters back to the
/// app.  Without this, `becomeFirstResponder` silently fails and no keyboard
/// appears.
///
/// The three required methods:
/// - `hasText` → always returns YES (simplifies things; no harm)
/// - `insertText:` → forwards the text to `IosWindow::handle_text_input`
/// - `deleteBackward` → dispatches a backspace via `crate::dispatch_text_input`
fn register_text_input_view_class() -> &'static AnyClass {
    TEXT_INPUT_VIEW_CLASS_REGISTERED.call_once(|| {
        let superclass = class!(UIView);
        let Some(mut decl) = ClassBuilder::new(c"GPUITextInputView", superclass) else {
            return;
        };

        // Declare protocol conformance so iOS knows this view can receive
        // keyboard text input.
        if let Some(protocol) = objc2::runtime::AnyProtocol::get(c"UIKeyInput") {
            decl.add_protocol(protocol);
        }

        // Store the IosWindow pointer so callbacks can reach the Rust window.
        decl.add_ivar::<*mut std::ffi::c_void>(c"gpui_window_ptr");

        // UITextInputTraits property storage — UIView doesn't provide these,
        // but iOS reads them from the first responder to configure the keyboard.
        decl.add_ivar::<isize>(c"_keyboardType"); // UIKeyboardType
        decl.add_ivar::<isize>(c"_autocorrectionType"); // UITextAutocorrectionType
        decl.add_ivar::<isize>(c"_autocapitalizationType"); // UITextAutocapitalizationType

        // --- UIKeyInput protocol methods ---

        // Bool hasText
        unsafe extern "C" fn has_text(_this: *mut AnyObject, _sel: Sel) -> Bool {
            Bool::YES
        }

        // void insertText:(NSString *)text
        unsafe extern "C" fn insert_text(this: *mut AnyObject, _sel: Sel, text: *mut AnyObject) {
            let window_ptr: *mut std::ffi::c_void = unsafe {
                #[allow(deprecated)]
                *(*this).get_ivar(GPUI_WINDOW_IVAR)
            };
            if window_ptr.is_null() || text.is_null() {
                return;
            }
            let window = unsafe { &*(window_ptr as *const IosWindow) };
            window.handle_text_input(text);
        }

        // void deleteBackward
        unsafe extern "C" fn delete_backward(this: *mut AnyObject, _sel: Sel) {
            let window_ptr: *mut std::ffi::c_void = unsafe {
                #[allow(deprecated)]
                *(*this).get_ivar(GPUI_WINDOW_IVAR)
            };
            if window_ptr.is_null() {
                return;
            }
            let window = unsafe { &*(window_ptr as *const IosWindow) };
            window.handle_delete_backward();
        }

        // canBecomeFirstResponder must return Bool::YES
        unsafe extern "C" fn can_become_first_responder(_this: *mut AnyObject, _sel: Sel) -> Bool {
            Bool::YES
        }

        // --- UITextInputTraits property accessors ---
        #[allow(deprecated)]
        unsafe extern "C" fn get_keyboard_type(this: *mut AnyObject, _sel: Sel) -> isize {
            unsafe { *(*this).get_ivar::<isize>("_keyboardType") }
        }
        #[allow(deprecated)]
        unsafe extern "C" fn set_keyboard_type(this: *mut AnyObject, _sel: Sel, val: isize) {
            unsafe {
                *(*this).get_mut_ivar::<isize>("_keyboardType") = val;
            }
        }
        #[allow(deprecated)]
        unsafe extern "C" fn get_autocorrection_type(this: *mut AnyObject, _sel: Sel) -> isize {
            unsafe { *(*this).get_ivar::<isize>("_autocorrectionType") }
        }
        #[allow(deprecated)]
        unsafe extern "C" fn set_autocorrection_type(this: *mut AnyObject, _sel: Sel, val: isize) {
            unsafe {
                *(*this).get_mut_ivar::<isize>("_autocorrectionType") = val;
            }
        }
        #[allow(deprecated)]
        unsafe extern "C" fn get_autocapitalization_type(this: *mut AnyObject, _sel: Sel) -> isize {
            unsafe { *(*this).get_ivar::<isize>("_autocapitalizationType") }
        }
        #[allow(deprecated)]
        unsafe extern "C" fn set_autocapitalization_type(
            this: *mut AnyObject,
            _sel: Sel,
            val: isize,
        ) {
            unsafe {
                *(*this).get_mut_ivar::<isize>("_autocapitalizationType") = val;
            }
        }

        unsafe {
            decl.add_method(
                sel!(hasText),
                has_text as unsafe extern "C" fn(*mut AnyObject, Sel) -> Bool,
            );
            decl.add_method(
                sel!(insertText:),
                insert_text as unsafe extern "C" fn(*mut AnyObject, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(deleteBackward),
                delete_backward as unsafe extern "C" fn(*mut AnyObject, Sel),
            );
            decl.add_method(
                sel!(canBecomeFirstResponder),
                can_become_first_responder as unsafe extern "C" fn(*mut AnyObject, Sel) -> Bool,
            );

            // UITextInputTraits property methods
            decl.add_method(
                sel!(keyboardType),
                get_keyboard_type as unsafe extern "C" fn(*mut AnyObject, Sel) -> isize,
            );
            decl.add_method(
                sel!(setKeyboardType:),
                set_keyboard_type as unsafe extern "C" fn(*mut AnyObject, Sel, isize),
            );
            decl.add_method(
                sel!(autocorrectionType),
                get_autocorrection_type as unsafe extern "C" fn(*mut AnyObject, Sel) -> isize,
            );
            decl.add_method(
                sel!(setAutocorrectionType:),
                set_autocorrection_type as unsafe extern "C" fn(*mut AnyObject, Sel, isize),
            );
            decl.add_method(
                sel!(autocapitalizationType),
                get_autocapitalization_type as unsafe extern "C" fn(*mut AnyObject, Sel) -> isize,
            );
            decl.add_method(
                sel!(setAutocapitalizationType:),
                set_autocapitalization_type as unsafe extern "C" fn(*mut AnyObject, Sel, isize),
            );
        }

        decl.register();
    });

    class!(GPUITextInputView)
}

/// Handle touch events from the GPUIMetalView
fn handle_touches(view: *mut AnyObject, touches: *mut AnyObject, event: *mut AnyObject) {
    unsafe {
        // Get the window pointer from the view's ivar
        #[allow(deprecated)]
        let window_ptr: *mut std::ffi::c_void = *(*view).get_ivar(GPUI_WINDOW_IVAR);
        if window_ptr.is_null() {
            log::warn!("GPUI iOS: Touch event but no window pointer set");
            return;
        }

        let window = &*(window_ptr as *const IosWindow);

        // Get all touches from the set
        let all_touches: *mut AnyObject = msg_send![touches, allObjects];
        let count: usize = msg_send![all_touches, count];

        for i in 0..count {
            let touch: *mut AnyObject = msg_send![all_touches, objectAtIndex: i];
            window.handle_touch(touch, event);
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) struct IosWindow {
    /// The UIWindow object
    window: *mut AnyObject,
    /// The UIViewController
    view_controller: *mut AnyObject,
    /// The Metal-backed UIView
    view: *mut AnyObject,
    /// The hidden text input view for keyboard input
    text_input_view: *mut AnyObject,
    /// Current bounds in pixels
    bounds: Cell<Bounds<Pixels>>,
    /// Scale factor
    scale_factor: Cell<f32>,
    /// Input handler for text input
    input_handler: RefCell<Option<PlatformInputHandler>>,
    request_frame_callback: RefCell<Option<Box<dyn FnMut(RequestFrameOptions)>>>,
    force_next_frame: Cell<bool>,
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
    insets_changed_callback: RefCell<Option<Box<dyn FnMut(WindowInsets)>>>,
    keyboard_height: Cell<f32>,
    /// Current mouse position (from touch)
    mouse_position: Cell<Point<Pixels>>,
    /// Current modifiers
    modifiers: Cell<Modifiers>,
    renderer: Mutex<MetalRenderer>,
}

// Required for raw_window_handle
unsafe impl Send for IosWindow {}
unsafe impl Sync for IosWindow {}

impl IosWindow {
    pub fn new(_handle: AnyWindowHandle, _params: WindowParams) -> anyhow::Result<Self> {
        // Create the window on the main screen
        let screen = IosDisplay::main();
        let screen_bounds = screen.bounds();
        let scale_factor = screen.scale();

        unsafe {
            // Create UIWindow
            let window_scene = super::ffi::window_scene();
            let screen_obj: *mut AnyObject = if window_scene.is_null() {
                msg_send![class!(UIScreen), mainScreen]
            } else {
                msg_send![window_scene, screen]
            };
            let screen_bounds_cg: ObjcCGRect = msg_send![screen_obj, bounds];
            let window: *mut AnyObject = msg_send![class!(UIWindow), alloc];
            let window: *mut AnyObject = if window_scene.is_null() {
                msg_send![window, initWithFrame: screen_bounds_cg]
            } else {
                let window: *mut AnyObject = msg_send![window, initWithWindowScene: window_scene];
                let _: () = msg_send![window, setFrame: screen_bounds_cg];
                window
            };

            // Create our custom UIViewController subclass that supports
            // dynamic `preferredStatusBarStyle` overrides.
            let vc_class = register_view_controller_class();
            let view_controller: *mut AnyObject = msg_send![vc_class, alloc];
            let view_controller: *mut AnyObject = msg_send![view_controller, init];

            // Create our custom Metal view using the registered class
            let metal_view_class = register_metal_view_class();
            let view: *mut AnyObject = msg_send![metal_view_class, alloc];
            let view: *mut AnyObject = msg_send![view, initWithFrame: screen_bounds_cg];

            let layer: *mut AnyObject = msg_send![view, layer];
            let scale: core_graphics::base::CGFloat = msg_send![screen_obj, scale];
            let _: () = msg_send![layer, setContentsScale: scale];

            // Auto-resize the Metal view when the parent view changes size
            // (e.g. rotation). UIViewAutoresizingFlexibleWidth | UIViewAutoresizingFlexibleHeight
            let _: () = msg_send![view, setAutoresizingMask: 18_usize]; // 0x02 | 0x10

            // Enable user interaction on the Metal view for touch handling
            let _: () = msg_send![view, setUserInteractionEnabled: true];
            let _: () = msg_send![view, setMultipleTouchEnabled: true];

            // Set the view as the view controller's view
            let _: () = msg_send![view_controller, setView: view];

            // Set the root view controller
            let _: () = msg_send![window, setRootViewController: view_controller];

            // Make the window visible
            let _: () = msg_send![window, makeKeyAndVisible];

            // Create a hidden text input view for keyboard handling.
            // Uses our custom GPUITextInputView which implements UIKeyInput
            // so iOS actually routes keyboard text to us.
            let text_input_class = register_text_input_view_class();
            let text_input_view: *mut AnyObject = msg_send![text_input_class, alloc];
            let text_input_frame = ObjcCGRect::new(0.0, 0.0, 1.0, 1.0);
            let text_input_view: *mut AnyObject =
                msg_send![text_input_view, initWithFrame: text_input_frame];
            let _: () = msg_send![text_input_view, setAlpha: 0.01_f64];
            let _: () = msg_send![text_input_view, setUserInteractionEnabled: true];
            let _: () = msg_send![view, addSubview: text_input_view];

            let pixel_w = (screen_bounds_cg.width * scale) as i32;
            let pixel_h = (screen_bounds_cg.height * scale) as i32;
            let mut renderer = MetalRenderer::from_layer(
                MetalContext::default(),
                layer.cast::<metal::CAMetalLayer>(),
                false,
            );
            renderer.update_drawable_size(size(DevicePixels(pixel_w), DevicePixels(pixel_h)));

            let ios_window = Self {
                window,
                view_controller,
                view,
                text_input_view,
                bounds: Cell::new(screen_bounds),
                scale_factor: Cell::new(scale_factor),
                input_handler: RefCell::new(None),
                request_frame_callback: RefCell::new(None),
                force_next_frame: Cell::new(true),
                input_callback: RefCell::new(None),
                active_status_callback: RefCell::new(None),
                hover_status_callback: RefCell::new(None),
                resize_callback: RefCell::new(None),
                moved_callback: RefCell::new(None),
                should_close_callback: RefCell::new(None),
                hit_test_callback: RefCell::new(None),
                close_callback: RefCell::new(None),
                appearance_changed_callback: RefCell::new(None),
                insets_changed_callback: RefCell::new(None),
                keyboard_height: Cell::new(0.),
                mouse_position: Cell::new(Point::default()),
                modifiers: Cell::new(Modifiers::default()),
                renderer: Mutex::new(renderer),
            };

            Ok(ios_window)
        }
    }

    /// Register this window with the FFI layer after it's been stored.
    /// This must be called after the window is placed at a stable address
    /// (e.g., in a Box or Arc).
    pub(crate) fn register_with_ffi(&self) {
        super::ffi::register_window(self as *const Self);

        // Set the window pointer on the view so touch events can find us,
        // and on the text input view so keyboard input can find us.
        unsafe {
            let window_ptr = self as *const Self as *mut std::ffi::c_void;
            #[allow(deprecated)]
            {
                *(*self.view).get_mut_ivar::<*mut c_void>(GPUI_WINDOW_IVAR) = window_ptr;
            }
            #[allow(deprecated)]
            {
                *(*self.text_input_view).get_mut_ivar::<*mut c_void>(GPUI_WINDOW_IVAR) = window_ptr;
            }
            log::info!(
                "GPUI iOS: Set window pointer {:p} on view {:p} and text input {:p}",
                window_ptr,
                self.view,
                self.text_input_view
            );
        }

        Self::register_keyboard_observers();
    }

    fn register_keyboard_observers() {
        KEYBOARD_OBSERVERS_REGISTERED.call_once(|| unsafe {
            let notification_center: *mut AnyObject =
                msg_send![class!(NSNotificationCenter), defaultCenter];
            let frame_change_name =
                crate::ios::util::nsstring("UIKeyboardWillChangeFrameNotification");
            let hide_name = crate::ios::util::nsstring("UIKeyboardWillHideNotification");

            let frame_change_block = block2::RcBlock::new(move |notification: *mut AnyObject| {
                if notification.is_null() {
                    return;
                }
                let user_info: *mut AnyObject = msg_send![notification, userInfo];
                if user_info.is_null() {
                    return;
                }
                let frame_key = crate::ios::util::nsstring("UIKeyboardFrameEndUserInfoKey");
                let frame_value: *mut AnyObject = msg_send![user_info, objectForKey: frame_key];
                if frame_value.is_null() {
                    return;
                }
                let frame: ObjcCGRect = msg_send![frame_value, CGRectValue];
                if let Some(wrapper) = super::ffi::IOS_WINDOW_LIST.get() {
                    for &window in &*wrapper.0.get() {
                        if let Some(window) = window.as_ref() {
                            window.set_keyboard_height(frame.height as f32);
                        }
                    }
                }
            });

            let hide_block = block2::RcBlock::new(move |_notification: *mut AnyObject| {
                if let Some(wrapper) = super::ffi::IOS_WINDOW_LIST.get() {
                    for &window in &*wrapper.0.get() {
                        if let Some(window) = window.as_ref() {
                            window.set_keyboard_height(0.);
                        }
                    }
                }
            });

            let _: *mut AnyObject = msg_send![notification_center,
                addObserverForName: frame_change_name,
                object: std::ptr::null::<AnyObject>(),
                queue: std::ptr::null::<AnyObject>(),
                usingBlock: &*frame_change_block
            ];
            let _: *mut AnyObject = msg_send![notification_center,
                addObserverForName: hide_name,
                object: std::ptr::null::<AnyObject>(),
                queue: std::ptr::null::<AnyObject>(),
                usingBlock: &*hide_block
            ];
        });
    }

    /// Delivers a UIKit touch through GPUI's platform-neutral touch API.
    pub fn handle_touch(&self, touch: *mut AnyObject, _event: *mut AnyObject) {
        let position = touch_location_in_view(touch, self.view);
        self.mouse_position.set(position);

        let event = TouchEvent {
            id: touch_id(touch),
            phase: touch_phase(touch).into(),
            position,
            force: touch_force(touch),
        };
        if let Some(callback) = self.input_callback.borrow_mut().as_mut() {
            callback(PlatformInput::Touch(event));
        }
    }

    pub(super) fn request_frame(&self) {
        let callback = self.request_frame_callback.borrow_mut().take();
        if let Some(mut callback) = callback {
            let force_render = self.force_next_frame.replace(false);
            callback(RequestFrameOptions {
                force_render,
                ..Default::default()
            });
            let mut callback_slot = self.request_frame_callback.borrow_mut();
            if callback_slot.is_none() {
                *callback_slot = Some(callback);
            }
        }
    }

    /// Query the safe area insets from the UIView.
    ///
    /// Returns `(top, bottom, left, right)` in logical points.
    /// These represent the areas occupied by system UI (status bar,
    /// home indicator, camera notch) that content should avoid.
    fn safe_area_insets(&self) -> (f32, f32, f32, f32) {
        if self.view.is_null() {
            return (0.0, 0.0, 0.0, 0.0);
        }
        unsafe {
            // UIEdgeInsets { top, left, bottom, right } — all CGFloat
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            struct UIEdgeInsets {
                top: f64,
                left: f64,
                bottom: f64,
                right: f64,
            }

            unsafe impl Encode for UIEdgeInsets {
                const ENCODING: Encoding = Encoding::Struct(
                    "UIEdgeInsets",
                    &[
                        Encoding::Double,
                        Encoding::Double,
                        Encoding::Double,
                        Encoding::Double,
                    ],
                );
            }

            unsafe impl RefEncode for UIEdgeInsets {
                const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
            }

            let insets: UIEdgeInsets = msg_send![self.view, safeAreaInsets];
            (
                insets.top as f32,
                insets.bottom as f32,
                insets.left as f32,
                insets.right as f32,
            )
        }
    }

    fn current_insets(&self) -> WindowInsets {
        let (top, bottom, left, right) = self.safe_area_insets();
        WindowInsets {
            safe_area: Edges {
                top: px(top),
                right: px(right),
                bottom: px(bottom),
                left: px(left),
            },
            ime: Edges {
                bottom: px(self.keyboard_height.get()),
                ..Default::default()
            },
        }
    }

    fn notify_insets_changed(&self) {
        if let Some(callback) = self.insets_changed_callback.borrow_mut().as_mut() {
            callback(self.current_insets());
        }
    }

    fn set_keyboard_height(&self, height: f32) {
        let height = height.max(0.);
        if (self.keyboard_height.get() - height).abs() <= 0.5 {
            return;
        }
        self.keyboard_height.set(height);
        self.notify_insets_changed();
    }

    /// Defers the UIKit responder transition to avoid synchronous layout callbacks
    /// re-entering GPUI while an input event is being dispatched.
    pub fn show_keyboard(&self) {
        unsafe {
            if self.text_input_view.is_null() {
                log::error!("GPUI iOS: Text input view is unavailable");
                return;
            }
            let _: () = msg_send![self.text_input_view, setKeyboardType: 0_isize];
            let _: () = msg_send![self.text_input_view, setAutocorrectionType: 1_isize];
            let _: () = msg_send![self.text_input_view, setAutocapitalizationType: 0_isize];
            let _: () = msg_send![self.text_input_view,
                performSelector: sel!(becomeFirstResponder),
                withObject: ptr::null::<AnyObject>(),
                afterDelay: 0.0_f64
            ];
        }
    }

    pub fn hide_keyboard(&self) {
        unsafe {
            let _: () = msg_send![self.text_input_view,
                performSelector: sel!(resignFirstResponder),
                withObject: ptr::null::<AnyObject>(),
                afterDelay: 0.0_f64
            ];
        }
    }

    pub fn handle_text_input(&self, text: *mut AnyObject) {
        if text.is_null() {
            return;
        }

        unsafe {
            let utf8: *const i8 = msg_send![text, UTF8String];
            if utf8.is_null() {
                return;
            }

            let text_str = std::ffi::CStr::from_ptr(utf8)
                .to_string_lossy()
                .into_owned();

            if let Some(handler) = self.input_handler.borrow_mut().as_mut() {
                handler.replace_text_in_range(None, &text_str);
                return;
            }

            for character in text_str.chars() {
                let keystroke = gpui::Keystroke {
                    modifiers: Modifiers::default(),
                    key: character.to_string(),
                    key_char: Some(character.to_string()),
                };

                let event = PlatformInput::KeyDown(gpui::KeyDownEvent {
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

    pub fn handle_delete_backward(&self) {
        let keystroke = gpui::Keystroke {
            modifiers: Modifiers::default(),
            key: "backspace".to_string(),
            key_char: None,
        };
        let event = PlatformInput::KeyDown(gpui::KeyDownEvent {
            keystroke,
            is_held: false,
            prefer_character_input: false,
        });
        if let Some(callback) = self.input_callback.borrow_mut().as_mut() {
            callback(event);
        }
    }

    pub fn handle_key_event(&self, key_code: u32, modifier_flags: u32, is_key_down: bool) {
        use super::text_input::{key_code_to_key_down, key_code_to_key_up};

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

    pub fn handle_layout_change(&self) {
        unsafe {
            let view_bounds: ObjcCGRect = msg_send![self.view, bounds];
            let screen: *mut AnyObject = msg_send![class!(UIScreen), mainScreen];
            let scale: core_graphics::base::CGFloat = msg_send![screen, scale];

            let new_w = view_bounds.width as f32;
            let new_h = view_bounds.height as f32;
            let new_scale = scale as f32;

            let old_bounds = self.bounds.get();
            let old_scale = self.scale_factor.get();

            let new_size = size(px(new_w), px(new_h));
            self.notify_insets_changed();

            if old_bounds.size == new_size && (old_scale - new_scale).abs() < 0.01 {
                return;
            }

            log::info!(
                "GPUI iOS: Layout changed — {:?} @{:.1}x → {:?} @{:.1}x",
                old_bounds.size,
                old_scale,
                new_size,
                new_scale,
            );

            // Update stored bounds (in logical pixels, matching GPUI convention).
            let new_bounds = Bounds {
                origin: Default::default(),
                size: new_size,
            };
            self.bounds.set(new_bounds);
            self.scale_factor.set(new_scale);

            // Update the Metal layer's contentsScale so the drawable has the
            // correct pixel dimensions.
            let layer: *mut AnyObject = msg_send![self.view, layer];
            let _: () = msg_send![layer, setContentsScale: scale];

            let pixel_w = (new_w * new_scale) as i32;
            let pixel_h = (new_h * new_scale) as i32;
            self.renderer
                .lock()
                .update_drawable_size(size(DevicePixels(pixel_w), DevicePixels(pixel_h)));

            // Fire the resize callback so GPUI re-layouts at the new size.
            let cb = self.resize_callback.borrow_mut().take();
            if let Some(mut cb) = cb {
                cb(new_size, new_scale);
                // Restore the callback for future resize events.
                let mut slot = self.resize_callback.borrow_mut();
                if slot.is_none() {
                    *slot = Some(cb);
                }
            }
        }
    }
}

impl Drop for IosWindow {
    fn drop(&mut self) {
        super::ffi::unregister_window(self);

        unsafe {
            #[allow(deprecated)]
            {
                *(*self.view).get_mut_ivar::<*mut c_void>(GPUI_WINDOW_IVAR) = ptr::null_mut();
                *(*self.text_input_view).get_mut_ivar::<*mut c_void>(GPUI_WINDOW_IVAR) =
                    ptr::null_mut();
            }
            let _: () = msg_send![self.text_input_view, removeFromSuperview];
            let _: () = msg_send![self.text_input_view, release];
            let _: () = msg_send![self.view, release];
            let _: () = msg_send![self.view_controller, release];
            let _: () = msg_send![self.window, release];
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
            let trait_collection: *mut AnyObject = msg_send![self.view, traitCollection];
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

    fn capslock(&self) -> Capslock {
        // Would need to check UIKeyModifierFlags
        Capslock { on: false }
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
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {
        unsafe {
            let _: () = msg_send![self.window, makeKeyAndVisible];
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            let app: *mut AnyObject = msg_send![class!(UIApplication), sharedApplication];
            let key_window: *mut AnyObject = msg_send![app, keyWindow];
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

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
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
        self.renderer.lock().draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.renderer.lock().sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {
        // iOS handles IME positioning automatically
    }

    fn insets(&self) -> WindowInsets {
        self.current_insets()
    }

    fn on_insets_changed(&self, callback: Box<dyn FnMut(WindowInsets)>) {
        *self.insets_changed_callback.borrow_mut() = Some(callback);
    }

    fn show_soft_keyboard(&self) {
        self.show_keyboard();
    }

    fn hide_soft_keyboard(&self) {
        self.hide_keyboard();
    }

    fn text_input_state_changed(&self, change: TextInputStateChange) {
        match change {
            TextInputStateChange::FocusGained => self.show_keyboard(),
            TextInputStateChange::FocusLost => self.hide_keyboard(),
            TextInputStateChange::SelectionChanged | TextInputStateChange::ContentChanged => unsafe {
                let _: () = msg_send![self.text_input_view, reloadInputViews];
            },
        }
    }
}
