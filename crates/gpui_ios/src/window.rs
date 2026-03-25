use crate::{
    display_link::DisplayLink,
    metal_renderer::{InstanceBufferPool, MetalRenderer},
};
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Bounds, Capslock, DevicePixels, DispatchEventResult, GpuSpecs, KeyDownEvent, KeyUpEvent,
    Keystroke, Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformWindow, Pixels, Point, PromptButton, PromptLevel, RequestFrameOptions, Scene,
    ScrollDelta, ScrollWheelEvent, Size, TouchPhase, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls, WindowParams,
};
use objc::{
    class, declare::ClassDecl, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use parking_lot::Mutex;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, UiKitDisplayHandle,
    UiKitWindowHandle, WindowHandle,
};
use std::{
    cell::RefCell,
    ffi::{CStr, c_void},
    ptr::NonNull,
    rc::{Rc, Weak},
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

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UIEdgeInsets {
    top: f64,
    left: f64,
    bottom: f64,
    right: f64,
}

// objc::Encode implementations allow these types to appear in add_method signatures.
unsafe impl objc::Encode for UIEdgeInsets {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{UIEdgeInsets=dddd}") }
    }
}
unsafe impl objc::Encode for CGPoint {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGPoint=dd}") }
    }
}
unsafe impl objc::Encode for CGSize {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGSize=dd}") }
    }
}
unsafe impl objc::Encode for CGRect {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") }
    }
}

// ─── Window state ─────────────────────────────────────────────────────────────

/// All mutable window state shared between `IosWindow` and the `layoutSubviews`
/// ObjC callback. Wrapped in `Rc<RefCell<>>` so the view's ivar can hold a
/// `Weak` reference without creating a retain cycle.
struct IosWindowState {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    /// Current hardware keyboard modifier state, updated on every key event.
    current_modifiers: Modifiers,
    current_capslock: Capslock,
    /// Last touch location in logical pixels; `None` when no finger is down.
    touch_position: Option<Point<Pixels>>,
    /// Single-finger touch that has not yet moved past TOUCH_SLOP. We defer
    /// `MouseDown` until the finger lifts (confirmed tap) or exceeds the slop
    /// threshold (confirmed drag), so that the first finger of a two-finger
    /// scroll gesture does not accidentally trigger click handlers.
    pending_tap: Option<Point<Pixels>>,
    input_handler: Option<PlatformInputHandler>,
    /// Raw pointer to the `ZedMetalView` UIView. Not retained — the view is
    /// owned by the UIWindow hierarchy and outlives this state while the scene
    /// is connected.
    ui_view: *mut Object,
    /// Whether the software keyboard is currently visible.
    keyboard_shown: bool,
    /// Set by confirmed single-finger taps; consumed by `set_input_handler`
    /// when transitioning to a newly-focused input element. Prevents keyboard
    /// from appearing on app launch (auto-focus) or during scroll gestures.
    show_keyboard_after_tap: bool,
    /// UIKit safe area insets (status bar, home indicator, etc.). Exposed
    /// via `PlatformWindow::safe_area_insets()` so views can apply padding.
    safe_area_insets: gpui::Edges<Pixels>,
    renderer: MetalRenderer,
    callbacks: IosWindowCallbacks,
}

impl IosWindowState {
    /// Updates drawable size and logical bounds from the view's current layout.
    /// Returns `Some((new_size, scale))` if the size changed — the caller must
    /// fire the resize callback *outside* the `RefCell` borrow to avoid
    /// re-entrancy panics (GPUI calls back into the window from the callback).
    unsafe fn apply_layout(
        &mut self,
        view: *mut Object,
    ) -> Option<(gpui::Size<Pixels>, f32)> {
        let view_bounds: CGRect = msg_send![view, bounds];
        let scale: f32 = msg_send![view, contentScaleFactor];
        let scale = if scale > 0.0 { scale } else { 2.0 };

        // Store safe area insets so views can apply them as layout padding.
        // The Metal layer stays full-screen for edge-to-edge background rendering.
        let insets: UIEdgeInsets = msg_send![view, safeAreaInsets];
        self.safe_area_insets = gpui::Edges {
            top: gpui::px(insets.top as f32),
            right: gpui::px(insets.right as f32),
            bottom: gpui::px(insets.bottom as f32),
            left: gpui::px(insets.left as f32),
        };

        let logical_width = view_bounds.size.width as f32;
        let logical_height = view_bounds.size.height as f32;
        if logical_width <= 0.0 || logical_height <= 0.0 {
            return None;
        }

        let device_width = (logical_width * scale).round() as i32;
        let device_height = (logical_height * scale).round() as i32;

        let layer_ptr = self.renderer.layer_ptr();
        let _: () = msg_send![layer_ptr, setFrame: view_bounds];
        let _: () = msg_send![layer_ptr, setContentsScale: scale as f64];
        self.renderer.update_drawable_size(gpui::size(
            DevicePixels(device_width),
            DevicePixels(device_height),
        ));

        let new_size = gpui::Size {
            width: gpui::px(logical_width),
            height: gpui::px(logical_height),
        };
        let old_size = self.bounds.size;
        self.bounds.size = new_size;
        self.scale_factor = scale;

        if old_size != new_size { Some((new_size, scale)) } else { None }
    }
}

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

/// iOS window backed by a UIView with a CAMetalLayer sublayer.
///
/// On creation we find the key UIWindow (created by Swift SceneDelegate),
/// attach a full-screen UIView to it, and add the renderer's CAMetalLayer
/// as a sublayer. CADisplayLink drives the frame request callback.
///
/// Drawable size and logical bounds are set by `ZedMetalView.layoutSubviews`,
/// which fires after UIKit lays out the view — reliably handling the initial
/// layout, device rotation, and Stage Manager window resizes.
pub struct IosWindow {
    state: Rc<RefCell<IosWindowState>>,
    display_link: RefCell<Option<DisplayLink>>,
    display: Rc<dyn PlatformDisplay>,
}

impl IosWindow {
    pub fn new(
        _params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
        instance_buffer_pool: Arc<Mutex<InstanceBufferPool>>,
    ) -> Result<Self> {
        let renderer = MetalRenderer::new(instance_buffer_pool);
        let ui_view = Self::create_and_attach_view(&renderer)?;

        let state = Rc::new(RefCell::new(IosWindowState {
            bounds: Bounds::default(),
            scale_factor: 2.0,
            current_modifiers: Modifiers::default(),
            current_capslock: Capslock::default(),
            touch_position: None,
            pending_tap: None,
            input_handler: None,
            ui_view,
            keyboard_shown: false,
            show_keyboard_after_tap: false,
            safe_area_insets: gpui::Edges::default(),
            renderer,
            callbacks: IosWindowCallbacks::default(),
        }));

        // Store a Weak in the view's ivar. The view calls back into us via
        // `layoutSubviews`; the Weak ensures we don't access freed state if
        // the window is ever torn down before the view.
        let weak: Weak<RefCell<IosWindowState>> = Rc::downgrade(&state);
        let weak_ptr = Box::into_raw(Box::new(weak)) as *mut c_void;
        unsafe {
            (*ui_view).set_ivar("_window_state", weak_ptr);
            // The initial addSubview: triggered layoutSubviews before the ivar
            // was set, so apply_layout was skipped. Force a layout pass now so
            // the Metal drawable gets valid dimensions before the first vsync.
            let _: () = msg_send![ui_view, setNeedsLayout];
            let _: () = msg_send![ui_view, layoutIfNeeded];
            // Make the view first responder immediately so hardware keyboard
            // events (pressesBegan/pressesEnded) are delivered even before any
            // editor or software keyboard is active.
            let _: bool = msg_send![ui_view, becomeFirstResponder];
        }

        Ok(Self {
            state,
            display_link: RefCell::new(None),
            display,
        })
    }

    /// Attaches a `ZedMetalView` (with the renderer's CAMetalLayer as a sublayer)
    /// to the key UIWindow. Returns the new view so the caller can set the
    /// `_window_state` ivar. Drawable size will be set by `layoutSubviews` before
    /// the first vsync.
    fn create_and_attach_view(renderer: &MetalRenderer) -> Result<*mut Object> {
        unsafe {
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let key_window: *mut Object = msg_send![app, keyWindow];
            anyhow::ensure!(
                !key_window.is_null(),
                "no key UIWindow — SceneDelegate must call makeKeyAndVisible before zed_ios_open_window"
            );

            // Attach to rootViewController.view, not to UIWindow directly.
            // UIKit only reliably propagates Stage Manager resize events (via
            // layoutSubviews) through the rootViewController's view hierarchy.
            // Bare UIWindow subviews do not receive autoresizing updates on
            // Stage Manager window resize.
            let root_vc: *mut Object = msg_send![key_window, rootViewController];
            anyhow::ensure!(
                !root_vc.is_null(),
                "UIWindow has no rootViewController — SceneDelegate must set one before zed_ios_open_window"
            );
            let container: *mut Object = msg_send![root_vc, view];

            // Use the container's current bounds for the initial frame so that
            // the autoresizing mask has a non-zero base to work from.
            // `layoutSubviews` will immediately correct this to view.bounds ×
            // contentScaleFactor, and will continue to handle rotation and
            // Stage Manager resizes.
            let container_bounds: CGRect = msg_send![container, bounds];
            let initial_frame = if container_bounds.size.width > 0.0 && container_bounds.size.height > 0.0 {
                container_bounds
            } else {
                let main_screen: *mut Object = msg_send![class!(UIScreen), mainScreen];
                msg_send![main_screen, bounds]
            };

            let view_class = register_metal_view_class();
            let view: *mut Object = msg_send![view_class, alloc];
            let view: *mut Object = msg_send![view, initWithFrame: initial_frame];

            // Stretch to fill the container on rotation or Stage Manager resize.
            let fill_mask: usize = (1 << 1) | (1 << 4); // FlexibleWidth | FlexibleHeight
            let _: () = msg_send![view, setAutoresizingMask: fill_mask];

            // Add the Metal layer as a sublayer (zero-sized; layoutSubviews will resize it).
            let layer_ptr = renderer.layer_ptr();
            let view_layer: *mut Object = msg_send![view, layer];
            let _: () = msg_send![view_layer, addSublayer: layer_ptr];

            // If the container already has subviews, force a synchronous layout
            // pass after adding ours so the Metal layer gets a valid drawable
            // size before the first CADisplayLink tick. The first window doesn't
            // need this because UIKit lays it out naturally before the first vsync;
            // subsequent windows are added to an already-laid-out container and
            // may not trigger layoutSubviews in time without an explicit nudge.
            let existing_subviews: *mut Object = msg_send![container, subviews];
            let subview_count: usize = msg_send![existing_subviews, count];
            let _: () = msg_send![container, addSubview: view];
            if subview_count > 0 {
                let _: () = msg_send![container, layoutIfNeeded];
            }

            // Attach a two-finger pan gesture recognizer for direct touch scrolling.
            // Raw touchesBegan: is filtered to single-finger only so there is no overlap.
            let pan: *mut Object = msg_send![class!(UIPanGestureRecognizer), alloc];
            let pan: *mut Object =
                msg_send![pan, initWithTarget: view action: sel!(handlePanGesture:)];
            let _: () = msg_send![pan, setMinimumNumberOfTouches: 2usize];
            let _: () = msg_send![pan, setMaximumNumberOfTouches: 2usize];
            let _: () = msg_send![view, addGestureRecognizer: pan];
            let _: () = msg_send![pan, release];

            // Separate pan gesture for trackpad/mouse scroll (iPadOS 13.4+,
            // also used by the simulator for trackpad scroll gestures).
            // Trackpad scroll events have zero touches, so this recognizer
            // must allow 0 minimum touches and opt into scroll event types
            // via allowedScrollTypesMask.
            let trackpad_pan: *mut Object = msg_send![class!(UIPanGestureRecognizer), alloc];
            let trackpad_pan: *mut Object =
                msg_send![trackpad_pan, initWithTarget: view action: sel!(handlePanGesture:)];
            let _: () = msg_send![trackpad_pan, setMinimumNumberOfTouches: 0usize];
            let _: () = msg_send![trackpad_pan, setMaximumNumberOfTouches: 0usize];
            // UIScrollTypeMask: Discrete=1, Continuous=2; allow both (3).
            let _: () = msg_send![trackpad_pan, setAllowedScrollTypesMask: 3usize];
            let _: () = msg_send![view, addGestureRecognizer: trackpad_pan];
            let _: () = msg_send![trackpad_pan, release];

            // Observe UIKeyboardDidHideNotification so our keyboard_shown state
            // stays in sync when the user dismisses the keyboard via UIKit's own
            // controls (e.g. the dismiss key on iPadOS).
            let center: *mut Object = msg_send![class!(NSNotificationCenter), defaultCenter];
            let name = ns_string("UIKeyboardDidHideNotification");
            let _: () = msg_send![center,
                addObserver: view
                selector: sel!(keyboardDidHide:)
                name: name
                object: std::ptr::null::<Object>()
            ];

            Ok(view)
        }
    }
}

// ─── raw-window-handle ───────────────────────────────────────────────────────

impl HasWindowHandle for IosWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let ptr = NonNull::new(self.state.borrow().ui_view as *mut c_void)
            .ok_or(HandleError::Unavailable)?;
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
        self.state.borrow().bounds
    }

    fn is_maximized(&self) -> bool {
        true
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.state.borrow().bounds)
    }

    fn content_size(&self) -> Size<Pixels> {
        self.state.borrow().bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.state.borrow_mut().bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        self.state.borrow().scale_factor
    }

    fn safe_area_insets(&self) -> gpui::Edges<Pixels> {
        self.state.borrow().safe_area_insets.clone()
    }

    fn appearance(&self) -> WindowAppearance {
        unsafe {
            let view = self.state.borrow().ui_view;
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
        self.state.borrow().touch_position.unwrap_or_default()
    }

    fn modifiers(&self) -> Modifiers {
        self.state.borrow().current_modifiers
    }

    fn capslock(&self) -> Capslock {
        self.state.borrow().current_capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        let mut state = self.state.borrow_mut();
        state.input_handler = Some(input_handler);

        if state.keyboard_shown {
            // Keyboard is already visible. Cancel any deferred dismiss that
            // take_input_handler scheduled earlier in this draw cycle — focus
            // is still on an input element, so the keyboard stays up.
            let view = state.ui_view;
            drop(state);
            unsafe {
                let _: () = msg_send![class!(NSObject),
                    cancelPreviousPerformRequestsWithTarget: view
                    selector: sel!(dismissKeyboard)
                    object: std::ptr::null::<Object>()
                ];
            }
            return;
        }

        // Show the software keyboard if the user just tapped. The flag is
        // only set by confirmed single-finger taps in handle_touches, so
        // this won't fire on app launch (auto-focus) or scroll gestures.
        if state.show_keyboard_after_tap {
            state.show_keyboard_after_tap = false;
            state.keyboard_shown = true;
            let view = state.ui_view;
            drop(state);
            unsafe {
                (*(view)).set_ivar("_keyboard_requested", true);
                let _: bool = msg_send![view, becomeFirstResponder];
            }
            return;
        }
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        let handler = self.state.borrow_mut().input_handler.take();
        let had_handler = handler.is_some();
        let state = self.state.borrow_mut();

        // If we had a handler and keyboard is visible, defer a dismiss.
        // If set_input_handler is called later in the same draw cycle (normal
        // per-frame recycle), it will cancel this. If set is NOT called (focus
        // left the input element), the dismiss fires on the next run-loop turn.
        if had_handler && state.keyboard_shown {
            let view = state.ui_view;
            drop(state);
            unsafe {
                let _: () = msg_send![class!(NSObject),
                    cancelPreviousPerformRequestsWithTarget: view
                    selector: sel!(dismissKeyboard)
                    object: std::ptr::null::<Object>()
                ];
                let _: () = msg_send![view,
                    performSelector: sel!(dismissKeyboard)
                    withObject: std::ptr::null::<Object>()
                    afterDelay: 0.0f64
                ];
            }
        }

        handler
    }

    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[PromptButton],
    ) -> Option<oneshot::Receiver<usize>> {
        let (tx, rx) = oneshot::channel::<usize>();
        let tx = Arc::new(Mutex::new(Some(tx)));

        unsafe {
            // UIAlertControllerStyleAlert == 1
            let title = ns_string(msg);
            let message = detail.map(|d| ns_string(d)).unwrap_or(std::ptr::null_mut());
            let alert: *mut Object = msg_send![class!(UIAlertController),
                alertControllerWithTitle: title
                message: message
                preferredStyle: 1isize
            ];
            let _: () = msg_send![title, release];
            if !message.is_null() {
                let _: () = msg_send![message, release];
            }

            for (index, answer) in answers.iter().enumerate() {
                let style: isize = match answer {
                    PromptButton::Cancel(_) => 1, // UIAlertActionStyleCancel
                    _ => {
                        if level == PromptLevel::Critical {
                            2 // UIAlertActionStyleDestructive
                        } else {
                            0 // UIAlertActionStyleDefault
                        }
                    }
                };
                let label = match answer {
                    PromptButton::Ok(s) | PromptButton::Cancel(s) | PromptButton::Other(s) => s,
                };
                let label_ns = ns_string(label);

                let tx_clone = tx.clone();
                let handler = block::ConcreteBlock::new(move |_action: *mut Object| {
                    if let Some(sender) = tx_clone.lock().take() {
                        sender.send(index).ok();
                    }
                });
                let handler = handler.copy();

                let action: *mut Object = msg_send![class!(UIAlertAction),
                    actionWithTitle: label_ns
                    style: style
                    handler: &*handler
                ];
                let _: () = msg_send![alert, addAction: action];
                let _: () = msg_send![label_ns, release];
            }

            // Present on the root view controller of the key window
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let key_window: *mut Object = msg_send![app, keyWindow];
            if !key_window.is_null() {
                let root_vc: *mut Object = msg_send![key_window, rootViewController];
                if !root_vc.is_null() {
                    let _: () = msg_send![root_vc,
                        presentViewController: alert
                        animated: true
                        completion: std::ptr::null::<c_void>()
                    ];
                }
            }
        }

        Some(rx)
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
        self.state.borrow_mut().callbacks.input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.state.borrow_mut().callbacks.active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.state.borrow_mut().callbacks.hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.state.borrow_mut().callbacks.resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.state.borrow_mut().callbacks.should_close = Some(callback);
    }

    fn on_hit_test_window_control(
        &self,
        callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
        self.state.borrow_mut().callbacks.hit_test_window_control = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.state.borrow_mut().callbacks.close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        self.state.borrow_mut().renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.state.borrow().renderer.sprite_atlas().clone()
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

// ─── Input event helpers ──────────────────────────────────────────────────────

// UIKeyModifierFlags bit positions.
const UI_KEY_MODIFIER_ALPHA_SHIFT: usize = 1 << 16; // Caps Lock
const UI_KEY_MODIFIER_SHIFT: usize = 1 << 17;
const UI_KEY_MODIFIER_CONTROL: usize = 1 << 18;
const UI_KEY_MODIFIER_ALTERNATE: usize = 1 << 19;
const UI_KEY_MODIFIER_COMMAND: usize = 1 << 20;

// UIKeyboardHIDUsage values — raw HID keyboard/keypad usage IDs (page 0x07).
// UIKey.keyCode returns these without the usage-page prefix, so 0x2A not 0x0007002A.
const HID_ENTER: usize = 0x28;
const HID_ESCAPE: usize = 0x29;
const HID_BACKSPACE: usize = 0x2A;
const HID_TAB: usize = 0x2B;
const HID_SPACE: usize = 0x2C;
const HID_F1: usize = 0x3A;
const HID_F12: usize = 0x45;
const HID_INSERT: usize = 0x49;
const HID_HOME: usize = 0x4A;
const HID_PAGE_UP: usize = 0x4B;
const HID_DELETE_FORWARD: usize = 0x4C;
const HID_END: usize = 0x4D;
const HID_PAGE_DOWN: usize = 0x4E;
const HID_RIGHT_ARROW: usize = 0x4F;
const HID_LEFT_ARROW: usize = 0x50;
const HID_DOWN_ARROW: usize = 0x51;
const HID_UP_ARROW: usize = 0x52;
// Modifier key HID codes — presses of these alone emit ModifiersChanged only.
const HID_CAPS_LOCK: usize = 0x39;
const HID_MOD_LEFT_CTRL: usize = 0xE0;
const HID_MOD_LEFT_SHIFT: usize = 0xE1;
const HID_MOD_LEFT_ALT: usize = 0xE2;
const HID_MOD_LEFT_GUI: usize = 0xE3;
const HID_MOD_RIGHT_CTRL: usize = 0xE4;
const HID_MOD_RIGHT_SHIFT: usize = 0xE5;
const HID_MOD_RIGHT_ALT: usize = 0xE6;
const HID_MOD_RIGHT_GUI: usize = 0xE7;

fn modifiers_from_ui_flags(flags: usize) -> (Modifiers, Capslock) {
    (
        Modifiers {
            shift: flags & UI_KEY_MODIFIER_SHIFT != 0,
            control: flags & UI_KEY_MODIFIER_CONTROL != 0,
            alt: flags & UI_KEY_MODIFIER_ALTERNATE != 0,
            platform: flags & UI_KEY_MODIFIER_COMMAND != 0,
            function: false,
        },
        Capslock { on: flags & UI_KEY_MODIFIER_ALPHA_SHIFT != 0 },
    )
}

fn is_modifier_key(hid_code: usize) -> bool {
    matches!(
        hid_code,
        HID_CAPS_LOCK
            | HID_MOD_LEFT_CTRL
            | HID_MOD_LEFT_SHIFT
            | HID_MOD_LEFT_ALT
            | HID_MOD_LEFT_GUI
            | HID_MOD_RIGHT_CTRL
            | HID_MOD_RIGHT_SHIFT
            | HID_MOD_RIGHT_ALT
            | HID_MOD_RIGHT_GUI
    )
}

/// Converts a `UIKey` ObjC object to a GPUI [`Keystroke`].
///
/// For named keys (arrows, function keys, etc.) we match on the HID usage code
/// since `charactersIgnoringModifiers` is empty for non-printable keys on iOS.
/// For printable keys we use `charactersIgnoringModifiers` for the key name and
/// `characters` for `key_char` (the actually-typed text, omitted when platform
/// or control modifiers are held since those suppress character output).
fn keystroke_from_ui_key(key: *mut Object, modifiers: Modifiers) -> Option<Keystroke> {
    unsafe {
    let hid_code: usize = msg_send![key, keyCode];

    // Named (non-printable) keys identified by HID usage code.
    let named = match hid_code {
        HID_ENTER => Some(("enter", Some("\n"))),
        HID_ESCAPE => Some(("escape", None)),
        HID_BACKSPACE => Some(("backspace", None)),
        HID_TAB => Some(("tab", Some("\t"))),
        HID_SPACE => Some(("space", Some(" "))),
        HID_UP_ARROW => Some(("up", None)),
        HID_DOWN_ARROW => Some(("down", None)),
        HID_LEFT_ARROW => Some(("left", None)),
        HID_RIGHT_ARROW => Some(("right", None)),
        HID_HOME => Some(("home", None)),
        HID_END => Some(("end", None)),
        HID_PAGE_UP => Some(("pageup", None)),
        HID_PAGE_DOWN => Some(("pagedown", None)),
        HID_DELETE_FORWARD => Some(("delete", None)),
        HID_INSERT => Some(("insert", None)),
        k if (HID_F1..=HID_F12).contains(&k) => {
            const NAMES: [&str; 12] =
                ["f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12"];
            Some((NAMES[k - HID_F1], None))
        }
        _ => None,
    };
    if let Some((key_name, key_char_str)) = named {
        return Some(Keystroke {
            modifiers,
            key: key_name.to_string(),
            key_char: key_char_str.map(str::to_string),
        });
    }

    // Printable keys: derive key name from the unmodified character string.
    let unmodified_ns: *mut Object = msg_send![key, charactersIgnoringModifiers];
    if unmodified_ns.is_null() {
        return None;
    }
    let unmodified_ptr: *const i8 = msg_send![unmodified_ns, UTF8String];
    if unmodified_ptr.is_null() {
        return None;
    }
    let unmodified = CStr::from_ptr(unmodified_ptr).to_str().unwrap_or("");
    if unmodified.is_empty() {
        return None;
    }
    // Key name is always lowercase (e.g. "a", not "A") regardless of shift.
    let key_name = unmodified.to_lowercase();

    // key_char is the actual typed character. Omit when platform or control
    // modifiers are held because those combinations don't produce text output.
    let key_char = if !modifiers.platform && !modifiers.control {
        let chars_ns: *mut Object = msg_send![key, characters];
        if chars_ns.is_null() {
            None
        } else {
            let chars_ptr: *const i8 = msg_send![chars_ns, UTF8String];
            if chars_ptr.is_null() {
                None
            } else {
                let chars = CStr::from_ptr(chars_ptr).to_str().unwrap_or("");
                if chars.is_empty() { None } else { Some(chars.to_string()) }
            }
        }
    } else {
        None
    };

    Some(Keystroke { modifiers, key: key_name, key_char })
    } // unsafe
}

/// Fires the `on_input` callback with `event` using the take/restore pattern
/// so the callback is invoked outside the `RefCell` borrow.
/// Returns the `DispatchEventResult` so callers can detect unhandled events.
fn dispatch_input_event(
    state_rc: &Rc<RefCell<IosWindowState>>,
    event: PlatformInput,
) -> DispatchEventResult {
    let mut callback = state_rc.borrow_mut().callbacks.input.take();
    let result = if let Some(ref mut f) = callback {
        f(event)
    } else {
        DispatchEventResult { propagate: true, default_prevented: false }
    };
    let mut state = state_rc.borrow_mut();
    if state.callbacks.input.is_none() {
        state.callbacks.input = callback;
    }
    result
}

// ─── ZedMetalView ObjC class ──────────────────────────────────────────────────

fn register_metal_view_class() -> &'static Class {
    use std::sync::OnceLock;
    static CLASS: OnceLock<&'static Class> = OnceLock::new();
    CLASS.get_or_init(|| {
        // Pre-register the helper classes so they exist in the ObjC runtime.
        let _ = register_text_position_class();
        let _ = register_text_range_class();

        let superclass = class!(UIView);
        let mut decl =
            ClassDecl::new("ZedMetalView", superclass).expect("ZedMetalView already registered");

        // Stores a raw pointer to `Box<Weak<RefCell<IosWindowState>>>`.
        // Set after IosWindow construction; freed in `dealloc`.
        decl.add_ivar::<*mut c_void>("_window_state");

        // Controls whether inputView returns nil (show keyboard) or an empty
        // UIView (suppress keyboard). Only set to true when we explicitly want
        // the software keyboard.
        decl.add_ivar::<bool>("_keyboard_requested");

        // Conform to UIKeyInput so UIKit routes software keyboard input here.
        if let Some(protocol) = Protocol::get("UIKeyInput") {
            decl.add_protocol(protocol);
        }

        unsafe {
            decl.add_method(
                sel!(layoutSubviews),
                layout_subviews as extern "C" fn(&Object, Sel),
            );
            decl.add_method(
                sel!(dealloc),
                view_dealloc as extern "C" fn(&Object, Sel),
            );
            // The view must be first responder to receive pressesBegan:/pressesEnded:
            // and touchesBegan: events. UIView.canBecomeFirstResponder defaults to NO.
            decl.add_method(
                sel!(canBecomeFirstResponder),
                can_become_first_responder as extern "C" fn(&Object, Sel) -> bool,
            );
            // Hardware keyboard
            decl.add_method(
                sel!(pressesBegan:withEvent:),
                presses_began as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(pressesEnded:withEvent:),
                presses_ended as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(pressesCancelled:withEvent:),
                presses_cancelled as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            // Touch (single-finger → left mouse button)
            decl.add_method(
                sel!(touchesBegan:withEvent:),
                touches_began as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(touchesMoved:withEvent:),
                touches_moved as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(touchesEnded:withEvent:),
                touches_ended as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            decl.add_method(
                sel!(touchesCancelled:withEvent:),
                touches_cancelled as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            // Two-finger pan gesture → ScrollWheel
            decl.add_method(
                sel!(handlePanGesture:),
                handle_pan_gesture as extern "C" fn(&Object, Sel, *mut Object),
            );

            // Mouse wheel scroll without pointer capture — the simulator
            // forwards macOS scroll wheel events as scrollWheel: when the
            // pointer is not captured. Harmless on device (never called).
            decl.add_method(
                sel!(scrollWheel:),
                handle_scroll_wheel as extern "C" fn(&Object, Sel, *mut Object),
            );

            // Simulator Cmd+K sends toggleSoftwareKeyboard: up the responder
            // chain. Override it to toggle our keyboard suppression state.
            decl.add_method(
                sel!(toggleSoftwareKeyboard:),
                toggle_software_keyboard as extern "C" fn(&Object, Sel, *mut Object),
            );

            // ── Responder actions (long-press context menu) ───────────────────
            // Return false for all edit actions until clipboard is wired up.
            // This prevents the system from showing a context menu with broken actions.
            decl.add_method(
                sel!(canPerformAction:withSender:),
                can_perform_action as extern "C" fn(&Object, Sel, Sel, *mut Object) -> bool,
            );

            // ── UIKeyInput methods ────────────────────────────────────────────
            decl.add_method(
                sel!(hasText),
                uit_has_text as extern "C" fn(&Object, Sel) -> bool,
            );
            decl.add_method(
                sel!(insertText:),
                uit_insert_text as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(deleteBackward),
                uit_delete_backward as extern "C" fn(&Object, Sel),
            );

            // ── UITextInputTraits ─────────────────────────────────────────────
            decl.add_method(
                sel!(autocorrectionType),
                traits_autocorrection_type as extern "C" fn(&Object, Sel) -> isize,
            );
            decl.add_method(
                sel!(autocapitalizationType),
                traits_autocapitalization_type as extern "C" fn(&Object, Sel) -> isize,
            );
            decl.add_method(
                sel!(spellCheckingType),
                traits_spell_checking_type as extern "C" fn(&Object, Sel) -> isize,
            );
            decl.add_method(
                sel!(smartQuotesType),
                traits_smart_quotes_type as extern "C" fn(&Object, Sel) -> isize,
            );
            decl.add_method(
                sel!(smartDashesType),
                traits_smart_dashes_type as extern "C" fn(&Object, Sel) -> isize,
            );

            // ── View lifecycle ───────────────────────────────────────────────
            decl.add_method(
                sel!(didMoveToWindow),
                did_move_to_window as extern "C" fn(&Object, Sel),
            );
            decl.add_method(
                sel!(traitCollectionDidChange:),
                trait_collection_did_change
                    as extern "C" fn(&Object, Sel, *mut Object),
            );

            // ── Keyboard control ──────────────────────────────────────────────
            // Override `inputView` to conditionally show/suppress the keyboard.
            decl.add_method(
                sel!(inputView),
                metal_view_input_view as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            // Also suppress the input accessory view (QuickType/predictive bar).
            decl.add_method(
                sel!(inputAccessoryView),
                metal_view_input_accessory_view as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            // Custom method called via deferred performSelector: to dismiss the
            // keyboard. Updates `keyboard_shown` state before resigning.
            decl.add_method(
                sel!(dismissKeyboard),
                metal_view_dismiss_keyboard as extern "C" fn(&Object, Sel),
            );
            // Notification handler for UIKeyboardDidHideNotification.
            decl.add_method(
                sel!(keyboardDidHide:),
                keyboard_did_hide as extern "C" fn(&Object, Sel, *mut Object),
            );

        }

        decl.register()
    })
}

/// Called by UIKit after it measures the view — on initial layout, rotation,
/// and Stage Manager resizes. Updates Metal drawable size and fires the GPUI
/// resize callback if the logical size changed.
///
/// The resize callback is fired *outside* the `RefCell` borrow because GPUI
/// calls back into `scale_factor()` (and other window methods) from within the
/// callback, which would cause a `RefCell already mutably borrowed` panic.
extern "C" fn layout_subviews(this: &Object, _sel: Sel) {
    unsafe {
        let superclass = class!(UIView);
        let _: () = msg_send![super(this, superclass), layoutSubviews];

        let raw: *mut c_void = *this.get_ivar("_window_state");
        if raw.is_null() {
            return;
        }
        let weak = &*(raw as *const Weak<RefCell<IosWindowState>>);
        let Some(state_rc) = weak.upgrade() else {
            return;
        };

        let view = this as *const Object as *mut Object;
        let resize_event = state_rc.borrow_mut().apply_layout(view);

        if let Some((new_size, scale)) = resize_event {
            // Take the callback out to call it without holding the borrow.
            let mut callback = state_rc.borrow_mut().callbacks.resize.take();
            if let Some(ref mut f) = callback {
                f(new_size, scale);
            }
            // Restore callback (a new one may have been set during the call, prefer it).
            let mut state = state_rc.borrow_mut();
            if state.callbacks.resize.is_none() {
                state.callbacks.resize = callback;
            }
        }
    }
}

/// Frees the `Box<Weak<…>>` stored in `_window_state` before the view is
/// released, then calls `[super dealloc]`.
extern "C" fn view_dealloc(this: &Object, _sel: Sel) {
    unsafe {
        let center: *mut Object = msg_send![class!(NSNotificationCenter), defaultCenter];
        let _: () = msg_send![center, removeObserver: this];

        let raw: *mut c_void = *this.get_ivar("_window_state");
        if !raw.is_null() {
            drop(Box::from_raw(raw as *mut Weak<RefCell<IosWindowState>>));
            let this_mut = this as *const Object as *mut Object;
            (*this_mut).set_ivar("_window_state", std::ptr::null_mut::<c_void>());
        }
        let superclass = class!(UIView);
        let _: () = msg_send![super(this, superclass), dealloc];
    }
}

/// Called by UIKit when the view is added to (or removed from) a UIWindow.
/// This is the canonical place to initialise display-dependent properties
/// like `contentsScale`.  It also serves as a safety-net layout pass: if
/// `layoutSubviews` has not yet fired by this point the Metal layer would
/// still be at 0×0, so we run `apply_layout` here as well.
extern "C" fn did_move_to_window(this: &Object, _sel: Sel) {
    unsafe {
        let superclass = class!(UIView);
        let _: () = msg_send![super(this, superclass), didMoveToWindow];

        let raw: *mut c_void = *this.get_ivar("_window_state");
        if raw.is_null() {
            return;
        }
        let weak = &*(raw as *const Weak<RefCell<IosWindowState>>);
        let Some(state_rc) = weak.upgrade() else {
            return;
        };

        let view = this as *const Object as *mut Object;
        let resize_event = state_rc.borrow_mut().apply_layout(view);

        if let Some((new_size, scale)) = resize_event {
            let mut callback = state_rc.borrow_mut().callbacks.resize.take();
            if let Some(ref mut f) = callback {
                f(new_size, scale);
            }
            let mut state = state_rc.borrow_mut();
            if state.callbacks.resize.is_none() {
                state.callbacks.resize = callback;
            }
        }
    }
}

/// Called by UIKit when the trait collection changes (e.g. dark/light mode
/// switch, Dynamic Type size change, or display gamut change).
extern "C" fn trait_collection_did_change(this: &Object, _sel: Sel, _previous: *mut Object) {
    unsafe {
        let raw: *mut c_void = *this.get_ivar("_window_state");
        if raw.is_null() {
            return;
        }
        let weak = &*(raw as *const Weak<RefCell<IosWindowState>>);
        let Some(state_rc) = weak.upgrade() else {
            return;
        };

        let mut callback = state_rc.borrow_mut().callbacks.appearance_changed.take();
        if let Some(ref mut f) = callback {
            f();
        }
        let mut state = state_rc.borrow_mut();
        if state.callbacks.appearance_changed.is_none() {
            state.callbacks.appearance_changed = callback;
        }
    }
}

extern "C" fn can_become_first_responder(_this: &Object, _sel: Sel) -> bool {
    true
}

extern "C" fn metal_view_input_view(this: &Object, _sel: Sel) -> *mut Object {
    unsafe {
        let requested: bool = *this.get_ivar::<bool>("_keyboard_requested");
        if requested {
            // Keyboard explicitly requested — return nil so UIKit shows the system keyboard.
            std::ptr::null_mut()
        } else {
            // Default: return an empty UIView to suppress the keyboard.
            let empty: *mut Object = msg_send![class!(UIView), new];
            let _: *mut Object = msg_send![empty, autorelease];
            empty
        }
    }
}

/// Suppress the QuickType / predictive text accessory bar above the keyboard.
extern "C" fn metal_view_input_accessory_view(_this: &Object, _sel: Sel) -> *mut Object {
    std::ptr::null_mut()
}

extern "C" fn metal_view_dismiss_keyboard(this: &Object, _sel: Sel) {
    if let Some(state_rc) = state_from_view(this) {
        let mut state = state_rc.borrow_mut();
        state.keyboard_shown = false;
        state.show_keyboard_after_tap = false;
    }
    unsafe {
        let this_mut = this as *const Object as *mut Object;
        (*this_mut).set_ivar("_keyboard_requested", false);
        let _: bool = msg_send![this, resignFirstResponder];
    }
}

/// Simulator Cmd+K toggle. Flip keyboard state and reload input views so
/// UIKit re-queries `inputView`.
extern "C" fn toggle_software_keyboard(this: &Object, _sel: Sel, _sender: *mut Object) {
    let keyboard_shown = state_from_view(this)
        .map(|s| s.borrow().keyboard_shown)
        .unwrap_or(false);

    if keyboard_shown {
        // Hide
        if let Some(state_rc) = state_from_view(this) {
            let mut state = state_rc.borrow_mut();
            state.keyboard_shown = false;
        }
        unsafe {
            let this_mut = this as *const Object as *mut Object;
            (*this_mut).set_ivar("_keyboard_requested", false);
            let _: () = msg_send![this, reloadInputViews];
        }
    } else {
        // Show — only if there's an active input handler (text element is focused)
        let has_handler = state_from_view(this)
            .map(|s| s.borrow().input_handler.is_some())
            .unwrap_or(false);
        if has_handler {
            if let Some(state_rc) = state_from_view(this) {
                let mut state = state_rc.borrow_mut();
                state.keyboard_shown = true;
            }
            unsafe {
                let this_mut = this as *const Object as *mut Object;
                (*this_mut).set_ivar("_keyboard_requested", true);
                let _: () = msg_send![this, reloadInputViews];
            }
        }
    }
}

/// Called when UIKit hides the keyboard for any reason (user dismiss key,
/// system gesture, etc.). Syncs our state so a subsequent tap can re-show it.
extern "C" fn keyboard_did_hide(this: &Object, _sel: Sel, _notification: *mut Object) {
    if let Some(state_rc) = state_from_view(this) {
        let mut state = state_rc.borrow_mut();
        state.keyboard_shown = false;
    }
    unsafe {
        let this_mut = this as *const Object as *mut Object;
        (*this_mut).set_ivar("_keyboard_requested", false);
    }
}

// ─── Hardware keyboard input ──────────────────────────────────────────────────

/// Extracts state from `_window_state` ivar. Returns `None` if unset or
/// if the `Weak` has been dropped.
fn state_from_view(this: &Object) -> Option<Rc<RefCell<IosWindowState>>> {
    unsafe {
        let raw: *mut c_void = *this.get_ivar("_window_state");
        if raw.is_null() {
            return None;
        }
        let weak = &*(raw as *const Weak<RefCell<IosWindowState>>);
        weak.upgrade()
    }
}

/// Processes a `UIPress` set and emits `ModifiersChanged` and `KeyDown`/`KeyUp`
/// events. `is_down` controls which key event is generated. Returns `true` if
/// at least one key was handled by GPUI (propagate=false or default_prevented=true).
fn handle_presses(this: &Object, presses: *mut Object, is_down: bool) -> bool {
    let Some(state_rc) = state_from_view(this) else { return false };
    let mut any_handled = false;

    unsafe {
        let array: *mut Object = msg_send![presses, allObjects];
        let count: usize = msg_send![array, count];
        for i in 0..count {
            let press: *mut Object = msg_send![array, objectAtIndex: i];
            let key: *mut Object = msg_send![press, key]; // UIKey* (nil on non-keyboard presses)
            if key.is_null() {
                continue;
            }

            let flags: usize = msg_send![key, modifierFlags];
            let (modifiers, capslock) = modifiers_from_ui_flags(flags);

            // Emit ModifiersChanged whenever the modifier set has changed.
            let prev_modifiers = state_rc.borrow().current_modifiers;
            if modifiers != prev_modifiers {
                state_rc.borrow_mut().current_modifiers = modifiers;
                state_rc.borrow_mut().current_capslock = capslock;
                dispatch_input_event(
                    &state_rc,
                    PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                        modifiers,
                        capslock,
                    }),
                );
            }

            let hid_code: usize = msg_send![key, keyCode];
            if is_modifier_key(hid_code) {
                continue;
            }

            if let Some(keystroke) = keystroke_from_ui_key(key, modifiers) {
                if is_down {
                    let result = dispatch_input_event(
                        &state_rc,
                        PlatformInput::KeyDown(KeyDownEvent {
                            keystroke,
                            is_held: false,
                            prefer_character_input: false,
                        }),
                    );
                    if !result.propagate || result.default_prevented {
                        any_handled = true;
                    }
                } else {
                    dispatch_input_event(&state_rc, PlatformInput::KeyUp(KeyUpEvent { keystroke }));
                }
            }
        }
    }

    any_handled
}

extern "C" fn presses_began(this: &Object, _sel: Sel, presses: *mut Object, event: *mut Object) {
    let handled = handle_presses(this, presses, true);
    if !handled {
        unsafe {
            let _: () = msg_send![super(this, class!(UIView)), pressesBegan: presses withEvent: event];
        }
    }
}

extern "C" fn presses_ended(this: &Object, _sel: Sel, presses: *mut Object, event: *mut Object) {
    handle_presses(this, presses, false);
    unsafe {
        let _: () = msg_send![super(this, class!(UIView)), pressesEnded: presses withEvent: event];
    }
}

extern "C" fn presses_cancelled(
    this: &Object,
    _sel: Sel,
    presses: *mut Object,
    event: *mut Object,
) {
    // Treat cancellation as key-up so GPUI doesn't see keys stuck in the down state.
    handle_presses(this, presses, false);
    unsafe {
        let _: () =
            msg_send![super(this, class!(UIView)), pressesCancelled: presses withEvent: event];
    }
}

// ── UITextInputTraits ─────────────────────────────────────────────────────────
// UITextAutocorrectionTypeNo = 2, UITextAutocapitalizationTypeNone = 0,
// UITextSpellCheckingTypeNo = 2, UITextSmartQuotesTypeNo = 2, UITextSmartDashesTypeNo = 2.

extern "C" fn traits_autocorrection_type(_this: &Object, _sel: Sel) -> isize {
    2 // UITextAutocorrectionTypeNo
}

extern "C" fn traits_autocapitalization_type(_this: &Object, _sel: Sel) -> isize {
    0 // UITextAutocapitalizationTypeNone
}

extern "C" fn traits_spell_checking_type(_this: &Object, _sel: Sel) -> isize {
    2 // UITextSpellCheckingTypeNo
}

extern "C" fn traits_smart_quotes_type(_this: &Object, _sel: Sel) -> isize {
    2 // UITextSmartQuotesTypeNo
}

extern "C" fn traits_smart_dashes_type(_this: &Object, _sel: Sel) -> isize {
    2 // UITextSmartDashesTypeNo
}

// ── Responder actions ─────────────────────────────────────────────────────────

extern "C" fn can_perform_action(_this: &Object, _sel: Sel, _action: Sel, _sender: *mut Object) -> bool {
    // Suppress the long-press context menu entirely until clipboard is integrated.
    false
}

// ─── Touch input (single-finger → left mouse button) ─────────────────────────

extern "C" fn touches_began(this: &Object, _sel: Sel, touches: *mut Object, _event: *mut Object) {
    handle_touches(this, touches, TouchKind::Began)
}

extern "C" fn touches_moved(this: &Object, _sel: Sel, touches: *mut Object, _event: *mut Object) {
    handle_touches(this, touches, TouchKind::Moved)
}

extern "C" fn touches_ended(
    this: &Object,
    _sel: Sel,
    touches: *mut Object,
    _event: *mut Object,
) {
    handle_touches(this, touches, TouchKind::Ended)
}

extern "C" fn touches_cancelled(
    this: &Object,
    _sel: Sel,
    touches: *mut Object,
    _event: *mut Object,
) {
    handle_touches(this, touches, TouchKind::Cancelled)
}

#[derive(PartialEq)]
enum TouchKind {
    Began,
    Moved,
    Ended,
    /// UIKit cancelled the touch (e.g. a gesture recognizer took over).
    Cancelled,
}

// ─── UITextInput support ──────────────────────────────────────────────────────

/// Creates an autoreleased NSString from a Rust `&str`.
unsafe fn ns_string(s: &str) -> *mut Object {
    let ns: *mut Object = msg_send![class!(NSString), alloc];
    msg_send![ns, initWithBytes: s.as_ptr()
                         length: s.len()
                       encoding: 4usize] // NSUTF8StringEncoding
}

fn ns_string_to_string(ns_str: *mut Object) -> String {
    if ns_str.is_null() {
        return String::new();
    }
    unsafe {
        let ptr: *const i8 = msg_send![ns_str, UTF8String];
        if ptr.is_null() {
            return String::new();
        }
        CStr::from_ptr(ptr).to_str().unwrap_or("").to_owned()
    }
}

// ── ZedTextPosition ───────────────────────────────────────────────────────────

fn register_text_position_class() -> &'static Class {
    use std::sync::OnceLock;
    static CLASS: OnceLock<&'static Class> = OnceLock::new();
    CLASS.get_or_init(|| {
        let superclass = class!(UITextPosition);
        let mut decl = ClassDecl::new("ZedTextPosition", superclass)
            .expect("ZedTextPosition already registered");
        decl.add_ivar::<usize>("_offset");
        decl.register()
    })
}

fn register_text_range_class() -> &'static Class {
    use std::sync::OnceLock;
    static CLASS: OnceLock<&'static Class> = OnceLock::new();
    CLASS.get_or_init(|| {
        let superclass = class!(UITextRange);
        let mut decl = ClassDecl::new("ZedTextRange", superclass)
            .expect("ZedTextRange already registered");
        decl.add_ivar::<usize>("_start");
        decl.add_ivar::<usize>("_end");
        unsafe {
            decl.add_method(
                sel!(isEmpty),
                text_range_is_empty as extern "C" fn(&Object, Sel) -> bool,
            );
            decl.add_method(
                sel!(start),
                text_range_start_pos as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(end),
                text_range_end_pos as extern "C" fn(&Object, Sel) -> *mut Object,
            );
        }
        decl.register()
    })
}

extern "C" fn text_range_is_empty(this: &Object, _sel: Sel) -> bool {
    unsafe {
        let start: usize = *this.get_ivar("_start");
        let end: usize = *this.get_ivar("_end");
        start == end
    }
}

extern "C" fn text_range_start_pos(this: &Object, _sel: Sel) -> *mut Object {
    unsafe {
        let start: usize = *this.get_ivar("_start");
        new_text_position(start)
    }
}

extern "C" fn text_range_end_pos(this: &Object, _sel: Sel) -> *mut Object {
    unsafe {
        let end: usize = *this.get_ivar("_end");
        new_text_position(end)
    }
}

/// Creates an autoreleased `ZedTextPosition` wrapping `offset`.
unsafe fn new_text_position(offset: usize) -> *mut Object {
    unsafe {
        let cls = register_text_position_class();
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, init];
        (*obj).set_ivar("_offset", offset);
        msg_send![obj, autorelease]
    }
}

// ── UITextInput method implementations ────────────────────────────────────────

extern "C" fn uit_insert_text(this: &Object, _sel: Sel, text: *mut Object) {
    let Some(state_rc) = state_from_view(this) else { return };
    let text_str = ns_string_to_string(text);
    state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .map(|h| h.replace_text_in_range(None, &text_str));
}

extern "C" fn uit_delete_backward(this: &Object, _sel: Sel) {
    // UIKit calls this for both software and hardware keyboard backspace.
    // pressesBegan: has already dispatched the KeyDown to GPUI's action
    // system for hardware keyboard; here we perform the actual text deletion.
    let Some(state_rc) = state_from_view(this) else { return };
    let selected = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.selected_text_range(false));
    if let Some(sel) = selected {
        let delete_range = if sel.range.start < sel.range.end {
            Some(sel.range)
        } else if sel.range.start > 0 {
            Some((sel.range.start - 1)..sel.range.start)
        } else {
            None
        };
        if let Some(range) = delete_range {
            state_rc
                .borrow_mut()
                .input_handler
                .as_mut()
                .map(|h| h.replace_text_in_range(Some(range), ""));
        }
    }
}

extern "C" fn uit_has_text(this: &Object, _sel: Sel) -> bool {
    let Some(state_rc) = state_from_view(this) else { return false };
    state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.selected_text_range(false))
        .is_some()
}

/// Distance (in logical pixels) a finger must travel before a touch is
/// reclassified from a pending tap to a drag. Matches gpui-mobile.
const TOUCH_SLOP: f32 = 8.0;

fn handle_touches(this: &Object, touches: *mut Object, kind: TouchKind) {
    let Some(state_rc) = state_from_view(this) else { return };

    let (position, tap_count) = unsafe {
        let array: *mut Object = msg_send![touches, allObjects];
        let count: usize = msg_send![array, count];
        // Multi-finger touches are handled by the UIPanGestureRecognizer.
        if count != 1 {
            return;
        }
        let touch: *mut Object = msg_send![array, objectAtIndex: 0usize];
        let view = this as *const Object as *mut Object;
        let location: CGPoint = msg_send![touch, locationInView: view];
        let tap_count: usize = msg_send![touch, tapCount];
        let position = Point {
            x: gpui::px(location.x as f32),
            y: gpui::px(location.y as f32),
        };
        (position, tap_count.max(1))
    };

    state_rc.borrow_mut().touch_position = Some(position);
    let modifiers = state_rc.borrow().current_modifiers;

    match kind {
        TouchKind::Began => {
            // Record the start position; defer MouseDown until we know whether
            // this is a tap or the beginning of a drag (or a cancelled touch
            // because a second finger landed and the pan gesture took over).
            state_rc.borrow_mut().pending_tap = Some(position);
        }
        TouchKind::Moved => {
            let pending = state_rc.borrow().pending_tap;
            if let Some(start) = pending {
                let dx = f32::from(position.x) - f32::from(start.x);
                let dy = f32::from(position.y) - f32::from(start.y);
                if dx * dx + dy * dy > TOUCH_SLOP * TOUCH_SLOP {
                    // Finger moved past slop — treat as a drag. Emit MouseDown
                    // at the start position so drag interactions (e.g. selection)
                    // work correctly, then immediately emit a MouseMove.
                    state_rc.borrow_mut().pending_tap = None;
                    dispatch_input_event(
                        &state_rc,
                        PlatformInput::MouseDown(MouseDownEvent {
                            button: MouseButton::Left,
                            position: start,
                            modifiers,
                            click_count: 1,
                            first_mouse: false,
                        }),
                    );
                    dispatch_input_event(
                        &state_rc,
                        PlatformInput::MouseMove(MouseMoveEvent {
                            position,
                            pressed_button: Some(MouseButton::Left),
                            modifiers,
                        }),
                    );
                }
                // Still within slop — not yet a drag, emit nothing.
            } else {
                // Already dragging.
                dispatch_input_event(
                    &state_rc,
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position,
                        pressed_button: Some(MouseButton::Left),
                        modifiers,
                    }),
                );
            }
        }
        TouchKind::Ended => {
            let pending = state_rc.borrow_mut().pending_tap.take();
            if let Some(tap_pos) = pending {
                // Finger lifted without leaving the slop zone: confirmed tap.
                // Emit MouseDown + MouseUp at the original touch position.
                dispatch_input_event(
                    &state_rc,
                    PlatformInput::MouseDown(MouseDownEvent {
                        button: MouseButton::Left,
                        position: tap_pos,
                        modifiers,
                        click_count: tap_count,
                        first_mouse: false,
                    }),
                );
                dispatch_input_event(
                    &state_rc,
                    PlatformInput::MouseUp(MouseUpEvent {
                        button: MouseButton::Left,
                        position: tap_pos,
                        modifiers,
                        click_count: tap_count,
                    }),
                );
                // Signal that the next input-handler focus transition should
                // show the software keyboard. This flag is consumed by
                // set_input_handler on the next draw cycle.
                state_rc.borrow_mut().show_keyboard_after_tap = true;
            } else {
                // End of a drag — emit MouseUp at current position.
                dispatch_input_event(
                    &state_rc,
                    PlatformInput::MouseUp(MouseUpEvent {
                        button: MouseButton::Left,
                        position,
                        modifiers,
                        click_count: 1,
                    }),
                );
            }
        }
        TouchKind::Cancelled => {
            // A gesture recognizer (e.g. UIPanGestureRecognizer) took over.
            // If we hadn't yet emitted MouseDown (still pending), just clear
            // state. If we were mid-drag, close it with a MouseUp.
            let was_pending = state_rc.borrow_mut().pending_tap.take().is_some();
            if !was_pending {
                dispatch_input_event(
                    &state_rc,
                    PlatformInput::MouseUp(MouseUpEvent {
                        button: MouseButton::Left,
                        position,
                        modifiers,
                        click_count: 1,
                    }),
                );
            }
        }
    }
}

// ─── Two-finger pan gesture → scroll ─────────────────────────────────────────

// UIGestureRecognizerState integer values.
const GESTURE_STATE_BEGAN: isize = 1;
const GESTURE_STATE_CHANGED: isize = 2;
const GESTURE_STATE_ENDED: isize = 3;
const GESTURE_STATE_CANCELLED: isize = 4;

extern "C" fn handle_pan_gesture(this: &Object, _sel: Sel, recognizer: *mut Object) {
    let Some(state_rc) = state_from_view(this) else { return };

    unsafe {
        let gesture_state: isize = msg_send![recognizer, state];

        let touch_phase = match gesture_state {
            GESTURE_STATE_BEGAN => TouchPhase::Started,
            GESTURE_STATE_CHANGED => TouchPhase::Moved,
            GESTURE_STATE_ENDED | GESTURE_STATE_CANCELLED => TouchPhase::Ended,
            _ => return,
        };

        let view = this as *const Object as *mut Object;

        // Accumulate incremental deltas by resetting the recognizer's translation
        // to zero after each callback so each firing gives us a delta, not total offset.
        let translation: CGPoint = msg_send![recognizer, translationInView: view];
        let zero = CGPoint { x: 0.0, y: 0.0 };
        let _: () = msg_send![recognizer, setTranslation: zero inView: view];

        let location: CGPoint = msg_send![recognizer, locationInView: view];
        let position = Point {
            x: gpui::px(location.x as f32),
            y: gpui::px(location.y as f32),
        };

        let modifiers = state_rc.borrow().current_modifiers;
        let event = PlatformInput::ScrollWheel(ScrollWheelEvent {
            position,
            delta: ScrollDelta::Pixels(Point {
                x: gpui::px(translation.x as f32),
                y: gpui::px(translation.y as f32),
            }),
            modifiers,
            touch_phase,
        });
        dispatch_input_event(&state_rc, event);
    }
}

// ─── Scroll wheel (simulator mouse wheel without pointer capture) ────────────

extern "C" fn handle_scroll_wheel(this: &Object, _sel: Sel, ui_event: *mut Object) {
    let Some(state_rc) = state_from_view(this) else { return };

    unsafe {
        let dx: f64 = msg_send![ui_event, _scrollingDeltaX];
        let dy: f64 = msg_send![ui_event, _scrollingDeltaY];

        if dx == 0.0 && dy == 0.0 {
            return;
        }

        // Fall back to view center since the simulator scroll wheel event
        // doesn't carry touch location.
        let view = this as *const Object as *mut Object;
        let bounds: CGRect = msg_send![view, bounds];
        let position = Point {
            x: gpui::px((bounds.size.width / 2.0) as f32),
            y: gpui::px((bounds.size.height / 2.0) as f32),
        };

        let modifiers = state_rc.borrow().current_modifiers;
        let event = PlatformInput::ScrollWheel(ScrollWheelEvent {
            position,
            delta: ScrollDelta::Pixels(Point {
                x: gpui::px(dx as f32),
                y: gpui::px(dy as f32),
            }),
            modifiers,
            touch_phase: TouchPhase::Moved,
        });
        dispatch_input_event(&state_rc, event);
    }
}
