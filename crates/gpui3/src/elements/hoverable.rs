use crate::{
    AnyElement, Bounds, DispatchPhase, Element, ElementId, Interactive, MouseEventListeners,
    MouseMoveEvent, ParentElement, Pixels, StatefulElement, Styled, ViewContext,
};
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use smallvec::SmallVec;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

pub struct Hoverable<E: Styled>
// where
// <E::Style as Refineable>::Refinement: 'static + Send + Sync,
{
    hovered: Arc<AtomicBool>,
    cascade_slot: CascadeSlot,
    hovered_style: <E::Style as Refineable>::Refinement,
    child: E,
}

impl<E: Styled> Hoverable<E> {
    pub fn new(mut child: E) -> Self {
        Self {
            hovered: Arc::new(AtomicBool::new(false)),
            cascade_slot: child.style_cascade().reserve(),
            hovered_style: Default::default(),
            child,
        }
    }
}

impl<E> Styled for Hoverable<E>
where
    E: Styled,
{
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<E::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        &mut self.hovered_style
    }
}

impl<S: 'static + Send + Sync, E: Interactive<S> + Styled> Interactive<S> for Hoverable<E> {
    fn listeners(&mut self) -> &mut MouseEventListeners<S> {
        self.child.listeners()
    }
}

impl<E> Element for Hoverable<E>
where
    E: Element + Styled,
    <E as Styled>::Style: 'static + Refineable + Send + Sync + Default,
    <<E as Styled>::Style as Refineable>::Refinement: 'static + Refineable + Send + Sync + Default,
{
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
        let hovered = bounds.contains_point(cx.mouse_position());
        let slot = self.cascade_slot;
        let style = hovered.then_some(self.hovered_style.clone());
        self.style_cascade().set(slot, style);
        self.hovered.store(hovered, SeqCst);

        let hovered = self.hovered.clone();
        cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if bounds.contains_point(event.position) != hovered.load(SeqCst) {
                    cx.notify();
                }
            }
        });

        self.child.paint(bounds, state, element_state, cx);
    }
}

impl<E: ParentElement + Styled> ParentElement for Hoverable<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

impl<E> StatefulElement for Hoverable<E>
where
    E: StatefulElement + Styled,
    <E as Styled>::Style: 'static + Refineable + Send + Sync + Default,
    <<E as Styled>::Style as Refineable>::Refinement: 'static + Refineable + Send + Sync + Default,
{
}
