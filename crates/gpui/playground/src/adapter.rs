use crate::element::{LayoutContext, PaintContext};
use gpui::geometry::rect::RectF;
use util::ResultExt;

use crate::element::AnyElement;

#[derive(Clone)]
pub struct Adapter<V>(pub(crate) AnyElement<V>);

impl<V: 'static> gpui::Element<V> for Adapter<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        legacy_cx: &mut gpui::LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        legacy_cx.push_layout_engine();
        let node = self
            .0
            .layout(view, &mut LayoutContext { legacy_cx })
            .log_err();

        if let Some(node) = node {
            let layout_engine = legacy_cx.layout_engine().unwrap();
            layout_engine.compute_layout(node, constraint.max).log_err();
        }
        legacy_cx.pop_layout_engine();

        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut (),
        view: &mut V,
        legacy_cx: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        let mut cx = PaintContext { legacy_cx, scene };
        self.0.paint(view, &mut cx).log_err();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> Option<RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> gpui::serde_json::Value {
        todo!()
    }
}
