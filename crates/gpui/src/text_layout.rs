use crate::{
    color::Color,
    fonts::{FontId, GlyphId, Underline},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform,
    platform::FontSystem,
    scene,
    window::WindowContext,
    SceneBuilder,
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
    pub underline: Underline,
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
            Line::new(layout, runs)
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
    font_size: OrderedFloat<f32>,
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
    font_size: OrderedFloat<f32>,
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

#[derive(Debug, Clone, Copy)]
struct StyleRun {
    len: u32,
    color: Color,
    underline: Underline,
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

#[derive(Clone, Debug)]
pub struct Glyph {
    pub id: GlyphId,
    pub position: Vector2F,
    pub index: usize,
    pub is_emoji: bool,
}

impl Line {
    fn new(layout: Arc<LineLayout>, runs: &[(usize, RunStyle)]) -> Self {
        let mut style_runs = SmallVec::new();
        for (len, style) in runs {
            style_runs.push(StyleRun {
                len: *len as u32,
                color: style.color,
                underline: style.underline,
            });
        }
        Self { layout, style_runs }
    }

    pub fn runs(&self) -> &[Run] {
        &self.layout.runs
    }

    pub fn width(&self) -> f32 {
        self.layout.width
    }

    pub fn font_size(&self) -> f32 {
        self.layout.font_size
    }

    pub fn x_for_index(&self, index: usize) -> f32 {
        for run in &self.layout.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return glyph.position.x();
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
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        line_height: f32,
        cx: &mut WindowContext,
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
                    if let Some(style_run) = style_runs.next() {
                        if let Some((_, underline_style)) = underline {
                            if style_run.underline != underline_style {
                                finished_underline = underline.take();
                            }
                        }
                        if style_run.underline.thickness.into_inner() > 0. {
                            underline.get_or_insert((
                                vec2f(
                                    glyph_origin.x(),
                                    origin.y() + baseline_offset.y() + 0.618 * self.layout.descent,
                                ),
                                Underline {
                                    color: Some(
                                        style_run.underline.color.unwrap_or(style_run.color),
                                    ),
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

                if glyph_origin.x() + max_glyph_width < visible_bounds.origin().x() {
                    continue;
                }

                if let Some((underline_origin, underline_style)) = finished_underline {
                    scene.push_underline(scene::Underline {
                        origin: underline_origin,
                        width: glyph_origin.x() - underline_origin.x(),
                        thickness: underline_style.thickness.into(),
                        color: underline_style.color.unwrap(),
                        squiggly: underline_style.squiggly,
                    });
                }

                if glyph.is_emoji {
                    scene.push_image_glyph(scene::ImageGlyph {
                        font_id: run.font_id,
                        font_size: self.layout.font_size,
                        id: glyph.id,
                        origin: glyph_origin,
                    });
                } else {
                    scene.push_glyph(scene::Glyph {
                        font_id: run.font_id,
                        font_size: self.layout.font_size,
                        id: glyph.id,
                        origin: glyph_origin,
                        color,
                    });
                }
            }
        }

        if let Some((underline_start, underline_style)) = underline.take() {
            let line_end_x = origin.x() + self.layout.width;
            scene.push_underline(scene::Underline {
                origin: underline_start,
                width: line_end_x - underline_start.x(),
                color: underline_style.color.unwrap(),
                thickness: underline_style.thickness.into(),
                squiggly: underline_style.squiggly,
            });
        }
    }

    pub fn paint_wrapped(
        &self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        line_height: f32,
        boundaries: &[ShapedBoundary],
        cx: &mut WindowContext,
    ) {
        let padding_top = (line_height - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_offset = vec2f(0., padding_top + self.layout.ascent);

        let mut boundaries = boundaries.into_iter().peekable();
        let mut color_runs = self.style_runs.iter();
        let mut style_run_end = 0;
        let mut color = Color::black();
        let mut underline: Option<(Vector2F, Underline)> = None;

        let mut glyph_origin = origin;
        let mut prev_position = 0.;
        for (run_ix, run) in self.layout.runs.iter().enumerate() {
            for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
                glyph_origin.set_x(glyph_origin.x() + glyph.position.x() - prev_position);

                if boundaries
                    .peek()
                    .map_or(false, |b| b.run_ix == run_ix && b.glyph_ix == glyph_ix)
                {
                    boundaries.next();
                    if let Some((underline_origin, underline_style)) = underline {
                        scene.push_underline(scene::Underline {
                            origin: underline_origin,
                            width: glyph_origin.x() - underline_origin.x(),
                            thickness: underline_style.thickness.into(),
                            color: underline_style.color.unwrap(),
                            squiggly: underline_style.squiggly,
                        });
                    }

                    glyph_origin = vec2f(origin.x(), glyph_origin.y() + line_height);
                }
                prev_position = glyph.position.x();

                let mut finished_underline = None;
                if glyph.index >= style_run_end {
                    if let Some(style_run) = color_runs.next() {
                        style_run_end += style_run.len as usize;
                        color = style_run.color;
                        if let Some((_, underline_style)) = underline {
                            if style_run.underline != underline_style {
                                finished_underline = underline.take();
                            }
                        }
                        if style_run.underline.thickness.into_inner() > 0. {
                            underline.get_or_insert((
                                glyph_origin
                                    + vec2f(0., baseline_offset.y() + 0.618 * self.layout.descent),
                                Underline {
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
                        color = Color::black();
                        finished_underline = underline.take();
                    }
                }

                if let Some((underline_origin, underline_style)) = finished_underline {
                    scene.push_underline(scene::Underline {
                        origin: underline_origin,
                        width: glyph_origin.x() - underline_origin.x(),
                        thickness: underline_style.thickness.into(),
                        color: underline_style.color.unwrap(),
                        squiggly: underline_style.squiggly,
                    });
                }

                let glyph_bounds = RectF::new(
                    glyph_origin,
                    cx.font_cache
                        .bounding_box(run.font_id, self.layout.font_size),
                );
                if glyph_bounds.intersects(visible_bounds) {
                    if glyph.is_emoji {
                        scene.push_image_glyph(scene::ImageGlyph {
                            font_id: run.font_id,
                            font_size: self.layout.font_size,
                            id: glyph.id,
                            origin: glyph_bounds.origin() + baseline_offset,
                        });
                    } else {
                        scene.push_glyph(scene::Glyph {
                            font_id: run.font_id,
                            font_size: self.layout.font_size,
                            id: glyph.id,
                            origin: glyph_bounds.origin() + baseline_offset,
                            color,
                        });
                    }
                }
            }
        }

        if let Some((underline_origin, underline_style)) = underline.take() {
            let line_end_x = glyph_origin.x() + self.layout.width - prev_position;
            scene.push_underline(scene::Underline {
                origin: underline_origin,
                width: line_end_x - underline_origin.x(),
                thickness: underline_style.thickness.into(),
                color: underline_style.color.unwrap(),
                squiggly: underline_style.squiggly,
            });
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
    pub fn new(ix: usize, next_indent: u32) -> Self {
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
                        underline: Default::default(),
                    },
                )],
            )
            .width
    }
}
