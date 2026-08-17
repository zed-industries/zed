use crate::Result;
use crate::model::ErrorDiagramLayout;
use crate::text::TextMeasurer;

pub const UPSTREAM_MERMAID_VERSION: &str = merman_core::baseline::PINNED_MERMAID_BASELINE_VERSION;

pub fn layout_error_diagram(
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    _measurer: &dyn TextMeasurer,
) -> Result<ErrorDiagramLayout> {
    Ok(ErrorDiagramLayout {
        viewbox_width: 2412.0,
        viewbox_height: 512.0,
        max_width_px: 512.0,
    })
}
