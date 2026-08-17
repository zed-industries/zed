use crate::models::class_diagram as class_typed;
use crate::{Error, ParseMetadata, Result};
use serde_json::Value;

use super::class_grammar;
use super::db::ClassDb;
use super::fast::parse_class_fast_db;
use super::lexer::Lexer;

fn prefer_fast_class_parser() -> bool {
    match std::env::var("MERMAN_CLASS_PARSER").as_deref() {
        Ok("slow") | Ok("0") | Ok("false") => false,
        Ok("fast") | Ok("1") | Ok("true") => true,
        // Default to "auto": attempt the fast parser and fall back to LALRPOP when it declines.
        _ => true,
    }
}

pub(super) fn parse_class_via_lalrpop_db<'a>(
    code: &str,
    meta: &'a ParseMetadata,
) -> Result<ClassDb<'a>> {
    let actions = class_grammar::ActionsParser::new()
        .parse(Lexer::new(code))
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("{e:?}"),
        })?;

    let mut db = ClassDb::new(&meta.effective_config);
    for a in actions {
        db.apply(a).map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: e,
        })?;
    }
    Ok(db)
}

pub(super) fn parse_class_via_lalrpop(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let db = parse_class_via_lalrpop_db(code, meta)?;
    Ok(db.into_model(meta))
}

pub fn parse_class(code: &str, meta: &ParseMetadata) -> Result<Value> {
    if prefer_fast_class_parser() {
        if let Some(db) = parse_class_fast_db(code, meta)? {
            return Ok(db.into_model(meta));
        }
    }

    parse_class_via_lalrpop(code, meta)
}

pub fn parse_class_typed(code: &str, meta: &ParseMetadata) -> Result<class_typed::ClassDiagram> {
    if prefer_fast_class_parser() {
        if let Some(db) = parse_class_fast_db(code, meta)? {
            return Ok(db.into_typed_model(meta));
        }
    }

    let db = parse_class_via_lalrpop_db(code, meta)?;
    Ok(db.into_typed_model(meta))
}
