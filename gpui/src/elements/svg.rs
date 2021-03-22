use crate::{
    geometry::vector::Vector2F, AfterLayoutContext, Element, Event, EventContext, LayoutContext,
    PaintContext, SizeConstraint,
};

pub struct Svg {
    path: String,
}

impl Svg {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Element for Svg {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        _: SizeConstraint,
        _: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        // let size;
        // match ctx.asset_cache.svg(&self.path) {
        //     Ok(tree) => {
        //         size = if constraint.max.x().is_infinite() && constraint.max.y().is_infinite() {
        //             let rect = usvg_rect_to_euclid_rect(&tree.svg_node().view_box.rect);
        //             rect.size()
        //         } else {
        //             let max_size = constraint.max;
        //             let svg_size = usvg_rect_to_euclid_rect(&tree.svg_node().view_box.rect).size();

        //             if max_size.x().is_infinite()
        //                 || max_size.x() / max_size.y() > svg_size.x() / svg_size.y()
        //             {
        //                 vec2f(svg_size.x() * max_size.y() / svg_size.y(), max_size.y())
        //             } else {
        //                 vec2f(max_size.x(), svg_size.y() * max_size.x() / svg_size.x())
        //             }
        //         };
        //         self.tree = Some(tree);
        //     }
        //     Err(error) => {
        //         log::error!("{}", error);
        //         size = constraint.min;
        //     }
        // };

        // size

        todo!()
    }

    fn after_layout(&mut self, _: Vector2F, _: &mut Self::LayoutState, _: &mut AfterLayoutContext) {
    }

    fn paint(
        &mut self,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut PaintContext,
    ) -> Self::PaintState {
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut EventContext,
    ) -> bool {
        false
    }
}
