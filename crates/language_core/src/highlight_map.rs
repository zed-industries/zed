use smallvec::SmallVec;
use std::{num::NonZeroU32, ops::Range, sync::Arc};

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[Option<HighlightId>]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CaptureId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedRange {
    pub range: Range<usize>,
    pub capture_ids: SmallVec<[CaptureId; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(NonZeroU32);

impl HighlightId {
    pub const TABSTOP_INSERT_ID: HighlightId = HighlightId(NonZeroU32::new(u32::MAX - 1).unwrap());
    pub const TABSTOP_REPLACE_ID: HighlightId = HighlightId(NonZeroU32::new(u32::MAX - 2).unwrap());

    pub fn new(capture_id: u32) -> Self {
        Self(NonZeroU32::new(capture_id + 1).unwrap_or(NonZeroU32::MAX))
    }
}

impl From<HighlightId> for usize {
    fn from(value: HighlightId) -> Self {
        value.0.get() as usize - 1
    }
}

impl HighlightMap {
    #[inline]
    pub fn from_ids(highlight_ids: impl IntoIterator<Item = Option<HighlightId>>) -> Self {
        Self(highlight_ids.into_iter().collect())
    }

    #[inline]
    pub fn get(&self, capture_id: CaptureId) -> Option<HighlightId> {
        self.0.get(capture_id.0 as usize).copied().flatten()
    }

    pub fn get_innermost(&self, capture_ids: &[CaptureId]) -> Option<HighlightId> {
        capture_ids
            .iter()
            .rev()
            .find_map(|&capture_id| self.get(capture_id))
    }
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}
