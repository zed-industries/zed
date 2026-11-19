//! Composites platform views beneath a window's GPUI content.
//!
//! GPUI's focus decides which view receives keyboard input: the window's first
//! responder is the native view whose handle GPUI has focused, and GPUI's own
//! view otherwise. The overrides here keep AppKit from moving it anywhere else,
//! except when the user clicks into a native view.
//!
//! A native view with keyboard input sees each key press first. GPUI is only
//! offered a key ahead of it for a pending or reserved binding, and otherwise
//! receives the keys that no view used, as AppKit passes them back up.

use super::{
    MacWindowState, convert_mouse_position, current_capslock, current_modifiers, get_window_state,
    window_state_if_set, with_input_handler,
};
use crate::events::platform_input_from_native;
use gpui::{
    Bounds, KeyDownEvent, ModifiersChangedEvent, NativeViewId, Pixels, PlatformInput, Point, Rgba,
    ScaledPixels, Scene, WindowBackgroundAppearance,
};
use objc2::{
    class, msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel},
    sel,
};
use objc2_app_kit::{NSColor, NSEventType, NSView, NSWindowOrderingMode, NSWorkspace};
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
    /// Set while the window routes a key equivalent between a native view and
    /// GPUI itself, so the GPUI view doesn't also handle it.
    routing_key_equivalent: bool,
    /// Set while a key press is delivered to a native view, which can ask the
    /// window to perform the same key as an equivalent before it has decided
    /// whether to use it.
    delivering_key_down: bool,
    /// The key equivalent GPUI was already offered once no view performed it.
    /// AppKit delivers it again as a key press, which must not reach GPUI twice.
    passed_up_key_equivalent: Option<Retained<AnyObject>>,
    /// The views the last scene showed, from back to front.
    stacking: Vec<NativeViewId>,
    /// The color the window was last given behind its native views.
    pub(super) background: Option<Rgba>,
    pub(super) hit_test_callback: Option<HitTestCallback>,
    pub(super) focused_callback: Option<Box<dyn FnMut(NativeViewId)>>,
    pub(super) key_down_before_callback: Option<Box<dyn FnMut(KeyDownEvent) -> bool>>,
}

struct AttachedView {
    id: NativeViewId,
    view: Retained<NSView>,
    /// The view's superview, which clips it to its element's content mask.
    container: Retained<NSView>,
    /// Where the view was last placed, or `None` while it is hidden.
    placement: Option<Placement>,
}

/// Where a native view is shown, as `[x, y, width, height]` frames in points.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Placement {
    /// The visible part of the view, in the window's content view.
    container: [f64; 4],
    /// The whole view, in its container.
    view: [f64; 4],
}

impl Placement {
    /// Places a view with the given bounds and clip, both in device pixels
    /// from the top left of the GPUI view, whose frame is `gpui_frame`.
    /// Returns `None` if the clip leaves none of the view visible.
    fn new(
        bounds: Bounds<ScaledPixels>,
        content_mask: Bounds<ScaledPixels>,
        scale_factor: f64,
        gpui_frame: [f64; 4],
    ) -> Option<Self> {
        let visible = bounds.intersect(&content_mask);
        if visible.is_empty() {
            return None;
        }
        // AppKit measures from the bottom left of the enclosing view.
        let to_frame = |bounds: Bounds<ScaledPixels>| {
            let [gpui_x, gpui_y, _, gpui_height] = gpui_frame;
            let to_points = |value: ScaledPixels| f64::from(value) / scale_factor;
            let height = to_points(bounds.size.height);
            [
                gpui_x + to_points(bounds.origin.x),
                gpui_y + gpui_height - to_points(bounds.origin.y) - height,
                to_points(bounds.size.width),
                height,
            ]
        };
        let container = to_frame(visible);
        let [x, y, width, height] = to_frame(bounds);
        Some(Self {
            container,
            view: [x - container[0], y - container[1], width, height],
        })
    }
}

impl NativeViews {
    fn shown(&self, id: NativeViewId) -> Option<&AttachedView> {
        self.views
            .iter()
            .find(|attached| attached.id == id && attached.placement.is_some())
    }

    fn is_showing_any(&self) -> bool {
        self.views
            .iter()
            .any(|attached| attached.placement.is_some())
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
        let (attached, native_window, gpui_view, restores_background) = {
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
            let attached = lock.native_views.views.remove(index);
            // No scene sets the background again once the last view is gone.
            let restores_background =
                lock.native_views.views.is_empty() && lock.native_views.background.take().is_some();
            (
                attached,
                lock.native_window,
                lock.native_view.as_ptr(),
                restores_background,
            )
        };
        // SAFETY: The window and both views are live, and attachments are dropped on
        // the main thread because `gpui::NativeView` is not `Send`.
        unsafe {
            // AppKit would otherwise hand first responder to the window itself.
            give_up_first_responder(native_window, gpui_view, &attached.view);
            attached.container.removeFromSuperview();
            attached.view.removeFromSuperview();
            if restores_background {
                let _: () = msg_send![native_window, setBackgroundColor: &*background_color(None)];
            }
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
    let gpui_view = window_state.lock().native_view.as_ptr();
    // SAFETY: The GPUI view is live and in the window's content view, and this
    // runs on the main thread. The window state is unlocked because AppKit calls
    // back into the window as its view hierarchy changes.
    let container = unsafe {
        let container: Retained<NSView> = msg_send![class!(NSView), new];
        let _: () = msg_send![&*container, setClipsToBounds: true];
        container.setHidden(true);
        container.addSubview(&view);
        let content_view: *mut AnyObject = msg_send![gpui_view, superview];
        let _: () = msg_send![
            content_view,
            addSubview: &*container,
            positioned: NSWindowOrderingMode::Below,
            relativeTo: gpui_view
        ];
        container
    };
    window_state.lock().native_views.views.push(AttachedView {
        id,
        view,
        container,
        placement: None,
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
    // SAFETY: The window is live for as long as its state.
    unsafe {
        let _: () = msg_send![
            state.native_window,
            setBackgroundColor: &*background_color(background)
        ];
    }
}

/// The background of an opaque window, which is black unless a native view asks for a color.
fn background_color(color: Option<Rgba>) -> Retained<NSColor> {
    let Rgba { r, g, b, .. } = color.unwrap_or_default();
    NSColor::colorWithSRGBRed_green_blue_alpha(r as f64, g as f64, b as f64, 1.)
}

/// What a scene changes about the window's attached views.
#[derive(Default)]
pub(super) struct Layout {
    /// Views to place somewhere new, or to hide for `None`.
    placements: Vec<(NativeViewId, Option<Placement>)>,
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
    let gpui_frame = [
        gpui_frame.origin.x,
        gpui_frame.origin.y,
        gpui_frame.size.width,
        gpui_frame.size.height,
    ];

    let mut placements = Vec::new();
    for attached in &state.native_views.views {
        let placement = scene
            .native_views
            .iter()
            .find(|native_view| native_view.id == attached.id)
            .and_then(|native_view| {
                Placement::new(
                    native_view.bounds,
                    native_view.content_mask.bounds,
                    scale_factor,
                    gpui_frame,
                )
            });
        if attached.placement != placement {
            placements.push((attached.id, placement));
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
    Layout {
        placements,
        stacking,
    }
}

/// Applies a planned layout. The window state must be unlocked, because AppKit
/// calls back into it when a view's frame or first responder changes.
pub(super) fn apply_layout(window_state: &Rc<Mutex<MacWindowState>>, layout: Layout) {
    if layout.placements.is_empty() && layout.stacking.is_none() {
        return;
    }
    let (native_window, gpui_view, placements, stacking) = {
        let mut lock = window_state.lock();
        let mut placements = Vec::new();
        for (id, placement) in layout.placements {
            if let Some(attached) = lock
                .native_views
                .views
                .iter_mut()
                .find(|attached| attached.id == id)
            {
                attached.placement = placement;
                placements.push((attached.view.clone(), attached.container.clone(), placement));
            }
        }
        let stacking = layout.stacking.map(|stacking| {
            let front_to_back = stacking
                .iter()
                .rev()
                .filter_map(|id| lock.native_views.views.iter().find(|view| view.id == *id))
                .map(|attached| Retained::as_ptr(&attached.container))
                .collect();
            let attached = lock
                .native_views
                .views
                .iter()
                .map(|attached| Retained::as_ptr(&attached.container))
                .collect();
            lock.native_views.stacking = stacking;
            Stacking {
                gpui_view: lock.native_view.as_ptr().cast_const().cast(),
                front_to_back,
                attached,
            }
        });
        (
            lock.native_window,
            lock.native_view.as_ptr(),
            placements,
            stacking,
        )
    };
    let to_rect = |[x, y, width, height]: [f64; 4]| {
        NSRect::new(NSPoint::new(x, y), NSSize::new(width, height))
    };
    for (view, container, placement) in placements {
        match placement {
            Some(placement) => {
                container.setFrame(to_rect(placement.container));
                view.setFrame(to_rect(placement.view));
                container.setHidden(false);
            }
            None => {
                // SAFETY: The window and both views are live, and drawing runs on the main thread.
                unsafe { give_up_first_responder(native_window, gpui_view, &view) };
                container.setHidden(true);
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
    /// The containers of the views the scene shows, starting with the one painted last.
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
        decl.add_method(
            sel!(keyDown:),
            window_key_down as extern "C" fn(_, Sel, *mut AnyObject),
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
/// which is the only time a native view may take first responder on its own, and
/// offers GPUI a key press ahead of the native view that would receive it.
///
/// GPUI is also given the mouse-down, which reaches nothing behind the view but
/// lets the window's own content notice a press outside it.
extern "C" fn send_event(this: &AnyObject, _: Sel, event: *mut AnyObject) {
    // SAFETY: `this` is a live GPUI window and `event` a live `NSEvent`.
    unsafe {
        let superclass = window_superclass(this);
        let Some(window_state) = window_state_if_set(this) else {
            return msg_send![super(this, superclass), sendEvent: event];
        };
        if let Some(key_down) = key_down_for_native_view(&window_state, this, event) {
            if offer_key_down_before_native_view(&window_state, key_down) {
                return;
            }
            window_state.lock().native_views.delivering_key_down = true;
            let _: () = msg_send![super(this, superclass), sendEvent: event];
            window_state.lock().native_views.delivering_key_down = false;
            return;
        }

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
            return msg_send![super(this, superclass), sendEvent: event];
        }
        let window_height = {
            let mut lock = window_state.lock();
            lock.native_views.clicked = clicked;
            lock.content_size().height
        };
        // GPUI goes first, so that focus it moves in response, such as a dismissed
        // popover restoring focus, doesn't take keyboard input back from the view.
        if let Some(mouse_down) = platform_input_from_native(event, Some(window_height)) {
            pass_input_to_gpui(&window_state, mouse_down);
        }
        let _: () = msg_send![super(this, superclass), sendEvent: event];
        window_state.lock().native_views.clicked = None;
    }
}

extern "C" fn make_first_responder(this: &AnyObject, _: Sel, responder: *mut AnyObject) -> Bool {
    // SAFETY: `this` is a live GPUI window and `responder` is null or a live responder.
    unsafe {
        let superclass = window_superclass(this);
        let Some(window_state) = window_state_if_set(this) else {
            return msg_send![super(this, superclass), makeFirstResponder: responder];
        };
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
        let focused_view = focused
            .and_then(|id| lock.native_views.shown(id))
            .map(|attached| attached.view.clone());
        let is_gpui_view = responder == lock.native_view.as_ptr();
        drop(lock);

        // AppKit offers the GPUI view first responder on every press inside it. While
        // GPUI's focus stays on a native view, taking it and handing it straight back
        // would make the native view lose and regain keyboard input, so it is refused.
        // A press that moves GPUI's focus clears `focused` first, and is accepted.
        if is_gpui_view && let Some(focused_view) = focused_view {
            let current: *mut AnyObject = msg_send![this, firstResponder];
            let is_view: bool =
                !current.is_null() && msg_send![current, isKindOfClass: class!(NSView)];
            let inside: bool = is_view && msg_send![current, isDescendantOf: &*focused_view];
            if inside {
                return Bool::NO;
            }
        }

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
                let callback = window_state.lock().native_views.focused_callback.take();
                if let Some(mut callback) = callback {
                    callback(requested);
                    window_state.lock().native_views.focused_callback = Some(callback);
                }
                // GPUI records the view as focused when it follows. If it could not,
                // such as while the window's focus is disabled, this takes first
                // responder back so the two never disagree.
                reconcile_first_responder(&window_state);
            }
            // Anything else that moved first responder away from the view GPUI has
            // focused is undone, such as AppKit handing it to the window itself, or
            // a full screen transition restoring it to the GPUI view.
            _ => reconcile_first_responder(&window_state),
        }
        accepted
    }
}

/// Returns the key press `event` describes, if a native view would receive it.
///
/// # Safety
///
/// `window` must be a live GPUI window and `event` a live `NSEvent`.
unsafe fn key_down_for_native_view(
    window_state: &Rc<Mutex<MacWindowState>>,
    window: &AnyObject,
    event: *mut AnyObject,
) -> Option<KeyDownEvent> {
    // SAFETY: The caller guarantees that `window` and `event` are live.
    unsafe {
        let event_type: NSEventType = msg_send![event, type];
        if event_type != NSEventType::KeyDown {
            return None;
        }
        let mut lock = window_state.try_lock()?;
        if lock.native_views.views.is_empty() {
            return None;
        }
        // AppKit can move first responder without asking the window, such as when
        // the responder leaves the view hierarchy. The press belongs to the view
        // GPUI has focused, so first responder is returned to it before routing.
        if lock
            .native_views
            .focused
            .is_some_and(|id| lock.native_views.shown(id).is_some())
        {
            drop(lock);
            reconcile_first_responder(window_state);
            lock = window_state.try_lock()?;
        }
        let first_responder: *mut AnyObject = msg_send![window, firstResponder];
        lock.native_views.containing(first_responder)?;
        let window_height = lock.content_size().height;
        drop(lock);
        match platform_input_from_native(event, Some(window_height))? {
            PlatformInput::KeyDown(key_down) => Some(key_down),
            _ => None,
        }
    }
}

/// Offers GPUI a key press that a native view has not seen, and returns whether GPUI used it.
fn offer_key_down_before_native_view(
    window_state: &Rc<Mutex<MacWindowState>>,
    key_down: KeyDownEvent,
) -> bool {
    let Some(mut callback) = window_state
        .lock()
        .native_views
        .key_down_before_callback
        .take()
    else {
        return false;
    };
    let handled = callback(key_down);
    window_state.lock().native_views.key_down_before_callback = Some(callback);
    handled
}

/// Dispatches a key press that no native view used, and returns whether GPUI used it.
///
/// This skips the input method handling a key press gets when the GPUI view
/// has keyboard input, because the native view has already done its own.
fn pass_key_down_to_gpui(window_state: &Rc<Mutex<MacWindowState>>, key_down: KeyDownEvent) -> bool {
    pass_input_to_gpui(window_state, PlatformInput::KeyDown(key_down))
}

/// Dispatches input in GPUI, and returns whether GPUI used it.
fn pass_input_to_gpui(window_state: &Rc<Mutex<MacWindowState>>, input: PlatformInput) -> bool {
    let Some(mut callback) = window_state.lock().event_callback.take() else {
        return false;
    };
    let handled = !callback(input).propagate;
    window_state.lock().event_callback = Some(callback);
    handled
}

/// Lets the native view with keyboard input perform a key equivalent before GPUI.
///
/// AppKit otherwise asks the views in an order that doesn't depend on which
/// one has keyboard input.
extern "C" fn window_perform_key_equivalent(
    this: &AnyObject,
    _: Sel,
    event: *mut AnyObject,
) -> Bool {
    // SAFETY: `this` is a live GPUI window and `event` a live `NSEvent`.
    unsafe {
        let superclass = window_superclass(this);
        let Some(window_state) = window_state_if_set(this) else {
            return msg_send![super(this, superclass), performKeyEquivalent: event];
        };
        let Some(key_down) = key_down_for_native_view(&window_state, this, event) else {
            return msg_send![super(this, superclass), performKeyEquivalent: event];
        };
        let delivering_key_down = window_state.lock().native_views.delivering_key_down;
        if !delivering_key_down
            && offer_key_down_before_native_view(&window_state, key_down.clone())
        {
            return Bool::YES;
        }

        window_state.lock().native_views.routing_key_equivalent = true;
        let performed: Bool = msg_send![super(this, superclass), performKeyEquivalent: event];
        window_state.lock().native_views.routing_key_equivalent = false;
        if performed.as_bool() || delivering_key_down {
            return performed;
        }

        // No view performed it. For a web view that means the page has already
        // seen the key and left it, since a web view claims every key equivalent
        // the first time and sends back the ones its page doesn't use.
        window_state.lock().native_views.passed_up_key_equivalent = Retained::retain(event);
        Bool::new(pass_key_down_to_gpui(&window_state, key_down))
    }
}

/// Gives GPUI a key press that came back up the responder chain unused.
extern "C" fn window_key_down(this: &AnyObject, _: Sel, event: *mut AnyObject) {
    // SAFETY: `this` is a live GPUI window and `event` a live `NSEvent`.
    unsafe {
        let superclass = window_superclass(this);
        let Some(window_state) = window_state_if_set(this) else {
            return msg_send![super(this, superclass), keyDown: event];
        };
        let Some(key_down) = key_down_for_native_view(&window_state, this, event) else {
            return msg_send![super(this, superclass), keyDown: event];
        };
        let passed_up = window_state
            .lock()
            .native_views
            .passed_up_key_equivalent
            .take();
        let already_passed_up =
            passed_up.is_some_and(|passed_up| ptr::eq(Retained::as_ptr(&passed_up), event));
        if !already_passed_up {
            pass_key_down_to_gpui(&window_state, key_down);
        }
        // The window's own handling of an unused key press is to beep.
    }
}

/// Whether the window is routing the current key equivalent itself.
pub(super) fn is_routing_key_equivalent(window_state: &Rc<Mutex<MacWindowState>>) -> bool {
    window_state
        .try_lock()
        .is_some_and(|lock| lock.native_views.routing_key_equivalent)
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{point, size};

    #[test]
    fn placement_clips_a_view_to_its_content_mask() {
        let scaled = |x: f32, y: f32, width: f32, height: f32| Bounds {
            origin: point(ScaledPixels::from(x), ScaledPixels::from(y)),
            size: size(ScaledPixels::from(width), ScaledPixels::from(height)),
        };
        // A 400 by 300 point GPUI view at twice the scale, 10 points above the
        // bottom of the content view.
        let gpui_frame = [0., 10., 400., 300.];
        let bounds = scaled(100., 40., 200., 400.);

        let unclipped = Placement::new(bounds, scaled(0., 0., 800., 600.), 2., gpui_frame);
        assert_eq!(
            unclipped,
            Some(Placement {
                container: [50., 90., 100., 200.],
                view: [0., 0., 100., 200.],
            })
        );

        // Clipping the top 60 and left 20 pixels shrinks the container, and the
        // view keeps its size and stays where it was in the window.
        let clipped = Placement::new(bounds, scaled(120., 100., 800., 600.), 2., gpui_frame);
        assert_eq!(
            clipped,
            Some(Placement {
                container: [60., 90., 90., 170.],
                view: [-10., 0., 100., 200.],
            })
        );

        let hidden = Placement::new(bounds, scaled(0., 500., 800., 100.), 2., gpui_frame);
        assert_eq!(hidden, None);
    }
}
