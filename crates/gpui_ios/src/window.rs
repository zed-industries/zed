use crate::{CGFloat, CGPoint, CGRect, CGSize, IosDisplay, id, nil, text_input};
use futures::channel::oneshot;
use gpui::{
    Bounds, Capslock, DispatchEventResult, GpuSpecs, KeyDownEvent, KeyUpEvent, Keystroke,
    Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, PinchEvent, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions,
    Scene, ScrollDelta, ScrollWheelEvent, Size, TouchPhase, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, point, px, size,
};
use gpui_apple::metal_renderer::{self, Renderer};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{BOOL, Class, NO, Object, Sel, YES},
    sel, sel_impl,
};
use raw_window_handle as rwh;
use std::{
    cell::RefCell,
    ffi::{CStr, c_void},
    mem,
    os::raw::c_char,
    ptr,
    rc::Rc,
    sync::{Arc, Once},
    time::{Duration, Instant},
};

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSDefaultRunLoopMode: id;
}

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    static UIApplicationWillResignActiveNotification: id;
    static UIApplicationDidBecomeActiveNotification: id;
    static UIKeyboardWillShowNotification: id;
    static UIKeyboardWillHideNotification: id;
    static UIKeyboardFrameEndUserInfoKey: id;
}

pub(crate) const WINDOW_STATE_IVAR: &str = "windowState";

const UI_GESTURE_RECOGNIZER_STATE_BEGAN: i64 = 1;
const UI_GESTURE_RECOGNIZER_STATE_CHANGED: i64 = 2;
const UI_GESTURE_RECOGNIZER_STATE_ENDED: i64 = 3;
const UI_GESTURE_RECOGNIZER_STATE_CANCELLED: i64 = 4;
const UI_GESTURE_RECOGNIZER_STATE_FAILED: i64 = 5;

// `UIKeyModifierFlags` bit masks.
const UI_KEY_MODIFIER_ALPHA_SHIFT: i64 = 1 << 16;
const UI_KEY_MODIFIER_SHIFT: i64 = 1 << 17;
const UI_KEY_MODIFIER_CONTROL: i64 = 1 << 18;
const UI_KEY_MODIFIER_ALTERNATE: i64 = 1 << 19;
const UI_KEY_MODIFIER_COMMAND: i64 = 1 << 20;

// `UIKeyboardHIDUsage` values (USB HID keyboard usage page).
const HID_USAGE_KEYBOARD_RETURN_OR_ENTER: i64 = 0x28;
const HID_USAGE_KEYBOARD_ESCAPE: i64 = 0x29;
const HID_USAGE_KEYBOARD_DELETE_OR_BACKSPACE: i64 = 0x2A;
const HID_USAGE_KEYBOARD_TAB: i64 = 0x2B;
const HID_USAGE_KEYBOARD_SPACEBAR: i64 = 0x2C;
const HID_USAGE_KEYBOARD_F1: i64 = 0x3A;
const HID_USAGE_KEYBOARD_F12: i64 = 0x45;
const HID_USAGE_KEYBOARD_HOME: i64 = 0x4A;
const HID_USAGE_KEYBOARD_PAGE_UP: i64 = 0x4B;
const HID_USAGE_KEYBOARD_DELETE_FORWARD: i64 = 0x4C;
const HID_USAGE_KEYBOARD_END: i64 = 0x4D;
const HID_USAGE_KEYBOARD_PAGE_DOWN: i64 = 0x4E;
const HID_USAGE_KEYBOARD_RIGHT_ARROW: i64 = 0x4F;
const HID_USAGE_KEYBOARD_LEFT_ARROW: i64 = 0x50;
const HID_USAGE_KEYBOARD_DOWN_ARROW: i64 = 0x51;
const HID_USAGE_KEYBOARD_UP_ARROW: i64 = 0x52;

/// `UIScrollView.DecelerationRate.normal`: a decelerating scroll's velocity
/// multiplies by 0.998 for every elapsed millisecond. Applied per tick as
/// `0.998^dt_ms` so 60Hz and 120Hz displays follow the same curve.
const MOMENTUM_DECAY_PER_MILLISECOND: f32 = 0.998;

/// Below this speed (points per second) a decelerating scroll moves less
/// than a physical pixel per frame, so it stops instead of trickling on.
const MOMENTUM_MINIMUM_SPEED: f32 = 10.;

/// Dragging, holding still, then lifting must stop the scroll dead, like
/// UIScrollView. `velocityInView:` can't be trusted for this: it reports the
/// velocity of the most recent movement samples, and a stationary finger
/// stops producing samples, so the pre-hold fling velocity survives to the
/// release. Instead, a release this long after the last translation change
/// starts no momentum.
const PAN_HOLD_SUPPRESSES_MOMENTUM: Duration = Duration::from_millis(100);

/// A synthesized deceleration continuing a finished pan gesture, stepped on
/// each display-link tick.
struct ScrollMomentum {
    /// Points per second, in the pan translation's sign convention.
    velocity: Point<f32>,
    /// Where the finger lifted; momentum events keep scrolling whatever is
    /// under that point.
    position: Point<Pixels>,
    last_tick: Instant,
}

pub(crate) struct IosWindowState {
    native_window: id,
    view_controller: id,
    native_view: id,
    text_input_view: id,
    display_link: id,
    display_link_target: id,
    renderer: Renderer,
    bounds: Bounds<Pixels>,
    /// The full screen size; `bounds` shrinks below it while the software
    /// keyboard is up (see `apply_keyboard_overlap`).
    screen_size: Size<Pixels>,
    /// How much of the window the software keyboard currently covers.
    keyboard_overlap: Pixels,
    /// Whether the text-input view was made first responder by the last
    /// responder poll (see `update_text_input_responder`).
    text_input_is_first_responder: bool,
    scale_factor: f32,
    request_frame_callback: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    input_callback: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    input_handler: Option<PlatformInputHandler>,
    mouse_position: Point<Pixels>,
    /// Last modifier state read off a hardware key press. UIKit has no
    /// flags-changed callback, so this is only as fresh as the most recent
    /// `presses*` event.
    modifiers: Modifiers,
    capslock: Capslock,
    is_active: bool,
    scroll_momentum: Option<ScrollMomentum>,
    /// When the active pan gesture's translation last changed; consulted at
    /// release to distinguish a flick from a drag-hold-release.
    pan_last_moved_at: Option<Instant>,
    /// The `UITouch` the pointer shim is following, compared by identity
    /// (UIKit keeps a touch's object stable for its whole lifetime).
    active_touch: Option<id>,
}

pub(crate) struct IosWindow(Rc<RefCell<IosWindowState>>);

impl IosWindow {
    /// Creates a full-screen `UIWindow` and drives frames with a
    /// `CADisplayLink` on the main run loop. Must be called on the main
    /// thread, after `application:didFinishLaunchingWithOptions:`.
    pub(crate) fn open(renderer_context: metal_renderer::Context) -> Self {
        unsafe {
            let screen: id = msg_send![class!(UIScreen), mainScreen];
            let screen_bounds: CGRect = msg_send![screen, bounds];
            let scale_factor: CGFloat = msg_send![screen, scale];
            let scale_factor = scale_factor as f32;

            let native_window: id = msg_send![class!(UIWindow), alloc];
            let native_window: id = msg_send![native_window, initWithFrame: screen_bounds];

            // UIKit requires a root view controller on every visible window.
            let view_controller: id = msg_send![class!(UIViewController), new];

            let native_view: id = msg_send![gpui_view_class(), alloc];
            let native_view: id = msg_send![native_view, initWithFrame: screen_bounds];
            // The pinch recognizer needs the second finger delivered to the
            // view; the `touches*` overrides preserve the pointer shim's
            // single-touch semantics by tracking only the first touch.
            let _: () = msg_send![native_view, setMultipleTouchEnabled: YES];
            let _: () = msg_send![view_controller, setView: native_view];

            let pan_recognizer: id = msg_send![class!(UIPanGestureRecognizer), alloc];
            let pan_recognizer: id = msg_send![
                pan_recognizer,
                initWithTarget: native_view
                action: sel!(handlePan:)
            ];
            // One finger pans; a second finger belongs to the pinch
            // recognizer. `cancelsTouchesInView` stays at its default YES so
            // recognition sends `touchesCancelled:` — that's the tap/scroll
            // arbitration: sub-slop touches stay taps, movement becomes a
            // scroll and the pending press is released off-window.
            let _: () = msg_send![pan_recognizer, setMaximumNumberOfTouches: 1usize];
            let _: () = msg_send![native_view, addGestureRecognizer: pan_recognizer];
            let _: () = msg_send![pan_recognizer, release];

            let pinch_recognizer: id = msg_send![class!(UIPinchGestureRecognizer), alloc];
            let pinch_recognizer: id = msg_send![
                pinch_recognizer,
                initWithTarget: native_view
                action: sel!(handlePinch:)
            ];
            let _: () = msg_send![native_view, addGestureRecognizer: pinch_recognizer];
            let _: () = msg_send![pinch_recognizer, release];

            let text_input_view: id = msg_send![text_input::text_input_view_class(), alloc];
            let text_input_view: id = msg_send![text_input_view, initWithFrame: CGRect::default()];
            let _: () = msg_send![native_view, addSubview: text_input_view];

            let _: () = msg_send![native_window, setRootViewController: view_controller];
            let _: () = msg_send![native_window, makeKeyAndVisible];
            // Hardware key presses are only delivered along the responder
            // chain, so the view must be first responder to see them.
            let _: BOOL = msg_send![native_view, becomeFirstResponder];

            let bounds = Bounds {
                origin: Point::default(),
                size: size(
                    px(screen_bounds.size.width as f32),
                    px(screen_bounds.size.height as f32),
                ),
            };

            let mut renderer = metal_renderer::new_renderer(
                renderer_context,
                native_window as *mut c_void,
                native_view as *mut c_void,
                bounds.size.map(|pixels| pixels.as_f32()),
                false,
            );

            let metal_layer = renderer.layer_ptr() as id;
            let view_layer: id = msg_send![native_view, layer];
            let _: () = msg_send![metal_layer, setFrame: screen_bounds];
            let _: () = msg_send![metal_layer, setContentsScale: scale_factor as CGFloat];
            let _: () = msg_send![view_layer, addSublayer: metal_layer];
            renderer.update_drawable_size(bounds.size.to_device_pixels(scale_factor));

            let window = Self(Rc::new(RefCell::new(IosWindowState {
                native_window,
                view_controller,
                native_view,
                text_input_view,
                display_link: nil,
                display_link_target: nil,
                renderer,
                bounds,
                screen_size: bounds.size,
                keyboard_overlap: px(0.),
                text_input_is_first_responder: false,
                scale_factor,
                request_frame_callback: None,
                active_status_change_callback: None,
                resize_callback: None,
                input_callback: None,
                input_handler: None,
                mouse_position: Point::default(),
                modifiers: Modifiers::default(),
                capslock: Capslock::default(),
                // The app launches foreground-active; UIKit only notifies on
                // transitions.
                is_active: true,
                scroll_momentum: None,
                pan_last_moved_at: None,
                active_touch: None,
            })));

            // The views and the display-link target each keep a strong `Rc`
            // reference to the window state in an ivar; `Drop` reclaims them.
            (*native_view).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *mut c_void,
            );
            (*text_input_view).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *mut c_void,
            );

            let display_link_target: id = msg_send![display_link_target_class(), new];
            (*display_link_target).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *mut c_void,
            );

            let display_link: id = msg_send![
                class!(CADisplayLink),
                displayLinkWithTarget: display_link_target
                selector: sel!(step:)
            ];
            let run_loop: id = msg_send![class!(NSRunLoop), mainRunLoop];
            let _: () =
                msg_send![display_link, addToRunLoop: run_loop forMode: NSDefaultRunLoopMode];

            let notification_center: id = msg_send![class!(NSNotificationCenter), defaultCenter];
            let _: () = msg_send![
                notification_center,
                addObserver: display_link_target
                selector: sel!(applicationWillResignActive:)
                name: UIApplicationWillResignActiveNotification
                object: nil
            ];
            let _: () = msg_send![
                notification_center,
                addObserver: display_link_target
                selector: sel!(applicationDidBecomeActive:)
                name: UIApplicationDidBecomeActiveNotification
                object: nil
            ];
            let _: () = msg_send![
                notification_center,
                addObserver: display_link_target
                selector: sel!(keyboardWillShow:)
                name: UIKeyboardWillShowNotification
                object: nil
            ];
            let _: () = msg_send![
                notification_center,
                addObserver: display_link_target
                selector: sel!(keyboardWillHide:)
                name: UIKeyboardWillHideNotification
                object: nil
            ];

            {
                let mut state = window.0.borrow_mut();
                state.display_link = display_link;
                state.display_link_target = display_link_target;
            }

            window
        }
    }
}

impl Drop for IosWindow {
    fn drop(&mut self) {
        let (
            display_link,
            display_link_target,
            view_controller,
            native_view,
            text_input_view,
            native_window,
        ) = {
            let state = self.0.borrow();
            (
                state.display_link,
                state.display_link_target,
                state.view_controller,
                state.native_view,
                state.text_input_view,
                state.native_window,
            )
        };
        unsafe {
            let notification_center: id = msg_send![class!(NSNotificationCenter), defaultCenter];
            let _: () = msg_send![notification_center, removeObserver: display_link_target];
            let _: () = msg_send![display_link, invalidate];
            // Reclaim the strong references the display-link target and the
            // views hold so the window state can actually be freed.
            let raw: *mut c_void = *(*display_link_target).get_ivar(WINDOW_STATE_IVAR);
            drop(Rc::from_raw(raw as *const RefCell<IosWindowState>));
            let raw: *mut c_void = *(*native_view).get_ivar(WINDOW_STATE_IVAR);
            drop(Rc::from_raw(raw as *const RefCell<IosWindowState>));
            let raw: *mut c_void = *(*text_input_view).get_ivar(WINDOW_STATE_IVAR);
            drop(Rc::from_raw(raw as *const RefCell<IosWindowState>));
            let _: () = msg_send![display_link_target, release];
            let _: () = msg_send![text_input_view, release];
            let _: () = msg_send![native_view, release];
            let _: () = msg_send![view_controller, release];
            let _: () = msg_send![native_window, release];
        }
    }
}

impl PlatformWindow for IosWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.0.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.0.borrow().bounds)
    }

    fn content_size(&self) -> Size<Pixels> {
        self.0.borrow().bounds.size
    }

    fn resize(&mut self, _size: Size<Pixels>) {}

    fn scale_factor(&self) -> f32 {
        self.0.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(IosDisplay::primary()))
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.0.borrow().mouse_position
    }

    fn modifiers(&self) -> Modifiers {
        self.0.borrow().modifiers
    }

    fn capslock(&self) -> Capslock {
        self.0.borrow().capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.borrow_mut().input_handler.take()
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
        self.0.borrow().is_active
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, _title: &str) {}

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        true
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.0.borrow_mut().request_frame_callback = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.0.borrow_mut().input_callback = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.borrow_mut().active_status_change_callback = Some(callback);
    }

    fn on_hover_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.borrow_mut().resize_callback = Some(callback);
    }

    fn on_moved(&self, _callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_hit_test_window_control(&self, _callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn draw(&self, scene: &Scene) {
        self.0.borrow_mut().renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.0.borrow().renderer.sprite_atlas().clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}
}

impl rwh::HasWindowHandle for IosWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        let native_view = self.0.borrow().native_view;
        let Some(native_view) = ptr::NonNull::new(native_view as *mut c_void) else {
            return Err(rwh::HandleError::Unavailable);
        };
        // SAFETY: The UIView pointer remains valid for the window's lifetime.
        unsafe {
            Ok(rwh::WindowHandle::borrow_raw(rwh::RawWindowHandle::UiKit(
                rwh::UiKitWindowHandle::new(native_view),
            )))
        }
    }
}

impl rwh::HasDisplayHandle for IosWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        Ok(rwh::DisplayHandle::uikit())
    }
}

fn gpui_view_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUIView", class!(UIView))
            .expect("GPUIView class is already registered");
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);
        unsafe {
            decl.add_method(
                sel!(touchesBegan:withEvent:),
                touches_began as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(touchesMoved:withEvent:),
                touches_moved as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(touchesEnded:withEvent:),
                touches_ended as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(touchesCancelled:withEvent:),
                touches_cancelled as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(handlePan:),
                handle_pan as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(handlePinch:),
                handle_pinch as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(canBecomeFirstResponder),
                can_become_first_responder as extern "C" fn(&Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(pressesBegan:withEvent:),
                presses_began as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(pressesEnded:withEvent:),
                presses_ended as extern "C" fn(&Object, Sel, id, id),
            );
            decl.add_method(
                sel!(pressesCancelled:withEvent:),
                presses_cancelled as extern "C" fn(&Object, Sel, id, id),
            );
        }
        decl.register();
    });
    Class::get("GPUIView").expect("GPUIView was just registered")
}

fn display_link_target_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUIDisplayLinkTarget", class!(NSObject))
            .expect("GPUIDisplayLinkTarget class is already registered");
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);
        unsafe {
            decl.add_method(sel!(step:), step as extern "C" fn(&Object, Sel, id));
            decl.add_method(
                sel!(applicationWillResignActive:),
                application_will_resign_active as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(applicationDidBecomeActive:),
                application_did_become_active as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(keyboardWillShow:),
                keyboard_will_show as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(keyboardWillHide:),
                keyboard_will_hide as extern "C" fn(&Object, Sel, id),
            );
        }
        decl.register();
    });
    Class::get("GPUIDisplayLinkTarget").expect("GPUIDisplayLinkTarget was just registered")
}

pub(crate) unsafe fn get_window_state(object: &Object) -> Rc<RefCell<IosWindowState>> {
    unsafe {
        let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
        let state = Rc::from_raw(raw as *const RefCell<IosWindowState>);
        let clone = state.clone();
        mem::forget(state);
        clone
    }
}

extern "C" fn step(this: &Object, _: Sel, _display_link: id) {
    let window_state = unsafe { get_window_state(this) };
    tick_scroll_momentum(&window_state);
    // Don't hold the RefCell borrow across the callback: gpui reenters the
    // window from inside it (e.g. `draw`).
    let callback = window_state.borrow_mut().request_frame_callback.take();
    if let Some(mut callback) = callback {
        callback(RequestFrameOptions::default());
        window_state.borrow_mut().request_frame_callback = Some(callback);
    }
    update_text_input_responder(&window_state);
}

/// Moves first-responder status between the text-input view and `GPUIView`
/// to track gpui focus. gpui has no push signal for "an editable element
/// gained or lost focus", so this polls the input handler after each frame:
/// the handler is (re)installed during `draw`, making post-frame the earliest
/// point the new focus state is observable.
fn update_text_input_responder(window_state: &Rc<RefCell<IosWindowState>>) {
    let input_handler = window_state.borrow_mut().input_handler.take();
    let accepts_text_input = match input_handler {
        Some(mut input_handler) => {
            // Queries gpui synchronously; the RefCell borrow must be released
            // first because gpui may reenter the window.
            let accepts = input_handler.query_accepts_text_input();
            window_state.borrow_mut().input_handler = Some(input_handler);
            accepts
        }
        None => false,
    };
    let (text_input_view, native_view) = {
        let mut state = window_state.borrow_mut();
        if state.text_input_is_first_responder == accepts_text_input {
            return;
        }
        state.text_input_is_first_responder = accepts_text_input;
        (state.text_input_view, state.native_view)
    };
    unsafe {
        if accepts_text_input {
            let _: BOOL = msg_send![text_input_view, becomeFirstResponder];
        } else {
            let _: BOOL = msg_send![text_input_view, resignFirstResponder];
            // Hardware key presses are only delivered along the responder
            // chain, so first-responder status must return to the main view.
            let _: BOOL = msg_send![native_view, becomeFirstResponder];
        }
    }
}

extern "C" fn keyboard_will_show(this: &Object, _: Sel, notification: id) {
    let window_state = unsafe { get_window_state(this) };
    let keyboard_frame: CGRect = unsafe {
        let user_info: id = msg_send![notification, userInfo];
        if user_info.is_null() {
            return;
        }
        let frame_value: id = msg_send![user_info, objectForKey: UIKeyboardFrameEndUserInfoKey];
        if frame_value.is_null() {
            return;
        }
        msg_send![frame_value, CGRectValue]
    };
    // The window is full-screen, so the keyboard's screen-coordinate frame
    // needs no conversion: everything below its top edge is covered.
    let overlap = {
        let state = window_state.borrow();
        (state.screen_size.height - px(keyboard_frame.origin.y as f32)).max(px(0.))
    };
    apply_keyboard_overlap(&window_state, overlap);
}

extern "C" fn keyboard_will_hide(this: &Object, _: Sel, _notification: id) {
    let window_state = unsafe { get_window_state(this) };
    apply_keyboard_overlap(&window_state, px(0.));
}

/// Shrinks the window (bounds, Metal layer, drawable) by the height the
/// software keyboard covers and reports the new size to gpui, so the focused
/// editable relayouts above the keyboard. gpui has no viewport-inset concept
/// a platform could set instead, so keyboard avoidance is expressed as a
/// window resize; hiding the keyboard restores the full screen size.
fn apply_keyboard_overlap(window_state: &Rc<RefCell<IosWindowState>>, overlap: Pixels) {
    let (new_size, scale_factor) = {
        let mut state = window_state.borrow_mut();
        if state.keyboard_overlap == overlap {
            return;
        }
        state.keyboard_overlap = overlap;
        let new_size = size(state.screen_size.width, state.screen_size.height - overlap);
        state.bounds.size = new_size;
        unsafe {
            let metal_layer = state.renderer.layer_ptr() as id;
            let frame = CGRect {
                origin: CGPoint::default(),
                size: CGSize {
                    width: new_size.width.as_f32() as CGFloat,
                    height: new_size.height.as_f32() as CGFloat,
                },
            };
            // Without this, Core Animation would animate the layer-frame
            // change while the drawable size snaps, stretching the content
            // for the transition's duration.
            let _: () = msg_send![class!(CATransaction), begin];
            let _: () = msg_send![class!(CATransaction), setDisableActions: YES];
            let _: () = msg_send![metal_layer, setFrame: frame];
            let _: () = msg_send![class!(CATransaction), commit];
        }
        let scale_factor = state.scale_factor;
        state
            .renderer
            .update_drawable_size(new_size.to_device_pixels(scale_factor));
        (new_size, scale_factor)
    };
    let callback = window_state.borrow_mut().resize_callback.take();
    if let Some(mut callback) = callback {
        callback(new_size, scale_factor);
        window_state.borrow_mut().resize_callback = Some(callback);
    }
}

fn tick_scroll_momentum(window_state: &Rc<RefCell<IosWindowState>>) {
    let event = {
        let mut state = window_state.borrow_mut();
        let Some(momentum) = state.scroll_momentum.as_mut() else {
            return;
        };
        let now = Instant::now();
        let elapsed_seconds = now.duration_since(momentum.last_tick).as_secs_f32();
        momentum.last_tick = now;
        let delta = point(
            px(momentum.velocity.x * elapsed_seconds),
            px(momentum.velocity.y * elapsed_seconds),
        );
        let position = momentum.position;
        let decay = MOMENTUM_DECAY_PER_MILLISECOND.powf(elapsed_seconds * 1000.);
        momentum.velocity.x *= decay;
        momentum.velocity.y *= decay;
        if momentum.velocity.x.hypot(momentum.velocity.y) < MOMENTUM_MINIMUM_SPEED {
            state.scroll_momentum = None;
            ScrollWheelEvent {
                position,
                delta: ScrollDelta::Pixels(Point::default()),
                modifiers: Modifiers::default(),
                touch_phase: TouchPhase::Ended,
            }
        } else {
            ScrollWheelEvent {
                position,
                delta: ScrollDelta::Pixels(delta),
                modifiers: Modifiers::default(),
                touch_phase: TouchPhase::Moved,
            }
        }
    };
    dispatch_input(window_state, PlatformInput::ScrollWheel(event));
}

extern "C" fn application_will_resign_active(this: &Object, _: Sel, _notification: id) {
    let window_state = unsafe { get_window_state(this) };
    let callback = {
        let mut state = window_state.borrow_mut();
        state.is_active = false;
        // Otherwise the pause duration would be applied as one giant
        // momentum step on the first tick after reactivation.
        state.scroll_momentum = None;
        unsafe {
            let _: () = msg_send![state.display_link, setPaused: YES];
        }
        state.active_status_change_callback.take()
    };
    // Invoke only after dropping the borrow: gpui may reenter the window
    // (e.g. to query `is_active`) from inside the callback.
    if let Some(mut callback) = callback {
        callback(false);
        window_state.borrow_mut().active_status_change_callback = Some(callback);
    }
}

extern "C" fn application_did_become_active(this: &Object, _: Sel, _notification: id) {
    let window_state = unsafe { get_window_state(this) };
    let callback = {
        let mut state = window_state.borrow_mut();
        state.is_active = true;
        unsafe {
            let _: () = msg_send![state.display_link, setPaused: NO];
        }
        state.active_status_change_callback.take()
    };
    // Invoke only after dropping the borrow: gpui may reenter the window
    // (e.g. to query `is_active`) from inside the callback.
    if let Some(mut callback) = callback {
        callback(true);
        window_state.borrow_mut().active_status_change_callback = Some(callback);
    }
}

/// Reads a touch's location and tap count. UIKit's `locationInView:`
/// coordinates are in points, which map 1:1 onto gpui's logical pixels.
unsafe fn touch_position_and_tap_count(view: &Object, touch: id) -> (Point<Pixels>, usize) {
    unsafe {
        let location: CGPoint = msg_send![touch, locationInView: view as *const Object as id];
        let tap_count: usize = msg_send![touch, tapCount];
        (
            point(px(location.x as f32), px(location.y as f32)),
            tap_count,
        )
    }
}

/// Returns the window's tracked touch if it's a member of `touches`, so a
/// `touches*` override only reacts to the finger the pointer shim follows.
fn tracked_touch_in_set(window_state: &Rc<RefCell<IosWindowState>>, touches: id) -> Option<id> {
    let tracked_touch = window_state.borrow().active_touch?;
    let is_member: bool = unsafe {
        let contains: objc::runtime::BOOL = msg_send![touches, containsObject: tracked_touch];
        contains == YES
    };
    is_member.then_some(tracked_touch)
}

/// Runs a closure against the window's input handler with the handler taken
/// out of the state and the `RefCell` borrow released: every handler method
/// calls into gpui synchronously, which may reenter the window. Returns
/// `None` when no input handler is installed (no editable element focused).
pub(crate) fn with_input_handler<R>(
    object: &Object,
    f: impl FnOnce(&mut PlatformInputHandler) -> R,
) -> Option<R> {
    let window_state = unsafe { get_window_state(object) };
    let input_handler = window_state.borrow_mut().input_handler.take();
    let mut input_handler = input_handler?;
    let result = f(&mut input_handler);
    window_state.borrow_mut().input_handler = Some(input_handler);
    Some(result)
}

/// Invokes the gpui input callback with the `RefCell` borrow released: gpui
/// may reenter the window (e.g. to read `mouse_position` or request a frame)
/// while handling the event. Without a registered callback the event is
/// reported as propagating, so it falls through to UIKit's default handling.
pub(crate) fn dispatch_input(
    window_state: &Rc<RefCell<IosWindowState>>,
    input: PlatformInput,
) -> DispatchEventResult {
    let callback = window_state.borrow_mut().input_callback.take();
    let Some(mut callback) = callback else {
        return DispatchEventResult {
            propagate: true,
            default_prevented: false,
        };
    };
    let result = callback(input);
    window_state.borrow_mut().input_callback = Some(callback);
    result
}

/// Parks the pointer just outside the window once the finger lifts. A touch
/// has no persistent pointer, but gpui recomputes hover from the last
/// mouse-move position, so without this the last-touched element would stay
/// hovered forever. `MouseExited` alone is not enough—it doesn't relocate
/// gpui's pointer—hence the synthetic off-window move before it.
fn clear_hover(window_state: &Rc<RefCell<IosWindowState>>) {
    let off_window_position = point(px(-1.), px(-1.));
    dispatch_input(
        window_state,
        PlatformInput::MouseMove(MouseMoveEvent {
            position: off_window_position,
            pressed_button: None,
            modifiers: Modifiers::default(),
        }),
    );
    dispatch_input(
        window_state,
        PlatformInput::MouseExited(MouseExitEvent {
            position: off_window_position,
            pressed_button: None,
            modifiers: Modifiers::default(),
        }),
    );
}

extern "C" fn touches_began(this: &Object, _: Sel, touches: id, _event: id) {
    let window_state = unsafe { get_window_state(this) };
    {
        let mut state = window_state.borrow_mut();
        // A finger landing anywhere catches a decelerating scroll, like
        // UIScrollView. Killed silently: the touch's own events supersede
        // the scroll, so no final Ended scroll event is needed.
        state.scroll_momentum = None;
        if state.active_touch.is_some() {
            return;
        }
    }
    let touch: id = unsafe { msg_send![touches, anyObject] };
    let (position, tap_count) = unsafe { touch_position_and_tap_count(this, touch) };
    {
        let mut state = window_state.borrow_mut();
        state.active_touch = Some(touch);
        state.mouse_position = position;
    }
    // A touch has no hover phase, so this move is gpui's only chance to
    // learn the pointer location before the press lands. The hover styling
    // it triggers while the finger is down reads as a pressed-state
    // highlight.
    dispatch_input(
        &window_state,
        PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: None,
            modifiers: Modifiers::default(),
        }),
    );
    dispatch_input(
        &window_state,
        PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position,
            modifiers: Modifiers::default(),
            click_count: tap_count,
            first_mouse: false,
        }),
    );
}

extern "C" fn touches_moved(this: &Object, _: Sel, touches: id, _event: id) {
    let window_state = unsafe { get_window_state(this) };
    let Some(touch) = tracked_touch_in_set(&window_state, touches) else {
        return;
    };
    let (position, _) = unsafe { touch_position_and_tap_count(this, touch) };
    window_state.borrow_mut().mouse_position = position;
    dispatch_input(
        &window_state,
        PlatformInput::MouseMove(MouseMoveEvent {
            position,
            // gpui models an in-progress drag as a move with the button held.
            pressed_button: Some(MouseButton::Left),
            modifiers: Modifiers::default(),
        }),
    );
}

extern "C" fn touches_ended(this: &Object, _: Sel, touches: id, _event: id) {
    let window_state = unsafe { get_window_state(this) };
    let Some(touch) = tracked_touch_in_set(&window_state, touches) else {
        return;
    };
    let (position, tap_count) = unsafe { touch_position_and_tap_count(this, touch) };
    {
        let mut state = window_state.borrow_mut();
        state.active_touch = None;
        state.mouse_position = position;
    }
    dispatch_input(
        &window_state,
        PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers: Modifiers::default(),
            click_count: tap_count,
        }),
    );
    clear_hover(&window_state);
}

extern "C" fn touches_cancelled(this: &Object, _: Sel, touches: id, _event: id) {
    let window_state = unsafe { get_window_state(this) };
    if tracked_touch_in_set(&window_state, touches).is_none() {
        return;
    }
    window_state.borrow_mut().active_touch = None;
    // UIKit cancels a touch when something else claims it (a gesture
    // recognizer or a system gesture), so the press must not complete as a
    // click. gpui fires click listeners when a `MouseUp` hit-tests to the
    // element holding the pending `MouseDown` (the window re-runs the hit
    // test at each event's own position), so releasing at the off-window
    // park position discards the pending mouse-down instead of clicking
    // whatever is still under the finger.
    let park_position = point(px(-1.), px(-1.));
    window_state.borrow_mut().mouse_position = park_position;
    dispatch_input(
        &window_state,
        PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position: park_position,
            modifiers: Modifiers::default(),
            click_count: 1,
        }),
    );
    clear_hover(&window_state);
}

extern "C" fn handle_pan(this: &Object, _: Sel, recognizer: id) {
    let window_state = unsafe { get_window_state(this) };
    let view = this as *const Object as id;
    let recognizer_state: i64 = unsafe { msg_send![recognizer, state] };
    let location: CGPoint = unsafe { msg_send![recognizer, locationInView: view] };
    let position = point(px(location.x as f32), px(location.y as f32));

    match recognizer_state {
        UI_GESTURE_RECOGNIZER_STATE_BEGAN => {
            {
                let mut state = window_state.borrow_mut();
                state.scroll_momentum = None;
                state.pan_last_moved_at = Some(Instant::now());
                state.mouse_position = position;
            }
            dispatch_input(
                &window_state,
                PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position,
                    delta: ScrollDelta::Pixels(Point::default()),
                    modifiers: Modifiers::default(),
                    touch_phase: TouchPhase::Started,
                }),
            );
        }
        UI_GESTURE_RECOGNIZER_STATE_CHANGED => {
            let translation: CGPoint = unsafe { msg_send![recognizer, translationInView: view] };
            // Resetting after each read turns the cumulative translation
            // into a per-event delta.
            let _: () =
                unsafe { msg_send![recognizer, setTranslation: CGPoint::default() inView: view] };
            {
                let mut state = window_state.borrow_mut();
                if translation.x != 0. || translation.y != 0. {
                    state.pan_last_moved_at = Some(Instant::now());
                }
                state.mouse_position = position;
            }
            // UIKit's pan translation grows downward/rightward with the
            // finger, and a positive gpui scroll delta also moves content
            // down/right (matching macOS natural scrolling, where dragging
            // two fingers down produces positive `scrollingDeltaY`), so the
            // sign passes through unchanged and content follows the finger.
            dispatch_input(
                &window_state,
                PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position,
                    delta: ScrollDelta::Pixels(point(
                        px(translation.x as f32),
                        px(translation.y as f32),
                    )),
                    modifiers: Modifiers::default(),
                    touch_phase: TouchPhase::Moved,
                }),
            );
        }
        UI_GESTURE_RECOGNIZER_STATE_ENDED
        | UI_GESTURE_RECOGNIZER_STATE_CANCELLED
        | UI_GESTURE_RECOGNIZER_STATE_FAILED => {
            dispatch_input(
                &window_state,
                PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position,
                    delta: ScrollDelta::Pixels(Point::default()),
                    modifiers: Modifiers::default(),
                    touch_phase: TouchPhase::Ended,
                }),
            );
            let pan_last_moved_at = window_state.borrow_mut().pan_last_moved_at.take();
            if recognizer_state == UI_GESTURE_RECOGNIZER_STATE_ENDED
                && pan_last_moved_at.is_some_and(|last_moved_at| {
                    last_moved_at.elapsed() < PAN_HOLD_SUPPRESSES_MOMENTUM
                })
            {
                let velocity: CGPoint = unsafe { msg_send![recognizer, velocityInView: view] };
                let velocity = point(velocity.x as f32, velocity.y as f32);
                if velocity.x.hypot(velocity.y) >= MOMENTUM_MINIMUM_SPEED {
                    window_state.borrow_mut().scroll_momentum = Some(ScrollMomentum {
                        velocity,
                        position,
                        last_tick: Instant::now(),
                    });
                }
            }
        }
        _ => {}
    }
}

extern "C" fn can_become_first_responder(_this: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn presses_began(this: &Object, _: Sel, presses: id, event: id) {
    if handle_presses(this, presses, true) {
        unsafe {
            let _: () = msg_send![super(this, class!(UIView)), pressesBegan: presses
                                                                  withEvent: event];
        }
    }
}

extern "C" fn presses_ended(this: &Object, _: Sel, presses: id, event: id) {
    if handle_presses(this, presses, false) {
        unsafe {
            let _: () = msg_send![super(this, class!(UIView)), pressesEnded: presses
                                                                  withEvent: event];
        }
    }
}

extern "C" fn presses_cancelled(this: &Object, _: Sel, presses: id, event: id) {
    if handle_presses(this, presses, false) {
        unsafe {
            let _: () = msg_send![super(this, class!(UIView)), pressesCancelled: presses
                                                                      withEvent: event];
        }
    }
}

/// Translates hardware key presses into gpui key events and reports whether
/// any press should be forwarded to `UIView`'s own implementation. Presses
/// gpui doesn't handle must reach the superclass so UIKit's text-input
/// machinery can consume them (which software-keyboard/IME support relies
/// on), as must presses with no `key` object (non-keyboard presses).
fn handle_presses(this: &Object, presses: id, is_key_down: bool) -> bool {
    let window_state = unsafe { get_window_state(this) };
    let mut forward_to_super = false;
    let press_array: id = unsafe { msg_send![presses, allObjects] };
    let press_count: usize = unsafe { msg_send![press_array, count] };
    for index in 0..press_count {
        let press: id = unsafe { msg_send![press_array, objectAtIndex: index] };
        let ui_key: id = unsafe { msg_send![press, key] };
        if ui_key.is_null() {
            forward_to_super = true;
            continue;
        }

        let modifier_flags: i64 = unsafe { msg_send![ui_key, modifierFlags] };
        let modifiers = Modifiers {
            control: modifier_flags & UI_KEY_MODIFIER_CONTROL != 0,
            alt: modifier_flags & UI_KEY_MODIFIER_ALTERNATE != 0,
            shift: modifier_flags & UI_KEY_MODIFIER_SHIFT != 0,
            platform: modifier_flags & UI_KEY_MODIFIER_COMMAND != 0,
            function: false,
        };
        let capslock = Capslock {
            on: modifier_flags & UI_KEY_MODIFIER_ALPHA_SHIFT != 0,
        };

        // UIKit has no flags-changed callback; modifier keys arrive as
        // ordinary presses and every press carries the full modifier state,
        // so ModifiersChanged is synthesized from state differences here.
        let modifiers_changed = {
            let mut state = window_state.borrow_mut();
            let changed = state.modifiers != modifiers || state.capslock != capslock;
            state.modifiers = modifiers;
            state.capslock = capslock;
            changed
        };
        if modifiers_changed {
            dispatch_input(
                &window_state,
                PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                    modifiers,
                    capslock,
                }),
            );
        }

        let Some(keystroke) = (unsafe { keystroke_for_ui_key(ui_key, modifiers) }) else {
            // A modifier-only press; the ModifiersChanged above covers it.
            forward_to_super = true;
            continue;
        };
        let input = if is_key_down {
            PlatformInput::KeyDown(KeyDownEvent {
                keystroke,
                is_held: false,
                prefer_character_input: false,
            })
        } else {
            PlatformInput::KeyUp(KeyUpEvent { keystroke })
        };
        if dispatch_input(&window_state, input).propagate {
            forward_to_super = true;
        }
    }
    forward_to_super
}

/// Builds a gpui keystroke from a `UIKey`, following the same conventions as
/// the macOS backend: special keys get gpui's cross-platform names, other
/// keys use their unmodified character, and `key_char` carries the text the
/// press would insert—withheld when command or control is held because those
/// chords never insert text. Returns `None` for modifier-only presses, which
/// produce no characters.
unsafe fn keystroke_for_ui_key(ui_key: id, modifiers: Modifiers) -> Option<Keystroke> {
    unsafe {
        let key_code: i64 = msg_send![ui_key, keyCode];
        let mut key_char = None;
        let key = match key_code {
            HID_USAGE_KEYBOARD_RETURN_OR_ENTER => {
                key_char = Some("\n".to_string());
                "enter".to_string()
            }
            HID_USAGE_KEYBOARD_ESCAPE => "escape".to_string(),
            HID_USAGE_KEYBOARD_DELETE_OR_BACKSPACE => "backspace".to_string(),
            HID_USAGE_KEYBOARD_TAB => {
                key_char = Some("\t".to_string());
                "tab".to_string()
            }
            HID_USAGE_KEYBOARD_SPACEBAR => {
                key_char = Some(" ".to_string());
                "space".to_string()
            }
            HID_USAGE_KEYBOARD_F1..=HID_USAGE_KEYBOARD_F12 => {
                format!("f{}", key_code - HID_USAGE_KEYBOARD_F1 + 1)
            }
            HID_USAGE_KEYBOARD_HOME => "home".to_string(),
            HID_USAGE_KEYBOARD_PAGE_UP => "pageup".to_string(),
            HID_USAGE_KEYBOARD_DELETE_FORWARD => "delete".to_string(),
            HID_USAGE_KEYBOARD_END => "end".to_string(),
            HID_USAGE_KEYBOARD_PAGE_DOWN => "pagedown".to_string(),
            HID_USAGE_KEYBOARD_RIGHT_ARROW => "right".to_string(),
            HID_USAGE_KEYBOARD_LEFT_ARROW => "left".to_string(),
            HID_USAGE_KEYBOARD_DOWN_ARROW => "down".to_string(),
            HID_USAGE_KEYBOARD_UP_ARROW => "up".to_string(),
            _ => {
                let unmodified: id = msg_send![ui_key, charactersIgnoringModifiers];
                let unmodified = string_from_ns_string(unmodified);
                if unmodified.is_empty() {
                    return None;
                }
                let characters: id = msg_send![ui_key, characters];
                let characters = string_from_ns_string(characters);
                if !modifiers.control
                    && !modifiers.platform
                    && !characters.is_empty()
                    && characters.chars().all(|character| !character.is_control())
                {
                    key_char = Some(characters);
                }
                unmodified.to_lowercase()
            }
        };
        Some(Keystroke {
            modifiers,
            key,
            key_char,
        })
    }
}

pub(crate) unsafe fn string_from_ns_string(ns_string: id) -> String {
    if ns_string.is_null() {
        return String::new();
    }
    unsafe {
        let utf8: *const c_char = msg_send![ns_string, UTF8String];
        if utf8.is_null() {
            return String::new();
        }
        CStr::from_ptr(utf8).to_string_lossy().into_owned()
    }
}

extern "C" fn handle_pinch(this: &Object, _: Sel, recognizer: id) {
    let window_state = unsafe { get_window_state(this) };
    let view = this as *const Object as id;
    let recognizer_state: i64 = unsafe { msg_send![recognizer, state] };
    let location: CGPoint = unsafe { msg_send![recognizer, locationInView: view] };
    let position = point(px(location.x as f32), px(location.y as f32));

    let (phase, delta) = match recognizer_state {
        UI_GESTURE_RECOGNIZER_STATE_BEGAN => (TouchPhase::Started, 0.),
        UI_GESTURE_RECOGNIZER_STATE_CHANGED => {
            let scale: CGFloat = unsafe { msg_send![recognizer, scale] };
            // Resetting after each read makes `scale - 1` the fractional
            // change since the previous event, which is what
            // `PinchEvent::delta` expects (macOS's `magnification`).
            let _: () = unsafe { msg_send![recognizer, setScale: 1. as CGFloat] };
            (TouchPhase::Moved, scale as f32 - 1.)
        }
        UI_GESTURE_RECOGNIZER_STATE_ENDED
        | UI_GESTURE_RECOGNIZER_STATE_CANCELLED
        | UI_GESTURE_RECOGNIZER_STATE_FAILED => (TouchPhase::Ended, 0.),
        _ => return,
    };
    dispatch_input(
        &window_state,
        PlatformInput::Pinch(PinchEvent {
            position,
            delta,
            modifiers: Modifiers::default(),
            phase,
        }),
    );
}
