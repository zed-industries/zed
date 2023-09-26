use crate::{
    px, AnyView, AppContext, AvailableSpace, Bounds, Context, Effect, Element, EntityId, Handle,
    LayoutId, MainThreadOnly, Pixels, Platform, PlatformWindow, Point, Reference, Scene, Size,
    Style, TaffyLayoutEngine, TextStyle, TextStyleRefinement, WeakHandle, WindowOptions,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use refineable::Refineable;
use std::{any::TypeId, future, marker::PhantomData, sync::Arc};
use util::ResultExt;

pub struct AnyWindow {}

pub struct Window {
    handle: AnyWindowHandle,
    platform_window: MainThreadOnly<Box<dyn PlatformWindow>>,
    rem_size: Pixels,
    content_size: Size<Pixels>,
    layout_engine: TaffyLayoutEngine,
    text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) root_view: Option<AnyView<()>>,
    mouse_position: Point<Pixels>,
    pub(crate) scene: Scene,
    pub(crate) dirty: bool,
}

impl Window {
    pub fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        platform: &dyn Platform,
        cx: &mut AppContext,
    ) -> Self {
        let platform_window = platform.open_window(handle, options);
        let mouse_position = platform_window.mouse_position();
        let content_size = platform_window.content_size();
        let scale_factor = platform_window.scale_factor();
        platform_window.on_resize(Box::new({
            let handle = handle;
            let cx = cx.to_async();
            move |content_size, scale_factor| {
                dbg!("!!!!!!!!!!!!");
                cx.update_window(handle, |cx| {
                    dbg!("!!!!!!!!");
                    cx.window.scene = Scene::new(scale_factor);
                    cx.window.content_size = content_size;
                    cx.window.dirty = true;
                })
                .log_err();
            }
        }));

        let platform_window = MainThreadOnly::new(Arc::new(platform_window), platform.dispatcher());

        Window {
            handle,
            platform_window,
            rem_size: px(16.),
            content_size,
            layout_engine: TaffyLayoutEngine::new(),
            text_style_stack: Vec::new(),
            root_view: None,
            mouse_position,
            scene: Scene::new(scale_factor),
            dirty: true,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct WindowContext<'a, 'b> {
    #[deref]
    #[deref_mut]
    app: Reference<'a, AppContext>,
    window: Reference<'b, Window>,
}

impl<'a, 'w> WindowContext<'a, 'w> {
    pub(crate) fn mutable(app: &'a mut AppContext, window: &'w mut Window) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
        }
    }

    pub(crate) fn immutable(app: &'a AppContext, window: &'w Window) -> Self {
        Self {
            app: Reference::Immutable(app),
            window: Reference::Immutable(window),
        }
    }

    pub(crate) fn draw(&mut self) -> Result<()> {
        let unit_entity = self.unit_entity.clone();
        self.update_entity(&unit_entity, |_, cx| {
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
            dbg!(&scene);
            let _ = cx.window.platform_window.read(|platform_window| {
                platform_window.draw(scene);
                future::ready(())
            });

            Ok(())
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

    pub fn rem_size(&self) -> Pixels {
        self.window.rem_size
    }

    pub fn push_text_style(&mut self, text_style: TextStyleRefinement) {
        self.window.text_style_stack.push(text_style);
    }

    pub fn pop_text_style(&mut self) {
        self.window.text_style_stack.pop();
    }

    pub fn text_style(&self) -> TextStyle {
        let mut style = TextStyle::default();
        for refinement in &self.window.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    pub fn mouse_position(&self) -> Point<Pixels> {
        self.window.mouse_position
    }

    fn update_window<R>(
        &mut self,
        window_handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        if window_handle == self.window.handle {
            Ok(update(self))
        } else {
            self.app.update_window(window_handle.id, update)
        }
    }
}

impl Context for WindowContext<'_, '_> {
    type EntityContext<'a, 'w, T: Send + Sync + 'static> = ViewContext<'a, 'w, T>;
    type Result<T> = T;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let id = self.entities.insert(None);
        let entity = Box::new(build_entity(&mut ViewContext::mutable(
            &mut *self.app,
            &mut self.window,
            id,
        )));
        self.entities.get_mut(id).unwrap().replace(entity);

        Handle {
            id,
            entity_type: PhantomData,
        }
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self
            .app
            .entities
            .get_mut(handle.id)
            .unwrap()
            .take()
            .unwrap()
            .downcast::<T>()
            .unwrap();

        let result = update(
            &mut *entity,
            &mut ViewContext::mutable(&mut *self.app, &mut *self.window, handle.id),
        );

        self.app
            .entities
            .get_mut(handle.id)
            .unwrap()
            .replace(entity);

        result
    }
}

#[derive(Deref, DerefMut)]
pub struct ViewContext<'a, 'w, T> {
    #[deref]
    #[deref_mut]
    window_cx: WindowContext<'a, 'w>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, 'w, T: Send + Sync + 'static> ViewContext<'a, 'w, T> {
    // fn update<R>(&mut self, update: impl FnOnce(&mut T, &mut Self) -> R) -> R {

    //     self.window_cx.update_entity(handle, update)

    //     let mut entity = self.window_cx.app.entities.remove(&self.entity_id).unwrap();
    //     let result = update(entity.downcast_mut::<T>().unwrap(), self);
    //     self.window_cx
    //         .app
    //         .entities
    //         .insert(self.entity_id, Box::new(entity));
    //     result
    // }

    fn mutable(app: &'a mut AppContext, window: &'w mut Window, entity_id: EntityId) -> Self {
        Self {
            window_cx: WindowContext::mutable(app, window),
            entity_id,
            entity_type: PhantomData,
        }
    }

    fn immutable(app: &'a AppContext, window: &'w Window, entity_id: EntityId) -> Self {
        Self {
            window_cx: WindowContext::immutable(app, window),
            entity_id,
            entity_type: PhantomData,
        }
    }

    pub fn erase_state<R>(&mut self, f: impl FnOnce(&mut ViewContext<()>) -> R) -> R {
        let entity_id = self.unit_entity.id;
        let mut cx = ViewContext::mutable(
            &mut *self.window_cx.app,
            &mut *self.window_cx.window,
            entity_id,
        );
        f(&mut cx)
    }

    pub fn handle(&self) -> WeakHandle<T> {
        WeakHandle {
            id: self.entity_id,
            entity_type: PhantomData,
        }
    }

    pub fn observe<E: Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_notify: impl Fn(&mut T, Handle<E>, &mut ViewContext<'_, '_, T>) + Send + Sync + 'static,
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
        let entity_id = self.entity_id;
        self.app
            .pending_effects
            .push_back(Effect::Notify(entity_id));
        self.window.dirty = true;
    }
}

impl<'a, 'w, T: 'static> Context for ViewContext<'a, 'w, T> {
    type EntityContext<'b, 'c, U: Send + Sync + 'static> = ViewContext<'b, 'c, U>;
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
