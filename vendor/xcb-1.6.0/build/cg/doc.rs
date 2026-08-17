use super::CodeGen;
use crate::cg;
use crate::ir;

use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub text: String,
}

impl DocField {
    pub(super) fn emit<O: Write>(&self, out: &mut O, ind: u32) -> io::Result<()> {
        emit_doc_text(out, ind, &self.text)?;
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocError {
    pub typ: String,
    pub text: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DocSee {
    pub typ: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
    pub fields: Vec<DocField>,
    pub errors: Vec<DocError>,
    pub sees: Vec<DocSee>,
}

impl Doc {
    pub(super) fn lookup_field(&self, name: &str) -> Option<&DocField> {
        self.fields.iter().find(|df| df.name == name)
    }

    pub(super) fn emit<O: Write>(&self, out: &mut O, ind: u32) -> io::Result<()> {
        let mut empty = true;
        if let Some(brief) = &self.brief {
            emit_doc_text(out, ind, brief)?;
            empty = false;
        }
        if let Some(desc) = &self.description {
            if !empty {
                emit_doc_text(out, ind, "")?;
            }
            emit_doc_text(out, ind, desc)?;
            empty = false;
        }
        if let Some(example) = &self.example {
            if example.contains("fn main") {
                if !empty {
                    emit_doc_text(out, ind, "")?;
                }
                emit_doc_text(out, ind, "# Example")?;
                emit_doc_text(out, ind, "```no_run")?;
                emit_doc_text(out, ind, example)?;
                emit_doc_text(out, ind, "```")?;
            }
        }
        Ok(())
    }
}

pub(super) fn emit_doc_text<O: Write>(out: &mut O, ind: u32, text: &str) -> io::Result<()> {
    for s in text.split('\n') {
        let s = s.trim_end();
        if !s.is_empty() {
            writeln!(out, "{}/// {}", cg::ind(ind), s.trim_end())?;
        } else {
            writeln!(out, "{}///", cg::ind(ind))?;
        }
    }
    Ok(())
}

impl CodeGen {
    pub(super) fn resolve_doc(&self, doc: Option<ir::Doc>) -> Option<Doc> {
        doc.map(|doc| Doc {
            brief: doc.brief,
            description: doc.description,
            example: doc.example,
            fields: doc
                .fields
                .into_iter()
                .map(|df| DocField {
                    name: cg::rust_field_name(&df.name),
                    text: df.text,
                })
                .collect(),
            errors: doc
                .errors
                .into_iter()
                .map(|de| DocError {
                    typ: de.typ,
                    text: de.text,
                })
                .collect(),
            sees: doc
                .sees
                .into_iter()
                .map(|de| DocSee {
                    typ: de.typ,
                    name: de.name,
                })
                .collect(),
        })
    }

    pub(super) fn doc_lookup_field(&self, doc: Option<&Doc>, name: &str) -> Option<DocField> {
        if let Some(doc) = doc {
            doc.lookup_field(name).cloned()
        } else {
            None
        }
    }
}
