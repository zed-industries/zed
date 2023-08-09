use std::ops::Range;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::json,
    AnyElement, Element, LayoutContext, PaintContext, SceneBuilder, SizeConstraint, View,
    ViewContext,
};

pub struct Hook<V: View> {
    child: AnyElement<V>,
    after_layout: Option<Box<dyn FnMut(Vector2F, &mut ViewContext<V>)>>,
}

impl<V: View> Hook<V> {
    pub fn new(child: impl Element<V>) -> Self {
        Self {
            child: child.into_any(),
            after_layout: None,
        }
    }

    pub fn on_after_layout(
        mut self,
        f: impl 'static + FnMut(Vector2F, &mut ViewContext<V>),
    ) -> Self {
        self.after_layout = Some(Box::new(f));
        self
    }
}

impl<V: View> Element<V> for Hook<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, view, cx);
        if let Some(handler) = self.after_layout.as_mut() {
            handler(size, cx);
        }
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) {
        self.child
            .paint(scene, bounds.origin(), visible_bounds, view, cx);
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "type": "Hooks",
            "child": self.child.debug(view, cx),
        })
    }
}
