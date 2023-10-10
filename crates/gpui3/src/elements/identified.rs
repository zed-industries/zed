use crate::{ElementId, Element, Bounds, ViewContext, LayoutId};
use anyhow::Result;
use derive_more::{Deref, DerefMut}

#[derive(Deref, DerefMut)]
pub struct Identified<E> {
    #[deref]
    #[deref_mut]
    element: E,
    id: ElementId,
}

impl<E: Element> Element for Identified<E> {
    type State = E::State;
    type FrameState = E::FrameState;

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.element.layout(state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<crate::Pixels>,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        cx.with_element_id(self.id, |cx| {
            self.element.paint(bounds, state, frame_state, cx)
        })
    }
}
