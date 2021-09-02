use crate::{
    fonts::TextStyle,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct LineBox {
    child: ElementBox,
    style: TextStyle,
}

impl LineBox {
    pub fn new(child: ElementBox, style: TextStyle) -> Self {
        Self { child, style }
    }
}

impl Element for LineBox {
    type LayoutState = f32;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let line_height = cx
            .font_cache
            .line_height(self.style.font_id, self.style.font_size);
        let character_height = cx
            .font_cache
            .ascent(self.style.font_id, self.style.font_size)
            + cx.font_cache
                .descent(self.style.font_id, self.style.font_size);
        let child_max = vec2f(constraint.max.x(), character_height);
        let child_size = self.child.layout(
            SizeConstraint::new(constraint.min.min(child_max), child_max),
            cx,
        );
        let size = vec2f(child_size.x(), line_height);
        (size, (line_height - character_height) / 2.)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        padding_top: &mut f32,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(
            bounds.origin() + vec2f(0., *padding_top),
            visible_bounds,
            cx,
        );
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "bounds": bounds.to_json(),
            "style": self.style.to_json(),
            "child": self.child.debug(cx),
        })
    }
}
