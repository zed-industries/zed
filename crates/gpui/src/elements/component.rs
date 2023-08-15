use std::marker::PhantomData;

use pathfinder_geometry::{rect::RectF, vector::Vector2F};

use crate::{
    AnyElement, Element, LayoutContext, PaintContext, SceneBuilder, SizeConstraint, View,
    ViewContext,
};

pub trait Component<V: View> {
    fn render(self, v: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V>;

    fn into_element(self) -> ComponentAdapter<V, Self>
    where
        Self: Sized,
    {
        ComponentAdapter::new(self)
    }
}

pub struct ComponentAdapter<V, E> {
    component: Option<E>,
    phantom: PhantomData<V>,
}

impl<E, V> ComponentAdapter<V, E> {
    pub fn new(e: E) -> Self {
        Self {
            component: Some(e),
            phantom: PhantomData,
        }
    }
}

impl<V: View, C: Component<V> + 'static> Element<V> for ComponentAdapter<V, C> {
    type LayoutState = AnyElement<V>;

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let component = self.component.take().unwrap();
        let mut element = component.render(view, cx.view_context());
        let constraint = element.layout(constraint, view, cx);
        (constraint, element)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        layout.paint(scene, bounds.origin(), visible_bounds, view, cx)
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _: RectF,
        _: RectF,
        element: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        element.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        element: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        element.debug(view, cx)
    }
}
