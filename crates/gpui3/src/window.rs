use crate::{
    image_cache::RenderImageParams, px, AnyView, AppContext, AsyncWindowContext, AvailableSpace,
    BorrowAppContext, Bounds, Context, Corners, DevicePixels, Effect, Element, EntityId, FontId,
    GlyphId, Handle, Hsla, ImageData, IsZero, LayerId, LayoutId, MainThread, MainThreadOnly,
    MonochromeSprite, Pixels, PlatformAtlas, PlatformWindow, Point, PolychromeSprite, Reference,
    RenderGlyphParams, RenderSvgParams, ScaledPixels, Scene, SharedString, Size, Style,
    TaffyLayoutEngine, Task, WeakHandle, WindowOptions, SUBPIXEL_VARIANTS,
};
use anyhow::Result;
use smallvec::SmallVec;
use std::{any::TypeId, borrow::Cow, future::Future, marker::PhantomData, mem, sync::Arc};
use util::ResultExt;

pub struct AnyWindow {}

pub struct Window {
    handle: AnyWindowHandle,
    platform_window: MainThreadOnly<Box<dyn PlatformWindow>>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    rem_size: Pixels,
    content_size: Size<Pixels>,
    layout_engine: TaffyLayoutEngine,
    pub(crate) root_view: Option<AnyView<()>>,
    mouse_position: Point<Pixels>,
    current_layer_id: LayerId,
    content_mask_stack: Vec<ContentMask>,
    pub(crate) scene: Scene,
    pub(crate) dirty: bool,
}

impl Window {
    pub fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        cx: &mut MainThread<AppContext>,
    ) -> Self {
        let platform_window = cx.platform().open_window(handle, options);
        let sprite_atlas = platform_window.sprite_atlas();
        let mouse_position = platform_window.mouse_position();
        let content_size = platform_window.content_size();
        let scale_factor = platform_window.scale_factor();
        platform_window.on_resize(Box::new({
            let handle = handle;
            let cx = cx.to_async();
            move |content_size, scale_factor| {
                cx.update_window(handle, |cx| {
                    cx.window.scene = Scene::new(scale_factor);
                    cx.window.content_size = content_size;
                    cx.window.dirty = true;
                })
                .log_err();
            }
        }));

        let platform_window = MainThreadOnly::new(Arc::new(platform_window), cx.executor.clone());

        Window {
            handle,
            platform_window,
            sprite_atlas,
            rem_size: px(16.),
            content_size,
            layout_engine: TaffyLayoutEngine::new(),
            root_view: None,
            mouse_position,
            current_layer_id: SmallVec::new(),
            content_mask_stack: Vec::new(),
            scene: Scene::new(scale_factor),
            dirty: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContentMask {
    pub bounds: Bounds<Pixels>,
}

impl ContentMask {
    pub fn scale(&self, factor: f32) -> ScaledContentMask {
        ScaledContentMask {
            bounds: self.bounds.scale(factor),
        }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let bounds = self.bounds.intersect(&other.bounds);
        ContentMask { bounds }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ScaledContentMask {
    bounds: Bounds<ScaledPixels>,
}

pub struct WindowContext<'a, 'w> {
    app: Reference<'a, AppContext>,
    window: Reference<'w, Window>,
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
    ) -> Result<LayoutId> {
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
    ) -> Result<LayoutId> {
        self.window
            .layout_engine
            .request_measured_layout(style, rem_size, measure)
    }

    pub fn layout(&mut self, layout_id: LayoutId) -> Result<Layout> {
        Ok(self
            .window
            .layout_engine
            .layout(layout_id)
            .map(Into::into)?)
    }

    pub fn scale_factor(&self) -> f32 {
        self.window.scene.scale_factor
    }

    pub fn rem_size(&self) -> Pixels {
        self.window.rem_size
    }

    pub fn mouse_position(&self) -> Point<Pixels> {
        self.window.mouse_position
    }

    pub fn scene(&mut self) -> &mut Scene {
        &mut self.window.scene
    }

    pub fn stack<R>(&mut self, order: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.window.current_layer_id.push(order);
        let result = f(self);
        self.window.current_layer_id.pop();
        result
    }

    pub fn current_layer_id(&self) -> LayerId {
        self.window.current_layer_id.clone()
    }

    pub fn paint_glyph(
        &mut self,
        origin: Point<Pixels>,
        order: u32,
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
            let layer_id = self.current_layer_id();
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

            self.window.scene.insert(
                layer_id,
                MonochromeSprite {
                    order,
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
        order: u32,
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
            let layer_id = self.current_layer_id();
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

            self.window.scene.insert(
                layer_id,
                PolychromeSprite {
                    order,
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
        order: u32,
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

        let layer_id = self.current_layer_id();
        let tile =
            self.window
                .sprite_atlas
                .get_or_insert_with(&params.clone().into(), &mut || {
                    let bytes = self.svg_renderer.render(&params)?;
                    Ok((params.size, Cow::Owned(bytes)))
                })?;
        let content_mask = self.content_mask().scale(scale_factor);

        self.window.scene.insert(
            layer_id,
            MonochromeSprite {
                order,
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
        order: u32,
        data: Arc<ImageData>,
        grayscale: bool,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let params = RenderImageParams { image_id: data.id };

        let layer_id = self.current_layer_id();
        let tile = self
            .window
            .sprite_atlas
            .get_or_insert_with(&params.clone().into(), &mut || {
                Ok((data.size(), Cow::Borrowed(data.as_bytes())))
            })?;
        let content_mask = self.content_mask().scale(scale_factor);
        let corner_radii = corner_radii.scale(scale_factor);

        self.window.scene.insert(
            layer_id,
            PolychromeSprite {
                order,
                bounds,
                content_mask,
                corner_radii,
                tile,
                grayscale,
            },
        );

        Ok(())
    }

    pub(crate) fn draw(&mut self) -> Result<()> {
        let unit_entity = self.unit_entity.clone();
        self.update_entity(&unit_entity, |view, cx| {
            let mut root_view = cx.window.root_view.take().unwrap();
            let (root_layout_id, mut frame_state) = root_view.layout(&mut (), cx)?;
            let available_space = cx.window.content_size.map(Into::into);

            cx.window
                .layout_engine
                .compute_layout(root_layout_id, available_space)?;
            let layout = cx.window.layout_engine.layout(root_layout_id)?;

            root_view.paint(layout, &mut (), &mut frame_state, cx)?;
            cx.window.root_view = Some(root_view);
            let scene = cx.window.scene.take();

            cx.run_on_main(view, |_, cx| {
                cx.window
                    .platform_window
                    .borrow_on_main_thread()
                    .draw(scene);
                cx.window.dirty = false;
            })
            .detach();

            Ok(())
        })
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

    fn with_content_mask<R>(&mut self, mask: ContentMask, f: impl FnOnce(&mut Self) -> R) -> R {
        let mask = mask.intersect(&self.content_mask());
        self.window_mut().content_mask_stack.push(mask);
        let result = f(self);
        self.window_mut().content_mask_stack.pop();
        result
    }

    fn content_mask(&self) -> ContentMask {
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

    fn with_text_style<F, R>(&mut self, style: crate::TextStyleRefinement, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.window_cx.app.push_text_style(style);
        let result = f(self);
        self.window_cx.app.pop_text_style();
        result
    }

    fn with_state<T: Send + Sync + 'static, F, R>(&mut self, state: T, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.window_cx.app.push_state(state);
        let result = f(self);
        self.window_cx.app.pop_state::<T>();
        result
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

    pub fn observe<E: Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_notify: impl Fn(&mut S, Handle<E>, &mut ViewContext<'_, '_, S>) + Send + Sync + 'static,
    ) {
        let this = self.handle();
        let handle = handle.downgrade();
        let window_handle = self.window.handle;
        self.app
            .observers
            .entry(handle.id)
            .or_default()
            .push(Arc::new(move |cx| {
                cx.update_window(window_handle.id, |cx| {
                    if let Some(handle) = handle.upgrade(cx) {
                        this.update(cx, |this, cx| on_notify(this, handle, cx))
                            .is_ok()
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
            }));
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

    pub(crate) fn erase_state<R>(&mut self, f: impl FnOnce(&mut ViewContext<()>) -> R) -> R {
        let entity_id = self.unit_entity.id;
        let mut cx = ViewContext::mutable(
            &mut *self.window_cx.app,
            &mut *self.window_cx.window,
            entity_id,
        );
        f(&mut cx)
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

#[derive(Clone)]
pub struct Layout {
    pub order: u32,
    pub bounds: Bounds<Pixels>,
}
