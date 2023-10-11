use crate::{BorrowWindow, Bounds, Element, ElementId, LayoutId, StatefulElement, ViewContext};

pub struct Identified<E> {
    pub(crate) element: E,
    pub(crate) id: ElementId,
}

impl<E: Element> Element for Identified<E> {
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.element.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        cx.with_element_id(self.id.clone(), |cx| {
            self.element.paint(bounds, state, element_state, cx)
        })
    }
}

impl<E: Element> StatefulElement for Identified<E> {}
