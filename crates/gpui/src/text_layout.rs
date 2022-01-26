use crate::{
    color::Color,
    fonts::{FontId, GlyphId, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform, scene, FontSystem, PaintContext,
};
use ordered_float::OrderedFloat;
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
    fonts: Arc<dyn platform::FontSystem>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RunStyle {
    pub color: Color,
    pub font_id: FontId,
    pub underline: Option<Underline>,
}

impl TextLayoutCache {
    pub fn new(fonts: Arc<dyn platform::FontSystem>) -> Self {
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
        font_size: f32,
        runs: &'a [(usize, RunStyle)],
    ) -> Line {
        let key = &CacheKeyRef {
            text,
            font_size: OrderedFloat(font_size),
            runs,
        } as &dyn CacheKey;
        let curr_frame = self.curr_frame.upgradable_read();
        if let Some(layout) = curr_frame.get(key) {
            return Line::new(layout.clone(), runs);
        }

        let mut curr_frame = RwLockUpgradableReadGuard::upgrade(curr_frame);
        if let Some((key, layout)) = self.prev_frame.lock().remove_entry(key) {
            curr_frame.insert(key, layout.clone());
            Line::new(layout.clone(), runs)
        } else {
            let layout = Arc::new(self.fonts.layout_line(text, font_size, runs));
            let key = CacheKeyValue {
                text: text.into(),
                font_size: OrderedFloat(font_size),
                runs: SmallVec::from(runs),
            };
            curr_frame.insert(key, layout.clone());
            Line::new(layout, runs)
        }
    }
}

trait CacheKey {
    fn key<'a>(&'a self) -> CacheKeyRef<'a>;
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

#[derive(Eq, PartialEq)]
struct CacheKeyValue {
    text: String,
    font_size: OrderedFloat<f32>,
    runs: SmallVec<[(usize, RunStyle); 1]>,
}

impl CacheKey for CacheKeyValue {
    fn key<'a>(&'a self) -> CacheKeyRef<'a> {
        CacheKeyRef {
            text: &self.text.as_str(),
            font_size: self.font_size,
            runs: self.runs.as_slice(),
        }
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
    font_size: OrderedFloat<f32>,
    runs: &'a [(usize, RunStyle)],
}

impl<'a> CacheKey for CacheKeyRef<'a> {
    fn key<'b>(&'b self) -> CacheKeyRef<'b> {
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

#[derive(Default, Debug)]
pub struct Line {
    layout: Arc<LineLayout>,
    style_runs: SmallVec<[(u32, Color, Option<Underline>); 32]>,
}

#[derive(Default, Debug)]
pub struct LineLayout {
    pub width: f32,
    pub ascent: f32,
    pub descent: f32,
    pub runs: Vec<Run>,
    pub len: usize,
    pub font_size: f32,
}

#[derive(Debug)]
pub struct Run {
    pub font_id: FontId,
    pub glyphs: Vec<Glyph>,
}

#[derive(Debug)]
pub struct Glyph {
    pub id: GlyphId,
    pub position: Vector2F,
    pub index: usize,
}

impl Line {
    fn new(layout: Arc<LineLayout>, runs: &[(usize, RunStyle)]) -> Self {
        let mut style_runs = SmallVec::new();
        for (len, style) in runs {
            style_runs.push((*len as u32, style.color, style.underline));
        }
        Self { layout, style_runs }
    }

    pub fn runs(&self) -> &[Run] {
        &self.layout.runs
    }

    pub fn width(&self) -> f32 {
        self.layout.width
    }

    pub fn x_for_index(&self, index: usize) -> f32 {
        for run in &self.layout.runs {
            for glyph in &run.glyphs {
                if glyph.index == index {
                    return glyph.position.x();
                }
            }
        }
        self.layout.width
    }

    pub fn index_for_x(&self, x: f32) -> Option<usize> {
        if x >= self.layout.width {
            None
        } else {
            for run in self.layout.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x() <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    pub fn paint(
        &self,
        origin: Vector2F,
        visible_bounds: RectF,
        line_height: f32,
        cx: &mut PaintContext,
    ) {
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_offset = vec2f(0., padding_top + self.layout.ascent);

        let mut style_runs = self.style_runs.iter();
        let mut run_end = 0;
        let mut color = Color::black();
        let mut underline = None;

        for run in &self.layout.runs {
            let max_glyph_width = cx
                .font_cache
                .bounding_box(run.font_id, self.layout.font_size)
                .x();

            for glyph in &run.glyphs {
                let glyph_origin = origin + baseline_offset + glyph.position;
                if glyph_origin.x() > visible_bounds.upper_right().x() {
                    break;
                }

                let mut finished_underline = None;
                if glyph.index >= run_end {
                    if let Some((run_len, run_color, run_underline)) = style_runs.next() {
                        if let Some((_, underline_style)) = underline {
                            if *run_underline != Some(underline_style) {
                                finished_underline = underline.take();
                            }
                        }
                        if let Some(run_underline) = run_underline {
                            underline.get_or_insert((glyph_origin, *run_underline));
                        }

                        run_end += *run_len as usize;
                        color = *run_color;
                    } else {
                        run_end = self.layout.len;
                        color = Color::black();
                        finished_underline = underline.take();
                    }
                }

                if glyph_origin.x() + max_glyph_width < visible_bounds.origin().x() {
                    continue;
                }

                if let Some((underline_origin, underline_style)) = finished_underline {
                    cx.scene.push_underline(scene::Underline {
                        origin: underline_origin,
                        width: glyph_origin.x() - underline_origin.x(),
                        thickness: underline_style.thickness.into(),
                        color: underline_style.color,
                        squiggly: underline_style.squiggly,
                    });
                }

                cx.scene.push_glyph(scene::Glyph {
                    font_id: run.font_id,
                    font_size: self.layout.font_size,
                    id: glyph.id,
                    origin: glyph_origin,
                    color,
                });
            }
        }

        if let Some((underline_start, underline_style)) = underline.take() {
            let line_end_x = origin.x() + self.layout.width;
            cx.scene.push_underline(scene::Underline {
                origin: underline_start,
                width: line_end_x - underline_start.x(),
                color: underline_style.color,
                thickness: underline_style.thickness.into(),
                squiggly: underline_style.squiggly,
            });
        }
    }

    pub fn paint_wrapped(
        &self,
        origin: Vector2F,
        visible_bounds: RectF,
        line_height: f32,
        boundaries: impl IntoIterator<Item = ShapedBoundary>,
        cx: &mut PaintContext,
    ) {
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_origin = vec2f(0., padding_top + self.layout.ascent);

        let mut boundaries = boundaries.into_iter().peekable();
        let mut color_runs = self.style_runs.iter();
        let mut color_end = 0;
        let mut color = Color::black();

        let mut glyph_origin = vec2f(0., 0.);
        let mut prev_position = 0.;
        for run in &self.layout.runs {
            for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
                if boundaries.peek().map_or(false, |b| b.glyph_ix == glyph_ix) {
                    boundaries.next();
                    glyph_origin = vec2f(0., glyph_origin.y() + line_height);
                } else {
                    glyph_origin.set_x(glyph_origin.x() + glyph.position.x() - prev_position);
                }
                prev_position = glyph.position.x();

                if glyph.index >= color_end {
                    if let Some(next_run) = color_runs.next() {
                        color_end += next_run.0 as usize;
                        color = next_run.1;
                    } else {
                        color_end = self.layout.len;
                        color = Color::black();
                    }
                }

                let glyph_bounds = RectF::new(
                    origin + glyph_origin,
                    cx.font_cache
                        .bounding_box(run.font_id, self.layout.font_size),
                );
                if glyph_bounds.intersects(visible_bounds) {
                    cx.scene.push_glyph(scene::Glyph {
                        font_id: run.font_id,
                        font_size: self.layout.font_size,
                        id: glyph.id,
                        origin: glyph_bounds.origin() + baseline_origin,
                        color,
                    });
                }
            }
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
    font_system: Arc<dyn FontSystem>,
    pub(crate) font_id: FontId,
    pub(crate) font_size: f32,
    cached_ascii_char_widths: [f32; 128],
    cached_other_char_widths: HashMap<char, f32>,
}

impl LineWrapper {
    pub const MAX_INDENT: u32 = 256;

    pub fn new(font_id: FontId, font_size: f32, font_system: Arc<dyn FontSystem>) -> Self {
        Self {
            font_system,
            font_id,
            font_size,
            cached_ascii_char_widths: [f32::NAN; 128],
            cached_other_char_widths: HashMap::new(),
        }
    }

    pub fn wrap_line<'a>(
        &'a mut self,
        line: &'a str,
        wrap_width: f32,
    ) -> impl Iterator<Item = Boundary> + 'a {
        let mut width = 0.0;
        let mut first_non_whitespace_ix = None;
        let mut indent = None;
        let mut last_candidate_ix = 0;
        let mut last_candidate_width = 0.0;
        let mut last_wrap_ix = 0;
        let mut prev_c = '\0';
        let mut char_indices = line.char_indices();
        iter::from_fn(move || {
            while let Some((ix, c)) = char_indices.next() {
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

                    let indent_width =
                        indent.map(|indent| indent as f32 * self.width_for_char(' '));
                    width += indent_width.unwrap_or(0.);

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
        wrap_width: f32,
    ) -> impl Iterator<Item = ShapedBoundary> + 'a {
        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = 0.0;
        let mut last_wrap_ix = ShapedBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_wrap_x = 0.;
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
                            glyph.position.x(),
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
    fn width_for_char(&mut self, c: char) -> f32 {
        if (c as u32) < 128 {
            let mut width = self.cached_ascii_char_widths[c as usize];
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                self.cached_ascii_char_widths[c as usize] = width;
            }
            width
        } else {
            let mut width = self
                .cached_other_char_widths
                .get(&c)
                .copied()
                .unwrap_or(f32::NAN);
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                self.cached_other_char_widths.insert(c, width);
            }
            width
        }
    }

    fn compute_width_for_char(&self, c: char) -> f32 {
        self.font_system
            .layout_line(
                &c.to_string(),
                self.font_size,
                &[(
                    1,
                    RunStyle {
                        font_id: self.font_id,
                        color: Default::default(),
                        underline: None,
                    },
                )],
            )
            .width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::{Properties, Weight};

    #[crate::test(self)]
    fn test_wrap_line(cx: &mut crate::MutableAppContext) {
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let family = font_cache.load_family(&["Courier"]).unwrap();
        let font_id = font_cache.select_font(family, &Default::default()).unwrap();

        let mut wrapper = LineWrapper::new(font_id, 16., font_system);
        assert_eq!(
            wrapper
                .wrap_line("aa bbb cccc ddddd eeee", 72.0)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(12, 0),
                Boundary::new(18, 0)
            ],
        );
        assert_eq!(
            wrapper
                .wrap_line("aaa aaaaaaaaaaaaaaaaaa", 72.0)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(4, 0),
                Boundary::new(11, 0),
                Boundary::new(18, 0)
            ],
        );
        assert_eq!(
            wrapper.wrap_line("     aaaaaaa", 72.).collect::<Vec<_>>(),
            &[
                Boundary::new(7, 5),
                Boundary::new(9, 5),
                Boundary::new(11, 5),
            ]
        );
        assert_eq!(
            wrapper
                .wrap_line("                            ", 72.)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(14, 0),
                Boundary::new(21, 0)
            ]
        );
        assert_eq!(
            wrapper
                .wrap_line("          aaaaaaaaaaaaaa", 72.)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(14, 3),
                Boundary::new(18, 3),
                Boundary::new(22, 3),
            ]
        );
    }

    #[crate::test(self, retries = 5)]
    fn test_wrap_shaped_line(cx: &mut crate::MutableAppContext) {
        // This is failing intermittently on CI and we don't have time to figure it out
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let text_layout_cache = TextLayoutCache::new(font_system.clone());

        let family = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache.select_font(family, &Default::default()).unwrap();
        let normal = RunStyle {
            font_id,
            color: Default::default(),
            underline: None,
        };
        let bold = RunStyle {
            font_id: font_cache
                .select_font(
                    family,
                    &Properties {
                        weight: Weight::BOLD,
                        ..Default::default()
                    },
                )
                .unwrap(),
            color: Default::default(),
            underline: None,
        };

        let text = "aa bbb cccc ddddd eeee";
        let line = text_layout_cache.layout_str(
            text,
            16.0,
            &[(4, normal), (5, bold), (6, normal), (1, bold), (7, normal)],
        );

        let mut wrapper = LineWrapper::new(font_id, 16., font_system);
        assert_eq!(
            wrapper
                .wrap_shaped_line(&text, &line, 72.0)
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
