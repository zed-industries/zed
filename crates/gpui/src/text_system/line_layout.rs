use crate::{px, EntityId, FontId, GlyphId, Pixels, PlatformTextSystem, Point, Size};
use collections::{FxHashMap, FxHashSet};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    sync::Arc,
};

/// A laid out and styled line of text
#[derive(Default, Debug)]
pub struct LineLayout {
    /// The font size for this line
    pub font_size: Pixels,
    /// The width of the line
    pub width: Pixels,
    /// The ascent of the line
    pub ascent: Pixels,
    /// The descent of the line
    pub descent: Pixels,
    /// The shaped runs that make up this line
    pub runs: Vec<ShapedRun>,
    /// The length of the line in utf-8 bytes
    pub len: usize,
}

/// A run of text that has been shaped .
#[derive(Debug)]
pub struct ShapedRun {
    /// The font id for this run
    pub font_id: FontId,
    /// The glyphs that make up this run
    pub glyphs: SmallVec<[ShapedGlyph; 8]>,
}

/// A single glyph, ready to paint.
#[derive(Clone, Debug)]
pub struct ShapedGlyph {
    /// The ID for this glyph, as determined by the text system.
    pub id: GlyphId,

    /// The position of this glyph in its containing line.
    pub position: Point<Pixels>,

    /// The index of this glyph in the original text.
    pub index: usize,

    /// Whether this glyph is an emoji
    pub is_emoji: bool,
}

impl LineLayout {
    /// The index for the character at the given x coordinate
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

    /// closest_index_for_x returns the character boundary closest to the given x coordinate
    /// (e.g. to handle aligning up/down arrow keys)
    pub fn closest_index_for_x(&self, x: Pixels) -> usize {
        let mut prev_index = 0;
        let mut prev_x = px(0.);

        for run in self.runs.iter() {
            for glyph in run.glyphs.iter() {
                if glyph.position.x >= x {
                    if glyph.position.x - x < x - prev_x {
                        return glyph.index;
                    } else {
                        return prev_index;
                    }
                }
                prev_index = glyph.index;
                prev_x = glyph.position.x;
            }
        }

        self.len
    }

    /// The x position of the character at the given index
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

/// A line of text that has been wrapped to fit a given width
#[derive(Default, Debug)]
pub struct WrappedLineLayout {
    /// The line layout, pre-wrapping.
    pub unwrapped_layout: Arc<LineLayout>,

    /// The boundaries at which the line was wrapped
    pub wrap_boundaries: SmallVec<[WrapBoundary; 1]>,

    /// The width of the line, if it was wrapped
    pub wrap_width: Option<Pixels>,
}

/// A boundary at which a line was wrapped
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrapBoundary {
    /// The index in the run just before the line was wrapped
    pub run_ix: usize,
    /// The index of the glyph just before the line was wrapped
    pub glyph_ix: usize,
}

impl WrappedLineLayout {
    /// The length of the underlying text, in utf8 bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.unwrapped_layout.len
    }

    /// The width of this line, in pixels, whether or not it was wrapped.
    pub fn width(&self) -> Pixels {
        self.wrap_width
            .unwrap_or(Pixels::MAX)
            .min(self.unwrapped_layout.width)
    }

    /// The size of the whole wrapped text, for the given line_height.
    /// can span multiple lines if there are multiple wrap boundaries.
    pub fn size(&self, line_height: Pixels) -> Size<Pixels> {
        Size {
            width: self.width(),
            height: line_height * (self.wrap_boundaries.len() + 1),
        }
    }

    /// The ascent of a line in this layout
    pub fn ascent(&self) -> Pixels {
        self.unwrapped_layout.ascent
    }

    /// The descent of a line in this layout
    pub fn descent(&self) -> Pixels {
        self.unwrapped_layout.descent
    }

    /// The wrap boundaries in this layout
    pub fn wrap_boundaries(&self) -> &[WrapBoundary] {
        &self.wrap_boundaries
    }

    /// The font size of this layout
    pub fn font_size(&self) -> Pixels {
        self.unwrapped_layout.font_size
    }

    /// The runs in this layout, sans wrapping
    pub fn runs(&self) -> &[ShapedRun] {
        &self.unwrapped_layout.runs
    }

    /// The index corresponding to a given position in this layout for the given line height.
    pub fn index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Option<usize> {
        let wrapped_line_ix = (position.y / line_height) as usize;

        let wrapped_line_start_x = if wrapped_line_ix > 0 {
            let wrap_boundary_ix = wrapped_line_ix - 1;
            let wrap_boundary = self.wrap_boundaries[wrap_boundary_ix];
            let run = &self.unwrapped_layout.runs[wrap_boundary.run_ix];
            run.glyphs[wrap_boundary.glyph_ix].position.x
        } else {
            Pixels::ZERO
        };

        let wrapped_line_end_x = if wrapped_line_ix < self.wrap_boundaries.len() {
            let next_wrap_boundary_ix = wrapped_line_ix;
            let next_wrap_boundary = self.wrap_boundaries[next_wrap_boundary_ix];
            let run = &self.unwrapped_layout.runs[next_wrap_boundary.run_ix];
            run.glyphs[next_wrap_boundary.glyph_ix].position.x
        } else {
            self.unwrapped_layout.width
        };

        let mut position_in_unwrapped_line = position;
        position_in_unwrapped_line.x += wrapped_line_start_x;
        if position_in_unwrapped_line.x > wrapped_line_end_x {
            None
        } else {
            self.unwrapped_layout
                .index_for_x(position_in_unwrapped_line.x)
        }
    }
}

pub(crate) struct LineLayoutCache {
    view_stack: Mutex<Vec<EntityId>>,
    previous_frame: Mutex<FxHashMap<CacheKey, Arc<LineLayout>>>,
    current_frame: RwLock<FxHashMap<CacheKey, Arc<LineLayout>>>,
    previous_frame_wrapped: Mutex<FxHashMap<CacheKey, Arc<WrappedLineLayout>>>,
    current_frame_wrapped: RwLock<FxHashMap<CacheKey, Arc<WrappedLineLayout>>>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
}

impl LineLayoutCache {
    pub fn new(platform_text_system: Arc<dyn PlatformTextSystem>) -> Self {
        Self {
            view_stack: Mutex::default(),
            previous_frame: Mutex::default(),
            current_frame: RwLock::default(),
            previous_frame_wrapped: Mutex::default(),
            current_frame_wrapped: RwLock::default(),
            platform_text_system,
        }
    }

    pub fn finish_frame(&self, reused_views: &FxHashSet<EntityId>) {
        debug_assert_eq!(self.view_stack.lock().len(), 0);

        let mut prev_frame = self.previous_frame.lock();
        let mut curr_frame = self.current_frame.write();
        for (key, layout) in prev_frame.drain() {
            if key
                .parent_view_id
                .map_or(false, |view_id| reused_views.contains(&view_id))
            {
                curr_frame.insert(key, layout);
            }
        }
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);

        let mut prev_frame_wrapped = self.previous_frame_wrapped.lock();
        let mut curr_frame_wrapped = self.current_frame_wrapped.write();
        for (key, layout) in prev_frame_wrapped.drain() {
            if key
                .parent_view_id
                .map_or(false, |view_id| reused_views.contains(&view_id))
            {
                curr_frame_wrapped.insert(key, layout);
            }
        }
        std::mem::swap(&mut *prev_frame_wrapped, &mut *curr_frame_wrapped);
    }

    pub fn with_view<R>(&self, view_id: EntityId, f: impl FnOnce() -> R) -> R {
        self.view_stack.lock().push(view_id);
        let result = f();
        self.view_stack.lock().pop();
        result
    }

    fn parent_view_id(&self) -> Option<EntityId> {
        self.view_stack.lock().last().copied()
    }

    pub fn layout_wrapped_line(
        &self,
        text: &str,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
    ) -> Arc<WrappedLineLayout> {
        let key = &CacheKeyRef {
            text,
            font_size,
            runs,
            wrap_width,
            parent_view_id: self.parent_view_id(),
        } as &dyn AsCacheKeyRef;

        let current_frame = self.current_frame_wrapped.upgradable_read();
        if let Some(layout) = current_frame.get(key) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
        if let Some((key, layout)) = self.previous_frame_wrapped.lock().remove_entry(key) {
            current_frame.insert(key, layout.clone());
            layout
        } else {
            let unwrapped_layout = self.layout_line(text, font_size, runs);
            let wrap_boundaries = if let Some(wrap_width) = wrap_width {
                unwrapped_layout.compute_wrap_boundaries(text.as_ref(), wrap_width)
            } else {
                SmallVec::new()
            };
            let layout = Arc::new(WrappedLineLayout {
                unwrapped_layout,
                wrap_boundaries,
                wrap_width,
            });
            let key = CacheKey {
                text: text.into(),
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
                parent_view_id: self.parent_view_id(),
            };
            current_frame.insert(key, layout.clone());
            layout
        }
    }

    pub fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> Arc<LineLayout> {
        let key = &CacheKeyRef {
            text,
            font_size,
            runs,
            wrap_width: None,
            parent_view_id: self.parent_view_id(),
        } as &dyn AsCacheKeyRef;

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.get(key) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
        if let Some((key, layout)) = self.previous_frame.lock().remove_entry(key) {
            current_frame.insert(key, layout.clone());
            layout
        } else {
            let layout = Arc::new(self.platform_text_system.layout_line(text, font_size, runs));
            let key = CacheKey {
                text: text.into(),
                font_size,
                runs: SmallVec::from(runs),
                wrap_width: None,
                parent_view_id: self.parent_view_id(),
            };
            current_frame.insert(key, layout.clone());
            layout
        }
    }
}

/// A run of text with a single font.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontRun {
    pub(crate) len: usize,
    pub(crate) font_id: FontId,
}

trait AsCacheKeyRef {
    fn as_cache_key_ref(&self) -> CacheKeyRef;
}

#[derive(Debug, Eq)]
struct CacheKey {
    text: String,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
    parent_view_id: Option<EntityId>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
    parent_view_id: Option<EntityId>,
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
            parent_view_id: self.parent_view_id,
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
