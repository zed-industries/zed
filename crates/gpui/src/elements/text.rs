use crate::{
    color::Color,
    fonts::{HighlightStyle, TextStyle},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::{Line, RunStyle, ShapedBoundary},
    Element, FontCache, PaintContext, SizeConstraint, TextLayoutCache, ViewContext, WindowContext,
};
use log::warn;
use serde_json::json;
use std::{borrow::Cow, ops::Range, sync::Arc};

pub struct Text {
    text: Cow<'static, str>,
    style: TextStyle,
    soft_wrap: bool,
    highlights: Option<Box<[(Range<usize>, HighlightStyle)]>>,
    custom_runs: Option<(
        Box<[Range<usize>]>,
        Box<dyn FnMut(usize, RectF, &mut WindowContext)>,
    )>,
}

pub struct LayoutState {
    shaped_lines: Vec<Line>,
    wrap_boundaries: Vec<Vec<ShapedBoundary>>,
    line_height: f32,
}

impl Text {
    pub fn new<I: Into<Cow<'static, str>>>(text: I, style: TextStyle) -> Self {
        Self {
            text: text.into(),
            style,
            soft_wrap: true,
            highlights: None,
            custom_runs: None,
        }
    }

    pub fn with_default_color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    pub fn with_highlights(
        mut self,
        runs: impl Into<Box<[(Range<usize>, HighlightStyle)]>>,
    ) -> Self {
        self.highlights = Some(runs.into());
        self
    }

    pub fn with_custom_runs(
        mut self,
        runs: impl Into<Box<[Range<usize>]>>,
        callback: impl 'static + FnMut(usize, RectF, &mut WindowContext),
    ) -> Self {
        self.custom_runs = Some((runs.into(), Box::new(callback)));
        self
    }

    pub fn with_soft_wrap(mut self, soft_wrap: bool) -> Self {
        self.soft_wrap = soft_wrap;
        self
    }
}

impl<V: 'static> Element<V> for Text {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        // Convert the string and highlight ranges into an iterator of highlighted chunks.

        let mut offset = 0;
        let mut highlight_ranges = self
            .highlights
            .as_ref()
            .map_or(Default::default(), AsRef::as_ref)
            .iter()
            .peekable();
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
                        range.end
                    );
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
            cx.text_layout_cache(),
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
        _: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        let mut origin = bounds.origin();
        let empty = Vec::new();
        let mut callback = |_, _, _: &mut WindowContext| {};

        let mouse_runs;
        let custom_run_callback;
        if let Some((runs, build_region)) = &mut self.custom_runs {
            mouse_runs = runs.iter();
            custom_run_callback = build_region.as_mut();
        } else {
            mouse_runs = [].iter();
            custom_run_callback = &mut callback;
        }
        let mut custom_runs = mouse_runs.enumerate().peekable();

        let mut offset = 0;
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
                        wrap_boundaries,
                        cx,
                    );
                } else {
                    line.paint(origin, visible_bounds, layout.line_height, cx);
                }
            }

            // Paint any custom runs that intersect this line.
            let end_offset = offset + line.len();
            if let Some((custom_run_ix, custom_run_range)) = custom_runs.peek().cloned() {
                if custom_run_range.start < end_offset {
                    let mut current_custom_run = None;
                    if custom_run_range.start <= offset {
                        current_custom_run = Some((custom_run_ix, custom_run_range.end, origin));
                    }

                    let mut glyph_origin = origin;
                    let mut prev_position = 0.;
                    let mut wrap_boundaries = wrap_boundaries.iter().copied().peekable();
                    for (run_ix, glyph_ix, glyph) in
                        line.runs().iter().enumerate().flat_map(|(run_ix, run)| {
                            run.glyphs()
                                .iter()
                                .enumerate()
                                .map(move |(ix, glyph)| (run_ix, ix, glyph))
                        })
                    {
                        glyph_origin.set_x(glyph_origin.x() + glyph.position.x() - prev_position);
                        prev_position = glyph.position.x();

                        // If we've reached a soft wrap position, move down one line. If there
                        // is a custom run in-progress, paint it.
                        if wrap_boundaries
                            .peek()
                            .map_or(false, |b| b.run_ix == run_ix && b.glyph_ix == glyph_ix)
                        {
                            if let Some((run_ix, _, run_origin)) = &mut current_custom_run {
                                let bounds = RectF::from_points(
                                    *run_origin,
                                    glyph_origin + vec2f(0., layout.line_height),
                                );
                                custom_run_callback(*run_ix, bounds, cx);
                                *run_origin =
                                    vec2f(origin.x(), glyph_origin.y() + layout.line_height);
                            }
                            wrap_boundaries.next();
                            glyph_origin = vec2f(origin.x(), glyph_origin.y() + layout.line_height);
                        }

                        // If we've reached the end of the current custom run, paint it.
                        if let Some((run_ix, run_end_offset, run_origin)) = current_custom_run {
                            if offset + glyph.index == run_end_offset {
                                current_custom_run.take();
                                let bounds = RectF::from_points(
                                    run_origin,
                                    glyph_origin + vec2f(0., layout.line_height),
                                );
                                custom_run_callback(run_ix, bounds, cx);
                                custom_runs.next();
                            }

                            if let Some((_, run_range)) = custom_runs.peek() {
                                if run_range.start >= end_offset {
                                    break;
                                }
                                if run_range.start == offset + glyph.index {
                                    current_custom_run =
                                        Some((run_ix, run_range.end, glyph_origin));
                                }
                            }
                        }

                        // If we've reached the start of a new custom run, start tracking it.
                        if let Some((run_ix, run_range)) = custom_runs.peek() {
                            if offset + glyph.index == run_range.start {
                                current_custom_run = Some((*run_ix, run_range.end, glyph_origin));
                            }
                        }
                    }

                    // If a custom run extends beyond the end of the line, paint it.
                    if let Some((run_ix, run_end_offset, run_origin)) = current_custom_run {
                        let line_end = glyph_origin + vec2f(line.width() - prev_position, 0.);
                        let bounds = RectF::from_points(
                            run_origin,
                            line_end + vec2f(0., layout.line_height),
                        );
                        custom_run_callback(run_ix, bounds, cx);
                        if end_offset == run_end_offset {
                            custom_runs.next();
                        }
                    }
                }
            }

            offset = end_offset + 1;
            origin.set_y(boundaries.max_y());
        }
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
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
    text_style: &TextStyle,
    text_layout_cache: &TextLayoutCache,
    font_cache: &Arc<FontCache>,
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
    use crate::{elements::Empty, fonts, AnyElement, AppContext, Entity, View, ViewContext};

    #[crate::test(self)]
    fn test_soft_wrapping_with_carriage_returns(cx: &mut AppContext) {
        cx.add_window(Default::default(), |cx| {
            let mut view = TestView;
            fonts::with_font_cache(cx.font_cache().clone(), || {
                let mut text = Text::new("Hello\r\n", Default::default()).with_soft_wrap(true);
                let (_, state) = text.layout(
                    SizeConstraint::new(Default::default(), vec2f(f32::INFINITY, f32::INFINITY)),
                    &mut view,
                    cx,
                );
                assert_eq!(state.shaped_lines.len(), 2);
                assert_eq!(state.wrap_boundaries.len(), 2);
            });
            view
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

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }
}
