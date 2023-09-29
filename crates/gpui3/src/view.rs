use parking_lot::Mutex;

use crate::{
    AnyElement, Element, Handle, IntoAnyElement, Layout, LayoutId, MainThread, Result, ViewContext,
    WindowContext,
};
use std::{any::Any, marker::PhantomData, sync::Arc};

pub struct View<S: Send + Sync, P, Thread = ()> {
    state: Handle<S>,
    render:
        Arc<dyn Fn(&mut S, &mut ViewContext<S, Thread>) -> AnyElement<S> + Send + Sync + 'static>,
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
    E: Element<State = S>,
{
    View {
        state,
        render: Arc::new(move |state, cx| render(state, cx).into_any()),
        parent_state_type: PhantomData,
    }
}

impl<S: Send + Sync + 'static, P: Send + 'static> Element for View<S, P> {
    type State = P;
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
        _: Layout,
        _: &mut Self::State,
        element: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.state
            .update(cx, |state, cx| element.paint(state, None, cx))
    }
}

trait ViewObject: Send + 'static {
    fn layout(&mut self, cx: &mut WindowContext) -> Result<(LayoutId, Box<dyn Any>)>;
    fn paint(
        &mut self,
        layout: Layout,
        element: &mut dyn Any,
        cx: &mut WindowContext,
    ) -> Result<()>;
}

impl<S: Send + Sync + 'static, P: Send + 'static> ViewObject for View<S, P> {
    fn layout(&mut self, cx: &mut WindowContext) -> Result<(LayoutId, Box<dyn Any>)> {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx)?;
            let element = Box::new(element) as Box<dyn Any>;
            Ok((layout_id, element))
        })
    }

    fn paint(&mut self, _: Layout, element: &mut dyn Any, cx: &mut WindowContext) -> Result<()> {
        self.state.update(cx, |state, cx| {
            element
                .downcast_mut::<AnyElement<S>>()
                .unwrap()
                .paint(state, None, cx)
        })
    }
}

pub struct AnyView<S> {
    view: Arc<Mutex<dyn ViewObject>>,
    parent_state_type: PhantomData<S>,
}

impl<S: 'static> Element for AnyView<S> {
    type State = ();
    type FrameState = Box<dyn Any>;

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.view.lock().layout(cx)
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        element: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.view.lock().paint(layout, element, cx)
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
