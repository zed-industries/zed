use gpui::{Element, View, Axis, AnyElement};

// Model for the center group: AdjustableGroup of AdjustableGroups
// Implementation notes
// - These have two representations: Exact pixel widths and ratios of elements compared to whole space
// - We have a constraint of minimum sizes for things.
//   - If The space is smaller than allowed, things run off the edge
// - When doing Drag resize, we update the pixel width representation, causing a recalc of the ratios
//   - If dragging past minimum, take space from next item, until out of space
// - When doing a reflow (e.g. layout) we read off the ratios and calculate pixels from that
// - When adding / removing items in an Adjustable flex, reset to default ratios (1:1)
// - By default, every item takes up as much space as possible
//


struct AdjustableFlex<V: View> {
    axis: Axis,
    handle_size: f32,
    items: Vec<(AnyElement<V>, f32)>
}

impl<V: View> AdjustableFlex<V> {
    fn new(axis: Axis) -> Self {
        AdjustableFlex {
            axis,
            handle_size: 2.,
            items: Vec::new(),
        }
    }

    fn add_item()
}

impl<V: View> Element<V> for AdjustableFlex<V> {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut gpui::LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        todo!()
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut gpui::ViewContext<V>,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> Option<gpui::geometry::rect::RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> serde_json::Value {
        todo!()
    }
}
