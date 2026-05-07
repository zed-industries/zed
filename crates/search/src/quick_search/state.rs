use super::LayoutMode;
use gpui::{Global, px};
use ui::prelude::*;

pub(crate) struct StackedLayoutState {
    pub(crate) results_height: Pixels,
    pub(crate) preview_height: Pixels,
}

impl StackedLayoutState {
    pub(crate) const DEFAULT_MODAL_WIDTH_REMS: f32 = 42.0;
    pub(crate) const MIN_MODAL_WIDTH_REMS: f32 = 30.0;
    pub(crate) const MAX_MODAL_WIDTH_REMS: f32 = 70.0;
    pub(crate) const DEFAULT_RESULTS_HEIGHT: f32 = 180.0;
    pub(crate) const DEFAULT_PREVIEW_HEIGHT: f32 = 280.0;
    pub(crate) const MIN_PANEL_HEIGHT: f32 = 80.0;

    pub(crate) fn new() -> Self {
        Self {
            results_height: px(Self::DEFAULT_RESULTS_HEIGHT),
            preview_height: px(Self::DEFAULT_PREVIEW_HEIGHT),
        }
    }
}

pub(crate) struct TelescopeLayoutState {
    pub(crate) content_height: Pixels,
    pub(crate) preview_width: Pixels,
}

impl TelescopeLayoutState {
    pub(crate) const DEFAULT_MODAL_WIDTH_REMS: f32 = 60.0;
    pub(crate) const MIN_MODAL_WIDTH_REMS: f32 = 45.0;
    pub(crate) const MAX_MODAL_WIDTH_REMS: f32 = 90.0;
    pub(crate) const DEFAULT_CONTENT_HEIGHT: f32 = 400.0;
    pub(crate) const MIN_CONTENT_HEIGHT: f32 = 200.0;
    pub(crate) const MAX_CONTENT_HEIGHT: f32 = 800.0;
    pub(crate) const DEFAULT_PREVIEW_WIDTH: f32 = 600.0;
    pub(crate) const MIN_PREVIEW_WIDTH: f32 = 200.0;
    pub(crate) const MAX_PREVIEW_WIDTH: f32 = 800.0;

    pub(crate) fn new() -> Self {
        Self {
            content_height: px(Self::DEFAULT_CONTENT_HEIGHT),
            preview_width: px(Self::DEFAULT_PREVIEW_WIDTH),
        }
    }
}

#[derive(Clone)]
pub(crate) struct SavedQuickSearchLayout {
    pub(crate) modal_width: Pixels,
    pub(crate) layout_mode: LayoutMode,
    pub(crate) stacked_results_height: Pixels,
    pub(crate) stacked_preview_height: Pixels,
    pub(crate) telescope_content_height: Pixels,
    pub(crate) telescope_preview_width: Pixels,
}

impl Global for SavedQuickSearchLayout {}
