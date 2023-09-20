use crate::{black, px};

use super::{
    point, Bounds, FontId, Glyph, Hsla, Pixels, PlatformTextSystem, Point, UnderlineStyle,
    WindowContext,
};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    iter,
    sync::Arc,
};

pub struct TextLayoutCache {
    prev_frame: Mutex<HashMap<CacheKeyValue, Arc<LineLayout>>>,
    curr_frame: RwLock<HashMap<CacheKeyValue, Arc<LineLayout>>>,
    fonts: Arc<dyn PlatformTextSystem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunStyle {
    pub color: Hsla,
    pub font_id: FontId,
    pub underline: Option<UnderlineStyle>,
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
                    len_a == len_b && style_a.font_id == style_b.font_id
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
            style_id.font_id.hash(state);
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

#[derive(Default, Debug)]
pub struct LineLayout {
    pub font_size: Pixels,
    pub width: Pixels,
    pub ascent: Pixels,
    pub descent: Pixels,
    pub runs: Vec<Run>,
    pub len: usize,
}

#[derive(Debug)]
pub struct Run {
    pub font_id: FontId,
    pub glyphs: Vec<Glyph>,
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
                .font_cache()
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
                        .font_cache()
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Boundary {
    pub ix: usize,
    pub next_indent: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapedBoundary {
    pub run_ix: usize,
    pub glyph_ix: usize,
}

impl Boundary {
    fn new(ix: usize, next_indent: u32) -> Self {
        Self { ix, next_indent }
    }
}

pub struct LineWrapper {
    font_system: Arc<dyn PlatformTextSystem>,
    pub(crate) font_id: FontId,
    pub(crate) font_size: Pixels,
    cached_ascii_char_widths: [Option<Pixels>; 128],
    cached_other_char_widths: HashMap<char, Pixels>,
}

impl LineWrapper {
    pub const MAX_INDENT: u32 = 256;

    pub fn new(
        font_id: FontId,
        font_size: Pixels,
        font_system: Arc<dyn PlatformTextSystem>,
    ) -> Self {
        Self {
            font_system,
            font_id,
            font_size,
            cached_ascii_char_widths: [None; 128],
            cached_other_char_widths: HashMap::new(),
        }
    }

    pub fn wrap_line<'a>(
        &'a mut self,
        line: &'a str,
        wrap_width: Pixels,
    ) -> impl Iterator<Item = Boundary> + 'a {
        let mut width = px(0.);
        let mut first_non_whitespace_ix = None;
        let mut indent = None;
        let mut last_candidate_ix = 0;
        let mut last_candidate_width = px(0.);
        let mut last_wrap_ix = 0;
        let mut prev_c = '\0';
        let mut char_indices = line.char_indices();
        iter::from_fn(move || {
            for (ix, c) in char_indices.by_ref() {
                if c == '\n' {
                    continue;
                }

                if self.is_boundary(prev_c, c) && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = ix;
                    last_candidate_width = width;
                }

                if c != ' ' && first_non_whitespace_ix.is_none() {
                    first_non_whitespace_ix = Some(ix);
                }

                let char_width = self.width_for_char(c);
                width += char_width;
                if width > wrap_width && ix > last_wrap_ix {
                    if let (None, Some(first_non_whitespace_ix)) = (indent, first_non_whitespace_ix)
                    {
                        indent = Some(
                            Self::MAX_INDENT.min((first_non_whitespace_ix - last_wrap_ix) as u32),
                        );
                    }

                    if last_candidate_ix > 0 {
                        last_wrap_ix = last_candidate_ix;
                        width -= last_candidate_width;
                        last_candidate_ix = 0;
                    } else {
                        last_wrap_ix = ix;
                        width = char_width;
                    }

                    if let Some(indent) = indent {
                        width += self.width_for_char(' ') * indent as f32;
                    }

                    return Some(Boundary::new(last_wrap_ix, indent.unwrap_or(0)));
                }
                prev_c = c;
            }

            None
        })
    }

    pub fn wrap_shaped_line<'a>(
        &'a mut self,
        str: &'a str,
        line: &'a Line,
        wrap_width: Pixels,
    ) -> impl Iterator<Item = ShapedBoundary> + 'a {
        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = px(0.);
        let mut last_wrap_ix = ShapedBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_wrap_x = px(0.);
        let mut prev_c = '\0';
        let mut glyphs = line
            .runs()
            .iter()
            .enumerate()
            .flat_map(move |(run_ix, run)| {
                run.glyphs()
                    .iter()
                    .enumerate()
                    .map(move |(glyph_ix, glyph)| {
                        let character = str[glyph.index..].chars().next().unwrap();
                        (
                            ShapedBoundary { run_ix, glyph_ix },
                            character,
                            glyph.position.x,
                        )
                    })
            })
            .peekable();

        iter::from_fn(move || {
            while let Some((ix, c, x)) = glyphs.next() {
                if c == '\n' {
                    continue;
                }

                if self.is_boundary(prev_c, c) && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = Some(ix);
                    last_candidate_x = x;
                }

                if c != ' ' && first_non_whitespace_ix.is_none() {
                    first_non_whitespace_ix = Some(ix);
                }

                let next_x = glyphs.peek().map_or(line.width(), |(_, _, x)| *x);
                let width = next_x - last_wrap_x;
                if width > wrap_width && ix > last_wrap_ix {
                    if let Some(last_candidate_ix) = last_candidate_ix.take() {
                        last_wrap_ix = last_candidate_ix;
                        last_wrap_x = last_candidate_x;
                    } else {
                        last_wrap_ix = ix;
                        last_wrap_x = x;
                    }

                    return Some(last_wrap_ix);
                }
                prev_c = c;
            }

            None
        })
    }

    fn is_boundary(&self, prev: char, next: char) -> bool {
        (prev == ' ') && (next != ' ')
    }

    #[inline(always)]
    fn width_for_char(&mut self, c: char) -> Pixels {
        if (c as u32) < 128 {
            if let Some(cached_width) = self.cached_ascii_char_widths[c as usize] {
                cached_width
            } else {
                let width = self.compute_width_for_char(c);
                self.cached_ascii_char_widths[c as usize] = Some(width);
                width
            }
        } else {
            if let Some(cached_width) = self.cached_other_char_widths.get(&c) {
                *cached_width
            } else {
                let width = self.compute_width_for_char(c);
                self.cached_other_char_widths.insert(c, width);
                width
            }
        }
    }

    fn compute_width_for_char(&self, c: char) -> Pixels {
        self.font_system
            .layout_line(
                &c.to_string(),
                self.font_size,
                &[(
                    1,
                    RunStyle {
                        font_id: self.font_id,
                        color: Default::default(),
                        underline: Default::default(),
                    },
                )],
            )
            .width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppContext, FontWeight};

    #[test]
    fn test_wrap_line() {
        let cx = AppContext::test();

        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().font_system();
        let family = font_cache
            .load_family(&["Courier"], &Default::default())
            .unwrap();
        let font_id = font_cache
            .select_font(family, Default::default(), Default::default())
            .unwrap();

        let mut wrapper = LineWrapper::new(font_id, px(16.), font_system);
        assert_eq!(
            wrapper
                .wrap_line("aa bbb cccc ddddd eeee", px(72.))
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(12, 0),
                Boundary::new(18, 0)
            ],
        );
        assert_eq!(
            wrapper
                .wrap_line("aaa aaaaaaaaaaaaaaaaaa", px(72.0))
                .collect::<Vec<_>>(),
            &[
                Boundary::new(4, 0),
                Boundary::new(11, 0),
                Boundary::new(18, 0)
            ],
        );
        assert_eq!(
            wrapper
                .wrap_line("     aaaaaaa", px(72.))
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 5),
                Boundary::new(9, 5),
                Boundary::new(11, 5),
            ]
        );
        assert_eq!(
            wrapper
                .wrap_line("                            ", px(72.))
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(14, 0),
                Boundary::new(21, 0)
            ]
        );
        assert_eq!(
            wrapper
                .wrap_line("          aaaaaaaaaaaaaa", px(72.))
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(14, 3),
                Boundary::new(18, 3),
                Boundary::new(22, 3),
            ]
        );
    }

    // todo! repeat this test
    #[test]
    fn test_wrap_shaped_line() {
        let cx = AppContext::test();
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().font_system();
        let text_layout_cache = TextLayoutCache::new(font_system.clone());

        let family = font_cache
            .load_family(&["Helvetica"], &Default::default())
            .unwrap();
        let font_id = font_cache
            .select_font(family, Default::default(), Default::default())
            .unwrap();
        let normal = RunStyle {
            font_id,
            color: Default::default(),
            underline: Default::default(),
        };
        let bold = RunStyle {
            font_id: font_cache
                .select_font(family, FontWeight::BOLD, Default::default())
                .unwrap(),
            color: Default::default(),
            underline: Default::default(),
        };

        let text = "aa bbb cccc ddddd eeee";
        let line = text_layout_cache.layout_str(
            text,
            px(16.),
            &[
                (4, normal.clone()),
                (5, bold.clone()),
                (6, normal.clone()),
                (1, bold),
                (7, normal),
            ],
        );

        let mut wrapper = LineWrapper::new(font_id, px(16.), font_system);
        assert_eq!(
            wrapper
                .wrap_shaped_line(text, &line, px(72.))
                .collect::<Vec<_>>(),
            &[
                ShapedBoundary {
                    run_ix: 1,
                    glyph_ix: 3
                },
                ShapedBoundary {
                    run_ix: 2,
                    glyph_ix: 3
                },
                ShapedBoundary {
                    run_ix: 4,
                    glyph_ix: 2
                }
            ],
        );
    }
}
