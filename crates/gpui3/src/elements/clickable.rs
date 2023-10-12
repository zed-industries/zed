use crate::{
    AnyElement, Bounds, DispatchPhase, Element, IdentifiedElement, Interactive, IntoAnyElement,
    MouseDownEvent, MouseEventListeners, MouseUpEvent, ParentElement, Pixels, Styled, ViewContext,
};
use parking_lot::Mutex;
use refineable::Cascade;
use smallvec::SmallVec;
use std::sync::Arc;

pub type ClickListener<S> =
    dyn Fn(&mut S, (&MouseDownEvent, &MouseUpEvent), &mut ViewContext<S>) + Send + Sync + 'static;

pub struct Clickable<E: Element> {
    child: E,
    listener: Arc<ClickListener<E::ViewState>>,
}

pub struct ClickableState<S> {
    last_mouse_down: Arc<Mutex<Option<MouseDownEvent>>>,
    child_state: S,
}

impl<E: Element> Clickable<E> {
    pub fn new(child: E, listener: Arc<ClickListener<E::ViewState>>) -> Self {
        Self { child, listener }
    }
}

impl<E> Styled for Clickable<E>
where
    E: Styled + IdentifiedElement,
{
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut Cascade<E::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.child.declared_style()
    }
}

impl<S, E> Interactive<S> for Clickable<E>
where
    S: 'static + Send + Sync,
    E: IdentifiedElement + Interactive<S>,
{
    fn listeners(&mut self) -> &mut MouseEventListeners<S> {
        self.child.listeners()
    }
}

impl<E: IdentifiedElement> IntoAnyElement<E::ViewState> for Clickable<E> {
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E> Element for Clickable<E>
where
    E: IdentifiedElement,
{
    type ViewState = E::ViewState;
    type ElementState = ClickableState<E::ElementState>;

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
            let (layout_id, child_state) =
                self.child
                    .layout(state, Some(element_state.child_state), cx);

            let element_state = ClickableState {
                last_mouse_down: element_state.last_mouse_down,
                child_state,
            };
            (layout_id, element_state)
        } else {
            let (layout_id, child_state) = self.child.layout(state, None, cx);
            let element_state = ClickableState {
                last_mouse_down: Default::default(),
                child_state,
            };
            (layout_id, element_state)
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        let last_mouse_down = element_state.last_mouse_down.clone();
        let is_some = last_mouse_down.lock().is_some();

        if is_some {
            let listener = self.listener.clone();
            cx.on_mouse_event(move |view, up_event: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Capture && !bounds.contains_point(up_event.position) {
                    *last_mouse_down.lock() = None;
                } else if phase == DispatchPhase::Bubble && bounds.contains_point(up_event.position)
                {
                    if let Some(down_event) = last_mouse_down.lock().take() {
                        listener(view, (&down_event, up_event), cx);
                    } else {
                        log::error!("No mouse down event found for click event");
                    }
                }
            })
        } else {
            cx.on_mouse_event(move |_, event: &MouseDownEvent, phase, _| {
                if phase == DispatchPhase::Bubble {
                    if bounds.contains_point(event.position) {
                        *last_mouse_down.lock() = Some(event.clone());
                    }
                }
            })
        }

        self.child
            .paint(bounds, state, &mut element_state.child_state, cx);
    }
}

impl<E: IdentifiedElement + ParentElement> ParentElement for Clickable<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

impl<E> IdentifiedElement for Clickable<E> where E: IdentifiedElement + Styled {}
