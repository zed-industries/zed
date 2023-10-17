use crate::{px, FontId, GlyphId, Pixels, PlatformTextSystem, Point, SharedString};
use derive_more::{Deref, DerefMut};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Default, Debug)]
pub struct LineLayout {
    pub font_size: Pixels,
    pub width: Pixels,
    pub ascent: Pixels,
    pub descent: Pixels,
    pub runs: Vec<ShapedRun>,
}

#[derive(Debug)]
pub struct ShapedRun {
    pub font_id: FontId,
    pub glyphs: SmallVec<[ShapedGlyph; 8]>,
}

#[derive(Clone, Debug)]
pub struct ShapedGlyph {
    pub id: GlyphId,
    pub position: Point<Pixels>,
    pub index: usize,
    pub is_emoji: bool,
}

impl LineLayout {
    pub fn index_for_x(&self, x: Pixels) -> Option<usize> {
        if x >= self.width {
            None
        } else {
            for run in self.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    pub fn x_for_index(&self, index: usize) -> Pixels {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return glyph.position.x;
                }
            }
        }
        self.width
    }

    pub fn font_for_index(&self, index: usize) -> Option<FontId> {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return Some(run.font_id);
                }
            }
        }

        None
    }

    fn compute_wrap_boundaries(
        &self,
        text: &str,
        wrap_width: Pixels,
    ) -> SmallVec<[WrapBoundary; 1]> {
        let mut boundaries = SmallVec::new();

        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = px(0.);
        let mut last_boundary = WrapBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_boundary_x = px(0.);
        let mut prev_ch = '\0';
        let mut glyphs = self
            .runs
            .iter()
            .enumerate()
            .flat_map(move |(run_ix, run)| {
                run.glyphs.iter().enumerate().map(move |(glyph_ix, glyph)| {
                    let character = text[glyph.index..].chars().next().unwrap();
                    (
                        WrapBoundary { run_ix, glyph_ix },
                        character,
                        glyph.position.x,
                    )
                })
            })
            .peekable();

        while let Some((boundary, ch, x)) = glyphs.next() {
            if ch == '\n' {
                continue;
            }

            if prev_ch == ' ' && ch != ' ' && first_non_whitespace_ix.is_some() {
                last_candidate_ix = Some(boundary);
                last_candidate_x = x;
            }

            if ch != ' ' && first_non_whitespace_ix.is_none() {
                first_non_whitespace_ix = Some(boundary);
            }

            let next_x = glyphs.peek().map_or(self.width, |(_, _, x)| *x);
            let width = next_x - last_boundary_x;
            if width > wrap_width && boundary > last_boundary {
                if let Some(last_candidate_ix) = last_candidate_ix.take() {
                    last_boundary = last_candidate_ix;
                    last_boundary_x = last_candidate_x;
                } else {
                    last_boundary = boundary;
                    last_boundary_x = x;
                }

                boundaries.push(last_boundary);
            }
            prev_ch = ch;
        }

        boundaries
    }
}

#[derive(Deref, DerefMut, Default, Debug)]
pub struct WrappedLineLayout {
    #[deref]
    #[deref_mut]
    pub layout: LineLayout,
    pub text: SharedString,
    pub wrap_boundaries: SmallVec<[WrapBoundary; 1]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrapBoundary {
    pub run_ix: usize,
    pub glyph_ix: usize,
}

pub(crate) struct LineLayoutCache {
    prev_frame: Mutex<HashMap<CacheKey, Arc<WrappedLineLayout>>>,
    curr_frame: RwLock<HashMap<CacheKey, Arc<WrappedLineLayout>>>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
}

impl LineLayoutCache {
    pub fn new(platform_text_system: Arc<dyn PlatformTextSystem>) -> Self {
        Self {
            prev_frame: Mutex::new(HashMap::new()),
            curr_frame: RwLock::new(HashMap::new()),
            platform_text_system,
        }
    }

    pub fn end_frame(&self) {
        let mut prev_frame = self.prev_frame.lock();
        let mut curr_frame = self.curr_frame.write();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.clear();
    }

    pub fn layout_line(
        &self,
        text: &SharedString,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
    ) -> Arc<WrappedLineLayout> {
        let key = &CacheKeyRef {
            text,
            font_size,
            runs,
            wrap_width,
        } as &dyn AsCacheKeyRef;
        let curr_frame = self.curr_frame.upgradable_read();
        if let Some(layout) = curr_frame.get(key) {
            return layout.clone();
        }

        let mut curr_frame = RwLockUpgradableReadGuard::upgrade(curr_frame);
        if let Some((key, layout)) = self.prev_frame.lock().remove_entry(key) {
            curr_frame.insert(key, layout.clone());
            layout
        } else {
            let layout = self.platform_text_system.layout_line(text, font_size, runs);
            let wrap_boundaries = wrap_width
                .map(|wrap_width| layout.compute_wrap_boundaries(text.as_ref(), wrap_width))
                .unwrap_or_default();
            let wrapped_line = Arc::new(WrappedLineLayout {
                layout,
                text: text.clone(),
                wrap_boundaries,
            });

            let key = CacheKey {
                text: text.clone(),
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
            };
            curr_frame.insert(key, wrapped_line.clone());
            wrapped_line
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontRun {
    pub(crate) len: usize,
    pub(crate) font_id: FontId,
}

trait AsCacheKeyRef {
    fn as_cache_key_ref(&self) -> CacheKeyRef;
}

#[derive(Eq)]
struct CacheKey {
    text: SharedString,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
}

impl<'a> PartialEq for (dyn AsCacheKeyRef + 'a) {
    fn eq(&self, other: &dyn AsCacheKeyRef) -> bool {
        self.as_cache_key_ref() == other.as_cache_key_ref()
    }
}

impl<'a> Eq for (dyn AsCacheKeyRef + 'a) {}

impl<'a> Hash for (dyn AsCacheKeyRef + 'a) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state)
    }
}

impl AsCacheKeyRef for CacheKey {
    fn as_cache_key_ref(&self) -> CacheKeyRef {
        CacheKeyRef {
            text: &self.text,
            font_size: self.font_size,
            runs: self.runs.as_slice(),
            wrap_width: self.wrap_width,
        }
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_cache_key_ref().eq(&other.as_cache_key_ref())
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state);
    }
}

impl<'a> Borrow<dyn AsCacheKeyRef + 'a> for CacheKey {
    fn borrow(&self) -> &(dyn AsCacheKeyRef + 'a) {
        self as &dyn AsCacheKeyRef
    }
}

impl<'a> AsCacheKeyRef for CacheKeyRef<'a> {
    fn as_cache_key_ref(&self) -> CacheKeyRef {
        *self
    }
}
