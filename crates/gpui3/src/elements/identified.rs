use refineable::{Cascade, Refineable};
use smallvec::SmallVec;

use crate::{
    AnyElement, BorrowWindow, Bounds, Element, ElementId, IdentifiedElement, LayoutId,
    ParentElement, Styled, ViewContext,
};

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

impl<E: Element> IdentifiedElement for Identified<E> {}

impl<E: Styled> Styled for Identified<E> {
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut Cascade<Self::Style> {
        self.element.style_cascade()
    }
    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        self.element.declared_style()
    }
}

impl<E: ParentElement> ParentElement for Identified<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.element.children_mut()
    }
}
