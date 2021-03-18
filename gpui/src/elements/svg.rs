use crate::{
    geometry::vector::Vector2F, AfterLayoutContext, AppContext, Element, Event, EventContext,
    LayoutContext, MutableAppContext, PaintContext, SizeConstraint,
};

pub struct Svg {
    path: String,
    // tree: Option<Rc<usvg::Tree>>,
    size: Option<Vector2F>,
}

impl Svg {
    pub fn new(path: String) -> Self {
        Self {
            path,
            // tree: None,
            size: None,
        }
    }
}

impl Element for Svg {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        _: &AppContext,
    ) -> Vector2F {
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

        // self.size = Some(size);
        // size
        todo!()
    }

    fn after_layout(&mut self, _: &mut AfterLayoutContext, _: &mut MutableAppContext) {}

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, _: &AppContext) {
        // if let Some(tree) = self.tree.as_ref() {
        //     ctx.canvas
        //         .draw_svg(tree, RectF::new(origin, self.size.unwrap()));
        // }
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }

    fn dispatch_event(&self, _: &Event, _: &mut EventContext, _: &AppContext) -> bool {
        false
    }
}
