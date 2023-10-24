use crate::{
    px, size, Action, AnyBox, AnyDrag, AnyView, AppContext, AsyncWindowContext, AvailableSpace,
    Bounds, BoxShadow, Context, Corners, DevicePixels, DispatchContext, DisplayId, DroppedFiles,
    Edges, Effect, Element, EntityId, EventEmitter, FileDropEvent, FocusEvent, FontId,
    GlobalElementId, GlyphId, Handle, Hsla, ImageData, InputEvent, IsZero, KeyListener, KeyMatch,
    KeyMatcher, Keystroke, LayoutId, MainThread, MainThreadOnly, Modifiers, MonochromeSprite,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Path, Pixels, PlatformAtlas,
    PlatformWindow, Point, PolychromeSprite, Quad, Reference, RenderGlyphParams, RenderImageParams,
    RenderSvgParams, ScaledPixels, SceneBuilder, Shadow, SharedString, Size, Style, Subscription,
    TaffyLayoutEngine, Task, Underline, UnderlineStyle, WeakHandle, WindowOptions,
    SUBPIXEL_VARIANTS,
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

#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq, Clone, Default)]
pub struct StackingOrder(pub(crate) SmallVec<[u32; 16]>);

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub enum DispatchPhase {
    /// After the capture phase comes the bubble phase, in which event handlers are
    /// invoked front to back. This is the phase you'll usually want to use for event handlers.
    #[default]
    Bubble,
    /// During the initial capture phase, event handlers are invoked back to front. This phase
    /// is used for special purposes such as clearing the "pressed" state for click events. If
    /// you stop event propagation during this phase, you need to know what you're doing. Handlers
    /// outside of the immediate region may rely on detecting non-local events during this phase.
    Capture,
}

type AnyListener = Arc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext) + Send + Sync + 'static>;
type AnyKeyListener = Arc<
    dyn Fn(
            &dyn Any,
            &[&DispatchContext],
            DispatchPhase,
            &mut WindowContext,
        ) -> Option<Box<dyn Action>>
        + Send
        + Sync
        + 'static,
>;
type AnyFocusListener = Arc<dyn Fn(&FocusEvent, &mut WindowContext) + Send + Sync + 'static>;

slotmap::new_key_type! { pub struct FocusId; }

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

    pub fn is_focused(&self, cx: &WindowContext) -> bool {
        cx.window.focus == Some(self.id)
    }

    pub fn contains_focused(&self, cx: &WindowContext) -> bool {
        cx.focused()
            .map_or(false, |focused| self.contains(&focused, cx))
    }

    pub fn within_focused(&self, cx: &WindowContext) -> bool {
        let focused = cx.focused();
        focused.map_or(false, |focused| focused.contains(self, cx))
    }

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

pub struct Window {
    handle: AnyWindowHandle,
    platform_window: MainThreadOnly<Box<dyn PlatformWindow>>,
    display_id: DisplayId,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    rem_size: Pixels,
    content_size: Size<Pixels>,
    layout_engine: TaffyLayoutEngine,
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

enum KeyDispatchStackFrame {
    Listener {
        event_type: TypeId,
        listener: AnyKeyListener,
    },
    Context(DispatchContext),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ContentMask<P: Clone + Default + Debug> {
    pub bounds: Bounds<P>,
}

impl ContentMask<Pixels> {
    pub fn scale(&self, factor: f32) -> ContentMask<ScaledPixels> {
        ContentMask {
            bounds: self.bounds.scale(factor),
        }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let bounds = self.bounds.intersect(&other.bounds);
        ContentMask { bounds }
    }
}

pub struct WindowContext<'a, 'w> {
    app: Reference<'a, AppContext>,
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

    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window.handle
    }

    pub fn notify(&mut self) {
        self.window.dirty = true;
    }

    pub fn focus_handle(&mut self) -> FocusHandle {
        FocusHandle::new(&self.window.focus_handles)
    }

    pub fn focused(&self) -> Option<FocusHandle> {
        self.window
            .focus
            .and_then(|id| FocusHandle::for_id(id, &self.window.focus_handles))
    }

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

    pub fn to_async(&self) -> AsyncWindowContext {
        AsyncWindowContext::new(self.app.to_async(), self.window.handle)
    }

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

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

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

    pub fn layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels> {
        let mut bounds = self
            .window
            .layout_engine
            .layout_bounds(layout_id)
            .map(Into::into);
        bounds.origin += self.element_offset();
        bounds
    }

    pub fn scale_factor(&self) -> f32 {
        self.window.scale_factor
    }

    pub fn rem_size(&self) -> Pixels {
        self.window.rem_size
    }

    pub fn line_height(&self) -> Pixels {
        let rem_size = self.rem_size();
        let text_style = self.text_style();
        text_style
            .line_height
            .to_pixels(text_style.font_size.into(), rem_size)
    }

    pub fn prevent_default(&mut self) {
        self.window.default_prevented = true;
    }

    pub fn default_prevented(&self) -> bool {
        self.window.default_prevented
    }

    pub fn on_mouse_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&Event, DispatchPhase, &mut WindowContext) + Send + Sync + 'static,
    ) {
        let order = self.window.z_index_stack.clone();
        self.window
            .mouse_listeners
            .entry(TypeId::of::<Event>())
            .or_default()
            .push((
                order,
                Arc::new(move |event: &dyn Any, phase, cx| {
                    handler(event.downcast_ref().unwrap(), phase, cx)
                }),
            ))
    }

    pub fn mouse_position(&self) -> Point<Pixels> {
        self.window.mouse_position
    }

    pub fn stack<R>(&mut self, order: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.z_index_stack.push(order);
        let result = f(self);
        self.window.z_index_stack.pop();
        result
    }

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

    pub(crate) fn draw(&mut self) {
        let unit_entity = self.unit_entity.clone();
        self.update_entity(&unit_entity, |view, cx| {
            cx.start_frame();

            let mut root_view = cx.window.root_view.take().unwrap();

            cx.stack(0, |cx| {
                let available_space = cx.window.content_size.map(Into::into);
                draw_any_view(&mut root_view, available_space, cx);
            });

            if let Some(mut active_drag) = cx.active_drag.take() {
                cx.stack(1, |cx| {
                    let offset = cx.mouse_position() - active_drag.cursor_offset;
                    cx.with_element_offset(Some(offset), |cx| {
                        let available_space =
                            size(AvailableSpace::MinContent, AvailableSpace::MinContent);
                        if let Some(drag_handle_view) = &mut active_drag.drag_handle_view {
                            draw_any_view(drag_handle_view, available_space, cx);
                        }
                        cx.active_drag = Some(active_drag);
                    });
                });
            }

            cx.window.root_view = Some(root_view);
            let scene = cx.window.scene_builder.build();

            cx.run_on_main(view, |_, cx| {
                cx.window
                    .platform_window
                    .borrow_on_main_thread()
                    .draw(scene);
                cx.window.dirty = false;
            })
            .detach();
        });

        fn draw_any_view(
            view: &mut AnyView,
            available_space: Size<AvailableSpace>,
            cx: &mut ViewContext<()>,
        ) {
            cx.with_optional_element_state(view.id(), |element_state, cx| {
                let mut element_state = view.initialize(&mut (), element_state, cx);
                let layout_id = view.layout(&mut (), &mut element_state, cx);
                cx.window
                    .layout_engine
                    .compute_layout(layout_id, available_space);
                let bounds = cx.window.layout_engine.layout_bounds(layout_id);
                view.paint(bounds, &mut (), &mut element_state, cx);
                ((), element_state)
            });
        }
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

    fn dispatch_event(&mut self, event: InputEvent) -> bool {
        let event = match event {
            InputEvent::MouseMove(mouse_move) => {
                self.window.mouse_position = mouse_move.position;
                InputEvent::MouseMove(mouse_move)
            }
            InputEvent::FileDrop(file_drop) => match file_drop {
                FileDropEvent::Entered { position, files } => {
                    self.active_drag.get_or_insert_with(|| AnyDrag {
                        drag_handle_view: None,
                        cursor_offset: position,
                        state: Box::new(files),
                        state_type: TypeId::of::<DroppedFiles>(),
                    });
                    InputEvent::MouseDown(MouseDownEvent {
                        position,
                        button: MouseButton::Left,
                        click_count: 1,
                        modifiers: Modifiers::default(),
                    })
                }
                FileDropEvent::Pending { position } => InputEvent::MouseMove(MouseMoveEvent {
                    position,
                    pressed_button: Some(MouseButton::Left),
                    modifiers: Modifiers::default(),
                }),
                FileDropEvent::Submit { position } => InputEvent::MouseUp(MouseUpEvent {
                    button: MouseButton::Left,
                    position,
                    modifiers: Modifiers::default(),
                    click_count: 1,
                }),
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

    pub fn observe_global<G: 'static>(
        &mut self,
        f: impl Fn(&mut WindowContext<'_, '_>) + Send + Sync + 'static,
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
    type EntityContext<'a, 'w, T> = ViewContext<'a, 'w, T>;
    type Result<T> = T;

    fn entity<T>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T>
    where
        T: Any + Send + Sync,
    {
        let slot = self.app.entities.reserve();
        let entity = build_entity(&mut ViewContext::mutable(
            &mut *self.app,
            &mut self.window,
            slot.downgrade(),
        ));
        self.entities.insert(slot, entity)
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self.entities.lease(handle);
        let result = update(
            &mut *entity,
            &mut ViewContext::mutable(&mut *self.app, &mut *self.window, handle.downgrade()),
        );
        self.entities.end_lease(entity);
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

    fn element_offset(&self) -> Point<Pixels> {
        self.window()
            .element_offset_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    fn with_element_state<S, R>(
        &mut self,
        id: ElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: Any + Send + Sync,
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

    fn with_optional_element_state<S, R>(
        &mut self,
        element_id: Option<ElementId>,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: Any + Send + Sync,
    {
        if let Some(element_id) = element_id {
            self.with_element_state(element_id, f)
        } else {
            f(None, self).0
        }
    }

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
    view_state: WeakHandle<V>,
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
    fn mutable(app: &'a mut AppContext, window: &'w mut Window, view_state: WeakHandle<V>) -> Self {
        Self {
            window_cx: WindowContext::mutable(app, window),
            view_state,
        }
    }

    pub fn handle(&self) -> WeakHandle<V> {
        self.view_state.clone()
    }

    pub fn stack<R>(&mut self, order: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.z_index_stack.push(order);
        let result = f(self);
        self.window.z_index_stack.pop();
        result
    }

    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut V, &mut ViewContext<V>) + Send + 'static)
    where
        V: Any + Send + Sync,
    {
        let entity = self.handle();
        self.window_cx.on_next_frame(move |cx| {
            entity.update(cx, f).ok();
        });
    }

    pub fn observe<E>(
        &mut self,
        handle: &Handle<E>,
        mut on_notify: impl FnMut(&mut V, Handle<E>, &mut ViewContext<'_, '_, V>)
            + Send
            + Sync
            + 'static,
    ) -> Subscription
    where
        E: 'static,
        V: Any + Send + Sync,
    {
        let this = self.handle();
        let handle = handle.downgrade();
        let window_handle = self.window.handle;
        self.app.observers.insert(
            handle.entity_id,
            Box::new(move |cx| {
                cx.update_window(window_handle.id, |cx| {
                    if let Some(handle) = handle.upgrade() {
                        this.update(cx, |this, cx| on_notify(this, handle, cx))
                            .is_ok()
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
            }),
        )
    }

    pub fn subscribe<E: EventEmitter>(
        &mut self,
        handle: &Handle<E>,
        mut on_event: impl FnMut(&mut V, Handle<E>, &E::Event, &mut ViewContext<'_, '_, V>)
            + Send
            + Sync
            + 'static,
    ) -> Subscription {
        let this = self.handle();
        let handle = handle.downgrade();
        let window_handle = self.window.handle;
        self.app.event_listeners.insert(
            handle.entity_id,
            Box::new(move |event, cx| {
                cx.update_window(window_handle.id, |cx| {
                    if let Some(handle) = handle.upgrade() {
                        let event = event.downcast_ref().expect("invalid event type");
                        this.update(cx, |this, cx| on_event(this, handle, event, cx))
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
        mut on_release: impl FnMut(&mut V, &mut WindowContext) + Send + Sync + 'static,
    ) -> Subscription {
        let window_handle = self.window.handle;
        self.app.release_listeners.insert(
            self.view_state.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                // todo!("are we okay with silently swallowing the error?")
                let _ = cx.update_window(window_handle.id, |cx| on_release(this, cx));
            }),
        )
    }

    pub fn observe_release<T: 'static>(
        &mut self,
        handle: &Handle<T>,
        mut on_release: impl FnMut(&mut V, &mut T, &mut ViewContext<'_, '_, V>) + Send + Sync + 'static,
    ) -> Subscription
    where
        V: Any + Send + Sync,
    {
        let this = self.handle();
        let window_handle = self.window.handle;
        self.app.release_listeners.insert(
            handle.entity_id,
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                // todo!("are we okay with silently swallowing the error?")
                let _ = cx.update_window(window_handle.id, |cx| {
                    this.update(cx, |this, cx| on_release(this, entity, cx))
                });
            }),
        )
    }

    pub fn notify(&mut self) {
        self.window_cx.notify();
        self.window_cx.app.push_effect(Effect::Notify {
            emitter: self.view_state.entity_id,
        });
    }

    pub fn on_focus_changed(
        &mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + Send + Sync + 'static,
    ) {
        let handle = self.handle();
        self.window.focus_listeners.push(Arc::new(move |event, cx| {
            handle
                .update(cx, |view, cx| listener(view, event, cx))
                .log_err();
        }));
    }

    pub fn with_key_listeners<R>(
        &mut self,
        key_listeners: &[(TypeId, KeyListener<V>)],
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if !self.window.freeze_key_dispatch_stack {
            for (event_type, listener) in key_listeners.iter().cloned() {
                let handle = self.handle();
                let listener = Arc::new(
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
            let prev_len = self.window.key_dispatch_stack.len() - key_listeners.len();
            self.window.key_dispatch_stack.truncate(prev_len);
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
            let handle = self.handle().upgrade().unwrap();
            self.window_cx.run_on_main(move |cx| handle.update(cx, f))
        }
    }

    pub fn spawn<Fut, R>(
        &mut self,
        f: impl FnOnce(WeakHandle<V>, AsyncWindowContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
        Fut: Future<Output = R> + Send + 'static,
    {
        let handle = self.handle();
        self.window_cx.spawn(move |_, cx| {
            let result = f(handle, cx);
            async move { result.await }
        })
    }

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static + Send + Sync,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

    pub fn observe_global<G: 'static>(
        &mut self,
        f: impl Fn(&mut V, &mut ViewContext<'_, '_, V>) + Send + Sync + 'static,
    ) -> Subscription {
        let window_id = self.window.handle.id;
        let handle = self.handle();
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
        handler: impl Fn(&mut V, &Event, DispatchPhase, &mut ViewContext<V>) + Send + Sync + 'static,
    ) {
        let handle = self.handle().upgrade().unwrap();
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
    V::Event: Any + Send + Sync,
{
    pub fn emit(&mut self, event: V::Event) {
        let emitter = self.view_state.entity_id;
        self.app.push_effect(Effect::Emit {
            emitter,
            event: Box::new(event),
        });
    }
}

impl<'a, 'w, V> Context for ViewContext<'a, 'w, V> {
    type EntityContext<'b, 'c, U> = ViewContext<'b, 'c, U>;
    type Result<U> = U;

    fn entity<T>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T>
    where
        T: 'static + Send + Sync,
    {
        self.window_cx.entity(build_entity)
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        self.window_cx.update_entity(handle, update)
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

#[cfg(any(test, feature = "test"))]
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
