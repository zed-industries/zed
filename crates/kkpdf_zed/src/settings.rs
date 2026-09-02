//! Configuration and user preferences for the PDF viewer.

use serde::{Deserialize, Serialize};

/// Layout and display mode for viewing pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PageLayoutMode {
    /// Continuous vertical scrolling of all pages.
    #[default]
    Continuous,
    /// Single page view with explicit page turn controls.
    SinglePage,
    /// Side-by-side two-page spread (book mode).
    TwoPageSpread,
}

/// Default zoom policy when opening a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DefaultZoomPolicy {
    /// Fit page width to current pane width.
    #[default]
    FitWidth,
    /// Fit entire page height and width inside pane.
    FitPage,
    /// 100% actual print size (72 DPI).
    ActualSize,
}

/// PDF viewer settings configuration structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfViewerSettings {
    /// Initial zoom behavior when opening a document.
    pub default_zoom: DefaultZoomPolicy,
    /// Page layout mode (continuous scroll, single page, or two-page spread).
    pub layout_mode: PageLayoutMode,
    /// Automatically apply smart luminosity-preserving dark mode when dark theme is active.
    pub smart_dark_mode: bool,
    /// Saturation threshold (0.0 to 1.0) for dark mode tone mapper. Saturated pixels (>= threshold) are kept untouched.
    pub saturation_threshold: f32,
    /// Memory cache budget in megabytes for pre-rendered page bitmaps.
    pub cache_budget_mb: usize,
    /// Debounce duration in milliseconds for hot-reloading when PDF file changes on disk.
    pub reload_debounce_ms: u64,
    /// High-DPI rasterization multiplier for ultra-crisp typography.
    pub target_dpi: f32,
}

impl Default for PdfViewerSettings {
    fn default() -> Self {
        Self {
            default_zoom: DefaultZoomPolicy::FitWidth,
            layout_mode: PageLayoutMode::Continuous,
            smart_dark_mode: true,
            saturation_threshold: 0.18,
            cache_budget_mb: 256,
            reload_debounce_ms: 200,
            target_dpi: 144.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_serialization() {
        let settings = PdfViewerSettings::default();
        let json = serde_json::to_string(&settings).unwrap_or_else(|_| "{}".into());
        assert!(json.contains("fit_width"));
        assert!(json.contains("continuous"));
    }
}
