use std::{num::NonZeroU32, sync::Arc};

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[Option<HighlightId>]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CaptureId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightCaptureRef {
    pub grammar_index: usize,
    pub capture_id: CaptureId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(NonZeroU32);

impl HighlightId {
    const MONOSPACE_BIT: u32 = 1 << 30;
    const NO_THEME_STYLE: u32 = 1 << 29;

    pub const TABSTOP_INSERT_ID: HighlightId = HighlightId(NonZeroU32::new(u32::MAX - 1).unwrap());
    pub const TABSTOP_REPLACE_ID: HighlightId = HighlightId(NonZeroU32::new(u32::MAX - 2).unwrap());

    pub fn new(capture_id: u32) -> Self {
        Self(NonZeroU32::new(capture_id + 1).unwrap_or(NonZeroU32::MAX))
    }

    pub fn monospace_only() -> Self {
        Self(NonZeroU32::new(Self::NO_THEME_STYLE | Self::MONOSPACE_BIT).unwrap())
    }

    pub fn with_monospace(self, monospace: bool) -> Self {
        if monospace && self != Self::TABSTOP_INSERT_ID && self != Self::TABSTOP_REPLACE_ID {
            Self(NonZeroU32::new(self.0.get() | Self::MONOSPACE_BIT).unwrap())
        } else {
            self
        }
    }

    pub fn is_monospace(self) -> bool {
        self != Self::TABSTOP_INSERT_ID
            && self != Self::TABSTOP_REPLACE_ID
            && (self.0.get() & Self::MONOSPACE_BIT) != 0
    }

    pub fn has_style(self) -> bool {
        self == Self::TABSTOP_INSERT_ID
            || self == Self::TABSTOP_REPLACE_ID
            || (self.0.get() & Self::NO_THEME_STYLE) == 0
    }
}

impl From<HighlightId> for usize {
    fn from(value: HighlightId) -> Self {
        if value == HighlightId::TABSTOP_INSERT_ID || value == HighlightId::TABSTOP_REPLACE_ID {
            return value.0.get() as usize - 1;
        }
        let raw = value.0.get();
        if (raw & HighlightId::NO_THEME_STYLE) != 0 {
            usize::MAX
        } else {
            ((raw & !HighlightId::MONOSPACE_BIT) - 1) as usize
        }
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

    #[inline]
    pub fn same(&self, other: &HighlightMap) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}
