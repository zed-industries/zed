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

// objc::Encode implementations allow these types to appear in add_method signatures.
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
    input_handler: Option<PlatformInputHandler>,
    /// Raw pointer to the `ZedMetalView` UIView. Not retained — the view is
    /// owned by the UIWindow hierarchy and outlives this state while the scene
    /// is connected.
    ui_view: *mut Object,
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

        let logical_width = view_bounds.size.width as f32;
        let logical_height = view_bounds.size.height as f32;
        if logical_width <= 0.0 || logical_height <= 0.0 {
            return None;
        }

        let device_width = (logical_width * scale).round() as i32;
        let device_height = (logical_height * scale).round() as i32;

        let layer_ptr = self.renderer.layer_ptr();
        let _: () = msg_send![layer_ptr, setFrame: view_bounds];
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
            input_handler: None,
            ui_view,
            renderer,
            callbacks: IosWindowCallbacks::default(),
        }));

        // Store a Weak in the view's ivar. The view calls back into us via
        // `layoutSubviews`; the Weak ensures we don't access freed state if
        // the window is ever torn down before the view.
        //
        // We rely on the natural UIKit run-loop order for the initial layout:
        // `addSubview` queues a layout pass that commits in the same CATransaction
        // flush, which happens before the next vsync. Since CADisplayLink only fires
        // on vsync, `layoutSubviews` → `handle_layout` will always set a valid
        // drawable size before the first `nextDrawable` call.
        let weak: Weak<RefCell<IosWindowState>> = Rc::downgrade(&state);
        let weak_ptr = Box::into_raw(Box::new(weak)) as *mut c_void;
        unsafe {
            (*ui_view).set_ivar("_window_state", weak_ptr);
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

            let _: () = msg_send![container, addSubview: view];

            // Attach a two-finger pan gesture recognizer for scrolling.
            // Raw touchesBegan: is filtered to single-finger only so there is no overlap.
            let pan: *mut Object = msg_send![class!(UIPanGestureRecognizer), alloc];
            let pan: *mut Object =
                msg_send![pan, initWithTarget: view action: sel!(handlePanGesture:)];
            let _: () = msg_send![pan, setMinimumNumberOfTouches: 2usize];
            let _: () = msg_send![pan, setMaximumNumberOfTouches: 2usize];
            let _: () = msg_send![view, addGestureRecognizer: pan];
            // The view retains the recognizer; release the alloc/init reference.
            let _: () = msg_send![pan, release];

            // Become first responder so UIKit routes pressesBegan:/touchesBegan: to us.
            let _: bool = msg_send![view, becomeFirstResponder];

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
        // Weak storage for the UITextInputDelegate set by UIKit before keyboard sessions.
        decl.add_ivar::<*mut Object>("_input_delegate");

        // Conform to UITextInput (which subsumes UIKeyInput).
        if let Some(protocol) = Protocol::get("UITextInput") {
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
                touches_ended as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );
            // Two-finger pan gesture → ScrollWheel
            decl.add_method(
                sel!(handlePanGesture:),
                handle_pan_gesture as extern "C" fn(&Object, Sel, *mut Object),
            );

            // ── UITextInputTraits — disable autocorrect/autocap for code editing ─
            // UITextInput subsumes UITextInputTraits; UIKit reads these properties
            // during keyboard session setup. We implement only the getters; setters
            // are no-ops (UIKit doesn't call them in practice for programmatic views).
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

            // ── Responder actions (long-press context menu) ───────────────────
            // Return false for all edit actions until clipboard is wired up.
            // This prevents the system from showing a context menu with broken actions.
            decl.add_method(
                sel!(canPerformAction:withSender:),
                can_perform_action as extern "C" fn(&Object, Sel, Sel, *mut Object) -> bool,
            );

            // ── UIKeyInput (required subset of UITextInput) ───────────────────
            decl.add_method(
                sel!(insertText:),
                uit_insert_text as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(deleteBackward),
                uit_delete_backward as extern "C" fn(&Object, Sel),
            );
            decl.add_method(
                sel!(hasText),
                uit_has_text as extern "C" fn(&Object, Sel) -> bool,
            );

            // ── UITextInput — text mutation ───────────────────────────────────
            decl.add_method(
                sel!(setMarkedText:selectedRange:),
                uit_set_marked_text as extern "C" fn(&Object, Sel, *mut Object, NSRange),
            );
            decl.add_method(
                sel!(unmarkText),
                uit_unmark_text as extern "C" fn(&Object, Sel),
            );

            // ── UITextInput — text query ──────────────────────────────────────
            decl.add_method(
                sel!(textInRange:),
                uit_text_in_range as extern "C" fn(&Object, Sel, *mut Object) -> *mut Object,
            );
            decl.add_method(
                sel!(shouldChangeTextInRange:replacementText:),
                uit_should_change_text_in_range
                    as extern "C" fn(&Object, Sel, *mut Object, *mut Object) -> bool,
            );

            // ── UITextInput — selection & marking properties ──────────────────
            decl.add_method(
                sel!(selectedTextRange),
                uit_get_selected_text_range as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(setSelectedTextRange:),
                uit_set_selected_text_range as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(markedTextRange),
                uit_get_marked_text_range as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(markedTextStyle),
                uit_get_marked_text_style as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(setMarkedTextStyle:),
                uit_set_marked_text_style as extern "C" fn(&Object, Sel, *mut Object),
            );

            // ── UITextInput — document boundary ───────────────────────────────
            decl.add_method(
                sel!(beginningOfDocument),
                uit_beginning_of_document as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(endOfDocument),
                uit_end_of_document as extern "C" fn(&Object, Sel) -> *mut Object,
            );

            // ── UITextInput — position/range arithmetic ───────────────────────
            decl.add_method(
                sel!(textRangeFromPosition:toPosition:),
                uit_text_range_from_position_to_position
                    as extern "C" fn(&Object, Sel, *mut Object, *mut Object) -> *mut Object,
            );
            decl.add_method(
                sel!(positionFromPosition:offset:),
                uit_position_from_position_offset
                    as extern "C" fn(&Object, Sel, *mut Object, isize) -> *mut Object,
            );
            decl.add_method(
                sel!(positionFromPosition:inDirection:offset:),
                uit_position_from_position_in_direction_offset
                    as extern "C" fn(&Object, Sel, *mut Object, usize, isize) -> *mut Object,
            );
            decl.add_method(
                sel!(comparePosition:toPosition:),
                uit_compare_position_to_position
                    as extern "C" fn(&Object, Sel, *mut Object, *mut Object) -> isize,
            );
            decl.add_method(
                sel!(offsetFromPosition:toPosition:),
                uit_offset_from_position_to_position
                    as extern "C" fn(&Object, Sel, *mut Object, *mut Object) -> isize,
            );
            decl.add_method(
                sel!(positionWithinRange:farthestInDirection:),
                uit_position_within_range_farthest_in_direction
                    as extern "C" fn(&Object, Sel, *mut Object, usize) -> *mut Object,
            );
            decl.add_method(
                sel!(characterRangeByExtendingPosition:inDirection:),
                uit_character_range_by_extending_position_in_direction
                    as extern "C" fn(&Object, Sel, *mut Object, usize) -> *mut Object,
            );

            // ── UITextInput — writing direction ───────────────────────────────
            decl.add_method(
                sel!(baseWritingDirectionForPosition:inDirection:),
                uit_base_writing_direction_for_position
                    as extern "C" fn(&Object, Sel, *mut Object, usize) -> isize,
            );
            decl.add_method(
                sel!(setBaseWritingDirection:forRange:),
                uit_set_base_writing_direction_for_range
                    as extern "C" fn(&Object, Sel, isize, *mut Object),
            );

            // ── UITextInput — geometry ────────────────────────────────────────
            decl.add_method(
                sel!(firstRectForRange:),
                uit_first_rect_for_range as extern "C" fn(&Object, Sel, *mut Object) -> CGRect,
            );
            decl.add_method(
                sel!(caretRectForPosition:),
                uit_caret_rect_for_position
                    as extern "C" fn(&Object, Sel, *mut Object) -> CGRect,
            );
            decl.add_method(
                sel!(closestPositionToPoint:),
                uit_closest_position_to_point
                    as extern "C" fn(&Object, Sel, CGPoint) -> *mut Object,
            );
            decl.add_method(
                sel!(closestPositionToPoint:withinRange:),
                uit_closest_position_to_point_within_range
                    as extern "C" fn(&Object, Sel, CGPoint, *mut Object) -> *mut Object,
            );
            decl.add_method(
                sel!(characterRangeAtPoint:),
                uit_character_range_at_point
                    as extern "C" fn(&Object, Sel, CGPoint) -> *mut Object,
            );
            decl.add_method(
                sel!(replaceRange:withText:),
                uit_replace_range_with_text
                    as extern "C" fn(&Object, Sel, *mut Object, *mut Object),
            );

            // ── UITextInput — delegate & tokenizer ────────────────────────────
            decl.add_method(
                sel!(inputDelegate),
                uit_get_input_delegate as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(setInputDelegate:),
                uit_set_input_delegate as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(tokenizer),
                uit_get_tokenizer as extern "C" fn(&Object, Sel) -> *mut Object,
            );
            decl.add_method(
                sel!(selectionRectsForRange:),
                uit_selection_rects_for_range
                    as extern "C" fn(&Object, Sel, *mut Object) -> *mut Object,
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

extern "C" fn can_become_first_responder(_this: &Object, _sel: Sel) -> bool {
    true
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
/// events. `is_down` controls which key event is generated.
fn handle_presses(this: &Object, presses: *mut Object, is_down: bool) {
    let Some(state_rc) = state_from_view(this) else { return };

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
                log::info!(
                    "key {}: {:?} (key_char={:?})",
                    if is_down { "down" } else { "up" },
                    keystroke.key,
                    keystroke.key_char,
                );
                if is_down {
                    let key_char = keystroke.key_char.clone();
                    let key_name = keystroke.key.clone();
                    let result = dispatch_input_event(
                        &state_rc,
                        PlatformInput::KeyDown(KeyDownEvent {
                            keystroke,
                            is_held: false,
                            prefer_character_input: false,
                        }),
                    );
                    // In the simulator's "Connect Hardware Keyboard" mode UIKit
                    // sends presses via pressesBegan: only — insertText: and
                    // deleteBackward are NOT called for hardware keyboard input.
                    // Forward unhandled keys to the UITextInput methods, which
                    // already have correct implementations.
                    log::info!(
                        "hw key fallback: propagate={} default_prevented={} key_char={:?} key_name={:?}",
                        result.propagate,
                        result.default_prevented,
                        key_char,
                        key_name,
                    );
                    if result.propagate && !result.default_prevented {
                        let view = this as *const Object as *mut Object;
                        if let Some(text) = key_char {
                            let ns_text: *mut Object =
                                msg_send![class!(NSString), stringWithUTF8String: text.as_ptr() as *const std::ffi::c_char];
                            let _: () = msg_send![view, insertText: ns_text];
                        } else if key_name == "backspace" {
                            log::info!("hw key fallback: calling deleteBackward");
                            let _: () = msg_send![view, deleteBackward];
                        }
                    }
                } else {
                    dispatch_input_event(&state_rc, PlatformInput::KeyUp(KeyUpEvent { keystroke }));
                }
            }
        }
    }
}

extern "C" fn presses_began(this: &Object, _sel: Sel, presses: *mut Object, event: *mut Object) {
    handle_presses(this, presses, true);
    unsafe {
        let _: () = msg_send![super(this, class!(UIView)), pressesBegan: presses withEvent: event];
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

#[derive(PartialEq)]
enum TouchKind {
    Began,
    Moved,
    Ended,
}

// ─── UITextInput support ──────────────────────────────────────────────────────

/// `NSRange` as used by UITextInput APIs. `location == NS_NOT_FOUND` signals nil/invalid.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct NSRange {
    location: usize,
    length: usize,
}

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        // NSRange is encoded as {_NSRange=QQ} on 64-bit (two NSUInteger).
        unsafe { objc::Encoding::from_str("{_NSRange=QQ}") }
    }
}

const NS_NOT_FOUND: usize = isize::MAX as usize;

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

/// Returns the UTF-16 length of the full document by querying the input handler,
/// or 0 if no handler is set.
fn document_length(state_rc: &Rc<RefCell<IosWindowState>>) -> usize {
    let mut adjusted: Option<std::ops::Range<usize>> = None;
    let text = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.text_for_range(0..usize::MAX, &mut adjusted));
    if text.is_some() {
        adjusted.map(|r| r.end).unwrap_or(0)
    } else {
        0
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

/// Creates an autoreleased `ZedTextRange` spanning `start..end`.
unsafe fn new_text_range(start: usize, end: usize) -> *mut Object {
    unsafe {
        let cls = register_text_range_class();
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, init];
        (*obj).set_ivar("_start", start);
        (*obj).set_ivar("_end", end);
        msg_send![obj, autorelease]
    }
}

unsafe fn read_text_position(pos: *mut Object) -> usize {
    unsafe {
        if pos.is_null() {
            return 0;
        }
        *(*pos).get_ivar("_offset")
    }
}

unsafe fn read_text_range(range: *mut Object) -> (usize, usize) {
    unsafe {
        if range.is_null() {
            return (0, 0);
        }
        let start: usize = *(*range).get_ivar("_start");
        let end: usize = *(*range).get_ivar("_end");
        (start, end)
    }
}

// ── UITextInput method implementations ────────────────────────────────────────

extern "C" fn uit_insert_text(this: &Object, _sel: Sel, text: *mut Object) {
    let Some(state_rc) = state_from_view(this) else { return };
    let text_str = ns_string_to_string(text);
    log::info!("UITextInput insertText: {:?}", text_str);
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
    log::info!("UITextInput deleteBackward: entered");
    let has_handler = state_rc.borrow().input_handler.is_some();
    log::info!("UITextInput deleteBackward: has_handler={has_handler}");
    let selected = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.selected_text_range(false));
    log::info!("UITextInput deleteBackward: selected={selected:?}");
    if let Some(sel) = selected {
        let delete_range = if sel.range.start < sel.range.end {
            Some(sel.range)
        } else if sel.range.start > 0 {
            Some((sel.range.start - 1)..sel.range.start)
        } else {
            None
        };
        log::info!("UITextInput deleteBackward: delete_range={delete_range:?}");
        if let Some(range) = delete_range {
            state_rc
                .borrow_mut()
                .input_handler
                .as_mut()
                .map(|h| h.replace_text_in_range(Some(range), ""));
            log::info!("UITextInput deleteBackward: replace_text_in_range called");
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

/// `selectedTextRange` property getter — returns a `ZedTextRange*` or nil.
extern "C" fn uit_get_selected_text_range(this: &Object, _sel: Sel) -> *mut Object {
    let Some(state_rc) = state_from_view(this) else {
        return std::ptr::null_mut();
    };
    let selection = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.selected_text_range(false));
    match selection {
        Some(sel) => unsafe { new_text_range(sel.range.start, sel.range.end) },
        None => std::ptr::null_mut(),
    }
}

/// `setSelectedTextRange:` property setter — ignored for now.
extern "C" fn uit_set_selected_text_range(
    _this: &Object,
    _sel: Sel,
    _range: *mut Object,
) {
}

/// `markedTextRange` property getter — returns `ZedTextRange*` or nil.
extern "C" fn uit_get_marked_text_range(this: &Object, _sel: Sel) -> *mut Object {
    let Some(state_rc) = state_from_view(this) else {
        return std::ptr::null_mut();
    };
    let range = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.marked_text_range());
    match range {
        Some(r) => unsafe { new_text_range(r.start, r.end) },
        None => std::ptr::null_mut(),
    }
}

/// `markedTextStyle` property getter — we don't supply custom marking styles.
extern "C" fn uit_get_marked_text_style(_this: &Object, _sel: Sel) -> *mut Object {
    std::ptr::null_mut()
}

/// `setMarkedTextStyle:` property setter — no-op.
extern "C" fn uit_set_marked_text_style(_this: &Object, _sel: Sel, _style: *mut Object) {}

/// `setMarkedText:selectedRange:` — IME composition update.
/// `text` may be an `NSString` or `NSAttributedString`; we extract the plain string.
extern "C" fn uit_set_marked_text(
    this: &Object,
    _sel: Sel,
    text: *mut Object,
    selected_range: NSRange,
) {
    let Some(state_rc) = state_from_view(this) else { return };
    let text_str = unsafe {
        // text may be NSAttributedString — extract the plain string if so.
        let is_attr: bool = msg_send![text, isKindOfClass: class!(NSAttributedString)];
        if is_attr {
            let ns_str: *mut Object = msg_send![text, string];
            ns_string_to_string(ns_str)
        } else {
            ns_string_to_string(text)
        }
    };

    let new_selected = if selected_range.location == NS_NOT_FOUND {
        None
    } else {
        let start = selected_range.location;
        let end = selected_range.location + selected_range.length;
        Some(start..end)
    };

    log::info!(
        "UITextInput setMarkedText: {:?} selectedRange={:?}",
        text_str,
        new_selected
    );

    state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .map(|h| h.replace_and_mark_text_in_range(None, &text_str, new_selected));
}

extern "C" fn uit_unmark_text(this: &Object, _sel: Sel) {
    let Some(state_rc) = state_from_view(this) else { return };
    log::info!("UITextInput unmarkText");
    state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .map(|h| h.unmark_text());
}

extern "C" fn uit_beginning_of_document(_this: &Object, _sel: Sel) -> *mut Object {
    unsafe { new_text_position(0) }
}

extern "C" fn uit_end_of_document(this: &Object, _sel: Sel) -> *mut Object {
    let end = state_from_view(this)
        .map(|rc| document_length(&rc))
        .unwrap_or(0);
    unsafe { new_text_position(end) }
}

extern "C" fn uit_text_range_from_position_to_position(
    _this: &Object,
    _sel: Sel,
    from: *mut Object,
    to: *mut Object,
) -> *mut Object {
    unsafe {
        let start = read_text_position(from);
        let end = read_text_position(to);
        if start <= end {
            new_text_range(start, end)
        } else {
            new_text_range(end, start)
        }
    }
}

extern "C" fn uit_position_from_position_offset(
    this: &Object,
    _sel: Sel,
    pos: *mut Object,
    offset: isize,
) -> *mut Object {
    let doc_len = state_from_view(this).map(|rc| document_length(&rc)).unwrap_or(0);
    let base = unsafe { read_text_position(pos) } as isize;
    let new_pos = base + offset;
    if new_pos < 0 || new_pos as usize > doc_len {
        return std::ptr::null_mut();
    }
    unsafe { new_text_position(new_pos as usize) }
}

/// `positionFromPosition:inDirection:offset:` — same arithmetic as the non-direction variant.
extern "C" fn uit_position_from_position_in_direction_offset(
    this: &Object,
    _sel: Sel,
    pos: *mut Object,
    _direction: usize,
    offset: isize,
) -> *mut Object {
    uit_position_from_position_offset(this, _sel, pos, offset)
}

/// `comparePosition:toPosition:` — returns NSComparisonResult (isize).
extern "C" fn uit_compare_position_to_position(
    _this: &Object,
    _sel: Sel,
    pos: *mut Object,
    other: *mut Object,
) -> isize {
    let a = unsafe { read_text_position(pos) };
    let b = unsafe { read_text_position(other) };
    if a < b { -1 } else if a > b { 1 } else { 0 }
}

/// `offsetFromPosition:toPosition:` — returns signed distance (NSInteger).
extern "C" fn uit_offset_from_position_to_position(
    _this: &Object,
    _sel: Sel,
    from: *mut Object,
    to: *mut Object,
) -> isize {
    let a = unsafe { read_text_position(from) } as isize;
    let b = unsafe { read_text_position(to) } as isize;
    b - a
}

/// `positionWithinRange:farthestInDirection:` — returns start or end of range.
extern "C" fn uit_position_within_range_farthest_in_direction(
    _this: &Object,
    _sel: Sel,
    range: *mut Object,
    direction: usize,
) -> *mut Object {
    let (start, end) = unsafe { read_text_range(range) };
    // UITextLayoutDirectionLeft = 3, UITextLayoutDirectionUp = 4 → start
    // UITextLayoutDirectionRight = 2, UITextLayoutDirectionDown = 5 → end
    let use_start = matches!(direction, 3 | 4);
    unsafe { new_text_position(if use_start { start } else { end }) }
}

/// `characterRangeByExtendingPosition:inDirection:` — returns a single-character range.
extern "C" fn uit_character_range_by_extending_position_in_direction(
    _this: &Object,
    _sel: Sel,
    pos: *mut Object,
    _direction: usize,
) -> *mut Object {
    let offset = unsafe { read_text_position(pos) };
    unsafe { new_text_range(offset, offset) }
}

/// `baseWritingDirectionForPosition:inDirection:` — always LTR.
extern "C" fn uit_base_writing_direction_for_position(
    _this: &Object,
    _sel: Sel,
    _pos: *mut Object,
    _direction: usize,
) -> isize {
    0 // NSWritingDirectionLeftToRight
}

/// `setBaseWritingDirection:forRange:` — no-op.
extern "C" fn uit_set_base_writing_direction_for_range(
    _this: &Object,
    _sel: Sel,
    _direction: isize,
    _range: *mut Object,
) {
}

/// `firstRectForRange:` — returns the pixel rect for the given text range.
extern "C" fn uit_first_rect_for_range(
    this: &Object,
    _sel: Sel,
    range: *mut Object,
) -> CGRect {
    let Some(state_rc) = state_from_view(this) else {
        return CGRect::default();
    };
    let (start, end) = unsafe { read_text_range(range) };
    let bounds = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.bounds_for_range(start..end));
    match bounds {
        Some(b) => CGRect {
            origin: CGPoint {
                x: f32::from(b.origin.x) as f64,
                y: f32::from(b.origin.y) as f64,
            },
            size: CGSize {
                width: f32::from(b.size.width) as f64,
                height: f32::from(b.size.height) as f64,
            },
        },
        None => CGRect::default(),
    }
}

/// `caretRectForPosition:` — stub; returns zero rect.
extern "C" fn uit_caret_rect_for_position(
    _this: &Object,
    _sel: Sel,
    _pos: *mut Object,
) -> CGRect {
    CGRect::default()
}

/// `closestPositionToPoint:` — returns the beginning of document as a stub.
extern "C" fn uit_closest_position_to_point(
    _this: &Object,
    _sel: Sel,
    _point: CGPoint,
) -> *mut Object {
    unsafe { new_text_position(0) }
}

/// `closestPositionToPoint:withinRange:` — returns start of the given range.
extern "C" fn uit_closest_position_to_point_within_range(
    _this: &Object,
    _sel: Sel,
    _point: CGPoint,
    range: *mut Object,
) -> *mut Object {
    let (start, _) = unsafe { read_text_range(range) };
    unsafe { new_text_position(start) }
}

/// `characterRangeAtPoint:` — stub; returns nil.
extern "C" fn uit_character_range_at_point(
    _this: &Object,
    _sel: Sel,
    _point: CGPoint,
) -> *mut Object {
    std::ptr::null_mut()
}

/// `textInRange:` — returns the text for the given range.
extern "C" fn uit_text_in_range(this: &Object, _sel: Sel, range: *mut Object) -> *mut Object {
    let Some(state_rc) = state_from_view(this) else {
        return std::ptr::null_mut();
    };
    let (start, end) = unsafe { read_text_range(range) };
    let mut adjusted = None;
    let text = state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .and_then(|h| h.text_for_range(start..end, &mut adjusted));
    match text {
        Some(s) => unsafe {
            let ns: *mut Object = msg_send![class!(NSString), alloc];
            let ns: *mut Object =
                msg_send![ns, initWithBytes: s.as_ptr() length: s.len() encoding: 4u32]; // NSUTF8StringEncoding = 4
            msg_send![ns, autorelease]
        },
        None => std::ptr::null_mut(),
    }
}

/// `shouldChangeTextInRange:replacementText:` — always allow.
extern "C" fn uit_should_change_text_in_range(
    _this: &Object,
    _sel: Sel,
    _range: *mut Object,
    _replacement: *mut Object,
) -> bool {
    true
}

/// `replaceRange:withText:` — called by autocorrect and predictive text.
extern "C" fn uit_replace_range_with_text(
    this: &Object,
    _sel: Sel,
    range: *mut Object,
    text: *mut Object,
) {
    let Some(state_rc) = state_from_view(this) else { return };
    let (start, end) = unsafe { read_text_range(range) };
    let text_str = ns_string_to_string(text);
    state_rc
        .borrow_mut()
        .input_handler
        .as_mut()
        .map(|h| h.replace_text_in_range(Some(start..end), &text_str));
}

/// `inputDelegate` property getter.
extern "C" fn uit_get_input_delegate(this: &Object, _sel: Sel) -> *mut Object {
    unsafe { *this.get_ivar("_input_delegate") }
}

/// `setInputDelegate:` property setter. UIKit calls this before/after keyboard sessions.
extern "C" fn uit_set_input_delegate(this: &Object, _sel: Sel, delegate: *mut Object) {
    unsafe {
        let this_mut = this as *const Object as *mut Object;
        (*this_mut).set_ivar("_input_delegate", delegate);
    }
}

/// `tokenizer` property getter — returns a `UITextInputStringTokenizer`.
extern "C" fn uit_get_tokenizer(this: &Object, _sel: Sel) -> *mut Object {
    unsafe {
        let view = this as *const Object as *mut Object;
        let tok: *mut Object = msg_send![class!(UITextInputStringTokenizer), alloc];
        let tok: *mut Object = msg_send![tok, initWithTextInput: view];
        msg_send![tok, autorelease]
    }
}

/// `selectionRectsForRange:` — returns an empty array.
/// UIKit calls this to draw selection highlight rects. Returning an empty
/// array prevents the "unrecognized selector" crash; highlights just won't appear.
extern "C" fn uit_selection_rects_for_range(
    _this: &Object,
    _sel: Sel,
    _range: *mut Object,
) -> *mut Object {
    unsafe { msg_send![class!(NSArray), array] }
}

fn handle_touches(this: &Object, touches: *mut Object, kind: TouchKind) {
    let Some(state_rc) = state_from_view(this) else { return };

    let (position, click_count) = unsafe {
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

    // On touch began, reclaim first responder if the view lost it (e.g. after
    // On touch began, reclaim first responder if lost (e.g. after keyboard
    // dismissal or simulator hardware-keyboard toggle). becomeFirstResponder
    // is a no-op when the view is already first responder.
    if kind == TouchKind::Began {
        let view = state_rc.borrow().ui_view;
        unsafe {
            let _: bool = msg_send![view, becomeFirstResponder];
        }
    }

    let modifiers = state_rc.borrow().current_modifiers;
    log::info!(
        "touch {:?}: ({:.1}, {:.1}) clicks={}",
        match kind {
            TouchKind::Began => "began",
            TouchKind::Moved => "moved",
            TouchKind::Ended => "ended",
        },
        f32::from(position.x),
        f32::from(position.y),
        click_count,
    );
    let event = match kind {
        TouchKind::Began => PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count,
            first_mouse: false,
        }),
        TouchKind::Moved => PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: Some(MouseButton::Left),
            modifiers,
        }),
        TouchKind::Ended => PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count,
        }),
    };
    dispatch_input_event(&state_rc, event);
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
