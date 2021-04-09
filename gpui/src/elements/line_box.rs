use crate::{
    font_cache::FamilyId,
    fonts::Properties,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    AfterLayoutContext, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext,
    PaintContext, SizeConstraint,
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
    type LayoutState = f32;
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
                let line_height = ctx.font_cache.line_height(font_id, self.font_size);
                let character_height = ctx.font_cache.ascent(font_id, self.font_size)
                    + ctx.font_cache.descent(font_id, self.font_size);
                let child_max = vec2f(constraint.max.x(), character_height);
                let child_size = self.child.layout(
                    SizeConstraint::new(constraint.min.min(child_max), child_max),
                    ctx,
                );
                let size = vec2f(child_size.x(), line_height);
                (size, (line_height - character_height) / 2.)
            }
            Err(error) => {
                log::error!("can't find font for LineBox: {}", error);
                (constraint.min, 0.0)
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
        padding_top: &mut f32,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child
            .paint(bounds.origin() + vec2f(0., *padding_top), ctx);
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

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        ctx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "bounds": bounds.to_json(),
            "font_family": ctx.font_cache.family_name(self.family_id).unwrap(),
            "font_size": self.font_size,
            "font_properties": self.font_properties.to_json(),
            "child": self.child.debug(ctx),
        })
    }
}
