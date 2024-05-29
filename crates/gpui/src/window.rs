use crate::{
    hash, point, prelude::*, px, size, transparent_black, Action, AnyDrag, AnyElement, AnyTooltip,
    AnyView, AppContext, Arena, Asset, AsyncWindowContext, AvailableSpace, Bounds, BoxShadow,
    Context, Corners, CursorStyle, DevicePixels, DispatchActionListener, DispatchNodeId,
    DispatchTree, DisplayId, Edges, Effect, Entity, EntityId, EventEmitter, FileDropEvent, Flatten,
    FontId, Global, GlobalElementId, GlyphId, Hsla, ImageData, InputHandler, IsZero, KeyBinding,
    KeyContext, KeyDownEvent, KeyEvent, KeyMatch, KeymatchResult, Keystroke, KeystrokeEvent,
    LayoutId, LineLayoutIndex, Model, ModelContext, Modifiers, ModifiersChangedEvent,
    MonochromeSprite, MouseButton, MouseEvent, MouseMoveEvent, MouseUpEvent, Path, Pixels,
    PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point,
    PolychromeSprite, PromptLevel, Quad, Render, RenderGlyphParams, RenderImageParams,
    RenderSvgParams, ScaledPixels, Scene, Shadow, SharedString, Size, StrikethroughStyle, Style,
    SubscriberSet, Subscription, TaffyLayoutEngine, Task, TextStyle, TextStyleRefinement,
    TransformationMatrix, Underline, UnderlineStyle, View, VisualContext, WeakView,
    WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowOptions, WindowParams,
    WindowTextSystem, SUBPIXEL_VARIANTS,
};
use anyhow::{anyhow, Context as _, Result};
use collections::{FxHashMap, FxHashSet};
use derive_more::{Deref, DerefMut};
use futures::channel::oneshot;
use futures::{future::Shared, FutureExt};
#[cfg(target_os = "macos")]
use media::core_video::CVImageBuffer;
use parking_lot::RwLock;
use refineable::Refineable;
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut, Cow},
    cell::{Cell, RefCell},
    cmp,
    fmt::{Debug, Display},
    future::Future,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    ops::Range,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc, Weak,
    },
    time::{Duration, Instant},
};
use util::post_inc;
use util::{measure, ResultExt};
use uuid::Uuid;

mod prompts;

pub use prompts::*;

pub(crate) const DEFAULT_WINDOW_SIZE: Size<DevicePixels> =
    size(DevicePixels(1024), DevicePixels(700));

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
    /// 8MB wasn't quite enough...
    pub(crate) static ELEMENT_ARENA: RefCell<Arena> = RefCell::new(Arena::new(32 * 1024 * 1024));
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

    /// Dispatch an action on the element that rendered this focus handle
    pub fn dispatch_action(&self, action: &dyn Action, cx: &mut WindowContext) {
        if let Some(node_id) = cx
            .window
            .rendered_frame
            .dispatch_tree
            .focusable_node_id(self.id)
        {
            cx.dispatch_action_on_node(node_id, action)
        }
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

pub(crate) type AnyMouseListener =
    Box<dyn FnMut(&dyn Any, DispatchPhase, &mut WindowContext) + 'static>;

#[derive(Clone)]
pub(crate) struct CursorStyleRequest {
    pub(crate) hitbox_id: HitboxId,
    pub(crate) style: CursorStyle,
}

/// An identifier for a [Hitbox].
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct HitboxId(usize);

impl HitboxId {
    /// Checks if the hitbox with this id is currently hovered.
    pub fn is_hovered(&self, cx: &WindowContext) -> bool {
        cx.window.mouse_hit_test.0.contains(self)
    }
}

/// A rectangular region that potentially blocks hitboxes inserted prior.
/// See [WindowContext::insert_hitbox] for more details.
#[derive(Clone, Debug, Deref)]
pub struct Hitbox {
    /// A unique identifier for the hitbox.
    pub id: HitboxId,
    /// The bounds of the hitbox.
    #[deref]
    pub bounds: Bounds<Pixels>,
    /// The content mask when the hitbox was inserted.
    pub content_mask: ContentMask<Pixels>,
    /// Whether the hitbox occludes other hitboxes inserted prior.
    pub opaque: bool,
}

impl Hitbox {
    /// Checks if the hitbox is currently hovered.
    pub fn is_hovered(&self, cx: &WindowContext) -> bool {
        self.id.is_hovered(cx)
    }
}

#[derive(Default, Eq, PartialEq)]
pub(crate) struct HitTest(SmallVec<[HitboxId; 8]>);

/// An identifier for a tooltip.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TooltipId(usize);

impl TooltipId {
    /// Checks if the tooltip is currently hovered.
    pub fn is_hovered(&self, cx: &WindowContext) -> bool {
        cx.window
            .tooltip_bounds
            .as_ref()
            .map_or(false, |tooltip_bounds| {
                tooltip_bounds.id == *self && tooltip_bounds.bounds.contains(&cx.mouse_position())
            })
    }
}

pub(crate) struct TooltipBounds {
    id: TooltipId,
    bounds: Bounds<Pixels>,
}

#[derive(Clone)]
pub(crate) struct TooltipRequest {
    id: TooltipId,
    tooltip: AnyTooltip,
}

pub(crate) struct DeferredDraw {
    priority: usize,
    parent_node: DispatchNodeId,
    element_id_stack: SmallVec<[ElementId; 32]>,
    text_style_stack: Vec<TextStyleRefinement>,
    element: Option<AnyElement>,
    absolute_offset: Point<Pixels>,
    prepaint_range: Range<PrepaintStateIndex>,
    paint_range: Range<PaintIndex>,
}

pub(crate) struct Frame {
    pub(crate) focus: Option<FocusId>,
    pub(crate) window_active: bool,
    pub(crate) element_states: FxHashMap<(GlobalElementId, TypeId), ElementStateBox>,
    accessed_element_states: Vec<(GlobalElementId, TypeId)>,
    pub(crate) mouse_listeners: Vec<Option<AnyMouseListener>>,
    pub(crate) dispatch_tree: DispatchTree,
    pub(crate) scene: Scene,
    pub(crate) hitboxes: Vec<Hitbox>,
    pub(crate) deferred_draws: Vec<DeferredDraw>,
    pub(crate) input_handlers: Vec<Option<PlatformInputHandler>>,
    pub(crate) tooltip_requests: Vec<Option<TooltipRequest>>,
    pub(crate) cursor_styles: Vec<CursorStyleRequest>,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) debug_bounds: FxHashMap<String, Bounds<Pixels>>,
}

#[derive(Clone, Default)]
pub(crate) struct PrepaintStateIndex {
    hitboxes_index: usize,
    tooltips_index: usize,
    deferred_draws_index: usize,
    dispatch_tree_index: usize,
    accessed_element_states_index: usize,
    line_layout_index: LineLayoutIndex,
}

#[derive(Clone, Default)]
pub(crate) struct PaintIndex {
    scene_index: usize,
    mouse_listeners_index: usize,
    input_handlers_index: usize,
    cursor_styles_index: usize,
    accessed_element_states_index: usize,
    line_layout_index: LineLayoutIndex,
}

impl Frame {
    pub(crate) fn new(dispatch_tree: DispatchTree) -> Self {
        Frame {
            focus: None,
            window_active: false,
            element_states: FxHashMap::default(),
            accessed_element_states: Vec::new(),
            mouse_listeners: Vec::new(),
            dispatch_tree,
            scene: Scene::default(),
            hitboxes: Vec::new(),
            deferred_draws: Vec::new(),
            input_handlers: Vec::new(),
            tooltip_requests: Vec::new(),
            cursor_styles: Vec::new(),

            #[cfg(any(test, feature = "test-support"))]
            debug_bounds: FxHashMap::default(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.element_states.clear();
        self.accessed_element_states.clear();
        self.mouse_listeners.clear();
        self.dispatch_tree.clear();
        self.scene.clear();
        self.input_handlers.clear();
        self.tooltip_requests.clear();
        self.cursor_styles.clear();
        self.hitboxes.clear();
        self.deferred_draws.clear();
    }

    pub(crate) fn hit_test(&self, position: Point<Pixels>) -> HitTest {
        let mut hit_test = HitTest::default();
        for hitbox in self.hitboxes.iter().rev() {
            let bounds = hitbox.bounds.intersect(&hitbox.content_mask.bounds);
            if bounds.contains(&position) {
                hit_test.0.push(hitbox.id);
                if hitbox.opaque {
                    break;
                }
            }
        }
        hit_test
    }

    pub(crate) fn focus_path(&self) -> SmallVec<[FocusId; 8]> {
        self.focus
            .map(|focus_id| self.dispatch_tree.focus_path(focus_id))
            .unwrap_or_default()
    }

    pub(crate) fn finish(&mut self, prev_frame: &mut Self) {
        for element_state_key in &self.accessed_element_states {
            if let Some((element_state_key, element_state)) =
                prev_frame.element_states.remove_entry(element_state_key)
            {
                self.element_states.insert(element_state_key, element_state);
            }
        }

        self.scene.finish();
    }
}

// Holds the state for a specific window.
#[doc(hidden)]
pub struct Window {
    pub(crate) handle: AnyWindowHandle,
    pub(crate) removed: bool,
    pub(crate) platform_window: Box<dyn PlatformWindow>,
    display_id: DisplayId,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    text_system: Arc<WindowTextSystem>,
    rem_size: Pixels,
    /// The stack of override values for the window's rem size.
    ///
    /// This is used by `with_rem_size` to allow rendering an element tree with
    /// a given rem size.
    rem_size_override_stack: SmallVec<[Pixels; 8]>,
    pub(crate) viewport_size: Size<Pixels>,
    layout_engine: Option<TaffyLayoutEngine>,
    pub(crate) root_view: Option<AnyView>,
    pub(crate) element_id_stack: SmallVec<[ElementId; 32]>,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) element_offset_stack: Vec<Point<Pixels>>,
    pub(crate) content_mask_stack: Vec<ContentMask<Pixels>>,
    pub(crate) requested_autoscroll: Option<Bounds<Pixels>>,
    pub(crate) rendered_frame: Frame,
    pub(crate) next_frame: Frame,
    pub(crate) next_hitbox_id: HitboxId,
    pub(crate) next_tooltip_id: TooltipId,
    pub(crate) tooltip_bounds: Option<TooltipBounds>,
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
    Prepaint,
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
    const DEFAULT_WINDOW_OFFSET: Point<DevicePixels> = point(DevicePixels(0), DevicePixels(35));

    cx.active_window()
        .and_then(|w| w.update(cx, |_, cx| cx.bounds()).ok())
        .map(|bounds| bounds.map_origin(|origin| origin + DEFAULT_WINDOW_OFFSET))
        .unwrap_or_else(|| {
            let display = display_id
                .map(|id| cx.find_display(id))
                .unwrap_or_else(|| cx.primary_display());

            display
                .map(|display| display.default_bounds())
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
            window_bounds,
            titlebar,
            focus,
            show,
            kind,
            is_movable,
            display_id,
            window_background,
            app_id,
        } = options;

        let bounds = window_bounds
            .map(|bounds| bounds.get_bounds())
            .unwrap_or_else(|| default_bounds(display_id, cx));
        let mut platform_window = cx.platform.open_window(
            handle,
            WindowParams {
                bounds,
                titlebar,
                kind,
                is_movable,
                focus,
                show,
                display_id,
                window_background,
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

        if let Some(ref window_open_state) = window_bounds {
            match window_open_state {
                WindowBounds::Fullscreen(_) => platform_window.toggle_fullscreen(),
                WindowBounds::Maximized(_) => platform_window.zoom(),
                WindowBounds::Windowed(_) => {}
            }
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

                handle
                    .update(&mut cx, |_, cx| {
                        cx.complete_frame();
                    })
                    .log_err();
            }
        }));
        platform_window.on_resize(Box::new({
            let mut cx = cx.to_async();
            move |_, _| {
                handle
                    .update(&mut cx, |_, cx| cx.bounds_changed())
                    .log_err();
            }
        }));
        platform_window.on_moved(Box::new({
            let mut cx = cx.to_async();
            move || {
                handle
                    .update(&mut cx, |_, cx| cx.bounds_changed())
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

        if let Some(app_id) = app_id {
            platform_window.set_app_id(&app_id);
        }

        Window {
            handle,
            removed: false,
            platform_window,
            display_id,
            sprite_atlas,
            text_system,
            rem_size: px(16.),
            rem_size_override_stack: SmallVec::new(),
            viewport_size: content_size,
            layout_engine: Some(TaffyLayoutEngine::new()),
            root_view: None,
            element_id_stack: SmallVec::default(),
            text_style_stack: Vec::new(),
            element_offset_stack: Vec::new(),
            content_mask_stack: Vec::new(),
            requested_autoscroll: None,
            rendered_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
            next_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
            next_frame_callbacks,
            next_hitbox_id: HitboxId::default(),
            next_tooltip_id: TooltipId::default(),
            tooltip_bounds: None,
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

    /// Return the `WindowBounds` to indicate that how a window should be opened
    /// after it has been closed
    pub fn window_bounds(&self) -> WindowBounds {
        self.window.platform_window.window_bounds()
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

    fn bounds_changed(&mut self) {
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
    pub fn bounds(&self) -> Bounds<DevicePixels> {
        self.window.platform_window.bounds()
    }

    /// Returns whether or not the window is currently fullscreen
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

    /// Opens the native title bar context menu, useful when implementing client side decorations (Wayland and X11)
    pub fn show_window_menu(&self, position: Point<Pixels>) {
        self.window.platform_window.show_window_menu(position)
    }

    /// Tells the compositor to take control of window movement (Wayland and X11)
    ///
    /// Events may not be received during a move operation.
    pub fn start_system_move(&self) {
        self.window.platform_window.start_system_move()
    }

    /// Returns whether the title bar window controls need to be rendered by the application (Wayland and X11)
    pub fn should_render_window_controls(&self) -> bool {
        self.window.platform_window.should_render_window_controls()
    }

    /// Updates the window's title at the platform level.
    pub fn set_window_title(&mut self, title: &str) {
        self.window.platform_window.set_title(title);
    }

    /// Sets the application identifier.
    pub fn set_app_id(&mut self, app_id: &str) {
        self.window.platform_window.set_app_id(app_id);
    }

    /// Sets the window background appearance.
    pub fn set_background_appearance(&mut self, background_appearance: WindowBackgroundAppearance) {
        self.window
            .platform_window
            .set_background_appearance(background_appearance);
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
        self.window
            .rem_size_override_stack
            .last()
            .copied()
            .unwrap_or(self.window.rem_size)
    }

    /// Sets the size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    pub fn set_rem_size(&mut self, rem_size: impl Into<Pixels>) {
        self.window.rem_size = rem_size.into();
    }

    /// Executes the provided function with the specified rem size.
    ///
    /// This method must only be called as part of element drawing.
    pub fn with_rem_size<F, R>(&mut self, rem_size: Option<impl Into<Pixels>>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        debug_assert!(
            matches!(
                self.window.draw_phase,
                DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during request_layout, prepaint, or paint"
        );

        if let Some(rem_size) = rem_size {
            self.window.rem_size_override_stack.push(rem_size.into());
            let result = f(self);
            self.window.rem_size_override_stack.pop();
            result
        } else {
            f(self)
        }
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

    fn complete_frame(&self) {
        self.window.platform_window.completed_frame();
    }

    /// Produces a new frame and assigns it to `rendered_frame`. To actually show
    /// the contents of the new [Scene], use [present].
    #[profiling::function]
    pub fn draw(&mut self) {
        self.window.dirty.set(false);
        self.window.requested_autoscroll = None;

        // Restore the previously-used input handler.
        if let Some(input_handler) = self.window.platform_window.take_input_handler() {
            self.window
                .rendered_frame
                .input_handlers
                .push(Some(input_handler));
        }

        self.draw_roots();
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

    fn draw_roots(&mut self) {
        self.window.draw_phase = DrawPhase::Prepaint;
        self.window.tooltip_bounds.take();

        // Layout all root elements.
        let mut root_element = self.window.root_view.as_ref().unwrap().clone().into_any();
        root_element.prepaint_as_root(Point::default(), self.window.viewport_size.into(), self);

        let mut sorted_deferred_draws =
            (0..self.window.next_frame.deferred_draws.len()).collect::<SmallVec<[_; 8]>>();
        sorted_deferred_draws.sort_by_key(|ix| self.window.next_frame.deferred_draws[*ix].priority);
        self.prepaint_deferred_draws(&sorted_deferred_draws);

        let mut prompt_element = None;
        let mut active_drag_element = None;
        let mut tooltip_element = None;
        if let Some(prompt) = self.window.prompt.take() {
            let mut element = prompt.view.any_view().into_any();
            element.prepaint_as_root(Point::default(), self.window.viewport_size.into(), self);
            prompt_element = Some(element);
            self.window.prompt = Some(prompt);
        } else if let Some(active_drag) = self.app.active_drag.take() {
            let mut element = active_drag.view.clone().into_any();
            let offset = self.mouse_position() - active_drag.cursor_offset;
            element.prepaint_as_root(offset, AvailableSpace::min_size(), self);
            active_drag_element = Some(element);
            self.app.active_drag = Some(active_drag);
        } else {
            tooltip_element = self.prepaint_tooltip();
        }

        self.window.mouse_hit_test = self.window.next_frame.hit_test(self.window.mouse_position);

        // Now actually paint the elements.
        self.window.draw_phase = DrawPhase::Paint;
        root_element.paint(self);

        self.paint_deferred_draws(&sorted_deferred_draws);

        if let Some(mut prompt_element) = prompt_element {
            prompt_element.paint(self)
        } else if let Some(mut drag_element) = active_drag_element {
            drag_element.paint(self);
        } else if let Some(mut tooltip_element) = tooltip_element {
            tooltip_element.paint(self);
        }
    }

    fn prepaint_tooltip(&mut self) -> Option<AnyElement> {
        let tooltip_request = self.window.next_frame.tooltip_requests.last().cloned()?;
        let tooltip_request = tooltip_request.unwrap();
        let mut element = tooltip_request.tooltip.view.clone().into_any();
        let mouse_position = tooltip_request.tooltip.mouse_position;
        let tooltip_size = element.layout_as_root(AvailableSpace::min_size(), self);

        let mut tooltip_bounds = Bounds::new(mouse_position + point(px(1.), px(1.)), tooltip_size);
        let window_bounds = Bounds {
            origin: Point::default(),
            size: self.viewport_size(),
        };

        if tooltip_bounds.right() > window_bounds.right() {
            let new_x = mouse_position.x - tooltip_bounds.size.width - px(1.);
            if new_x >= Pixels::ZERO {
                tooltip_bounds.origin.x = new_x;
            } else {
                tooltip_bounds.origin.x = cmp::max(
                    Pixels::ZERO,
                    tooltip_bounds.origin.x - tooltip_bounds.right() - window_bounds.right(),
                );
            }
        }

        if tooltip_bounds.bottom() > window_bounds.bottom() {
            let new_y = mouse_position.y - tooltip_bounds.size.height - px(1.);
            if new_y >= Pixels::ZERO {
                tooltip_bounds.origin.y = new_y;
            } else {
                tooltip_bounds.origin.y = cmp::max(
                    Pixels::ZERO,
                    tooltip_bounds.origin.y - tooltip_bounds.bottom() - window_bounds.bottom(),
                );
            }
        }

        self.with_absolute_element_offset(tooltip_bounds.origin, |cx| element.prepaint(cx));

        self.window.tooltip_bounds = Some(TooltipBounds {
            id: tooltip_request.id,
            bounds: tooltip_bounds,
        });
        Some(element)
    }

    fn prepaint_deferred_draws(&mut self, deferred_draw_indices: &[usize]) {
        assert_eq!(self.window.element_id_stack.len(), 0);

        let mut deferred_draws = mem::take(&mut self.window.next_frame.deferred_draws);
        for deferred_draw_ix in deferred_draw_indices {
            let deferred_draw = &mut deferred_draws[*deferred_draw_ix];
            self.window
                .element_id_stack
                .clone_from(&deferred_draw.element_id_stack);
            self.window
                .text_style_stack
                .clone_from(&deferred_draw.text_style_stack);
            self.window
                .next_frame
                .dispatch_tree
                .set_active_node(deferred_draw.parent_node);

            let prepaint_start = self.prepaint_index();
            if let Some(element) = deferred_draw.element.as_mut() {
                self.with_absolute_element_offset(deferred_draw.absolute_offset, |cx| {
                    element.prepaint(cx)
                });
            } else {
                self.reuse_prepaint(deferred_draw.prepaint_range.clone());
            }
            let prepaint_end = self.prepaint_index();
            deferred_draw.prepaint_range = prepaint_start..prepaint_end;
        }
        assert_eq!(
            self.window.next_frame.deferred_draws.len(),
            0,
            "cannot call defer_draw during deferred drawing"
        );
        self.window.next_frame.deferred_draws = deferred_draws;
        self.window.element_id_stack.clear();
        self.window.text_style_stack.clear();
    }

    fn paint_deferred_draws(&mut self, deferred_draw_indices: &[usize]) {
        assert_eq!(self.window.element_id_stack.len(), 0);

        let mut deferred_draws = mem::take(&mut self.window.next_frame.deferred_draws);
        for deferred_draw_ix in deferred_draw_indices {
            let mut deferred_draw = &mut deferred_draws[*deferred_draw_ix];
            self.window
                .element_id_stack
                .clone_from(&deferred_draw.element_id_stack);
            self.window
                .next_frame
                .dispatch_tree
                .set_active_node(deferred_draw.parent_node);

            let paint_start = self.paint_index();
            if let Some(element) = deferred_draw.element.as_mut() {
                element.paint(self);
            } else {
                self.reuse_paint(deferred_draw.paint_range.clone());
            }
            let paint_end = self.paint_index();
            deferred_draw.paint_range = paint_start..paint_end;
        }
        self.window.next_frame.deferred_draws = deferred_draws;
        self.window.element_id_stack.clear();
    }

    pub(crate) fn prepaint_index(&self) -> PrepaintStateIndex {
        PrepaintStateIndex {
            hitboxes_index: self.window.next_frame.hitboxes.len(),
            tooltips_index: self.window.next_frame.tooltip_requests.len(),
            deferred_draws_index: self.window.next_frame.deferred_draws.len(),
            dispatch_tree_index: self.window.next_frame.dispatch_tree.len(),
            accessed_element_states_index: self.window.next_frame.accessed_element_states.len(),
            line_layout_index: self.window.text_system.layout_index(),
        }
    }

    pub(crate) fn reuse_prepaint(&mut self, range: Range<PrepaintStateIndex>) {
        let window = &mut self.window;
        window.next_frame.hitboxes.extend(
            window.rendered_frame.hitboxes[range.start.hitboxes_index..range.end.hitboxes_index]
                .iter()
                .cloned(),
        );
        window.next_frame.tooltip_requests.extend(
            window.rendered_frame.tooltip_requests
                [range.start.tooltips_index..range.end.tooltips_index]
                .iter_mut()
                .map(|request| request.take()),
        );
        window.next_frame.accessed_element_states.extend(
            window.rendered_frame.accessed_element_states[range.start.accessed_element_states_index
                ..range.end.accessed_element_states_index]
                .iter()
                .map(|(id, type_id)| (GlobalElementId(id.0.clone()), *type_id)),
        );
        window
            .text_system
            .reuse_layouts(range.start.line_layout_index..range.end.line_layout_index);

        let reused_subtree = window.next_frame.dispatch_tree.reuse_subtree(
            range.start.dispatch_tree_index..range.end.dispatch_tree_index,
            &mut window.rendered_frame.dispatch_tree,
        );
        window.next_frame.deferred_draws.extend(
            window.rendered_frame.deferred_draws
                [range.start.deferred_draws_index..range.end.deferred_draws_index]
                .iter()
                .map(|deferred_draw| DeferredDraw {
                    parent_node: reused_subtree.refresh_node_id(deferred_draw.parent_node),
                    element_id_stack: deferred_draw.element_id_stack.clone(),
                    text_style_stack: deferred_draw.text_style_stack.clone(),
                    priority: deferred_draw.priority,
                    element: None,
                    absolute_offset: deferred_draw.absolute_offset,
                    prepaint_range: deferred_draw.prepaint_range.clone(),
                    paint_range: deferred_draw.paint_range.clone(),
                }),
        );
    }

    pub(crate) fn paint_index(&self) -> PaintIndex {
        PaintIndex {
            scene_index: self.window.next_frame.scene.len(),
            mouse_listeners_index: self.window.next_frame.mouse_listeners.len(),
            input_handlers_index: self.window.next_frame.input_handlers.len(),
            cursor_styles_index: self.window.next_frame.cursor_styles.len(),
            accessed_element_states_index: self.window.next_frame.accessed_element_states.len(),
            line_layout_index: self.window.text_system.layout_index(),
        }
    }

    pub(crate) fn reuse_paint(&mut self, range: Range<PaintIndex>) {
        let window = &mut self.window;

        window.next_frame.cursor_styles.extend(
            window.rendered_frame.cursor_styles
                [range.start.cursor_styles_index..range.end.cursor_styles_index]
                .iter()
                .cloned(),
        );
        window.next_frame.input_handlers.extend(
            window.rendered_frame.input_handlers
                [range.start.input_handlers_index..range.end.input_handlers_index]
                .iter_mut()
                .map(|handler| handler.take()),
        );
        window.next_frame.mouse_listeners.extend(
            window.rendered_frame.mouse_listeners
                [range.start.mouse_listeners_index..range.end.mouse_listeners_index]
                .iter_mut()
                .map(|listener| listener.take()),
        );
        window.next_frame.accessed_element_states.extend(
            window.rendered_frame.accessed_element_states[range.start.accessed_element_states_index
                ..range.end.accessed_element_states_index]
                .iter()
                .map(|(id, type_id)| (GlobalElementId(id.0.clone()), *type_id)),
        );
        window
            .text_system
            .reuse_layouts(range.start.line_layout_index..range.end.line_layout_index);
        window.next_frame.scene.replay(
            range.start.scene_index..range.end.scene_index,
            &window.rendered_frame.scene,
        );
    }

    /// Push a text style onto the stack, and call a function with that style active.
    /// Use [`AppContext::text_style`] to get the current, combined text style. This method
    /// should only be called as part of element drawing.
    pub fn with_text_style<F, R>(&mut self, style: Option<TextStyleRefinement>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        debug_assert!(
            matches!(
                self.window.draw_phase,
                DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during request_layout, prepaint, or paint"
        );
        if let Some(style) = style {
            self.window.text_style_stack.push(style);
            let result = f(self);
            self.window.text_style_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Updates the cursor style at the platform level. This method should only be called
    /// during the prepaint phase of element drawing.
    pub fn set_cursor_style(&mut self, style: CursorStyle, hitbox: &Hitbox) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );
        self.window
            .next_frame
            .cursor_styles
            .push(CursorStyleRequest {
                hitbox_id: hitbox.id,
                style,
            });
    }

    /// Sets a tooltip to be rendered for the upcoming frame. This method should only be called
    /// during the paint phase of element drawing.
    pub fn set_tooltip(&mut self, tooltip: AnyTooltip) -> TooltipId {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );
        let id = TooltipId(post_inc(&mut self.window.next_tooltip_id.0));
        self.window
            .next_frame
            .tooltip_requests
            .push(Some(TooltipRequest { id, tooltip }));
        id
    }

    /// Invoke the given function with the given content mask after intersecting it
    /// with the current mask. This method should only be called during element drawing.
    pub fn with_content_mask<R>(
        &mut self,
        mask: Option<ContentMask<Pixels>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        debug_assert!(
            matches!(
                self.window.draw_phase,
                DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during request_layout, prepaint, or paint"
        );
        if let Some(mask) = mask {
            let mask = mask.intersect(&self.content_mask());
            self.window_mut().content_mask_stack.push(mask);
            let result = f(self);
            self.window_mut().content_mask_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Updates the global element offset relative to the current offset. This is used to implement
    /// scrolling. This method should only be called during the prepaint phase of element drawing.
    pub fn with_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout, or prepaint"
        );

        if offset.is_zero() {
            return f(self);
        };

        let abs_offset = self.element_offset() + offset;
        self.with_absolute_element_offset(abs_offset, f)
    }

    /// Updates the global element offset based on the given offset. This is used to implement
    /// drag handles and other manual painting of elements. This method should only be called during
    /// the prepaint phase of element drawing.
    pub fn with_absolute_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout, or prepaint"
        );
        self.window_mut().element_offset_stack.push(offset);
        let result = f(self);
        self.window_mut().element_offset_stack.pop();
        result
    }

    /// Perform prepaint on child elements in a "retryable" manner, so that any side effects
    /// of prepaints can be discarded before prepainting again. This is used to support autoscroll
    /// where we need to prepaint children to detect the autoscroll bounds, then adjust the
    /// element offset and prepaint again. See [`List`] for an example. This method should only be
    /// called during the prepaint phase of element drawing.
    pub fn transact<T, U>(&mut self, f: impl FnOnce(&mut Self) -> Result<T, U>) -> Result<T, U> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );
        let index = self.prepaint_index();
        let result = f(self);
        if result.is_err() {
            self.window
                .next_frame
                .hitboxes
                .truncate(index.hitboxes_index);
            self.window
                .next_frame
                .tooltip_requests
                .truncate(index.tooltips_index);
            self.window
                .next_frame
                .deferred_draws
                .truncate(index.deferred_draws_index);
            self.window
                .next_frame
                .dispatch_tree
                .truncate(index.dispatch_tree_index);
            self.window
                .next_frame
                .accessed_element_states
                .truncate(index.accessed_element_states_index);
            self.window
                .text_system
                .truncate_layouts(index.line_layout_index);
        }
        result
    }

    /// When you call this method during [`prepaint`], containing elements will attempt to
    /// scroll to cause the specified bounds to become visible. When they decide to autoscroll, they will call
    /// [`prepaint`] again with a new set of bounds. See [`List`] for an example of an element
    /// that supports this method being called on the elements it contains. This method should only be
    /// called during the prepaint phase of element drawing.
    pub fn request_autoscroll(&mut self, bounds: Bounds<Pixels>) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );
        self.window.requested_autoscroll = Some(bounds);
    }

    /// This method can be called from a containing element such as [`List`] to support the autoscroll behavior
    /// described in [`request_autoscroll`].
    pub fn take_autoscroll(&mut self) -> Option<Bounds<Pixels>> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );
        self.window.requested_autoscroll.take()
    }

    /// Remove an asset from GPUI's cache
    pub fn remove_cached_asset<A: Asset + 'static>(
        &mut self,
        source: &A::Source,
    ) -> Option<A::Output> {
        self.asset_cache.remove::<A>(source)
    }

    /// Asynchronously load an asset, if the asset hasn't finished loading this will return None.
    /// Your view will be re-drawn once the asset has finished loading.
    ///
    /// Note that the multiple calls to this method will only result in one `Asset::load` call.
    /// The results of that call will be cached, and returned on subsequent uses of this API.
    ///
    /// Use [Self::remove_cached_asset] to reload your asset.
    pub fn use_cached_asset<A: Asset + 'static>(
        &mut self,
        source: &A::Source,
    ) -> Option<A::Output> {
        self.asset_cache.get::<A>(source).or_else(|| {
            if let Some(asset) = self.use_asset::<A>(source) {
                self.asset_cache
                    .insert::<A>(source.to_owned(), asset.clone());
                Some(asset)
            } else {
                None
            }
        })
    }

    /// Asynchronously load an asset, if the asset hasn't finished loading this will return None.
    /// Your view will be re-drawn once the asset has finished loading.
    ///
    /// Note that the multiple calls to this method will only result in one `Asset::load` call at a
    /// time.
    ///
    /// This asset will not be cached by default, see [Self::use_cached_asset]
    pub fn use_asset<A: Asset + 'static>(&mut self, source: &A::Source) -> Option<A::Output> {
        let asset_id = (TypeId::of::<A>(), hash(source));
        let mut is_first = false;
        let task = self
            .loading_assets
            .remove(&asset_id)
            .map(|boxed_task| *boxed_task.downcast::<Shared<Task<A::Output>>>().unwrap())
            .unwrap_or_else(|| {
                is_first = true;
                let future = A::load(source.clone(), self);
                let task = self.background_executor().spawn(future).shared();
                task
            });

        task.clone().now_or_never().or_else(|| {
            if is_first {
                let parent_id = self.parent_view_id();
                self.spawn({
                    let task = task.clone();
                    |mut cx| async move {
                        task.await;

                        cx.on_next_frame(move |cx| {
                            if let Some(parent_id) = parent_id {
                                cx.notify(parent_id)
                            } else {
                                cx.refresh()
                            }
                        });
                    }
                })
                .detach();
            }

            self.loading_assets.insert(asset_id, Box::new(task));

            None
        })
    }

    /// Obtain the current element offset. This method should only be called during the
    /// prepaint phase of element drawing.
    pub fn element_offset(&self) -> Point<Pixels> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );
        self.window()
            .element_offset_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    /// Obtain the current content mask. This method should only be called during element drawing.
    pub fn content_mask(&self) -> ContentMask<Pixels> {
        debug_assert!(
            matches!(
                self.window.draw_phase,
                DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during prepaint, or paint"
        );
        self.window()
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

    /// Provide elements in the called function with a new namespace in which their identiers must be unique.
    /// This can be used within a custom element to distinguish multiple sets of child elements.
    pub fn with_element_namespace<R>(
        &mut self,
        element_id: impl Into<ElementId>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.window.element_id_stack.push(element_id.into());
        let result = f(self);
        self.window.element_id_stack.pop();
        result
    }

    /// Updates or initializes state for an element with the given id that lives across multiple
    /// frames. If an element with this ID existed in the rendered frame, its state will be passed
    /// to the given closure. The state returned by the closure will be stored so it can be referenced
    /// when drawing the next frame. This method should only be called as part of element drawing.
    pub fn with_element_state<S, R>(
        &mut self,
        global_id: &GlobalElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static,
    {
        debug_assert!(
            matches!(
                self.window.draw_phase,
                DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during request_layout, prepaint, or paint"
        );

        let key = (GlobalElementId(global_id.0.clone()), TypeId::of::<S>());
        self.window
            .next_frame
            .accessed_element_states
            .push((GlobalElementId(key.0.clone()), TypeId::of::<S>()));

        if let Some(any) = self
            .window
            .next_frame
            .element_states
            .remove(&key)
            .or_else(|| self.window.rendered_frame.element_states.remove(&key))
        {
            let ElementStateBox {
                inner,
                #[cfg(debug_assertions)]
                type_name,
            } = any;
            // Using the extra inner option to avoid needing to reallocate a new box.
            let mut state_box = inner
                .downcast::<Option<S>>()
                .map_err(|_| {
                    #[cfg(debug_assertions)]
                    {
                        anyhow::anyhow!(
                            "invalid element state type for id, requested {:?}, actual: {:?}",
                            std::any::type_name::<S>(),
                            type_name
                        )
                    }

                    #[cfg(not(debug_assertions))]
                    {
                        anyhow::anyhow!(
                            "invalid element state type for id, requested {:?}",
                            std::any::type_name::<S>(),
                        )
                    }
                })
                .unwrap();

            let state = state_box.take().expect(
                "reentrant call to with_element_state for the same state type and element id",
            );
            let (result, state) = f(Some(state), self);
            state_box.replace(state);
            self.window.next_frame.element_states.insert(
                key,
                ElementStateBox {
                    inner: state_box,
                    #[cfg(debug_assertions)]
                    type_name,
                },
            );
            result
        } else {
            let (result, state) = f(None, self);
            self.window.next_frame.element_states.insert(
                key,
                ElementStateBox {
                    inner: Box::new(Some(state)),
                    #[cfg(debug_assertions)]
                    type_name: std::any::type_name::<S>(),
                },
            );
            result
        }
    }

    /// A variant of `with_element_state` that allows the element's id to be optional. This is a convenience
    /// method for elements where the element id may or may not be assigned. Prefer using `with_element_state`
    /// when the element is guaranteed to have an id.
    pub fn with_optional_element_state<S, R>(
        &mut self,
        global_id: Option<&GlobalElementId>,
        f: impl FnOnce(Option<Option<S>>, &mut Self) -> (R, Option<S>),
    ) -> R
    where
        S: 'static,
    {
        debug_assert!(
            matches!(
                self.window.draw_phase,
                DrawPhase::Prepaint | DrawPhase::Paint
            ),
            "this method can only be called during request_layout, prepaint, or paint"
        );

        if let Some(global_id) = global_id {
            self.with_element_state(global_id, |state, cx| {
                let (result, state) = f(Some(state), cx);
                let state =
                    state.expect("you must return some state when you pass some element id");
                (result, state)
            })
        } else {
            let (result, state) = f(None, self);
            debug_assert!(
                state.is_none(),
                "you must not return an element state when passing None for the global id"
            );
            result
        }
    }

    /// Defers the drawing of the given element, scheduling it to be painted on top of the currently-drawn tree
    /// at a later time. The `priority` parameter determines the drawing order relative to other deferred elements,
    /// with higher values being drawn on top.
    ///
    /// This method should only be called as part of the prepaint phase of element drawing.
    pub fn defer_draw(
        &mut self,
        element: AnyElement,
        absolute_offset: Point<Pixels>,
        priority: usize,
    ) {
        let window = &mut self.window;
        debug_assert_eq!(
            window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout or prepaint"
        );
        let parent_node = window.next_frame.dispatch_tree.active_node_id().unwrap();
        window.next_frame.deferred_draws.push(DeferredDraw {
            parent_node,
            element_id_stack: window.element_id_stack.clone(),
            text_style_stack: window.text_style_stack.clone(),
            priority,
            element: Some(element),
            absolute_offset,
            prepaint_range: PrepaintStateIndex::default()..PrepaintStateIndex::default(),
            paint_range: PaintIndex::default()..PaintIndex::default(),
        });
    }

    /// Creates a new painting layer for the specified bounds. A "layer" is a batch
    /// of geometry that are non-overlapping and have the same draw order. This is typically used
    /// for performance reasons.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_layer<R>(&mut self, bounds: Bounds<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let clipped_bounds = bounds.intersect(&content_mask.bounds);
        if !clipped_bounds.is_empty() {
            self.window
                .next_frame
                .scene
                .push_layer(clipped_bounds.scale(scale_factor));
        }

        let result = f(self);

        if !clipped_bounds.is_empty() {
            self.window.next_frame.scene.pop_layer();
        }

        result
    }

    /// Paint one or more drop shadows into the scene for the next frame at the current z-index.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_shadows(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        shadows: &[BoxShadow],
    ) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        for shadow in shadows {
            let mut shadow_bounds = bounds;
            shadow_bounds.origin += shadow.offset;
            shadow_bounds.dilate(shadow.spread_radius);
            self.window.next_frame.scene.insert_primitive(Shadow {
                order: 0,
                blur_radius: shadow.blur_radius.scale(scale_factor),
                bounds: shadow_bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                corner_radii: corner_radii.scale(scale_factor),
                color: shadow.color,
            });
        }
    }

    /// Paint one or more quads into the scene for the next frame at the current stacking context.
    /// Quads are colored rectangular regions with an optional background, border, and corner radius.
    /// see [`fill`](crate::fill), [`outline`](crate::outline), and [`quad`](crate::quad) to construct this type.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_quad(&mut self, quad: PaintQuad) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        self.window.next_frame.scene.insert_primitive(Quad {
            order: 0,
            pad: 0,
            bounds: quad.bounds.scale(scale_factor),
            content_mask: content_mask.scale(scale_factor),
            background: quad.background,
            border_color: quad.border_color,
            corner_radii: quad.corner_radii.scale(scale_factor),
            border_widths: quad.border_widths.scale(scale_factor),
        });
    }

    /// Paint the given `Path` into the scene for the next frame at the current z-index.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Hsla>) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        path.content_mask = content_mask;
        path.color = color.into();
        self.window
            .next_frame
            .scene
            .insert_primitive(path.scale(scale_factor));
    }

    /// Paint an underline into the scene for the next frame at the current z-index.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_underline(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &UnderlineStyle,
    ) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

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

        self.window.next_frame.scene.insert_primitive(Underline {
            order: 0,
            pad: 0,
            bounds: bounds.scale(scale_factor),
            content_mask: content_mask.scale(scale_factor),
            color: style.color.unwrap_or_default(),
            thickness: style.thickness.scale(scale_factor),
            wavy: style.wavy,
        });
    }

    /// Paint a strikethrough into the scene for the next frame at the current z-index.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_strikethrough(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &StrikethroughStyle,
    ) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let height = style.thickness;
        let bounds = Bounds {
            origin,
            size: size(width, height),
        };
        let content_mask = self.content_mask();

        self.window.next_frame.scene.insert_primitive(Underline {
            order: 0,
            pad: 0,
            bounds: bounds.scale(scale_factor),
            content_mask: content_mask.scale(scale_factor),
            thickness: style.thickness.scale(scale_factor),
            color: style.color.unwrap_or_default(),
            wavy: false,
        });
    }

    /// Paints a monochrome (non-emoji) glyph into the scene for the next frame at the current z-index.
    ///
    /// The y component of the origin is the baseline of the glyph.
    /// You should generally prefer to use the [`ShapedLine::paint`](crate::ShapedLine::paint) or
    /// [`WrappedLine::paint`](crate::WrappedLine::paint) methods in the [`TextSystem`](crate::TextSystem).
    /// This method is only useful if you need to paint a single glyph that has already been shaped.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_glyph(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
        color: Hsla,
    ) -> Result<()> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

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
            let tile = self
                .window
                .sprite_atlas
                .get_or_insert_with(&params.clone().into(), &mut || {
                    let (size, bytes) = self.text_system().rasterize_glyph(&params)?;
                    Ok(Some((size, Cow::Owned(bytes))))
                })?
                .expect("Callback above only errors or returns Some");
            let bounds = Bounds {
                origin: glyph_origin.map(|px| px.floor()) + raster_bounds.origin.map(Into::into),
                size: tile.bounds.size.map(Into::into),
            };
            let content_mask = self.content_mask().scale(scale_factor);
            self.window
                .next_frame
                .scene
                .insert_primitive(MonochromeSprite {
                    order: 0,
                    pad: 0,
                    bounds,
                    content_mask,
                    color,
                    tile,
                    transformation: TransformationMatrix::unit(),
                });
        }
        Ok(())
    }

    /// Paints an emoji glyph into the scene for the next frame at the current z-index.
    ///
    /// The y component of the origin is the baseline of the glyph.
    /// You should generally prefer to use the [`ShapedLine::paint`](crate::ShapedLine::paint) or
    /// [`WrappedLine::paint`](crate::WrappedLine::paint) methods in the [`TextSystem`](crate::TextSystem).
    /// This method is only useful if you need to paint a single emoji that has already been shaped.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_emoji(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
    ) -> Result<()> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

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
            let tile = self
                .window
                .sprite_atlas
                .get_or_insert_with(&params.clone().into(), &mut || {
                    let (size, bytes) = self.text_system().rasterize_glyph(&params)?;
                    Ok(Some((size, Cow::Owned(bytes))))
                })?
                .expect("Callback above only errors or returns Some");

            let bounds = Bounds {
                origin: glyph_origin.map(|px| px.floor()) + raster_bounds.origin.map(Into::into),
                size: tile.bounds.size.map(Into::into),
            };
            let content_mask = self.content_mask().scale(scale_factor);

            self.window
                .next_frame
                .scene
                .insert_primitive(PolychromeSprite {
                    order: 0,
                    grayscale: false,
                    bounds,
                    corner_radii: Default::default(),
                    content_mask,
                    tile,
                });
        }
        Ok(())
    }

    /// Paint a monochrome SVG into the scene for the next frame at the current stacking context.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_svg(
        &mut self,
        bounds: Bounds<Pixels>,
        path: SharedString,
        transformation: TransformationMatrix,
        color: Hsla,
    ) -> Result<()> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        // Render the SVG at twice the size to get a higher quality result.
        let params = RenderSvgParams {
            path,
            size: bounds
                .size
                .map(|pixels| DevicePixels::from((pixels.0 * 2.).ceil() as i32)),
        };

        let Some(tile) =
            self.window
                .sprite_atlas
                .get_or_insert_with(&params.clone().into(), &mut || {
                    let Some(bytes) = self.svg_renderer.render(&params)? else {
                        return Ok(None);
                    };
                    Ok(Some((params.size, Cow::Owned(bytes))))
                })?
        else {
            return Ok(());
        };
        let content_mask = self.content_mask().scale(scale_factor);

        self.window
            .next_frame
            .scene
            .insert_primitive(MonochromeSprite {
                order: 0,
                pad: 0,
                bounds,
                content_mask,
                color,
                tile,
                transformation,
            });

        Ok(())
    }

    /// Paint an image into the scene for the next frame at the current z-index.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn paint_image(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        data: Arc<ImageData>,
        grayscale: bool,
    ) -> Result<()> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let params = RenderImageParams { image_id: data.id };

        let tile = self
            .window
            .sprite_atlas
            .get_or_insert_with(&params.clone().into(), &mut || {
                Ok(Some((data.size(), Cow::Borrowed(data.as_bytes()))))
            })?
            .expect("Callback above only returns Some");
        let content_mask = self.content_mask().scale(scale_factor);
        let corner_radii = corner_radii.scale(scale_factor);

        self.window
            .next_frame
            .scene
            .insert_primitive(PolychromeSprite {
                order: 0,
                grayscale,
                bounds,
                content_mask,
                corner_radii,
                tile,
            });
        Ok(())
    }

    /// Paint a surface into the scene for the next frame at the current z-index.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    #[cfg(target_os = "macos")]
    pub fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVImageBuffer) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let content_mask = self.content_mask().scale(scale_factor);
        self.window
            .next_frame
            .scene
            .insert_primitive(crate::Surface {
                order: 0,
                bounds,
                content_mask,
                image_buffer,
            });
    }

    #[must_use]
    /// Add a node to the layout tree for the current frame. Takes the `Style` of the element for which
    /// layout is being requested, along with the layout ids of any children. This method is called during
    /// calls to the [`Element::request_layout`] trait method and enables any element to participate in layout.
    ///
    /// This method should only be called as part of the request_layout or prepaint phase of element drawing.
    pub fn request_layout(
        &mut self,
        style: Style,
        children: impl IntoIterator<Item = LayoutId>,
    ) -> LayoutId {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout, or prepaint"
        );

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
    ///
    /// This method should only be called as part of the request_layout or prepaint phase of element drawing.
    pub fn request_measured_layout<
        F: FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut WindowContext) -> Size<Pixels>
            + 'static,
    >(
        &mut self,
        style: Style,
        measure: F,
    ) -> LayoutId {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout, or prepaint"
        );

        let rem_size = self.rem_size();
        self.window
            .layout_engine
            .as_mut()
            .unwrap()
            .request_measured_layout(style, rem_size, measure)
    }

    /// Compute the layout for the given id within the given available space.
    /// This method is called for its side effect, typically by the framework prior to painting.
    /// After calling it, you can request the bounds of the given layout node id or any descendant.
    ///
    /// This method should only be called as part of the prepaint phase of element drawing.
    pub fn compute_layout(&mut self, layout_id: LayoutId, available_space: Size<AvailableSpace>) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout, or prepaint"
        );

        let mut layout_engine = self.window.layout_engine.take().unwrap();
        layout_engine.compute_layout(layout_id, available_space, self);
        self.window.layout_engine = Some(layout_engine);
    }

    /// Obtain the bounds computed for the given LayoutId relative to the window. This method will usually be invoked by
    /// GPUI itself automatically in order to pass your element its `Bounds` automatically.
    ///
    /// This method should only be called as part of element drawing.
    pub fn layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels> {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during request_layout, prepaint, or paint"
        );

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

    /// This method should be called during `prepaint`. You can use
    /// the returned [Hitbox] during `paint` or in an event handler
    /// to determine whether the inserted hitbox was the topmost.
    ///
    /// This method should only be called as part of the prepaint phase of element drawing.
    pub fn insert_hitbox(&mut self, bounds: Bounds<Pixels>, opaque: bool) -> Hitbox {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );

        let content_mask = self.content_mask();
        let window = &mut self.window;
        let id = window.next_hitbox_id;
        window.next_hitbox_id.0 += 1;
        let hitbox = Hitbox {
            id,
            bounds,
            content_mask,
            opaque,
        };
        window.next_frame.hitboxes.push(hitbox.clone());
        hitbox
    }

    /// Sets the key context for the current element. This context will be used to translate
    /// keybindings into actions.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn set_key_context(&mut self, context: KeyContext) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );
        self.window
            .next_frame
            .dispatch_tree
            .set_key_context(context);
    }

    /// Sets the focus handle for the current element. This handle will be used to manage focus state
    /// and keyboard event dispatch for the element.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn set_focus_handle(&mut self, focus_handle: &FocusHandle) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );
        self.window
            .next_frame
            .dispatch_tree
            .set_focus_id(focus_handle.id);
    }

    /// Sets the view id for the current element, which will be used to manage view caching.
    ///
    /// This method should only be called as part of element prepaint. We plan on removing this
    /// method eventually when we solve some issues that require us to construct editor elements
    /// directly instead of always using editors via views.
    pub fn set_view_id(&mut self, view_id: EntityId) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Prepaint,
            "this method can only be called during prepaint"
        );
        self.window.next_frame.dispatch_tree.set_view_id(view_id);
    }

    /// Get the last view id for the current element
    pub fn parent_view_id(&mut self) -> Option<EntityId> {
        self.window.next_frame.dispatch_tree.parent_view_id()
    }

    /// Sets an input handler, such as [`ElementInputHandler`][element_input_handler], which interfaces with the
    /// platform to receive textual input with proper integration with concerns such
    /// as IME interactions. This handler will be active for the upcoming frame until the following frame is
    /// rendered.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    ///
    /// [element_input_handler]: crate::ElementInputHandler
    pub fn handle_input(&mut self, focus_handle: &FocusHandle, input_handler: impl InputHandler) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        if focus_handle.is_focused(self) {
            let cx = self.to_async();
            self.window
                .next_frame
                .input_handlers
                .push(Some(PlatformInputHandler::new(cx, Box::new(input_handler))));
        }
    }

    /// Register a mouse event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn on_mouse_event<Event: MouseEvent>(
        &mut self,
        mut handler: impl FnMut(&Event, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        self.window.next_frame.mouse_listeners.push(Some(Box::new(
            move |event: &dyn Any, phase: DispatchPhase, cx: &mut WindowContext<'_>| {
                if let Some(event) = event.downcast_ref() {
                    handler(event, phase, cx)
                }
            },
        )));
    }

    /// Register a key event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn on_key_event<Event: KeyEvent>(
        &mut self,
        listener: impl Fn(&Event, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        self.window.next_frame.dispatch_tree.on_key_event(Rc::new(
            move |event: &dyn Any, phase, cx: &mut WindowContext<'_>| {
                if let Some(event) = event.downcast_ref::<Event>() {
                    listener(event, phase, cx)
                }
            },
        ));
    }

    /// Register a modifiers changed event listener on the window for the next frame.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    ///
    /// This method should only be called as part of the paint phase of element drawing.
    pub fn on_modifiers_changed(
        &mut self,
        listener: impl Fn(&ModifiersChangedEvent, &mut WindowContext) + 'static,
    ) {
        debug_assert_eq!(
            self.window.draw_phase,
            DrawPhase::Paint,
            "this method can only be called during paint"
        );

        self.window
            .next_frame
            .dispatch_tree
            .on_modifiers_changed(Rc::new(
                move |event: &ModifiersChangedEvent, cx: &mut WindowContext<'_>| {
                    listener(event, cx)
                },
            ));
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

        // Capture phase, events bubble from back to front. Handlers for this phase are used for
        // special purposes, such as detecting events outside of a given Bounds.
        for listener in &mut mouse_listeners {
            let listener = listener.as_mut().unwrap();
            listener(event, DispatchPhase::Capture, self);
            if !self.app.propagate_event {
                break;
            }
        }

        // Bubble phase, where most normal handlers do their work.
        if self.app.propagate_event {
            for listener in mouse_listeners.iter_mut().rev() {
                let listener = listener.as_mut().unwrap();
                listener(event, DispatchPhase::Bubble, self);
                if !self.app.propagate_event {
                    break;
                }
            }
        }

        self.window.rendered_frame.mouse_listeners = mouse_listeners;

        if self.has_active_drag() {
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
                listener(event, self);
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
                    listener(any_action, DispatchPhase::Capture, self);

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
                    listener(any_action, DispatchPhase::Bubble, self);

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

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<crate::Reservation<T>> {
        self.app.reserve_model()
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: crate::Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>> {
        self.app.insert_model(reservation, build_model)
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

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<crate::Reservation<T>> {
        self.window_cx.reserve_model()
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: crate::Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>> {
        self.window_cx.insert_model(reservation, build_model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> R {
        self.window_cx.update_model(model, update)
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

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        self.window_cx.update_window(window, update)
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

unsafe impl<V> Send for WindowHandle<V> {}
unsafe impl<V> Sync for WindowHandle<V> {}

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
    /// A UUID.
    Uuid(Uuid),
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
            ElementId::Uuid(uuid) => write!(f, "{}", uuid)?,
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

impl From<Uuid> for ElementId {
    fn from(value: Uuid) -> Self {
        Self::Uuid(value)
    }
}

impl From<(&'static str, u32)> for ElementId {
    fn from((name, id): (&'static str, u32)) -> Self {
        ElementId::NamedInteger(name.into(), id as usize)
    }
}

/// A rectangle to be rendered in the window at the given position and size.
/// Passed as an argument [`WindowContext::paint_quad`].
#[derive(Clone)]
pub struct PaintQuad {
    /// The bounds of the quad within the window.
    pub bounds: Bounds<Pixels>,
    /// The radii of the quad's corners.
    pub corner_radii: Corners<Pixels>,
    /// The background color of the quad.
    pub background: Hsla,
    /// The widths of the quad's borders.
    pub border_widths: Edges<Pixels>,
    /// The color of the quad's borders.
    pub border_color: Hsla,
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
