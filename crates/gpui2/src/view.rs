use parking_lot::Mutex;

use crate::{
    AnyBox, AnyElement, BorrowWindow, Bounds, Element, ElementId, EntityId, Handle, IntoAnyElement,
    LayoutId, Pixels, ViewContext, WindowContext,
};
use std::{marker::PhantomData, sync::Arc};

pub struct View<V> {
    state: Handle<V>,
    render: Arc<dyn Fn(&mut V, &mut ViewContext<V>) -> AnyElement<V> + Send + Sync + 'static>,
}

impl<V> View<V> {
    pub fn into_any(self) -> AnyView {
        AnyView {
            view: Arc::new(Mutex::new(self)),
        }
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

pub fn view<V, E>(
    state: Handle<V>,
    render: impl Fn(&mut V, &mut ViewContext<V>) -> E + Send + Sync + 'static,
) -> View<V>
where
    E: IntoAnyElement<V>,
    V: 'static + Send + Sync,
{
    View {
        state,
        render: Arc::new(move |state, cx| render(state, cx).into_any()),
    }
}

impl<V, ParentViewState> IntoAnyElement<ParentViewState> for View<V> {
    fn into_any(self) -> AnyElement<ParentViewState> {
        AnyElement::new(EraseViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl<V> Element for View<V> {
    type ViewState = ();
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
        self.state.update(cx, |state, cx| {
            let mut any_element = (self.render)(state, cx);
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
        self.state.update(cx, |state, cx| element.layout(state, cx))
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) {
        self.state.update(cx, |state, cx| element.paint(state, cx))
    }
}

struct EraseViewState<V, ParentV> {
    view: View<V>,
    parent_view_state_type: PhantomData<ParentV>,
}

impl<V, ParentV> IntoAnyElement<ParentV> for EraseViewState<V, ParentV> {
    fn into_any(self) -> AnyElement<ParentV> {
        AnyElement::new(self)
    }
}

impl<V, ParentV> Element for EraseViewState<V, ParentV> {
    type ViewState = ParentV;
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Element::id(&self.view)
    }

    fn initialize(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        ViewObject::initialize(&mut self.view, cx)
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> LayoutId {
        ViewObject::layout(&mut self.view, element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::ViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        ViewObject::paint(&mut self.view, bounds, element, cx)
    }
}

trait ViewObject {
    fn entity_id(&self) -> EntityId;
    fn initialize(&mut self, cx: &mut WindowContext) -> AnyBox;
    fn layout(&mut self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId;
    fn paint(&mut self, bounds: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext);
}

impl<V: 'static + Send + Sync> ViewObject for View<V> {
    fn entity_id(&self) -> EntityId {
        self.state.entity_id
    }

    fn initialize(&mut self, cx: &mut WindowContext) -> AnyBox {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.state.update(cx, |state, cx| {
                let mut any_element = Box::new((self.render)(state, cx));
                any_element.initialize(state, cx);
                any_element as AnyBox
            })
        })
    }

    fn layout(&mut self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.state.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.layout(state, cx)
            })
        })
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext) {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.state.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.paint(state, cx);
            });
        });
    }
}

pub struct AnyView {
    view: Arc<Mutex<dyn ViewObject>>,
}

impl<ParentV> IntoAnyElement<ParentV> for AnyView
where
    ParentV: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<ParentV> {
        AnyElement::new(EraseAnyViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl Element for AnyView {
    type ViewState = ();
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.view.lock().entity_id()))
    }

    fn initialize(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        self.view.lock().initialize(cx)
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> LayoutId {
        self.view.lock().layout(element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut (),
        element: &mut AnyBox,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.view.lock().paint(bounds, element, cx)
    }
}

struct EraseAnyViewState<ParentViewState> {
    view: AnyView,
    parent_view_state_type: PhantomData<ParentViewState>,
}

impl<ParentV> IntoAnyElement<ParentV> for EraseAnyViewState<ParentV>
where
    ParentV: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<ParentV> {
        AnyElement::new(self)
    }
}

impl<ParentV> Element for EraseAnyViewState<ParentV>
where
    ParentV: 'static + Send + Sync,
{
    type ViewState = ParentV;
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Element::id(&self.view)
    }

    fn initialize(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        self.view.view.lock().initialize(cx)
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> LayoutId {
        self.view.view.lock().layout(element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::ViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.view.view.lock().paint(bounds, element, cx)
    }
}

impl Clone for AnyView {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
        }
    }
}
