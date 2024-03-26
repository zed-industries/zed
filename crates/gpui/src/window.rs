use crate::{
    point, px, size, transparent_black, Action, AnyDrag, AnyView, AppContext, Arena,
    AsyncWindowContext, Bounds, Context, Corners, CursorStyle, DevicePixels,
    DispatchActionListener, DispatchNodeId, DispatchTree, DisplayId, Edges, Effect, Entity,
    EntityId, EventEmitter, FileDropEvent, Flatten, Global, GlobalElementId, Hsla, KeyBinding,
    KeyDownEvent, KeyMatch, KeymatchResult, Keystroke, KeystrokeEvent, Model, ModelContext,
    Modifiers, ModifiersChangedEvent, MouseButton, MouseMoveEvent, MouseUpEvent, Pixels,
    PlatformAtlas, PlatformDisplay, PlatformInput, PlatformWindow, Point, PromptLevel, Render,
    ScaledPixels, SharedString, Size, SubscriberSet, Subscription, TaffyLayoutEngine, Task,
    TextStyle, TextStyleRefinement, View, VisualContext, WeakView, WindowAppearance, WindowOptions,
    WindowParams, WindowTextSystem,
};
use anyhow::{anyhow, Context as _, Result};
use collections::FxHashSet;
use derive_more::{Deref, DerefMut};
use futures::channel::oneshot;
use parking_lot::RwLock;
use refineable::Refineable;
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    fmt::{Debug, Display},
    future::Future,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc, Weak,
    },
    time::{Duration, Instant},
};
use util::{measure, ResultExt};

mod element_cx;
mod prompts;

pub use element_cx::*;
pub use prompts::*;

/// Represents the two different phases when dispatching events.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub enum DispatchPhase {
    /// After the capture phase comes the bubble phase, in which mouse event listeners are
    /// invoked front to back and keyboard event listeners are invoked from the focused element
    /// to the root of the element tree. This is the phase you'll most commonly want to use when
    /// registering event listeners.
    #[default]
    Bubble,
    /// During the initial capture phase, mouse event listeners are invoked back to front, and keyboard
    /// listeners are invoked from the root of the tree downward toward the focused element. This phase
    /// is used for special purposes such as clearing the "pressed" state for click events. If
    /// you stop event propagation during this phase, you need to know what you're doing. Handlers
    /// outside of the immediate region may rely on detecting non-local events during this phase.
    Capture,
}

impl DispatchPhase {
    /// Returns true if this represents the "bubble" phase.
    pub fn bubble(self) -> bool {
        self == DispatchPhase::Bubble
    }

    /// Returns true if this represents the "capture" phase.
    pub fn capture(self) -> bool {
        self == DispatchPhase::Capture
    }
}

type AnyObserver = Box<dyn FnMut(&mut WindowContext) -> bool + 'static>;

type AnyWindowFocusListener = Box<dyn FnMut(&FocusEvent, &mut WindowContext) -> bool + 'static>;

struct FocusEvent {
    previous_focus_path: SmallVec<[FocusId; 8]>,
    current_focus_path: SmallVec<[FocusId; 8]>,
}

slotmap::new_key_type! {
    /// A globally unique identifier for a focusable element.
    pub struct FocusId;
}

thread_local! {
    pub(crate) static ELEMENT_ARENA: RefCell<Arena> = RefCell::new(Arena::new(8 * 1024 * 1024));
}

impl FocusId {
    /// Obtains whether the element associated with this handle is currently focused.
    pub fn is_focused(&self, cx: &WindowContext) -> bool {
        cx.window.focus == Some(*self)
    }

    /// Obtains whether the element associated with this handle contains the focused
    /// element or is itself focused.
    pub fn contains_focused(&self, cx: &WindowContext) -> bool {
        cx.focused()
            .map_or(false, |focused| self.contains(focused.id, cx))
    }

    /// Obtains whether the element associated with this handle is contained within the
    /// focused element or is itself focused.
    pub fn within_focused(&self, cx: &WindowContext) -> bool {
        let focused = cx.focused();
        focused.map_or(false, |focused| focused.id.contains(*self, cx))
    }

    /// Obtains whether this handle contains the given handle in the most recently rendered frame.
    pub(crate) fn contains(&self, other: Self, cx: &WindowContext) -> bool {
        cx.window
            .rendered_frame
            .dispatch_tree
            .focus_contains(*self, other)
    }
}

/// A handle which can be used to track and manipulate the focused element in a window.
pub struct FocusHandle {
    pub(crate) id: FocusId,
    handles: Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
}

impl std::fmt::Debug for FocusHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("FocusHandle({:?})", self.id))
    }
}

impl FocusHandle {
    pub(crate) fn new(handles: &Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>) -> Self {
        let id = handles.write().insert(AtomicUsize::new(1));
        Self {
            id,
            handles: handles.clone(),
        }
    }

    pub(crate) fn for_id(
        id: FocusId,
        handles: &Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
    ) -> Option<Self> {
        let lock = handles.read();
        let ref_count = lock.get(id)?;
        if ref_count.load(SeqCst) == 0 {
            None
        } else {
            ref_count.fetch_add(1, SeqCst);
            Some(Self {
                id,
                handles: handles.clone(),
            })
        }
    }

    /// Converts this focus handle into a weak variant, which does not prevent it from being released.
    pub fn downgrade(&self) -> WeakFocusHandle {
        WeakFocusHandle {
            id: self.id,
            handles: Arc::downgrade(&self.handles),
        }
    }

    /// Moves the focus to the element associated with this handle.
    pub fn focus(&self, cx: &mut WindowContext) {
        cx.focus(self)
    }

    /// Obtains whether the element associated with this handle is currently focused.
    pub fn is_focused(&self, cx: &WindowContext) -> bool {
        self.id.is_focused(cx)
    }

    /// Obtains whether the element associated with this handle contains the focused
    /// element or is itself focused.
    pub fn contains_focused(&self, cx: &WindowContext) -> bool {
        self.id.contains_focused(cx)
    }

    /// Obtains whether the element associated with this handle is contained within the
    /// focused element or is itself focused.
    pub fn within_focused(&self, cx: &WindowContext) -> bool {
        self.id.within_focused(cx)
    }

    /// Obtains whether this handle contains the given handle in the most recently rendered frame.
    pub fn contains(&self, other: &Self, cx: &WindowContext) -> bool {
        self.id.contains(other.id, cx)
    }
}

impl Clone for FocusHandle {
    fn clone(&self) -> Self {
        Self::for_id(self.id, &self.handles).unwrap()
    }
}

impl PartialEq for FocusHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for FocusHandle {}

impl Drop for FocusHandle {
    fn drop(&mut self) {
        self.handles
            .read()
            .get(self.id)
            .unwrap()
            .fetch_sub(1, SeqCst);
    }
}

/// A weak reference to a focus handle.
#[derive(Clone, Debug)]
pub struct WeakFocusHandle {
    pub(crate) id: FocusId,
    handles: Weak<RwLock<SlotMap<FocusId, AtomicUsize>>>,
}

impl WeakFocusHandle {
    /// Attempts to upgrade the [WeakFocusHandle] to a [FocusHandle].
    pub fn upgrade(&self) -> Option<FocusHandle> {
        let handles = self.handles.upgrade()?;
        FocusHandle::for_id(self.id, &handles)
    }
}

impl PartialEq for WeakFocusHandle {
    fn eq(&self, other: &WeakFocusHandle) -> bool {
        self.id == other.id
    }
}

impl Eq for WeakFocusHandle {}

impl PartialEq<FocusHandle> for WeakFocusHandle {
    fn eq(&self, other: &FocusHandle) -> bool {
        self.id == other.id
    }
}

impl PartialEq<WeakFocusHandle> for FocusHandle {
    fn eq(&self, other: &WeakFocusHandle) -> bool {
        self.id == other.id
    }
}

/// FocusableView allows users of your view to easily
/// focus it (using cx.focus_view(view))
pub trait FocusableView: 'static + Render {
    /// Returns the focus handle associated with this view.
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle;
}

/// ManagedView is a view (like a Modal, Popover, Menu, etc.)
/// where the lifecycle of the view is handled by another view.
pub trait ManagedView: FocusableView + EventEmitter<DismissEvent> {}

impl<M: FocusableView + EventEmitter<DismissEvent>> ManagedView for M {}

/// Emitted by implementers of [`ManagedView`] to indicate the view should be dismissed, such as when a view is presented as a modal.
pub struct DismissEvent;

type FrameCallback = Box<dyn FnOnce(&mut WindowContext)>;

// Holds the state for a specific window.
#[doc(hidden)]
pub struct Window {
    pub(crate) handle: AnyWindowHandle,
    pub(crate) removed: bool,
    pub(crate) platform_window: Box<dyn PlatformWindow>,
    display_id: DisplayId,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    text_system: Arc<WindowTextSystem>,
    pub(crate) rem_size: Pixels,
    pub(crate) viewport_size: Size<Pixels>,
    layout_engine: Option<TaffyLayoutEngine>,
    pub(crate) root_view: Option<AnyView>,
    pub(crate) element_id_stack: GlobalElementId,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) rendered_frame: Frame,
    pub(crate) next_frame: Frame,
    pub(crate) next_hitbox_id: HitboxId,
    next_frame_callbacks: Rc<RefCell<Vec<FrameCallback>>>,
    pub(crate) dirty_views: FxHashSet<EntityId>,
    pub(crate) focus_handles: Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
    focus_listeners: SubscriberSet<(), AnyWindowFocusListener>,
    focus_lost_listeners: SubscriberSet<(), AnyObserver>,
    default_prevented: bool,
    mouse_position: Point<Pixels>,
    mouse_hit_test: HitTest,
    modifiers: Modifiers,
    scale_factor: f32,
    bounds_observers: SubscriberSet<(), AnyObserver>,
    appearance: WindowAppearance,
    appearance_observers: SubscriberSet<(), AnyObserver>,
    active: Rc<Cell<bool>>,
    pub(crate) dirty: Rc<Cell<bool>>,
    pub(crate) needs_present: Rc<Cell<bool>>,
    pub(crate) last_input_timestamp: Rc<Cell<Instant>>,
    pub(crate) refreshing: bool,
    pub(crate) draw_phase: DrawPhase,
    activation_observers: SubscriberSet<(), AnyObserver>,
    pub(crate) focus: Option<FocusId>,
    focus_enabled: bool,
    pending_input: Option<PendingInput>,
    prompt: Option<RenderablePromptHandle>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DrawPhase {
    None,
    Layout,
    Paint,
    Focus,
}

#[derive(Default, Debug)]
struct PendingInput {
    keystrokes: SmallVec<[Keystroke; 1]>,
    bindings: SmallVec<[KeyBinding; 1]>,
    focus: Option<FocusId>,
    timer: Option<Task<()>>,
}

impl PendingInput {
    fn input(&self) -> String {
        self.keystrokes
            .iter()
            .flat_map(|k| k.ime_key.clone())
            .collect::<Vec<String>>()
            .join("")
    }

    fn used_by_binding(&self, binding: &KeyBinding) -> bool {
        if self.keystrokes.is_empty() {
            return true;
        }
        let keystroke = &self.keystrokes[0];
        for candidate in keystroke.match_candidates() {
            if binding.match_keystrokes(&[candidate]) == KeyMatch::Pending {
                return true;
            }
        }
        false
    }
}

pub(crate) struct ElementStateBox {
    pub(crate) inner: Box<dyn Any>,
    #[cfg(debug_assertions)]
    pub(crate) type_name: &'static str,
}

fn default_bounds(display_id: Option<DisplayId>, cx: &mut AppContext) -> Bounds<DevicePixels> {
    const DEFAULT_WINDOW_SIZE: Size<DevicePixels> = size(DevicePixels(1024), DevicePixels(700));
    const DEFAULT_WINDOW_OFFSET: Point<DevicePixels> = point(DevicePixels(0), DevicePixels(35));

    cx.active_window()
        .and_then(|w| w.update(cx, |_, cx| cx.window_bounds()).ok())
        .map(|bounds| bounds.map_origin(|origin| origin + DEFAULT_WINDOW_OFFSET))
        .unwrap_or_else(|| {
            let display = display_id
                .map(|id| cx.find_display(id))
                .unwrap_or_else(|| cx.primary_display());

            display
                .map(|display| {
                    let center = display.bounds().center();
                    let offset = DEFAULT_WINDOW_SIZE / 2;
                    let origin = point(center.x - offset.width, center.y - offset.height);
                    Bounds::new(origin, DEFAULT_WINDOW_SIZE)
                })
                .unwrap_or_else(|| {
                    Bounds::new(point(DevicePixels(0), DevicePixels(0)), DEFAULT_WINDOW_SIZE)
                })
        })
}

impl Window {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        cx: &mut AppContext,
    ) -> Self {
        let WindowOptions {
            bounds,
            titlebar,
            focus,
            show,
            kind,
            is_movable,
            display_id,
            fullscreen,
        } = options;

        let bounds = bounds.unwrap_or_else(|| default_bounds(display_id, cx));
        let platform_window = cx.platform.open_window(
            handle,
            WindowParams {
                bounds,
                titlebar,
                kind,
                is_movable,
                focus,
                show,
                display_id,
            },
        );
        let display_id = platform_window.display().id();
        let sprite_atlas = platform_window.sprite_atlas();
        let mouse_position = platform_window.mouse_position();
        let modifiers = platform_window.modifiers();
        let content_size = platform_window.content_size();
        let scale_factor = platform_window.scale_factor();
        let appearance = platform_window.appearance();
        let text_system = Arc::new(WindowTextSystem::new(cx.text_system().clone()));
        let dirty = Rc::new(Cell::new(true));
        let active = Rc::new(Cell::new(platform_window.is_active()));
        let needs_present = Rc::new(Cell::new(false));
        let next_frame_callbacks: Rc<RefCell<Vec<FrameCallback>>> = Default::default();
        let last_input_timestamp = Rc::new(Cell::new(Instant::now()));

        if fullscreen {
            platform_window.toggle_fullscreen();
        }

        platform_window.on_close(Box::new({
            let mut cx = cx.to_async();
            move || {
                let _ = handle.update(&mut cx, |_, cx| cx.remove_window());
            }
        }));
        platform_window.on_request_frame(Box::new({
            let mut cx = cx.to_async();
            let dirty = dirty.clone();
            let active = active.clone();
            let needs_present = needs_present.clone();
            let next_frame_callbacks = next_frame_callbacks.clone();
            let last_input_timestamp = last_input_timestamp.clone();
            move || {
                let next_frame_callbacks = next_frame_callbacks.take();
                if !next_frame_callbacks.is_empty() {
                    handle
                        .update(&mut cx, |_, cx| {
                            for callback in next_frame_callbacks {
                                callback(cx);
                            }
                        })
                        .log_err();
                }

                // Keep presenting the current scene for 1 extra second since the
                // last input to prevent the display from underclocking the refresh rate.
                let needs_present = needs_present.get()
                    || (active.get()
                        && last_input_timestamp.get().elapsed() < Duration::from_secs(1));

                if dirty.get() {
                    measure("frame duration", || {
                        handle
                            .update(&mut cx, |_, cx| {
                                cx.draw();
                                cx.present();
                            })
                            .log_err();
                    })
                } else if needs_present {
                    handle.update(&mut cx, |_, cx| cx.present()).log_err();
                }
            }
        }));
        platform_window.on_resize(Box::new({
            let mut cx = cx.to_async();
            move |_, _| {
                handle
                    .update(&mut cx, |_, cx| cx.window_bounds_changed())
                    .log_err();
            }
        }));
        platform_window.on_moved(Box::new({
            let mut cx = cx.to_async();
            move || {
                handle
                    .update(&mut cx, |_, cx| cx.window_bounds_changed())
                    .log_err();
            }
        }));
        platform_window.on_appearance_changed(Box::new({
            let mut cx = cx.to_async();
            move || {
                handle
                    .update(&mut cx, |_, cx| cx.appearance_changed())
                    .log_err();
            }
        }));
        platform_window.on_active_status_change(Box::new({
            let mut cx = cx.to_async();
            move |active| {
                handle
                    .update(&mut cx, |_, cx| {
                        cx.window.active.set(active);
                        cx.window
                            .activation_observers
                            .clone()
                            .retain(&(), |callback| callback(cx));
                        cx.refresh();
                    })
                    .log_err();
            }
        }));

        platform_window.on_input({
            let mut cx = cx.to_async();
            Box::new(move |event| {
                handle
                    .update(&mut cx, |_, cx| cx.dispatch_event(event))
                    .log_err()
                    .unwrap_or(DispatchEventResult::default())
            })
        });

        Window {
            handle,
            removed: false,
            platform_window,
            display_id,
            sprite_atlas,
            text_system,
            rem_size: px(16.),
            viewport_size: content_size,
            layout_engine: Some(TaffyLayoutEngine::new()),
            root_view: None,
            element_id_stack: GlobalElementId::default(),
            text_style_stack: Vec::new(),
            rendered_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
            next_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
            next_frame_callbacks,
            next_hitbox_id: HitboxId::default(),
            dirty_views: FxHashSet::default(),
            focus_handles: Arc::new(RwLock::new(SlotMap::with_key())),
            focus_listeners: SubscriberSet::new(),
            focus_lost_listeners: SubscriberSet::new(),
            default_prevented: true,
            mouse_position,
            mouse_hit_test: HitTest::default(),
            modifiers,
            scale_factor,
            bounds_observers: SubscriberSet::new(),
            appearance,
            appearance_observers: SubscriberSet::new(),
            active,
            dirty,
            needs_present,
            last_input_timestamp,
            refreshing: false,
            draw_phase: DrawPhase::None,
            activation_observers: SubscriberSet::new(),
            focus: None,
            focus_enabled: true,
            pending_input: None,
            prompt: None,
        }
    }
    fn new_focus_listener(
        &mut self,
        value: AnyWindowFocusListener,
    ) -> (Subscription, impl FnOnce()) {
        self.focus_listeners.insert((), value)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct DispatchEventResult {
    pub propagate: bool,
    pub default_prevented: bool,
}

/// Indicates which region of the window is visible. Content falling outside of this mask will not be
/// rendered. Currently, only rectangular content masks are supported, but we give the mask its own type
/// to leave room to support more complex shapes in the future.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ContentMask<P: Clone + Default + Debug> {
    /// The bounds
    pub bounds: Bounds<P>,
}

impl ContentMask<Pixels> {
    /// Scale the content mask's pixel units by the given scaling factor.
    pub fn scale(&self, factor: f32) -> ContentMask<ScaledPixels> {
        ContentMask {
            bounds: self.bounds.scale(factor),
        }
    }

    /// Intersect the content mask with the given content mask.
    pub fn intersect(&self, other: &Self) -> Self {
        let bounds = self.bounds.intersect(&other.bounds);
        ContentMask { bounds }
    }
}

/// Provides access to application state in the context of a single window. Derefs
/// to an [`AppContext`], so you can also pass a [`WindowContext`] to any method that takes
/// an [`AppContext`] and call any [`AppContext`] methods.
pub struct WindowContext<'a> {
    pub(crate) app: &'a mut AppContext,
    pub(crate) window: &'a mut Window,
}

impl<'a> WindowContext<'a> {
    pub(crate) fn new(app: &'a mut AppContext, window: &'a mut Window) -> Self {
        Self { app, window }
    }

    /// Obtain a handle to the window that belongs to this context.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window.handle
    }

    /// Mark the window as dirty, scheduling it to be redrawn on the next frame.
    pub fn refresh(&mut self) {
        if self.window.draw_phase == DrawPhase::None {
            self.window.refreshing = true;
            self.window.dirty.set(true);
        }
    }

    /// Indicate that this view has changed, which will invoke any observers and also mark the window as dirty.
    /// If this view or any of its ancestors are *cached*, notifying it will cause it or its ancestors to be redrawn.
    pub fn notify(&mut self, view_id: EntityId) {
        for view_id in self
            .window
            .rendered_frame
            .dispatch_tree
            .view_path(view_id)
            .into_iter()
            .rev()
        {
            if !self.window.dirty_views.insert(view_id) {
                break;
            }
        }

        if self.window.draw_phase == DrawPhase::None {
            self.window.dirty.set(true);
            self.app.push_effect(Effect::Notify { emitter: view_id });
        }
    }

    /// Close this window.
    pub fn remove_window(&mut self) {
        self.window.removed = true;
    }

    /// Obtain a new [`FocusHandle`], which allows you to track and manipulate the keyboard focus
    /// for elements rendered within this window.
    pub fn focus_handle(&mut self) -> FocusHandle {
        FocusHandle::new(&self.window.focus_handles)
    }

    /// Obtain the currently focused [`FocusHandle`]. If no elements are focused, returns `None`.
    pub fn focused(&self) -> Option<FocusHandle> {
        self.window
            .focus
            .and_then(|id| FocusHandle::for_id(id, &self.window.focus_handles))
    }

    /// Move focus to the element associated with the given [`FocusHandle`].
    pub fn focus(&mut self, handle: &FocusHandle) {
        if !self.window.focus_enabled || self.window.focus == Some(handle.id) {
            return;
        }

        self.window.focus = Some(handle.id);
        self.window
            .rendered_frame
            .dispatch_tree
            .clear_pending_keystrokes();
        self.refresh();
    }

    /// Remove focus from all elements within this context's window.
    pub fn blur(&mut self) {
        if !self.window.focus_enabled {
            return;
        }

        self.window.focus = None;
        self.refresh();
    }

    /// Blur the window and don't allow anything in it to be focused again.
    pub fn disable_focus(&mut self) {
        self.blur();
        self.window.focus_enabled = false;
    }

    /// Accessor for the text system.
    pub fn text_system(&self) -> &Arc<WindowTextSystem> {
        &self.window.text_system
    }

    /// The current text style. Which is composed of all the style refinements provided to `with_text_style`.
    pub fn text_style(&self) -> TextStyle {
        let mut style = TextStyle::default();
        for refinement in &self.window.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    /// Check if the platform window is maximized
    /// On some platforms (namely Windows) this is different than the bounds being the size of the display
    pub fn is_maximized(&self) -> bool {
        self.window.platform_window.is_maximized()
    }

    /// Check if the platform window is minimized
    /// On some platforms (namely Windows) the position is incorrect when minimized
    pub fn is_minimized(&self) -> bool {
        self.window.platform_window.is_minimized()
    }

    /// Dispatch the given action on the currently focused element.
    pub fn dispatch_action(&mut self, action: Box<dyn Action>) {
        let focus_handle = self.focused();

        let window = self.window.handle;
        self.app.defer(move |cx| {
            window
                .update(cx, |_, cx| {
                    let node_id = focus_handle
                        .and_then(|handle| {
                            cx.window
                                .rendered_frame
                                .dispatch_tree
                                .focusable_node_id(handle.id)
                        })
                        .unwrap_or_else(|| cx.window.rendered_frame.dispatch_tree.root_node_id());

                    cx.dispatch_action_on_node(node_id, action.as_ref());
                })
                .log_err();
        })
    }

    pub(crate) fn dispatch_keystroke_observers(
        &mut self,
        event: &dyn Any,
        action: Option<Box<dyn Action>>,
    ) {
        let Some(key_down_event) = event.downcast_ref::<KeyDownEvent>() else {
            return;
        };

        self.keystroke_observers
            .clone()
            .retain(&(), move |callback| {
                (callback)(
                    &KeystrokeEvent {
                        keystroke: key_down_event.keystroke.clone(),
                        action: action.as_ref().map(|action| action.boxed_clone()),
                    },
                    self,
                );
                true
            });
    }

    pub(crate) fn clear_pending_keystrokes(&mut self) {
        self.window
            .rendered_frame
            .dispatch_tree
            .clear_pending_keystrokes();
        self.window
            .next_frame
            .dispatch_tree
            .clear_pending_keystrokes();
    }

    /// Schedules the given function to be run at the end of the current effect cycle, allowing entities
    /// that are currently on the stack to be returned to the app.
    pub fn defer(&mut self, f: impl FnOnce(&mut WindowContext) + 'static) {
        let handle = self.window.handle;
        self.app.defer(move |cx| {
            handle.update(cx, |_, cx| f(cx)).ok();
        });
    }

    /// Subscribe to events emitted by a model or view.
    /// The entity to which you're subscribing must implement the [`EventEmitter`] trait.
    /// The callback will be invoked a handle to the emitting entity (either a [`View`] or [`Model`]), the event, and a window context for the current window.
    pub fn subscribe<Emitter, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Evt, &mut WindowContext<'_>) + 'static,
    ) -> Subscription
    where
        Emitter: EventEmitter<Evt>,
        E: Entity<Emitter>,
        Evt: 'static,
    {
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();
        let window_handle = self.window.handle;
        self.app.new_subscription(
            entity_id,
            (
                TypeId::of::<Evt>(),
                Box::new(move |event, cx| {
                    window_handle
                        .update(cx, |_, cx| {
                            if let Some(handle) = E::upgrade_from(&entity) {
                                let event = event.downcast_ref().expect("invalid event type");
                                on_event(handle, event, cx);
                                true
                            } else {
                                false
                            }
                        })
                        .unwrap_or(false)
                }),
            ),
        )
    }

    /// Creates an [`AsyncWindowContext`], which has a static lifetime and can be held across
    /// await points in async code.
    pub fn to_async(&self) -> AsyncWindowContext {
        AsyncWindowContext::new(self.app.to_async(), self.window.handle)
    }

    /// Schedule the given closure to be run directly after the current frame is rendered.
    pub fn on_next_frame(&mut self, callback: impl FnOnce(&mut WindowContext) + 'static) {
        RefCell::borrow_mut(&self.window.next_frame_callbacks).push(Box::new(callback));
    }

    /// Spawn the future returned by the given closure on the application thread pool.
    /// The closure is provided a handle to the current window and an `AsyncWindowContext` for
    /// use within your future.
    pub fn spawn<Fut, R>(&mut self, f: impl FnOnce(AsyncWindowContext) -> Fut) -> Task<R>
    where
        R: 'static,
        Fut: Future<Output = R> + 'static,
    {
        self.app
            .spawn(|app| f(AsyncWindowContext::new(app, self.window.handle)))
    }

    fn window_bounds_changed(&mut self) {
        self.window.scale_factor = self.window.platform_window.scale_factor();
        self.window.viewport_size = self.window.platform_window.content_size();
        self.window.display_id = self.window.platform_window.display().id();
        self.refresh();

        self.window
            .bounds_observers
            .clone()
            .retain(&(), |callback| callback(self));
    }

    /// Returns the bounds of the current window in the global coordinate space, which could span across multiple displays.
    pub fn window_bounds(&self) -> Bounds<DevicePixels> {
        self.window.platform_window.bounds()
    }

    /// Retusn whether or not the window is currently fullscreen
    pub fn is_fullscreen(&self) -> bool {
        self.window.platform_window.is_fullscreen()
    }

    fn appearance_changed(&mut self) {
        self.window.appearance = self.window.platform_window.appearance();

        self.window
            .appearance_observers
            .clone()
            .retain(&(), |callback| callback(self));
    }

    /// Returns the appearance of the current window.
    pub fn appearance(&self) -> WindowAppearance {
        self.window.appearance
    }

    /// Returns the size of the drawable area within the window.
    pub fn viewport_size(&self) -> Size<Pixels> {
        self.window.viewport_size
    }

    /// Returns whether this window is focused by the operating system (receiving key events).
    pub fn is_window_active(&self) -> bool {
        self.window.active.get()
    }

    /// Toggle zoom on the window.
    pub fn zoom_window(&self) {
        self.window.platform_window.zoom();
    }

    /// Updates the window's title at the platform level.
    pub fn set_window_title(&mut self, title: &str) {
        self.window.platform_window.set_title(title);
    }

    /// Mark the window as dirty at the platform level.
    pub fn set_window_edited(&mut self, edited: bool) {
        self.window.platform_window.set_edited(edited);
    }

    /// Determine the display on which the window is visible.
    pub fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.platform
            .displays()
            .into_iter()
            .find(|display| display.id() == self.window.display_id)
    }

    /// Show the platform character palette.
    pub fn show_character_palette(&self) {
        self.window.platform_window.show_character_palette();
    }

    /// The scale factor of the display associated with the window. For example, it could
    /// return 2.0 for a "retina" display, indicating that each logical pixel should actually
    /// be rendered as two pixels on screen.
    pub fn scale_factor(&self) -> f32 {
        self.window.scale_factor
    }

    /// The size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    pub fn rem_size(&self) -> Pixels {
        self.window.rem_size
    }

    /// Sets the size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    pub fn set_rem_size(&mut self, rem_size: impl Into<Pixels>) {
        self.window.rem_size = rem_size.into();
    }

    /// The line height associated with the current text style.
    pub fn line_height(&self) -> Pixels {
        let rem_size = self.rem_size();
        let text_style = self.text_style();
        text_style
            .line_height
            .to_pixels(text_style.font_size, rem_size)
    }

    /// Call to prevent the default action of an event. Currently only used to prevent
    /// parent elements from becoming focused on mouse down.
    pub fn prevent_default(&mut self) {
        self.window.default_prevented = true;
    }

    /// Obtain whether default has been prevented for the event currently being dispatched.
    pub fn default_prevented(&self) -> bool {
        self.window.default_prevented
    }

    /// Determine whether the given action is available along the dispatch path to the currently focused element.
    pub fn is_action_available(&self, action: &dyn Action) -> bool {
        let target = self
            .focused()
            .and_then(|focused_handle| {
                self.window
                    .rendered_frame
                    .dispatch_tree
                    .focusable_node_id(focused_handle.id)
            })
            .unwrap_or_else(|| self.window.rendered_frame.dispatch_tree.root_node_id());
        self.window
            .rendered_frame
            .dispatch_tree
            .is_action_available(action, target)
    }

    /// The position of the mouse relative to the window.
    pub fn mouse_position(&self) -> Point<Pixels> {
        self.window.mouse_position
    }

    /// The current state of the keyboard's modifiers
    pub fn modifiers(&self) -> Modifiers {
        self.window.modifiers
    }

    /// Produces a new frame and assigns it to `rendered_frame`. To actually show
    /// the contents of the new [Scene], use [present].
    #[profiling::function]
    pub fn draw(&mut self) {
        self.window.dirty.set(false);

        // Restore the previously-used input handler.
        if let Some(input_handler) = self.window.platform_window.take_input_handler() {
            self.window
                .rendered_frame
                .input_handlers
                .push(Some(input_handler));
        }

        self.with_element_context(|cx| cx.draw_roots());
        self.window.dirty_views.clear();

        self.window
            .next_frame
            .dispatch_tree
            .preserve_pending_keystrokes(
                &mut self.window.rendered_frame.dispatch_tree,
                self.window.focus,
            );
        self.window.next_frame.focus = self.window.focus;
        self.window.next_frame.window_active = self.window.active.get();

        // Register requested input handler with the platform window.
        if let Some(input_handler) = self.window.next_frame.input_handlers.pop() {
            self.window
                .platform_window
                .set_input_handler(input_handler.unwrap());
        }

        self.window.layout_engine.as_mut().unwrap().clear();
        self.text_system().finish_frame();
        self.window
            .next_frame
            .finish(&mut self.window.rendered_frame);
        ELEMENT_ARENA.with_borrow_mut(|element_arena| {
            let percentage = (element_arena.len() as f32 / element_arena.capacity() as f32) * 100.;
            if percentage >= 80. {
                log::warn!("elevated element arena occupation: {}.", percentage);
            }
            element_arena.clear();
        });

        self.window.draw_phase = DrawPhase::Focus;
        let previous_focus_path = self.window.rendered_frame.focus_path();
        let previous_window_active = self.window.rendered_frame.window_active;
        mem::swap(&mut self.window.rendered_frame, &mut self.window.next_frame);
        self.window.next_frame.clear();
        let current_focus_path = self.window.rendered_frame.focus_path();
        let current_window_active = self.window.rendered_frame.window_active;

        if previous_focus_path != current_focus_path
            || previous_window_active != current_window_active
        {
            if !previous_focus_path.is_empty() && current_focus_path.is_empty() {
                self.window
                    .focus_lost_listeners
                    .clone()
                    .retain(&(), |listener| listener(self));
            }

            let event = FocusEvent {
                previous_focus_path: if previous_window_active {
                    previous_focus_path
                } else {
                    Default::default()
                },
                current_focus_path: if current_window_active {
                    current_focus_path
                } else {
                    Default::default()
                },
            };
            self.window
                .focus_listeners
                .clone()
                .retain(&(), |listener| listener(&event, self));
        }

        self.reset_cursor_style();
        self.window.refreshing = false;
        self.window.draw_phase = DrawPhase::None;
        self.window.needs_present.set(true);
    }

    #[profiling::function]
    fn present(&self) {
        self.window
            .platform_window
            .draw(&self.window.rendered_frame.scene);
        self.window.needs_present.set(false);
        profiling::finish_frame!();
    }

    fn reset_cursor_style(&self) {
        // Set the cursor only if we're the active window.
        if self.is_window_active() {
            let style = self
                .window
                .rendered_frame
                .cursor_styles
                .iter()
                .rev()
                .find(|request| request.hitbox_id.is_hovered(self))
                .map(|request| request.style)
                .unwrap_or(CursorStyle::Arrow);
            self.platform.set_cursor_style(style);
        }
    }

    /// Dispatch a given keystroke as though the user had typed it.
    /// You can create a keystroke with Keystroke::parse("").
    pub fn dispatch_keystroke(&mut self, keystroke: Keystroke) -> bool {
        let keystroke = keystroke.with_simulated_ime();
        let result = self.dispatch_event(PlatformInput::KeyDown(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
        }));
        if !result.propagate {
            return true;
        }

        if let Some(input) = keystroke.ime_key {
            if let Some(mut input_handler) = self.window.platform_window.take_input_handler() {
                input_handler.dispatch_input(&input, self);
                self.window.platform_window.set_input_handler(input_handler);
                return true;
            }
        }

        false
    }

    /// Represent this action as a key binding string, to display in the UI.
    pub fn keystroke_text_for(&self, action: &dyn Action) -> String {
        self.bindings_for_action(action)
            .into_iter()
            .next()
            .map(|binding| {
                binding
                    .keystrokes()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_else(|| action.name().to_string())
    }

    /// Dispatch a mouse or keyboard event on the window.
    #[profiling::function]
    pub fn dispatch_event(&mut self, event: PlatformInput) -> DispatchEventResult {
        self.window.last_input_timestamp.set(Instant::now());
        // Handlers may set this to false by calling `stop_propagation`.
        self.app.propagate_event = true;
        // Handlers may set this to true by calling `prevent_default`.
        self.window.default_prevented = false;

        let event = match event {
            // Track the mouse position with our own state, since accessing the platform
            // API for the mouse position can only occur on the main thread.
            PlatformInput::MouseMove(mouse_move) => {
                self.window.mouse_position = mouse_move.position;
                self.window.modifiers = mouse_move.modifiers;
                PlatformInput::MouseMove(mouse_move)
            }
            PlatformInput::MouseDown(mouse_down) => {
                self.window.mouse_position = mouse_down.position;
                self.window.modifiers = mouse_down.modifiers;
                PlatformInput::MouseDown(mouse_down)
            }
            PlatformInput::MouseUp(mouse_up) => {
                self.window.mouse_position = mouse_up.position;
                self.window.modifiers = mouse_up.modifiers;
                PlatformInput::MouseUp(mouse_up)
            }
            PlatformInput::MouseExited(mouse_exited) => {
                self.window.modifiers = mouse_exited.modifiers;
                PlatformInput::MouseExited(mouse_exited)
            }
            PlatformInput::ModifiersChanged(modifiers_changed) => {
                self.window.modifiers = modifiers_changed.modifiers;
                PlatformInput::ModifiersChanged(modifiers_changed)
            }
            PlatformInput::ScrollWheel(scroll_wheel) => {
                self.window.mouse_position = scroll_wheel.position;
                self.window.modifiers = scroll_wheel.modifiers;
                PlatformInput::ScrollWheel(scroll_wheel)
            }
            // Translate dragging and dropping of external files from the operating system
            // to internal drag and drop events.
            PlatformInput::FileDrop(file_drop) => match file_drop {
                FileDropEvent::Entered { position, paths } => {
                    self.window.mouse_position = position;
                    if self.active_drag.is_none() {
                        self.active_drag = Some(AnyDrag {
                            value: Box::new(paths.clone()),
                            view: self.new_view(|_| paths).into(),
                            cursor_offset: position,
                        });
                    }
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position,
                        pressed_button: Some(MouseButton::Left),
                        modifiers: Modifiers::default(),
                    })
                }
                FileDropEvent::Pending { position } => {
                    self.window.mouse_position = position;
                    PlatformInput::MouseMove(MouseMoveEvent {
                        position,
                        pressed_button: Some(MouseButton::Left),
                        modifiers: Modifiers::default(),
                    })
                }
                FileDropEvent::Submit { position } => {
                    self.activate(true);
                    self.window.mouse_position = position;
                    PlatformInput::MouseUp(MouseUpEvent {
                        button: MouseButton::Left,
                        position,
                        modifiers: Modifiers::default(),
                        click_count: 1,
                    })
                }
                FileDropEvent::Exited => {
                    self.active_drag.take();
                    PlatformInput::FileDrop(FileDropEvent::Exited)
                }
            },
            PlatformInput::KeyDown(_) | PlatformInput::KeyUp(_) => event,
        };

        if let Some(any_mouse_event) = event.mouse_event() {
            self.dispatch_mouse_event(any_mouse_event);
        } else if let Some(any_key_event) = event.keyboard_event() {
            self.dispatch_key_event(any_key_event);
        }

        DispatchEventResult {
            propagate: self.app.propagate_event,
            default_prevented: self.window.default_prevented,
        }
    }

    fn dispatch_mouse_event(&mut self, event: &dyn Any) {
        let hit_test = self.window.rendered_frame.hit_test(self.mouse_position());
        if hit_test != self.window.mouse_hit_test {
            self.window.mouse_hit_test = hit_test;
            self.reset_cursor_style();
        }

        let mut mouse_listeners = mem::take(&mut self.window.rendered_frame.mouse_listeners);
        self.with_element_context(|cx| {
            // Capture phase, events bubble from back to front. Handlers for this phase are used for
            // special purposes, such as detecting events outside of a given Bounds.
            for listener in &mut mouse_listeners {
                let listener = listener.as_mut().unwrap();
                listener(event, DispatchPhase::Capture, cx);
                if !cx.app.propagate_event {
                    break;
                }
            }

            // Bubble phase, where most normal handlers do their work.
            if cx.app.propagate_event {
                for listener in mouse_listeners.iter_mut().rev() {
                    let listener = listener.as_mut().unwrap();
                    listener(event, DispatchPhase::Bubble, cx);
                    if !cx.app.propagate_event {
                        break;
                    }
                }
            }
        });
        self.window.rendered_frame.mouse_listeners = mouse_listeners;

        if self.app.propagate_event && self.has_active_drag() {
            if event.is::<MouseMoveEvent>() {
                // If this was a mouse move event, redraw the window so that the
                // active drag can follow the mouse cursor.
                self.refresh();
            } else if event.is::<MouseUpEvent>() {
                // If this was a mouse up event, cancel the active drag and redraw
                // the window.
                self.active_drag = None;
                self.refresh();
            }
        }
    }

    fn dispatch_key_event(&mut self, event: &dyn Any) {
        if self.window.dirty.get() {
            self.draw();
        }

        let node_id = self
            .window
            .focus
            .and_then(|focus_id| {
                self.window
                    .rendered_frame
                    .dispatch_tree
                    .focusable_node_id(focus_id)
            })
            .unwrap_or_else(|| self.window.rendered_frame.dispatch_tree.root_node_id());

        let dispatch_path = self
            .window
            .rendered_frame
            .dispatch_tree
            .dispatch_path(node_id);

        if let Some(key_down_event) = event.downcast_ref::<KeyDownEvent>() {
            let KeymatchResult { bindings, pending } = self
                .window
                .rendered_frame
                .dispatch_tree
                .dispatch_key(&key_down_event.keystroke, &dispatch_path);

            if pending {
                let mut currently_pending = self.window.pending_input.take().unwrap_or_default();
                if currently_pending.focus.is_some() && currently_pending.focus != self.window.focus
                {
                    currently_pending = PendingInput::default();
                }
                currently_pending.focus = self.window.focus;
                currently_pending
                    .keystrokes
                    .push(key_down_event.keystroke.clone());
                for binding in bindings {
                    currently_pending.bindings.push(binding);
                }

                currently_pending.timer = Some(self.spawn(|mut cx| async move {
                    cx.background_executor.timer(Duration::from_secs(1)).await;
                    cx.update(move |cx| {
                        cx.clear_pending_keystrokes();
                        let Some(currently_pending) = cx.window.pending_input.take() else {
                            return;
                        };
                        cx.replay_pending_input(currently_pending)
                    })
                    .log_err();
                }));

                self.window.pending_input = Some(currently_pending);

                self.propagate_event = false;
                return;
            } else if let Some(currently_pending) = self.window.pending_input.take() {
                if bindings
                    .iter()
                    .all(|binding| !currently_pending.used_by_binding(binding))
                {
                    self.replay_pending_input(currently_pending)
                }
            }

            if !bindings.is_empty() {
                self.clear_pending_keystrokes();
            }

            self.propagate_event = true;
            for binding in bindings {
                self.dispatch_action_on_node(node_id, binding.action.as_ref());
                if !self.propagate_event {
                    self.dispatch_keystroke_observers(event, Some(binding.action));
                    return;
                }
            }
        }

        self.dispatch_key_down_up_event(event, &dispatch_path);
        if !self.propagate_event {
            return;
        }

        self.dispatch_modifiers_changed_event(event, &dispatch_path);
        if !self.propagate_event {
            return;
        }

        self.dispatch_keystroke_observers(event, None);
    }

    fn dispatch_key_down_up_event(
        &mut self,
        event: &dyn Any,
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) {
        // Capture phase
        for node_id in dispatch_path {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);

            for key_listener in node.key_listeners.clone() {
                self.with_element_context(|cx| {
                    key_listener(event, DispatchPhase::Capture, cx);
                });
                if !self.propagate_event {
                    return;
                }
            }
        }

        // Bubble phase
        for node_id in dispatch_path.iter().rev() {
            // Handle low level key events
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);
            for key_listener in node.key_listeners.clone() {
                self.with_element_context(|cx| {
                    key_listener(event, DispatchPhase::Bubble, cx);
                });
                if !self.propagate_event {
                    return;
                }
            }
        }
    }

    fn dispatch_modifiers_changed_event(
        &mut self,
        event: &dyn Any,
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) {
        let Some(event) = event.downcast_ref::<ModifiersChangedEvent>() else {
            return;
        };
        for node_id in dispatch_path.iter().rev() {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);
            for listener in node.modifiers_changed_listeners.clone() {
                self.with_element_context(|cx| {
                    listener(event, cx);
                });
                if !self.propagate_event {
                    return;
                }
            }
        }
    }

    /// Determine whether a potential multi-stroke key binding is in progress on this window.
    pub fn has_pending_keystrokes(&self) -> bool {
        self.window
            .rendered_frame
            .dispatch_tree
            .has_pending_keystrokes()
    }

    fn replay_pending_input(&mut self, currently_pending: PendingInput) {
        let node_id = self
            .window
            .focus
            .and_then(|focus_id| {
                self.window
                    .rendered_frame
                    .dispatch_tree
                    .focusable_node_id(focus_id)
            })
            .unwrap_or_else(|| self.window.rendered_frame.dispatch_tree.root_node_id());

        if self.window.focus != currently_pending.focus {
            return;
        }

        let input = currently_pending.input();

        self.propagate_event = true;
        for binding in currently_pending.bindings {
            self.dispatch_action_on_node(node_id, binding.action.as_ref());
            if !self.propagate_event {
                return;
            }
        }

        let dispatch_path = self
            .window
            .rendered_frame
            .dispatch_tree
            .dispatch_path(node_id);

        for keystroke in currently_pending.keystrokes {
            let event = KeyDownEvent {
                keystroke,
                is_held: false,
            };

            self.dispatch_key_down_up_event(&event, &dispatch_path);
            if !self.propagate_event {
                return;
            }
        }

        if !input.is_empty() {
            if let Some(mut input_handler) = self.window.platform_window.take_input_handler() {
                input_handler.dispatch_input(&input, self);
                self.window.platform_window.set_input_handler(input_handler)
            }
        }
    }

    fn dispatch_action_on_node(&mut self, node_id: DispatchNodeId, action: &dyn Action) {
        let dispatch_path = self
            .window
            .rendered_frame
            .dispatch_tree
            .dispatch_path(node_id);

        // Capture phase for global actions.
        self.propagate_event = true;
        if let Some(mut global_listeners) = self
            .global_action_listeners
            .remove(&action.as_any().type_id())
        {
            for listener in &global_listeners {
                listener(action.as_any(), DispatchPhase::Capture, self);
                if !self.propagate_event {
                    break;
                }
            }

            global_listeners.extend(
                self.global_action_listeners
                    .remove(&action.as_any().type_id())
                    .unwrap_or_default(),
            );

            self.global_action_listeners
                .insert(action.as_any().type_id(), global_listeners);
        }

        if !self.propagate_event {
            return;
        }

        // Capture phase for window actions.
        for node_id in &dispatch_path {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);
            for DispatchActionListener {
                action_type,
                listener,
            } in node.action_listeners.clone()
            {
                let any_action = action.as_any();
                if action_type == any_action.type_id() {
                    self.with_element_context(|cx| {
                        listener(any_action, DispatchPhase::Capture, cx);
                    });

                    if !self.propagate_event {
                        return;
                    }
                }
            }
        }

        // Bubble phase for window actions.
        for node_id in dispatch_path.iter().rev() {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);
            for DispatchActionListener {
                action_type,
                listener,
            } in node.action_listeners.clone()
            {
                let any_action = action.as_any();
                if action_type == any_action.type_id() {
                    self.propagate_event = false; // Actions stop propagation by default during the bubble phase

                    self.with_element_context(|cx| {
                        listener(any_action, DispatchPhase::Bubble, cx);
                    });

                    if !self.propagate_event {
                        return;
                    }
                }
            }
        }

        // Bubble phase for global actions.
        if let Some(mut global_listeners) = self
            .global_action_listeners
            .remove(&action.as_any().type_id())
        {
            for listener in global_listeners.iter().rev() {
                self.propagate_event = false; // Actions stop propagation by default during the bubble phase

                listener(action.as_any(), DispatchPhase::Bubble, self);
                if !self.propagate_event {
                    break;
                }
            }

            global_listeners.extend(
                self.global_action_listeners
                    .remove(&action.as_any().type_id())
                    .unwrap_or_default(),
            );

            self.global_action_listeners
                .insert(action.as_any().type_id(), global_listeners);
        }
    }

    /// Register the given handler to be invoked whenever the global of the given type
    /// is updated.
    pub fn observe_global<G: Global>(
        &mut self,
        f: impl Fn(&mut WindowContext<'_>) + 'static,
    ) -> Subscription {
        let window_handle = self.window.handle;
        let (subscription, activate) = self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| window_handle.update(cx, |_, cx| f(cx)).is_ok()),
        );
        self.app.defer(move |_| activate());
        subscription
    }

    /// Focus the current window and bring it to the foreground at the platform level.
    pub fn activate_window(&self) {
        self.window.platform_window.activate();
    }

    /// Minimize the current window at the platform level.
    pub fn minimize_window(&self) {
        self.window.platform_window.minimize();
    }

    /// Toggle full screen status on the current window at the platform level.
    pub fn toggle_fullscreen(&self) {
        self.window.platform_window.toggle_fullscreen();
    }

    /// Present a platform dialog.
    /// The provided message will be presented, along with buttons for each answer.
    /// When a button is clicked, the returned Receiver will receive the index of the clicked button.
    pub fn prompt(
        &mut self,
        level: PromptLevel,
        message: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        let prompt_builder = self.app.prompt_builder.take();
        let Some(prompt_builder) = prompt_builder else {
            unreachable!("Re-entrant window prompting is not supported by GPUI");
        };

        let receiver = match &prompt_builder {
            PromptBuilder::Default => self
                .window
                .platform_window
                .prompt(level, message, detail, answers)
                .unwrap_or_else(|| {
                    self.build_custom_prompt(&prompt_builder, level, message, detail, answers)
                }),
            PromptBuilder::Custom(_) => {
                self.build_custom_prompt(&prompt_builder, level, message, detail, answers)
            }
        };

        self.app.prompt_builder = Some(prompt_builder);

        receiver
    }

    fn build_custom_prompt(
        &mut self,
        prompt_builder: &PromptBuilder,
        level: PromptLevel,
        message: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        let (sender, receiver) = oneshot::channel();
        let handle = PromptHandle::new(sender);
        let handle = (prompt_builder)(level, message, detail, answers, handle, self);
        self.window.prompt = Some(handle);
        receiver
    }

    /// Returns all available actions for the focused element.
    pub fn available_actions(&self) -> Vec<Box<dyn Action>> {
        let node_id = self
            .window
            .focus
            .and_then(|focus_id| {
                self.window
                    .rendered_frame
                    .dispatch_tree
                    .focusable_node_id(focus_id)
            })
            .unwrap_or_else(|| self.window.rendered_frame.dispatch_tree.root_node_id());

        let mut actions = self
            .window
            .rendered_frame
            .dispatch_tree
            .available_actions(node_id);
        for action_type in self.global_action_listeners.keys() {
            if let Err(ix) = actions.binary_search_by_key(action_type, |a| a.as_any().type_id()) {
                let action = self.actions.build_action_type(action_type).ok();
                if let Some(action) = action {
                    actions.insert(ix, action);
                }
            }
        }
        actions
    }

    /// Returns key bindings that invoke the given action on the currently focused element.
    pub fn bindings_for_action(&self, action: &dyn Action) -> Vec<KeyBinding> {
        self.window
            .rendered_frame
            .dispatch_tree
            .bindings_for_action(
                action,
                &self.window.rendered_frame.dispatch_tree.context_stack,
            )
    }

    /// Returns any bindings that would invoke the given action on the given focus handle if it were focused.
    pub fn bindings_for_action_in(
        &self,
        action: &dyn Action,
        focus_handle: &FocusHandle,
    ) -> Vec<KeyBinding> {
        let dispatch_tree = &self.window.rendered_frame.dispatch_tree;

        let Some(node_id) = dispatch_tree.focusable_node_id(focus_handle.id) else {
            return vec![];
        };
        let context_stack: Vec<_> = dispatch_tree
            .dispatch_path(node_id)
            .into_iter()
            .filter_map(|node_id| dispatch_tree.node(node_id).context.clone())
            .collect();
        dispatch_tree.bindings_for_action(action, &context_stack)
    }

    /// Returns a generic event listener that invokes the given listener with the view and context associated with the given view handle.
    pub fn listener_for<V: Render, E>(
        &self,
        view: &View<V>,
        f: impl Fn(&mut V, &E, &mut ViewContext<V>) + 'static,
    ) -> impl Fn(&E, &mut WindowContext) + 'static {
        let view = view.downgrade();
        move |e: &E, cx: &mut WindowContext| {
            view.update(cx, |view, cx| f(view, e, cx)).ok();
        }
    }

    /// Returns a generic handler that invokes the given handler with the view and context associated with the given view handle.
    pub fn handler_for<V: Render>(
        &self,
        view: &View<V>,
        f: impl Fn(&mut V, &mut ViewContext<V>) + 'static,
    ) -> impl Fn(&mut WindowContext) {
        let view = view.downgrade();
        move |cx: &mut WindowContext| {
            view.update(cx, |view, cx| f(view, cx)).ok();
        }
    }

    /// Register a callback that can interrupt the closing of the current window based the returned boolean.
    /// If the callback returns false, the window won't be closed.
    pub fn on_window_should_close(&mut self, f: impl Fn(&mut WindowContext) -> bool + 'static) {
        let mut this = self.to_async();
        self.window
            .platform_window
            .on_should_close(Box::new(move || this.update(|cx| f(cx)).unwrap_or(true)))
    }

    /// Register an action listener on the window for the next frame. The type of action
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This is a fairly low-level method, so prefer using action handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_action(
        &mut self,
        action_type: TypeId,
        listener: impl Fn(&dyn Any, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        self.window
            .next_frame
            .dispatch_tree
            .on_action(action_type, Rc::new(listener));
    }
}

#[cfg(target_os = "windows")]
impl WindowContext<'_> {
    /// Returns the raw HWND handle for the window.
    pub fn get_raw_handle(&self) -> windows::Win32::Foundation::HWND {
        self.window.platform_window.get_raw_handle()
    }
}

impl Context for WindowContext<'_> {
    type Result<T> = T;

    fn new_model<T>(&mut self, build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T) -> Model<T>
    where
        T: 'static,
    {
        let slot = self.app.entities.reserve();
        let model = build_model(&mut ModelContext::new(&mut *self.app, slot.downgrade()));
        self.entities.insert(slot, model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> R {
        let mut entity = self.entities.lease(model);
        let result = update(
            &mut *entity,
            &mut ModelContext::new(&mut *self.app, model.downgrade()),
        );
        self.entities.end_lease(entity);
        result
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        if window == self.window.handle {
            let root_view = self.window.root_view.clone().unwrap();
            Ok(update(root_view, self))
        } else {
            window.update(self.app, update)
        }
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let entity = self.entities.read(handle);
        read(entity, &*self.app)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        if window.any_handle == self.window.handle {
            let root_view = self
                .window
                .root_view
                .clone()
                .unwrap()
                .downcast::<T>()
                .map_err(|_| anyhow!("the type of the window's root view has changed"))?;
            Ok(read(root_view, self))
        } else {
            self.app.read_window(window, read)
        }
    }
}

impl VisualContext for WindowContext<'_> {
    fn new_view<V>(
        &mut self,
        build_view_state: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render,
    {
        let slot = self.app.entities.reserve();
        let view = View {
            model: slot.clone(),
        };
        let mut cx = ViewContext::new(&mut *self.app, &mut *self.window, &view);
        let entity = build_view_state(&mut cx);
        cx.entities.insert(slot, entity);

        // Non-generic part to avoid leaking SubscriberSet to invokers of `new_view`.
        fn notify_observers(cx: &mut WindowContext, tid: TypeId, view: AnyView) {
            cx.new_view_observers.clone().retain(&tid, |observer| {
                let any_view = view.clone();
                (observer)(any_view, cx);
                true
            });
        }
        notify_observers(self, TypeId::of::<V>(), AnyView::from(view.clone()));

        view
    }

    /// Updates the given view. Prefer calling [`View::update`] instead, which calls this method.
    fn update_view<T: 'static, R>(
        &mut self,
        view: &View<T>,
        update: impl FnOnce(&mut T, &mut ViewContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let mut lease = self.app.entities.lease(&view.model);
        let mut cx = ViewContext::new(&mut *self.app, &mut *self.window, view);
        let result = update(&mut *lease, &mut cx);
        cx.app.entities.end_lease(lease);
        result
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render,
    {
        let view = self.new_view(build_view);
        self.window.root_view = Some(view.clone().into());
        self.refresh();
        view
    }

    fn focus_view<V: crate::FocusableView>(&mut self, view: &View<V>) -> Self::Result<()> {
        self.update_view(view, |view, cx| {
            view.focus_handle(cx).clone().focus(cx);
        })
    }

    fn dismiss_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: ManagedView,
    {
        self.update_view(view, |_, cx| cx.emit(DismissEvent))
    }
}

impl<'a> std::ops::Deref for WindowContext<'a> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<'a> std::ops::DerefMut for WindowContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.app
    }
}

impl<'a> Borrow<AppContext> for WindowContext<'a> {
    fn borrow(&self) -> &AppContext {
        self.app
    }
}

impl<'a> BorrowMut<AppContext> for WindowContext<'a> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        self.app
    }
}

/// This trait contains functionality that is shared across [`ViewContext`] and [`WindowContext`]
pub trait BorrowWindow: BorrowMut<Window> + BorrowMut<AppContext> {
    #[doc(hidden)]
    fn app_mut(&mut self) -> &mut AppContext {
        self.borrow_mut()
    }

    #[doc(hidden)]
    fn app(&self) -> &AppContext {
        self.borrow()
    }

    #[doc(hidden)]
    fn window(&self) -> &Window {
        self.borrow()
    }

    #[doc(hidden)]
    fn window_mut(&mut self) -> &mut Window {
        self.borrow_mut()
    }
}

impl Borrow<Window> for WindowContext<'_> {
    fn borrow(&self) -> &Window {
        self.window
    }
}

impl BorrowMut<Window> for WindowContext<'_> {
    fn borrow_mut(&mut self) -> &mut Window {
        self.window
    }
}

impl<T> BorrowWindow for T where T: BorrowMut<AppContext> + BorrowMut<Window> {}

/// Provides access to application state that is specialized for a particular [`View`].
/// Allows you to interact with focus, emit events, etc.
/// ViewContext also derefs to [`WindowContext`], giving you access to all of its methods as well.
/// When you call [`View::update`], you're passed a `&mut V` and an `&mut ViewContext<V>`.
pub struct ViewContext<'a, V> {
    window_cx: WindowContext<'a>,
    view: &'a View<V>,
}

impl<V> Borrow<AppContext> for ViewContext<'_, V> {
    fn borrow(&self) -> &AppContext {
        &*self.window_cx.app
    }
}

impl<V> BorrowMut<AppContext> for ViewContext<'_, V> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        &mut *self.window_cx.app
    }
}

impl<V> Borrow<Window> for ViewContext<'_, V> {
    fn borrow(&self) -> &Window {
        &*self.window_cx.window
    }
}

impl<V> BorrowMut<Window> for ViewContext<'_, V> {
    fn borrow_mut(&mut self) -> &mut Window {
        &mut *self.window_cx.window
    }
}

impl<'a, V: 'static> ViewContext<'a, V> {
    pub(crate) fn new(app: &'a mut AppContext, window: &'a mut Window, view: &'a View<V>) -> Self {
        Self {
            window_cx: WindowContext::new(app, window),
            view,
        }
    }

    /// Get the entity_id of this view.
    pub fn entity_id(&self) -> EntityId {
        self.view.entity_id()
    }

    /// Get the view pointer underlying this context.
    pub fn view(&self) -> &View<V> {
        self.view
    }

    /// Get the model underlying this view.
    pub fn model(&self) -> &Model<V> {
        &self.view.model
    }

    /// Access the underlying window context.
    pub fn window_context(&mut self) -> &mut WindowContext<'a> {
        &mut self.window_cx
    }

    /// Sets a given callback to be run on the next frame.
    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut V, &mut ViewContext<V>) + 'static)
    where
        V: 'static,
    {
        let view = self.view().clone();
        self.window_cx.on_next_frame(move |cx| view.update(cx, f));
    }

    /// Schedules the given function to be run at the end of the current effect cycle, allowing entities
    /// that are currently on the stack to be returned to the app.
    pub fn defer(&mut self, f: impl FnOnce(&mut V, &mut ViewContext<V>) + 'static) {
        let view = self.view().downgrade();
        self.window_cx.defer(move |cx| {
            view.update(cx, f).ok();
        });
    }

    /// Observe another model or view for changes to its state, as tracked by [`ModelContext::notify`].
    pub fn observe<V2, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(&mut V, E, &mut ViewContext<'_, V>) + 'static,
    ) -> Subscription
    where
        V2: 'static,
        V: 'static,
        E: Entity<V2>,
    {
        let view = self.view().downgrade();
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();
        let window_handle = self.window.handle;
        self.app.new_observer(
            entity_id,
            Box::new(move |cx| {
                window_handle
                    .update(cx, |_, cx| {
                        if let Some(handle) = E::upgrade_from(&entity) {
                            view.update(cx, |this, cx| on_notify(this, handle, cx))
                                .is_ok()
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            }),
        )
    }

    /// Subscribe to events emitted by another model or view.
    /// The entity to which you're subscribing must implement the [`EventEmitter`] trait.
    /// The callback will be invoked with a reference to the current view, a handle to the emitting entity (either a [`View`] or [`Model`]), the event, and a view context for the current view.
    pub fn subscribe<V2, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(&mut V, E, &Evt, &mut ViewContext<'_, V>) + 'static,
    ) -> Subscription
    where
        V2: EventEmitter<Evt>,
        E: Entity<V2>,
        Evt: 'static,
    {
        let view = self.view().downgrade();
        let entity_id = entity.entity_id();
        let handle = entity.downgrade();
        let window_handle = self.window.handle;
        self.app.new_subscription(
            entity_id,
            (
                TypeId::of::<Evt>(),
                Box::new(move |event, cx| {
                    window_handle
                        .update(cx, |_, cx| {
                            if let Some(handle) = E::upgrade_from(&handle) {
                                let event = event.downcast_ref().expect("invalid event type");
                                view.update(cx, |this, cx| on_event(this, handle, event, cx))
                                    .is_ok()
                            } else {
                                false
                            }
                        })
                        .unwrap_or(false)
                }),
            ),
        )
    }

    /// Register a callback to be invoked when the view is released.
    ///
    /// The callback receives a handle to the view's window. This handle may be
    /// invalid, if the window was closed before the view was released.
    pub fn on_release(
        &mut self,
        on_release: impl FnOnce(&mut V, AnyWindowHandle, &mut AppContext) + 'static,
    ) -> Subscription {
        let window_handle = self.window.handle;
        let (subscription, activate) = self.app.release_listeners.insert(
            self.view.model.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                on_release(this, window_handle, cx)
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when the given Model or View is released.
    pub fn observe_release<V2, E>(
        &mut self,
        entity: &E,
        mut on_release: impl FnMut(&mut V, &mut V2, &mut ViewContext<'_, V>) + 'static,
    ) -> Subscription
    where
        V: 'static,
        V2: 'static,
        E: Entity<V2>,
    {
        let view = self.view().downgrade();
        let entity_id = entity.entity_id();
        let window_handle = self.window.handle;
        let (subscription, activate) = self.app.release_listeners.insert(
            entity_id,
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                let _ = window_handle.update(cx, |_, cx| {
                    view.update(cx, |this, cx| on_release(this, entity, cx))
                });
            }),
        );
        activate();
        subscription
    }

    /// Indicate that this view has changed, which will invoke any observers and also mark the window as dirty.
    /// If this view or any of its ancestors are *cached*, notifying it will cause it or its ancestors to be redrawn.
    pub fn notify(&mut self) {
        self.window_cx.notify(self.view.entity_id());
    }

    /// Register a callback to be invoked when the window is resized.
    pub fn observe_window_bounds(
        &mut self,
        mut callback: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let (subscription, activate) = self.window.bounds_observers.insert(
            (),
            Box::new(move |cx| view.update(cx, |view, cx| callback(view, cx)).is_ok()),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when the window is activated or deactivated.
    pub fn observe_window_activation(
        &mut self,
        mut callback: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let (subscription, activate) = self.window.activation_observers.insert(
            (),
            Box::new(move |cx| view.update(cx, |view, cx| callback(view, cx)).is_ok()),
        );
        activate();
        subscription
    }

    /// Registers a callback to be invoked when the window appearance changes.
    pub fn observe_window_appearance(
        &mut self,
        mut callback: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let (subscription, activate) = self.window.appearance_observers.insert(
            (),
            Box::new(move |cx| view.update(cx, |view, cx| callback(view, cx)).is_ok()),
        );
        activate();
        subscription
    }

    /// Register a listener to be called when the given focus handle receives focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus(
        &mut self,
        handle: &FocusHandle,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let focus_id = handle.id;
        let (subscription, activate) =
            self.window.new_focus_listener(Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.last() != Some(&focus_id)
                        && event.current_focus_path.last() == Some(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }));
        self.app.defer(|_| activate());
        subscription
    }

    /// Register a listener to be called when the given focus handle or one of its descendants receives focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus_in(
        &mut self,
        handle: &FocusHandle,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let focus_id = handle.id;
        let (subscription, activate) =
            self.window.new_focus_listener(Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if !event.previous_focus_path.contains(&focus_id)
                        && event.current_focus_path.contains(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }));
        self.app.defer(move |_| activate());
        subscription
    }

    /// Register a listener to be called when the given focus handle loses focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_blur(
        &mut self,
        handle: &FocusHandle,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let focus_id = handle.id;
        let (subscription, activate) =
            self.window.new_focus_listener(Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.last() == Some(&focus_id)
                        && event.current_focus_path.last() != Some(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }));
        self.app.defer(move |_| activate());
        subscription
    }

    /// Register a listener to be called when nothing in the window has focus.
    /// This typically happens when the node that was focused is removed from the tree,
    /// and this callback lets you chose a default place to restore the users focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus_lost(
        &mut self,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let (subscription, activate) = self.window.focus_lost_listeners.insert(
            (),
            Box::new(move |cx| view.update(cx, |view, cx| listener(view, cx)).is_ok()),
        );
        activate();
        subscription
    }

    /// Register a listener to be called when the given focus handle or one of its descendants loses focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus_out(
        &mut self,
        handle: &FocusHandle,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let focus_id = handle.id;
        let (subscription, activate) =
            self.window.new_focus_listener(Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.contains(&focus_id)
                        && !event.current_focus_path.contains(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }));
        self.app.defer(move |_| activate());
        subscription
    }

    /// Schedule a future to be run asynchronously.
    /// The given callback is invoked with a [`WeakView<V>`] to avoid leaking the view for a long-running process.
    /// It's also given an [`AsyncWindowContext`], which can be used to access the state of the view across await points.
    /// The returned future will be polled on the main thread.
    pub fn spawn<Fut, R>(
        &mut self,
        f: impl FnOnce(WeakView<V>, AsyncWindowContext) -> Fut,
    ) -> Task<R>
    where
        R: 'static,
        Fut: Future<Output = R> + 'static,
    {
        let view = self.view().downgrade();
        self.window_cx.spawn(|cx| f(view, cx))
    }

    /// Register a callback to be invoked when the given global state changes.
    pub fn observe_global<G: Global>(
        &mut self,
        mut f: impl FnMut(&mut V, &mut ViewContext<'_, V>) + 'static,
    ) -> Subscription {
        let window_handle = self.window.handle;
        let view = self.view().downgrade();
        let (subscription, activate) = self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                window_handle
                    .update(cx, |_, cx| view.update(cx, |view, cx| f(view, cx)).is_ok())
                    .unwrap_or(false)
            }),
        );
        self.app.defer(move |_| activate());
        subscription
    }

    /// Register a callback to be invoked when the given Action type is dispatched to the window.
    pub fn on_action(
        &mut self,
        action_type: TypeId,
        listener: impl Fn(&mut V, &dyn Any, DispatchPhase, &mut ViewContext<V>) + 'static,
    ) {
        let handle = self.view().clone();
        self.window_cx
            .on_action(action_type, move |action, phase, cx| {
                handle.update(cx, |view, cx| {
                    listener(view, action, phase, cx);
                })
            });
    }

    /// Emit an event to be handled any other views that have subscribed via [ViewContext::subscribe].
    pub fn emit<Evt>(&mut self, event: Evt)
    where
        Evt: 'static,
        V: EventEmitter<Evt>,
    {
        let emitter = self.view.model.entity_id;
        self.app.push_effect(Effect::Emit {
            emitter,
            event_type: TypeId::of::<Evt>(),
            event: Box::new(event),
        });
    }

    /// Move focus to the current view, assuming it implements [`FocusableView`].
    pub fn focus_self(&mut self)
    where
        V: FocusableView,
    {
        self.defer(|view, cx| view.focus_handle(cx).focus(cx))
    }

    /// Convenience method for accessing view state in an event callback.
    ///
    /// Many GPUI callbacks take the form of `Fn(&E, &mut WindowContext)`,
    /// but it's often useful to be able to access view state in these
    /// callbacks. This method provides a convenient way to do so.
    pub fn listener<E>(
        &self,
        f: impl Fn(&mut V, &E, &mut ViewContext<V>) + 'static,
    ) -> impl Fn(&E, &mut WindowContext) + 'static {
        let view = self.view().downgrade();
        move |e: &E, cx: &mut WindowContext| {
            view.update(cx, |view, cx| f(view, e, cx)).ok();
        }
    }
}

impl<V> Context for ViewContext<'_, V> {
    type Result<U> = U;

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.window_cx.new_model(build_model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> R {
        self.window_cx.update_model(model, update)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        self.window_cx.update_window(window, update)
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.window_cx.read_model(handle, read)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.window_cx.read_window(window, read)
    }
}

impl<V: 'static> VisualContext for ViewContext<'_, V> {
    fn new_view<W: Render + 'static>(
        &mut self,
        build_view_state: impl FnOnce(&mut ViewContext<'_, W>) -> W,
    ) -> Self::Result<View<W>> {
        self.window_cx.new_view(build_view_state)
    }

    fn update_view<V2: 'static, R>(
        &mut self,
        view: &View<V2>,
        update: impl FnOnce(&mut V2, &mut ViewContext<'_, V2>) -> R,
    ) -> Self::Result<R> {
        self.window_cx.update_view(view, update)
    }

    fn replace_root_view<W>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, W>) -> W,
    ) -> Self::Result<View<W>>
    where
        W: 'static + Render,
    {
        self.window_cx.replace_root_view(build_view)
    }

    fn focus_view<W: FocusableView>(&mut self, view: &View<W>) -> Self::Result<()> {
        self.window_cx.focus_view(view)
    }

    fn dismiss_view<W: ManagedView>(&mut self, view: &View<W>) -> Self::Result<()> {
        self.window_cx.dismiss_view(view)
    }
}

impl<'a, V> std::ops::Deref for ViewContext<'a, V> {
    type Target = WindowContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.window_cx
    }
}

impl<'a, V> std::ops::DerefMut for ViewContext<'a, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window_cx
    }
}

// #[derive(Clone, Copy, Eq, PartialEq, Hash)]
slotmap::new_key_type! {
    /// A unique identifier for a window.
    pub struct WindowId;
}

impl WindowId {
    /// Converts this window ID to a `u64`.
    pub fn as_u64(&self) -> u64 {
        self.0.as_ffi()
    }
}

/// A handle to a window with a specific root view type.
/// Note that this does not keep the window alive on its own.
#[derive(Deref, DerefMut)]
pub struct WindowHandle<V> {
    #[deref]
    #[deref_mut]
    pub(crate) any_handle: AnyWindowHandle,
    state_type: PhantomData<V>,
}

impl<V: 'static + Render> WindowHandle<V> {
    /// Creates a new handle from a window ID.
    /// This does not check if the root type of the window is `V`.
    pub fn new(id: WindowId) -> Self {
        WindowHandle {
            any_handle: AnyWindowHandle {
                id,
                state_type: TypeId::of::<V>(),
            },
            state_type: PhantomData,
        }
    }

    /// Get the root view out of this window.
    ///
    /// This will fail if the window is closed or if the root view's type does not match `V`.
    pub fn root<C>(&self, cx: &mut C) -> Result<View<V>>
    where
        C: Context,
    {
        Flatten::flatten(cx.update_window(self.any_handle, |root_view, _| {
            root_view
                .downcast::<V>()
                .map_err(|_| anyhow!("the type of the window's root view has changed"))
        }))
    }

    /// Updates the root view of this window.
    ///
    /// This will fail if the window has been closed or if the root view's type does not match
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Result<R>
    where
        C: Context,
    {
        cx.update_window(self.any_handle, |root_view, cx| {
            let view = root_view
                .downcast::<V>()
                .map_err(|_| anyhow!("the type of the window's root view has changed"))?;
            Ok(cx.update_view(&view, update))
        })?
    }

    /// Read the root view out of this window.
    ///
    /// This will fail if the window is closed or if the root view's type does not match `V`.
    pub fn read<'a>(&self, cx: &'a AppContext) -> Result<&'a V> {
        let x = cx
            .windows
            .get(self.id)
            .and_then(|window| {
                window
                    .as_ref()
                    .and_then(|window| window.root_view.clone())
                    .map(|root_view| root_view.downcast::<V>())
            })
            .ok_or_else(|| anyhow!("window not found"))?
            .map_err(|_| anyhow!("the type of the window's root view has changed"))?;

        Ok(x.read(cx))
    }

    /// Read the root view out of this window, with a callback
    ///
    /// This will fail if the window is closed or if the root view's type does not match `V`.
    pub fn read_with<C, R>(&self, cx: &C, read_with: impl FnOnce(&V, &AppContext) -> R) -> Result<R>
    where
        C: Context,
    {
        cx.read_window(self, |root_view, cx| read_with(root_view.read(cx), cx))
    }

    /// Read the root view pointer off of this window.
    ///
    /// This will fail if the window is closed or if the root view's type does not match `V`.
    pub fn root_view<C>(&self, cx: &C) -> Result<View<V>>
    where
        C: Context,
    {
        cx.read_window(self, |root_view, _cx| root_view.clone())
    }

    /// Check if this window is 'active'.
    ///
    /// Will return `None` if the window is closed or currently
    /// borrowed.
    pub fn is_active(&self, cx: &mut AppContext) -> Option<bool> {
        cx.update_window(self.any_handle, |_, cx| cx.is_window_active())
            .ok()
    }
}

impl<V> Copy for WindowHandle<V> {}

impl<V> Clone for WindowHandle<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V> PartialEq for WindowHandle<V> {
    fn eq(&self, other: &Self) -> bool {
        self.any_handle == other.any_handle
    }
}

impl<V> Eq for WindowHandle<V> {}

impl<V> Hash for WindowHandle<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_handle.hash(state);
    }
}

impl<V: 'static> From<WindowHandle<V>> for AnyWindowHandle {
    fn from(val: WindowHandle<V>) -> Self {
        val.any_handle
    }
}

/// A handle to a window with any root view type, which can be downcast to a window with a specific root view type.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnyWindowHandle {
    pub(crate) id: WindowId,
    state_type: TypeId,
}

impl AnyWindowHandle {
    /// Get the ID of this window.
    pub fn window_id(&self) -> WindowId {
        self.id
    }

    /// Attempt to convert this handle to a window handle with a specific root view type.
    /// If the types do not match, this will return `None`.
    pub fn downcast<T: 'static>(&self) -> Option<WindowHandle<T>> {
        if TypeId::of::<T>() == self.state_type {
            Some(WindowHandle {
                any_handle: *self,
                state_type: PhantomData,
            })
        } else {
            None
        }
    }

    /// Updates the state of the root view of this window.
    ///
    /// This will fail if the window has been closed.
    pub fn update<C, R>(
        self,
        cx: &mut C,
        update: impl FnOnce(AnyView, &mut WindowContext<'_>) -> R,
    ) -> Result<R>
    where
        C: Context,
    {
        cx.update_window(self, update)
    }

    /// Read the state of the root view of this window.
    ///
    /// This will fail if the window has been closed.
    pub fn read<T, C, R>(self, cx: &C, read: impl FnOnce(View<T>, &AppContext) -> R) -> Result<R>
    where
        C: Context,
        T: 'static,
    {
        let view = self
            .downcast::<T>()
            .context("the type of the window's root view has changed")?;

        cx.read_window(&view, read)
    }
}

/// An identifier for an [`Element`](crate::Element).
///
/// Can be constructed with a string, a number, or both, as well
/// as other internal representations.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ElementId {
    /// The ID of a View element
    View(EntityId),
    /// An integer ID.
    Integer(usize),
    /// A string based ID.
    Name(SharedString),
    /// An ID that's equated with a focus handle.
    FocusHandle(FocusId),
    /// A combination of a name and an integer.
    NamedInteger(SharedString, usize),
}

impl Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementId::View(entity_id) => write!(f, "view-{}", entity_id)?,
            ElementId::Integer(ix) => write!(f, "{}", ix)?,
            ElementId::Name(name) => write!(f, "{}", name)?,
            ElementId::FocusHandle(_) => write!(f, "FocusHandle")?,
            ElementId::NamedInteger(s, i) => write!(f, "{}-{}", s, i)?,
        }

        Ok(())
    }
}

impl TryInto<SharedString> for ElementId {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<SharedString> {
        if let ElementId::Name(name) = self {
            Ok(name)
        } else {
            Err(anyhow!("element id is not string"))
        }
    }
}

impl From<usize> for ElementId {
    fn from(id: usize) -> Self {
        ElementId::Integer(id)
    }
}

impl From<i32> for ElementId {
    fn from(id: i32) -> Self {
        Self::Integer(id as usize)
    }
}

impl From<SharedString> for ElementId {
    fn from(name: SharedString) -> Self {
        ElementId::Name(name)
    }
}

impl From<&'static str> for ElementId {
    fn from(name: &'static str) -> Self {
        ElementId::Name(name.into())
    }
}

impl<'a> From<&'a FocusHandle> for ElementId {
    fn from(handle: &'a FocusHandle) -> Self {
        ElementId::FocusHandle(handle.id)
    }
}

impl From<(&'static str, EntityId)> for ElementId {
    fn from((name, id): (&'static str, EntityId)) -> Self {
        ElementId::NamedInteger(name.into(), id.as_u64() as usize)
    }
}

impl From<(&'static str, usize)> for ElementId {
    fn from((name, id): (&'static str, usize)) -> Self {
        ElementId::NamedInteger(name.into(), id)
    }
}

impl From<(&'static str, u64)> for ElementId {
    fn from((name, id): (&'static str, u64)) -> Self {
        ElementId::NamedInteger(name.into(), id as usize)
    }
}

/// A rectangle to be rendered in the window at the given position and size.
/// Passed as an argument [`ElementContext::paint_quad`].
#[derive(Clone)]
pub struct PaintQuad {
    bounds: Bounds<Pixels>,
    corner_radii: Corners<Pixels>,
    background: Hsla,
    border_widths: Edges<Pixels>,
    border_color: Hsla,
}

impl PaintQuad {
    /// Sets the corner radii of the quad.
    pub fn corner_radii(self, corner_radii: impl Into<Corners<Pixels>>) -> Self {
        PaintQuad {
            corner_radii: corner_radii.into(),
            ..self
        }
    }

    /// Sets the border widths of the quad.
    pub fn border_widths(self, border_widths: impl Into<Edges<Pixels>>) -> Self {
        PaintQuad {
            border_widths: border_widths.into(),
            ..self
        }
    }

    /// Sets the border color of the quad.
    pub fn border_color(self, border_color: impl Into<Hsla>) -> Self {
        PaintQuad {
            border_color: border_color.into(),
            ..self
        }
    }

    /// Sets the background color of the quad.
    pub fn background(self, background: impl Into<Hsla>) -> Self {
        PaintQuad {
            background: background.into(),
            ..self
        }
    }
}

/// Creates a quad with the given parameters.
pub fn quad(
    bounds: Bounds<Pixels>,
    corner_radii: impl Into<Corners<Pixels>>,
    background: impl Into<Hsla>,
    border_widths: impl Into<Edges<Pixels>>,
    border_color: impl Into<Hsla>,
) -> PaintQuad {
    PaintQuad {
        bounds,
        corner_radii: corner_radii.into(),
        background: background.into(),
        border_widths: border_widths.into(),
        border_color: border_color.into(),
    }
}

/// Creates a filled quad with the given bounds and background color.
pub fn fill(bounds: impl Into<Bounds<Pixels>>, background: impl Into<Hsla>) -> PaintQuad {
    PaintQuad {
        bounds: bounds.into(),
        corner_radii: (0.).into(),
        background: background.into(),
        border_widths: (0.).into(),
        border_color: transparent_black(),
    }
}

/// Creates a rectangle outline with the given bounds, border color, and a 1px border width
pub fn outline(bounds: impl Into<Bounds<Pixels>>, border_color: impl Into<Hsla>) -> PaintQuad {
    PaintQuad {
        bounds: bounds.into(),
        corner_radii: (0.).into(),
        background: transparent_black(),
        border_widths: (1.).into(),
        border_color: border_color.into(),
    }
}
