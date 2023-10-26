use crate::{
    AnyBox, AnyElement, BorrowWindow, Bounds, Component, Element, ElementId, EntityId, Handle,
    LayoutId, Pixels, ViewContext, WeakHandle, WindowContext,
};
use parking_lot::Mutex;
use std::{
    marker::PhantomData,
    sync::{Arc, Weak},
};

pub struct View<V> {
    pub(crate) state: Handle<V>,
    render: Arc<Mutex<dyn Fn(&mut V, &mut ViewContext<V>) -> AnyElement<V> + Send + 'static>>,
}

pub fn view<V, E>(
    state: Handle<V>,
    render: impl Fn(&mut V, &mut ViewContext<'_, '_, V>) -> E + Send + 'static,
) -> View<V>
where
    E: Component<V>,
{
    View {
        state,
        render: Arc::new(Mutex::new(
            move |state: &mut V, cx: &mut ViewContext<'_, '_, V>| render(state, cx).render(),
        )),
    }
}

impl<V: 'static> View<V> {
    pub fn into_any(self) -> AnyView {
        AnyView(Arc::new(self))
    }

    pub fn downgrade(&self) -> WeakView<V> {
        WeakView {
            state: self.state.downgrade(),
            render: Arc::downgrade(&self.render),
        }
    }
}

impl<V: 'static> View<V> {
    pub fn update<R>(
        &self,
        cx: &mut WindowContext,
        f: impl FnOnce(&mut V, &mut ViewContext<V>) -> R,
    ) -> R {
        let this = self.clone();
        let mut lease = cx.app.entities.lease(&self.state);
        let mut cx = ViewContext::mutable(&mut *cx.app, &mut *cx.window, this);
        let result = f(&mut *lease, &mut cx);
        cx.app.entities.end_lease(lease);
        result
    }
}

impl<V> Clone for View<V> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            render: self.render.clone(),
        }
    }
}

impl<V: 'static, ParentViewState: 'static> Component<ParentViewState> for View<V> {
    fn render(self) -> AnyElement<ParentViewState> {
        AnyElement::new(EraseViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl<V: 'static> Element<()> for View<V> {
    type ElementState = AnyElement<V>;

    fn id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.state.entity_id))
    }

    fn initialize(
        &mut self,
        _: &mut (),
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<()>,
    ) -> Self::ElementState {
        self.update(cx, |state, cx| {
            let mut any_element = (self.render.lock())(state, cx);
            any_element.initialize(state, cx);
            any_element
        })
    }

    fn layout(
        &mut self,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) -> LayoutId {
        self.update(cx, |state, cx| element.layout(state, cx))
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) {
        self.update(cx, |state, cx| element.paint(state, cx))
    }
}

pub struct WeakView<V> {
    state: WeakHandle<V>,
    render: Weak<Mutex<dyn Fn(&mut V, &mut ViewContext<V>) -> AnyElement<V> + Send + 'static>>,
}

impl<V> WeakView<V> {
    pub fn upgrade(&self) -> Option<View<V>> {
        let state = self.state.upgrade()?;
        let render = self.render.upgrade()?;
        Some(View { state, render })
    }
}

struct EraseViewState<V, ParentV> {
    view: View<V>,
    parent_view_state_type: PhantomData<ParentV>,
}

unsafe impl<V, ParentV> Send for EraseViewState<V, ParentV> {}

impl<V: 'static, ParentV: 'static> Component<ParentV> for EraseViewState<V, ParentV> {
    fn render(self) -> AnyElement<ParentV> {
        AnyElement::new(self)
    }
}

impl<V: 'static, ParentV: 'static> Element<ParentV> for EraseViewState<V, ParentV> {
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Element::id(&self.view)
    }

    fn initialize(
        &mut self,
        _: &mut ParentV,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentV>,
    ) -> Self::ElementState {
        ViewObject::initialize(&mut self.view, cx)
    }

    fn layout(
        &mut self,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) -> LayoutId {
        ViewObject::layout(&mut self.view, element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) {
        ViewObject::paint(&mut self.view, bounds, element, cx)
    }
}

trait ViewObject: Send + Sync {
    fn entity_id(&self) -> EntityId;
    fn initialize(&self, cx: &mut WindowContext) -> AnyBox;
    fn layout(&self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId;
    fn paint(&self, bounds: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext);
}

impl<V: 'static> ViewObject for View<V> {
    fn entity_id(&self) -> EntityId {
        self.state.entity_id
    }

    fn initialize(&self, cx: &mut WindowContext) -> AnyBox {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let mut any_element = Box::new((self.render.lock())(state, cx));
                any_element.initialize(state, cx);
                any_element as AnyBox
            })
        })
    }

    fn layout(&self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.layout(state, cx)
            })
        })
    }

    fn paint(&self, _: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext) {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.paint(state, cx);
            });
        });
    }
}

#[derive(Clone)]
pub struct AnyView(Arc<dyn ViewObject>);

impl<ParentV: 'static> Component<ParentV> for AnyView {
    fn render(self) -> AnyElement<ParentV> {
        AnyElement::new(EraseAnyViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl Element<()> for AnyView {
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.0.entity_id()))
    }

    fn initialize(
        &mut self,
        _: &mut (),
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<()>,
    ) -> Self::ElementState {
        self.0.initialize(cx)
    }

    fn layout(
        &mut self,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) -> LayoutId {
        self.0.layout(element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut (),
        element: &mut AnyBox,
        cx: &mut ViewContext<()>,
    ) {
        self.0.paint(bounds, element, cx)
    }
}

struct EraseAnyViewState<ParentViewState> {
    view: AnyView,
    parent_view_state_type: PhantomData<ParentViewState>,
}

unsafe impl<ParentV> Send for EraseAnyViewState<ParentV> {}

impl<ParentV: 'static> Component<ParentV> for EraseAnyViewState<ParentV> {
    fn render(self) -> AnyElement<ParentV> {
        AnyElement::new(self)
    }
}

impl<ParentV: 'static> Element<ParentV> for EraseAnyViewState<ParentV> {
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Element::id(&self.view)
    }

    fn initialize(
        &mut self,
        _: &mut ParentV,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentV>,
    ) -> Self::ElementState {
        self.view.0.initialize(cx)
    }

    fn layout(
        &mut self,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) -> LayoutId {
        self.view.0.layout(element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) {
        self.view.0.paint(bounds, element, cx)
    }
}
