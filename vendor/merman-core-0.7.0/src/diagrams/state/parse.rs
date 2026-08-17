use crate::{Error, ParseMetadata, Result};
use serde_json::Value;

use super::db::StateDb;
use super::{Lexer, StateDiagramRenderModel, Stmt};

pub fn parse_state(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let mut doc = super::state_grammar::RootParser::new()
        .parse(Lexer::new(code))
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("{e:?}"),
        })?;

    let mut divider_cnt = 0usize;
    assign_divider_ids(&mut doc, &mut divider_cnt);

    let mut db = StateDb::new();
    db.set_root_doc(doc);
    db.to_model(meta)
}

pub fn parse_state_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<StateDiagramRenderModel> {
    let mut doc = super::state_grammar::RootParser::new()
        .parse(Lexer::new(code))
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("{e:?}"),
        })?;

    let mut divider_cnt = 0usize;
    assign_divider_ids(&mut doc, &mut divider_cnt);

    let mut db = StateDb::new();
    db.set_root_doc(doc);
    db.to_model_for_render_typed(meta)
}

fn assign_divider_ids(stmts: &mut [Stmt], cnt: &mut usize) {
    let mut stack = vec![stmts.iter_mut()];
    while let Some(iter) = stack.last_mut() {
        let Some(stmt) = iter.next() else {
            stack.pop();
            continue;
        };

        match stmt {
            Stmt::State(st) => {
                if st.ty == "divider" && st.id == "__divider__" {
                    *cnt += 1;
                    st.id = format!("divider-id-{cnt}");
                }
                if let Some(doc) = st.doc.as_mut() {
                    stack.push(doc.iter_mut());
                }
            }
            Stmt::Relation(relation) => {
                if relation.state1.ty == "divider" && relation.state1.id == "__divider__" {
                    *cnt += 1;
                    relation.state1.id = format!("divider-id-{cnt}");
                }
                if relation.state2.ty == "divider" && relation.state2.id == "__divider__" {
                    *cnt += 1;
                    relation.state2.id = format!("divider-id-{cnt}");
                }
            }
            _ => {}
        }
    }
}
