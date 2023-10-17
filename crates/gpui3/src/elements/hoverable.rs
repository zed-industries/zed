use crate::{
    group_bounds, AnyElement, Bounds, DispatchPhase, Element, ElementId, ElementKind,
    IdentifiedElement, IntoAnyElement, LayoutId, LayoutNode, MouseMoveEvent, Pixels, SharedString,
    Style, StyleCascade, StyleRefinement, Styled, ViewContext,
};
use refineable::CascadeSlot;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

pub trait Hoverable {
    fn hover_style(&mut self) -> &mut StyleRefinement;

    fn hover(mut self, f: impl FnOnce(&mut StyleRefinement) -> &mut StyleRefinement) -> Self
    where
        Self: Sized,
    {
        f(self.hover_style());
        self
    }
}

pub struct HoverableElement<E> {
    hover_style: StyleRefinement,
    group: Option<SharedString>,
    cascade_slot: CascadeSlot,
    hovered: Arc<AtomicBool>,
    child: E,
}

impl<E: Styled + Element> HoverableElement<E> {
    pub fn replace_child<E2: Element<ViewState = E::ViewState>>(
        self,
        replace: impl FnOnce(E) -> E2,
    ) -> HoverableElement<E2> {
        HoverableElement {
            hover_style: self.hover_style,
            group: self.group,
            cascade_slot: self.cascade_slot,
            hovered: self.hovered,
            child: replace(self.child),
        }
    }
}

impl<E> IntoAnyElement<E::ViewState> for HoverableElement<E>
where
    E: Styled + Element,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E> Element for HoverableElement<E>
where
    E: Styled + Element,
{
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn id(&self) -> Option<ElementId> {
        self.child.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.child.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        let target_bounds = self
            .group
            .as_ref()
            .and_then(|group| group_bounds(group, cx))
            .unwrap_or(bounds);

        let hovered = target_bounds.contains_point(cx.mouse_position());

        let slot = self.cascade_slot;
        let style = hovered.then_some(self.hover_style.clone());
        self.child.style_cascade().set(slot, style);
        self.hovered.store(hovered, SeqCst);

        let hovered = self.hovered.clone();
        cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if target_bounds.contains_point(event.position) != hovered.load(SeqCst) {
                    cx.notify();
                }
            }
        });

        self.child.paint(bounds, state, element_state, cx);
    }
}

impl<E, K, V> LayoutNode<V, K> for HoverableElement<E>
where
    E: LayoutNode<V, K>,
    K: ElementKind,
    V: 'static + Send + Sync,
{
    fn children_mut(&mut self) -> &mut smallvec::SmallVec<[AnyElement<V>; 2]> {
        self.child.children_mut()
    }

    fn group_mut(&mut self) -> &mut Option<SharedString> {
        self.child.group_mut()
    }
}

impl<E: Styled + Element> Hoverable for HoverableElement<E> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        &mut self.hover_style
    }
}

impl<E: Styled + Element> Styled for HoverableElement<E> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        self.child.style_cascade()
    }

    fn computed_style(&mut self) -> &Style {
        self.child.computed_style()
    }
}

impl<E: Styled + IdentifiedElement> IdentifiedElement for HoverableElement<E> {}
