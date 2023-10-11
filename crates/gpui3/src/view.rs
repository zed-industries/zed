use parking_lot::Mutex;

use crate::{
    AnyBox, AnyElement, Bounds, Element, Handle, IntoAnyElement, LayoutId, Pixels, ViewContext,
    WindowContext,
};
use std::{any::Any, marker::PhantomData, sync::Arc};

pub struct View<S: Send + Sync, P> {
    state: Handle<S>,
    render: Arc<dyn Fn(&mut S, &mut ViewContext<S>) -> AnyElement<S> + Send + Sync + 'static>,
    parent_state_type: PhantomData<P>,
}

impl<S: 'static + Send + Sync, P: 'static + Send> View<S, P> {
    pub fn into_any(self) -> AnyView<P> {
        AnyView {
            view: Arc::new(Mutex::new(self)),
            parent_state_type: PhantomData,
        }
    }
}

impl<S: Send + Sync, P> Clone for View<S, P> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            render: self.render.clone(),
            parent_state_type: PhantomData,
        }
    }
}

pub type RootView<S> = View<S, ()>;

pub fn view<S, P, E>(
    state: Handle<S>,
    render: impl Fn(&mut S, &mut ViewContext<S>) -> E + Send + Sync + 'static,
) -> View<S, P>
where
    S: 'static + Send + Sync,
    P: 'static,
    E: Element<ViewState = S>,
{
    View {
        state,
        render: Arc::new(move |state, cx| render(state, cx).into_any()),
        parent_state_type: PhantomData,
    }
}

impl<S: 'static + Send + Sync, P: 'static + Send + Sync> Element for View<S, P> {
    type ViewState = P;
    type ElementState = AnyElement<S>;

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

trait ViewObject: Send + 'static {
    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, AnyBox);
    fn paint(&mut self, bounds: Bounds<Pixels>, element: &mut dyn Any, cx: &mut WindowContext);
}

impl<S: Send + Sync + 'static, P: Send + 'static> ViewObject for View<S, P> {
    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, AnyBox) {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx);
            let element = Box::new(element) as AnyBox;
            (layout_id, element)
        })
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut dyn Any, cx: &mut WindowContext) {
        self.state.update(cx, |state, cx| {
            let element = element.downcast_mut::<AnyElement<S>>().unwrap();
            element.paint(state, None, cx);
        });
    }
}

pub struct AnyView<S> {
    view: Arc<Mutex<dyn ViewObject>>,
    parent_state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> Element for AnyView<S> {
    type ViewState = ();
    type ElementState = AnyBox;

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

impl<S> Clone for AnyView<S> {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            parent_state_type: PhantomData,
        }
    }
}
