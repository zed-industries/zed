use crate::{
    color::Color,
    fonts::{HighlightStyle, TextStyle},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::{Line, RunStyle, ShapedBoundary},
    DebugContext, Element, Event, EventContext, FontCache, LayoutContext, PaintContext,
    SizeConstraint, TextLayoutCache,
};
use log::warn;
use serde_json::json;
use std::{borrow::Cow, ops::Range, sync::Arc};

pub struct Text {
    text: String,
    style: TextStyle,
    soft_wrap: bool,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
}

pub struct LayoutState {
    shaped_lines: Vec<Line>,
    wrap_boundaries: Vec<Vec<ShapedBoundary>>,
    line_height: f32,
}

impl Text {
    pub fn new(text: String, style: TextStyle) -> Self {
        Self {
            text,
            style,
            soft_wrap: true,
            highlights: Vec::new(),
        }
    }

    pub fn with_default_color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    pub fn with_highlights(mut self, runs: Vec<(Range<usize>, HighlightStyle)>) -> Self {
        self.highlights = runs;
        self
    }

    pub fn with_soft_wrap(mut self, soft_wrap: bool) -> Self {
        self.soft_wrap = soft_wrap;
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
        // Convert the string and highlight ranges into an iterator of highlighted chunks.
        
        let mut offset = 0;
        let mut highlight_ranges = self.highlights.iter().peekable();
        let chunks = std::iter::from_fn(|| {
            let result;
            if let Some((range, highlight_style)) = highlight_ranges.peek() {
                if offset < range.start {
                    result = Some((&self.text[offset..range.start], None));
                    offset = range.start;
                } else if range.end <= self.text.len() {
                    result = Some((&self.text[range.clone()], Some(*highlight_style)));
                    highlight_ranges.next();
                    offset = range.end;
                } else {
                    warn!(
                        "Highlight out of text range. Text len: {}, Highlight range: {}..{}",
                        self.text.len(),
                        range.start,
                        range.end);
                    result = None;
                }
            } else if offset < self.text.len() {
                result = Some((&self.text[offset..], None));
                offset = self.text.len();
            } else {
                result = None;
            }
            result
        });

        // Perform shaping on these highlighted chunks
        let shaped_lines = layout_highlighted_chunks(
            chunks,
            &self.style,
            cx.text_layout_cache,
            &cx.font_cache,
            usize::MAX,
            self.text.matches('\n').count() + 1,
        );

        // If line wrapping is enabled, wrap each of the shaped lines.
        let font_id = self.style.font_id;
        let mut line_count = 0;
        let mut max_line_width = 0_f32;
        let mut wrap_boundaries = Vec::new();
        let mut wrapper = cx.font_cache.line_wrapper(font_id, self.style.font_size);
        for (line, shaped_line) in self.text.split('\n').zip(&shaped_lines) {
            if self.soft_wrap {
                let boundaries = wrapper
                    .wrap_shaped_line(line, shaped_line, constraint.max.x())
                    .collect::<Vec<_>>();
                line_count += boundaries.len() + 1;
                wrap_boundaries.push(boundaries);
            } else {
                line_count += 1;
            }
            max_line_width = max_line_width.max(shaped_line.width());
        }

        let line_height = cx.font_cache.line_height(self.style.font_size);
        let size = vec2f(
            max_line_width
                .ceil()
                .max(constraint.min.x())
                .min(constraint.max.x()),
            (line_height * line_count as f32).ceil(),
        );
        (
            size,
            LayoutState {
                shaped_lines,
                wrap_boundaries,
                line_height,
            },
        )
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let mut origin = bounds.origin();
        let empty = Vec::new();
        for (ix, line) in layout.shaped_lines.iter().enumerate() {
            let wrap_boundaries = layout.wrap_boundaries.get(ix).unwrap_or(&empty);
            let boundaries = RectF::new(
                origin,
                vec2f(
                    bounds.width(),
                    (wrap_boundaries.len() + 1) as f32 * layout.line_height,
                ),
            );

            if boundaries.intersects(visible_bounds) {
                if self.soft_wrap {
                    line.paint_wrapped(
                        origin,
                        visible_bounds,
                        layout.line_height,
                        wrap_boundaries.iter().copied(),
                        cx,
                    );
                } else {
                    line.paint(origin, visible_bounds, layout.line_height, cx);
                }
            }
            origin.set_y(boundaries.max_y());
        }
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
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

/// Perform text layout on a series of highlighted chunks of text.
pub fn layout_highlighted_chunks<'a>(
    chunks: impl Iterator<Item = (&'a str, Option<HighlightStyle>)>,
    text_style: &'a TextStyle,
    text_layout_cache: &'a TextLayoutCache,
    font_cache: &'a Arc<FontCache>,
    max_line_len: usize,
    max_line_count: usize,
) -> Vec<Line> {
    let mut layouts = Vec::with_capacity(max_line_count);
    let mut line = String::new();
    let mut styles = Vec::new();
    let mut row = 0;
    let mut line_exceeded_max_len = false;
    for (chunk, highlight_style) in chunks.chain([("\n", Default::default())]) {
        for (ix, mut line_chunk) in chunk.split('\n').enumerate() {
            if ix > 0 {
                layouts.push(text_layout_cache.layout_str(&line, text_style.font_size, &styles));
                line.clear();
                styles.clear();
                row += 1;
                line_exceeded_max_len = false;
                if row == max_line_count {
                    return layouts;
                }
            }

            if !line_chunk.is_empty() && !line_exceeded_max_len {
                let text_style = if let Some(style) = highlight_style {
                    text_style
                        .clone()
                        .highlight(style, font_cache)
                        .map(Cow::Owned)
                        .unwrap_or_else(|_| Cow::Borrowed(text_style))
                } else {
                    Cow::Borrowed(text_style)
                };

                if line.len() + line_chunk.len() > max_line_len {
                    let mut chunk_len = max_line_len - line.len();
                    while !line_chunk.is_char_boundary(chunk_len) {
                        chunk_len -= 1;
                    }
                    line_chunk = &line_chunk[..chunk_len];
                    line_exceeded_max_len = true;
                }

                line.push_str(line_chunk);
                styles.push((
                    line_chunk.len(),
                    RunStyle {
                        font_id: text_style.font_id,
                        color: text_style.color,
                        underline: text_style.underline,
                    },
                ));
            }
        }
    }

    layouts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        elements::Empty, fonts, ElementBox, Entity, MutableAppContext, RenderContext, View,
    };

    #[crate::test(self)]
    fn test_soft_wrapping_with_carriage_returns(cx: &mut MutableAppContext) {
        let (window_id, _) = cx.add_window(Default::default(), |_| TestView);
        let mut presenter = cx.build_presenter(window_id, Default::default());
        fonts::with_font_cache(cx.font_cache().clone(), || {
            let mut text = Text::new("Hello\r\n".into(), Default::default()).with_soft_wrap(true);
            let (_, state) = text.layout(
                SizeConstraint::new(Default::default(), vec2f(f32::INFINITY, f32::INFINITY)),
                &mut presenter.build_layout_context(Default::default(), false, cx),
            );
            assert_eq!(state.shaped_lines.len(), 2);
            assert_eq!(state.wrap_boundaries.len(), 2);
        });
    }

    struct TestView;

    impl Entity for TestView {
        type Event = ();
    }

    impl View for TestView {
        fn ui_name() -> &'static str {
            "TestView"
        }

        fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
            Empty::new().boxed()
        }
    }
}
