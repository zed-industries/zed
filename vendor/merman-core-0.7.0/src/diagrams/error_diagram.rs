use crate::{
    ParseMetadata, Result, common_db,
    diagram::{ParsedDiagram, ParsedDiagramRender, RenderSemanticModel},
};
use serde_json::{Value, json};

pub fn parse_error(_code: &str, meta: &ParseMetadata) -> Result<Value> {
    Ok(error_model(meta))
}

pub(crate) fn suppressed_error_diagram(source_meta: &ParseMetadata) -> ParsedDiagram {
    let (meta, model) = suppressed_error_parts(source_meta);
    ParsedDiagram { meta, model }
}

pub(crate) fn suppressed_error_render_diagram(source_meta: &ParseMetadata) -> ParsedDiagramRender {
    let (meta, model) = suppressed_error_parts(source_meta);
    ParsedDiagramRender {
        meta,
        model: RenderSemanticModel::Json(model),
    }
}

fn suppressed_error_parts(source_meta: &ParseMetadata) -> (ParseMetadata, Value) {
    let mut meta = source_meta.clone();
    meta.diagram_type = "error".to_string();

    let mut model = error_model(&meta);
    common_db::apply_common_db_sanitization(&mut model, &meta.effective_config);

    (meta, model)
}

fn error_model(meta: &ParseMetadata) -> Value {
    json!({
        "type": meta.diagram_type,
    })
}
