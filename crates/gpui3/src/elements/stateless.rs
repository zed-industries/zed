use crate::{Bounds, Element, Pixels};
use std::marker::PhantomData;

pub struct Stateless<E: Element<State = ()>, S> {
    element: E,
    parent_state_type: PhantomData<S>,
}

impl<E: Element<State = ()>, S: Send + Sync + 'static> Element for Stateless<E, S> {
    type State = S;
    type FrameState = E::FrameState;

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut crate::ViewContext<Self::State>,
    ) -> anyhow::Result<(crate::LayoutId, Self::FrameState)> {
        cx.erase_state(|cx| self.element.layout(&mut (), cx))
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut crate::ViewContext<Self::State>,
    ) -> anyhow::Result<()> {
        cx.erase_state(|cx| self.element.paint(bounds, &mut (), frame_state, cx))
    }
}
