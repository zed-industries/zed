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
    ops::Range,
    sync::Arc,
};

pub struct TextLayoutCache {
    prev_frame: Mutex<HashMap<CacheKeyValue, Arc<Line>>>,
    curr_frame: RwLock<HashMap<CacheKeyValue, Arc<Line>>>,
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
        runs: &'a [(Range<usize>, FontId)],
    ) -> Arc<Line> {
        let key = &CacheKeyRef {
            text,
            font_size: OrderedFloat(font_size),
            runs,
        } as &dyn CacheKey;
        let curr_frame = self.curr_frame.upgradable_read();
        if let Some(line) = curr_frame.get(key) {
            return line.clone();
        }

        let mut curr_frame = RwLockUpgradableReadGuard::upgrade(curr_frame);
        if let Some((key, line)) = self.prev_frame.lock().remove_entry(key) {
            curr_frame.insert(key, line.clone());
            line.clone()
        } else {
            let line = Arc::new(self.fonts.layout_str(text, font_size, runs));
            let key = CacheKeyValue {
                text: text.into(),
                font_size: OrderedFloat(font_size),
                runs: SmallVec::from(runs),
            };
            curr_frame.insert(key, line.clone());
            line
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
    runs: SmallVec<[(Range<usize>, FontId); 1]>,
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
    runs: &'a [(Range<usize>, FontId)],
}

impl<'a> CacheKey for CacheKeyRef<'a> {
    fn key<'b>(&'b self) -> CacheKeyRef<'b> {
        *self
    }
}

#[derive(Default, Debug)]
pub struct Line {
    pub width: f32,
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
    pub fn x_for_index(&self, index: usize) -> f32 {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index == index {
                    return glyph.position.x();
                }
            }
        }
        self.width
    }

    pub fn index_for_x(&self, x: f32) -> Option<usize> {
        if x >= self.width {
            None
        } else {
            for run in self.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x() <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    pub fn paint(&self, bounds: RectF, colors: &[(Range<usize>, ColorU)], ctx: &mut PaintContext) {
        let mut colors = colors.iter().peekable();
        let mut color = ColorU::black();

        for run in &self.runs {
            let bounding_box = ctx.font_cache.bounding_box(run.font_id, self.font_size);
            let descent = ctx.font_cache.descent(run.font_id, self.font_size);
            let max_glyph_width = bounding_box.x();
            for glyph in &run.glyphs {
                let glyph_origin = bounds.origin() + glyph.position - vec2f(0.0, descent);
                if glyph_origin.x() + max_glyph_width < bounds.origin().x() {
                    continue;
                }
                if glyph_origin.x() > bounds.upper_right().x() {
                    break;
                }

                while let Some((range, next_color)) = colors.peek() {
                    if glyph.index >= range.end {
                        colors.next();
                    } else {
                        color = *next_color;
                        break;
                    }
                }

                ctx.scene.push_glyph(scene::Glyph {
                    font_id: run.font_id,
                    font_size: self.font_size,
                    id: glyph.id,
                    origin: glyph_origin + vec2f(0., bounding_box.y() / 2.),
                    color,
                });
            }
        }
    }
}
