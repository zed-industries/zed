use crate::{
    color::ColorU,
    fonts::{FontId, GlyphId},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform, scene, PaintContext,
};
use ordered_float::OrderedFloat;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub struct TextLayoutCache {
    prev_frame: Mutex<HashMap<CacheKeyValue, Arc<LineLayout>>>,
    curr_frame: RwLock<HashMap<CacheKeyValue, Arc<LineLayout>>>,
    fonts: Arc<dyn platform::FontSystem>,
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
        runs: &'a [(usize, FontId, ColorU)],
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
            let layout = Arc::new(self.fonts.layout_str(text, font_size, runs));
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
    runs: SmallVec<[(usize, FontId, ColorU); 1]>,
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: OrderedFloat<f32>,
    runs: &'a [(usize, FontId, ColorU)],
}

impl<'a> CacheKey for CacheKeyRef<'a> {
    fn key<'b>(&'b self) -> CacheKeyRef<'b> {
        *self
    }
}

#[derive(Default, Debug)]
pub struct Line {
    layout: Arc<LineLayout>,
    color_runs: SmallVec<[(u32, ColorU); 32]>,
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
    fn new(layout: Arc<LineLayout>, runs: &[(usize, FontId, ColorU)]) -> Self {
        let mut color_runs = SmallVec::new();
        for (len, _, color) in runs {
            color_runs.push((*len as u32, *color));
        }
        Self { layout, color_runs }
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

    pub fn paint(&self, origin: Vector2F, bounds: RectF, ctx: &mut PaintContext) {
        let padding_top = (bounds.height() - self.layout.ascent - self.layout.descent) / 2.;
        let baseline_origin = vec2f(0., padding_top + self.layout.ascent);

        let mut color_runs = self.color_runs.iter();
        let mut color_end = 0;
        let mut color = ColorU::black();

        for run in &self.layout.runs {
            let max_glyph_width = ctx
                .font_cache
                .bounding_box(run.font_id, self.layout.font_size)
                .x();

            for glyph in &run.glyphs {
                let glyph_origin = baseline_origin + glyph.position;

                if glyph_origin.x() + max_glyph_width < bounds.origin().x() {
                    continue;
                }
                if glyph_origin.x() > bounds.upper_right().x() {
                    break;
                }

                if glyph.index >= color_end {
                    if let Some(next_run) = color_runs.next() {
                        color_end += next_run.0 as usize;
                        color = next_run.1;
                    } else {
                        color_end = self.layout.len;
                        color = ColorU::black();
                    }
                }

                ctx.scene.push_glyph(scene::Glyph {
                    font_id: run.font_id,
                    font_size: self.layout.font_size,
                    id: glyph.id,
                    origin: origin + glyph_origin,
                    color,
                });
            }
        }
    }
}
