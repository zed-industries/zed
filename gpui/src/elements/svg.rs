use crate::{
    color::ColorU,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    scene, AfterLayoutContext, Element, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct Svg {
    path: String,
    color: ColorU,
}

impl Svg {
    pub fn new(path: String) -> Self {
        Self {
            path,
            color: ColorU::black(),
        }
    }

    pub fn with_color(mut self, color: ColorU) -> Self {
        self.color = color;
        self
    }
}

impl Element for Svg {
    type LayoutState = Option<usvg::Tree>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        match ctx.asset_cache.svg(&self.path) {
            Ok(tree) => {
                let size = if constraint.max.x().is_infinite() && constraint.max.y().is_infinite() {
                    let rect = from_usvg_rect(tree.svg_node().view_box.rect);
                    rect.size()
                } else {
                    let max_size = constraint.max;
                    let svg_size = from_usvg_rect(tree.svg_node().view_box.rect).size();

                    if max_size.x().is_infinite()
                        || max_size.x() / max_size.y() > svg_size.x() / svg_size.y()
                    {
                        vec2f(svg_size.x() * max_size.y() / svg_size.y(), max_size.y())
                    } else {
                        vec2f(max_size.x(), svg_size.y() * max_size.x() / svg_size.x())
                    }
                };
                (size, Some(tree))
            }
            Err(error) => {
                log::error!("{}", error);
                (constraint.min, None)
            }
        }
    }

    fn after_layout(&mut self, _: Vector2F, _: &mut Self::LayoutState, _: &mut AfterLayoutContext) {
    }

    fn paint(&mut self, bounds: RectF, svg: &mut Self::LayoutState, ctx: &mut PaintContext) {
        if let Some(svg) = svg.clone() {
            ctx.scene.push_icon(scene::Icon {
                bounds,
                svg,
                path: self.path.clone(),
                color: self.color,
            });
        }
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut EventContext,
    ) -> bool {
        false
    }
}

fn from_usvg_rect(rect: usvg::Rect) -> RectF {
    RectF::new(
        vec2f(rect.x() as f32, rect.y() as f32),
        vec2f(rect.width() as f32, rect.height() as f32),
    )
}
