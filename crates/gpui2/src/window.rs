use crate::{
    key_dispatch::DispatchActionListener, px, size, Action, AnyDrag, AnyView, AppContext,
    AsyncWindowContext, AvailableSpace, Bounds, BoxShadow, Context, Corners, CursorStyle,
    DevicePixels, DispatchNodeId, DispatchTree, DisplayId, Edges, Effect, Entity, EntityId,
    EventEmitter, FileDropEvent, Flatten, FocusEvent, FontId, GlobalElementId, GlyphId, Hsla,
    ImageData, InputEvent, IsZero, KeyBinding, KeyContext, KeyDownEvent, KeystrokeEvent, LayoutId,
    Model, ModelContext, Modifiers, MonochromeSprite, MouseButton, MouseMoveEvent, MouseUpEvent,
    Path, Pixels, PlatformAtlas, PlatformDisplay, PlatformInputHandler, PlatformWindow, Point,
    PolychromeSprite, PromptLevel, Quad, Render, RenderGlyphParams, RenderImageParams,
    RenderSvgParams, ScaledPixels, Scene, SceneBuilder, Shadow, SharedString, Size, Style,
    SubscriberSet, Subscription, Surface, TaffyLayoutEngine, Task, Underline, UnderlineStyle, View,
    VisualContext, WeakView, WindowBounds, WindowOptions, SUBPIXEL_VARIANTS,
};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
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
    fmt::Debug,
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
use util::ResultExt;

const ACTIVE_DRAG_Z_INDEX: u32 = 1;

/// A global stacking order, which is created by stacking successive z-index values.
/// Each z-index will always be interpreted in the context of its parent z-index.
#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq, Clone, Default, Debug)]
pub struct StackingOrder(pub(crate) SmallVec<[u32; 16]>);

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
    pub fn bubble(self) -> bool {
        self == DispatchPhase::Bubble
    }

    pub fn capture(self) -> bool {
        self == DispatchPhase::Capture
    }
}

type AnyObserver = Box<dyn FnMut(&mut WindowContext) -> bool + 'static>;
type AnyMouseListener = Box<dyn FnMut(&dyn Any, DispatchPhase, &mut WindowContext) + 'static>;
type AnyFocusListener = Box<dyn Fn(&FocusEvent, &mut WindowContext) + 'static>;
type AnyWindowFocusListener = Box<dyn FnMut(&FocusEvent, &mut WindowContext) -> bool + 'static>;

slotmap::new_key_type! { pub struct FocusId; }

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
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle;
}

/// ManagedView is a view (like a Modal, Popover, Menu, etc.)
/// where the lifecycle of the view is handled by another view.
pub trait ManagedView: FocusableView + EventEmitter<DismissEvent> {}

impl<M: FocusableView + EventEmitter<DismissEvent>> ManagedView for M {}

pub struct DismissEvent;

// Holds the state for a specific window.
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
    pub(crate) focus_handles: Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
    pub(crate) focus_listeners: SubscriberSet<(), AnyWindowFocusListener>,
    pub(crate) blur_listeners: SubscriberSet<(), AnyObserver>,
    default_prevented: bool,
    mouse_position: Point<Pixels>,
    requested_cursor_style: Option<CursorStyle>,
    scale_factor: f32,
    bounds: WindowBounds,
    bounds_observers: SubscriberSet<(), AnyObserver>,
    active: bool,
    pub(crate) dirty: bool,
    activation_observers: SubscriberSet<(), AnyObserver>,
    pub(crate) last_blur: Option<Option<FocusId>>,
    pub(crate) focus: Option<FocusId>,
}

pub(crate) struct ElementStateBox {
    inner: Box<dyn Any>,
    #[cfg(debug_assertions)]
    type_name: &'static str,
}

// #[derive(Default)]
pub(crate) struct Frame {
    pub(crate) element_states: HashMap<GlobalElementId, ElementStateBox>,
    mouse_listeners: HashMap<TypeId, Vec<(StackingOrder, AnyMouseListener)>>,
    pub(crate) dispatch_tree: DispatchTree,
    pub(crate) focus_listeners: Vec<AnyFocusListener>,
    pub(crate) scene_builder: SceneBuilder,
    pub(crate) depth_map: Vec<(StackingOrder, Bounds<Pixels>)>,
    pub(crate) z_index_stack: StackingOrder,
    content_mask_stack: Vec<ContentMask<Pixels>>,
    element_offset_stack: Vec<Point<Pixels>>,
}

impl Frame {
    fn new(dispatch_tree: DispatchTree) -> Self {
        Frame {
            element_states: HashMap::default(),
            mouse_listeners: HashMap::default(),
            dispatch_tree,
            focus_listeners: Vec::new(),
            scene_builder: SceneBuilder::default(),
            z_index_stack: StackingOrder::default(),
            depth_map: Default::default(),
            content_mask_stack: Vec::new(),
            element_offset_stack: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.element_states.clear();
        self.mouse_listeners.values_mut().for_each(Vec::clear);
        self.focus_listeners.clear();
        self.dispatch_tree.clear();
        self.depth_map.clear();
    }
}

impl Window {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        cx: &mut AppContext,
    ) -> Self {
        let platform_window = cx.platform.open_window(
            handle,
            options,
            Box::new({
                let mut cx = cx.to_async();
                move || handle.update(&mut cx, |_, cx| cx.draw())
            }),
        );
        let display_id = platform_window.display().id();
        let sprite_atlas = platform_window.sprite_atlas();
        let mouse_position = platform_window.mouse_position();
        let content_size = platform_window.content_size();
        let scale_factor = platform_window.scale_factor();
        let bounds = platform_window.bounds();

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
            focus_handles: Arc::new(RwLock::new(SlotMap::with_key())),
            focus_listeners: SubscriberSet::new(),
            blur_listeners: SubscriberSet::new(),
            default_prevented: true,
            mouse_position,
            requested_cursor_style: None,
            scale_factor,
            bounds,
            bounds_observers: SubscriberSet::new(),
            active: false,
            dirty: false,
            activation_observers: SubscriberSet::new(),
            last_blur: None,
            focus: None,
        }
    }
}

/// Indicates which region of the window is visible. Content falling outside of this mask will not be
/// rendered. Currently, only rectangular content masks are supported, but we give the mask its own type
/// to leave room to support more complex shapes in the future.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ContentMask<P: Clone + Default + Debug> {
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
/// to an `AppContext`, so you can also pass a `WindowContext` to any method that takes
/// an `AppContext` and call any `AppContext` methods.
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
    pub fn notify(&mut self) {
        self.window.dirty = true;
    }

    /// Close this window.
    pub fn remove_window(&mut self) {
        self.window.removed = true;
    }

    /// Obtain a new `FocusHandle`, which allows you to track and manipulate the keyboard focus
    /// for elements rendered within this window.
    pub fn focus_handle(&mut self) -> FocusHandle {
        FocusHandle::new(&self.window.focus_handles)
    }

    /// Obtain the currently focused `FocusHandle`. If no elements are focused, returns `None`.
    pub fn focused(&self) -> Option<FocusHandle> {
        self.window
            .focus
            .and_then(|id| FocusHandle::for_id(id, &self.window.focus_handles))
    }

    /// Move focus to the element associated with the given `FocusHandle`.
    pub fn focus(&mut self, handle: &FocusHandle) {
        if self.window.focus == Some(handle.id) {
            return;
        }

        let focus_id = handle.id;

        if self.window.last_blur.is_none() {
            self.window.last_blur = Some(self.window.focus);
        }

        self.window.focus = Some(focus_id);
        self.window
            .rendered_frame
            .dispatch_tree
            .clear_pending_keystrokes();
        self.app.push_effect(Effect::FocusChanged {
            window_handle: self.window.handle,
            focused: Some(focus_id),
        });
        self.notify();
    }

    /// Remove focus from all elements within this context's window.
    pub fn blur(&mut self) {
        if self.window.last_blur.is_none() {
            self.window.last_blur = Some(self.window.focus);
        }

        self.window.focus = None;
        self.app.push_effect(Effect::FocusChanged {
            window_handle: self.window.handle,
            focused: None,
        });
        self.notify();
    }

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

    /// Schedules the given function to be run at the end of the current effect cycle, allowing entities
    /// that are currently on the stack to be returned to the app.
    pub fn defer(&mut self, f: impl FnOnce(&mut WindowContext) + 'static) {
        let handle = self.window.handle;
        self.app.defer(move |cx| {
            handle.update(cx, |_, cx| f(cx)).ok();
        });
    }

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

    /// Create an `AsyncWindowContext`, which has a static lifetime and can be held across
    /// await points in async code.
    pub fn to_async(&self) -> AsyncWindowContext {
        AsyncWindowContext::new(self.app.to_async(), self.window.handle)
    }

    /// Schedule the given closure to be run directly after the current frame is rendered.
    pub fn on_next_frame(&mut self, callback: impl FnOnce(&mut WindowContext) + 'static) {
        let handle = self.window.handle;
        let display_id = self.window.display_id;

        if !self.frame_consumers.contains_key(&display_id) {
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
            self.frame_consumers.insert(display_id, consumer_task);
        }

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

    /// Update the global of the given type. The given closure is given simultaneous mutable
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
        self.app.layout_id_buffer.extend(children.into_iter());
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
        self.notify();

        self.window
            .bounds_observers
            .clone()
            .retain(&(), |callback| callback(self));
    }

    pub fn window_bounds(&self) -> WindowBounds {
        self.window.bounds
    }

    pub fn viewport_size(&self) -> Size<Pixels> {
        self.window.viewport_size
    }

    pub fn is_window_active(&self) -> bool {
        self.window.active
    }

    pub fn zoom_window(&self) {
        self.window.platform_window.zoom();
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.window.platform_window.set_title(title);
    }

    pub fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.platform
            .displays()
            .into_iter()
            .find(|display| display.id() == self.window.display_id)
    }

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
            .to_pixels(text_style.font_size.into(), rem_size)
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
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_mouse_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&Event, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        let order = self.window.next_frame.z_index_stack.clone();
        self.window
            .next_frame
            .mouse_listeners
            .entry(TypeId::of::<Event>())
            .or_default()
            .push((
                order,
                Box::new(move |event: &dyn Any, phase, cx| {
                    handler(event.downcast_ref().unwrap(), phase, cx)
                }),
            ))
    }

    /// Register a key event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_key_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&Event, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        self.window
            .next_frame
            .dispatch_tree
            .on_key_event(Rc::new(move |event, phase, cx| {
                if let Some(event) = event.downcast_ref::<Event>() {
                    handler(event, phase, cx)
                }
            }));
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
        handler: impl Fn(&dyn Any, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        self.window.next_frame.dispatch_tree.on_action(
            action_type,
            Rc::new(move |action, phase, cx| handler(action, phase, cx)),
        );
    }

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

    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.window.requested_cursor_style = Some(style)
    }

    /// Called during painting to invoke the given closure in a new stacking context. The given
    /// z-index is interpreted relative to the previous call to `stack`.
    pub fn with_z_index<R>(&mut self, z_index: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.next_frame.z_index_stack.push(z_index);
        let result = f(self);
        self.window.next_frame.z_index_stack.pop();
        result
    }

    /// Called during painting to track which z-index is on top at each pixel position
    pub fn add_opaque_layer(&mut self, bounds: Bounds<Pixels>) {
        let stacking_order = self.window.next_frame.z_index_stack.clone();
        let depth_map = &mut self.window.next_frame.depth_map;
        match depth_map.binary_search_by(|(level, _)| stacking_order.cmp(&level)) {
            Ok(i) | Err(i) => depth_map.insert(i, (stacking_order, bounds)),
        }
    }

    /// Returns true if the top-most opaque layer painted over this point was part of the
    /// same layer as the given stacking order.
    pub fn was_top_layer(&self, point: &Point<Pixels>, level: &StackingOrder) -> bool {
        for (stack, bounds) in self.window.rendered_frame.depth_map.iter() {
            if bounds.contains(point) {
                return level.starts_with(stack) || stack.starts_with(level);
            }
        }

        false
    }

    pub fn was_top_layer_under_active_drag(
        &self,
        point: &Point<Pixels>,
        level: &StackingOrder,
    ) -> bool {
        for (stack, bounds) in self.window.rendered_frame.depth_map.iter() {
            if stack.starts_with(&[ACTIVE_DRAG_Z_INDEX]) {
                continue;
            }
            if bounds.contains(point) {
                return level.starts_with(stack) || stack.starts_with(level);
            }
        }

        false
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
        let window = &mut *self.window;
        for shadow in shadows {
            let mut shadow_bounds = bounds;
            shadow_bounds.origin += shadow.offset;
            shadow_bounds.dilate(shadow.spread_radius);
            window.next_frame.scene_builder.insert(
                &window.next_frame.z_index_stack,
                Shadow {
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
    pub fn paint_quad(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        background: impl Into<Hsla>,
        border_widths: Edges<Pixels>,
        border_color: impl Into<Hsla>,
    ) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();

        let window = &mut *self.window;
        window.next_frame.scene_builder.insert(
            &window.next_frame.z_index_stack,
            Quad {
                order: 0,
                bounds: bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                background: background.into(),
                border_color: border_color.into(),
                corner_radii: corner_radii.scale(scale_factor),
                border_widths: border_widths.scale(scale_factor),
            },
        );
    }

    /// Paint the given `Path` into the scene for the next frame at the current z-index.
    pub fn paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Hsla>) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        path.content_mask = content_mask;
        path.color = color.into();
        let window = &mut *self.window;
        window
            .next_frame
            .scene_builder
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
        let window = &mut *self.window;
        window.next_frame.scene_builder.insert(
            &window.next_frame.z_index_stack,
            Underline {
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
            let window = &mut *self.window;
            window.next_frame.scene_builder.insert(
                &window.next_frame.z_index_stack,
                MonochromeSprite {
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
            let window = &mut *self.window;

            window.next_frame.scene_builder.insert(
                &window.next_frame.z_index_stack,
                PolychromeSprite {
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

        let window = &mut *self.window;
        window.next_frame.scene_builder.insert(
            &window.next_frame.z_index_stack,
            MonochromeSprite {
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

        let window = &mut *self.window;
        window.next_frame.scene_builder.insert(
            &window.next_frame.z_index_stack,
            PolychromeSprite {
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
        let window = &mut *self.window;
        window.next_frame.scene_builder.insert(
            &window.next_frame.z_index_stack,
            Surface {
                order: 0,
                bounds,
                content_mask,
                image_buffer,
            },
        );
    }

    /// Draw pixels to the display for this window based on the contents of its scene.
    pub(crate) fn draw(&mut self) -> Scene {
        let window_was_focused = self
            .window
            .focus
            .and_then(|focus_id| {
                self.window
                    .rendered_frame
                    .dispatch_tree
                    .focusable_node_id(focus_id)
            })
            .is_some();
        self.text_system().start_frame();
        self.window.platform_window.clear_input_handler();
        self.window.layout_engine.as_mut().unwrap().clear();
        self.window.next_frame.clear();
        let root_view = self.window.root_view.take().unwrap();

        self.with_z_index(0, |cx| {
            cx.with_key_dispatch(Some(KeyContext::default()), None, |_, cx| {
                for (action_type, action_listeners) in &cx.app.global_action_listeners {
                    for action_listener in action_listeners.iter().cloned() {
                        cx.window.next_frame.dispatch_tree.on_action(
                            *action_type,
                            Rc::new(move |action, phase, cx| action_listener(action, phase, cx)),
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
        } else if let Some(active_tooltip) = self.app.active_tooltip.take() {
            self.with_z_index(1, |cx| {
                let available_space = size(AvailableSpace::MinContent, AvailableSpace::MinContent);
                active_tooltip
                    .view
                    .draw(active_tooltip.cursor_offset, available_space, cx);
            });
        }

        let window_is_focused = self
            .window
            .focus
            .and_then(|focus_id| {
                self.window
                    .next_frame
                    .dispatch_tree
                    .focusable_node_id(focus_id)
            })
            .is_some();
        if window_was_focused && !window_is_focused {
            self.window
                .blur_listeners
                .clone()
                .retain(&(), |listener| listener(self));
        }

        self.window
            .next_frame
            .dispatch_tree
            .preserve_pending_keystrokes(
                &mut self.window.rendered_frame.dispatch_tree,
                self.window.focus,
            );
        self.window.root_view = Some(root_view);

        let window = &mut self.window;
        mem::swap(&mut window.rendered_frame, &mut window.next_frame);

        let scene = self.window.rendered_frame.scene_builder.build();

        // Set the cursor only if we're the active window.
        let cursor_style = self
            .window
            .requested_cursor_style
            .take()
            .unwrap_or(CursorStyle::Arrow);
        if self.is_window_active() {
            self.platform.set_cursor_style(cursor_style);
        }

        self.window.dirty = false;

        scene
    }

    /// Dispatch a mouse or keyboard event on the window.
    pub fn dispatch_event(&mut self, event: InputEvent) -> bool {
        // Handlers may set this to false by calling `stop_propagation`.
        self.app.propagate_event = true;
        // Handlers may set this to true by calling `prevent_default`.
        self.window.default_prevented = false;

        let event = match event {
            // Track the mouse position with our own state, since accessing the platform
            // API for the mouse position can only occur on the main thread.
            InputEvent::MouseMove(mouse_move) => {
                self.window.mouse_position = mouse_move.position;
                InputEvent::MouseMove(mouse_move)
            }
            InputEvent::MouseDown(mouse_down) => {
                self.window.mouse_position = mouse_down.position;
                InputEvent::MouseDown(mouse_down)
            }
            InputEvent::MouseUp(mouse_up) => {
                self.window.mouse_position = mouse_up.position;
                InputEvent::MouseUp(mouse_up)
            }
            // Translate dragging and dropping of external files from the operating system
            // to internal drag and drop events.
            InputEvent::FileDrop(file_drop) => match file_drop {
                FileDropEvent::Entered { position, files } => {
                    self.window.mouse_position = position;
                    if self.active_drag.is_none() {
                        self.active_drag = Some(AnyDrag {
                            view: self.build_view(|_| files).into(),
                            cursor_offset: position,
                        });
                    }
                    InputEvent::MouseMove(MouseMoveEvent {
                        position,
                        pressed_button: Some(MouseButton::Left),
                        modifiers: Modifiers::default(),
                    })
                }
                FileDropEvent::Pending { position } => {
                    self.window.mouse_position = position;
                    InputEvent::MouseMove(MouseMoveEvent {
                        position,
                        pressed_button: Some(MouseButton::Left),
                        modifiers: Modifiers::default(),
                    })
                }
                FileDropEvent::Submit { position } => {
                    self.activate(true);
                    self.window.mouse_position = position;
                    InputEvent::MouseUp(MouseUpEvent {
                        button: MouseButton::Left,
                        position,
                        modifiers: Modifiers::default(),
                        click_count: 1,
                    })
                }
                FileDropEvent::Exited => InputEvent::MouseUp(MouseUpEvent {
                    button: MouseButton::Left,
                    position: Point::default(),
                    modifiers: Modifiers::default(),
                    click_count: 1,
                }),
            },
            _ => event,
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
            handlers.sort_by(|(a, _), (b, _)| a.cmp(b));

            // Capture phase, events bubble from back to front. Handlers for this phase are used for
            // special purposes, such as detecting events outside of a given Bounds.
            for (_, handler) in &mut handlers {
                handler(event, DispatchPhase::Capture, self);
                if !self.app.propagate_event {
                    break;
                }
            }

            // Bubble phase, where most normal handlers do their work.
            if self.app.propagate_event {
                for (_, handler) in handlers.iter_mut().rev() {
                    handler(event, DispatchPhase::Bubble, self);
                    if !self.app.propagate_event {
                        break;
                    }
                }
            }

            if self.app.propagate_event && event.downcast_ref::<MouseUpEvent>().is_some() {
                self.active_drag = None;
            }

            self.window
                .rendered_frame
                .mouse_listeners
                .insert(event.type_id(), handlers);
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

        // Capture phase
        let mut context_stack: SmallVec<[KeyContext; 16]> = SmallVec::new();
        self.propagate_event = true;

        for node_id in &dispatch_path {
            let node = self.window.rendered_frame.dispatch_tree.node(*node_id);

            if let Some(context) = node.context.clone() {
                context_stack.push(context);
            }

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

        for action in actions {
            self.dispatch_action_on_node(node_id, action.boxed_clone());
            if !self.propagate_event {
                self.dispatch_keystroke_observers(event, Some(action));
                return;
            }
        }
        self.dispatch_keystroke_observers(event, None);
    }

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

    pub fn activate_window(&self) {
        self.window.platform_window.activate();
    }

    pub fn minimize_window(&self) {
        self.window.platform_window.minimize();
    }

    pub fn toggle_full_screen(&self) {
        self.window.platform_window.toggle_full_screen();
    }

    pub fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        self.window.platform_window.prompt(level, msg, answers)
    }

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

    pub fn bindings_for_action(&self, action: &dyn Action) -> Vec<KeyBinding> {
        self.window
            .rendered_frame
            .dispatch_tree
            .bindings_for_action(
                action,
                &self.window.rendered_frame.dispatch_tree.context_stack,
            )
    }

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

    //========== ELEMENT RELATED FUNCTIONS ===========
    pub fn with_key_dispatch<R>(
        &mut self,
        context: Option<KeyContext>,
        focus_handle: Option<FocusHandle>,
        f: impl FnOnce(Option<FocusHandle>, &mut Self) -> R,
    ) -> R {
        let window = &mut self.window;
        window.next_frame.dispatch_tree.push_node(context.clone());
        if let Some(focus_handle) = focus_handle.as_ref() {
            window
                .next_frame
                .dispatch_tree
                .make_focusable(focus_handle.id);
        }
        let result = f(focus_handle, self);

        self.window.next_frame.dispatch_tree.pop_node();

        result
    }

    /// Register a focus listener for the next frame only. It will be cleared
    /// on the next frame render. You should use this method only from within elements,
    /// and we may want to enforce that better via a different context type.
    // todo!() Move this to `FrameContext` to emphasize its individuality?
    pub fn on_focus_changed(
        &mut self,
        listener: impl Fn(&FocusEvent, &mut WindowContext) + 'static,
    ) {
        self.window
            .next_frame
            .focus_listeners
            .push(Box::new(move |event, cx| {
                listener(event, cx);
            }));
    }

    /// Set an input handler, such as [ElementInputHandler], which interfaces with the
    /// platform to receive textual input with proper integration with concerns such
    /// as IME interactions.
    pub fn handle_input(
        &mut self,
        focus_handle: &FocusHandle,
        input_handler: impl PlatformInputHandler,
    ) {
        if focus_handle.is_focused(self) {
            self.window
                .platform_window
                .set_input_handler(Box::new(input_handler));
        }
    }

    pub fn on_window_should_close(&mut self, f: impl Fn(&mut WindowContext) -> bool + 'static) {
        let mut this = self.to_async();
        self.window
            .platform_window
            .on_should_close(Box::new(move || this.update(|_, cx| f(cx)).unwrap_or(true)))
    }
}

impl Context for WindowContext<'_> {
    type Result<T> = T;

    fn build_model<T>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T>
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
        read(&*entity, &*self.app)
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
    fn build_view<V>(
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

    /// Update the given view. Prefer calling `View::update` instead, which calls this method.
    fn update_view<T: 'static, R>(
        &mut self,
        view: &View<T>,
        update: impl FnOnce(&mut T, &mut ViewContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let mut lease = self.app.entities.lease(&view.model);
        let mut cx = ViewContext::new(&mut *self.app, &mut *self.window, &view);
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
        let slot = self.app.entities.reserve();
        let view = View {
            model: slot.clone(),
        };
        let mut cx = ViewContext::new(&mut *self.app, &mut *self.window, &view);
        let entity = build_view(&mut cx);
        self.entities.insert(slot, entity);
        self.window.root_view = Some(view.clone().into());
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
        &self.app
    }
}

impl<'a> std::ops::DerefMut for WindowContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<'a> Borrow<AppContext> for WindowContext<'a> {
    fn borrow(&self) -> &AppContext {
        &self.app
    }
}

impl<'a> BorrowMut<AppContext> for WindowContext<'a> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        &mut self.app
    }
}

pub trait BorrowWindow: BorrowMut<Window> + BorrowMut<AppContext> {
    fn app_mut(&mut self) -> &mut AppContext {
        self.borrow_mut()
    }

    fn app(&self) -> &AppContext {
        self.borrow()
    }

    fn window(&self) -> &Window {
        self.borrow()
    }

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
            window.element_id_stack.push(id.into());
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
        self.window_mut().next_frame.content_mask_stack.push(mask);
        let result = f(self);
        self.window_mut().next_frame.content_mask_stack.pop();
        result
    }

    /// Update the global element offset relative to the current offset. This is used to implement
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

    /// Update the global element offset based on the given offset. This is used to implement
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

    /// Update or initialize state for an element with the given id that lives across multiple
    /// frames. If an element with this id existed in the rendered frame, its state will be passed
    /// to the given closure. The state returned by the closure will be stored so it can be referenced
    /// when drawing the next frame.
    fn with_element_state<S, R>(
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
                // Requested: () <- AnyElemet
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

                        #[cfg(debug_assertions)]
                        type_name
                    });
                result
            } else {
                let (result, state) = f(None, cx);
                cx.window_mut()
                    .next_frame
                    .element_states
                    .insert(global_id,
                        ElementStateBox {
                            inner: Box::new(Some(state)),

                            #[cfg(debug_assertions)]
                            type_name: std::any::type_name::<S>()
                        }

                    );
                result
            }
        })
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
        &self.window
    }
}

impl BorrowMut<Window> for WindowContext<'_> {
    fn borrow_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}

impl<T> BorrowWindow for T where T: BorrowMut<AppContext> + BorrowMut<Window> {}

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

    pub fn entity_id(&self) -> EntityId {
        self.view.entity_id()
    }

    pub fn view(&self) -> &View<V> {
        self.view
    }

    pub fn model(&self) -> &Model<V> {
        &self.view.model
    }

    /// Access the underlying window context.
    pub fn window_context(&mut self) -> &mut WindowContext<'a> {
        &mut self.window_cx
    }

    pub fn with_z_index<R>(&mut self, z_index: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.next_frame.z_index_stack.push(z_index);
        let result = f(self);
        self.window.next_frame.z_index_stack.pop();
        result
    }

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

    pub fn on_release(
        &mut self,
        on_release: impl FnOnce(&mut V, &mut WindowContext) + 'static,
    ) -> Subscription {
        let window_handle = self.window.handle;
        let (subscription, activate) = self.app.release_listeners.insert(
            self.view.model.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                let _ = window_handle.update(cx, |_, cx| on_release(this, cx));
            }),
        );
        activate();
        subscription
    }

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

    pub fn notify(&mut self) {
        self.window_cx.notify();
        self.window_cx.app.push_effect(Effect::Notify {
            emitter: self.view.model.entity_id,
        });
    }

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
    /// Unlike [on_focus_changed], returns a subscription and persists until the subscription
    /// is dropped.
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
                    if event.focused.as_ref().map(|focused| focused.id) == Some(focus_id) {
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
    /// Unlike [on_focus_changed], returns a subscription and persists until the subscription
    /// is dropped.
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
                    if event
                        .focused
                        .as_ref()
                        .map_or(false, |focused| focus_id.contains(focused.id, cx))
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
    /// Unlike [on_focus_changed], returns a subscription and persists until the subscription
    /// is dropped.
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
                    if event.blurred.as_ref().map(|blurred| blurred.id) == Some(focus_id) {
                        listener(view, cx)
                    }
                })
                .is_ok()
            }),
        );
        self.app.defer(move |_| activate());
        subscription
    }

    /// Register a listener to be called when the window loses focus.
    /// Unlike [on_focus_changed], returns a subscription and persists until the subscription
    /// is dropped.
    pub fn on_blur_window(
        &mut self,
        mut listener: impl FnMut(&mut V, &mut ViewContext<V>) + 'static,
    ) -> Subscription {
        let view = self.view.downgrade();
        let (subscription, activate) = self.window.blur_listeners.insert(
            (),
            Box::new(move |cx| view.update(cx, |view, cx| listener(view, cx)).is_ok()),
        );
        activate();
        subscription
    }

    /// Register a listener to be called when the given focus handle or one of its descendants loses focus.
    /// Unlike [on_focus_changed], returns a subscription and persists until the subscription
    /// is dropped.
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
                    if event
                        .blurred
                        .as_ref()
                        .map_or(false, |blurred| focus_id.contains(blurred.id, cx))
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

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

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

    pub fn on_mouse_event<Event: 'static>(
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

    pub fn on_key_event<Event: 'static>(
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

    pub fn on_action(
        &mut self,
        action_type: TypeId,
        handler: impl Fn(&mut V, &dyn Any, DispatchPhase, &mut ViewContext<V>) + 'static,
    ) {
        let handle = self.view().clone();
        self.window_cx
            .on_action(action_type, move |action, phase, cx| {
                handle.update(cx, |view, cx| {
                    handler(view, action, phase, cx);
                })
            });
    }

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

    pub fn focus_self(&mut self)
    where
        V: FocusableView,
    {
        self.defer(|view, cx| view.focus_handle(cx).focus(cx))
    }

    pub fn dismiss_self(&mut self)
    where
        V: ManagedView,
    {
        self.defer(|_, cx| cx.emit(DismissEvent))
    }

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

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.window_cx.build_model(build_model)
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
    fn build_view<W: Render + 'static>(
        &mut self,
        build_view_state: impl FnOnce(&mut ViewContext<'_, W>) -> W,
    ) -> Self::Result<View<W>> {
        self.window_cx.build_view(build_view_state)
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
slotmap::new_key_type! { pub struct WindowId; }

impl WindowId {
    pub fn as_u64(&self) -> u64 {
        self.0.as_ffi()
    }
}

#[derive(Deref, DerefMut)]
pub struct WindowHandle<V> {
    #[deref]
    #[deref_mut]
    pub(crate) any_handle: AnyWindowHandle,
    state_type: PhantomData<V>,
}

impl<V: 'static + Render> WindowHandle<V> {
    pub fn new(id: WindowId) -> Self {
        WindowHandle {
            any_handle: AnyWindowHandle {
                id,
                state_type: TypeId::of::<V>(),
            },
            state_type: PhantomData,
        }
    }

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

    pub fn read_with<C, R>(&self, cx: &C, read_with: impl FnOnce(&V, &AppContext) -> R) -> Result<R>
    where
        C: Context,
    {
        cx.read_window(self, |root_view, cx| read_with(root_view.read(cx), cx))
    }

    pub fn root_view<C>(&self, cx: &C) -> Result<View<V>>
    where
        C: Context,
    {
        cx.read_window(self, |root_view, _cx| root_view.clone())
    }

    pub fn is_active(&self, cx: &AppContext) -> Option<bool> {
        cx.windows
            .get(self.id)
            .and_then(|window| window.as_ref().map(|window| window.active))
    }
}

impl<V> Copy for WindowHandle<V> {}

impl<V> Clone for WindowHandle<V> {
    fn clone(&self) -> Self {
        WindowHandle {
            any_handle: self.any_handle,
            state_type: PhantomData,
        }
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

impl<V: 'static> Into<AnyWindowHandle> for WindowHandle<V> {
    fn into(self) -> AnyWindowHandle {
        self.any_handle
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnyWindowHandle {
    pub(crate) id: WindowId,
    state_type: TypeId,
}

impl AnyWindowHandle {
    pub fn window_id(&self) -> WindowId {
        self.id
    }

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

#[cfg(any(test, feature = "test-support"))]
impl From<SmallVec<[u32; 16]>> for StackingOrder {
    fn from(small_vec: SmallVec<[u32; 16]>) -> Self {
        StackingOrder(small_vec)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ElementId {
    View(EntityId),
    Integer(usize),
    Name(SharedString),
    FocusHandle(FocusId),
    NamedInteger(SharedString, usize),
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
