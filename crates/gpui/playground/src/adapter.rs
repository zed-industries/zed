use crate::element::{LayoutContext, PaintContext};
use gpui::{geometry::rect::RectF, LayoutEngine};
use util::ResultExt;

use crate::element::AnyElement;

pub struct Adapter<V>(pub(crate) AnyElement<V>);

impl<V: 'static> gpui::Element<V> for Adapter<V> {
    type LayoutState = Option<LayoutEngine>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        cx.push_layout_engine(LayoutEngine::new());
        let node = self.0.layout(view, cx).log_err();

        if let Some(node) = node {
            let layout_engine = cx.layout_engine().unwrap();
            layout_engine.compute_layout(node, constraint.max).log_err();
        }
        let layout_engine = cx.pop_layout_engine();
        debug_assert!(layout_engine.is_some());
        (constraint.max, layout_engine)
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout_engine: &mut Option<LayoutEngine>,
        view: &mut V,
        legacy_cx: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        legacy_cx.push_layout_engine(layout_engine.take().unwrap());
        let mut cx = PaintContext::new(legacy_cx, scene);
        self.0.paint(view, &mut cx).log_err();
        *layout_engine = legacy_cx.pop_layout_engine();
        debug_assert!(layout_engine.is_some());
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
