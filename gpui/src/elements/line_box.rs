use super::{AppContext, Element, MutableAppContext};
use crate::{
    fonts::{FamilyId, FontId, Properties},
    geometry::vector::{vec2f, Vector2F},
    AfterLayoutContext, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};

pub struct LineBox {
    child: Box<dyn Element>,
    family_id: FamilyId,
    font_size: f32,
    font_properties: Properties,
    font_id: Option<FontId>,
    size: Option<Vector2F>,
}

impl LineBox {
    pub fn new(family_id: FamilyId, font_size: f32, child: Box<dyn Element>) -> Self {
        Self {
            child,
            family_id,
            font_size,
            font_properties: Properties::default(),
            font_id: None,
            size: None,
        }
    }
}

impl Element for LineBox {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        match ctx
            .font_cache
            .select_font(self.family_id, &self.font_properties)
        {
            Ok(font_id) => {
                self.font_id = Some(font_id);
                let line_height = ctx.font_cache.bounding_box(font_id, self.font_size).y();
                let child_max = vec2f(
                    constraint.max.x(),
                    ctx.font_cache.ascent(font_id, self.font_size)
                        - ctx.font_cache.descent(font_id, self.font_size),
                );
                let child_size = self.child.layout(
                    SizeConstraint::new(constraint.min.min(child_max), child_max),
                    ctx,
                    app,
                );
                let size = vec2f(child_size.x(), line_height);
                self.size = Some(size);
                size
            }
            Err(error) => {
                log::error!("can't layout LineBox: {}", error);
                self.size = Some(constraint.min);
                constraint.min
            }
        }
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        if let Some(font_id) = self.font_id {
            let descent = ctx.font_cache.descent(font_id, self.font_size);
            self.child.paint(origin + vec2f(0.0, -descent), ctx, app);
        }
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        self.child.dispatch_event(event, ctx, app)
    }
}
