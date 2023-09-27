use crate::{
    black, point, px, Bounds, FontId, Glyph, Hsla, LineLayout, Pixels, PlatformTextSystem, Point,
    Run, RunStyle, UnderlineStyle, WindowContext,
};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub(crate) struct TextLayoutCache {
    prev_frame: Mutex<HashMap<CacheKeyValue, Arc<LineLayout>>>,
    curr_frame: RwLock<HashMap<CacheKeyValue, Arc<LineLayout>>>,
    fonts: Arc<dyn PlatformTextSystem>,
}

impl TextLayoutCache {
    pub fn new(fonts: Arc<dyn PlatformTextSystem>) -> Self {
        Self {
            prev_frame: Mutex::new(HashMap::new()),
            curr_frame: RwLock::new(HashMap::new()),
            fonts,
        }
    }

    pub fn finish_frame(&self) {
        let mut prev_frame = self.prev_frame.lock();
        let mut curr_frame = self.curr_frame.write();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.clear();
    }

    pub fn layout_str<'a>(
        &'a self,
        text: &'a str,
        font_size: Pixels,
        runs: &'a [(usize, RunStyle)],
    ) -> Line {
        let key = &CacheKeyRef {
            text,
            font_size,
            runs,
        } as &dyn CacheKey;
        let curr_frame = self.curr_frame.upgradable_read();
        if let Some(layout) = curr_frame.get(key) {
            return Line::new(layout.clone(), runs);
        }

        let mut curr_frame = RwLockUpgradableReadGuard::upgrade(curr_frame);
        if let Some((key, layout)) = self.prev_frame.lock().remove_entry(key) {
            curr_frame.insert(key, layout.clone());
            Line::new(layout, runs)
        } else {
            let layout = Arc::new(self.fonts.layout_line(text, font_size, runs));
            let key = CacheKeyValue {
                text: text.into(),
                font_size,
                runs: SmallVec::from(runs),
            };
            curr_frame.insert(key, layout.clone());
            Line::new(layout, runs)
        }
    }
}

trait CacheKey {
    fn key(&self) -> CacheKeyRef;
}

impl<'a> PartialEq for (dyn CacheKey + 'a) {
    fn eq(&self, other: &dyn CacheKey) -> bool {
        self.key() == other.key()
    }
}

impl<'a> Eq for (dyn CacheKey + 'a) {}

impl<'a> Hash for (dyn CacheKey + 'a) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

#[derive(Eq)]
struct CacheKeyValue {
    text: String,
    font_size: Pixels,
    runs: SmallVec<[(usize, RunStyle); 1]>,
}

impl CacheKey for CacheKeyValue {
    fn key(&self) -> CacheKeyRef {
        CacheKeyRef {
            text: self.text.as_str(),
            font_size: self.font_size,
            runs: self.runs.as_slice(),
        }
    }
}

impl PartialEq for CacheKeyValue {
    fn eq(&self, other: &Self) -> bool {
        self.key().eq(&other.key())
    }
}

impl Hash for CacheKeyValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl<'a> Borrow<dyn CacheKey + 'a> for CacheKeyValue {
    fn borrow(&self) -> &(dyn CacheKey + 'a) {
        self as &dyn CacheKey
    }
}

#[derive(Copy, Clone)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [(usize, RunStyle)],
}

impl<'a> CacheKey for CacheKeyRef<'a> {
    fn key(&self) -> CacheKeyRef {
        *self
    }
}

impl<'a> PartialEq for CacheKeyRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.font_size == other.font_size
            && self.runs.len() == other.runs.len()
            && self.runs.iter().zip(other.runs.iter()).all(
                |((len_a, style_a), (len_b, style_b))| {
                    len_a == len_b && style_a.font == style_b.font
                },
            )
    }
}

impl<'a> Hash for CacheKeyRef<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.font_size.hash(state);
        for (len, style_id) in self.runs {
            len.hash(state);
            style_id.font.hash(state);
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Line {
    layout: Arc<LineLayout>,
    style_runs: SmallVec<[StyleRun; 32]>,
}

#[derive(Debug, Clone)]
struct StyleRun {
    len: u32,
    color: Hsla,
    underline: UnderlineStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapedBoundary {
    pub run_ix: usize,
    pub glyph_ix: usize,
}

impl Line {
    pub fn new(layout: Arc<LineLayout>, runs: &[(usize, RunStyle)]) -> Self {
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

    pub fn runs(&self) -> &[Run] {
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

    pub fn paint(
        &self,
        origin: Point<Pixels>,
        visible_bounds: Bounds<Pixels>,
        line_height: Pixels,
        cx: &mut WindowContext,
    ) {
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_offset = point(px(0.), padding_top + self.layout.ascent);

        let mut style_runs = self.style_runs.iter();
        let mut run_end = 0;
        let mut color = black();
        let mut underline = None;

        for run in &self.layout.runs {
            let max_glyph_width = cx
                .text_system()
                .bounding_box(run.font_id, self.layout.font_size)
                .width;

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
                                    origin.y + baseline_offset.y + (self.layout.descent * 0.618),
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
                    // cx.scene().insert(Underline {
                    //     origin: underline_origin,
                    //     width: glyph_origin.x - underline_origin.x,
                    //     thickness: underline_style.thickness.into(),
                    //     color: underline_style.color.unwrap(),
                    //     squiggly: underline_style.squiggly,
                    // });
                }

                // todo!()
                // if glyph.is_emoji {
                //     cx.scene().push_image_glyph(scene::ImageGlyph {
                //         font_id: run.font_id,
                //         font_size: self.layout.font_size,
                //         id: glyph.id,
                //         origin: glyph_origin,
                //     });
                // } else {
                //     cx.scene().push_glyph(scene::Glyph {
                //         font_id: run.font_id,
                //         font_size: self.layout.font_size,
                //         id: glyph.id,
                //         origin: glyph_origin,
                //         color,
                //     });
                // }
            }
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
    }

    pub fn paint_wrapped(
        &self,
        origin: Point<Pixels>,
        _visible_bounds: Bounds<Pixels>,
        line_height: Pixels,
        boundaries: &[ShapedBoundary],
        cx: &mut WindowContext,
    ) {
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_offset = point(px(0.), padding_top + self.layout.ascent);

        let mut boundaries = boundaries.into_iter().peekable();
        let mut color_runs = self.style_runs.iter();
        let mut style_run_end = 0;
        let mut color = black();
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
                        color = style_run.color;
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
                        color = black();
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

                let _glyph_bounds = Bounds {
                    origin: glyph_origin,
                    size: cx
                        .text_system()
                        .bounding_box(run.font_id, self.layout.font_size),
                };
                // todo!()
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
    }
}

impl Run {
    pub fn glyphs(&self) -> &[Glyph] {
        &self.glyphs
    }
}
