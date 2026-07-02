use crate::{CGFloat, CGPoint, CGRect, IosDisplay, id, nil};
use futures::channel::oneshot;
use gpui::{
    Bounds, Capslock, DispatchEventResult, GpuSpecs, Modifiers, MouseButton, MouseDownEvent,
    MouseExitEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformAtlas, PlatformDisplay,
    PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel,
    RequestFrameOptions, Scene, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    WindowControlArea, point, px, size,
};
use gpui_apple::metal_renderer::{self, Renderer};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, NO, Object, Sel, YES},
    sel, sel_impl,
};
use raw_window_handle as rwh;
use std::{
    cell::RefCell,
    ffi::c_void,
    mem, ptr,
    rc::Rc,
    sync::{Arc, Once},
};

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    static NSDefaultRunLoopMode: id;
}

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    static UIApplicationWillResignActiveNotification: id;
    static UIApplicationDidBecomeActiveNotification: id;
}

const WINDOW_STATE_IVAR: &str = "windowState";

struct IosWindowState {
    native_window: id,
    view_controller: id,
    native_view: id,
    display_link: id,
    display_link_target: id,
    renderer: Renderer,
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    request_frame_callback: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    active_status_change_callback: Option<Box<dyn FnMut(bool)>>,
    input_callback: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    input_handler: Option<PlatformInputHandler>,
    mouse_position: Point<Pixels>,
    is_active: bool,
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
            // The pointer shim can only represent one touch; with multi-touch
            // disabled UIKit coalesces additional fingers instead of
            // interleaving their events.
            let _: () = msg_send![native_view, setMultipleTouchEnabled: NO];
            let _: () = msg_send![view_controller, setView: native_view];

            let _: () = msg_send![native_window, setRootViewController: view_controller];
            let _: () = msg_send![native_window, makeKeyAndVisible];

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
                display_link: nil,
                display_link_target: nil,
                renderer,
                bounds,
                scale_factor,
                request_frame_callback: None,
                active_status_change_callback: None,
                input_callback: None,
                input_handler: None,
                mouse_position: Point::default(),
                // The app launches foreground-active; UIKit only notifies on
                // transitions.
                is_active: true,
            })));

            // The view and the display-link target each keep a strong `Rc`
            // reference to the window state in an ivar; `Drop` reclaims them.
            (*native_view).set_ivar(
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
        let (display_link, display_link_target, view_controller, native_view, native_window) = {
            let state = self.0.borrow();
            (
                state.display_link,
                state.display_link_target,
                state.view_controller,
                state.native_view,
                state.native_window,
            )
        };
        unsafe {
            let notification_center: id = msg_send![class!(NSNotificationCenter), defaultCenter];
            let _: () = msg_send![notification_center, removeObserver: display_link_target];
            let _: () = msg_send![display_link, invalidate];
            // Reclaim the strong references the display-link target and the
            // view hold so the window state can actually be freed.
            let raw: *mut c_void = *(*display_link_target).get_ivar(WINDOW_STATE_IVAR);
            drop(Rc::from_raw(raw as *const RefCell<IosWindowState>));
            let raw: *mut c_void = *(*native_view).get_ivar(WINDOW_STATE_IVAR);
            drop(Rc::from_raw(raw as *const RefCell<IosWindowState>));
            let _: () = msg_send![display_link_target, release];
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
        Modifiers::default()
    }

    fn capslock(&self) -> Capslock {
        Capslock::default()
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

    fn on_resize(&self, _callback: Box<dyn FnMut(Size<Pixels>, f32)>) {}

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
        }
        decl.register();
    });
    Class::get("GPUIDisplayLinkTarget").expect("GPUIDisplayLinkTarget was just registered")
}

unsafe fn get_window_state(object: &Object) -> Rc<RefCell<IosWindowState>> {
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
    // Don't hold the RefCell borrow across the callback: gpui reenters the
    // window from inside it (e.g. `draw`).
    let callback = window_state.borrow_mut().request_frame_callback.take();
    if let Some(mut callback) = callback {
        callback(RequestFrameOptions::default());
        window_state.borrow_mut().request_frame_callback = Some(callback);
    }
}

extern "C" fn application_will_resign_active(this: &Object, _: Sel, _notification: id) {
    let window_state = unsafe { get_window_state(this) };
    let callback = {
        let mut state = window_state.borrow_mut();
        state.is_active = false;
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

/// Reads the single coalesced touch out of a `touches` set. UIKit's
/// `locationInView:` coordinates are in points, which map 1:1 onto gpui's
/// logical pixels.
unsafe fn touch_position_and_tap_count(view: &Object, touches: id) -> (Point<Pixels>, usize) {
    unsafe {
        let touch: id = msg_send![touches, anyObject];
        let location: CGPoint = msg_send![touch, locationInView: view as *const Object as id];
        let tap_count: usize = msg_send![touch, tapCount];
        (
            point(px(location.x as f32), px(location.y as f32)),
            tap_count,
        )
    }
}

/// Invokes the gpui input callback with the `RefCell` borrow released: gpui
/// may reenter the window (e.g. to read `mouse_position` or request a frame)
/// while handling the event. The returned `DispatchEventResult` is ignored
/// because no gesture-recognizer arbitration exists yet to act on it.
fn dispatch_input(window_state: &Rc<RefCell<IosWindowState>>, input: PlatformInput) {
    let callback = window_state.borrow_mut().input_callback.take();
    if let Some(mut callback) = callback {
        callback(input);
        window_state.borrow_mut().input_callback = Some(callback);
    }
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
    let (position, tap_count) = unsafe { touch_position_and_tap_count(this, touches) };
    window_state.borrow_mut().mouse_position = position;
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
    let (position, _) = unsafe { touch_position_and_tap_count(this, touches) };
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
    let (position, tap_count) = unsafe { touch_position_and_tap_count(this, touches) };
    window_state.borrow_mut().mouse_position = position;
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

extern "C" fn touches_cancelled(this: &Object, _: Sel, _touches: id, _event: id) {
    let window_state = unsafe { get_window_state(this) };
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
