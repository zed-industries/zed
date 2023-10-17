use crate::{
    AnonymousElementKind, AnyElement, Bounds, Clickable, ClickableElement, ClickableElementState,
    Element, ElementId, ElementKind, Hoverable, HoverableElement, IdentifiedElementKind,
    IntoAnyElement, LayoutId, LayoutNode, LayoutNodeElement, Overflow, Pixels, Point, SharedString,
    Style, StyleCascade, StyleRefinement, Styled, ViewContext, ClickListeners,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct ScrollState(Arc<Mutex<Point<Pixels>>>);

impl ScrollState {
    pub fn x(&self) -> Pixels {
        self.0.lock().x
    }

    pub fn set_x(&self, value: Pixels) {
        self.0.lock().x = value;
    }

    pub fn y(&self) -> Pixels {
        self.0.lock().y
    }

    pub fn set_y(&self, value: Pixels) {
        self.0.lock().y = value;
    }
}

pub struct Div<V: 'static + Send + Sync, K: ElementKind>(
    ClickableElement<HoverableElement<LayoutNodeElement<V, K>>>,
);

impl<V: 'static + Send + Sync, K: ElementKind> Div<V, K> {
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        *self.0.group_mut() = Some(group.into());
        self
    }

    pub fn z_index(mut self, z_index: u32) -> Self {
        self.base_style().z_index = Some(z_index);
        self
    }

    pub fn overflow_hidden(mut self) -> Self {
        self.base_style().overflow.x = Some(Overflow::Hidden);
        self.base_style().overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_x(mut self) -> Self {
        self.base_style().overflow.x = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_y(mut self) -> Self {
        self.base_style().overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_scroll(mut self, _scroll_state: ScrollState) -> Self {
        // todo!("impl scrolling")
        // self.scroll_state = Some(scroll_state);
        self.base_style().overflow.x = Some(Overflow::Scroll);
        self.base_style().overflow.y = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_x_scroll(mut self, _scroll_state: ScrollState) -> Self {
        // todo!("impl scrolling")
        // self.scroll_state = Some(scroll_state);
        self.base_style().overflow.x = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_y_scroll(mut self, _scroll_state: ScrollState) -> Self {
        // todo!("impl scrolling")
        // self.scroll_state = Some(scroll_state);
        self.base_style().overflow.y = Some(Overflow::Scroll);
        self
    }

    fn base_style(&mut self) -> &mut StyleRefinement {
        self.style_cascade().base()
    }
}

impl<V: 'static + Send + Sync> Div<V, AnonymousElementKind> {
    pub fn id(self, id: impl Into<ElementId>) -> Div<V, IdentifiedElementKind> {
        Div(self.0.replace_child(|hoverable| {
            hoverable.replace_child(|layout_node| layout_node.identify(id))
        }))
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> LayoutNode<V, K> for Div<V, K> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        self.0.children_mut()
    }

    fn group_mut(&mut self) -> &mut Option<SharedString> {
        self.0.group_mut()
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Styled for Div<V, K> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        self.0.style_cascade()
    }

    fn computed_style(&mut self) -> &Style {
        self.0.computed_style()
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Hoverable for Div<V, K> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.0.hover_style()
    }
}

impl<V: 'static + Send + Sync> Clickable for Div<V, IdentifiedElementKind> {
    fn active_style(&mut self) -> &mut StyleRefinement {
        self.0.active_style()
    }

    fn listeners(&mut self) -> &mut ClickListeners<V> {
        self.0.listeners()
    }
}

impl<V, K> IntoAnyElement<V> for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, K> Element for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    type ViewState = V;
    type ElementState = ClickableElementState<()>;

    fn id(&self) -> Option<ElementId> {
        self.0.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.0.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.0.paint(bounds, state, element_state, cx);
    }
}
