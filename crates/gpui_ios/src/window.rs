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
    PlatformWindow, Pixels, Point, PromptButton, PromptLevel, RequestFrameOptions, Scene, Size,
    WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowControls,
    WindowParams,
};
use objc::{
    class, declare::ClassDecl, msg_send,
    runtime::{Class, Object, Sel},
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

// UIKeyboardHIDUsage values. These are full HID values including usage page 0x0007.
const HID_ENTER: usize = 0x00070028;
const HID_ESCAPE: usize = 0x00070029;
const HID_BACKSPACE: usize = 0x0007002A;
const HID_TAB: usize = 0x0007002B;
const HID_SPACE: usize = 0x0007002C;
const HID_F1: usize = 0x0007003A;
const HID_F12: usize = 0x00070045;
const HID_INSERT: usize = 0x00070049;
const HID_HOME: usize = 0x0007004A;
const HID_PAGE_UP: usize = 0x0007004B;
const HID_DELETE_FORWARD: usize = 0x0007004C;
const HID_END: usize = 0x0007004D;
const HID_PAGE_DOWN: usize = 0x0007004E;
const HID_RIGHT_ARROW: usize = 0x0007004F;
const HID_LEFT_ARROW: usize = 0x00070050;
const HID_DOWN_ARROW: usize = 0x00070051;
const HID_UP_ARROW: usize = 0x00070052;
// Modifier key HID codes — presses of these alone emit ModifiersChanged only.
const HID_CAPS_LOCK: usize = 0x00070039;
const HID_MOD_LEFT_CTRL: usize = 0x000700E0;
const HID_MOD_LEFT_SHIFT: usize = 0x000700E1;
const HID_MOD_LEFT_ALT: usize = 0x000700E2;
const HID_MOD_LEFT_GUI: usize = 0x000700E3;
const HID_MOD_RIGHT_CTRL: usize = 0x000700E4;
const HID_MOD_RIGHT_SHIFT: usize = 0x000700E5;
const HID_MOD_RIGHT_ALT: usize = 0x000700E6;
const HID_MOD_RIGHT_GUI: usize = 0x000700E7;

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
fn dispatch_input_event(state_rc: &Rc<RefCell<IosWindowState>>, event: PlatformInput) {
    let mut callback = state_rc.borrow_mut().callbacks.input.take();
    if let Some(ref mut f) = callback {
        f(event);
    }
    let mut state = state_rc.borrow_mut();
    if state.callbacks.input.is_none() {
        state.callbacks.input = callback;
    }
}

// ─── ZedMetalView ObjC class ──────────────────────────────────────────────────

fn register_metal_view_class() -> &'static Class {
    use std::sync::OnceLock;
    static CLASS: OnceLock<&'static Class> = OnceLock::new();
    CLASS.get_or_init(|| {
        let superclass = class!(UIView);
        let mut decl =
            ClassDecl::new("ZedMetalView", superclass).expect("ZedMetalView already registered");

        // Stores a raw pointer to `Box<Weak<RefCell<IosWindowState>>>`.
        // Set after IosWindow construction; freed in `dealloc`.
        decl.add_ivar::<*mut c_void>("_window_state");

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
                let event = if is_down {
                    PlatformInput::KeyDown(KeyDownEvent {
                        keystroke,
                        is_held: false,
                        prefer_character_input: false,
                    })
                } else {
                    PlatformInput::KeyUp(KeyUpEvent { keystroke })
                };
                dispatch_input_event(&state_rc, event);
            }
        }
    }
}

extern "C" fn presses_began(this: &Object, _sel: Sel, presses: *mut Object, _event: *mut Object) {
    handle_presses(this, presses, true)
}

extern "C" fn presses_ended(this: &Object, _sel: Sel, presses: *mut Object, _event: *mut Object) {
    handle_presses(this, presses, false)
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

enum TouchKind {
    Began,
    Moved,
    Ended,
}

fn handle_touches(this: &Object, touches: *mut Object, kind: TouchKind) {
    let Some(state_rc) = state_from_view(this) else { return };

    let (position, click_count) = unsafe {
        let array: *mut Object = msg_send![touches, allObjects];
        let count: usize = msg_send![array, count];
        if count == 0 {
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
