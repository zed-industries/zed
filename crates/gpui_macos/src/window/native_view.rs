//! Composites platform views beneath a window's GPUI content.
//!
//! GPUI's focus decides which view receives keyboard input: the window's first
//! responder is the native view whose handle GPUI has focused, and GPUI's own
//! view otherwise. The overrides here keep AppKit from moving it anywhere else,
//! except when the user clicks into a native view.

use super::{
    MacWindowState, convert_mouse_position, current_capslock, current_modifiers, get_window_state,
    handle_key_event, with_input_handler,
};
use gpui::{
    ModifiersChangedEvent, NativeViewId, Pixels, PlatformInput, Point, Rgba, ScaledPixels, Scene,
    WindowBackgroundAppearance,
};
use objc2::{
    class, msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel},
    sel,
};
use objc2_app_kit::{
    NSColor, NSEventModifierFlags, NSEventType, NSView, NSWindowOrderingMode, NSWorkspace,
};
use objc2_foundation::{NSComparisonResult, NSPoint, NSRect, NSSize};
use parking_lot::Mutex;
use std::{
    any::Any,
    ffi::c_void,
    ptr::{self, NonNull},
    rc::{Rc, Weak},
};

type HitTestCallback = Box<dyn FnMut(Point<Pixels>) -> Option<NativeViewId>>;

#[derive(Default)]
pub(super) struct NativeViews {
    views: Vec<AttachedView>,
    /// The view GPUI has focused, which is the one that should be first responder.
    focused: Option<NativeViewId>,
    /// The view a mouse-down event is being delivered to, which may take
    /// first responder even though GPUI has not focused it yet.
    clicked: Option<NativeViewId>,
    /// Set while the window offers a key equivalent to GPUI ahead of the
    /// native first responder, so the view hierarchy doesn't offer it again.
    offered_key_equivalent: bool,
    /// The views the last scene showed, from back to front.
    stacking: Vec<NativeViewId>,
    /// The color the window was last given behind its native views.
    pub(super) background: Option<Rgba>,
    pub(super) hit_test_callback: Option<HitTestCallback>,
    pub(super) focused_callback: Option<Box<dyn FnMut(NativeViewId)>>,
}

struct AttachedView {
    id: NativeViewId,
    view: Retained<NSView>,
    /// The frame the view was last given, or `None` while it is hidden.
    frame: Option<[f64; 4]>,
}

impl NativeViews {
    fn shown(&self, id: NativeViewId) -> Option<&AttachedView> {
        self.views
            .iter()
            .find(|attached| attached.id == id && attached.frame.is_some())
    }

    fn is_showing_any(&self) -> bool {
        self.views.iter().any(|attached| attached.frame.is_some())
    }

    /// The attached view that contains `responder`, if it is a view inside one.
    fn containing(&self, responder: *mut AnyObject) -> Option<NativeViewId> {
        if responder.is_null() {
            return None;
        }
        // SAFETY: `responder` is a live object AppKit passed as a responder, and the
        // attached views are retained.
        unsafe {
            let is_view: bool = msg_send![responder, isKindOfClass: class!(NSView)];
            if !is_view {
                return None;
            }
            self.views.iter().find_map(|attached| {
                let inside: bool = msg_send![responder, isDescendantOf: &*attached.view];
                inside.then_some(attached.id)
            })
        }
    }
}

struct Attachment {
    window_state: Weak<Mutex<MacWindowState>>,
    id: NativeViewId,
}

impl Drop for Attachment {
    fn drop(&mut self) {
        let Some(window_state) = self.window_state.upgrade() else {
            return;
        };
        let (attached, native_window, gpui_view) = {
            let mut lock = window_state.lock();
            let Some(index) = lock
                .native_views
                .views
                .iter()
                .position(|attached| attached.id == self.id)
            else {
                return;
            };
            if lock.native_views.focused == Some(self.id) {
                lock.native_views.focused = None;
            }
            (
                lock.native_views.views.remove(index),
                lock.native_window,
                lock.native_view.as_ptr(),
            )
        };
        // SAFETY: The window and both views are live, and attachments are dropped on
        // the main thread because `gpui::NativeView` is not `Send`.
        unsafe {
            // AppKit would otherwise hand first responder to the window itself.
            give_up_first_responder(native_window, gpui_view, &attached.view);
            attached.view.removeFromSuperview();
        }
    }
}

/// Adds `view` beneath the window's GPUI view, hidden until a scene shows it.
///
/// # Safety
///
/// `view` must point to a live `NSView` that is not in another view hierarchy,
/// and this must be called on the main thread.
pub(super) unsafe fn attach(
    window_state: &Rc<Mutex<MacWindowState>>,
    id: NativeViewId,
    view: NonNull<c_void>,
) -> Option<Box<dyn Any>> {
    // SAFETY: The caller guarantees `view` is a live `NSView`.
    let view = unsafe { Retained::retain(view.as_ptr().cast::<NSView>()) }?;
    let mut lock = window_state.lock();
    let gpui_view = lock.native_view.as_ptr();
    view.setHidden(true);
    // SAFETY: The GPUI view is live and in the window's content view.
    unsafe {
        let content_view: *mut AnyObject = msg_send![gpui_view, superview];
        let _: () = msg_send![
            content_view,
            addSubview: &*view,
            positioned: NSWindowOrderingMode::Below,
            relativeTo: gpui_view
        ];
    }
    lock.native_views.views.push(AttachedView {
        id,
        view,
        frame: None,
    });
    Some(Box::new(Attachment {
        window_state: Rc::downgrade(window_state),
        id,
    }))
}

/// Shows the scene's native view background behind the window's content.
///
/// A view's frame change reaches the screen a frame or two before the scene
/// that moves its opening, so the opening briefly uncovers whatever is behind
/// the views. That is the window's background, which is given the color the
/// scene asks for. Presenting scenes in step with AppKit would close the gap,
/// but costs every frame of a window that shows a native view.
pub(super) fn update_background(state: &mut MacWindowState, scene: &Scene) {
    // The background of a transparent or blurred window is meant to show through.
    if state.native_views.views.is_empty()
        || state.background_appearance != WindowBackgroundAppearance::Opaque
    {
        return;
    }
    let background = scene
        .native_views
        .iter()
        .rev()
        .map(|native_view| Rgba::from(native_view.background))
        .find(|background| background.a > 0.);
    if state.native_views.background == background {
        return;
    }
    state.native_views.background = background;
    let Rgba { r, g, b, .. } = background.unwrap_or_default();
    let color = NSColor::colorWithSRGBRed_green_blue_alpha(r as f64, g as f64, b as f64, 1.);
    // SAFETY: The window is live for as long as its state.
    unsafe {
        let _: () = msg_send![state.native_window, setBackgroundColor: &*color];
    }
}

/// What a scene changes about the window's attached views.
#[derive(Default)]
pub(super) struct Layout {
    /// Views to move to a new frame, or to hide for `None`.
    frames: Vec<(NativeViewId, Retained<NSView>, Option<[f64; 4]>)>,
    /// The views the scene shows, from back to front, if that order changed.
    stacking: Option<Vec<NativeViewId>>,
}

/// Works out which attached views the scene moves, shows, hides, or restacks.
pub(super) fn plan_layout(state: &MacWindowState, scene: &Scene) -> Layout {
    if state.native_views.views.is_empty() {
        return Layout::default();
    }
    let scale_factor = state.scale_factor() as f64;
    // SAFETY: The GPUI view is live for as long as the window state.
    let gpui_frame: NSRect = unsafe { msg_send![state.native_view.as_ptr(), frame] };
    let to_points = |value: ScaledPixels| f64::from(value) / scale_factor;

    let mut frames = Vec::new();
    for attached in &state.native_views.views {
        let placement = scene
            .native_views
            .iter()
            .find(|placement| placement.id == attached.id)
            .map(|placement| {
                let bounds = placement.bounds;
                let height = to_points(bounds.size.height);
                [
                    gpui_frame.origin.x + to_points(bounds.origin.x),
                    // AppKit measures from the bottom of the content view.
                    gpui_frame.origin.y + gpui_frame.size.height
                        - to_points(bounds.origin.y)
                        - height,
                    to_points(bounds.size.width),
                    height,
                ]
            });
        if attached.frame != placement {
            frames.push((attached.id, attached.view.clone(), placement));
        }
    }

    // The scene's native views are sorted by draw order, so the ones painted
    // later, which belong in front, come last.
    let mut stacking = Vec::new();
    for placement in &scene.native_views {
        if !stacking.contains(&placement.id) {
            stacking.push(placement.id);
        }
    }
    let stacking = (stacking != state.native_views.stacking).then_some(stacking);
    Layout { frames, stacking }
}

/// Applies a planned layout. The window state must be unlocked, because AppKit
/// calls back into it when a view's frame or first responder changes.
pub(super) fn apply_layout(window_state: &Rc<Mutex<MacWindowState>>, layout: Layout) {
    if layout.frames.is_empty() && layout.stacking.is_none() {
        return;
    }
    let (native_window, gpui_view, stacking) = {
        let mut lock = window_state.lock();
        for (id, _, frame) in &layout.frames {
            if let Some(attached) = lock
                .native_views
                .views
                .iter_mut()
                .find(|attached| attached.id == *id)
            {
                attached.frame = *frame;
            }
        }
        let stacking = layout.stacking.map(|stacking| {
            let front_to_back = stacking
                .iter()
                .rev()
                .filter_map(|id| lock.native_views.views.iter().find(|view| view.id == *id))
                .map(|attached| Retained::as_ptr(&attached.view))
                .collect();
            let attached = lock
                .native_views
                .views
                .iter()
                .map(|attached| Retained::as_ptr(&attached.view))
                .collect();
            lock.native_views.stacking = stacking;
            Stacking {
                gpui_view: lock.native_view.as_ptr().cast_const().cast(),
                front_to_back,
                attached,
            }
        });
        (lock.native_window, lock.native_view.as_ptr(), stacking)
    };
    for (_, view, frame) in layout.frames {
        match frame {
            Some([x, y, width, height]) => {
                view.setFrame(NSRect::new(NSPoint::new(x, y), NSSize::new(width, height)));
                view.setHidden(false);
            }
            None => {
                // SAFETY: The window and both views are live, and drawing runs on the main thread.
                unsafe { give_up_first_responder(native_window, gpui_view, &view) };
                view.setHidden(true);
            }
        }
    }
    if let Some(mut stacking) = stacking
        && stacking.front_to_back.len() > 1
    {
        // SAFETY: The GPUI view is live and in the content view, the comparison
        // function only reads `stacking`, which outlives the call, and sorting
        // reorders the subviews in place, so none of them leaves the window.
        unsafe {
            let content_view: *mut NSView = msg_send![gpui_view, superview];
            if let Some(content_view) = content_view.as_ref() {
                content_view.sortSubviewsUsingFunction_context(
                    compare_stacking,
                    (&raw mut stacking).cast(),
                );
            }
        }
    }
    // A view that GPUI focused while it was hidden can take keyboard input now.
    reconcile_first_responder(window_state);
}

/// The order to give the content view's subviews.
struct Stacking {
    gpui_view: *const NSView,
    /// The views the scene shows, starting with the one painted last.
    front_to_back: Vec<*const NSView>,
    attached: Vec<*const NSView>,
}

impl Stacking {
    /// How far forward a subview belongs, with greater values in front.
    fn rank(&self, view: *const NSView) -> usize {
        if view == self.gpui_view {
            usize::MAX
        } else if let Some(index) = self.front_to_back.iter().position(|shown| *shown == view) {
            // Shown views sit above hidden ones, in the scene's order.
            2 + self.front_to_back.len() - index
        } else if self.attached.contains(&view) {
            1
        } else {
            // Views that aren't native views, such as the blur behind a
            // translucent window, stay at the back.
            0
        }
    }
}

unsafe extern "C-unwind" fn compare_stacking(
    first: NonNull<NSView>,
    second: NonNull<NSView>,
    stacking: *mut c_void,
) -> NSComparisonResult {
    // SAFETY: `apply_layout` passes a `Stacking` that outlives the sort.
    let stacking = unsafe { &*stacking.cast::<Stacking>() };
    match stacking
        .rank(first.as_ptr())
        .cmp(&stacking.rank(second.as_ptr()))
    {
        std::cmp::Ordering::Less => NSComparisonResult::Ascending,
        std::cmp::Ordering::Equal => NSComparisonResult::Same,
        std::cmp::Ordering::Greater => NSComparisonResult::Descending,
    }
}

/// Records the view GPUI has focused and gives it keyboard input.
pub(super) fn set_focused(window_state: &Rc<Mutex<MacWindowState>>, id: Option<NativeViewId>) {
    window_state.lock().native_views.focused = id;
    reconcile_first_responder(window_state);
}

/// Makes the window's first responder match the view GPUI has focused.
pub(super) fn reconcile_first_responder(window_state: &Rc<Mutex<MacWindowState>>) {
    let (native_window, gpui_view, focused_view) = {
        let lock = window_state.lock();
        let focused_view = lock
            .native_views
            .focused
            .and_then(|id| lock.native_views.shown(id))
            .map(|attached| attached.view.clone());
        (lock.native_window, lock.native_view.as_ptr(), focused_view)
    };
    // SAFETY: The window and views are live, and this runs on the main thread. The
    // window state is unlocked because AppKit calls back into it synchronously.
    unsafe {
        let current: *mut AnyObject = msg_send![native_window, firstResponder];
        match focused_view {
            Some(view) => {
                let is_view: bool =
                    !current.is_null() && msg_send![current, isKindOfClass: class!(NSView)];
                let inside: bool = is_view && msg_send![current, isDescendantOf: &*view];
                if !inside {
                    let _: Bool = msg_send![native_window, makeFirstResponder: &*view];
                }
            }
            None => {
                if current != gpui_view {
                    let _: Bool = msg_send![native_window, makeFirstResponder: gpui_view];
                }
            }
        }
    }
}

/// Returns first responder to the GPUI view if it is inside `view`.
///
/// # Safety
///
/// All three objects must be live, and this must be called on the main thread
/// without the window state locked.
unsafe fn give_up_first_responder(
    native_window: *mut AnyObject,
    gpui_view: *mut AnyObject,
    view: &NSView,
) {
    unsafe {
        let current: *mut AnyObject = msg_send![native_window, firstResponder];
        let is_view: bool = !current.is_null() && msg_send![current, isKindOfClass: class!(NSView)];
        let inside: bool = is_view && msg_send![current, isDescendantOf: view];
        if inside {
            let _: Bool = msg_send![native_window, makeFirstResponder: gpui_view];
        }
    }
}

pub(super) fn add_window_methods(decl: &mut ClassBuilder) {
    // SAFETY: The function signatures match the selectors' Objective-C signatures.
    unsafe {
        decl.add_method(
            sel!(makeFirstResponder:),
            make_first_responder as extern "C" fn(_, Sel, *mut AnyObject) -> Bool,
        );
        decl.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(_, Sel, *mut AnyObject),
        );
        decl.add_method(
            sel!(performKeyEquivalent:),
            window_perform_key_equivalent as extern "C" fn(_, Sel, *mut AnyObject) -> Bool,
        );
    }
}

pub(super) fn add_view_methods(decl: &mut ClassBuilder) {
    // SAFETY: The function signatures match the selectors' Objective-C signatures.
    unsafe {
        decl.add_method(
            sel!(hitTest:),
            hit_test as extern "C" fn(_, Sel, NSPoint) -> *mut AnyObject,
        );
        decl.add_method(
            sel!(becomeFirstResponder),
            become_first_responder as extern "C" fn(_, Sel) -> Bool,
        );
        decl.add_method(
            sel!(resignFirstResponder),
            resign_first_responder as extern "C" fn(_, Sel) -> Bool,
        );
    }
}

/// GPUI windows subclass either `NSWindow` or `NSPanel`.
fn window_superclass(this: &AnyObject) -> &'static AnyClass {
    // SAFETY: `this` is a live window.
    let is_panel: bool = unsafe { msg_send![this, isKindOfClass: class!(NSPanel)] };
    if is_panel {
        class!(NSPanel)
    } else {
        class!(NSWindow)
    }
}

/// Asks GPUI which native view, if any, a point in window coordinates reaches.
fn hit_test_native_view(
    window_state: &Rc<Mutex<MacWindowState>>,
    position_in_window: NSPoint,
) -> Option<NativeViewId> {
    // AppKit can hit-test while the state is locked for another call into it.
    let mut lock = window_state.try_lock()?;
    if !lock.native_views.is_showing_any() {
        return None;
    }
    let position = convert_mouse_position(position_in_window, lock.content_size().height);
    let mut callback = lock.native_views.hit_test_callback.take()?;
    drop(lock);
    let id = callback(position);
    let mut lock = window_state.lock();
    lock.native_views.hit_test_callback = Some(callback);
    id.filter(|id| lock.native_views.shown(*id).is_some())
}

/// Lets mouse events that GPUI routes to a native view fall through to it.
extern "C" fn hit_test(this: &AnyObject, _: Sel, point: NSPoint) -> *mut AnyObject {
    // SAFETY: `this` is the live GPUI view, whose state outlives it.
    unsafe {
        let window_state = get_window_state(this);
        let superview: *mut AnyObject = msg_send![this, superview];
        let position_in_window: NSPoint =
            msg_send![superview, convertPoint: point, toView: ptr::null::<AnyObject>()];
        if hit_test_native_view(&window_state, position_in_window).is_some() {
            return ptr::null_mut();
        }
        msg_send![super(this, class!(NSView)), hitTest: point]
    }
}

/// Notes which native view a mouse-down is headed for while AppKit delivers it,
/// which is the only time a native view may take first responder on its own.
extern "C" fn send_event(this: &AnyObject, _: Sel, event: *mut AnyObject) {
    // SAFETY: `this` is a live GPUI window and `event` a live `NSEvent`.
    unsafe {
        let window_state = get_window_state(this);
        let event_type: NSEventType = msg_send![event, type];
        let clicked = if matches!(
            event_type,
            NSEventType::LeftMouseDown | NSEventType::RightMouseDown | NSEventType::OtherMouseDown
        ) {
            let position: NSPoint = msg_send![event, locationInWindow];
            hit_test_native_view(&window_state, position)
        } else {
            None
        };
        if clicked.is_none() {
            return msg_send![super(this, window_superclass(this)), sendEvent: event];
        }
        window_state.lock().native_views.clicked = clicked;
        let _: () = msg_send![super(this, window_superclass(this)), sendEvent: event];
        window_state.lock().native_views.clicked = None;
    }
}

extern "C" fn make_first_responder(this: &AnyObject, _: Sel, responder: *mut AnyObject) -> Bool {
    // SAFETY: `this` is a live GPUI window and `responder` is null or a live responder.
    unsafe {
        let window_state = get_window_state(this);
        let superclass = window_superclass(this);
        // The state is locked while a window is still being set up, before it has native views.
        let Some(lock) = window_state.try_lock() else {
            return msg_send![super(this, superclass), makeFirstResponder: responder];
        };
        if lock.native_views.views.is_empty() {
            drop(lock);
            return msg_send![super(this, superclass), makeFirstResponder: responder];
        }
        let requested = lock.native_views.containing(responder);
        let focused = lock.native_views.focused;
        let clicked = lock.native_views.clicked;
        drop(lock);

        // Web views ask for first responder on their own, such as when a page script
        // focuses its window. Only GPUI's focus, a click, or VoiceOver's cursor may move it.
        let voice_over_enabled: bool =
            msg_send![&*NSWorkspace::sharedWorkspace(), isVoiceOverEnabled];
        if let Some(requested) = requested
            && focused != Some(requested)
            && clicked != Some(requested)
            && !voice_over_enabled
        {
            return Bool::NO;
        }

        let accepted: Bool = msg_send![super(this, superclass), makeFirstResponder: responder];
        if !accepted.as_bool() {
            return accepted;
        }

        match requested {
            Some(requested) if focused != Some(requested) => {
                let mut lock = window_state.lock();
                lock.native_views.focused = Some(requested);
                if let Some(mut callback) = lock.native_views.focused_callback.take() {
                    drop(lock);
                    callback(requested);
                    window_state.lock().native_views.focused_callback = Some(callback);
                }
            }
            // Anything else that moved first responder away from the view GPUI has
            // focused is undone, such as AppKit handing it to the window itself, or
            // a full screen transition restoring it to the GPUI view.
            _ => reconcile_first_responder(&window_state),
        }
        accepted
    }
}

/// Offers shortcuts to GPUI before a native first responder, which would
/// otherwise claim every key equivalent for itself.
extern "C" fn window_perform_key_equivalent(
    this: &AnyObject,
    _: Sel,
    event: *mut AnyObject,
) -> Bool {
    // SAFETY: `this` is a live GPUI window and `event` a live `NSEvent`.
    unsafe {
        let window_state = get_window_state(this);
        let superclass = window_superclass(this);
        let gpui_view = {
            let Some(lock) = window_state.try_lock() else {
                return msg_send![super(this, superclass), performKeyEquivalent: event];
            };
            let first_responder: *mut AnyObject = msg_send![this, firstResponder];
            let modifiers: NSEventModifierFlags = msg_send![event, modifierFlags];
            let is_shortcut =
                modifiers.intersects(NSEventModifierFlags::Command | NSEventModifierFlags::Control);
            if !is_shortcut || lock.native_views.containing(first_responder).is_none() {
                drop(lock);
                return msg_send![super(this, superclass), performKeyEquivalent: event];
            }
            lock.native_view.as_ptr()
        };

        if handle_key_event(&*gpui_view, event, true).as_bool() {
            return Bool::YES;
        }
        window_state.lock().native_views.offered_key_equivalent = true;
        let handled: Bool = msg_send![super(this, superclass), performKeyEquivalent: event];
        window_state.lock().native_views.offered_key_equivalent = false;
        handled
    }
}

/// Whether the window already offered the current key equivalent to GPUI.
pub(super) fn already_offered_key_equivalent(window_state: &Rc<Mutex<MacWindowState>>) -> bool {
    window_state
        .try_lock()
        .is_some_and(|lock| lock.native_views.offered_key_equivalent)
}

/// Whether assistive technology should treat the GPUI view as focused, which
/// it isn't while a native view has keyboard input.
pub(super) fn gpui_view_is_focused(state: &MacWindowState) -> bool {
    let native_window = state.native_window();
    if !native_window.isKeyWindow() {
        return false;
    }
    if state.native_views.views.is_empty() {
        return true;
    }
    // SAFETY: The window is live for as long as its state.
    let first_responder: *mut AnyObject = unsafe { msg_send![state.native_window, firstResponder] };
    first_responder == state.native_view.as_ptr()
}

/// Tells the accessibility adapter whether the GPUI view is first responder,
/// which the window does not report correctly while that is changing.
fn update_accessibility_focus(window_state: &Rc<Mutex<MacWindowState>>, is_first_responder: bool) {
    let events = {
        let Some(mut lock) = window_state.try_lock() else {
            return;
        };
        let is_focused = is_first_responder && lock.native_window().isKeyWindow();
        lock.accesskit_adapter
            .as_mut()
            .and_then(|adapter| adapter.update_view_focus_state(is_focused))
    };
    if let Some(events) = events {
        events.raise();
    }
}

extern "C" fn become_first_responder(this: &AnyObject, _: Sel) -> Bool {
    // SAFETY: `this` is the live GPUI view, whose state outlives it.
    unsafe {
        let accepted: Bool = msg_send![super(this, class!(NSView)), becomeFirstResponder];
        let window_state = get_window_state(this);
        // The state is locked while the window is still being set up.
        let Some(lock) = window_state.try_lock() else {
            return accepted;
        };
        if !accepted.as_bool() || lock.native_views.views.is_empty() {
            return accepted;
        }
        // A native first responder consumes modifier changes, so GPUI may have missed
        // some. GPUI can be in the middle of an update here, so report them afterwards.
        let executor = lock.foreground_executor.clone();
        drop(lock);
        update_accessibility_focus(&window_state, true);
        executor
            .spawn(async move {
                let event = PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                    modifiers: current_modifiers(),
                    capslock: current_capslock(),
                });
                let mut lock = window_state.lock();
                let unchanged = matches!(
                    (&lock.previous_modifiers_changed_event, &event),
                    (
                        Some(PlatformInput::ModifiersChanged(previous)),
                        PlatformInput::ModifiersChanged(current),
                    ) if previous.modifiers == current.modifiers
                        && previous.capslock == current.capslock
                );
                if unchanged {
                    return;
                }
                lock.previous_modifiers_changed_event = Some(event.clone());
                if let Some(mut callback) = lock.event_callback.take() {
                    drop(lock);
                    callback(event);
                    window_state.lock().event_callback = Some(callback);
                }
            })
            .detach();
        accepted
    }
}

extern "C" fn resign_first_responder(this: &AnyObject, _: Sel) -> Bool {
    // SAFETY: `this` is the live GPUI view, whose state outlives it.
    unsafe {
        let accepted: Bool = msg_send![super(this, class!(NSView)), resignFirstResponder];
        let window_state = get_window_state(this);
        let has_native_views = window_state
            .try_lock()
            .is_some_and(|lock| !lock.native_views.views.is_empty());
        if accepted.as_bool() && has_native_views {
            // Keyboard input is leaving GPUI, so an unfinished composition ends here,
            // as it would in a native text view.
            with_input_handler(this, |input_handler| input_handler.unmark_text());
            let input_context: *mut AnyObject = msg_send![this, inputContext];
            let _: () = msg_send![input_context, discardMarkedText];
            update_accessibility_focus(&window_state, false);
        }
        accepted
    }
}
