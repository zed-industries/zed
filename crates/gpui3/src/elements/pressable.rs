use crate::{
    AnyElement, Bounds, DispatchPhase, Element, IdentifiedElement, Interactive, MouseDownEvent,
    MouseEventListeners, MouseUpEvent, ParentElement, Pixels, Styled, ViewContext,
};
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use smallvec::SmallVec;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

pub struct Pressable<E: Styled> {
    cascade_slot: CascadeSlot,
    pressed_style: <E::Style as Refineable>::Refinement,
    child: E,
}

pub struct PressableState<S> {
    pressed: Arc<AtomicBool>,
    child_state: S,
}

impl<E: Styled> Pressable<E> {
    pub fn new(mut child: E) -> Self {
        Self {
            cascade_slot: child.style_cascade().reserve(),
            pressed_style: Default::default(),
            child,
        }
    }
}

impl<E> Styled for Pressable<E>
where
    E: Styled,
{
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<E::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        &mut self.pressed_style
    }
}

impl<S: 'static + Send + Sync, E: Interactive<S> + Styled> Interactive<S> for Pressable<E> {
    fn listeners(&mut self) -> &mut MouseEventListeners<S> {
        self.child.listeners()
    }
}

impl<E> Element for Pressable<E>
where
    E: Styled + IdentifiedElement,
    <E as Styled>::Style: 'static + Refineable + Send + Sync + Default,
    <<E as Styled>::Style as Refineable>::Refinement: 'static + Refineable + Send + Sync + Default,
{
    type ViewState = E::ViewState;
    type ElementState = PressableState<E::ElementState>;

    fn element_id(&self) -> Option<crate::ElementId> {
        Some(IdentifiedElement::element_id(&self.child))
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        if let Some(element_state) = element_state {
            let (id, child_state) = self
                .child
                .layout(state, Some(element_state.child_state), cx);
            let element_state = PressableState {
                pressed: element_state.pressed,
                child_state,
            };
            (id, element_state)
        } else {
            let (id, child_state) = self.child.layout(state, None, cx);
            let element_state = PressableState {
                pressed: Default::default(),
                child_state,
            };
            (id, element_state)
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        let style = element_state
            .pressed
            .load(SeqCst)
            .then_some(self.pressed_style.clone());
        let slot = self.cascade_slot;
        self.style_cascade().set(slot, style);

        let pressed = element_state.pressed.clone();
        cx.on_mouse_event(move |_, event: &MouseDownEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if bounds.contains_point(event.position) {
                    pressed.store(true, SeqCst);
                    cx.notify();
                }
            }
        });
        let pressed = element_state.pressed.clone();
        cx.on_mouse_event(move |_, _: &MouseUpEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if pressed.load(SeqCst) {
                    pressed.store(false, SeqCst);
                    cx.notify();
                }
            }
        });

        self.child
            .paint(bounds, state, &mut element_state.child_state, cx);
    }
}

impl<E> ParentElement for Pressable<E>
where
    E: ParentElement + IdentifiedElement + Styled,
{
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

impl<E> IdentifiedElement for Pressable<E>
where
    E: IdentifiedElement + Styled,
    <E as Styled>::Style: 'static + Refineable + Send + Sync + Default,
    <<E as Styled>::Style as Refineable>::Refinement: 'static + Refineable + Send + Sync + Default,
{
}
