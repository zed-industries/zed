use crate::sanitize::sanitize_text;
use crate::{Error, MermaidConfig, ParseMetadata, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

const NODE_TYPE_DEFAULT: i32 = 0;
const NODE_TYPE_ROUNDED_RECT: i32 = 1;
const NODE_TYPE_RECT: i32 = 2;
const NODE_TYPE_CIRCLE: i32 = 3;
const NODE_TYPE_CLOUD: i32 = 4;
const NODE_TYPE_BANG: i32 = 5;
const NODE_TYPE_HEXAGON: i32 = 6;

#[derive(Debug, Clone)]
struct KanbanNode {
    id: String,
    level: usize,
    label: String,
    width: i64,
    padding: i64,
    parent_id: Option<String>,

    ticket: Option<String>,
    priority: Option<String>,
    assigned: Option<String>,
    icon: Option<String>,
    css_classes: Option<String>,
    shape: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KanbanDiagramRenderModel {
    #[serde(default)]
    pub nodes: Vec<KanbanRenderNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KanbanRenderNode {
    pub id: String,
    pub label: String,
    #[serde(default, rename = "isGroup")]
    pub is_group: bool,
    #[serde(default, rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub assigned: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Default)]
struct KanbanDb {
    nodes: Vec<KanbanNode>,
    section_indices: Vec<usize>,
    next_auto_id: i64,
}

impl KanbanDb {
    fn clear(&mut self) {
        *self = Self::default();
    }

    fn get_section_index(&self, level: usize) -> Result<Option<usize>> {
        if self.nodes.is_empty() {
            return Ok(None);
        }

        let section_level = self.nodes[0].level;
        let mut last_section_idx: Option<usize> = None;
        for (idx, node) in self.nodes.iter().enumerate().rev() {
            if node.level == section_level && last_section_idx.is_none() {
                last_section_idx = Some(idx);
            }
            if node.level < section_level {
                return Err(Error::DiagramParse {
                    diagram_type: "kanban".to_string(),
                    message: format!(
                        "Items without section detected, found section (\"{}\")",
                        node.label
                    ),
                });
            }
        }

        let Some(last_section_idx) = last_section_idx else {
            return Ok(None);
        };

        if level == self.nodes[last_section_idx].level {
            return Ok(None);
        }
        Ok(Some(last_section_idx))
    }

    fn decorate_last(
        &mut self,
        class: Option<String>,
        icon: Option<String>,
        config: &MermaidConfig,
    ) {
        let Some(last) = self.nodes.last_mut() else {
            return;
        };
        if let Some(icon) = icon {
            last.icon = Some(sanitize_text(&icon, config));
        }
        if let Some(class) = class {
            last.css_classes = Some(sanitize_text(&class, config));
        }
    }

    fn add_node(
        &mut self,
        level: usize,
        id_raw: &str,
        descr_raw: &str,
        ty: i32,
        shape_data: Option<String>,
        config: &MermaidConfig,
    ) -> Result<()> {
        let mut padding = get_i64(config, "mindmap.padding").unwrap_or(10);
        let width = get_i64(config, "mindmap.maxNodeWidth").unwrap_or(200);
        match ty {
            NODE_TYPE_ROUNDED_RECT | NODE_TYPE_RECT | NODE_TYPE_HEXAGON => {
                padding *= 2;
            }
            _ => {}
        }

        let mut id = sanitize_text(id_raw, config);
        if id.is_empty() {
            id = format!("kbn{}", self.next_auto_id);
            self.next_auto_id += 1;
        }

        let mut node = KanbanNode {
            id,
            level,
            label: sanitize_text(descr_raw, config),
            width,
            padding,
            parent_id: None,
            ticket: None,
            priority: None,
            assigned: None,
            icon: None,
            css_classes: None,
            shape: None,
        };

        if let Some(shape_data) = shape_data {
            apply_shape_data(&mut node, &shape_data)?;
        }

        if let Some(section_idx) = self.get_section_index(level)? {
            node.parent_id = Some(self.nodes[section_idx].id.clone());
        } else {
            self.section_indices.push(self.nodes.len());
        }
        self.nodes.push(node);
        Ok(())
    }

    fn sections_json(&self) -> Value {
        let mut out = Vec::new();
        for &idx in &self.section_indices {
            let Some(n) = self.nodes.get(idx) else {
                continue;
            };
            let mut obj = Map::new();
            obj.insert("id".to_string(), json!(n.id));
            obj.insert("label".to_string(), json!(n.label));
            obj.insert("level".to_string(), json!(n.level));
            obj.insert("width".to_string(), json!(n.width));
            obj.insert("padding".to_string(), json!(n.padding));
            obj.insert("isGroup".to_string(), json!(false));
            if let Some(v) = &n.ticket {
                obj.insert("ticket".to_string(), json!(v));
            }
            if let Some(v) = &n.priority {
                obj.insert("priority".to_string(), json!(v));
            }
            if let Some(v) = &n.assigned {
                obj.insert("assigned".to_string(), json!(v));
            }
            if let Some(v) = &n.icon {
                obj.insert("icon".to_string(), json!(v));
            }
            if let Some(v) = &n.css_classes {
                obj.insert("cssClasses".to_string(), json!(v));
            }
            if let Some(v) = &n.shape {
                obj.insert("shape".to_string(), json!(v));
            }
            out.push(Value::Object(obj));
        }
        Value::Array(out)
    }

    fn data_nodes_json(&self, config: &MermaidConfig) -> Vec<Value> {
        let look = config.get_str("look").unwrap_or("classic").to_string();

        let mut out = Vec::new();
        for &section_idx in &self.section_indices {
            let Some(section) = self.nodes.get(section_idx) else {
                continue;
            };
            out.push(json!({
                "id": section.id,
                "label": sanitize_text(&section.label, config),
                "isGroup": true,
                "ticket": section.ticket,
                "shape": "kanbanSection",
                "level": section.level,
                "look": look,
            }));

            for item in self
                .nodes
                .iter()
                .filter(|n| n.parent_id.as_deref() == Some(&section.id))
            {
                out.push(json!({
                    "id": item.id,
                    "parentId": section.id,
                    "label": sanitize_text(&item.label, config),
                    "isGroup": false,
                    "ticket": item.ticket,
                    "priority": item.priority,
                    "assigned": item.assigned,
                    "icon": item.icon,
                    "shape": "kanbanItem",
                    "level": item.level,
                    "rx": 5,
                    "ry": 5,
                    "cssStyles": ["text-align: left"],
                }));
            }
        }
        out
    }

    fn data_nodes_for_render(&self, config: &MermaidConfig) -> Vec<KanbanRenderNode> {
        let mut out = Vec::new();
        for &section_idx in &self.section_indices {
            let Some(section) = self.nodes.get(section_idx) else {
                continue;
            };
            out.push(KanbanRenderNode {
                id: section.id.clone(),
                label: sanitize_text(&section.label, config),
                is_group: true,
                parent_id: None,
                ticket: section.ticket.clone(),
                priority: None,
                assigned: None,
                icon: None,
            });

            for item in self
                .nodes
                .iter()
                .filter(|n| n.parent_id.as_deref() == Some(&section.id))
            {
                out.push(KanbanRenderNode {
                    id: item.id.clone(),
                    label: sanitize_text(&item.label, config),
                    is_group: false,
                    parent_id: Some(section.id.clone()),
                    ticket: item.ticket.clone(),
                    priority: item.priority.clone(),
                    assigned: item.assigned.clone(),
                    icon: item.icon.clone(),
                });
            }
        }
        out
    }
}

fn apply_shape_data(node: &mut KanbanNode, shape_data: &str) -> Result<()> {
    let yaml_data = if !shape_data.contains('\n') {
        format!("{{\n{shape_data}\n}}")
    } else {
        format!("{shape_data}\n")
    };

    let doc: serde_yaml::Value =
        serde_yaml::from_str(&yaml_data).map_err(|e| Error::DiagramParse {
            diagram_type: "kanban".to_string(),
            message: e.to_string(),
        })?;

    let doc = serde_json::to_value(doc).unwrap_or(Value::Null);
    let Some(obj) = doc.as_object() else {
        return Ok(());
    };

    if let Some(Value::String(shape)) = obj.get("shape") {
        if shape != &shape.to_lowercase() || shape.contains('_') {
            return Err(Error::DiagramParse {
                diagram_type: "kanban".to_string(),
                message: format!("No such shape: {shape}. Shape names should be lowercase."),
            });
        }
        if shape == "kanbanItem" {
            node.shape = Some(shape.clone());
        }
    }

    if let Some(Value::String(label)) = obj.get("label") {
        node.label = label.clone();
    }
    if let Some(icon) = obj.get("icon") {
        node.icon = Some(value_to_string(icon));
    }
    if let Some(assigned) = obj.get("assigned") {
        node.assigned = Some(value_to_string(assigned));
    }
    if let Some(ticket) = obj.get("ticket") {
        node.ticket = Some(value_to_string(ticket));
    }
    if let Some(priority) = obj.get("priority") {
        node.priority = Some(value_to_string(priority));
    }

    Ok(())
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        other => other.to_string(),
    }
}

fn starts_with_case_insensitive(haystack: &str, needle: &str) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    haystack
        .as_bytes()
        .iter()
        .take(needle.len())
        .copied()
        .map(|b| b.to_ascii_lowercase())
        .eq(needle
            .as_bytes()
            .iter()
            .copied()
            .map(|b| b.to_ascii_lowercase()))
}

fn split_indent(line: &str) -> (usize, &str) {
    let mut indent_chars = 0usize;
    let mut byte_idx = line.len();
    for (idx, ch) in line.char_indices() {
        if ch.is_whitespace() {
            indent_chars += 1;
            continue;
        }
        byte_idx = idx;
        break;
    }
    if indent_chars == 0 {
        byte_idx = 0;
    } else if byte_idx == line.len() {
        byte_idx = line.len();
    }
    (indent_chars, &line[byte_idx..])
}

fn strip_inline_comment(line: &str) -> &str {
    let mut in_quote = false;
    let mut in_backtick_quote = false;

    let mut it = line.char_indices().peekable();
    while let Some((idx, ch)) = it.next() {
        if in_backtick_quote {
            if ch == '`' && it.peek().is_some_and(|(_, next)| *next == '"') {
                in_backtick_quote = false;
                it.next();
            }
            continue;
        }

        if in_quote {
            if ch == '"' {
                in_quote = false;
            }
            continue;
        }

        if ch == '"' {
            if it.peek().is_some_and(|(_, next)| *next == '`') {
                in_backtick_quote = true;
                it.next();
                continue;
            }
            in_quote = true;
            continue;
        }

        if ch == '%' && it.peek().is_some_and(|(_, next)| *next == '%') {
            return &line[..idx];
        }
    }

    line
}

fn parse_node_spec(input: &str) -> std::result::Result<(String, String, i32), String> {
    let input = input.trim_end();
    if input.is_empty() {
        return Err("expected node".to_string());
    }

    if let Some((start, end)) = node_delimiter_pair_at_start(input) {
        let (inner, tail) = extract_delimited(input, start, end)?;
        if !tail.trim().is_empty() {
            return Err("unexpected trailing input".to_string());
        }
        let descr = unquote_node_descr(inner);
        let ty = node_type_for(start, end);
        return Ok((descr.clone(), descr, ty));
    }

    let (id_raw, rest) = split_node_id(input);
    let id_raw = id_raw.to_string();
    let rest = rest.trim_end();
    if rest.is_empty() {
        return Ok((id_raw.clone(), id_raw, NODE_TYPE_DEFAULT));
    }

    let Some((start, end)) = node_delimiter_pair_at_start(rest) else {
        return Err("expected node delimiter".to_string());
    };

    let (inner, tail) = extract_delimited(rest, start, end)?;
    if !tail.trim().is_empty() {
        return Err("unexpected trailing input".to_string());
    }

    let descr = unquote_node_descr(inner);
    let ty = node_type_for(start, end);
    Ok((id_raw, descr, ty))
}

fn split_node_id(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes();
    for (idx, b) in bytes.iter().enumerate() {
        match b {
            b'(' | b')' | b'[' | b'{' | b'}' => return (&input[..idx], &input[idx..]),
            _ => {}
        }
    }
    (input, "")
}

fn node_delimiter_pair_at_start(input: &str) -> Option<(&'static str, &'static str)> {
    let pairs: &[(&str, &str)] = &[
        ("(-", "-)"),
        ("-)", "(-"),
        ("((", "))"),
        ("))", "(("),
        ("{{", "}}"),
        ("[", "]"),
        (")", "("),
        ("(", ")"),
    ];

    for (start, end) in pairs {
        if input.starts_with(start) {
            return Some((*start, *end));
        }
    }
    None
}

fn extract_delimited<'a>(
    input: &'a str,
    start: &str,
    end: &str,
) -> std::result::Result<(&'a str, &'a str), String> {
    if !input.starts_with(start) {
        return Err("expected delimiter start".to_string());
    }
    let mut in_quote = false;
    let mut in_backtick_quote = false;

    let start_len = start.len();
    let mut it = input[start_len..].char_indices().peekable();
    while let Some((off, ch)) = it.next() {
        let idx = start_len + off;

        if in_backtick_quote {
            if ch == '`' && it.peek().is_some_and(|(_, next)| *next == '"') {
                in_backtick_quote = false;
                it.next();
            }
            continue;
        }

        if in_quote {
            if ch == '"' {
                in_quote = false;
            }
            continue;
        }

        if ch == '"' {
            if it.peek().is_some_and(|(_, next)| *next == '`') {
                in_backtick_quote = true;
                it.next();
                continue;
            }
            in_quote = true;
            continue;
        }

        if input[idx..].starts_with(end) {
            let inner = &input[start_len..idx];
            let tail = &input[idx + end.len()..];
            return Ok((inner, tail));
        }
    }

    Err("unterminated node delimiter".to_string())
}

fn unquote_node_descr(raw: &str) -> String {
    if let Some(inner) = raw.strip_prefix("\"`").and_then(|s| s.strip_suffix("`\"")) {
        return inner.to_string();
    }
    if let Some(inner) = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return inner.to_string();
    }
    raw.to_string()
}

fn node_type_for(start: &str, end: &str) -> i32 {
    match start {
        "[" => NODE_TYPE_RECT,
        "(" => {
            if end == ")" {
                NODE_TYPE_ROUNDED_RECT
            } else {
                NODE_TYPE_CLOUD
            }
        }
        "((" => NODE_TYPE_CIRCLE,
        ")" => NODE_TYPE_CLOUD,
        "))" => NODE_TYPE_BANG,
        "{{" => NODE_TYPE_HEXAGON,
        _ => NODE_TYPE_DEFAULT,
    }
}

fn get_i64(cfg: &MermaidConfig, dotted_path: &str) -> Option<i64> {
    let mut cur = cfg.as_value();
    for segment in dotted_path.split('.') {
        cur = cur.as_object()?.get(segment)?;
    }
    cur.as_i64().or_else(|| cur.as_f64().map(|f| f as i64))
}

fn consume_shape_data(lines: &mut std::str::Lines<'_>, first: &str) -> Result<String> {
    let Some(mut rest) = first.strip_prefix("@{") else {
        return Ok(String::new());
    };

    let mut out = String::new();
    let mut in_quote = false;
    let mut quoted = String::new();

    loop {
        let it = rest.char_indices().peekable();
        for (_idx, ch) in it {
            if in_quote {
                if ch == '"' {
                    out.push_str(&replace_newline_whitespace_with_br(&quoted));
                    quoted.clear();
                    out.push('"');
                    in_quote = false;
                    continue;
                }
                quoted.push(ch);
                continue;
            }

            if ch == '"' {
                out.push('"');
                in_quote = true;
                continue;
            }

            if ch == '}' {
                return Ok(out);
            }

            out.push(ch);
        }

        let Some(next_line) = lines.next() else {
            return Err(Error::DiagramParse {
                diagram_type: "kanban".to_string(),
                message: "unterminated @{ ... } metadata block".to_string(),
            });
        };
        if in_quote {
            quoted.push('\n');
        } else {
            out.push('\n');
        }
        rest = next_line;
    }
}

fn replace_newline_whitespace_with_br(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars().peekable();
    while let Some(ch) = it.next() {
        if ch == '\n' {
            while it.peek().is_some_and(|c| c.is_whitespace()) {
                it.next();
            }
            out.push_str("<br/>");
            continue;
        }
        out.push(ch);
    }
    out
}

fn split_node_and_shape_data(
    lines: &mut std::str::Lines<'_>,
    rest: &str,
) -> Result<(String, Option<String>)> {
    let mut in_quote = false;
    let mut in_backtick_quote = false;
    let mut it = rest.char_indices().peekable();
    while let Some((idx, ch)) = it.next() {
        if in_backtick_quote {
            if ch == '`' && it.peek().is_some_and(|(_, next)| *next == '"') {
                in_backtick_quote = false;
                it.next();
            }
            continue;
        }

        if in_quote {
            if ch == '"' {
                in_quote = false;
            }
            continue;
        }

        if ch == '"' {
            if it.peek().is_some_and(|(_, next)| *next == '`') {
                in_backtick_quote = true;
                it.next();
                continue;
            }
            in_quote = true;
            continue;
        }

        if rest[idx..].starts_with("@{") {
            let node_part = rest[..idx].trim_end().to_string();
            let shape_data = consume_shape_data(lines, &rest[idx..])?;
            return Ok((node_part, Some(shape_data)));
        }
    }

    Ok((rest.trim_end().to_string(), None))
}

fn parse_kanban_db(code: &str, meta: &ParseMetadata) -> Result<KanbanDb> {
    let mut db = KanbanDb::default();
    db.clear();

    let mut lines = code.lines();
    let mut found_header = false;
    let mut header_tail: Option<String> = None;
    for line in lines.by_ref() {
        let t = strip_inline_comment(line);
        let trimmed = t.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("kanban") {
            found_header = true;
            break;
        }
        if starts_with_case_insensitive(trimmed, "kanban")
            && trimmed.len() > "kanban".len()
            && trimmed["kanban".len()..]
                .chars()
                .next()
                .is_some_and(|c| c.is_whitespace())
        {
            found_header = true;
            let after_keyword = &trimmed["kanban".len()..];
            let indent = after_keyword
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            let rest = after_keyword.trim_start();
            if !rest.is_empty() {
                header_tail = Some(format!("{}{}", " ".repeat(indent), rest));
            }
            break;
        }
        break;
    }

    if !found_header {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected kanban header".to_string(),
        });
    }

    if let Some(tail) = &header_tail {
        let tail = strip_inline_comment(tail);
        if !tail.trim().is_empty() {
            let (indent, rest) = split_indent(tail);
            let rest = rest.trim_end();
            if starts_with_case_insensitive(rest, "::icon(") {
                let after = &rest["::icon(".len()..];
                if let Some(end) = after.find(')') {
                    db.decorate_last(None, Some(after[..end].to_string()), &meta.effective_config);
                }
            } else if let Some(after) = rest.strip_prefix(":::") {
                db.decorate_last(Some(after.trim().to_string()), None, &meta.effective_config);
            } else {
                let (node_part, shape_data) = split_node_and_shape_data(&mut lines, rest)?;
                if !node_part.trim().is_empty() {
                    let (id_raw, descr_raw, ty) =
                        parse_node_spec(&node_part).map_err(|message| Error::DiagramParse {
                            diagram_type: meta.diagram_type.clone(),
                            message,
                        })?;
                    db.add_node(
                        indent,
                        &id_raw,
                        &descr_raw,
                        ty,
                        shape_data,
                        &meta.effective_config,
                    )?;
                }
            }
        }
    }

    while let Some(line) = lines.next() {
        let line = strip_inline_comment(line);
        if line.trim().is_empty() {
            continue;
        }

        let (indent, rest) = split_indent(line);
        let rest = rest.trim_end();
        if rest.is_empty() {
            continue;
        }

        if starts_with_case_insensitive(rest, "::icon(") {
            let after = &rest["::icon(".len()..];
            if let Some(end) = after.find(')') {
                db.decorate_last(None, Some(after[..end].to_string()), &meta.effective_config);
            }
            continue;
        }

        if let Some(after) = rest.strip_prefix(":::") {
            db.decorate_last(Some(after.trim().to_string()), None, &meta.effective_config);
            continue;
        }

        let (node_part, shape_data) = split_node_and_shape_data(&mut lines, rest)?;
        if node_part.trim().is_empty() {
            continue;
        }

        let (id_raw, descr_raw, ty) =
            parse_node_spec(&node_part).map_err(|message| Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message,
            })?;

        db.add_node(
            indent,
            &id_raw,
            &descr_raw,
            ty,
            shape_data,
            &meta.effective_config,
        )?;
    }

    Ok(db)
}

pub fn parse_kanban(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let db = parse_kanban_db(code, meta)?;
    let mut out = Map::with_capacity(6);
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert("sections".to_string(), db.sections_json());
    out.insert(
        "nodes".to_string(),
        Value::Array(db.data_nodes_json(&meta.effective_config)),
    );
    out.insert("edges".to_string(), Value::Array(Vec::new()));
    out.insert("other".to_string(), Value::Object(Map::new()));
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );
    Ok(Value::Object(out))
}

pub fn parse_kanban_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<KanbanDiagramRenderModel> {
    let db = parse_kanban_db(code, meta)?;
    Ok(KanbanDiagramRenderModel {
        nodes: db.data_nodes_for_render(&meta.effective_config),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions};
    use futures::executor::block_on;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    fn sections(model: &Value) -> Vec<Value> {
        model["sections"].as_array().cloned().unwrap_or_default()
    }

    fn data_nodes(model: &Value) -> Vec<Value> {
        model["nodes"].as_array().cloned().unwrap_or_default()
    }

    #[test]
    fn knbn_1_simple_root() {
        let model = parse("kanban\n    root");
        let sections = sections(&model);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0]["label"].as_str().unwrap(), "root");
    }

    #[test]
    fn knbn_2_hierarchy_two_children() {
        let model = parse("kanban\n    root\n      child1\n      child2\n");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0]["label"].as_str().unwrap(), "root");
        assert_eq!(children.len(), 2);
        assert_eq!(children[0]["label"].as_str().unwrap(), "child1");
        assert_eq!(children[1]["label"].as_str().unwrap(), "child2");
    }

    #[test]
    fn knbn_3_shape_without_id() {
        let model = parse("kanban\n    (root)");
        let sections = sections(&model);
        assert_eq!(sections[0]["label"].as_str().unwrap(), "root");
    }

    #[test]
    fn knbn_4_does_not_distinguish_deeper_levels() {
        let model = parse("kanban\n    root\n      child1\n        leaf1\n      child2");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections.len(), 1);
        assert_eq!(children.len(), 3);
    }

    #[test]
    fn knbn_5_multiple_sections() {
        let model = parse("kanban\n    section1\n    section2");
        let sections = sections(&model);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0]["label"].as_str().unwrap(), "section1");
        assert_eq!(sections[1]["label"].as_str().unwrap(), "section2");
    }

    #[test]
    fn knbn_6_real_root_in_wrong_place_is_error() {
        let engine = Engine::new();
        let text = "kanban\n          root\n        fakeRoot\n    realRootWrongPlace";
        let err = block_on(engine.parse_diagram(text, ParseOptions::default())).unwrap_err();
        assert!(
            err.to_string()
                .contains("Items without section detected, found section (\"fakeRoot\")")
        );
    }

    #[test]
    fn knbn_7_id_and_label_rect() {
        let model = parse("kanban\n    root[The root]\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(sections[0]["label"].as_str().unwrap(), "The root");
    }

    #[test]
    fn knbn_8_child_id_and_label() {
        let model = parse("kanban\n    root\n      theId(child1)");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections[0]["label"].as_str().unwrap(), "root");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0]["label"].as_str().unwrap(), "child1");
        assert_eq!(children[0]["id"].as_str().unwrap(), "theId");
    }

    #[test]
    fn knbn_9_child_id_and_label_without_indent_on_root() {
        let model = parse("kanban\nroot\n      theId(child1)");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections[0]["label"].as_str().unwrap(), "root");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0]["label"].as_str().unwrap(), "child1");
        assert_eq!(children[0]["id"].as_str().unwrap(), "theId");
    }

    #[test]
    fn knbn_13_set_icon_for_node() {
        let model = parse("kanban\n    root[The root]\n    ::icon(bomb)\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(sections[0]["label"].as_str().unwrap(), "The root");
        assert_eq!(sections[0]["icon"].as_str().unwrap(), "bomb");
    }

    #[test]
    fn knbn_14_set_classes_for_node() {
        let model = parse("kanban\n    root[The root]\n    :::m-4 p-8\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(sections[0]["label"].as_str().unwrap(), "The root");
        assert_eq!(sections[0]["cssClasses"].as_str().unwrap(), "m-4 p-8");
    }

    #[test]
    fn knbn_15_set_classes_and_icon_classes_first() {
        let model = parse("kanban\n    root[The root]\n    :::m-4 p-8\n    ::icon(bomb)\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["cssClasses"].as_str().unwrap(), "m-4 p-8");
        assert_eq!(sections[0]["icon"].as_str().unwrap(), "bomb");
    }

    #[test]
    fn knbn_16_set_classes_and_icon_icon_first() {
        let model = parse("kanban\n    root[The root]\n    ::icon(bomb)\n    :::m-4 p-8\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["cssClasses"].as_str().unwrap(), "m-4 p-8");
        assert_eq!(sections[0]["icon"].as_str().unwrap(), "bomb");
    }

    #[test]
    fn knbn_17_node_syntax_in_description() {
        let model = parse("kanban\n    root[\"String containing []\"]\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(
            sections[0]["label"].as_str().unwrap(),
            "String containing []"
        );
    }

    #[test]
    fn knbn_18_node_syntax_in_child_description() {
        let model = parse(
            "kanban\n    root[\"String containing []\"]\n      child1[\"String containing ()\"]\n",
        );
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(
            sections[0]["label"].as_str().unwrap(),
            "String containing []"
        );
        assert_eq!(children.len(), 1);
        assert_eq!(
            children[0]["label"].as_str().unwrap(),
            "String containing ()"
        );
    }

    #[test]
    fn knbn_19_child_after_class_assignment() {
        let model = parse(
            "kanban\n  root(Root)\n    Child(Child)\n    :::hot\n      a(a)\n      b[New Stuff]",
        );
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(sections[0]["label"].as_str().unwrap(), "Root");
        assert_eq!(children.len(), 3);
        assert_eq!(children[0]["id"].as_str().unwrap(), "Child");
        assert_eq!(children[1]["id"].as_str().unwrap(), "a");
        assert_eq!(children[2]["id"].as_str().unwrap(), "b");
    }

    #[test]
    fn knbn_20_empty_rows() {
        let model =
            parse("kanban\n  root(Root)\n    Child(Child)\n      a(a)\n\n      b[New Stuff]");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(sections[0]["label"].as_str().unwrap(), "Root");
        assert_eq!(children.len(), 3);
        assert_eq!(children[0]["id"].as_str().unwrap(), "Child");
        assert_eq!(children[1]["id"].as_str().unwrap(), "a");
        assert_eq!(children[2]["id"].as_str().unwrap(), "b");
    }

    #[test]
    fn knbn_22_inline_comment_at_end_of_line() {
        let model = parse(
            "kanban\n  root(Root)\n    Child(Child)\n      a(a) %% This is a comment\n      b[New Stuff]",
        );
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(children.len(), 3);
        assert_eq!(children[0]["id"].as_str().unwrap(), "Child");
        assert_eq!(children[1]["id"].as_str().unwrap(), "a");
        assert_eq!(children[2]["id"].as_str().unwrap(), "b");
    }

    #[test]
    fn knbn_23_rows_with_only_spaces_should_not_interfere() {
        let model = parse("kanban\nroot\n A\n \n\n B");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0]["id"].as_str().unwrap(), "A");
        assert_eq!(children[1]["id"].as_str().unwrap(), "B");
    }

    #[test]
    fn knbn_24_rows_above_header() {
        let model = parse("\n \nkanban\nroot\n A\n \n\n B");
        let sections = sections(&model);
        let nodes = data_nodes(&model);
        let section_id = sections[0]["id"].as_str().unwrap();
        let children: Vec<Value> = nodes
            .into_iter()
            .filter(|n| n["parentId"].as_str() == Some(section_id))
            .collect();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0]["id"].as_str().unwrap(), "A");
        assert_eq!(children[1]["id"].as_str().unwrap(), "B");
    }

    #[test]
    fn knbn_30_priority_metadata() {
        let model = parse("kanban\n        root@{ priority: high }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["id"].as_str().unwrap(), "root");
        assert_eq!(sections[0]["priority"].as_str().unwrap(), "high");
    }

    #[test]
    fn knbn_31_assigned_metadata() {
        let model = parse("kanban\n        root@{ assigned: knsv }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["assigned"].as_str().unwrap(), "knsv");
    }

    #[test]
    fn knbn_32_icon_metadata() {
        let model = parse("kanban\n        root@{ icon: star }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["icon"].as_str().unwrap(), "star");
    }

    #[test]
    fn knbn_34_multiline_metadata() {
        let model = parse(
            "kanban\n        root@{\n          icon: star\n          assigned: knsv\n        }\n",
        );
        let sections = sections(&model);
        assert_eq!(sections[0]["icon"].as_str().unwrap(), "star");
        assert_eq!(sections[0]["assigned"].as_str().unwrap(), "knsv");
    }

    #[test]
    fn knbn_35_inline_metadata_multiple_pairs() {
        let model = parse("kanban\n        root@{ icon: star, assigned: knsv }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["icon"].as_str().unwrap(), "star");
        assert_eq!(sections[0]["assigned"].as_str().unwrap(), "knsv");
    }

    #[test]
    fn knbn_36_label_override_metadata() {
        let model = parse("kanban\n        root@{ icon: star, label: 'fix things' }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["label"].as_str().unwrap(), "fix things");
    }

    #[test]
    fn knbn_37_ticket_metadata() {
        let model = parse("kanban\n        root@{ ticket: MC-1234 }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["ticket"].as_str().unwrap(), "MC-1234");
    }

    #[test]
    fn kanban_get_data_sanitizes_labels_again() {
        let model = parse("kanban\n    root[<b>x</b>]");
        let nodes = data_nodes(&model);
        assert_eq!(nodes[0]["label"].as_str().unwrap(), "<b>x</b>");
    }

    #[test]
    fn kanban_shape_data_rewrites_newline_whitespace_in_double_quotes() {
        let model = parse("kanban\n  root@{ label: \"line1\n      line2\" }\n");
        let sections = sections(&model);
        assert_eq!(sections[0]["label"].as_str().unwrap(), "line1<br/>line2");
    }
}
