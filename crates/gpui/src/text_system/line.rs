use crate::{
    black, fill, point, px, size, Bounds, ElementContext, Hsla, LineLayout, Pixels, Point, Result,
    SharedString, UnderlineStyle, WrapBoundary, WrappedLineLayout,
};
use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DecorationRun {
    pub len: u32,
    pub color: Hsla,
    pub background_color: Option<Hsla>,
    pub underline: Option<UnderlineStyle>,
}

#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct ShapedLine {
    #[deref]
    #[deref_mut]
    pub(crate) layout: Arc<LineLayout>,
    pub text: SharedString,
    pub(crate) decoration_runs: SmallVec<[DecorationRun; 32]>,
}

impl ShapedLine {
    /// The length of the line in utf-8 bytes.
    pub fn len(&self) -> usize {
        self.layout.len
    }

    pub fn paint(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        cx: &mut ElementContext,
    ) -> Result<()> {
        paint_line(
            origin,
            &self.layout,
            line_height,
            &self.decoration_runs,
            &[],
            cx,
        )?;

        Ok(())
    }
}

#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct WrappedLine {
    #[deref]
    #[deref_mut]
    pub(crate) layout: Arc<WrappedLineLayout>,
    pub text: SharedString,
    pub(crate) decoration_runs: SmallVec<[DecorationRun; 32]>,
}

impl WrappedLine {
    pub fn len(&self) -> usize {
        self.layout.len()
    }

    pub fn paint(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        cx: &mut ElementContext,
    ) -> Result<()> {
        paint_line(
            origin,
            &self.layout.unwrapped_layout,
            line_height,
            &self.decoration_runs,
            &self.wrap_boundaries,
            cx,
        )?;

        Ok(())
    }
}

fn paint_line(
    origin: Point<Pixels>,
    layout: &LineLayout,
    line_height: Pixels,
    decoration_runs: &[DecorationRun],
    wrap_boundaries: &[WrapBoundary],
    cx: &mut ElementContext<'_>,
) -> Result<()> {
    let padding_top = (line_height - layout.ascent - layout.descent) / 2.;
    let baseline_offset = point(px(0.), padding_top + layout.ascent);
    let mut decoration_runs = decoration_runs.iter();
    let mut wraps = wrap_boundaries.iter().peekable();
    let mut run_end = 0;
    let mut color = black();
    let mut current_underline: Option<(Point<Pixels>, UnderlineStyle)> = None;
    let mut current_background: Option<(Point<Pixels>, Hsla)> = None;
    let text_system = cx.text_system().clone();
    let mut glyph_origin = origin;
    let mut prev_glyph_position = Point::default();
    for (run_ix, run) in layout.runs.iter().enumerate() {
        let max_glyph_size = text_system.bounding_box(run.font_id, layout.font_size).size;

        for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
            glyph_origin.x += glyph.position.x - prev_glyph_position.x;

            if wraps.peek() == Some(&&WrapBoundary { run_ix, glyph_ix }) {
                wraps.next();
                if let Some((background_origin, background_color)) = current_background.as_mut() {
                    cx.paint_quad(fill(
                        Bounds {
                            origin: *background_origin,
                            size: size(glyph_origin.x - background_origin.x, line_height),
                        },
                        *background_color,
                    ));
                    background_origin.x = origin.x;
                    background_origin.y += line_height;
                }
                if let Some((underline_origin, underline_style)) = current_underline.as_mut() {
                    cx.paint_underline(
                        *underline_origin,
                        glyph_origin.x - underline_origin.x,
                        underline_style,
                    );
                    underline_origin.x = origin.x;
                    underline_origin.y += line_height;
                }

                glyph_origin.x = origin.x;
                glyph_origin.y += line_height;
            }
            prev_glyph_position = glyph.position;

            let mut finished_background: Option<(Point<Pixels>, Hsla)> = None;
            let mut finished_underline: Option<(Point<Pixels>, UnderlineStyle)> = None;
            if glyph.index >= run_end {
                if let Some(style_run) = decoration_runs.next() {
                    if let Some((_, background_color)) = &mut current_background {
                        if style_run.background_color.as_ref() != Some(background_color) {
                            finished_background = current_background.take();
                        }
                    }
                    if let Some(run_background) = style_run.background_color {
                        current_background
                            .get_or_insert((point(glyph_origin.x, glyph_origin.y), run_background));
                    }

                    if let Some((_, underline_style)) = &mut current_underline {
                        if style_run.underline.as_ref() != Some(underline_style) {
                            finished_underline = current_underline.take();
                        }
                    }
                    if let Some(run_underline) = style_run.underline.as_ref() {
                        current_underline.get_or_insert((
                            point(
                                glyph_origin.x,
                                glyph_origin.y + baseline_offset.y + (layout.descent * 0.618),
                            ),
                            UnderlineStyle {
                                color: Some(run_underline.color.unwrap_or(style_run.color)),
                                thickness: run_underline.thickness,
                                wavy: run_underline.wavy,
                            },
                        ));
                    }

                    run_end += style_run.len as usize;
                    color = style_run.color;
                } else {
                    run_end = layout.len;
                    finished_background = current_background.take();
                    finished_underline = current_underline.take();
                }
            }

            if let Some((background_origin, background_color)) = finished_background {
                cx.paint_quad(fill(
                    Bounds {
                        origin: background_origin,
                        size: size(glyph_origin.x - background_origin.x, line_height),
                    },
                    background_color,
                ));
            }

            if let Some((underline_origin, underline_style)) = finished_underline {
                cx.paint_underline(
                    underline_origin,
                    glyph_origin.x - underline_origin.x,
                    &underline_style,
                );
            }

            let max_glyph_bounds = Bounds {
                origin: glyph_origin,
                size: max_glyph_size,
            };

            let content_mask = cx.content_mask();
            if max_glyph_bounds.intersects(&content_mask.bounds) {
                if glyph.is_emoji {
                    cx.paint_emoji(
                        glyph_origin + baseline_offset,
                        run.font_id,
                        glyph.id,
                        layout.font_size,
                    )?;
                } else {
                    cx.paint_glyph(
                        glyph_origin + baseline_offset,
                        run.font_id,
                        glyph.id,
                        layout.font_size,
                        color,
                    )?;
                }
            }
        }
    }

    let mut last_line_end_x = origin.x + layout.width;
    if let Some(boundary) = wrap_boundaries.last() {
        let run = &layout.runs[boundary.run_ix];
        let glyph = &run.glyphs[boundary.glyph_ix];
        last_line_end_x -= glyph.position.x;
    }

    if let Some((background_origin, background_color)) = current_background.take() {
        cx.paint_quad(fill(
            Bounds {
                origin: background_origin,
                size: size(last_line_end_x - background_origin.x, line_height),
            },
            background_color,
        ));
    }

    if let Some((underline_start, underline_style)) = current_underline.take() {
        cx.paint_underline(
            underline_start,
            last_line_end_x - underline_start.x,
            &underline_style,
        );
    }

    Ok(())
}
