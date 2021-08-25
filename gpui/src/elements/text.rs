use crate::{
    color::Color,
    font_cache::FamilyId,
    fonts::{FontId, TextStyle},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::Line,
    DebugContext, Element, Event, EventContext, FontCache, LayoutContext, PaintContext,
    SizeConstraint,
};
use serde::Deserialize;
use serde_json::json;
use smallvec::{smallvec, SmallVec};

pub struct Text {
    text: String,
    family_id: FamilyId,
    font_size: f32,
    style: TextStyle,
}

impl Text {
    pub fn new(text: String, family_id: FamilyId, font_size: f32) -> Self {
        Self {
            text,
            family_id,
            font_size,
            style: Default::default(),
        }
    }

    pub fn with_style(mut self, style: &TextStyle) -> Self {
        self.style = style.clone();
        self
    }

    pub fn with_default_color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }
}

impl Element for Text {
    type LayoutState = Line;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let font_id = cx
            .font_cache
            .select_font(self.family_id, &self.style.font_properties)
            .unwrap();

        todo!()
        // let line =
        //     cx.text_layout_cache
        //         .layout_str(self.text.as_str(), self.font_size, runs.as_slice());

        // let size = vec2f(
        //     line.width().max(constraint.min.x()).min(constraint.max.x()),
        //     cx.font_cache.line_height(font_id, self.font_size).ceil(),
        // );

        // (size, line)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        line: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        line.paint(
            bounds.origin(),
            RectF::new(vec2f(0., 0.), bounds.size()),
            cx,
        )
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

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> Value {
        json!({
            "type": "Label",
            "bounds": bounds.to_json(),
            "text": &self.text,
            "font_family": cx.font_cache.family_name(self.family_id).unwrap(),
            "font_size": self.font_size,
            "style": self.style.to_json(),
        })
    }
}
