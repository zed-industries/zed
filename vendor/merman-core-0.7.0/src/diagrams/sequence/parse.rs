use crate::{Error, ParseMetadata, Result};
use serde_json::Value;

use super::SequenceDiagramRenderModel;
use super::db::{SequenceDb, fast_parse_sequence_signals_only_db};
use super::lexer::Lexer;
use super::sequence_grammar;

pub fn parse_sequence(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let db = parse_sequence_db(code, meta)?;
    Ok(db.into_model(meta))
}

pub fn parse_sequence_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<SequenceDiagramRenderModel> {
    let db = parse_sequence_db(code, meta)?;
    Ok(db.into_render_model())
}

fn parse_sequence_db(code: &str, meta: &ParseMetadata) -> Result<SequenceDb> {
    let wrap_enabled = meta
        .effective_config
        .as_value()
        .get("wrap")
        .and_then(|v| v.as_bool())
        .or_else(|| {
            meta.effective_config
                .as_value()
                .get("sequence")
                .and_then(|v| v.get("wrap"))
                .and_then(|v| v.as_bool())
        });

    if let Some(db) = fast_parse_sequence_signals_only_db(code, wrap_enabled) {
        return Ok(db);
    }

    let actions = sequence_grammar::ActionsParser::new()
        .parse(Lexer::new(code))
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("{e:?}"),
        })?;

    let mut db = SequenceDb::new(wrap_enabled);
    for a in actions {
        db.apply(a).map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: e,
        })?;
    }

    Ok(db)
}
