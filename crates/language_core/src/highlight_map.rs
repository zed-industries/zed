use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[HighlightId]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(pub u32);

const DEFAULT_SYNTAX_HIGHLIGHT_ID: HighlightId = HighlightId(u32::MAX);

impl HighlightMap {
    pub fn from_ids(highlight_ids: impl Into<Arc<[HighlightId]>>) -> Self {
        Self(highlight_ids.into())
    }

    pub fn get(&self, capture_id: u32) -> HighlightId {
        self.0
            .get(capture_id as usize)
            .copied()
            .unwrap_or(DEFAULT_SYNTAX_HIGHLIGHT_ID)
    }
}

impl HighlightId {
    pub const TABSTOP_INSERT_ID: HighlightId = HighlightId(u32::MAX - 1);
    pub const TABSTOP_REPLACE_ID: HighlightId = HighlightId(u32::MAX - 2);

    pub fn is_default(&self) -> bool {
        *self == DEFAULT_SYNTAX_HIGHLIGHT_ID
    }

    /// Returns the underlying index. Useful for extension traits that need
    /// to look up theme data by index.
    pub fn index(&self) -> u32 {
        self.0
    }
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}

impl Default for HighlightId {
    fn default() -> Self {
        DEFAULT_SYNTAX_HIGHLIGHT_ID
    }
}
