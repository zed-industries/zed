use crate::{AnyElement, Element, IntoAnyElement, Style, StyleCascade, StyleRefinement};
use refineable::Refineable;
use smallvec::SmallVec;

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
        let style = Style::from_refinement(&self.style_cascade().merged());
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

struct HoverableState<Child: Styled + Element> {
    hover_style: StyleRefinement,
    child: Child,
}

impl<Child: Styled + Element> HoverableState<Child> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        &mut self.hover_style
    }
}

struct Div<V: 'static + Send + Sync>(HoverableState<LayoutNodeState<V>>);

impl<V: 'static + Send + Sync> LayoutNode<V> for Div<V> {
    fn state(&mut self) -> &mut LayoutNodeState<V> {
        &mut self.0.child
    }
}

impl<V: 'static + Send + Sync> Styled for LayoutNodeState<V> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        &mut self.style_cascade
    }
}

impl<V: 'static + Send + Sync> Styled for Div<V> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        &mut self.0.child.style_cascade
    }
}

impl<V: 'static + Send + Sync> Hoverable for Div<V> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.0.hover_style()
    }
}
