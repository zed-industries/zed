use crate::{
    AnyElement, Bounds, DispatchPhase, Element, ElementId, Hoverable, IdentifiedElement,
    IntoAnyElement, LayoutId, MouseDownEvent, MouseUpEvent, ParentElement, Pixels, SharedString,
    StyleRefinement, Styled, ViewContext,
};
use parking_lot::Mutex;
use refineable::CascadeSlot;
use smallvec::SmallVec;
use std::sync::Arc;

pub trait Clickable: Element + Sized {
    fn active_style(&mut self) -> &mut StyleRefinement;
    fn listeners(&mut self) -> &mut ClickListeners<Self::ViewState>;

    fn on_click(
        &mut self,
        f: impl Fn(&mut Self::ViewState, &MouseClickEvent, &mut ViewContext<Self::ViewState>)
            + 'static
            + Send
            + Sync,
    ) where
        Self: Sized,
    {
        self.listeners().push(Arc::new(f));
    }

    fn active(mut self, f: impl FnOnce(&mut StyleRefinement) -> &mut StyleRefinement) -> Self
    where
        Self: Sized,
    {
        f(self.active_style());
        self
    }
}

pub type ClickListeners<V> =
    SmallVec<[Arc<dyn Fn(&mut V, &MouseClickEvent, &mut ViewContext<V>) + Send + Sync>; 1]>;

pub struct ClickableElementState<E: 'static + Send + Sync> {
    mouse_down: Arc<Mutex<Option<MouseDownEvent>>>,
    child_state: E,
}

pub struct MouseClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}

pub struct ClickableElement<E: Element> {
    child: E,
    listeners: ClickListeners<E::ViewState>,
    active_style: StyleRefinement,
    cascade_slot: CascadeSlot,
}

impl<E: Styled + Element> ClickableElement<E> {
    pub fn new(mut child: E) -> Self {
        let cascade_slot = child.style_cascade().reserve();
        ClickableElement {
            child,
            listeners: Default::default(),
            active_style: Default::default(),
            cascade_slot,
        }
    }

    pub fn replace_child<E2: Element<ViewState = E::ViewState>>(
        self,
        replace: impl FnOnce(E) -> E2,
    ) -> ClickableElement<E2> {
        ClickableElement {
            child: replace(self.child),
            listeners: self.listeners,
            active_style: self.active_style,
            cascade_slot: self.cascade_slot,
        }
    }
}

impl<E> IntoAnyElement<E::ViewState> for ClickableElement<E>
where
    E: Styled + Element,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E> Element for ClickableElement<E>
where
    E: Styled + Element,
{
    type ViewState = E::ViewState;
    type ElementState = ClickableElementState<E::ElementState>;

    fn id(&self) -> Option<ElementId> {
        self.child.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        if let Some(element_state) = element_state {
            if element_state.mouse_down.lock().is_some() {
                self.child
                    .style_cascade()
                    .set(self.cascade_slot, Some(self.active_style.clone()));
            }

            let (layout_id, child_state) =
                self.child
                    .layout(state, Some(element_state.child_state), cx);
            (
                layout_id,
                ClickableElementState {
                    mouse_down: element_state.mouse_down,
                    child_state,
                },
            )
        } else {
            let (layout_id, child_state) = self.child.layout(state, None, cx);
            (
                layout_id,
                ClickableElementState {
                    mouse_down: Default::default(),
                    child_state,
                },
            )
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        if !self.listeners.is_empty() || self.active_style.is_some() {
            if let Some(mouse_down) = element_state.mouse_down.lock().clone() {
                self.child
                    .style_cascade()
                    .set(self.cascade_slot, Some(self.active_style.clone()));
                let listeners = self.listeners.clone();
                let mouse_down_mutex = element_state.mouse_down.clone();
                cx.on_mouse_event(move |view, up: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(up.position) {
                        for listener in &*listeners {
                            listener(
                                view,
                                &MouseClickEvent {
                                    down: mouse_down.clone(),
                                    up: up.clone(),
                                },
                                cx,
                            );
                        }
                    }

                    mouse_down_mutex.lock().take();
                    cx.notify();
                });
            } else {
                let mouse_down_mutex = element_state.mouse_down.clone();
                cx.on_mouse_event(move |_view, down: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(down.position) {
                        *mouse_down_mutex.lock() = Some(down.clone());
                        cx.notify();
                    }
                });
            }
        }

        self.child
            .paint(bounds, state, &mut element_state.child_state, cx);
    }
}

impl<E: Styled + IdentifiedElement> IdentifiedElement for ClickableElement<E> {}

impl<E> ParentElement for ClickableElement<E>
where
    E: Styled + ParentElement,
{
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::ViewState>; 2]> {
        self.child.children_mut()
    }

    fn group_mut(&mut self) -> &mut Option<SharedString> {
        self.child.group_mut()
    }
}

impl<E> Styled for ClickableElement<E>
where
    E: Styled + Element,
{
    fn style_cascade(&mut self) -> &mut crate::StyleCascade {
        self.child.style_cascade()
    }

    fn computed_style(&mut self) -> &crate::Style {
        self.child.computed_style()
    }
}

impl<E> Hoverable for ClickableElement<E>
where
    E: Element + Hoverable,
{
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.child.hover_style()
    }
}

impl<E> Clickable for ClickableElement<E>
where
    E: Styled + IdentifiedElement,
{
    fn active_style(&mut self) -> &mut StyleRefinement {
        &mut self.active_style
    }

    fn listeners(&mut self) -> &mut ClickListeners<Self::ViewState> {
        &mut self.listeners
    }
}
