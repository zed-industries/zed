use crate::{
    AnyElement, Bounds, DispatchPhase, Element, ElementId, Interactive, MouseEventListeners,
    MouseMoveEvent, ParentElement, Pixels, Stateful, Styled, ViewContext,
};
use anyhow::Result;
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use smallvec::SmallVec;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

pub struct Hoverable<E: Styled> {
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

impl<E: Element + Styled> Element for Hoverable<E> {
    type State = E::State;
    type FrameState = E::FrameState;

    fn element_id(&self) -> Option<ElementId> {
        self.child.element_id()
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(crate::LayoutId, Self::FrameState)> {
        Ok(self.child.layout(state, cx)?)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        let hovered = bounds.contains_point(cx.mouse_position());
        let slot = self.cascade_slot;
        let style = hovered.then_some(self.hovered_style.clone());
        self.style_cascade().set(slot, style);
        self.hovered.store(hovered, SeqCst);

        let hovered = self.hovered.clone();
        cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if bounds.contains_point(event.position) != hovered.load(SeqCst) {
                    cx.notify();
                }
            }
        });

        self.child.paint(bounds, state, frame_state, cx)?;
        Ok(())
    }
}

impl<E: ParentElement + Styled> ParentElement for Hoverable<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

impl<E: Stateful + Styled> Stateful for Hoverable<E> {}
