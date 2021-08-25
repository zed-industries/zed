use crate::{
    color::Color,
    font_cache::FamilyId,
    fonts::TextStyle,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::{Line, LineWrapper, ShapedBoundary},
    DebugContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use serde_json::json;

pub struct Text {
    text: String,
    family_id: FamilyId,
    font_size: f32,
    style: TextStyle,
}

pub struct LayoutState {
    line: Line,
    wrap_boundaries: Vec<ShapedBoundary>,
    line_height: f32,
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
    type LayoutState = LayoutState;
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
        let line_height = cx.font_cache.line_height(font_id, self.font_size);
        let line = cx.text_layout_cache.layout_str(
            self.text.as_str(),
            self.font_size,
            &[(self.text.len(), font_id, self.style.color)],
        );
        let mut wrapper = LineWrapper::acquire(font_id, self.font_size, cx.font_system.clone());
        let wrap_boundaries = wrapper
            .wrap_shaped_line(&self.text, &line, constraint.max.x())
            .collect::<Vec<_>>();
        let size = vec2f(
            line.width()
                .ceil()
                .max(constraint.min.x())
                .min(constraint.max.x()),
            (line_height * (wrap_boundaries.len() + 1) as f32).ceil(),
        );
        let layout = LayoutState {
            line,
            wrap_boundaries,
            line_height,
        };

        (size, layout)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        layout.line.paint_wrapped(
            bounds.origin(),
            layout.line_height,
            layout.wrap_boundaries.iter().copied(),
            cx,
        );
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
