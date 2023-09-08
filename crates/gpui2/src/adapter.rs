use crate::{paint_context::PaintContext, ViewContext};
use gpui::{geometry::rect::RectF, LayoutEngine, LayoutId};
use util::ResultExt;

/// Makes a new, gpui2-style element into a legacy element.
pub struct AdapterElement<V>(pub(crate) crate::element::AnyElement<V>);

impl<V: 'static> gpui::Element<V> for AdapterElement<V> {
    type LayoutState = Option<(LayoutEngine, LayoutId)>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut gpui::LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        cx.push_layout_engine(LayoutEngine::new());

        let mut cx = ViewContext::new(cx);
        let layout_id = self.0.layout(view, &mut cx).log_err();
        if let Some(layout_id) = layout_id {
            cx.layout_engine()
                .unwrap()
                .compute_layout(layout_id, constraint.max)
                .log_err();
        }

        let layout_engine = cx.pop_layout_engine();
        debug_assert!(layout_engine.is_some(),
            "unexpected layout stack state. is there an unmatched pop_layout_engine in the called code?"
        );

        (constraint.max, layout_engine.zip(layout_id))
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _visible_bounds: RectF,
        layout_data: &mut Option<(LayoutEngine, LayoutId)>,
        view: &mut V,
        legacy_cx: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        let (layout_engine, layout_id) = layout_data.take().unwrap();
        legacy_cx.push_layout_engine(layout_engine);
        let mut cx = PaintContext::new(legacy_cx);
        self.0.paint(view, bounds.origin(), &mut cx);
        *layout_data = legacy_cx.pop_layout_engine().zip(Some(layout_id));
        debug_assert!(layout_data.is_some());
    }

    fn rect_for_text_range(
        &self,
        _range_utf16: std::ops::Range<usize>,
        _bounds: RectF,
        _visible_bounds: RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _view: &V,
        _cx: &gpui::ViewContext<V>,
    ) -> Option<RectF> {
        todo!("implement before merging to main")
    }

    fn debug(
        &self,
        _bounds: RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _view: &V,
        _cx: &gpui::ViewContext<V>,
    ) -> gpui::serde_json::Value {
        todo!("implement before merging to main")
    }
}
