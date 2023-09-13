use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use gpui2::{Layout, LayoutId, Reference, Vector2F};
use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

pub struct AppContext {
    entity_count: usize,
    entities: HashMap<EntityId, Box<dyn Any>>,
    window_count: usize,
    windows: HashMap<WindowId, Window>,
}

impl AppContext {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn open_window<S>(
        &mut self,
        build_root_view: impl FnOnce(&mut WindowContext) -> View<S>,
    ) -> WindowHandle<S> {
        unimplemented!()
    }

    fn add_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut ModelContext<T>) -> T,
    ) -> Handle<T> {
        let id = EntityId::new(&mut self.entity_count);
        let entity = build_entity(&mut ModelContext::mutable(self, id));
        self.entities.insert(id, Box::new(entity));
        Handle {
            id,
            entity_type: PhantomData,
        }
    }

    fn update_window<R>(
        &mut self,
        window_id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        let mut window = self
            .windows
            .remove(&window_id)
            .ok_or_else(|| anyhow!("window not found"))?;
        let result = update(&mut WindowContext::mutable(self, &mut window));
        self.windows.insert(window_id, window);
        Ok(result)
    }
}

pub struct ModelContext<'a, T> {
    app: Reference<'a, AppContext>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    fn mutable(app: &'a mut AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn immutable(app: &'a AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Immutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn update<R>(&mut self, update: impl FnOnce(&mut T, &mut Self) -> R) -> R {
        let mut entity = self.app.entities.remove(&self.entity_id).unwrap();
        let result = update(entity.downcast_mut::<T>().unwrap(), self);
        self.app.entities.insert(self.entity_id, Box::new(entity));
        result
    }
}

pub struct Window {
    id: WindowId,
}

#[derive(Deref, DerefMut)]
pub struct WindowContext<'a, 'b> {
    #[deref]
    #[deref_mut]
    app: Reference<'a, AppContext>,
    window: Reference<'b, Window>,
}

impl<'a, 'w> WindowContext<'a, 'w> {
    fn mutable(app: &'a mut AppContext, window: &'w mut Window) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
        }
    }

    fn immutable(app: &'a AppContext, window: &'w Window) -> Self {
        Self {
            app: Reference::Immutable(app),
            window: Reference::Immutable(window),
        }
    }

    fn app_context(&mut self) -> &mut AppContext {
        &mut *self.app
    }
}

impl<'a, 'w> WindowContext<'a, 'w> {
    fn entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut ViewContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let id = EntityId::new(&mut self.app_context().entity_count);
        let entity = build_entity(&mut ViewContext::mutable(
            &mut *self.app,
            &mut *self.window,
            id,
        ));
        self.app.entities.insert(id, Box::new(entity));
        Handle {
            id,
            entity_type: PhantomData,
        }
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut ViewContext<T>) -> R,
    ) -> R {
        let mut entity = self.app.entities.remove(&handle.id).unwrap();
        let result = update(
            entity.downcast_mut().unwrap(),
            &mut ViewContext::mutable(&mut *self.app, &mut *self.window, handle.id),
        );
        self.app.entities.insert(handle.id, entity);
        result
    }

    fn update_window<R>(
        &mut self,
        window_id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        if window_id == self.window.id {
            Ok(update(self))
        } else {
            self.app.update_window(window_id, update)
        }
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
    fn update<R>(&mut self, update: impl FnOnce(&mut T, &mut Self) -> R) -> R {
        let mut entity = self.window_cx.app.entities.remove(&self.entity_id).unwrap();
        let result = update(entity.downcast_mut::<T>().unwrap(), self);
        self.window_cx
            .app
            .entities
            .insert(self.entity_id, Box::new(entity));
        result
    }

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
}

impl<'a, 'w, T: 'static> Context for ViewContext<'a, 'w, T> {
    type EntityContext<'b, 'c, U: 'static> = ViewContext<'b, 'c, U>;

    fn update_entity<U: 'static, R>(
        &mut self,
        handle: &Handle<U>,
        update: impl FnOnce(&mut U, &mut Self::EntityContext<'_, '_, U>) -> R,
    ) -> R {
        ViewContext::mutable(
            &mut *self.window_cx.app,
            &mut *self.window_cx.window,
            handle.id,
        )
        .update(update)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct WindowId(usize);

impl WindowId {
    fn new(window_count: &mut usize) -> Self {
        let id = *window_count;
        *window_count += 1;
        Self(id)
    }
}

pub struct WindowHandle<S> {
    id: WindowId,
    state_type: PhantomData<S>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct EntityId(usize);

impl EntityId {
    fn new(entity_count: &mut usize) -> EntityId {
        let id = *entity_count;
        *entity_count += 1;
        Self(id)
    }
}

trait Context {
    type EntityContext<'a, 'w, T: 'static>;

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R;
}

impl Context for AppContext {
    type EntityContext<'a, 'w, T: 'static> = ModelContext<'a, T>;

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self
            .entities
            .remove(&handle.id)
            .unwrap()
            .downcast::<T>()
            .unwrap();
        let result = update(&mut *entity, &mut ModelContext::mutable(self, handle.id));
        self.entities.insert(handle.id, Box::new(entity));
        result
    }
}

impl Context for WindowContext<'_, '_> {
    type EntityContext<'a, 'w, T: 'static> = ViewContext<'a, 'w, T>;

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self
            .app
            .entities
            .remove(&handle.id)
            .unwrap()
            .downcast::<T>()
            .unwrap();
        let result = update(
            &mut *entity,
            &mut ViewContext::mutable(&mut *self.app, &mut *self.window, handle.id),
        );
        self.entities.insert(handle.id, Box::new(entity));
        result
    }
}

pub struct Handle<T> {
    id: EntityId,
    entity_type: PhantomData<T>,
}

impl<T: 'static> Handle<T> {
    fn update<C: Context, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        cx.update_entity(self, update)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            entity_type: PhantomData,
        }
    }
}

pub trait Element: 'static {
    type State;
    type FrameState;

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)>;

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()>;
}

pub trait ParentElement<S> {
    fn child(self, child: impl IntoAnyElement<S>) -> Self;
}

trait ElementObject<S> {
    fn layout(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> Result<LayoutId>;
    fn paint(
        &mut self,
        parent_origin: Vector2F,
        state: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<()>;
}

struct RenderedElement<E: Element> {
    element: E,
    phase: ElementRenderPhase<E::FrameState>,
}

enum ElementRenderPhase<S> {
    Rendered,
    LayoutNodeAdded { layout_id: LayoutId, frame_state: S },
    Painted { layout: Layout, frame_state: S },
}

impl<E: Element> RenderedElement<E> {
    fn new(element: E) -> Self {
        RenderedElement {
            element,
            phase: ElementRenderPhase::Rendered,
        }
    }
}

impl<E: Element> ElementObject<E::State> for RenderedElement<E> {
    fn layout(&mut self, state: &mut E::State, cx: &mut ViewContext<E::State>) -> Result<LayoutId> {
        let (layout_id, frame_state) = self.element.layout(state, cx)?;
        self.phase = ElementRenderPhase::LayoutNodeAdded {
            layout_id,
            frame_state,
        };
        Ok(layout_id)
    }

    fn paint(
        &mut self,
        parent_origin: Vector2F,
        state: &mut E::State,
        cx: &mut ViewContext<E::State>,
    ) -> Result<()> {
        todo!()
    }
}

pub struct AnyElement<S>(Box<dyn ElementObject<S>>);

impl<S> AnyElement<S> {
    pub fn layout(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> Result<LayoutId> {
        self.0.layout(state, cx)
    }

    pub fn paint(
        &mut self,
        parent_origin: Vector2F,
        state: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<()> {
        self.0.paint(parent_origin, state, cx)
    }
}

pub trait IntoAnyElement<S> {
    fn into_any(self) -> AnyElement<S>;
}

impl<E: Element> IntoAnyElement<E::State> for E {
    fn into_any(self) -> AnyElement<E::State> {
        AnyElement(Box::new(RenderedElement::new(self)))
    }
}

impl<S> IntoAnyElement<S> for AnyElement<S> {
    fn into_any(self) -> AnyElement<S> {
        self
    }
}

#[derive(Clone)]
pub struct View<S> {
    state: Handle<S>,
    render: Rc<dyn Fn(&mut S, &mut ViewContext<S>) -> AnyElement<S>>,
}

pub fn view<S: 'static, E: Element<State = S>>(
    state: Handle<S>,
    render: impl 'static + Fn(&mut S, &mut ViewContext<S>) -> E,
) -> View<S> {
    View {
        state,
        render: Rc::new(move |state, cx| render(state, cx).into_any()),
    }
}

impl<S: 'static> View<S> {
    pub fn into_any<ParentState>(self) -> AnyView<ParentState> {
        AnyView {
            view: Rc::new(RefCell::new(self)),
            parent_state_type: PhantomData,
        }
    }
}

impl<S: 'static> Element for View<S> {
    type State = ();
    type FrameState = AnyElement<S>;

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx)?;
            Ok((layout_id, element))
        })
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        element: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.state.update(cx, |state, cx| {
            element.paint(layout.bounds.origin(), state, cx)
        })
    }
}

trait ViewObject {
    fn layout(&mut self, cx: &mut WindowContext) -> Result<(LayoutId, Box<dyn Any>)>;
    fn paint(
        &mut self,
        layout: Layout,
        element: &mut dyn Any,
        cx: &mut WindowContext,
    ) -> Result<()>;
}

impl<S: 'static> ViewObject for View<S> {
    fn layout(&mut self, cx: &mut WindowContext) -> Result<(LayoutId, Box<dyn Any>)> {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx)?;
            let element = Box::new(element) as Box<dyn Any>;
            Ok((layout_id, element))
        })
    }

    fn paint(
        &mut self,
        layout: Layout,
        element: &mut dyn Any,
        cx: &mut WindowContext,
    ) -> Result<()> {
        self.state.update(cx, |state, cx| {
            element.downcast_mut::<AnyElement<S>>().unwrap().paint(
                layout.bounds.origin(),
                state,
                cx,
            )
        })
    }
}

pub struct AnyView<S> {
    view: Rc<RefCell<dyn ViewObject>>,
    parent_state_type: PhantomData<S>,
}

impl<S: 'static> Element for AnyView<S> {
    type State = S;
    type FrameState = Box<dyn Any>;

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.view.borrow_mut().layout(cx)
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        element: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.view.borrow_mut().paint(layout, element, cx)
    }
}

impl<S> Clone for AnyView<S> {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            parent_state_type: PhantomData,
        }
    }
}

pub struct Div<S>(PhantomData<S>);

impl<S: 'static> Element for Div<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        todo!()
    }

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        todo!()
    }
}

impl<S> ParentElement<S> for Div<S> {
    fn child(self, child: impl IntoAnyElement<S>) -> Self {
        todo!()
    }
}

pub fn div<S>() -> Div<S> {
    todo!()
}

pub struct Workspace {
    left_panel: AnyView<Self>,
}

fn workspace(cx: &mut WindowContext) -> View<Workspace> {
    let workspace = cx.entity(|cx| Workspace {
        left_panel: collab_panel(cx).into_any(),
    });
    view(workspace, |workspace, cx| {
        div().child(workspace.left_panel.clone())
    })
}

pub struct CollabPanel {
    filter_editor: Handle<Editor>,
}

fn collab_panel(cx: &mut WindowContext) -> View<CollabPanel> {
    let panel = cx.entity(|cx| CollabPanel::new(cx));
    view(panel, |panel, cx| div())
}

impl CollabPanel {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            filter_editor: cx.entity(|cx| Editor::new(cx)),
        }
    }
}

struct EditorElement {
    input: bool,
}

impl EditorElement {
    pub fn input(mut self) -> Self {
        self.input = true;
        self
    }
}

struct Editor {}

impl Editor {
    pub fn new(_: &mut ViewContext<Self>) -> Self {
        Editor {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cx = AppContext::new();
        cx.open_window(|cx| workspace(cx));
    }
}
