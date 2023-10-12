use crate::{FontId, Pixels, PlatformTextSystem, ShapedGlyph, ShapedLine, ShapedRun};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub(crate) struct TextLayoutCache {
    prev_frame: Mutex<HashMap<CacheKeyValue, Arc<ShapedLine>>>,
    curr_frame: RwLock<HashMap<CacheKeyValue, Arc<ShapedLine>>>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
}

impl TextLayoutCache {
    pub fn new(fonts: Arc<dyn PlatformTextSystem>) -> Self {
        Self {
            prev_frame: Mutex::new(HashMap::new()),
            curr_frame: RwLock::new(HashMap::new()),
            platform_text_system: fonts,
        }
    }

    pub fn end_frame(&self) {
        let mut prev_frame = self.prev_frame.lock();
        let mut curr_frame = self.curr_frame.write();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.clear();
    }

    pub fn layout_line<'a>(
        &'a self,
        text: &'a str,
        font_size: Pixels,
        runs: &[(usize, FontId)],
    ) -> Arc<ShapedLine> {
        let key = &CacheKeyRef {
            text,
            font_size,
            runs,
        } as &dyn CacheKey;
        let curr_frame = self.curr_frame.upgradable_read();
        if let Some(layout) = curr_frame.get(key) {
            return layout.clone();
        }

        let mut curr_frame = RwLockUpgradableReadGuard::upgrade(curr_frame);
        if let Some((key, layout)) = self.prev_frame.lock().remove_entry(key) {
            curr_frame.insert(key, layout.clone());
            layout
        } else {
            let layout = Arc::new(self.platform_text_system.layout_line(text, font_size, runs));
            let key = CacheKeyValue {
                text: text.into(),
                font_size,
                runs: SmallVec::from(runs),
            };
            curr_frame.insert(key, layout.clone());
            layout
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
    runs: SmallVec<[(usize, FontId); 1]>,
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

#[derive(Copy, Clone, PartialEq, Eq)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [(usize, FontId)],
}

impl<'a> CacheKey for CacheKeyRef<'a> {
    fn key(&self) -> CacheKeyRef {
        *self
    }
}

impl<'a> Hash for CacheKeyRef<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.font_size.hash(state);
        for (len, font_id) in self.runs {
            len.hash(state);
            font_id.hash(state);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapedBoundary {
    pub run_ix: usize,
    pub glyph_ix: usize,
}

impl ShapedRun {
    pub fn glyphs(&self) -> &[ShapedGlyph] {
        &self.glyphs
    }
}
