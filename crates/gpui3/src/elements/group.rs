use crate::{
    AnyElement, AppContext, Bounds, Element, ElementId, IdentifiedElement, Interactive,
    MouseEventListeners, ParentElement, Pixels, SharedString, Styled, ViewContext,
};
use collections::HashMap;
use refineable::Cascade;
use smallvec::SmallVec;

#[derive(Default)]
struct GroupBounds(HashMap<SharedString, SmallVec<[Bounds<Pixels>; 1]>>);

pub fn group_bounds(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
    cx.default_global::<GroupBounds>()
        .0
        .get(name)
        .and_then(|bounds_stack| bounds_stack.last().cloned())
}

pub struct Group<E> {
    name: SharedString,
    child: E,
}

impl<E> Group<E> {
    pub fn new(name: SharedString, child: E) -> Self {
        Group { name, child }
    }
}

impl<E: Element> Element for Group<E> {
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

impl<E: ParentElement> ParentElement for Group<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

impl<E> IdentifiedElement for Group<E> where E: IdentifiedElement {}

impl<E> Styled for Group<E>
where
    E: Styled,
{
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut Cascade<E::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.child.declared_style()
    }
}

impl<S: 'static + Send + Sync, E: Interactive<S> + Styled> Interactive<S> for Group<E> {
    fn listeners(&mut self) -> &mut MouseEventListeners<S> {
        self.child.listeners()
    }
}
