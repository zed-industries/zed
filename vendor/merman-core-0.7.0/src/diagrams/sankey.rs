use crate::sanitize::sanitize_text;
use crate::{Error, ParseMetadata, Result};
use serde_json::{Map, Value, json};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SankeyRenderNode {
    pub id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SankeyRenderLink {
    pub source: String,
    pub target: String,
    pub value: Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SankeyRenderGraph {
    #[serde(default)]
    pub nodes: Vec<SankeyRenderNode>,
    #[serde(default)]
    pub links: Vec<SankeyRenderLink>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SankeyDiagramRenderModel {
    #[serde(default)]
    pub graph: SankeyRenderGraph,
}

#[derive(Debug, Default, Clone)]
struct SankeyDb {
    nodes: Vec<SankeyRenderNode>,
    nodes_map: HashMap<String, usize>,
    links: Vec<SankeyRenderLink>,
}

impl SankeyDb {
    fn find_or_create_node(&mut self, id_raw: &str, meta: &ParseMetadata) -> String {
        let id = sanitize_text(id_raw, &meta.effective_config);
        if self.nodes_map.contains_key(&id) {
            return id;
        }
        let idx = self.nodes.len();
        self.nodes.push(SankeyRenderNode { id: id.clone() });
        self.nodes_map.insert(id.clone(), idx);
        id
    }

    fn add_link(&mut self, source: String, target: String, value: Value) {
        self.links.push(SankeyRenderLink {
            source,
            target,
            value,
        });
    }

    #[inline]
    fn graph_value(&self) -> Value {
        json!({
            "nodes": self.nodes.iter().map(|n| json!({"id": n.id})).collect::<Vec<_>>(),
            "links": self.links.iter().map(|l| {
                json!({
                    "source": l.source,
                    "target": l.target,
                    "value": l.value,
                })
            }).collect::<Vec<_>>(),
        })
    }

    #[inline]
    fn into_render_model(self) -> SankeyDiagramRenderModel {
        SankeyDiagramRenderModel {
            graph: SankeyRenderGraph {
                nodes: self.nodes,
                links: self.links,
            },
        }
    }
}

pub fn parse_sankey(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let db = parse_sankey_db(code, meta)?;
    let mut out = Map::with_capacity(3);
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert("graph".to_string(), db.graph_value());
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );
    Ok(Value::Object(out))
}

pub fn parse_sankey_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<SankeyDiagramRenderModel> {
    parse_sankey_db(code, meta).map(SankeyDb::into_render_model)
}

#[inline]
fn parse_sankey_db(code: &str, meta: &ParseMetadata) -> Result<SankeyDb> {
    let prepared = prepare_text_for_parsing(code);

    let (header, rest) = prepared
        .split_once('\n')
        .ok_or_else(|| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected sankey header followed by csv".to_string(),
        })?;

    let header = header.trim();
    if !is_sankey_header(header) {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected sankey".to_string(),
        });
    }

    let mut db = SankeyDb::default();
    let records = parse_csv_records(rest).map_err(|message| Error::DiagramParse {
        diagram_type: "sankey".to_string(),
        message,
    })?;
    if records.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "sankey".to_string(),
            message: "expected at least one csv record".to_string(),
        });
    }

    for (source_raw, target_raw, value_raw) in records {
        let source_raw = normalize_field_value(&source_raw);
        let target_raw = normalize_field_value(&target_raw);
        let value_raw = value_raw.trim();

        let source = db.find_or_create_node(&source_raw, meta);
        let target = db.find_or_create_node(&target_raw, meta);
        let value = parse_float_json(value_raw);
        db.add_link(source, target, value);
    }

    Ok(db)
}

fn is_sankey_header(header: &str) -> bool {
    let h = header.trim_start().to_ascii_lowercase();
    h == "sankey" || h == "sankey-beta"
}

fn normalize_field_value(s: &str) -> String {
    // Mermaid's jison action: `$field.trim().replaceAll('""','"')`
    let trimmed = s.trim();
    trimmed.replace("\"\"", "\"")
}

fn parse_float_json(s: &str) -> Value {
    let t = s.trim();
    if !t.contains(['.', 'e', 'E']) {
        if let Ok(i) = t.parse::<i64>() {
            return Value::Number(i.into());
        }
    }

    let v = t.parse::<f64>().unwrap_or(f64::NAN);
    if !v.is_finite() {
        return Value::Null;
    }

    let Some(n) = serde_json::Number::from_f64(v) else {
        return Value::Null;
    };
    Value::Number(n)
}

fn prepare_text_for_parsing(text: &str) -> String {
    // Mermaid's `prepareTextForParsing`:
    // - `.replaceAll(/^[^\S\n\r]+|[^\S\n\r]+$/g, '')`
    // - `.replaceAll(/([\n\r])+/g, '\n')`
    // - `.trim()`
    let mut s = text.to_string();
    s = trim_non_newline_ws_ends(&s);
    s = collapse_newlines(&s);
    s.trim().to_string()
}

fn trim_non_newline_ws_ends(s: &str) -> String {
    let start = s
        .char_indices()
        .find(|(_, ch)| !ch.is_whitespace() || *ch == '\n' || *ch == '\r')
        .map(|(idx, _)| idx)
        .unwrap_or(s.len());

    let end = s
        .char_indices()
        .rev()
        .find(|(_, ch)| !ch.is_whitespace() || *ch == '\n' || *ch == '\r')
        .map(|(idx, ch)| idx + ch.len_utf8())
        .unwrap_or(0);

    if start >= end {
        return String::new();
    }
    s[start..end].to_string()
}

fn collapse_newlines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\n' || ch == '\r' {
            out.push('\n');
            while chars.peek().is_some_and(|c| *c == '\n' || *c == '\r') {
                chars.next();
            }
            continue;
        }
        out.push(ch);
    }
    out
}

fn parse_csv_records(input: &str) -> std::result::Result<Vec<(String, String, String)>, String> {
    let mut p = CsvParser::new(input);
    let mut records = Vec::new();
    p.consume_newlines();
    while !p.eof() {
        let source = p.parse_field()?;
        p.consume_char(',')?;
        let target = p.parse_field()?;
        p.consume_char(',')?;
        let value = p.parse_field()?;

        // End of record: optional \n or EOF.
        if p.try_consume_newline() {
            // If there are multiple newlines (should be collapsed already), consume them.
            p.consume_newlines();
        } else if !p.eof() {
            return Err("expected end of record".to_string());
        }

        records.push((source, target, value));
    }
    Ok(records)
}

struct CsvParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> CsvParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn rest(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn peek_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn consume_char(&mut self, ch: char) -> std::result::Result<(), String> {
        if self.rest().starts_with(ch) {
            self.pos += ch.len_utf8();
            Ok(())
        } else {
            Err(format!("expected '{ch}'"))
        }
    }

    fn consume_newlines(&mut self) {
        while self.try_consume_newline() {}
    }

    fn try_consume_newline(&mut self) -> bool {
        match self.peek_char() {
            Some('\n') => {
                self.pos += 1;
                true
            }
            Some('\r') => {
                self.pos += 1;
                if self.peek_char() == Some('\n') {
                    self.pos += 1;
                }
                true
            }
            _ => false,
        }
    }

    fn parse_field(&mut self) -> std::result::Result<String, String> {
        match self.peek_char() {
            Some('"') => self.parse_quoted_field(),
            Some('\n' | '\r') => Ok(String::new()),
            None => Ok(String::new()),
            _ => self.parse_unquoted_field(),
        }
    }

    fn parse_unquoted_field(&mut self) -> std::result::Result<String, String> {
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == ',' || ch == '\n' || ch == '\r' {
                break;
            }
            out.push(ch);
            self.pos += ch.len_utf8();
        }
        Ok(out)
    }

    fn parse_quoted_field(&mut self) -> std::result::Result<String, String> {
        self.consume_char('"')?;
        let mut out = String::new();
        while let Some(ch) = self.peek_char() {
            self.pos += ch.len_utf8();
            if ch == '"' {
                if self.peek_char() == Some('"') {
                    // Escaped quote
                    self.pos += 1;
                    out.push('"');
                    continue;
                }
                // Closing quote
                return Ok(out);
            }
            out.push(ch);
        }
        Err("unterminated quoted field".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions};
    use futures::executor::block_on;
    use serde_json::json;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    #[test]
    fn sankey_parses_csv_with_sankey_beta_header() {
        let model = parse(
            r#"sankey-beta

%% comment line should be removed
    Agricultural 'waste',Bio-conversion,124.729   
Bio-conversion,Liquid,0.597

%% quoted sankey keyword
"sankey",target,10

%% escaped quotes
"""Biomass imports""",Solid,35

%% commas in field
"District heating","Heating and cooling, commercial",22.505
"#,
        );

        let graph = &model["graph"];
        assert!(graph["nodes"].as_array().unwrap().len() >= 5);
        assert_eq!(
            graph["links"][0],
            json!({
                "source": "Agricultural 'waste'",
                "target": "Bio-conversion",
                "value": 124.729,
            })
        );
        assert_eq!(
            graph["links"][2],
            json!({
                "source": "sankey",
                "target": "target",
                "value": 10,
            })
        );
        assert_eq!(
            graph["links"][3],
            json!({
                "source": "\"Biomass imports\"",
                "target": "Solid",
                "value": 35,
            })
        );
        assert_eq!(
            graph["links"][4],
            json!({
                "source": "District heating",
                "target": "Heating and cooling, commercial",
                "value": 22.505,
            })
        );
    }

    #[test]
    fn sankey_parses_csv_with_sankey_header() {
        let model = parse(
            r#"sankey
A,B,0.597
"#,
        );
        assert_eq!(
            model["graph"],
            json!({
                "nodes": [{"id": "A"}, {"id": "B"}],
                "links": [{"source": "A", "target": "B", "value": 0.597}],
            })
        );
    }

    #[test]
    fn sankey_allows_proto_as_id() {
        let model = parse(
            r#"sankey-beta
__proto__,A,0.597
A,__proto__,0.403
"#,
        );
        let nodes = model["graph"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|n| n["id"].as_str())
            .collect::<Vec<_>>();
        assert!(nodes.contains(&"__proto__"));
    }
}
