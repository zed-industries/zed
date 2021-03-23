use crate::{
    color::ColorU,
    fonts::{FontCache, FontId, GlyphId},
    geometry::rect::RectF,
    scene, PaintContext,
};
use core_foundation::{
    attributed_string::CFMutableAttributedString,
    base::{CFRange, TCFType},
    string::CFString,
};
use core_text::{font::CTFont, line::CTLine, string_attributes::kCTFontAttributeName};
use ordered_float::OrderedFloat;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    char,
    collections::HashMap,
    convert::TryFrom,
    hash::{Hash, Hasher},
    ops::Range,
    sync::Arc,
};

pub struct TextLayoutCache {
    prev_frame: Mutex<HashMap<CacheKeyValue, Arc<Line>>>,
    curr_frame: RwLock<HashMap<CacheKeyValue, Arc<Line>>>,
}

impl TextLayoutCache {
    pub fn new() -> Self {
        Self {
            prev_frame: Mutex::new(HashMap::new()),
            curr_frame: RwLock::new(HashMap::new()),
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
        font_cache: &'a FontCache,
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
            let line = Arc::new(layout_str(text, font_size, runs, font_cache));
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

#[derive(Default)]
pub struct Line {
    pub width: f32,
    pub runs: Vec<Run>,
    pub len: usize,
    font_size: f32,
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
            let ascent = ctx.font_cache.scale_metric(
                ctx.font_cache.metric(run.font_id, |m| m.ascent),
                run.font_id,
                self.font_size,
            );
            let descent = ctx.font_cache.scale_metric(
                ctx.font_cache.metric(run.font_id, |m| m.descent),
                run.font_id,
                self.font_size,
            );

            let max_glyph_width = bounding_box.x();
            let font = ctx.font_cache.font(run.font_id);
            let font_name = ctx.font_cache.font_name(run.font_id);
            let is_emoji = ctx.font_cache.is_emoji(run.font_id);
            for glyph in &run.glyphs {
                let glyph_origin = bounds.origin() + glyph.position;
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
                    origin: glyph_origin,
                    color,
                });
            }
        }
    }
}

pub fn layout_str(
    text: &str,
    font_size: f32,
    runs: &[(Range<usize>, FontId)],
    font_cache: &FontCache,
) -> Line {
    let mut string = CFMutableAttributedString::new();
    string.replace_str(&CFString::new(text), CFRange::init(0, 0));

    let mut utf16_lens = text.chars().map(|c| c.len_utf16());
    let mut prev_char_ix = 0;
    let mut prev_utf16_ix = 0;

    for (range, font_id) in runs {
        let utf16_start = prev_utf16_ix
            + utf16_lens
                .by_ref()
                .take(range.start - prev_char_ix)
                .sum::<usize>();
        let utf16_end = utf16_start
            + utf16_lens
                .by_ref()
                .take(range.end - range.start)
                .sum::<usize>();
        prev_char_ix = range.end;
        prev_utf16_ix = utf16_end;

        let cf_range = CFRange::init(utf16_start as isize, (utf16_end - utf16_start) as isize);
        let native_font = font_cache.native_font(*font_id, font_size);
        unsafe {
            string.set_attribute(cf_range, kCTFontAttributeName, &native_font);
        }
    }

    let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());

    let width = line.get_typographic_bounds().width as f32;

    let mut utf16_chars = text.encode_utf16();
    let mut char_ix = 0;
    let mut prev_utf16_ix = 0;

    let mut runs = Vec::new();
    for run in line.glyph_runs().into_iter() {
        let font_id = font_cache.font_id_for_native_font(unsafe {
            run.attributes()
                .unwrap()
                .get(kCTFontAttributeName)
                .downcast::<CTFont>()
                .unwrap()
        });

        let mut glyphs = Vec::new();
        for ((glyph_id, position), utf16_ix) in run
            .glyphs()
            .iter()
            .zip(run.positions().iter())
            .zip(run.string_indices().iter())
        {
            let utf16_ix = usize::try_from(*utf16_ix).unwrap();
            char_ix +=
                char::decode_utf16(utf16_chars.by_ref().take(utf16_ix - prev_utf16_ix)).count();
            prev_utf16_ix = utf16_ix;

            glyphs.push(Glyph {
                id: *glyph_id as GlyphId,
                position: vec2f(position.x as f32, position.y as f32),
                index: char_ix,
            });
        }

        runs.push(Run { font_id, glyphs })
    }

    Line {
        width,
        runs,
        font_size,
        len: char_ix + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use font_kit::properties::{
        Properties as FontProperties, Style as FontStyle, Weight as FontWeight,
    };

    #[test]
    fn test_layout_str() -> Result<()> {
        let mut font_cache = FontCache::new();
        let menlo = font_cache.load_family(&["Menlo"])?;
        let menlo_regular = font_cache.select_font(menlo, &FontProperties::new())?;
        let menlo_italic =
            font_cache.select_font(menlo, &FontProperties::new().style(FontStyle::Italic))?;
        let menlo_bold =
            font_cache.select_font(menlo, &FontProperties::new().weight(FontWeight::BOLD))?;

        let line = layout_str(
            "hello world 😃",
            16.0,
            &[
                (0..2, menlo_bold),
                (2..6, menlo_italic),
                (6..13, menlo_regular),
            ],
            &mut font_cache,
        );

        assert!(font_cache.is_emoji(line.runs.last().unwrap().font_id));

        Ok(())
    }

    #[test]
    fn test_char_indices() -> Result<()> {
        let mut font_cache = FontCache::new();
        let zapfino = font_cache.load_family(&["Zapfino"])?;
        let zapfino_regular = font_cache.select_font(zapfino, &FontProperties::new())?;
        let menlo = font_cache.load_family(&["Menlo"])?;
        let menlo_regular = font_cache.select_font(menlo, &FontProperties::new())?;

        let text = "This is, m𐍈re 𐍈r less, Zapfino!𐍈";
        let line = layout_str(
            text,
            16.0,
            &[
                (0..9, zapfino_regular),
                (11..22, menlo_regular),
                (22..text.encode_utf16().count(), zapfino_regular),
            ],
            &mut font_cache,
        );
        assert_eq!(
            line.runs
                .iter()
                .flat_map(|r| r.glyphs.iter())
                .map(|g| g.index)
                .collect::<Vec<_>>(),
            vec![
                0, 2, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                31, 32
            ]
        );
        Ok(())
    }
}
