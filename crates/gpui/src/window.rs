#![deny(missing_docs)]

use crate::{
    px, size, transparent_black, Action, AnyDrag, AnyTooltip, AnyView, AppContext, Arena,
    AsyncWindowContext, AvailableSpace, Bounds, BoxShadow, Context, Corners, CursorStyle,
    DevicePixels, DispatchActionListener, DispatchNodeId, DispatchTree, DisplayId, Edges, Effect,
    Entity, EntityId, EventEmitter, FileDropEvent, Flatten, FontId, GlobalElementId, GlyphId, Hsla,
    ImageData, IsZero, KeyBinding, KeyContext, KeyDownEvent, KeyEvent, KeystrokeEvent, LayoutId,
    Model, ModelContext, Modifiers, MonochromeSprite, MouseButton, MouseEvent, MouseMoveEvent,
    MouseUpEvent, Path, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PolychromeSprite, PromptLevel, Quad, Render,
    RenderGlyphParams, RenderImageParams, RenderSvgParams, ScaledPixels, Scene, Shadow,
    SharedString, Size, Style, SubscriberSet, Subscription, Surface, TaffyLayoutEngine, Task,
    Underline, UnderlineStyle, View, VisualContext, WeakView, WindowBounds, WindowOptions,
    SUBPIXEL_VARIANTS,
};
use anyhow::{anyhow, Context as _, Result};
use collections::{FxHashMap, FxHashSet};
use derive_more::{Deref, DerefMut};
use futures::{
    channel::{mpsc, oneshot},
    StreamExt,
};
use media::core_video::CVImageBuffer;
use parking_lot::RwLock;
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut, Cow},
    cell::RefCell,
    collections::hash_map::Entry,
    fmt::{Debug, Display},
    future::Future,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use util::{post_inc, ResultExt};

const ACTIVE_DRAG_Z_INDEX: u8 = 1;

/// A global stacking order, which is created by stacking successive z-index values.
/// Each z-index will always be interpreted in the context of its parent z-index.
#[derive(Deref, DerefMut, Clone, Ord, PartialOrd, PartialEq, Eq, Default)]
pub struct StackingOrder {
    #[deref]
    #[deref_mut]
    context_stack: SmallVec<[u8; 64]>,
    id: u32,
}

impl std::fmt::Debug for StackingOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stacks = self.context_stack.iter().peekable();
        write!(f, "[({}): ", self.id)?;
        while let Some(z_index) = stacks.next() {
            write!(f, "{z_index}")?;
            if stacks.peek().is_some() {
                write!(f, "->")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

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
type AnyMouseListener = Box<dyn FnMut(&dyn Any, DispatchPhase, &mut WindowContext) + 'static>;
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
    pub(crate) static ELEMENT_ARENA: RefCell<Arena> = RefCell::new(Arena::new(4 * 1024 * 1024));
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

// Holds the state for a specific window.
#[doc(hidden)]
pub struct Window {
    pub(crate) handle: AnyWindowHandle,
    pub(crate) removed: bool,
    pub(crate) platform_window: Box<dyn PlatformWindow>,
    display_id: DisplayId,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    rem_size: Pixels,
    viewport_size: Size<Pixels>,
    layout_engine: Option<TaffyLayoutEngine>,
    pub(crate) root_view: Option<AnyView>,
    pub(crate) element_id_stack: GlobalElementId,
    pub(crate) rendered_frame: Frame,
    pub(crate) next_frame: Frame,
    pub(crate) dirty_views: FxHashSet<EntityId>,
    pub(crate) focus_handles: Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
    focus_listeners: SubscriberSet<(), AnyWindowFocusListener>,
    focus_lost_listeners: SubscriberSet<(), AnyObserver>,
    default_prevented: bool,
    mouse_position: Point<Pixels>,
    modifiers: Modifiers,
    scale_factor: f32,
    bounds: WindowBounds,
    bounds_observers: SubscriberSet<(), AnyObserver>,
    active: bool,
    pub(crate) dirty: bool,
    pub(crate) refreshing: bool,
    pub(crate) drawing: bool,
    activation_observers: SubscriberSet<(), AnyObserver>,
    pub(crate) focus: Option<FocusId>,
    focus_enabled: bool,

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) focus_invalidated: bool,
}

pub(crate) struct ElementStateBox {
    inner: Box<dyn Any>,
    parent_view_id: EntityId,
    #[cfg(debug_assertions)]
    type_name: &'static str,
}

struct RequestedInputHandler {
    view_id: EntityId,
    handler: Option<Box<dyn PlatformInputHandler>>,
}

struct TooltipRequest {
    view_id: EntityId,
    tooltip: AnyTooltip,
}

pub(crate) struct Frame {
    focus: Option<FocusId>,
    window_active: bool,
    pub(crate) element_states: FxHashMap<GlobalElementId, ElementStateBox>,
    mouse_listeners: FxHashMap<TypeId, Vec<(StackingOrder, EntityId, AnyMouseListener)>>,
    pub(crate) dispatch_tree: DispatchTree,
    pub(crate) scene: Scene,
    pub(crate) depth_map: Vec<(StackingOrder, EntityId, Bounds<Pixels>)>,
    pub(crate) z_index_stack: StackingOrder,
    pub(crate) next_stacking_order_id: u32,
    next_root_z_index: u8,
    content_mask_stack: Vec<ContentMask<Pixels>>,
    element_offset_stack: Vec<Point<Pixels>>,
    requested_input_handler: Option<RequestedInputHandler>,
    tooltip_request: Option<TooltipRequest>,
    cursor_styles: FxHashMap<EntityId, CursorStyle>,
    requested_cursor_style: Option<CursorStyle>,
    pub(crate) view_stack: Vec<EntityId>,
    pub(crate) reused_views: FxHashSet<EntityId>,

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) debug_bounds: collections::FxHashMap<String, Bounds<Pixels>>,
}

impl Frame {
    fn new(dispatch_tree: DispatchTree) -> Self {
        Frame {
            focus: None,
            window_active: false,
            element_states: FxHashMap::default(),
            mouse_listeners: FxHashMap::default(),
            dispatch_tree,
            scene: Scene::default(),
            depth_map: Vec::new(),
            z_index_stack: StackingOrder::default(),
            next_stacking_order_id: 0,
            next_root_z_index: 0,
            content_mask_stack: Vec::new(),
            element_offset_stack: Vec::new(),
            requested_input_handler: None,
            tooltip_request: None,
            cursor_styles: FxHashMap::default(),
            requested_cursor_style: None,
            view_stack: Vec::new(),
            reused_views: FxHashSet::default(),

            #[cfg(any(test, feature = "test-support"))]
            debug_bounds: FxHashMap::default(),
        }
    }

    fn clear(&mut self) {
        self.element_states.clear();
        self.mouse_listeners.values_mut().for_each(Vec::clear);
        self.dispatch_tree.clear();
        self.depth_map.clear();
        self.next_stacking_order_id = 0;
        self.next_root_z_index = 0;
        self.reused_views.clear();
        self.scene.clear();
        self.requested_input_handler.take();
        self.tooltip_request.take();
        self.cursor_styles.clear();
        self.requested_cursor_style.take();
        debug_assert_eq!(self.view_stack.len(), 0);
    }

    fn focus_path(&self) -> SmallVec<[FocusId; 8]> {
        self.focus
            .map(|focus_id| self.dispatch_tree.focus_path(focus_id))
            .unwrap_or_default()
    }

    fn finish(&mut self, prev_frame: &mut Self) {
        // Reuse mouse listeners that didn't change since the last frame.
        for (type_id, listeners) in &mut prev_frame.mouse_listeners {
            let next_listeners = self.mouse_listeners.entry(*type_id).or_default();
            for (order, view_id, listener) in listeners.drain(..) {
                if self.reused_views.contains(&view_id) {
                    next_listeners.push((order, view_id, listener));
                }
            }
        }

        // Reuse entries in the depth map that didn't change since the last frame.
        for (order, view_id, bounds) in prev_frame.depth_map.drain(..) {
            if self.reused_views.contains(&view_id) {
                match self
                    .depth_map
                    .binary_search_by(|(level, _, _)| order.cmp(level))
                {
                    Ok(i) | Err(i) => self.depth_map.insert(i, (order, view_id, bounds)),
                }
            }
        }

        // Retain element states for views that didn't change since the last frame.
        for (element_id, state) in prev_frame.element_states.drain() {
            if self.reused_views.contains(&state.parent_view_id) {
                self.element_states.entry(element_id).or_insert(state);
            }
        }

        // Reuse geometry that didn't change since the last frame.
        self.scene
            .reuse_views(&self.reused_views, &mut prev_frame.scene);
        self.scene.finish();
    }
}

impl Window {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        cx: &mut AppContext,
    ) -> Self {
        let platform_window = cx.platform.open_window(handle, options);
        let display_id = platform_window.display().id();
        let sprite_atlas = platform_window.sprite_atlas();
        let mouse_position = platform_window.mouse_position();
        let modifiers = platform_window.modifiers();
        let content_size = platform_window.content_size();
        let scale_factor = platform_window.scale_factor();
        let bounds = platform_window.bounds();

        platform_window.on_request_frame(Box::new({
            let mut cx = cx.to_async();
            move || {
                handle.update(&mut cx, |_, cx| cx.draw()).log_err();
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
        platform_window.on_active_status_change(Box::new({
            let mut cx = cx.to_async();
            move |active| {
                handle
                    .update(&mut cx, |_, cx| {
                        cx.window.active = active;
                        cx.window
                            .activation_observers
                            .clone()
                            .retain(&(), |callback| callback(cx));
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
                    .unwrap_or(false)
            })
        });

        Window {
            handle,
            removed: false,
            platform_window,
            display_id,
            sprite_atlas,
            rem_size: px(16.),
            viewport_size: content_size,
            layout_engine: Some(TaffyLayoutEngine::new()),
            root_view: None,
            element_id_stack: GlobalElementId::default(),
            rendered_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
            next_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
            dirty_views: FxHashSet::default(),
            focus_handles: Arc::new(RwLock::new(SlotMap::with_key())),
            focus_listeners: SubscriberSet::new(),
            focus_lost_listeners: SubscriberSet::new(),
            default_prevented: true,
            mouse_position,
            modifiers,
            scale_factor,
            bounds,
            bounds_observers: SubscriberSet::new(),
            active: false,
            dirty: false,
            refreshing: false,
            drawing: false,
            activation_observers: SubscriberSet::new(),
            focus: None,
            focus_enabled: true,

            #[cfg(any(test, feature = "test-support"))]
            focus_invalidated: false,
        }
    }
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
        if !self.window.drawing {
            self.window.refreshing = true;
            self.window.dirty = true;
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

        #[cfg(any(test, feature = "test-support"))]
        {
            self.window.focus_invalidated = true;
        }

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

    /// Dispatch the given action on the currently focused element.
    pub fn dispatch_action(&mut self, action: Box<dyn Action>) {
        let focus_handle = self.focused();

        self.defer(move |cx| {
            let node_id = focus_handle
                .and_then(|handle| {
                    cx.window
                        .rendered_frame
                        .dispatch_tree
                        .focusable_node_id(handle.id)
                })
                .unwrap_or_else(|| cx.window.rendered_frame.dispatch_tree.root_node_id());

            cx.propagate_event = true;
            cx.dispatch_action_on_node(node_id, action);
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
        let (subscription, activate) = self.app.event_listeners.insert(
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
        );
        self.app.defer(move |_| activate());
        subscription
    }

    /// Creates an [`AsyncWindowContext`], which has a static lifetime and can be held across
    /// await points in async code.
    pub fn to_async(&self) -> AsyncWindowContext {
        AsyncWindowContext::new(self.app.to_async(), self.window.handle)
    }

    /// Schedule the given closure to be run directly after the current frame is rendered.
    pub fn on_next_frame(&mut self, callback: impl FnOnce(&mut WindowContext) + 'static) {
        let handle = self.window.handle;
        let display_id = self.window.display_id;

        let mut frame_consumers = std::mem::take(&mut self.app.frame_consumers);
        if let Entry::Vacant(e) = frame_consumers.entry(display_id) {
            let (tx, mut rx) = mpsc::unbounded::<()>();
            self.platform.set_display_link_output_callback(
                display_id,
                Box::new(move |_current_time, _output_time| _ = tx.unbounded_send(())),
            );

            let consumer_task = self.app.spawn(|cx| async move {
                while rx.next().await.is_some() {
                    cx.update(|cx| {
                        for callback in cx
                            .next_frame_callbacks
                            .get_mut(&display_id)
                            .unwrap()
                            .drain(..)
                            .collect::<SmallVec<[_; 32]>>()
                        {
                            callback(cx);
                        }
                    })
                    .ok();

                    // Flush effects, then stop the display link if no new next_frame_callbacks have been added.

                    cx.update(|cx| {
                        if cx.next_frame_callbacks.is_empty() {
                            cx.platform.stop_display_link(display_id);
                        }
                    })
                    .ok();
                }
            });
            e.insert(consumer_task);
        }
        debug_assert!(self.app.frame_consumers.is_empty());
        self.app.frame_consumers = frame_consumers;

        if self.next_frame_callbacks.is_empty() {
            self.platform.start_display_link(display_id);
        }

        self.next_frame_callbacks
            .entry(display_id)
            .or_default()
            .push(Box::new(move |cx: &mut AppContext| {
                cx.update_window(handle, |_root_view, cx| callback(cx)).ok();
            }));
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

    /// Updates the global of the given type. The given closure is given simultaneous mutable
    /// access both to the global and the context.
    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

    #[must_use]
    /// Add a node to the layout tree for the current frame. Takes the `Style` of the element for which
    /// layout is being requested, along with the layout ids of any children. This method is called during
    /// calls to the `Element::layout` trait method and enables any element to participate in layout.
    pub fn request_layout(
        &mut self,
        style: &Style,
        children: impl IntoIterator<Item = LayoutId>,
    ) -> LayoutId {
        self.app.layout_id_buffer.clear();
        self.app.layout_id_buffer.extend(children);
        let rem_size = self.rem_size();

        self.window.layout_engine.as_mut().unwrap().request_layout(
            style,
            rem_size,
            &self.app.layout_id_buffer,
        )
    }

    /// Add a node to the layout tree for the current frame. Instead of taking a `Style` and children,
    /// this variant takes a function that is invoked during layout so you can use arbitrary logic to
    /// determine the element's size. One place this is used internally is when measuring text.
    ///
    /// The given closure is invoked at layout time with the known dimensions and available space and
    /// returns a `Size`.
    pub fn request_measured_layout<
        F: FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut WindowContext) -> Size<Pixels>
            + 'static,
    >(
        &mut self,
        style: Style,
        measure: F,
    ) -> LayoutId {
        let rem_size = self.rem_size();
        self.window
            .layout_engine
            .as_mut()
            .unwrap()
            .request_measured_layout(style, rem_size, measure)
    }

    pub(crate) fn layout_style(&self, layout_id: LayoutId) -> Option<&Style> {
        self.window
            .layout_engine
            .as_ref()
            .unwrap()
            .requested_style(layout_id)
    }

    /// Compute the layout for the given id within the given available space.
    /// This method is called for its side effect, typically by the framework prior to painting.
    /// After calling it, you can request the bounds of the given layout node id or any descendant.
    pub fn compute_layout(&mut self, layout_id: LayoutId, available_space: Size<AvailableSpace>) {
        let mut layout_engine = self.window.layout_engine.take().unwrap();
        layout_engine.compute_layout(layout_id, available_space, self);
        self.window.layout_engine = Some(layout_engine);
    }

    /// Obtain the bounds computed for the given LayoutId relative to the window. This method should not
    /// be invoked until the paint phase begins, and will usually be invoked by GPUI itself automatically
    /// in order to pass your element its `Bounds` automatically.
    pub fn layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels> {
        let mut bounds = self
            .window
            .layout_engine
            .as_mut()
            .unwrap()
            .layout_bounds(layout_id)
            .map(Into::into);
        bounds.origin += self.element_offset();
        bounds
    }

    fn window_bounds_changed(&mut self) {
        self.window.scale_factor = self.window.platform_window.scale_factor();
        self.window.viewport_size = self.window.platform_window.content_size();
        self.window.bounds = self.window.platform_window.bounds();
        self.window.display_id = self.window.platform_window.display().id();
        self.refresh();

        self.window
            .bounds_observers
            .clone()
            .retain(&(), |callback| callback(self));
    }

    /// Returns the bounds of the current window in the global coordinate space, which could span across multiple displays.
    pub fn window_bounds(&self) -> WindowBounds {
        self.window.bounds
    }

    /// Returns the size of the drawable area within the window.
    pub fn viewport_size(&self) -> Size<Pixels> {
        self.window.viewport_size
    }

    /// Returns whether this window is focused by the operating system (receiving key events).
    pub fn is_window_active(&self) -> bool {
        self.window.active
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

    /// Register a mouse event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    pub fn on_mouse_event<Event: MouseEvent>(
        &mut self,
        mut handler: impl FnMut(&Event, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        let view_id = self.parent_view_id();
        let order = self.window.next_frame.z_index_stack.clone();
        self.window
            .next_frame
            .mouse_listeners
            .entry(TypeId::of::<Event>())
            .or_default()
            .push((
                order,
                view_id,
                Box::new(
                    move |event: &dyn Any, phase: DispatchPhase, cx: &mut WindowContext<'_>| {
                        handler(event.downcast_ref().unwrap(), phase, cx)
                    },
                ),
            ))
    }

    /// Register a key event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_key_event<Event: KeyEvent>(
        &mut self,
        listener: impl Fn(&Event, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        self.window.next_frame.dispatch_tree.on_key_event(Rc::new(
            move |event: &dyn Any, phase, cx: &mut WindowContext<'_>| {
                if let Some(event) = event.downcast_ref::<Event>() {
                    listener(event, phase, cx)
                }
            },
        ));
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

    /// Updates the cursor style at the platform level.
    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        let view_id = self.parent_view_id();
        self.window.next_frame.cursor_styles.insert(view_id, style);
        self.window.next_frame.requested_cursor_style = Some(style);
    }

    /// Sets a tooltip to be rendered for the upcoming frame
    pub fn set_tooltip(&mut self, tooltip: AnyTooltip) {
        let view_id = self.parent_view_id();
        self.window.next_frame.tooltip_request = Some(TooltipRequest { view_id, tooltip });
    }

    /// Called during painting to track which z-index is on top at each pixel position
    pub fn add_opaque_layer(&mut self, bounds: Bounds<Pixels>) {
        let stacking_order = self.window.next_frame.z_index_stack.clone();
        let view_id = self.parent_view_id();
        let depth_map = &mut self.window.next_frame.depth_map;
        match depth_map.binary_search_by(|(level, _, _)| stacking_order.cmp(level)) {
            Ok(i) | Err(i) => depth_map.insert(i, (stacking_order, view_id, bounds)),
        }
    }

    /// Returns true if there is no opaque layer containing the given point
    /// on top of the given level. Layers whose level is an extension of the
    /// level are not considered to be on top of the level.
    pub fn was_top_layer(&self, point: &Point<Pixels>, level: &StackingOrder) -> bool {
        for (opaque_level, _, bounds) in self.window.rendered_frame.depth_map.iter() {
            if level >= opaque_level {
                break;
            }

            if bounds.contains(point) && !opaque_level.starts_with(level) {
                return false;
            }
        }
        true
    }

    pub(crate) fn was_top_layer_under_active_drag(
        &self,
        point: &Point<Pixels>,
        level: &StackingOrder,
    ) -> bool {
        for (opaque_level, _, bounds) in self.window.rendered_frame.depth_map.iter() {
            if level >= opaque_level {
                break;
            }
            if opaque_level.starts_with(&[ACTIVE_DRAG_Z_INDEX]) {
                continue;
            }

            if bounds.contains(point) && !opaque_level.starts_with(level) {
                return false;
            }
        }
        true
    }

    /// Called during painting to get the current stacking order.
    pub fn stacking_order(&self) -> &StackingOrder {
        &self.window.next_frame.z_index_stack
    }

    /// Paint one or more drop shadows into the scene for the next frame at the current z-index.
    pub fn paint_shadows(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        shadows: &[BoxShadow],
    ) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let view_id = self.parent_view_id();
        let window = &mut *self.window;
        for shadow in shadows {
            let mut shadow_bounds = bounds;
            shadow_bounds.origin += shadow.offset;
            shadow_bounds.dilate(shadow.spread_radius);
            window.next_frame.scene.insert(
                &window.next_frame.z_index_stack,
                Shadow {
                    view_id: view_id.into(),
                    layer_id: 0,
                    order: 0,
                    bounds: shadow_bounds.scale(scale_factor),
                    content_mask: content_mask.scale(scale_factor),
                    corner_radii: corner_radii.scale(scale_factor),
                    color: shadow.color,
                    blur_radius: shadow.blur_radius.scale(scale_factor),
                },
            );
        }
    }

    /// Paint one or more quads into the scene for the next frame at the current stacking context.
    /// Quads are colored rectangular regions with an optional background, border, and corner radius.
    /// see [`fill`], [`outline`], and [`quad`] to construct this type.
    pub fn paint_quad(&mut self, quad: PaintQuad) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            Quad {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds: quad.bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                background: quad.background,
                border_color: quad.border_color,
                corner_radii: quad.corner_radii.scale(scale_factor),
                border_widths: quad.border_widths.scale(scale_factor),
            },
        );
    }

    /// Paint the given `Path` into the scene for the next frame at the current z-index.
    pub fn paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Hsla>) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let view_id = self.parent_view_id();

        path.content_mask = content_mask;
        path.color = color.into();
        path.view_id = view_id.into();
        let window = &mut *self.window;
        window
            .next_frame
            .scene
            .insert(&window.next_frame.z_index_stack, path.scale(scale_factor));
    }

    /// Paint an underline into the scene for the next frame at the current z-index.
    pub fn paint_underline(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &UnderlineStyle,
    ) {
        let scale_factor = self.scale_factor();
        let height = if style.wavy {
            style.thickness * 3.
        } else {
            style.thickness
        };
        let bounds = Bounds {
            origin,
            size: size(width, height),
        };
        let content_mask = self.content_mask();
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            Underline {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds: bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                thickness: style.thickness.scale(scale_factor),
                color: style.color.unwrap_or_default(),
                wavy: style.wavy,
            },
        );
    }

    /// Paint a monochrome (non-emoji) glyph into the scene for the next frame at the current z-index.
    /// The y component of the origin is the baseline of the glyph.
    pub fn paint_glyph(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
        color: Hsla,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let glyph_origin = origin.scale(scale_factor);
        let subpixel_variant = Point {
            x: (glyph_origin.x.0.fract() * SUBPIXEL_VARIANTS as f32).floor() as u8,
            y: (glyph_origin.y.0.fract() * SUBPIXEL_VARIANTS as f32).floor() as u8,
        };
        let params = RenderGlyphParams {
            font_id,
            glyph_id,
            font_size,
            subpixel_variant,
            scale_factor,
            is_emoji: false,
        };

        let raster_bounds = self.text_system().raster_bounds(&params)?;
        if !raster_bounds.is_zero() {
            let tile =
                self.window
                    .sprite_atlas
                    .get_or_insert_with(&params.clone().into(), &mut || {
                        let (size, bytes) = self.text_system().rasterize_glyph(&params)?;
                        Ok((size, Cow::Owned(bytes)))
                    })?;
            let bounds = Bounds {
                origin: glyph_origin.map(|px| px.floor()) + raster_bounds.origin.map(Into::into),
                size: tile.bounds.size.map(Into::into),
            };
            let content_mask = self.content_mask().scale(scale_factor);
            let view_id = self.parent_view_id();
            let window = &mut *self.window;
            window.next_frame.scene.insert(
                &window.next_frame.z_index_stack,
                MonochromeSprite {
                    view_id: view_id.into(),
                    layer_id: 0,
                    order: 0,
                    bounds,
                    content_mask,
                    color,
                    tile,
                },
            );
        }
        Ok(())
    }

    /// Paint an emoji glyph into the scene for the next frame at the current z-index.
    /// The y component of the origin is the baseline of the glyph.
    pub fn paint_emoji(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let glyph_origin = origin.scale(scale_factor);
        let params = RenderGlyphParams {
            font_id,
            glyph_id,
            font_size,
            // We don't render emojis with subpixel variants.
            subpixel_variant: Default::default(),
            scale_factor,
            is_emoji: true,
        };

        let raster_bounds = self.text_system().raster_bounds(&params)?;
        if !raster_bounds.is_zero() {
            let tile =
                self.window
                    .sprite_atlas
                    .get_or_insert_with(&params.clone().into(), &mut || {
                        let (size, bytes) = self.text_system().rasterize_glyph(&params)?;
                        Ok((size, Cow::Owned(bytes)))
                    })?;
            let bounds = Bounds {
                origin: glyph_origin.map(|px| px.floor()) + raster_bounds.origin.map(Into::into),
                size: tile.bounds.size.map(Into::into),
            };
            let content_mask = self.content_mask().scale(scale_factor);
            let view_id = self.parent_view_id();
            let window = &mut *self.window;

            window.next_frame.scene.insert(
                &window.next_frame.z_index_stack,
                PolychromeSprite {
                    view_id: view_id.into(),
                    layer_id: 0,
                    order: 0,
                    bounds,
                    corner_radii: Default::default(),
                    content_mask,
                    tile,
                    grayscale: false,
                },
            );
        }
        Ok(())
    }

    /// Paint a monochrome SVG into the scene for the next frame at the current stacking context.
    pub fn paint_svg(
        &mut self,
        bounds: Bounds<Pixels>,
        path: SharedString,
        color: Hsla,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        // Render the SVG at twice the size to get a higher quality result.
        let params = RenderSvgParams {
            path,
            size: bounds
                .size
                .map(|pixels| DevicePixels::from((pixels.0 * 2.).ceil() as i32)),
        };

        let tile =
            self.window
                .sprite_atlas
                .get_or_insert_with(&params.clone().into(), &mut || {
                    let bytes = self.svg_renderer.render(&params)?;
                    Ok((params.size, Cow::Owned(bytes)))
                })?;
        let content_mask = self.content_mask().scale(scale_factor);
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            MonochromeSprite {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds,
                content_mask,
                color,
                tile,
            },
        );

        Ok(())
    }

    /// Paint an image into the scene for the next frame at the current z-index.
    pub fn paint_image(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        data: Arc<ImageData>,
        grayscale: bool,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let params = RenderImageParams { image_id: data.id };

        let tile = self
            .window
            .sprite_atlas
            .get_or_insert_with(&params.clone().into(), &mut || {
                Ok((data.size(), Cow::Borrowed(data.as_bytes())))
            })?;
        let content_mask = self.content_mask().scale(scale_factor);
        let corner_radii = corner_radii.scale(scale_factor);
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            PolychromeSprite {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds,
                content_mask,
                corner_radii,
                tile,
                grayscale,
            },
        );
        Ok(())
    }

    /// Paint a surface into the scene for the next frame at the current z-index.
    pub fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVImageBuffer) {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let content_mask = self.content_mask().scale(scale_factor);
        let view_id = self.parent_view_id();
        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            Surface {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds,
                content_mask,
                image_buffer,
            },
        );
    }

    pub(crate) fn reuse_view(&mut self) {
        let view_id = self.parent_view_id();
        let grafted_view_ids = self
            .window
            .next_frame
            .dispatch_tree
            .reuse_view(view_id, &mut self.window.rendered_frame.dispatch_tree);
        for view_id in grafted_view_ids {
            assert!(self.window.next_frame.reused_views.insert(view_id));

            // Reuse the previous input handler requested during painting of the reused view.
            if self
                .window
                .rendered_frame
                .requested_input_handler
                .as_ref()
                .map_or(false, |requested| requested.view_id == view_id)
            {
                self.window.next_frame.requested_input_handler =
                    self.window.rendered_frame.requested_input_handler.take();
            }

            // Reuse the tooltip previously requested during painting of the reused view.
            if self
                .window
                .rendered_frame
                .tooltip_request
                .as_ref()
                .map_or(false, |requested| requested.view_id == view_id)
            {
                self.window.next_frame.tooltip_request =
                    self.window.rendered_frame.tooltip_request.take();
            }

            // Reuse the cursor styles previously requested during painting of the reused view.
            if let Some(style) = self.window.rendered_frame.cursor_styles.remove(&view_id) {
                self.window.next_frame.cursor_styles.insert(view_id, style);
                self.window.next_frame.requested_cursor_style = Some(style);
            }
        }
    }

    /// Draw pixels to the display for this window based on the contents of its scene.
    pub(crate) fn draw(&mut self) {
        self.window.dirty = false;
        self.window.drawing = true;

        #[cfg(any(test, feature = "test-support"))]
        {
            self.window.focus_invalidated = false;
        }

        if let Some(requested_handler) = self.window.rendered_frame.requested_input_handler.as_mut()
        {
            requested_handler.handler = self.window.platform_window.take_input_handler();
        }

        let root_view = self.window.root_view.take().unwrap();

        self.with_z_index(0, |cx| {
            cx.with_key_dispatch(Some(KeyContext::default()), None, |_, cx| {
                for (action_type, action_listeners) in &cx.app.global_action_listeners {
                    for action_listener in action_listeners.iter().cloned() {
                        cx.window.next_frame.dispatch_tree.on_action(
                            *action_type,
                            Rc::new(move |action: &dyn Any, phase, cx: &mut WindowContext<'_>| {
                                action_listener(action, phase, cx)
                            }),
                        )
                    }
                }

                let available_space = cx.window.viewport_size.map(Into::into);
                root_view.draw(Point::default(), available_space, cx);
            })
        });

        if let Some(active_drag) = self.app.active_drag.take() {
            self.with_z_index(ACTIVE_DRAG_Z_INDEX, |cx| {
                let offset = cx.mouse_position() - active_drag.cursor_offset;
                let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
                active_drag.view.draw(offset, available_space, cx);
            });
            self.active_drag = Some(active_drag);
        } else if let Some(tooltip_request) = self.window.next_frame.tooltip_request.take() {
            self.with_z_index(1, |cx| {
                let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
                tooltip_request.tooltip.view.draw(
                    tooltip_request.tooltip.cursor_offset,
                    available_space,
                    cx,
                );
            });
            self.window.next_frame.tooltip_request = Some(tooltip_request);
        }
        self.window.dirty_views.clear();

        self.window
            .next_frame
            .dispatch_tree
            .preserve_pending_keystrokes(
                &mut self.window.rendered_frame.dispatch_tree,
                self.window.focus,
            );
        self.window.next_frame.focus = self.window.focus;
        self.window.next_frame.window_active = self.window.active;
        self.window.root_view = Some(root_view);

        // Set the cursor only if we're the active window.
        let cursor_style = self
            .window
            .next_frame
            .requested_cursor_style
            .take()
            .unwrap_or(CursorStyle::Arrow);
        if self.is_window_active() {
            self.platform.set_cursor_style(cursor_style);
        }

        // Register requested input handler with the platform window.
        if let Some(requested_input) = self.window.next_frame.requested_input_handler.as_mut() {
            if let Some(handler) = requested_input.handler.take() {
                self.window.platform_window.set_input_handler(handler);
            }
        }

        self.window.layout_engine.as_mut().unwrap().clear();
        self.text_system()
            .finish_frame(&self.window.next_frame.reused_views);
        self.window
            .next_frame
            .finish(&mut self.window.rendered_frame);
        ELEMENT_ARENA.with_borrow_mut(|element_arena| element_arena.clear());

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

        self.window
            .platform_window
            .draw(&self.window.rendered_frame.scene);
        self.window.refreshing = false;
        self.window.drawing = false;
    }

    /// Dispatch a mouse or keyboard event on the window.
    pub fn dispatch_event(&mut self, event: PlatformInput) -> bool {
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
                FileDropEvent::Exited => PlatformInput::MouseUp(MouseUpEvent {
                    button: MouseButton::Left,
                    position: Point::default(),
                    modifiers: Modifiers::default(),
                    click_count: 1,
                }),
            },
            PlatformInput::KeyDown(_) | PlatformInput::KeyUp(_) => event,
        };

        if let Some(any_mouse_event) = event.mouse_event() {
            self.dispatch_mouse_event(any_mouse_event);
        } else if let Some(any_key_event) = event.keyboard_event() {
            self.dispatch_key_event(any_key_event);
        }

        !self.app.propagate_event
    }

    fn dispatch_mouse_event(&mut self, event: &dyn Any) {
        if let Some(mut handlers) = self
            .window
            .rendered_frame
            .mouse_listeners
            .remove(&event.type_id())
        {
            // Because handlers may add other handlers, we sort every time.
            handlers.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

            // Capture phase, events bubble from back to front. Handlers for this phase are used for
            // special purposes, such as detecting events outside of a given Bounds.
            for (_, _, handler) in &mut handlers {
                handler(event, DispatchPhase::Capture, self);
                if !self.app.propagate_event {
                    break;
                }
            }

            // Bubble phase, where most normal handlers do their work.
            if self.app.propagate_event {
                for (_, _, handler) in handlers.iter_mut().rev() {
                    handler(event, DispatchPhase::Bubble, self);
                    if !self.app.propagate_event {
                        break;
                    }
                }
            }

            self.window
                .rendered_frame
                .mouse_listeners
                .insert(event.type_id(), handlers);
        }

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

        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        let mut context_stack: SmallVec<[KeyContext; 16]> = SmallVec::new();
        for node_id in &dispatch_path {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);

            if let Some(context) = node.context.clone() {
                context_stack.push(context);
            }
        }

        for node_id in dispatch_path.iter().rev() {
            // Match keystrokes
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);
            if node.context.is_some() {
                if let Some(key_down_event) = event.downcast_ref::<KeyDownEvent>() {
                    let mut new_actions = self
                        .window
                        .rendered_frame
                        .dispatch_tree
                        .dispatch_key(&key_down_event.keystroke, &context_stack);
                    actions.append(&mut new_actions);
                }

                context_stack.pop();
            }
        }

        if !actions.is_empty() {
            self.clear_pending_keystrokes();
        }

        self.propagate_event = true;
        for action in actions {
            self.dispatch_action_on_node(node_id, action.boxed_clone());
            if !self.propagate_event {
                self.dispatch_keystroke_observers(event, Some(action));
                return;
            }
        }

        // Capture phase
        for node_id in &dispatch_path {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);

            for key_listener in node.key_listeners.clone() {
                key_listener(event, DispatchPhase::Capture, self);
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
                key_listener(event, DispatchPhase::Bubble, self);
                if !self.propagate_event {
                    return;
                }
            }
        }

        self.dispatch_keystroke_observers(event, None);
    }

    /// Determine whether a potential multi-stroke key binding is in progress on this window.
    pub fn has_pending_keystrokes(&self) -> bool {
        self.window
            .rendered_frame
            .dispatch_tree
            .has_pending_keystrokes()
    }

    fn dispatch_action_on_node(&mut self, node_id: DispatchNodeId, action: Box<dyn Action>) {
        let dispatch_path = self
            .window
            .rendered_frame
            .dispatch_tree
            .dispatch_path(node_id);

        // Capture phase
        for node_id in &dispatch_path {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);
            for DispatchActionListener {
                action_type,
                listener,
            } in node.action_listeners.clone()
            {
                let any_action = action.as_any();
                if action_type == any_action.type_id() {
                    listener(any_action, DispatchPhase::Capture, self);
                    if !self.propagate_event {
                        return;
                    }
                }
            }
        }
        // Bubble phase
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
                    listener(any_action, DispatchPhase::Bubble, self);
                    if !self.propagate_event {
                        return;
                    }
                }
            }
        }
    }

    /// Register the given handler to be invoked whenever the global of the given type
    /// is updated.
    pub fn observe_global<G: 'static>(
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
    pub fn toggle_full_screen(&self) {
        self.window.platform_window.toggle_full_screen();
    }

    /// Present a platform dialog.
    /// The provided message will be presented, along with buttons for each answer.
    /// When a button is clicked, the returned Receiver will receive the index of the clicked button.
    pub fn prompt(
        &self,
        level: PromptLevel,
        message: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        self.window.platform_window.prompt(level, message, answers)
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

        self.window
            .rendered_frame
            .dispatch_tree
            .available_actions(node_id)
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
        let context_stack = dispatch_tree
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

    /// Invoke the given function with the given focus handle present on the key dispatch stack.
    /// If you want an element to participate in key dispatch, use this method to push its key context and focus handle into the stack during paint.
    pub fn with_key_dispatch<R>(
        &mut self,
        context: Option<KeyContext>,
        focus_handle: Option<FocusHandle>,
        f: impl FnOnce(Option<FocusHandle>, &mut Self) -> R,
    ) -> R {
        let window = &mut self.window;
        let focus_id = focus_handle.as_ref().map(|handle| handle.id);
        window
            .next_frame
            .dispatch_tree
            .push_node(context.clone(), focus_id, None);

        let result = f(focus_handle, self);

        self.window.next_frame.dispatch_tree.pop_node();

        result
    }

    /// Invoke the given function with the given view id present on the view stack.
    /// This is a fairly low-level method used to layout views.
    pub fn with_view_id<R>(&mut self, view_id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R {
        let text_system = self.text_system().clone();
        text_system.with_view(view_id, || {
            if self.window.next_frame.view_stack.last() == Some(&view_id) {
                return f(self);
            } else {
                self.window.next_frame.view_stack.push(view_id);
                let result = f(self);
                self.window.next_frame.view_stack.pop();
                result
            }
        })
    }

    /// Invoke the given function with the given view id present on the view stack.
    /// This is a fairly low-level method used to paint views.
    pub fn paint_view<R>(&mut self, view_id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R {
        let text_system = self.text_system().clone();
        text_system.with_view(view_id, || {
            if self.window.next_frame.view_stack.last() == Some(&view_id) {
                return f(self);
            } else {
                self.window.next_frame.view_stack.push(view_id);
                self.window
                    .next_frame
                    .dispatch_tree
                    .push_node(None, None, Some(view_id));
                let result = f(self);
                self.window.next_frame.dispatch_tree.pop_node();
                self.window.next_frame.view_stack.pop();
                result
            }
        })
    }

    /// Updates or initializes state for an element with the given id that lives across multiple
    /// frames. If an element with this ID existed in the rendered frame, its state will be passed
    /// to the given closure. The state returned by the closure will be stored so it can be referenced
    /// when drawing the next frame.
    pub(crate) fn with_element_state<S, R>(
        &mut self,
        id: ElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static,
    {
        self.with_element_id(Some(id), |cx| {
            let global_id = cx.window().element_id_stack.clone();

            if let Some(any) = cx
                .window_mut()
                .next_frame
                .element_states
                .remove(&global_id)
                .or_else(|| {
                    cx.window_mut()
                        .rendered_frame
                        .element_states
                        .remove(&global_id)
                })
            {
                let ElementStateBox {
                    inner,
                    parent_view_id,
                    #[cfg(debug_assertions)]
                    type_name
                } = any;
                // Using the extra inner option to avoid needing to reallocate a new box.
                let mut state_box = inner
                    .downcast::<Option<S>>()
                    .map_err(|_| {
                        #[cfg(debug_assertions)]
                        {
                            anyhow!(
                                "invalid element state type for id, requested_type {:?}, actual type: {:?}",
                                std::any::type_name::<S>(),
                                type_name
                            )
                        }

                        #[cfg(not(debug_assertions))]
                        {
                            anyhow!(
                                "invalid element state type for id, requested_type {:?}",
                                std::any::type_name::<S>(),
                            )
                        }
                    })
                    .unwrap();

                // Actual: Option<AnyElement> <- View
                // Requested: () <- AnyElement
                let state = state_box
                    .take()
                    .expect("element state is already on the stack");
                let (result, state) = f(Some(state), cx);
                state_box.replace(state);
                cx.window_mut()
                    .next_frame
                    .element_states
                    .insert(global_id, ElementStateBox {
                        inner: state_box,
                        parent_view_id,
                        #[cfg(debug_assertions)]
                        type_name
                    });
                result
            } else {
                let (result, state) = f(None, cx);
                let parent_view_id = cx.parent_view_id();
                cx.window_mut()
                    .next_frame
                    .element_states
                    .insert(global_id,
                        ElementStateBox {
                            inner: Box::new(Some(state)),
                            parent_view_id,
                            #[cfg(debug_assertions)]
                            type_name: std::any::type_name::<S>()
                        }

                    );
                result
            }
        })
    }

    fn parent_view_id(&self) -> EntityId {
        *self
            .window
            .next_frame
            .view_stack
            .last()
            .expect("a view should always be on the stack while drawing")
    }

    /// Sets an input handler, such as [`ElementInputHandler`][element_input_handler], which interfaces with the
    /// platform to receive textual input with proper integration with concerns such
    /// as IME interactions. This handler will be active for the upcoming frame until the following frame is
    /// rendered.
    ///
    /// [element_input_handler]: crate::ElementInputHandler
    pub fn handle_input(
        &mut self,
        focus_handle: &FocusHandle,
        input_handler: impl PlatformInputHandler,
    ) {
        if focus_handle.is_focused(self) {
            let view_id = self.parent_view_id();
            self.window.next_frame.requested_input_handler = Some(RequestedInputHandler {
                view_id,
                handler: Some(Box::new(input_handler)),
            })
        }
    }

    /// Register a callback that can interrupt the closing of the current window based the returned boolean.
    /// If the callback returns false, the window won't be closed.
    pub fn on_window_should_close(&mut self, f: impl Fn(&mut WindowContext) -> bool + 'static) {
        let mut this = self.to_async();
        self.window
            .platform_window
            .on_should_close(Box::new(move || {
                this.update(|_, cx| {
                    // Ensure that the window is removed from the app if it's been closed
                    // by always pre-empting the system close event.
                    if f(cx) {
                        cx.remove_window();
                    }
                    false
                })
                .unwrap_or(true)
            }))
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

        cx.new_view_observers
            .clone()
            .retain(&TypeId::of::<V>(), |observer| {
                let any_view = AnyView::from(view.clone());
                (observer)(any_view, self);
                true
            });

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

    /// Pushes the given element id onto the global stack and invokes the given closure
    /// with a `GlobalElementId`, which disambiguates the given id in the context of its ancestor
    /// ids. Because elements are discarded and recreated on each frame, the `GlobalElementId` is
    /// used to associate state with identified elements across separate frames.
    fn with_element_id<R>(
        &mut self,
        id: Option<impl Into<ElementId>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(id) = id.map(Into::into) {
            let window = self.window_mut();
            window.element_id_stack.push(id);
            let result = f(self);
            let window: &mut Window = self.borrow_mut();
            window.element_id_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Invoke the given function with the given content mask after intersecting it
    /// with the current mask.
    fn with_content_mask<R>(
        &mut self,
        mask: Option<ContentMask<Pixels>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(mask) = mask {
            let mask = mask.intersect(&self.content_mask());
            self.window_mut().next_frame.content_mask_stack.push(mask);
            let result = f(self);
            self.window_mut().next_frame.content_mask_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Invoke the given function with the content mask reset to that
    /// of the window.
    fn break_content_mask<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let mask = ContentMask {
            bounds: Bounds {
                origin: Point::default(),
                size: self.window().viewport_size,
            },
        };
        let new_stacking_order_id =
            post_inc(&mut self.window_mut().next_frame.next_stacking_order_id);
        let new_root_z_index = post_inc(&mut self.window_mut().next_frame.next_root_z_index);
        let old_stacking_order = mem::take(&mut self.window_mut().next_frame.z_index_stack);
        self.window_mut().next_frame.z_index_stack.id = new_stacking_order_id;
        self.window_mut()
            .next_frame
            .z_index_stack
            .push(new_root_z_index);
        self.window_mut().next_frame.content_mask_stack.push(mask);
        let result = f(self);
        self.window_mut().next_frame.content_mask_stack.pop();
        self.window_mut().next_frame.z_index_stack = old_stacking_order;
        result
    }

    /// Called during painting to invoke the given closure in a new stacking context. The given
    /// z-index is interpreted relative to the previous call to `stack`.
    fn with_z_index<R>(&mut self, z_index: u8, f: impl FnOnce(&mut Self) -> R) -> R {
        let new_stacking_order_id =
            post_inc(&mut self.window_mut().next_frame.next_stacking_order_id);
        let old_stacking_order_id = mem::replace(
            &mut self.window_mut().next_frame.z_index_stack.id,
            new_stacking_order_id,
        );
        self.window_mut().next_frame.z_index_stack.id = new_stacking_order_id;
        self.window_mut().next_frame.z_index_stack.push(z_index);
        let result = f(self);
        self.window_mut().next_frame.z_index_stack.id = old_stacking_order_id;
        self.window_mut().next_frame.z_index_stack.pop();
        result
    }

    /// Updates the global element offset relative to the current offset. This is used to implement
    /// scrolling.
    fn with_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if offset.is_zero() {
            return f(self);
        };

        let abs_offset = self.element_offset() + offset;
        self.with_absolute_element_offset(abs_offset, f)
    }

    /// Updates the global element offset based on the given offset. This is used to implement
    /// drag handles and other manual painting of elements.
    fn with_absolute_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.window_mut()
            .next_frame
            .element_offset_stack
            .push(offset);
        let result = f(self);
        self.window_mut().next_frame.element_offset_stack.pop();
        result
    }

    /// Obtain the current element offset.
    fn element_offset(&self) -> Point<Pixels> {
        self.window()
            .next_frame
            .element_offset_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    /// Obtain the current content mask.
    fn content_mask(&self) -> ContentMask<Pixels> {
        self.window()
            .next_frame
            .content_mask_stack
            .last()
            .cloned()
            .unwrap_or_else(|| ContentMask {
                bounds: Bounds {
                    origin: Point::default(),
                    size: self.window().viewport_size,
                },
            })
    }

    /// The size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    fn rem_size(&self) -> Pixels {
        self.window().rem_size
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
        let (subscription, activate) = self.app.observers.insert(
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
        );
        self.app.defer(move |_| activate());
        subscription
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
        let (subscription, activate) = self.app.event_listeners.insert(
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
        );
        self.app.defer(move |_| activate());
        subscription
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
        for view_id in self
            .window
            .rendered_frame
            .dispatch_tree
            .view_path(self.view.entity_id())
            .into_iter()
            .rev()
        {
            if !self.window.dirty_views.insert(view_id) {
                break;
            }
        }

        if !self.window.drawing {
            self.window_cx.window.dirty = true;
            self.window_cx.app.push_effect(Effect::Notify {
                emitter: self.view.model.entity_id,
            });
        }
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

    /// Register a listener to be called when the given focus handle receives focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus(
        &mut self,
        handle: &FocusHandle,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let focus_id = handle.id;
        let (subscription, activate) = self.window.focus_listeners.insert(
            (),
            Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.last() != Some(&focus_id)
                        && event.current_focus_path.last() == Some(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }),
        );
        self.app.defer(move |_| activate());
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
        let (subscription, activate) = self.window.focus_listeners.insert(
            (),
            Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if !event.previous_focus_path.contains(&focus_id)
                        && event.current_focus_path.contains(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }),
        );
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
        let (subscription, activate) = self.window.focus_listeners.insert(
            (),
            Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.last() == Some(&focus_id)
                        && event.current_focus_path.last() != Some(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }),
        );
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
        let (subscription, activate) = self.window.focus_listeners.insert(
            (),
            Box::new(move |event, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.contains(&focus_id)
                        && !event.current_focus_path.contains(&focus_id)
                    {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }),
        );
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

    /// Updates the global state of the given type.
    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

    /// Register a callback to be invoked when the given global state changes.
    pub fn observe_global<G: 'static>(
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

    /// Add a listener for any mouse event that occurs in the window.
    /// This is a fairly low level method.
    /// Typically, you'll want to use methods on UI elements, which perform bounds checking etc.
    pub fn on_mouse_event<Event: MouseEvent>(
        &mut self,
        handler: impl Fn(&mut V, &Event, DispatchPhase, &mut ViewContext<V>) + 'static,
    ) {
        let handle = self.view().clone();
        self.window_cx.on_mouse_event(move |event, phase, cx| {
            handle.update(cx, |view, cx| {
                handler(view, event, phase, cx);
            })
        });
    }

    /// Register a callback to be invoked when the given Key Event is dispatched to the window.
    pub fn on_key_event<Event: KeyEvent>(
        &mut self,
        handler: impl Fn(&mut V, &Event, DispatchPhase, &mut ViewContext<V>) + 'static,
    ) {
        let handle = self.view().clone();
        self.window_cx.on_key_event(move |event, phase, cx| {
            handle.update(cx, |view, cx| {
                handler(view, event, phase, cx);
            })
        });
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
    /// Will return `None` if the window is closed.
    pub fn is_active(&self, cx: &AppContext) -> Option<bool> {
        cx.windows
            .get(self.id)
            .and_then(|window| window.as_ref().map(|window| window.active))
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
            ElementId::FocusHandle(__) => write!(f, "FocusHandle")?,
            ElementId::NamedInteger(s, i) => write!(f, "{}-{}", s, i)?,
        }

        Ok(())
    }
}

impl ElementId {
    pub(crate) fn from_entity_id(entity_id: EntityId) -> Self {
        ElementId::View(entity_id)
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
/// Passed as an argument [`WindowContext::paint_quad`].
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
