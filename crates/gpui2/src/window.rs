use crate::{
    px, size, Action, AnyBox, AnyDrag, AnyView, AppContext, AsyncWindowContext, AvailableSpace,
    Bounds, BoxShadow, Context, Corners, DevicePixels, DispatchContext, DisplayId, Edges, Effect,
    Entity, EntityId, EventEmitter, FileDropEvent, FocusEvent, FontId, GlobalElementId, GlyphId,
    Hsla, ImageData, InputEvent, IsZero, KeyListener, KeyMatch, KeyMatcher, Keystroke, LayoutId,
    MainThread, MainThreadOnly, Model, ModelContext, Modifiers, MonochromeSprite, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Path, Pixels, PlatformAtlas, PlatformWindow,
    Point, PolychromeSprite, Quad, Reference, RenderGlyphParams, RenderImageParams,
    RenderSvgParams, ScaledPixels, SceneBuilder, Shadow, SharedString, Size, Style, Subscription,
    TaffyLayoutEngine, Task, Underline, UnderlineStyle, View, VisualContext, WeakModel, WeakView,
    WindowOptions, SUBPIXEL_VARIANTS,
};
use anyhow::Result;
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut, Cow},
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    mem,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use util::ResultExt;

/// A global stacking order, which is created by stacking successive z-index values.
/// Each z-index will always be interpreted in the context of its parent z-index.
#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq, Clone, Default)]
pub(crate) struct StackingOrder(pub(crate) SmallVec<[u32; 16]>);

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

type AnyListener = Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext) + Send + 'static>;
type AnyKeyListener = Box<
    dyn Fn(
            &dyn Any,
            &[&DispatchContext],
            DispatchPhase,
            &mut WindowContext,
        ) -> Option<Box<dyn Action>>
        + Send
        + 'static,
>;
type AnyFocusListener = Box<dyn Fn(&FocusEvent, &mut WindowContext) + Send + 'static>;

slotmap::new_key_type! { pub struct FocusId; }

/// A handle which can be used to track and manipulate the focused element in a window.
pub struct FocusHandle {
    pub(crate) id: FocusId,
    handles: Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
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

    /// Obtains whether the element associated with this handle is currently focused.
    pub fn is_focused(&self, cx: &WindowContext) -> bool {
        cx.window.focus == Some(self.id)
    }

    /// Obtains whether the element associated with this handle contains the focused
    /// element or is itself focused.
    pub fn contains_focused(&self, cx: &WindowContext) -> bool {
        cx.focused()
            .map_or(false, |focused| self.contains(&focused, cx))
    }

    /// Obtains whether the element associated with this handle is contained within the
    /// focused element or is itself focused.
    pub fn within_focused(&self, cx: &WindowContext) -> bool {
        let focused = cx.focused();
        focused.map_or(false, |focused| focused.contains(self, cx))
    }

    /// Obtains whether this handle contains the given handle in the most recently rendered frame.
    pub(crate) fn contains(&self, other: &Self, cx: &WindowContext) -> bool {
        let mut ancestor = Some(other.id);
        while let Some(ancestor_id) = ancestor {
            if self.id == ancestor_id {
                return true;
            } else {
                ancestor = cx.window.focus_parents_by_child.get(&ancestor_id).copied();
            }
        }
        false
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

// Holds the state for a specific window.
pub struct Window {
    handle: AnyWindowHandle,
    platform_window: MainThreadOnly<Box<dyn PlatformWindow>>,
    display_id: DisplayId,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    rem_size: Pixels,
    content_size: Size<Pixels>,
    pub(crate) layout_engine: TaffyLayoutEngine,
    pub(crate) root_view: Option<AnyView>,
    pub(crate) element_id_stack: GlobalElementId,
    prev_frame_element_states: HashMap<GlobalElementId, AnyBox>,
    element_states: HashMap<GlobalElementId, AnyBox>,
    prev_frame_key_matchers: HashMap<GlobalElementId, KeyMatcher>,
    key_matchers: HashMap<GlobalElementId, KeyMatcher>,
    z_index_stack: StackingOrder,
    content_mask_stack: Vec<ContentMask<Pixels>>,
    element_offset_stack: Vec<Point<Pixels>>,
    mouse_listeners: HashMap<TypeId, Vec<(StackingOrder, AnyListener)>>,
    key_dispatch_stack: Vec<KeyDispatchStackFrame>,
    freeze_key_dispatch_stack: bool,
    focus_stack: Vec<FocusId>,
    focus_parents_by_child: HashMap<FocusId, FocusId>,
    pub(crate) focus_listeners: Vec<AnyFocusListener>,
    pub(crate) focus_handles: Arc<RwLock<SlotMap<FocusId, AtomicUsize>>>,
    default_prevented: bool,
    mouse_position: Point<Pixels>,
    scale_factor: f32,
    pub(crate) scene_builder: SceneBuilder,
    pub(crate) dirty: bool,
    pub(crate) last_blur: Option<Option<FocusId>>,
    pub(crate) focus: Option<FocusId>,
}

impl Window {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        cx: &mut MainThread<AppContext>,
    ) -> Self {
        let platform_window = cx.platform().open_window(handle, options);
        let display_id = platform_window.display().id();
        let sprite_atlas = platform_window.sprite_atlas();
        let mouse_position = platform_window.mouse_position();
        let content_size = platform_window.content_size();
        let scale_factor = platform_window.scale_factor();
        platform_window.on_resize(Box::new({
            let cx = cx.to_async();
            move |content_size, scale_factor| {
                cx.update_window(handle, |cx| {
                    cx.window.scale_factor = scale_factor;
                    cx.window.scene_builder = SceneBuilder::new();
                    cx.window.content_size = content_size;
                    cx.window.display_id = cx
                        .window
                        .platform_window
                        .borrow_on_main_thread()
                        .display()
                        .id();
                    cx.window.dirty = true;
                })
                .log_err();
            }
        }));

        platform_window.on_input({
            let cx = cx.to_async();
            Box::new(move |event| {
                cx.update_window(handle, |cx| cx.dispatch_event(event))
                    .log_err()
                    .unwrap_or(true)
            })
        });

        let platform_window = MainThreadOnly::new(Arc::new(platform_window), cx.executor.clone());

        Window {
            handle,
            platform_window,
            display_id,
            sprite_atlas,
            rem_size: px(16.),
            content_size,
            layout_engine: TaffyLayoutEngine::new(),
            root_view: None,
            element_id_stack: GlobalElementId::default(),
            prev_frame_element_states: HashMap::default(),
            element_states: HashMap::default(),
            prev_frame_key_matchers: HashMap::default(),
            key_matchers: HashMap::default(),
            z_index_stack: StackingOrder(SmallVec::new()),
            content_mask_stack: Vec::new(),
            element_offset_stack: Vec::new(),
            mouse_listeners: HashMap::default(),
            key_dispatch_stack: Vec::new(),
            freeze_key_dispatch_stack: false,
            focus_stack: Vec::new(),
            focus_parents_by_child: HashMap::default(),
            focus_listeners: Vec::new(),
            focus_handles: Arc::new(RwLock::new(SlotMap::with_key())),
            default_prevented: true,
            mouse_position,
            scale_factor,
            scene_builder: SceneBuilder::new(),
            dirty: true,
            last_blur: None,
            focus: None,
        }
    }
}

/// When constructing the element tree, we maintain a stack of key dispatch frames until we
/// find the focused element. We interleave key listeners with dispatch contexts so we can use the
/// contexts when matching key events against the keymap.
enum KeyDispatchStackFrame {
    Listener {
        event_type: TypeId,
        listener: AnyKeyListener,
    },
    Context(DispatchContext),
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
pub struct WindowContext<'a, 'w> {
    pub(crate) app: Reference<'a, AppContext>,
    pub(crate) window: Reference<'w, Window>,
}

impl<'a, 'w> WindowContext<'a, 'w> {
    pub(crate) fn immutable(app: &'a AppContext, window: &'w Window) -> Self {
        Self {
            app: Reference::Immutable(app),
            window: Reference::Immutable(window),
        }
    }

    pub(crate) fn mutable(app: &'a mut AppContext, window: &'w mut Window) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
        }
    }

    /// Obtain a handle to the window that belongs to this context.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window.handle
    }

    /// Mark the window as dirty, scheduling it to be redrawn on the next frame.
    pub fn notify(&mut self) {
        self.window.dirty = true;
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
        if self.window.last_blur.is_none() {
            self.window.last_blur = Some(self.window.focus);
        }

        let window_id = self.window.handle.id;
        self.window.focus = Some(handle.id);
        self.app.push_effect(Effect::FocusChanged {
            window_id,
            focused: Some(handle.id),
        });
        self.notify();
    }

    /// Remove focus from all elements within this context's window.
    pub fn blur(&mut self) {
        if self.window.last_blur.is_none() {
            self.window.last_blur = Some(self.window.focus);
        }

        let window_id = self.window.handle.id;
        self.window.focus = None;
        self.app.push_effect(Effect::FocusChanged {
            window_id,
            focused: None,
        });
        self.notify();
    }

    /// Schedule the given closure to be run on the main thread. It will be invoked with
    /// a `MainThread<WindowContext>`, which provides access to platform-specific functionality
    /// of the window.
    pub fn run_on_main<R>(
        &mut self,
        f: impl FnOnce(&mut MainThread<WindowContext<'_, '_>>) -> R + Send + 'static,
    ) -> Task<Result<R>>
    where
        R: Send + 'static,
    {
        if self.executor.is_main_thread() {
            Task::ready(Ok(f(unsafe {
                mem::transmute::<&mut Self, &mut MainThread<Self>>(self)
            })))
        } else {
            let id = self.window.handle.id;
            self.app.run_on_main(move |cx| cx.update_window(id, f))
        }
    }

    /// Create an `AsyncWindowContext`, which has a static lifetime and can be held across
    /// await points in async code.
    pub fn to_async(&self) -> AsyncWindowContext {
        AsyncWindowContext::new(self.app.to_async(), self.window.handle)
    }

    /// Schedule the given closure to be run directly after the current frame is rendered.
    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut WindowContext) + Send + 'static) {
        let f = Box::new(f);
        let display_id = self.window.display_id;
        self.run_on_main(move |cx| {
            if let Some(callbacks) = cx.next_frame_callbacks.get_mut(&display_id) {
                callbacks.push(f);
                // If there was already a callback, it means that we already scheduled a frame.
                if callbacks.len() > 1 {
                    return;
                }
            } else {
                let async_cx = cx.to_async();
                cx.next_frame_callbacks.insert(display_id, vec![f]);
                cx.platform().set_display_link_output_callback(
                    display_id,
                    Box::new(move |_current_time, _output_time| {
                        let _ = async_cx.update(|cx| {
                            let callbacks = cx
                                .next_frame_callbacks
                                .get_mut(&display_id)
                                .unwrap()
                                .drain(..)
                                .collect::<Vec<_>>();
                            for callback in callbacks {
                                callback(cx);
                            }

                            cx.run_on_main(move |cx| {
                                if cx.next_frame_callbacks.get(&display_id).unwrap().is_empty() {
                                    cx.platform().stop_display_link(display_id);
                                }
                            })
                            .detach();
                        });
                    }),
                );
            }

            cx.platform().start_display_link(display_id);
        })
        .detach();
    }

    /// Spawn the future returned by the given closure on the application thread pool.
    /// The closure is provided a handle to the current window and an `AsyncWindowContext` for
    /// use within your future.
    pub fn spawn<Fut, R>(
        &mut self,
        f: impl FnOnce(AnyWindowHandle, AsyncWindowContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
        Fut: Future<Output = R> + Send + 'static,
    {
        let window = self.window.handle;
        self.app.spawn(move |app| {
            let cx = AsyncWindowContext::new(app, window);
            let future = f(window, cx);
            async move { future.await }
        })
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

        self.window
            .layout_engine
            .request_layout(style, rem_size, &self.app.layout_id_buffer)
    }

    /// Add a node to the layout tree for the current frame. Instead of taking a `Style` and children,
    /// this variant takes a function that is invoked during layout so you can use arbitrary logic to
    /// determine the element's size. One place this is used internally is when measuring text.
    ///
    /// The given closure is invoked at layout time with the known dimensions and available space and
    /// returns a `Size`.
    pub fn request_measured_layout<
        F: Fn(Size<Option<Pixels>>, Size<AvailableSpace>) -> Size<Pixels> + Send + Sync + 'static,
    >(
        &mut self,
        style: Style,
        rem_size: Pixels,
        measure: F,
    ) -> LayoutId {
        self.window
            .layout_engine
            .request_measured_layout(style, rem_size, measure)
    }

    /// Obtain the bounds computed for the given LayoutId relative to the window. This method should not
    /// be invoked until the paint phase begins, and will usually be invoked by GPUI itself automatically
    /// in order to pass your element its `Bounds` automatically.
    pub fn layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels> {
        let mut bounds = self
            .window
            .layout_engine
            .layout_bounds(layout_id)
            .map(Into::into);
        bounds.origin += self.element_offset();
        bounds
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

    /// Register a mouse event listener on the window for the current frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_mouse_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&Event, DispatchPhase, &mut WindowContext) + Send + 'static,
    ) {
        let order = self.window.z_index_stack.clone();
        self.window
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

    /// The position of the mouse relative to the window.
    pub fn mouse_position(&self) -> Point<Pixels> {
        self.window.mouse_position
    }

    /// Called during painting to invoke the given closure in a new stacking context. The given
    /// z-index is interpreted relative to the previous call to `stack`.
    pub fn stack<R>(&mut self, z_index: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.z_index_stack.push(z_index);
        let result = f(self);
        self.window.z_index_stack.pop();
        result
    }

    /// Paint one or more drop shadows into the scene for the current frame at the current z-index.
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
            window.scene_builder.insert(
                &window.z_index_stack,
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

    /// Paint one or more quads into the scene for the current frame at the current stacking context.
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
        window.scene_builder.insert(
            &window.z_index_stack,
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

    /// Paint the given `Path` into the scene for the current frame at the current z-index.
    pub fn paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Hsla>) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        path.content_mask = content_mask;
        path.color = color.into();
        let window = &mut *self.window;
        window
            .scene_builder
            .insert(&window.z_index_stack, path.scale(scale_factor));
    }

    /// Paint an underline into the scene for the current frame at the current z-index.
    pub fn paint_underline(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &UnderlineStyle,
    ) -> Result<()> {
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
        window.scene_builder.insert(
            &window.z_index_stack,
            Underline {
                order: 0,
                bounds: bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                thickness: style.thickness.scale(scale_factor),
                color: style.color.unwrap_or_default(),
                wavy: style.wavy,
            },
        );
        Ok(())
    }

    /// Paint a monochrome (non-emoji) glyph into the scene for the current frame at the current z-index.
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
            window.scene_builder.insert(
                &window.z_index_stack,
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

    /// Paint an emoji glyph into the scene for the current frame at the current z-index.
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

            window.scene_builder.insert(
                &window.z_index_stack,
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

    /// Paint a monochrome SVG into the scene for the current frame at the current stacking context.
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
        window.scene_builder.insert(
            &window.z_index_stack,
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

    /// Paint an image into the scene for the current frame at the current z-index.
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
        window.scene_builder.insert(
            &window.z_index_stack,
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

    /// Draw pixels to the display for this window based on the contents of its scene.
    pub(crate) fn draw(&mut self) {
        let root_view = self.window.root_view.take().unwrap();

        self.start_frame();

        self.stack(0, |cx| {
            let available_space = cx.window.content_size.map(Into::into);
            root_view.draw(available_space, cx);
        });

        if let Some(active_drag) = self.app.active_drag.take() {
            self.stack(1, |cx| {
                let offset = cx.mouse_position() - active_drag.cursor_offset;
                cx.with_element_offset(Some(offset), |cx| {
                    let available_space =
                        size(AvailableSpace::MinContent, AvailableSpace::MinContent);
                    active_drag.view.draw(available_space, cx);
                    cx.active_drag = Some(active_drag);
                });
            });
        }

        self.window.root_view = Some(root_view);
        let scene = self.window.scene_builder.build();

        self.run_on_main(|cx| {
            cx.window
                .platform_window
                .borrow_on_main_thread()
                .draw(scene);
            cx.window.dirty = false;
        })
        .detach();
    }

    fn start_frame(&mut self) {
        self.text_system().start_frame();

        let window = &mut *self.window;

        // Move the current frame element states to the previous frame.
        // The new empty element states map will be populated for any element states we
        // reference during the upcoming frame.
        mem::swap(
            &mut window.element_states,
            &mut window.prev_frame_element_states,
        );
        window.element_states.clear();

        // Make the current key matchers the previous, and then clear the current.
        // An empty key matcher map will be created for every identified element in the
        // upcoming frame.
        mem::swap(
            &mut window.key_matchers,
            &mut window.prev_frame_key_matchers,
        );
        window.key_matchers.clear();

        // Clear mouse event listeners, because elements add new element listeners
        // when the upcoming frame is painted.
        window.mouse_listeners.values_mut().for_each(Vec::clear);

        // Clear focus state, because we determine what is focused when the new elements
        // in the upcoming frame are initialized.
        window.focus_listeners.clear();
        window.key_dispatch_stack.clear();
        window.focus_parents_by_child.clear();
        window.freeze_key_dispatch_stack = false;
    }

    /// Dispatch a mouse or keyboard event on the window.
    fn dispatch_event(&mut self, event: InputEvent) -> bool {
        let event = match event {
            // Track the mouse position with our own state, since accessing the platform
            // API for the mouse position can only occur on the main thread.
            InputEvent::MouseMove(mouse_move) => {
                self.window.mouse_position = mouse_move.position;
                InputEvent::MouseMove(mouse_move)
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
                    InputEvent::MouseDown(MouseDownEvent {
                        position,
                        button: MouseButton::Left,
                        click_count: 1,
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
            // Handlers may set this to false by calling `stop_propagation`
            self.app.propagate_event = true;
            self.window.default_prevented = false;

            if let Some(mut handlers) = self
                .window
                .mouse_listeners
                .remove(&any_mouse_event.type_id())
            {
                // Because handlers may add other handlers, we sort every time.
                handlers.sort_by(|(a, _), (b, _)| a.cmp(b));

                // Capture phase, events bubble from back to front. Handlers for this phase are used for
                // special purposes, such as detecting events outside of a given Bounds.
                for (_, handler) in &handlers {
                    handler(any_mouse_event, DispatchPhase::Capture, self);
                    if !self.app.propagate_event {
                        break;
                    }
                }

                // Bubble phase, where most normal handlers do their work.
                if self.app.propagate_event {
                    for (_, handler) in handlers.iter().rev() {
                        handler(any_mouse_event, DispatchPhase::Bubble, self);
                        if !self.app.propagate_event {
                            break;
                        }
                    }
                }

                if self.app.propagate_event
                    && any_mouse_event.downcast_ref::<MouseUpEvent>().is_some()
                {
                    self.active_drag = None;
                }

                // Just in case any handlers added new handlers, which is weird, but possible.
                handlers.extend(
                    self.window
                        .mouse_listeners
                        .get_mut(&any_mouse_event.type_id())
                        .into_iter()
                        .flat_map(|handlers| handlers.drain(..)),
                );
                self.window
                    .mouse_listeners
                    .insert(any_mouse_event.type_id(), handlers);
            }
        } else if let Some(any_key_event) = event.keyboard_event() {
            let key_dispatch_stack = mem::take(&mut self.window.key_dispatch_stack);
            let key_event_type = any_key_event.type_id();
            let mut context_stack = SmallVec::<[&DispatchContext; 16]>::new();

            for (ix, frame) in key_dispatch_stack.iter().enumerate() {
                match frame {
                    KeyDispatchStackFrame::Listener {
                        event_type,
                        listener,
                    } => {
                        if key_event_type == *event_type {
                            if let Some(action) = listener(
                                any_key_event,
                                &context_stack,
                                DispatchPhase::Capture,
                                self,
                            ) {
                                self.dispatch_action(action, &key_dispatch_stack[..ix]);
                            }
                            if !self.app.propagate_event {
                                break;
                            }
                        }
                    }
                    KeyDispatchStackFrame::Context(context) => {
                        context_stack.push(&context);
                    }
                }
            }

            if self.app.propagate_event {
                for (ix, frame) in key_dispatch_stack.iter().enumerate().rev() {
                    match frame {
                        KeyDispatchStackFrame::Listener {
                            event_type,
                            listener,
                        } => {
                            if key_event_type == *event_type {
                                if let Some(action) = listener(
                                    any_key_event,
                                    &context_stack,
                                    DispatchPhase::Bubble,
                                    self,
                                ) {
                                    self.dispatch_action(action, &key_dispatch_stack[..ix]);
                                }

                                if !self.app.propagate_event {
                                    break;
                                }
                            }
                        }
                        KeyDispatchStackFrame::Context(_) => {
                            context_stack.pop();
                        }
                    }
                }
            }

            drop(context_stack);
            self.window.key_dispatch_stack = key_dispatch_stack;
        }

        true
    }

    /// Attempt to map a keystroke to an action based on the keymap.
    pub fn match_keystroke(
        &mut self,
        element_id: &GlobalElementId,
        keystroke: &Keystroke,
        context_stack: &[&DispatchContext],
    ) -> KeyMatch {
        let key_match = self
            .window
            .key_matchers
            .get_mut(element_id)
            .unwrap()
            .match_keystroke(keystroke, context_stack);

        if key_match.is_some() {
            for matcher in self.window.key_matchers.values_mut() {
                matcher.clear_pending();
            }
        }

        key_match
    }

    /// Register the given handler to be invoked whenever the global of the given type
    /// is updated.
    pub fn observe_global<G: 'static>(
        &mut self,
        f: impl Fn(&mut WindowContext<'_, '_>) + Send + 'static,
    ) -> Subscription {
        let window_id = self.window.handle.id;
        self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| cx.update_window(window_id, |cx| f(cx)).is_ok()),
        )
    }

    fn dispatch_action(
        &mut self,
        action: Box<dyn Action>,
        dispatch_stack: &[KeyDispatchStackFrame],
    ) {
        let action_type = action.as_any().type_id();

        if let Some(mut global_listeners) = self.app.global_action_listeners.remove(&action_type) {
            for listener in &global_listeners {
                listener(action.as_ref(), DispatchPhase::Capture, self);
                if !self.app.propagate_event {
                    break;
                }
            }
            global_listeners.extend(
                self.global_action_listeners
                    .remove(&action_type)
                    .unwrap_or_default(),
            );
            self.global_action_listeners
                .insert(action_type, global_listeners);
        }

        if self.app.propagate_event {
            for stack_frame in dispatch_stack {
                if let KeyDispatchStackFrame::Listener {
                    event_type,
                    listener,
                } = stack_frame
                {
                    if action_type == *event_type {
                        listener(action.as_any(), &[], DispatchPhase::Capture, self);
                        if !self.app.propagate_event {
                            break;
                        }
                    }
                }
            }
        }

        if self.app.propagate_event {
            for stack_frame in dispatch_stack.iter().rev() {
                if let KeyDispatchStackFrame::Listener {
                    event_type,
                    listener,
                } = stack_frame
                {
                    if action_type == *event_type {
                        listener(action.as_any(), &[], DispatchPhase::Bubble, self);
                        if !self.app.propagate_event {
                            break;
                        }
                    }
                }
            }
        }

        if self.app.propagate_event {
            if let Some(mut global_listeners) =
                self.app.global_action_listeners.remove(&action_type)
            {
                for listener in global_listeners.iter().rev() {
                    listener(action.as_ref(), DispatchPhase::Bubble, self);
                    if !self.app.propagate_event {
                        break;
                    }
                }
                global_listeners.extend(
                    self.global_action_listeners
                        .remove(&action_type)
                        .unwrap_or_default(),
                );
                self.global_action_listeners
                    .insert(action_type, global_listeners);
            }
        }
    }
}

impl Context for WindowContext<'_, '_> {
    type ModelContext<'a, T> = ModelContext<'a, T>;
    type Result<T> = T;

    fn build_model<T>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Model<T>
    where
        T: 'static + Send,
    {
        let slot = self.app.entities.reserve();
        let model = build_model(&mut ModelContext::mutable(&mut *self.app, slot.downgrade()));
        self.entities.insert(slot, model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> R {
        let mut entity = self.entities.lease(model);
        let result = update(
            &mut *entity,
            &mut ModelContext::mutable(&mut *self.app, model.downgrade()),
        );
        self.entities.end_lease(entity);
        result
    }
}

impl VisualContext for WindowContext<'_, '_> {
    type ViewContext<'a, 'w, V> = ViewContext<'a, 'w, V>;

    fn build_view<V>(
        &mut self,
        build_view_state: impl FnOnce(&mut Self::ViewContext<'_, '_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Send,
    {
        let slot = self.app.entities.reserve();
        let view = View {
            model: slot.clone(),
        };
        let mut cx = ViewContext::mutable(&mut *self.app, &mut *self.window, view.downgrade());
        let entity = build_view_state(&mut cx);
        self.entities.insert(slot, entity);
        view
    }

    /// Update the given view. Prefer calling `View::update` instead, which calls this method.
    fn update_view<T: 'static, R>(
        &mut self,
        view: &View<T>,
        update: impl FnOnce(&mut T, &mut Self::ViewContext<'_, '_, T>) -> R,
    ) -> Self::Result<R> {
        let mut lease = self.app.entities.lease(&view.model);
        let mut cx = ViewContext::mutable(&mut *self.app, &mut *self.window, view.downgrade());
        let result = update(&mut *lease, &mut cx);
        cx.app.entities.end_lease(lease);
        result
    }
}

impl<'a, 'w> std::ops::Deref for WindowContext<'a, 'w> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<'a, 'w> std::ops::DerefMut for WindowContext<'a, 'w> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<'a, 'w> Borrow<AppContext> for WindowContext<'a, 'w> {
    fn borrow(&self) -> &AppContext {
        &self.app
    }
}

impl<'a, 'w> BorrowMut<AppContext> for WindowContext<'a, 'w> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        &mut self.app
    }
}

pub trait BorrowWindow: BorrowMut<Window> + BorrowMut<AppContext> {
    fn app_mut(&mut self) -> &mut AppContext {
        self.borrow_mut()
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
        id: impl Into<ElementId>,
        f: impl FnOnce(GlobalElementId, &mut Self) -> R,
    ) -> R {
        let keymap = self.app_mut().keymap.clone();
        let window = self.window_mut();
        window.element_id_stack.push(id.into());
        let global_id = window.element_id_stack.clone();

        if window.key_matchers.get(&global_id).is_none() {
            window.key_matchers.insert(
                global_id.clone(),
                window
                    .prev_frame_key_matchers
                    .remove(&global_id)
                    .unwrap_or_else(|| KeyMatcher::new(keymap)),
            );
        }

        let result = f(global_id, self);
        let window: &mut Window = self.borrow_mut();
        window.element_id_stack.pop();
        result
    }

    /// Invoke the given function with the given content mask after intersecting it
    /// with the current mask.
    fn with_content_mask<R>(
        &mut self,
        mask: ContentMask<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let mask = mask.intersect(&self.content_mask());
        self.window_mut().content_mask_stack.push(mask);
        let result = f(self);
        self.window_mut().content_mask_stack.pop();
        result
    }

    /// Update the global element offset based on the given offset. This is used to implement
    /// scrolling and position drag handles.
    fn with_element_offset<R>(
        &mut self,
        offset: Option<Point<Pixels>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let Some(offset) = offset else {
            return f(self);
        };

        let offset = self.element_offset() + offset;
        self.window_mut().element_offset_stack.push(offset);
        let result = f(self);
        self.window_mut().element_offset_stack.pop();
        result
    }

    /// Obtain the current element offset.
    fn element_offset(&self) -> Point<Pixels> {
        self.window()
            .element_offset_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    /// Update or intialize state for an element with the given id that lives across multiple
    /// frames. If an element with this id existed in the previous frame, its state will be passed
    /// to the given closure. The state returned by the closure will be stored so it can be referenced
    /// when drawing the next frame.
    fn with_element_state<S, R>(
        &mut self,
        id: ElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static + Send,
    {
        self.with_element_id(id, |global_id, cx| {
            if let Some(any) = cx
                .window_mut()
                .element_states
                .remove(&global_id)
                .or_else(|| cx.window_mut().prev_frame_element_states.remove(&global_id))
            {
                // Using the extra inner option to avoid needing to reallocate a new box.
                let mut state_box = any
                    .downcast::<Option<S>>()
                    .expect("invalid element state type for id");
                let state = state_box
                    .take()
                    .expect("element state is already on the stack");
                let (result, state) = f(Some(state), cx);
                state_box.replace(state);
                cx.window_mut().element_states.insert(global_id, state_box);
                result
            } else {
                let (result, state) = f(None, cx);
                cx.window_mut()
                    .element_states
                    .insert(global_id, Box::new(Some(state)));
                result
            }
        })
    }

    /// Like `with_element_state`, but for situations where the element_id is optional. If the
    /// id is `None`, no state will be retrieved or stored.
    fn with_optional_element_state<S, R>(
        &mut self,
        element_id: Option<ElementId>,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static + Send,
    {
        if let Some(element_id) = element_id {
            self.with_element_state(element_id, f)
        } else {
            f(None, self).0
        }
    }

    /// Obtain the current content mask.
    fn content_mask(&self) -> ContentMask<Pixels> {
        self.window()
            .content_mask_stack
            .last()
            .cloned()
            .unwrap_or_else(|| ContentMask {
                bounds: Bounds {
                    origin: Point::default(),
                    size: self.window().content_size,
                },
            })
    }

    /// The size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    fn rem_size(&self) -> Pixels {
        self.window().rem_size
    }
}

impl Borrow<Window> for WindowContext<'_, '_> {
    fn borrow(&self) -> &Window {
        &self.window
    }
}

impl BorrowMut<Window> for WindowContext<'_, '_> {
    fn borrow_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}

impl<T> BorrowWindow for T where T: BorrowMut<AppContext> + BorrowMut<Window> {}

pub struct ViewContext<'a, 'w, V> {
    window_cx: WindowContext<'a, 'w>,
    view: WeakView<V>,
}

impl<V> Borrow<AppContext> for ViewContext<'_, '_, V> {
    fn borrow(&self) -> &AppContext {
        &*self.window_cx.app
    }
}

impl<V> BorrowMut<AppContext> for ViewContext<'_, '_, V> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        &mut *self.window_cx.app
    }
}

impl<V> Borrow<Window> for ViewContext<'_, '_, V> {
    fn borrow(&self) -> &Window {
        &*self.window_cx.window
    }
}

impl<V> BorrowMut<Window> for ViewContext<'_, '_, V> {
    fn borrow_mut(&mut self) -> &mut Window {
        &mut *self.window_cx.window
    }
}

impl<'a, 'w, V: 'static> ViewContext<'a, 'w, V> {
    pub(crate) fn mutable(
        app: &'a mut AppContext,
        window: &'w mut Window,
        view: WeakView<V>,
    ) -> Self {
        Self {
            window_cx: WindowContext::mutable(app, window),
            view,
        }
    }

    pub fn view(&self) -> WeakView<V> {
        self.view.clone()
    }

    pub fn model(&self) -> WeakModel<V> {
        self.view.model.clone()
    }

    pub fn stack<R>(&mut self, order: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.z_index_stack.push(order);
        let result = f(self);
        self.window.z_index_stack.pop();
        result
    }

    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut V, &mut ViewContext<V>) + Send + 'static)
    where
        V: Any + Send,
    {
        let view = self.view().upgrade().unwrap();
        self.window_cx.on_next_frame(move |cx| view.update(cx, f));
    }

    pub fn observe<V2, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(&mut V, E, &mut ViewContext<'_, '_, V>) + Send + 'static,
    ) -> Subscription
    where
        V2: 'static,
        V: Any + Send,
        E: Entity<V2>,
    {
        let view = self.view();
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();
        let window_handle = self.window.handle;
        self.app.observers.insert(
            entity_id,
            Box::new(move |cx| {
                cx.update_window(window_handle.id, |cx| {
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

    pub fn subscribe<V2, E>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(&mut V, E, &V2::Event, &mut ViewContext<'_, '_, V>) + Send + 'static,
    ) -> Subscription
    where
        V2: EventEmitter,
        E: Entity<V2>,
    {
        let view = self.view();
        let entity_id = entity.entity_id();
        let handle = entity.downgrade();
        let window_handle = self.window.handle;
        self.app.event_listeners.insert(
            entity_id,
            Box::new(move |event, cx| {
                cx.update_window(window_handle.id, |cx| {
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
        )
    }

    pub fn on_release(
        &mut self,
        mut on_release: impl FnMut(&mut V, &mut WindowContext) + Send + 'static,
    ) -> Subscription {
        let window_handle = self.window.handle;
        self.app.release_listeners.insert(
            self.view.model.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                // todo!("are we okay with silently swallowing the error?")
                let _ = cx.update_window(window_handle.id, |cx| on_release(this, cx));
            }),
        )
    }

    pub fn observe_release<V2, E>(
        &mut self,
        entity: &E,
        mut on_release: impl FnMut(&mut V, &mut V2, &mut ViewContext<'_, '_, V>) + Send + 'static,
    ) -> Subscription
    where
        V: Any + Send,
        V2: 'static,
        E: Entity<V2>,
    {
        let view = self.view();
        let entity_id = entity.entity_id();
        let window_handle = self.window.handle;
        self.app.release_listeners.insert(
            entity_id,
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                let _ = cx.update_window(window_handle.id, |cx| {
                    view.update(cx, |this, cx| on_release(this, entity, cx))
                });
            }),
        )
    }

    pub fn notify(&mut self) {
        self.window_cx.notify();
        self.window_cx.app.push_effect(Effect::Notify {
            emitter: self.view.model.entity_id,
        });
    }

    pub fn on_focus_changed(
        &mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + Send + 'static,
    ) {
        let handle = self.view();
        self.window.focus_listeners.push(Box::new(move |event, cx| {
            handle
                .update(cx, |view, cx| listener(view, event, cx))
                .log_err();
        }));
    }

    pub fn with_key_listeners<R>(
        &mut self,
        key_listeners: impl IntoIterator<Item = (TypeId, KeyListener<V>)>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let old_stack_len = self.window.key_dispatch_stack.len();
        if !self.window.freeze_key_dispatch_stack {
            for (event_type, listener) in key_listeners {
                let handle = self.view();
                let listener = Box::new(
                    move |event: &dyn Any,
                          context_stack: &[&DispatchContext],
                          phase: DispatchPhase,
                          cx: &mut WindowContext<'_, '_>| {
                        handle
                            .update(cx, |view, cx| {
                                listener(view, event, context_stack, phase, cx)
                            })
                            .log_err()
                            .flatten()
                    },
                );
                self.window
                    .key_dispatch_stack
                    .push(KeyDispatchStackFrame::Listener {
                        event_type,
                        listener,
                    });
            }
        }

        let result = f(self);

        if !self.window.freeze_key_dispatch_stack {
            self.window.key_dispatch_stack.truncate(old_stack_len);
        }

        result
    }

    pub fn with_key_dispatch_context<R>(
        &mut self,
        context: DispatchContext,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if context.is_empty() {
            return f(self);
        }

        if !self.window.freeze_key_dispatch_stack {
            self.window
                .key_dispatch_stack
                .push(KeyDispatchStackFrame::Context(context));
        }

        let result = f(self);

        if !self.window.freeze_key_dispatch_stack {
            self.window.key_dispatch_stack.pop();
        }

        result
    }

    pub fn with_focus<R>(
        &mut self,
        focus_handle: FocusHandle,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(parent_focus_id) = self.window.focus_stack.last().copied() {
            self.window
                .focus_parents_by_child
                .insert(focus_handle.id, parent_focus_id);
        }
        self.window.focus_stack.push(focus_handle.id);

        if Some(focus_handle.id) == self.window.focus {
            self.window.freeze_key_dispatch_stack = true;
        }

        let result = f(self);

        self.window.focus_stack.pop();
        result
    }

    pub fn run_on_main<R>(
        &mut self,
        view: &mut V,
        f: impl FnOnce(&mut V, &mut MainThread<ViewContext<'_, '_, V>>) -> R + Send + 'static,
    ) -> Task<Result<R>>
    where
        R: Send + 'static,
    {
        if self.executor.is_main_thread() {
            let cx = unsafe { mem::transmute::<&mut Self, &mut MainThread<Self>>(self) };
            Task::ready(Ok(f(view, cx)))
        } else {
            let view = self.view().upgrade().unwrap();
            self.window_cx.run_on_main(move |cx| view.update(cx, f))
        }
    }

    pub fn spawn<Fut, R>(
        &mut self,
        f: impl FnOnce(WeakView<V>, AsyncWindowContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
        Fut: Future<Output = R> + Send + 'static,
    {
        let view = self.view();
        self.window_cx.spawn(move |_, cx| {
            let result = f(view, cx);
            async move { result.await }
        })
    }

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static + Send,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

    pub fn observe_global<G: 'static>(
        &mut self,
        f: impl Fn(&mut V, &mut ViewContext<'_, '_, V>) + Send + 'static,
    ) -> Subscription {
        let window_id = self.window.handle.id;
        let handle = self.view();
        self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                cx.update_window(window_id, |cx| {
                    handle.update(cx, |view, cx| f(view, cx)).is_ok()
                })
                .unwrap_or(false)
            }),
        )
    }

    pub fn on_mouse_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&mut V, &Event, DispatchPhase, &mut ViewContext<V>) + Send + 'static,
    ) {
        let handle = self.view().upgrade().unwrap();
        self.window_cx.on_mouse_event(move |event, phase, cx| {
            handle.update(cx, |view, cx| {
                handler(view, event, phase, cx);
            })
        });
    }
}

impl<'a, 'w, V> ViewContext<'a, 'w, V>
where
    V: EventEmitter,
    V::Event: Any + Send,
{
    pub fn emit(&mut self, event: V::Event) {
        let emitter = self.view.model.entity_id;
        self.app.push_effect(Effect::Emit {
            emitter,
            event: Box::new(event),
        });
    }
}

impl<'a, 'w, V> Context for ViewContext<'a, 'w, V> {
    type ModelContext<'b, U> = ModelContext<'b, U>;
    type Result<U> = U;

    fn build_model<T>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Model<T>
    where
        T: 'static + Send,
    {
        self.window_cx.build_model(build_model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> R {
        self.window_cx.update_model(model, update)
    }
}

impl<V: 'static> VisualContext for ViewContext<'_, '_, V> {
    type ViewContext<'a, 'w, V2> = ViewContext<'a, 'w, V2>;

    fn build_view<W: 'static + Send>(
        &mut self,
        build_view: impl FnOnce(&mut Self::ViewContext<'_, '_, W>) -> W,
    ) -> Self::Result<View<W>> {
        self.window_cx.build_view(build_view)
    }

    fn update_view<V2: 'static, R>(
        &mut self,
        view: &View<V2>,
        update: impl FnOnce(&mut V2, &mut Self::ViewContext<'_, '_, V2>) -> R,
    ) -> Self::Result<R> {
        self.window_cx.update_view(view, update)
    }
}

impl<'a, 'w, V> std::ops::Deref for ViewContext<'a, 'w, V> {
    type Target = WindowContext<'a, 'w>;

    fn deref(&self) -> &Self::Target {
        &self.window_cx
    }
}

impl<'a, 'w, V> std::ops::DerefMut for ViewContext<'a, 'w, V> {
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

#[derive(PartialEq, Eq)]
pub struct WindowHandle<V> {
    id: WindowId,
    state_type: PhantomData<V>,
}

impl<S> Copy for WindowHandle<S> {}

impl<S> Clone for WindowHandle<S> {
    fn clone(&self) -> Self {
        WindowHandle {
            id: self.id,
            state_type: PhantomData,
        }
    }
}

impl<S> WindowHandle<S> {
    pub fn new(id: WindowId) -> Self {
        WindowHandle {
            id,
            state_type: PhantomData,
        }
    }
}

impl<S: 'static> Into<AnyWindowHandle> for WindowHandle<S> {
    fn into(self) -> AnyWindowHandle {
        AnyWindowHandle {
            id: self.id,
            state_type: TypeId::of::<S>(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct AnyWindowHandle {
    pub(crate) id: WindowId,
    state_type: TypeId,
}

impl AnyWindowHandle {
    pub fn window_id(&self) -> WindowId {
        self.id
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
    Number(usize),
    Name(SharedString),
    FocusHandle(FocusId),
}

impl From<EntityId> for ElementId {
    fn from(id: EntityId) -> Self {
        ElementId::View(id)
    }
}

impl From<usize> for ElementId {
    fn from(id: usize) -> Self {
        ElementId::Number(id)
    }
}

impl From<i32> for ElementId {
    fn from(id: i32) -> Self {
        Self::Number(id as usize)
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
