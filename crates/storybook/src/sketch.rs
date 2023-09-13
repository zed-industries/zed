use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use gpui2::{Layout, LayoutId, Reference, Vector2F};
use std::{any::Any, collections::HashMap, marker::PhantomData, rc::Rc};

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

    pub fn open_window<E, S>(
        &self,
        root_view: impl Fn(ViewContext<S>) -> View<S>,
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

impl<'a, T> ModelContext<'a, T> {
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
}

pub struct Window {
    id: WindowId,
}

pub struct WindowContext<'a, 'b> {
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

    fn app_context(&mut self) -> &mut AppContext {
        &mut *self.app
    }
}

impl<'app, 'win> WindowContext<'app, 'win> {
    fn add_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut ViewContext<'_, 'app, 'win, T>) -> T,
    ) -> Handle<T> {
        let id = EntityId::new(&mut self.app_context().entity_count);
        let mut cx = ViewContext::mutable(self, id);
        let entity = build_entity(&mut cx);
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
            &mut ViewContext::mutable(self, handle.id),
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
pub struct ViewContext<'parent, 'app, 'win, T> {
    #[deref]
    #[deref_mut]
    window_cx: Reference<'parent, WindowContext<'app, 'win>>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'cx, 'app: 'cx, 'win: 'cx, V> ViewContext<'cx, 'app, 'win, V> {
    fn mutable(window_cx: &'cx mut WindowContext<'app, 'win>, entity_id: EntityId) -> Self {
        Self {
            window_cx: Reference::Mutable(window_cx),
            entity_id,
            entity_type: PhantomData,
        }
    }

    fn immutable(window_cx: &'cx WindowContext<'app, 'win>, entity_id: EntityId) -> Self {
        Self {
            window_cx: Reference::Immutable(window_cx),
            entity_id,
            entity_type: PhantomData,
        }
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

pub struct Handle<T> {
    id: EntityId,
    entity_type: PhantomData<T>,
}

trait Context {
    type EntityContext<'cx, 'app, 'win, T: 'static>;

    fn update_entity<'a, 'app, 'win, T: 'static, R>(
        &'a mut self,
        handle: &Handle<T>,
        update: impl for<'cx> FnOnce(&mut T, &mut Self::EntityContext<'cx, 'app, 'win, T>) -> R,
    ) -> R;
}

impl Context for AppContext {
    type EntityContext<'cx, 'app, 'win, T: 'static> = ModelContext<'cx, T>;

    fn update_entity<'a, 'app, 'win, T: 'static, R>(
        &'a mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, 'app, 'win, T>) -> R,
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

// impl<'app, 'win> Context for WindowContext<'app, 'win> {
//     type EntityContext<'cx, 'app, 'win, T: 'static> = ModelContext<'cx, T>;

//     fn update_entity<'a, 'app, 'win, T: 'static, R>(
//         &'a mut self,
//         handle: &Handle<T>,
//         update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, 'app, 'win, T>) -> R,
//     ) -> R {
//         let mut entity = self
//             .entities
//             .remove(&handle.id)
//             .unwrap()
//             .downcast::<T>()
//             .unwrap();
//         let result = update(&mut *entity, &mut ModelContext::mutable(self, handle.id));
//         self.entities.insert(handle.id, Box::new(entity));
//         result
//     }
// }

impl<T: 'static> Handle<T> {
    fn update<'app, 'win, C: Context, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, 'app, 'win, T>) -> R,
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
        // render: Rc::new(move |cx| state.update(cx, |state, cx| render(state, cx))),
        render: todo!(),
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
    div().child(state.left_panel.render(cx))
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
        // let collab_panel = cx.add_entity(|cx| CollabPanel::new(cx));

        // let
        // let mut workspace = Workspace {
        //     left_panel: view(),
        // }

        // cx.open_window(workspace::Workspace, state)
    }
}
