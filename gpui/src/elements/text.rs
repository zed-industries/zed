use crate::{
    color::Color,
    fonts::TextStyle,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::{Line, ShapedBoundary},
    DebugContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use serde_json::json;

pub struct Text {
    text: String,
    style: TextStyle,
}

pub struct LayoutState {
    lines: Vec<(Line, Vec<ShapedBoundary>)>,
    line_height: f32,
}

impl Text {
    pub fn new(text: String, style: TextStyle) -> Self {
        Self { text, style }
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
        let font_id = self.style.font_id;
        let line_height = cx.font_cache.line_height(font_id, self.style.font_size);

        let mut wrapper = cx.font_cache.line_wrapper(font_id, self.style.font_size);
        let mut lines = Vec::new();
        let mut line_count = 0;
        let mut max_line_width = 0_f32;
        for line in self.text.lines() {
            let shaped_line = cx.text_layout_cache.layout_str(
                line,
                self.style.font_size,
                &[(line.len(), font_id, self.style.color)],
            );
            let wrap_boundaries = wrapper
                .wrap_shaped_line(line, &shaped_line, constraint.max.x())
                .collect::<Vec<_>>();

            max_line_width = max_line_width.max(shaped_line.width());
            line_count += wrap_boundaries.len() + 1;
            lines.push((shaped_line, wrap_boundaries));
        }

        let size = vec2f(
            max_line_width
                .ceil()
                .max(constraint.min.x())
                .min(constraint.max.x()),
            (line_height * line_count as f32).ceil(),
        );
        (size, LayoutState { lines, line_height })
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let mut origin = bounds.origin();
        for (line, wrap_boundaries) in &layout.lines {
            let wrapped_line_boundaries = RectF::new(
                origin,
                vec2f(
                    bounds.width(),
                    (wrap_boundaries.len() + 1) as f32 * layout.line_height,
                ),
            );

            if wrapped_line_boundaries.intersects(visible_bounds) {
                line.paint_wrapped(
                    origin,
                    visible_bounds,
                    layout.line_height,
                    wrap_boundaries.iter().copied(),
                    cx,
                );
            }
            origin.set_y(wrapped_line_boundaries.max_y());
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

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> Value {
        json!({
            "type": "Text",
            "bounds": bounds.to_json(),
            "text": &self.text,
            "style": self.style.to_json(),
        })
    }
}
