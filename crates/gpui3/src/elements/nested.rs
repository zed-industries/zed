use crate::{
    group_bounds, AnyElement, DispatchPhase, Element, IntoAnyElement, MouseMoveEvent, SharedString,
    Style, StyleCascade, StyleRefinement,
};
use refineable::CascadeSlot;
use smallvec::SmallVec;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

trait LayoutNode<V: 'static + Send + Sync> {
    fn state(&mut self) -> &mut LayoutNodeState<V>;

    fn child(mut self, child: impl IntoAnyElement<V>) -> Self
    where
        Self: Sized,
    {
        self.state().children.push(child.into_any());
        self
    }

    fn children<C, E>(mut self, children: C) -> Self
    where
        C: IntoIterator<Item = E>,
        E: IntoAnyElement<V>,
        Self: Sized,
    {
        for child in children {
            self.state().children.push(child.into_any());
        }
        self
    }
}

struct LayoutNodeState<V: 'static + Send + Sync> {
    style_cascade: StyleCascade,
    computed_style: Option<Style>,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V> IntoAnyElement<V> for LayoutNodeState<V>
where
    V: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: 'static + Send + Sync> Element for LayoutNodeState<V> {
    type ViewState = V;
    type ElementState = ();

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        let layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.layout(state, cx))
            .collect::<Vec<_>>();

        // todo!("pass just the style cascade")
        let style = self.computed_style().clone();
        let layout_id = cx.request_layout(style, layout_ids);
        (layout_id, ())
    }

    fn paint(
        &mut self,
        _: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        _: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) {
        for child in &mut self.children {
            child.paint(state, None, cx);
        }
    }
}

pub trait Styled {
    fn style_cascade(&mut self) -> &mut StyleCascade;
    fn computed_style(&mut self) -> &Style;
}

pub struct StyledElement<E> {
    child: E,
}

impl<E> IntoAnyElement<E::ViewState> for StyledElement<E>
where
    E: Element + Styled,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E: Element + Styled> Element for StyledElement<E> {
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn element_id(&self) -> Option<crate::ElementId> {
        self.child.element_id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        self.child.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) {
        self.child.computed_style().paint(bounds, cx);
        self.child.paint(bounds, state, element_state, cx);
    }
}

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

struct HoverableElement<Child> {
    hover_style: StyleRefinement,
    group: Option<SharedString>,
    cascade_slot: CascadeSlot,
    hovered: Arc<AtomicBool>,
    child: Child,
}

impl<Child: Styled + Element> HoverableElement<Child> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        &mut self.hover_style
    }
}

impl<E> IntoAnyElement<E::ViewState> for HoverableElement<E>
where
    E: Element + Styled,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E> Element for HoverableElement<E>
where
    E: Element + Styled,
{
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn element_id(&self) -> Option<crate::ElementId> {
        self.child.element_id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        self.child.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
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

struct Div<V: 'static + Send + Sync>(HoverableElement<LayoutNodeState<V>>);

impl<V: 'static + Send + Sync> LayoutNode<V> for Div<V> {
    fn state(&mut self) -> &mut LayoutNodeState<V> {
        &mut self.0.child
    }
}

impl<V: 'static + Send + Sync> Styled for LayoutNodeState<V> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        &mut self.style_cascade
    }

    fn computed_style(&mut self) -> &Style {
        self.computed_style
            .get_or_insert_with(|| Style::from(self.style_cascade.merged()))
    }
}

impl<V: 'static + Send + Sync> Styled for Div<V> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        self.0.child.style_cascade()
    }

    fn computed_style(&mut self) -> &Style {
        self.0.child.computed_style()
    }
}

impl<V: 'static + Send + Sync> Hoverable for Div<V> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.0.hover_style()
    }
}
