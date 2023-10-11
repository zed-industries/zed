use crate::{
    px, size, AnyBox, AnyView, AppContext, AsyncWindowContext, AvailableSpace, BorrowAppContext,
    Bounds, BoxShadow, Context, Corners, DevicePixels, DisplayId, Edges, Effect, Element, EntityId,
    Event, EventEmitter, FontId, GlobalElementId, GlyphId, Handle, Hsla, ImageData, IsZero,
    LayoutId, MainThread, MainThreadOnly, MonochromeSprite, MouseMoveEvent, Path, Pixels, Platform,
    PlatformAtlas, PlatformWindow, Point, PolychromeSprite, Quad, Reference, RenderGlyphParams,
    RenderImageParams, RenderSvgParams, ScaledPixels, SceneBuilder, Shadow, SharedString, Size,
    Style, Subscription, TaffyLayoutEngine, Task, Underline, UnderlineStyle, WeakHandle,
    WindowOptions, SUBPIXEL_VARIANTS,
};
use anyhow::Result;
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    mem,
    sync::Arc,
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

type MouseEventHandler =
    Arc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext) + Send + Sync + 'static>;

pub struct Window {
    handle: AnyWindowHandle,
    platform_window: MainThreadOnly<Box<dyn PlatformWindow>>,
    pub(crate) display_id: DisplayId, // todo!("make private again?")
    sprite_atlas: Arc<dyn PlatformAtlas>,
    rem_size: Pixels,
    content_size: Size<Pixels>,
    layout_engine: TaffyLayoutEngine,
    pub(crate) root_view: Option<AnyView<()>>,
    pub(crate) element_id_stack: GlobalElementId,
    prev_element_states: HashMap<GlobalElementId, AnyBox>,
    element_states: HashMap<GlobalElementId, AnyBox>,
    z_index_stack: StackingOrder,
    content_mask_stack: Vec<ContentMask<Pixels>>,
    mouse_event_handlers: HashMap<TypeId, Vec<(StackingOrder, MouseEventHandler)>>,
    propagate_event: bool,
    mouse_position: Point<Pixels>,
    scale_factor: f32,
    pub(crate) scene_builder: SceneBuilder,
    pub(crate) dirty: bool,
}

impl Window {
    pub fn new(
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

        platform_window.on_event({
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
            prev_element_states: HashMap::default(),
            element_states: HashMap::default(),
            z_index_stack: StackingOrder(SmallVec::new()),
            content_mask_stack: Vec::new(),
            mouse_event_handlers: HashMap::default(),
            propagate_event: true,
            mouse_position,
            scale_factor,
            scene_builder: SceneBuilder::new(),
            dirty: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct ContentMask<P: Clone + Debug> {
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
    pub(crate) fn mutable(app: &'a mut AppContext, window: &'w mut Window) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
        }
    }

    pub fn notify(&mut self) {
        self.window.dirty = true;
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

    pub fn request_layout(
        &mut self,
        style: Style,
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
        self.window
            .layout_engine
            .layout_bounds(layout_id)
            .map(Into::into)
    }

    pub fn scale_factor(&self) -> f32 {
        self.window.scale_factor
    }

    pub fn rem_size(&self) -> Pixels {
        self.window.rem_size
    }

    pub fn stop_propagation(&mut self) {
        self.window.propagate_event = false;
    }

    pub fn on_mouse_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&Event, DispatchPhase, &mut WindowContext) + Send + Sync + 'static,
    ) {
        let order = self.window.z_index_stack.clone();
        self.window
            .mouse_event_handlers
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

            if let Some(element_id) = root_view.element_id() {
                cx.with_element_state(element_id, |element_state, cx| {
                    let element_state = draw_with_element_state(&mut root_view, element_state, cx);
                    ((), element_state)
                });
            } else {
                draw_with_element_state(&mut root_view, None, cx);
            };

            cx.window.root_view = Some(root_view);
            let scene = cx.window.scene_builder.build();
            cx.end_frame();

            cx.run_on_main(view, |_, cx| {
                cx.window
                    .platform_window
                    .borrow_on_main_thread()
                    .draw(scene);
                cx.window.dirty = false;
            })
            .detach();
        });

        fn draw_with_element_state(
            root_view: &mut AnyView<()>,
            element_state: Option<AnyBox>,
            cx: &mut ViewContext<()>,
        ) -> AnyBox {
            let (layout_id, mut element_state) = root_view.layout(&mut (), element_state, cx);
            let available_space = cx.window.content_size.map(Into::into);
            cx.window
                .layout_engine
                .compute_layout(layout_id, available_space);
            let bounds = cx.window.layout_engine.layout_bounds(layout_id);
            root_view.paint(bounds, &mut (), &mut element_state, cx);
            element_state
        }
    }

    fn start_frame(&mut self) {
        // Make the current element states the previous, and then clear the current.
        // The empty element states map will be populated for any element states we
        // reference during the upcoming frame.
        let window = &mut *self.window;
        mem::swap(&mut window.element_states, &mut window.prev_element_states);
        self.window.element_states.clear();

        // Clear mouse event listeners, because elements add new element listeners
        // when the upcoming frame is painted.
        self.window
            .mouse_event_handlers
            .values_mut()
            .for_each(Vec::clear);
    }

    fn end_frame(&mut self) {
        self.text_system().end_frame();
    }

    fn dispatch_event(&mut self, event: Event) -> bool {
        if let Some(any_mouse_event) = event.mouse_event() {
            if let Some(MouseMoveEvent { position, .. }) = any_mouse_event.downcast_ref() {
                self.window.mouse_position = *position;
            }

            if let Some(mut handlers) = self
                .window
                .mouse_event_handlers
                .remove(&any_mouse_event.type_id())
            {
                // Because handlers may add other handlers, we sort every time.
                handlers.sort_by(|(a, _), (b, _)| a.cmp(b));

                // Handlers may set this to false by calling `stop_propagation`
                self.window.propagate_event = true;

                // Capture phase, events bubble from back to front. Handlers for this phase are used for
                // special purposes, such as detecting events outside of a given Bounds.
                for (_, handler) in &handlers {
                    handler(any_mouse_event, DispatchPhase::Capture, self);
                    if !self.window.propagate_event {
                        break;
                    }
                }

                // Bubble phase, where most normal handlers do their work.
                if self.window.propagate_event {
                    for (_, handler) in handlers.iter().rev() {
                        handler(any_mouse_event, DispatchPhase::Bubble, self);
                        if !self.window.propagate_event {
                            break;
                        }
                    }
                }

                // Just in case any handlers added new handlers, which is weird, but possible.
                handlers.extend(
                    self.window
                        .mouse_event_handlers
                        .get_mut(&any_mouse_event.type_id())
                        .into_iter()
                        .flat_map(|handlers| handlers.drain(..)),
                );
                self.window
                    .mouse_event_handlers
                    .insert(any_mouse_event.type_id(), handlers);
            }
        }

        true
    }
}

impl<'a, 'w> MainThread<WindowContext<'a, 'w>> {
    fn platform(&self) -> &dyn Platform {
        self.platform.borrow_on_main_thread()
    }
}

impl Context for WindowContext<'_, '_> {
    type EntityContext<'a, 'w, T: 'static + Send + Sync> = ViewContext<'a, 'w, T>;
    type Result<T> = T;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let slot = self.app.entities.reserve();
        let entity = build_entity(&mut ViewContext::mutable(
            &mut *self.app,
            &mut self.window,
            slot.id,
        ));
        self.entities.insert(slot, entity)
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self.entities.lease(handle);
        let result = update(
            &mut *entity,
            &mut ViewContext::mutable(&mut *self.app, &mut *self.window, handle.id),
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

impl BorrowAppContext for WindowContext<'_, '_> {
    fn app_mut(&mut self) -> &mut AppContext {
        &mut *self.app
    }
}

pub trait BorrowWindow: BorrowAppContext {
    fn window(&self) -> &Window;
    fn window_mut(&mut self) -> &mut Window;

    fn with_element_id<R>(
        &mut self,
        id: impl Into<ElementId>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.window_mut().element_id_stack.push(id.into());
        let result = f(self);
        self.window_mut().element_id_stack.pop();
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

    fn with_element_state<S: 'static + Send + Sync, R>(
        &mut self,
        id: ElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R {
        self.with_element_id(id, |cx| {
            let global_id = cx.window_mut().element_id_stack.clone();
            if let Some(any) = cx
                .window_mut()
                .element_states
                .remove(&global_id)
                .or_else(|| cx.window_mut().prev_element_states.remove(&global_id))
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

impl BorrowWindow for WindowContext<'_, '_> {
    fn window(&self) -> &Window {
        &*self.window
    }

    fn window_mut(&mut self) -> &mut Window {
        &mut *self.window
    }
}

pub struct ViewContext<'a, 'w, S> {
    window_cx: WindowContext<'a, 'w>,
    entity_type: PhantomData<S>,
    entity_id: EntityId,
}

impl<S> BorrowAppContext for ViewContext<'_, '_, S> {
    fn app_mut(&mut self) -> &mut AppContext {
        &mut *self.window_cx.app
    }
}

impl<S> BorrowWindow for ViewContext<'_, '_, S> {
    fn window(&self) -> &Window {
        &self.window_cx.window
    }

    fn window_mut(&mut self) -> &mut Window {
        &mut *self.window_cx.window
    }
}

impl<'a, 'w, S: Send + Sync + 'static> ViewContext<'a, 'w, S> {
    fn mutable(app: &'a mut AppContext, window: &'w mut Window, entity_id: EntityId) -> Self {
        Self {
            window_cx: WindowContext::mutable(app, window),
            entity_id,
            entity_type: PhantomData,
        }
    }

    pub fn handle(&self) -> WeakHandle<S> {
        self.entities.weak_handle(self.entity_id)
    }

    pub fn stack<R>(&mut self, order: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.z_index_stack.push(order);
        let result = f(self);
        self.window.z_index_stack.pop();
        result
    }

    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut S, &mut ViewContext<S>) + Send + 'static) {
        let entity = self.handle();
        self.window_cx.on_next_frame(move |cx| {
            entity.update(cx, f).ok();
        });
    }

    pub fn observe<E: Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_notify: impl Fn(&mut S, Handle<E>, &mut ViewContext<'_, '_, S>) + Send + Sync + 'static,
    ) -> Subscription {
        let this = self.handle();
        let handle = handle.downgrade();
        let window_handle = self.window.handle;
        self.app.observers.insert(
            handle.id,
            Box::new(move |cx| {
                cx.update_window(window_handle.id, |cx| {
                    if let Some(handle) = handle.upgrade(cx) {
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

    pub fn subscribe<E: EventEmitter + Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_event: impl Fn(&mut S, Handle<E>, &E::Event, &mut ViewContext<'_, '_, S>)
            + Send
            + Sync
            + 'static,
    ) -> Subscription {
        let this = self.handle();
        let handle = handle.downgrade();
        let window_handle = self.window.handle;
        self.app.event_handlers.insert(
            handle.id,
            Box::new(move |event, cx| {
                cx.update_window(window_handle.id, |cx| {
                    if let Some(handle) = handle.upgrade(cx) {
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

    pub fn notify(&mut self) {
        self.window_cx.notify();
        self.window_cx
            .app
            .pending_effects
            .push_back(Effect::Notify(self.entity_id));
    }

    pub fn run_on_main<R>(
        &mut self,
        view: &mut S,
        f: impl FnOnce(&mut S, &mut MainThread<ViewContext<'_, '_, S>>) -> R + Send + 'static,
    ) -> Task<Result<R>>
    where
        R: Send + 'static,
    {
        if self.executor.is_main_thread() {
            let cx = unsafe { mem::transmute::<&mut Self, &mut MainThread<Self>>(self) };
            Task::ready(Ok(f(view, cx)))
        } else {
            let handle = self.handle().upgrade(self).unwrap();
            self.window_cx.run_on_main(move |cx| handle.update(cx, f))
        }
    }

    pub fn spawn<Fut, R>(
        &mut self,
        f: impl FnOnce(WeakHandle<S>, AsyncWindowContext) -> Fut + Send + 'static,
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

    pub fn on_mouse_event<Event: 'static>(
        &mut self,
        handler: impl Fn(&mut S, &Event, DispatchPhase, &mut ViewContext<S>) + Send + Sync + 'static,
    ) {
        let handle = self.handle().upgrade(self).unwrap();
        self.window_cx.on_mouse_event(move |event, phase, cx| {
            handle.update(cx, |view, cx| {
                handler(view, event, phase, cx);
            })
        });
    }
}

impl<'a, 'w, S: EventEmitter + Send + Sync + 'static> ViewContext<'a, 'w, S> {
    pub fn emit(&mut self, event: S::Event) {
        let entity_id = self.entity_id;
        self.app.pending_effects.push_back(Effect::Emit {
            entity_id,
            event: Box::new(event),
        });
    }
}

impl<'a, 'w, S> Context for ViewContext<'a, 'w, S> {
    type EntityContext<'b, 'c, U: 'static + Send + Sync> = ViewContext<'b, 'c, U>;
    type Result<U> = U;

    fn entity<T2: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T2>) -> T2,
    ) -> Handle<T2> {
        self.window_cx.entity(build_entity)
    }

    fn update_entity<U: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<U>,
        update: impl FnOnce(&mut U, &mut Self::EntityContext<'_, '_, U>) -> R,
    ) -> R {
        self.window_cx.update_entity(handle, update)
    }
}

impl<'a, 'w, S: 'static> std::ops::Deref for ViewContext<'a, 'w, S> {
    type Target = WindowContext<'a, 'w>;

    fn deref(&self) -> &Self::Target {
        &self.window_cx
    }
}

impl<'a, 'w, S: 'static> std::ops::DerefMut for ViewContext<'a, 'w, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window_cx
    }
}

// #[derive(Clone, Copy, Eq, PartialEq, Hash)]
slotmap::new_key_type! { pub struct WindowId; }

#[derive(PartialEq, Eq)]
pub struct WindowHandle<S> {
    id: WindowId,
    state_type: PhantomData<S>,
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
