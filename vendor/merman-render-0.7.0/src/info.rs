use crate::Result;
use crate::model::{Bounds, InfoDiagramLayout};
use crate::text::TextMeasurer;
use merman_core::baseline::PINNED_MERMAID_BASELINE_VERSION;
use merman_core::diagrams::info::InfoDiagramRenderModel;

pub fn layout_info_diagram(
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    _measurer: &dyn TextMeasurer,
) -> Result<InfoDiagramLayout> {
    let _ = semantic;
    layout_info_diagram_typed(
        &InfoDiagramRenderModel::default(),
        _effective_config,
        _measurer,
    )
}

pub fn layout_info_diagram_typed(
    model: &InfoDiagramRenderModel,
    _effective_config: &serde_json::Value,
    _measurer: &dyn TextMeasurer,
) -> Result<InfoDiagramLayout> {
    let _ = model.show_info;
    Ok(InfoDiagramLayout {
        bounds: Some(Bounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 400.0,
            max_y: 80.0,
        }),
        version: format!("v{PINNED_MERMAID_BASELINE_VERSION}"),
    })
}
