use anyhow::{anyhow, Result};
use gpui2::{Layout, LayoutId, Reference, Vector2F};
use std::{any::Any, collections::HashMap, marker::PhantomData, rc::Rc};

pub trait Context {
    type EntityContext<'a, 'b, T>;

    fn add_entity<F, T>(&mut self, build_entity: F) -> Handle<T>
    where
        F: FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T;

    fn update_entity<F, T, R>(&mut self, handle: &Handle<T>, update: F) -> R
    where
        F: FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
        T: 'static;

    fn update_window<F, R>(&mut self, window_id: WindowId, update: F) -> Result<R>
    where
        F: FnOnce(&mut WindowContext) -> R;
}

pub struct AppContext {
    entity_count: usize,
    entities: HashMap<EntityId, Box<dyn Any>>,
    windows: HashMap<WindowId, Window>,
}

impl Context for AppContext {
    type EntityContext<'a, 'b, T> = ModelContext<'a, T>;

    fn add_entity<F, T>(&mut self, build: F) -> Handle<T>
    where
        F: FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    {
        let id = EntityId::new(&mut self.entity_count);
        let entity = build(&mut ModelContext::mutable(self, id));
        self.entities.insert(id, Box::new(entity));
        Handle {
            id,
            entity_type: PhantomData,
        }
    }

    fn update_entity<F, T, R>(&mut self, handle: &Handle<T>, update: F) -> R
    where
        F: FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
        T: 'static,
    {
        let mut entity = self.entities.remove(&handle.id).unwrap();
        let result = update(
            entity.downcast_mut().unwrap(),
            &mut ModelContext::mutable(self, handle.id),
        );
        self.entities.insert(handle.id, entity);
        result
    }

    fn update_window<F, R>(&mut self, window_id: WindowId, update: F) -> Result<R>
    where
        F: FnOnce(&mut WindowContext<'_, '_>) -> R,
    {
        let mut window = self
            .windows
            .remove(&window_id)
            .ok_or_else(|| anyhow!("window closed"))?;
        let result = update(&mut WindowContext::mutable(self, &mut window));
        self.windows.insert(window_id, window);
        Ok(result)
    }
}

impl AppContext {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn open_window<E, S>(&self, root_element: E, state: S) -> WindowHandle<S> {
        unimplemented!()
    }

    pub fn add_entity<T, F>(&mut self, entity: F) -> Handle<T>
    where
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        let id = EntityId::new(&mut self.entity_count);

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

        let mut cx = WindowContext::mutable(self, &mut window);
        let result = update(&mut cx);
        self.windows.insert(window_id, window);
        Ok(result)
    }
}

pub struct ModelContext<'a, T> {
    app: Reference<'a, AppContext>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, T> ModelContext<'a, T> {
    fn mutable(app: &mut AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn immutable(app: &AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Immutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }
}

pub struct Window {
    id: WindowId,
}

pub struct WindowContext<'a, 'b> {
    app: Reference<'a, AppContext>,
    window: Reference<'b, Window>,
}

impl<'a, 'b> WindowContext<'a, 'b> {
    fn mutable(app: &mut AppContext, window: &mut Window) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
        }
    }

    fn app_context(&mut self) -> &mut AppContext {
        &mut *self.app
    }
}

impl<'a, 'b> Context for WindowContext<'a, 'b> {
    type EntityContext<'c, 'd, T> = ViewContext<'c, 'd, T>;

    fn add_entity<F, T>(&mut self, build_entity: F) -> Handle<T>
    where
        F: FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    {
        let id = EntityId::new(&mut self.app_context().entity_count);
        let mut cx = ViewContext::mutable(&mut self.app_context(), &mut self.window, id);
        let entity = build_entity(&mut cx);
        self.app.entities.insert(id, Box::new(entity));
        Handle {
            id,
            entity_type: PhantomData,
        }
    }

    fn update_entity<F, T, R>(&mut self, handle: &Handle<T>, update: F) -> R
    where
        F: FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
        T: 'static,
    {
        let mut entity = self.app.entities.remove(&handle.id).unwrap();
        let result = update(
            entity.downcast_mut().unwrap(),
            &mut ViewContext::mutable(&mut self.app, &mut self.window, handle.id),
        );
        self.app.entities.insert(handle.id, entity);
        result
    }

    fn update_window<F, R>(&mut self, window_id: WindowId, update: F) -> Result<R>
    where
        F: FnOnce(&mut WindowContext) -> R,
    {
        if window_id == self.window.id {
            Ok(update(self))
        } else {
            self.app.update_window(window_id, update)
        }
    }
}

pub struct ViewContext<'a, 'b, T> {
    app: Reference<'a, AppContext>,
    window: Reference<'b, Window>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, 'b, V> ViewContext<'a, 'b, V> {
    fn mutable(app: &'a mut AppContext, window: &'b mut Window, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn immutable(app: &'a AppContext, window: &'b Window, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Immutable(app),
            window: Reference::Immutable(window),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn window_context(&self) -> WindowContext {
        WindowContext {
            app: Reference::Immutable(&*self.app),
            window: Reference::Immutable(&*self.window),
        }
    }

    fn window_context_mut(&mut self) -> WindowContext {
        WindowContext {
            app: Reference::Mutable(&mut *self.app),
            window: Reference::Mutable(&mut *self.window),
        }
    }
}

impl<'a, 'b, V> Context for ViewContext<'a, 'b, V> {
    type EntityContext<'c, 'd, T> = ViewContext<'c, 'd, T>;

    fn add_entity<F, T>(&mut self, build_entity: F) -> Handle<T>
    where
        F: FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    {
        self.window_context_mut().add_entity(build_entity)
    }

    fn update_entity<F, T, R>(&mut self, handle: &Handle<T>, update: F) -> R
    where
        F: FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
        T: 'static,
    {
        self.window_context_mut().update_entity(handle, update)
    }

    fn update_window<F, R>(&mut self, window_id: WindowId, update: F) -> Result<R>
    where
        F: FnOnce(&mut WindowContext<'_, '_>) -> R,
    {
        self.window_context_mut().update_window(window_id, update)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct WindowId(usize);

pub struct WindowHandle<S>(PhantomData<S>);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct EntityId(usize);

impl EntityId {
    fn new(entity_count: &mut usize) -> EntityId {
        let id = *entity_count;
        *entity_count += 1;
        Self(id)
    }
}

pub struct Handle<T> {
    id: EntityId,
    entity_type: PhantomData<T>,
}

impl<T: 'static> Handle<T> {
    fn update<'a, C: Context, R>(
        &self,
        cx: &'a mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        cx.update_entity(self, update)
    }
}

pub trait Element: 'static {
    type State;
    type FrameState;

    fn add_layout_node(
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
    fn add_layout_node(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> Result<LayoutId>;
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
    fn add_layout_node(
        &mut self,
        state: &mut E::State,
        cx: &mut ViewContext<E::State>,
    ) -> Result<LayoutId> {
        let (layout_id, frame_state) = self.element.add_layout_node(state, cx)?;
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
        self.0.add_layout_node(state, cx)
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
    fn into_any_element(self) -> AnyElement<S>;
}

impl<E: Element> IntoAnyElement<E::State> for E {
    fn into_any_element(self) -> AnyElement<E::State> {
        AnyElement(Box::new(RenderedElement::new(self)))
    }
}

impl<S> IntoAnyElement<S> for AnyElement<S> {
    fn into_any_element(self) -> AnyElement<S> {
        self
    }
}

pub struct View<S> {
    render: Rc<dyn Fn(&mut WindowContext) -> AnyElement<S>>,
}

impl<S> View<S> {
    fn render(&self, cx: &mut WindowContext) -> AnyElement<S> {
        (self.render)(cx)
    }
}

pub fn view<ParentState, ChildState: 'static>(
    state: Handle<ChildState>,
    render: impl 'static + Fn(&mut ChildState, &mut ViewContext<ChildState>) -> AnyElement<ParentState>,
) -> View<ParentState> {
    View {
        render: Rc::new(move |cx| state.update(cx, |state, cx| render(state, cx))),
    }
}

pub struct Div<S>(PhantomData<S>);

impl<S: 'static> Element for Div<S> {
    type State = S;
    type FrameState = ();

    fn add_layout_node(
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
    left_panel: View<Self>,
}

fn workspace(
    state: &mut Workspace,
    cx: &mut ViewContext<Workspace>,
) -> impl Element<State = Workspace> {
    div().child(state.left_panel.render(&mut cx))
}

pub struct CollabPanel {
    filter_editor: Handle<Editor>,
}

impl CollabPanel {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            filter_editor: cx.add_entity(|cx| Editor::new(cx)),
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
        let collab_panel = cx.add_entity(|cx| CollabPanel::new(cx));

        // let
        // let mut workspace = Workspace {
        //     left_panel: view(),
        // }

        // cx.open_window(workspace::Workspace, state)
    }
}
