use crate::{BorrowWindow, Bounds, Element, ElementId, LayoutId, ViewContext};
use anyhow::Result;

pub trait Identified {
    fn id(&self) -> ElementId;
}

pub struct ElementWithId<E> {
    pub(crate) element: E,
    pub(crate) id: ElementId,
}

impl<E: Element> Element for ElementWithId<E> {
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
        cx.with_element_id(self.id.clone(), |cx| {
            self.element.paint(bounds, state, frame_state, cx)
        })
    }
}

impl<E> Identified for ElementWithId<E> {
    fn id(&self) -> ElementId {
        self.id.clone()
    }
}
