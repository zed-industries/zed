use parking_lot::Mutex;

use crate::{
    AnyBox, AnyElement, BorrowWindow, Bounds, Element, ElementId, EntityId, Handle,
    IdentifiedElement, IntoAnyElement, LayoutId, Pixels, ViewContext, WindowContext,
};
use std::{any::Any, marker::PhantomData, sync::Arc};

pub struct View<S: Send + Sync> {
    state: Handle<S>,
    render: Arc<dyn Fn(&mut S, &mut ViewContext<S>) -> AnyElement<S> + Send + Sync + 'static>,
}

impl<S: 'static + Send + Sync> View<S> {
    pub fn into_any(self) -> AnyView {
        AnyView {
            view: Arc::new(Mutex::new(self)),
        }
    }
}

impl<S: Send + Sync> Clone for View<S> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            render: self.render.clone(),
        }
    }
}

pub fn view<S, E>(
    state: Handle<S>,
    render: impl Fn(&mut S, &mut ViewContext<S>) -> E + Send + Sync + 'static,
) -> View<S>
where
    S: 'static + Send + Sync,
    E: Element<ViewState = S>,
{
    View {
        state,
        render: Arc::new(move |state, cx| render(state, cx).into_any()),
    }
}

impl<S: 'static + Send + Sync, ParentViewState: 'static + Send + Sync>
    IntoAnyElement<ParentViewState> for View<S>
{
    fn into_any(self) -> AnyElement<ParentViewState> {
        AnyElement::new(EraseViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl<S: 'static + Send + Sync> Element for View<S> {
    type ViewState = ();
    type ElementState = AnyElement<S>;

    fn element_id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.state.id))
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx);
            (layout_id, element)
        })
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut Self::ViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.state
            .update(cx, |state, cx| element.paint(state, None, cx))
    }
}

struct EraseViewState<ViewState: 'static + Send + Sync, ParentViewState> {
    view: View<ViewState>,
    parent_view_state_type: PhantomData<ParentViewState>,
}

impl<ViewState, ParentViewState> IntoAnyElement<ParentViewState>
    for EraseViewState<ViewState, ParentViewState>
where
    ViewState: 'static + Send + Sync,
    ParentViewState: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<ParentViewState> {
        AnyElement::new(self)
    }
}

impl<ViewState, ParentViewState> Element for EraseViewState<ViewState, ParentViewState>
where
    ViewState: 'static + Send + Sync,
    ParentViewState: 'static + Send + Sync,
{
    type ViewState = ParentViewState;
    type ElementState = AnyBox;

    fn element_id(&self) -> Option<crate::ElementId> {
        Element::element_id(&self.view)
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        ViewObject::layout(&mut self.view, cx)
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

trait ViewObject: 'static + Send + Sync {
    fn entity_id(&self) -> EntityId;
    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, AnyBox);
    fn paint(&mut self, bounds: Bounds<Pixels>, element: &mut dyn Any, cx: &mut WindowContext);
}

impl<S: Send + Sync + 'static> IdentifiedElement for View<S> {}

impl<S: Send + Sync + 'static> ViewObject for View<S> {
    fn entity_id(&self) -> EntityId {
        self.state.id
    }

    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, AnyBox) {
        cx.with_element_id(IdentifiedElement::element_id(self), |cx| {
            self.state.update(cx, |state, cx| {
                let mut element = (self.render)(state, cx);
                let layout_id = element.layout(state, cx);
                let element = Box::new(element) as AnyBox;
                (layout_id, element)
            })
        })
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut dyn Any, cx: &mut WindowContext) {
        cx.with_element_id(IdentifiedElement::element_id(self), |cx| {
            self.state.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<S>>().unwrap();
                element.paint(state, None, cx);
            });
        });
    }
}

pub struct AnyView {
    view: Arc<Mutex<dyn ViewObject>>,
}

impl<ParentViewState> IntoAnyElement<ParentViewState> for AnyView
where
    ParentViewState: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<ParentViewState> {
        AnyElement::new(EraseAnyViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl Element for AnyView {
    type ViewState = ();
    type ElementState = AnyBox;

    fn element_id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.view.lock().entity_id()))
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.view.lock().layout(cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut (),
        element: &mut AnyBox,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.view.lock().paint(bounds, element.as_mut(), cx)
    }
}

struct EraseAnyViewState<ParentViewState> {
    view: AnyView,
    parent_view_state_type: PhantomData<ParentViewState>,
}

impl<ParentViewState> IntoAnyElement<ParentViewState> for EraseAnyViewState<ParentViewState>
where
    ParentViewState: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<ParentViewState> {
        AnyElement::new(self)
    }
}

impl<ParentViewState> Element for EraseAnyViewState<ParentViewState>
where
    ParentViewState: 'static + Send + Sync,
{
    type ViewState = ParentViewState;
    type ElementState = AnyBox;

    fn element_id(&self) -> Option<crate::ElementId> {
        Element::element_id(&self.view)
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.view.view.lock().layout(cx)
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
