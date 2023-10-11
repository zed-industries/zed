use crate::{
    AnyElement, AppContext, Bounds, Element, ElementId, IdentifiedElement, ParentElement, Pixels,
    SharedString, ViewContext,
};
use collections::HashMap;
use smallvec::SmallVec;

#[derive(Default)]
struct GroupBounds(HashMap<SharedString, SmallVec<[Bounds<Pixels>; 1]>>);

pub fn element_group_bounds(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
    cx.default_global::<GroupBounds>()
        .0
        .get(name)
        .and_then(|bounds_stack| bounds_stack.last().cloned())
}

pub struct ElementGroup<E> {
    name: SharedString,
    child: E,
}

impl<E> ElementGroup<E> {
    pub fn new(name: SharedString, child: E) -> Self {
        ElementGroup { name, child }
    }
}

impl<E: Element> Element for ElementGroup<E> {
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn element_id(&self) -> Option<ElementId> {
        self.child.element_id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        self.child.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        cx.default_global::<GroupBounds>()
            .0
            .entry(self.name.clone())
            .or_default()
            .push(bounds);
        self.child.paint(bounds, state, element_state, cx);
        cx.default_global::<GroupBounds>()
            .0
            .get_mut(&self.name)
            .unwrap()
            .pop();
    }
}

impl<E: ParentElement> ParentElement for ElementGroup<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

impl<E> IdentifiedElement for ElementGroup<E> where E: IdentifiedElement {}
