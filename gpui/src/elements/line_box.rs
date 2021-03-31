use crate::{
    font_cache::FamilyId,
    fonts::{FontId, Properties},
    geometry::vector::{vec2f, Vector2F},
    AfterLayoutContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct LineBox {
    child: ElementBox,
    family_id: FamilyId,
    font_size: f32,
    font_properties: Properties,
}

impl LineBox {
    pub fn new(family_id: FamilyId, font_size: f32, child: ElementBox) -> Self {
        Self {
            child,
            family_id,
            font_size,
            font_properties: Properties::default(),
        }
    }
}

impl Element for LineBox {
    type LayoutState = Option<FontId>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        match ctx
            .font_cache
            .select_font(self.family_id, &self.font_properties)
        {
            Ok(font_id) => {
                let line_height = ctx
                    .font_cache
                    .bounding_box(font_id, self.font_size)
                    .y()
                    .ceil();
                let child_max = vec2f(
                    constraint.max.x(),
                    ctx.font_cache.ascent(font_id, self.font_size)
                        - ctx.font_cache.descent(font_id, self.font_size),
                );
                let child_size = self.child.layout(
                    SizeConstraint::new(constraint.min.min(child_max), child_max),
                    ctx,
                );
                let size = vec2f(child_size.x(), line_height);
                (size, Some(font_id))
            }
            Err(error) => {
                log::error!("can't find font for LineBox: {}", error);
                (constraint.min, None)
            }
        }
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        ctx: &mut AfterLayoutContext,
    ) {
        self.child.after_layout(ctx);
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        font_id: &mut Option<FontId>,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        if let Some(font_id) = font_id {
            let descent = ctx.font_cache.descent(*font_id, self.font_size);
            self.child
                .paint(bounds.origin() + vec2f(0.0, -descent), ctx);
        }
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, ctx)
    }
}
