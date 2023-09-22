use crate::{
    px, renderer::Renderer, taffy::LayoutId, AppContext, AvailableSpace, Bounds, Context, EntityId,
    Handle, MainThreadOnly, Pixels, Platform, PlatformWindow, Point, Reference, Size, Style,
    TaffyLayoutEngine, TextStyle, TextStyleRefinement, WindowOptions,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use refineable::Refineable;
use std::{
    any::{Any, TypeId},
    future::Future,
    marker::PhantomData,
    sync::Arc,
};

pub struct AnyWindow {}

pub struct Window {
    handle: AnyWindowHandle,
    platform_window: MainThreadOnly<Box<dyn PlatformWindow>>,
    renderer: Renderer,
    rem_size: Pixels,
    layout_engine: TaffyLayoutEngine,
    text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) root_view: Option<Box<dyn Any + Send>>,
    mouse_position: Point<Pixels>,
}

impl Window {
    pub fn new(
        handle: AnyWindowHandle,
        options: WindowOptions,
        platform: &dyn Platform,
    ) -> impl Future<Output = Window> + 'static {
        let platform_window = platform.open_window(handle, options);
        let renderer = Renderer::new(&platform_window.as_ref());
        let mouse_position = platform_window.mouse_position();
        let platform_window = MainThreadOnly::new(Arc::new(platform_window), platform.dispatcher());

        async move {
            let renderer = renderer.await;
            Window {
                handle,
                platform_window,
                renderer,
                rem_size: px(16.),
                layout_engine: TaffyLayoutEngine::new(),
                text_style_stack: Vec::new(),
                root_view: None,
                mouse_position,
            }
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
        window_id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        if window_id == self.window.handle.id {
            Ok(update(self))
        } else {
            self.app.update_window(window_id, update)
        }
    }
}

impl Context for WindowContext<'_, '_> {
    type EntityContext<'a, 'w, T: Send + 'static> = ViewContext<'a, 'w, T>;

    fn entity<T: Send + 'static>(
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

    fn update_entity<T: Send + 'static, R>(
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

impl<'a, 'w, T: 'static> ViewContext<'a, 'w, T> {
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
        let unit_entity_id = self.unit_entity_id;
        let mut cx = ViewContext::mutable(
            &mut *self.window_cx.app,
            &mut *self.window_cx.window,
            unit_entity_id,
        );
        f(&mut cx)
    }
}

impl<'a, 'w, T: 'static> Context for ViewContext<'a, 'w, T> {
    type EntityContext<'b, 'c, U: Send + 'static> = ViewContext<'b, 'c, U>;

    fn entity<T2: Send + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T2>) -> T2,
    ) -> Handle<T2> {
        self.window_cx.entity(build_entity)
    }

    fn update_entity<U: Send + 'static, R>(
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
    id: WindowId,
    state_type: TypeId,
}

#[derive(Clone)]
pub struct Layout {
    pub order: u32,
    pub bounds: Bounds<Pixels>,
}
