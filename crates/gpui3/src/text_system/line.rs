use crate::{
    black, point, px, Bounds, FontId, Hsla, Layout, MonochromeSprite, Pixels, Point,
    RasterizedGlyphId, RunStyle, ShapedBoundary, ShapedLine, ShapedRun, UnderlineStyle,
    WindowContext,
};
use anyhow::Result;
use smallvec::SmallVec;
use std::sync::Arc;
use util::ResultExt;

#[derive(Default, Debug, Clone)]
pub struct Line {
    layout: Arc<ShapedLine>,
    style_runs: SmallVec<[StyleRun; 32]>,
}

#[derive(Debug, Clone)]
struct StyleRun {
    len: u32,
    color: Hsla,
    underline: UnderlineStyle,
}

impl Line {
    pub fn new(layout: Arc<ShapedLine>, runs: &[(usize, RunStyle)]) -> Self {
        let mut style_runs = SmallVec::new();
        for (len, style) in runs {
            style_runs.push(StyleRun {
                len: *len as u32,
                color: style.color,
                underline: style.underline.clone().unwrap_or_default(),
            });
        }
        Self { layout, style_runs }
    }

    pub fn runs(&self) -> &[ShapedRun] {
        &self.layout.runs
    }

    pub fn width(&self) -> Pixels {
        self.layout.width
    }

    pub fn font_size(&self) -> Pixels {
        self.layout.font_size
    }

    pub fn x_for_index(&self, index: usize) -> Pixels {
        for run in &self.layout.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return glyph.position.x;
                }
            }
        }
        self.layout.width
    }

    pub fn font_for_index(&self, index: usize) -> Option<FontId> {
        for run in &self.layout.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return Some(run.font_id);
                }
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.layout.len
    }

    pub fn is_empty(&self) -> bool {
        self.layout.len == 0
    }

    pub fn index_for_x(&self, x: Pixels) -> Option<usize> {
        if x >= self.layout.width {
            None
        } else {
            for run in self.layout.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    // todo!
    pub fn paint(
        &self,
        layout: &Layout,
        visible_bounds: Bounds<Pixels>,
        line_height: Pixels,
        cx: &mut WindowContext,
    ) -> Result<()> {
        let origin = layout.bounds.origin;
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_offset = point(px(0.), padding_top + self.layout.ascent);

        let mut style_runs = self.style_runs.iter();
        let mut run_end = 0;
        let mut color = black();
        let mut underline = None;
        let text_system = cx.text_system().clone();

        for run in &self.layout.runs {
            text_system.with_font(run.font_id, |system, font| {
                let max_glyph_width = system.bounding_box(font, self.layout.font_size)?.size.width;

                for glyph in &run.glyphs {
                    let glyph_origin = origin + baseline_offset + glyph.position;
                    if glyph_origin.x > visible_bounds.upper_right().x {
                        break;
                    }

                    let mut finished_underline: Option<(Point<Pixels>, UnderlineStyle)> = None;
                    if glyph.index >= run_end {
                        if let Some(style_run) = style_runs.next() {
                            if let Some((_, underline_style)) = &mut underline {
                                if style_run.underline != *underline_style {
                                    finished_underline = underline.take();
                                }
                            }
                            if style_run.underline.thickness > px(0.) {
                                underline.get_or_insert((
                                    point(
                                        glyph_origin.x,
                                        origin.y
                                            + baseline_offset.y
                                            + (self.layout.descent * 0.618),
                                    ),
                                    UnderlineStyle {
                                        color: style_run.underline.color,
                                        thickness: style_run.underline.thickness,
                                        squiggly: style_run.underline.squiggly,
                                    },
                                ));
                            }

                            run_end += style_run.len as usize;
                            color = style_run.color;
                        } else {
                            run_end = self.layout.len;
                            finished_underline = underline.take();
                        }
                    }

                    if glyph_origin.x + max_glyph_width < visible_bounds.origin.x {
                        continue;
                    }

                    if let Some((_underline_origin, _underline_style)) = finished_underline {
                        todo!()
                        // cx.scene().insert(Underline {
                        //     origin: underline_origin,
                        //     width: glyph_origin.x - underline_origin.x,
                        //     thickness: underline_style.thickness.into(),
                        //     color: underline_style.color.unwrap(),
                        //     squiggly: underline_style.squiggly,
                        // });
                    }

                    if glyph.is_emoji {
                        todo!()
                        // cx.scene().push_image_glyph(scene::ImageGlyph {
                        //     font_id: run.font_id,
                        //     font_size: self.layout.font_size,
                        //     id: glyph.id,
                        //     origin: glyph_origin,
                        // });
                    } else {
                        if let Some((tile, bounds)) = cx
                            .rasterize_glyph(
                                run.font_id,
                                glyph.id,
                                self.layout.font_size,
                                cx.scale_factor(),
                                glyph_origin,
                            )
                            .log_err()
                        {
                            let layer_id = cx.current_layer_id();
                            cx.scene().insert(
                                layer_id,
                                MonochromeSprite {
                                    order: layout.order,
                                    bounds,
                                    color,
                                    tile,
                                },
                            );
                        }

                        // cx.scene().insert(Symbol {
                        //     order: layout.order,
                        //     origin,
                        //     font_id: run.font_id,
                        //     font_size: self.layout.font_size,
                        //     id: glyph.id,
                        //     color,
                        // });
                    }
                }

                anyhow::Ok(())
            })??;
        }

        if let Some((_underline_start, _underline_style)) = underline.take() {
            let _line_end_x = origin.x + self.layout.width;
            // cx.scene().push_underline(Underline {
            //     origin: underline_start,
            //     width: line_end_x - underline_start.x,
            //     color: underline_style.color,
            //     thickness: underline_style.thickness.into(),
            //     squiggly: underline_style.squiggly,
            // });
        }

        Ok(())
    }

    pub fn paint_wrapped(
        &self,
        origin: Point<Pixels>,
        _visible_bounds: Bounds<Pixels>,
        line_height: Pixels,
        boundaries: &[ShapedBoundary],
        cx: &mut WindowContext,
    ) -> Result<()> {
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_offset = point(px(0.), padding_top + self.layout.ascent);

        let mut boundaries = boundaries.into_iter().peekable();
        let mut color_runs = self.style_runs.iter();
        let mut style_run_end = 0;
        let mut _color = black(); // todo!
        let mut underline: Option<(Point<Pixels>, UnderlineStyle)> = None;

        let mut glyph_origin = origin;
        let mut prev_position = px(0.);
        for (run_ix, run) in self.layout.runs.iter().enumerate() {
            for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
                glyph_origin.x += glyph.position.x - prev_position;

                if boundaries
                    .peek()
                    .map_or(false, |b| b.run_ix == run_ix && b.glyph_ix == glyph_ix)
                {
                    boundaries.next();
                    if let Some((_underline_origin, _underline_style)) = underline.take() {
                        // cx.scene().push_underline(Underline {
                        //     origin: underline_origin,
                        //     width: glyph_origin.x - underline_origin.x,
                        //     thickness: underline_style.thickness.into(),
                        //     color: underline_style.color.unwrap(),
                        //     squiggly: underline_style.squiggly,
                        // });
                    }

                    glyph_origin = point(origin.x, glyph_origin.y + line_height);
                }
                prev_position = glyph.position.x;

                let mut finished_underline = None;
                if glyph.index >= style_run_end {
                    if let Some(style_run) = color_runs.next() {
                        style_run_end += style_run.len as usize;
                        _color = style_run.color;
                        if let Some((_, underline_style)) = &mut underline {
                            if style_run.underline != *underline_style {
                                finished_underline = underline.take();
                            }
                        }
                        if style_run.underline.thickness > px(0.) {
                            underline.get_or_insert((
                                glyph_origin
                                    + point(
                                        px(0.),
                                        baseline_offset.y + (self.layout.descent * 0.618),
                                    ),
                                UnderlineStyle {
                                    color: Some(
                                        style_run.underline.color.unwrap_or(style_run.color),
                                    ),
                                    thickness: style_run.underline.thickness,
                                    squiggly: style_run.underline.squiggly,
                                },
                            ));
                        }
                    } else {
                        style_run_end = self.layout.len;
                        _color = black();
                        finished_underline = underline.take();
                    }
                }

                if let Some((_underline_origin, _underline_style)) = finished_underline {
                    // cx.scene().push_underline(Underline {
                    //     origin: underline_origin,
                    //     width: glyph_origin.x - underline_origin.x,
                    //     thickness: underline_style.thickness.into(),
                    //     color: underline_style.color.unwrap(),
                    //     squiggly: underline_style.squiggly,
                    // });
                }

                cx.text_system().with_font(run.font_id, |system, font| {
                    let _glyph_bounds = Bounds {
                        origin: glyph_origin,
                        size: system.bounding_box(font, self.layout.font_size)?.size,
                    };
                    // if glyph_bounds.intersects(visible_bounds) {
                    //     if glyph.is_emoji {
                    //         cx.scene().push_image_glyph(scene::ImageGlyph {
                    //             font_id: run.font_id,
                    //             font_size: self.layout.font_size,
                    //             id: glyph.id,
                    //             origin: glyph_bounds.origin() + baseline_offset,
                    //         });
                    //     } else {
                    //         cx.scene().push_glyph(scene::Glyph {
                    //             font_id: run.font_id,
                    //             font_size: self.layout.font_size,
                    //             id: glyph.id,
                    //             origin: glyph_bounds.origin() + baseline_offset,
                    //             color,
                    //         });
                    //     }
                    // }
                    anyhow::Ok(())
                })??;
            }
        }

        if let Some((_underline_origin, _underline_style)) = underline.take() {
            // let line_end_x = glyph_origin.x + self.layout.width - prev_position;
            // cx.scene().push_underline(Underline {
            //     origin: underline_origin,
            //     width: line_end_x - underline_origin.x,
            //     thickness: underline_style.thickness.into(),
            //     color: underline_style.color,
            //     squiggly: underline_style.squiggly,
            // });
        }

        Ok(())
    }
}
