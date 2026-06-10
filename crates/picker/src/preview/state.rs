use crate::{ViewPortHeight, ViewPortWidth};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LayoutMode {
    Hidden,
    Stacked(StackedLayout),
    Telescope(TelescopeLayout),
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self::Hidden
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StackedLayout {
    pub(crate) preview_size: ViewPortHeight,
}

impl StackedLayout {
    pub(crate) fn new() -> Self {
        Self {
            preview_size: ViewPortHeight(0.3),
        }
    }
}

impl Default for StackedLayout {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TelescopeLayout {
    pub(crate) preview_size: ViewPortWidth,
}

impl TelescopeLayout {
    pub(crate) fn new() -> Self {
        Self {
            preview_size: ViewPortWidth(0.3),
        }
    }
}

impl Default for TelescopeLayout {
    fn default() -> Self {
        Self::new()
    }
}
