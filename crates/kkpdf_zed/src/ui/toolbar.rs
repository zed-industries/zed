//! PDF viewer toolbar control models and layout definitions.
//!
//! Provides navigation, zoom presets, page jump controls, and dark mode toggles.

use crate::settings::PageLayoutMode;

/// User actions emitted from toolbar buttons and inputs.
#[derive(Debug, Clone, PartialEq)]
pub enum PdfToolbarAction {
    ZoomIn,
    ZoomOut,
    ResetZoom,
    FitToWidth,
    FitToPage,
    SetZoomPercentage(u32),
    PreviousPage,
    NextPage,
    GoToPage(usize),
    ToggleDarkMode,
    SetLayoutMode(PageLayoutMode),
    Reload,
}

/// Read-only snapshot of viewer state consumed by the toolbar UI renderer.
#[derive(Debug, Clone, PartialEq)]
pub struct PdfToolbarState {
    /// Active 1-indexed page number.
    pub current_page: usize,
    /// Total pages in document.
    pub total_pages: usize,
    /// Current zoom percentage (e.g. 100 for 100%).
    pub zoom_percentage: u32,
    /// Whether smart dark mode tone mapping is enabled.
    pub dark_mode: bool,
    /// Current layout mode.
    pub layout_mode: PageLayoutMode,
    /// Whether previous page action is available.
    pub can_prev_page: bool,
    /// Whether next page action is available.
    pub can_next_page: bool,
}

impl PdfToolbarState {
    /// Constructs a toolbar state from active parameters.
    pub fn new(
        current_page: usize,
        total_pages: usize,
        zoom_percentage: u32,
        dark_mode: bool,
        layout_mode: PageLayoutMode,
    ) -> Self {
        let can_prev_page = current_page > 1;
        let can_next_page = total_pages > 0 && current_page < total_pages;

        Self {
            current_page,
            total_pages,
            zoom_percentage,
            dark_mode,
            layout_mode,
            can_prev_page,
            can_next_page,
        }
    }

    /// Returns formatted page status string (e.g. "Page 3 of 42").
    pub fn page_display_text(&self) -> String {
        if self.total_pages == 0 {
            "0 / 0".to_string()
        } else {
            format!("{} / {}", self.current_page, self.total_pages)
        }
    }

    /// Returns formatted zoom percentage string (e.g. "125%").
    pub fn zoom_display_text(&self) -> String {
        format!("{}%", self.zoom_percentage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_state_text_and_navigation_flags() {
        let state = PdfToolbarState::new(1, 10, 100, false, PageLayoutMode::Continuous);
        assert_eq!(state.page_display_text(), "1 / 10");
        assert_eq!(state.zoom_display_text(), "100%");
        assert!(!state.can_prev_page);
        assert!(state.can_next_page);

        let mid_state = PdfToolbarState::new(5, 10, 150, true, PageLayoutMode::SinglePage);
        assert!(mid_state.can_prev_page);
        assert!(mid_state.can_next_page);
        assert_eq!(mid_state.zoom_display_text(), "150%");

        let last_state = PdfToolbarState::new(10, 10, 100, false, PageLayoutMode::Continuous);
        assert!(last_state.can_prev_page);
        assert!(!last_state.can_next_page);
    }
}
