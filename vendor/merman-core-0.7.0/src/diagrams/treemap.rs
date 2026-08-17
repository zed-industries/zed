use crate::{Error, ParseMetadata, Result};
use serde_json::{Map, Value, json};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TreemapNodeRenderModel {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TreemapNodeRenderModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(
        default,
        rename = "classSelector",
        skip_serializing_if = "Option::is_none"
    )]
    pub class_selector: Option<String>,
    #[serde(
        default,
        rename = "cssCompiledStyles",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_compiled_styles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TreemapDiagramRenderModel {
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub title: Option<String>,
    pub root: TreemapNodeRenderModel,
}

impl TreemapDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemType {
    Section,
    Leaf,
}

#[derive(Debug, Clone)]
struct ClassDefStatement {
    class_name: String,
    style_text: Option<String>,
}

#[derive(Debug, Clone)]
struct ItemRow {
    indent: usize,
    name: String,
    item_type: ItemType,
    value: Option<Value>,
    class_selector: Option<String>,
}

#[derive(Debug, Clone)]
enum TreemapRow {
    Item(ItemRow),
    ClassDef(ClassDefStatement),
}

#[derive(Debug, Clone)]
struct StyleClassDef {
    id: String,
    styles: Vec<String>,
    text_styles: Vec<String>,
}

#[derive(Debug, Clone)]
struct NodeRecord {
    name: String,
    value: Option<Value>,
    class_selector: Option<String>,
    css_compiled_styles: Option<Vec<String>>,
    children: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Arena {
    nodes: Vec<NodeRecord>,
}

impl Arena {
    fn push(&mut self, node: NodeRecord) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }
}

struct TreemapParsedInput {
    title: Option<String>,
    acc_title: Option<String>,
    acc_descr: Option<String>,
    rows: Vec<TreemapRow>,
}

pub fn parse_treemap(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let Some(parsed) = parse_treemap_input(code, meta)? else {
        return Ok(json!({}));
    };

    let class_defs = class_defs_from_rows(&parsed.rows)?;
    let classes = class_defs_to_value_map(&class_defs);
    let flat_items = flat_items_from_rows(&parsed.rows, &class_defs);

    let (arena, roots) = build_hierarchy(&flat_items);
    let mut root_obj = Map::new();
    root_obj.insert("name".to_string(), Value::String(String::new()));
    root_obj.insert(
        "children".to_string(),
        Value::Array(
            roots
                .iter()
                .map(|&idx| node_to_value(&arena, idx))
                .collect(),
        ),
    );
    let root_value = Value::Object(root_obj);

    let mut nodes_preorder: Vec<Value> = Vec::new();
    for &idx in &roots {
        flatten_preorder(&arena, idx, 0, &mut nodes_preorder);
    }

    let mut out = Map::new();
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert(
        "title".to_string(),
        parsed.title.map(Value::String).unwrap_or(Value::Null),
    );
    out.insert(
        "accTitle".to_string(),
        parsed.acc_title.map(Value::String).unwrap_or(Value::Null),
    );
    out.insert(
        "accDescr".to_string(),
        parsed.acc_descr.map(Value::String).unwrap_or(Value::Null),
    );
    out.insert("root".to_string(), root_value);
    out.insert("nodes".to_string(), Value::Array(nodes_preorder));
    out.insert("classes".to_string(), Value::Object(classes));
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );

    Ok(Value::Object(out))
}

pub fn parse_treemap_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<TreemapDiagramRenderModel> {
    let Some(parsed) = parse_treemap_input(code, meta)? else {
        return Ok(TreemapDiagramRenderModel::default());
    };

    treemap_parsed_input_to_render_model(parsed)
}

fn parse_treemap_input(code: &str, meta: &ParseMetadata) -> Result<Option<TreemapParsedInput>> {
    let mut lines = code.lines();

    let header = loop {
        let Some(line) = lines.next() else {
            return Ok(None);
        };
        let t = strip_inline_comment_aware(line).trim();
        if t.is_empty() {
            continue;
        }
        break t.to_string();
    };

    if !is_treemap_header(&header) {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected treemap".to_string(),
        });
    }

    let mut title: Option<String> = None;
    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;
    let mut rows: Vec<TreemapRow> = Vec::new();
    let mut saw_body_statement = false;
    let mut pending_trailing_ws_only_line = false;

    for raw in lines {
        let raw = raw.trim_end_matches('\r');
        if raw.trim_start().starts_with("%%") {
            continue;
        }

        let t = strip_inline_comment_aware(raw);
        if t.is_empty() {
            continue;
        }

        if t.trim().is_empty() {
            if saw_body_statement {
                pending_trailing_ws_only_line = true;
            }
            continue;
        }

        pending_trailing_ws_only_line = false;
        saw_body_statement = true;

        if let Some(v) = parse_title(t) {
            title = Some(v);
            continue;
        }
        if let Some(v) = parse_key_value(t, "accTitle") {
            acc_title = Some(v);
            continue;
        }
        if let Some(v) = parse_acc_descr(t) {
            acc_descr = Some(v);
            continue;
        }

        let (indent, rest) = split_indent(t);
        let rest = rest.trim_end();
        if rest.is_empty() {
            continue;
        }

        if let Some(class_def) = parse_class_def(rest) {
            rows.push(TreemapRow::ClassDef(class_def));
            continue;
        }

        let item = parse_item_row(indent, rest).map_err(|message| Error::DiagramParse {
            diagram_type: "treemap".to_string(),
            message,
        })?;
        rows.push(TreemapRow::Item(item));
    }

    // Mermaid CLI treats trailing whitespace-only lines as a syntax error for treemap.
    if pending_trailing_ws_only_line {
        return Err(Error::DiagramParse {
            diagram_type: "treemap".to_string(),
            message: "unexpected trailing whitespace-only line".to_string(),
        });
    }

    Ok(Some(TreemapParsedInput {
        title,
        acc_title,
        acc_descr,
        rows,
    }))
}

fn treemap_parsed_input_to_render_model(
    parsed: TreemapParsedInput,
) -> Result<TreemapDiagramRenderModel> {
    let class_defs = class_defs_from_rows(&parsed.rows)?;
    let flat_items = flat_items_from_rows(&parsed.rows, &class_defs);
    let (arena, roots) = build_hierarchy(&flat_items);
    Ok(TreemapDiagramRenderModel {
        title: parsed.title,
        acc_title: parsed.acc_title,
        acc_descr: parsed.acc_descr,
        root: TreemapNodeRenderModel {
            name: String::new(),
            children: Some(
                roots
                    .iter()
                    .map(|&idx| node_to_render_model(&arena, idx))
                    .collect(),
            ),
            value: None,
            class_selector: None,
            css_compiled_styles: None,
        },
    })
}

fn class_defs_from_rows(
    rows: &[TreemapRow],
) -> Result<std::collections::HashMap<String, StyleClassDef>> {
    let mut class_defs: std::collections::HashMap<String, StyleClassDef> =
        std::collections::HashMap::new();
    for row in rows {
        let TreemapRow::ClassDef(c) = row else {
            continue;
        };
        if let Some(style) = c.style_text.as_deref() {
            validate_class_def_style(style).map_err(|message| Error::DiagramParse {
                diagram_type: "treemap".to_string(),
                message,
            })?;
        }
        add_class(
            &mut class_defs,
            &c.class_name,
            c.style_text.as_deref().unwrap_or(""),
        );
    }

    Ok(class_defs)
}

fn class_defs_to_value_map(
    class_defs: &std::collections::HashMap<String, StyleClassDef>,
) -> Map<String, Value> {
    let mut classes: Map<String, Value> = Map::new();
    for (k, v) in class_defs {
        classes.insert(
            k.clone(),
            json!({
                "id": v.id,
                "styles": v.styles,
                "textStyles": v.text_styles,
            }),
        );
    }

    classes
}

fn flat_items_from_rows(
    rows: &[TreemapRow],
    class_defs: &std::collections::HashMap<String, StyleClassDef>,
) -> Vec<FlatItem> {
    let mut flat_items: Vec<FlatItem> = Vec::new();
    for row in rows {
        let TreemapRow::Item(item) = row else {
            continue;
        };

        let styles = item
            .class_selector
            .as_deref()
            .map(|cls| get_styles_for_class(class_defs, cls))
            .unwrap_or_default();
        let compiled = if !styles.is_empty() {
            Some(styles.join(";"))
        } else {
            None
        };
        let css_compiled_styles = compiled.and_then(|s| if s.is_empty() { None } else { Some(s) });

        flat_items.push(FlatItem {
            level: item.indent,
            name: item.name.clone(),
            item_type: item.item_type,
            value: item.value.clone(),
            class_selector: item.class_selector.clone(),
            css_compiled_styles,
        });
    }

    flat_items
}

#[derive(Debug, Clone)]
struct FlatItem {
    level: usize,
    name: String,
    item_type: ItemType,
    value: Option<Value>,
    class_selector: Option<String>,
    css_compiled_styles: Option<String>,
}

fn build_hierarchy(items: &[FlatItem]) -> (Arena, Vec<usize>) {
    if items.is_empty() {
        return (Arena { nodes: Vec::new() }, Vec::new());
    }

    let mut arena = Arena { nodes: Vec::new() };
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new(); // (node_idx, item.level)

    for item in items {
        let mut node = NodeRecord {
            name: item.name.clone(),
            value: None,
            class_selector: item.class_selector.clone(),
            css_compiled_styles: item.css_compiled_styles.as_ref().map(|s| vec![s.clone()]),
            children: match item.item_type {
                ItemType::Leaf => None,
                ItemType::Section => Some(Vec::new()),
            },
        };
        if item.item_type == ItemType::Leaf {
            node.value = item.value.clone();
        }

        let idx = arena.push(node);

        while stack.last().is_some_and(|(_, lvl)| *lvl >= item.level) {
            stack.pop();
        }

        if stack.is_empty() {
            roots.push(idx);
        } else {
            let parent_idx = stack[stack.len() - 1].0;
            let parent = &mut arena.nodes[parent_idx];
            if let Some(children) = parent.children.as_mut() {
                children.push(idx);
            } else {
                parent.children = Some(vec![idx]);
            }
        }

        if item.item_type != ItemType::Leaf {
            stack.push((idx, item.level));
        }
    }

    (arena, roots)
}

fn node_to_value(arena: &Arena, idx: usize) -> Value {
    let mut values: Vec<Option<Value>> = vec![None; arena.nodes.len()];
    let mut stack = vec![(idx, false)];

    while let Some((node_idx, visited)) = stack.pop() {
        let Some(node) = arena.nodes.get(node_idx) else {
            continue;
        };

        if visited {
            let mut obj = Map::new();
            obj.insert("name".to_string(), Value::String(node.name.clone()));
            if let Some(v) = &node.value {
                obj.insert("value".to_string(), v.clone());
            }
            if let Some(cls) = &node.class_selector {
                obj.insert("classSelector".to_string(), Value::String(cls.clone()));
            }
            if let Some(css) = &node.css_compiled_styles {
                obj.insert(
                    "cssCompiledStyles".to_string(),
                    Value::Array(css.iter().cloned().map(Value::String).collect()),
                );
            }
            if let Some(children) = &node.children {
                obj.insert(
                    "children".to_string(),
                    Value::Array(
                        children
                            .iter()
                            .filter_map(|&child_idx| {
                                values.get_mut(child_idx).and_then(Option::take)
                            })
                            .collect(),
                    ),
                );
            }
            values[node_idx] = Some(Value::Object(obj));
        } else {
            stack.push((node_idx, true));
            if let Some(children) = &node.children {
                for &child_idx in children.iter().rev() {
                    stack.push((child_idx, false));
                }
            }
        }
    }

    values
        .get_mut(idx)
        .and_then(Option::take)
        .unwrap_or_else(|| json!({ "name": "" }))
}

fn node_to_render_model(arena: &Arena, idx: usize) -> TreemapNodeRenderModel {
    let mut models: Vec<Option<TreemapNodeRenderModel>> = vec![None; arena.nodes.len()];
    let mut stack = vec![(idx, false)];

    while let Some((node_idx, visited)) = stack.pop() {
        let Some(node) = arena.nodes.get(node_idx) else {
            continue;
        };

        if visited {
            let children = node.children.as_ref().map(|children| {
                children
                    .iter()
                    .filter_map(|&child_idx| models.get_mut(child_idx).and_then(Option::take))
                    .collect()
            });
            models[node_idx] = Some(TreemapNodeRenderModel {
                name: node.name.clone(),
                children,
                value: node.value.clone(),
                class_selector: node.class_selector.clone(),
                css_compiled_styles: node.css_compiled_styles.clone(),
            });
        } else {
            stack.push((node_idx, true));
            if let Some(children) = &node.children {
                for &child_idx in children.iter().rev() {
                    stack.push((child_idx, false));
                }
            }
        }
    }

    models
        .get_mut(idx)
        .and_then(Option::take)
        .unwrap_or_default()
}

fn flatten_preorder(arena: &Arena, idx: usize, level: i64, out: &mut Vec<Value>) {
    let mut stack = vec![(idx, level)];
    while let Some((node_idx, node_level)) = stack.pop() {
        let Some(node) = arena.nodes.get(node_idx) else {
            continue;
        };

        let mut obj = Map::new();
        obj.insert("level".to_string(), Value::Number(node_level.into()));
        obj.insert("name".to_string(), Value::String(node.name.clone()));
        if let Some(v) = &node.value {
            obj.insert("value".to_string(), v.clone());
        }
        if let Some(cls) = &node.class_selector {
            obj.insert("classSelector".to_string(), Value::String(cls.clone()));
        }
        if let Some(css) = &node.css_compiled_styles {
            obj.insert(
                "cssCompiledStyles".to_string(),
                Value::Array(css.iter().cloned().map(Value::String).collect()),
            );
        }
        out.push(Value::Object(obj));

        if let Some(children) = &node.children {
            for &child_idx in children.iter().rev() {
                stack.push((child_idx, node_level.saturating_add(1)));
            }
        }
    }
}

fn add_class(
    classes: &mut std::collections::HashMap<String, StyleClassDef>,
    id: &str,
    style: &str,
) {
    let mut style_class = classes.get(id).cloned().unwrap_or_else(|| StyleClassDef {
        id: id.to_string(),
        styles: Vec::new(),
        text_styles: Vec::new(),
    });

    const PLACEHOLDER: &str = "ก์ก์ก์";
    let replaced = style.replace("\\,", PLACEHOLDER);
    let replaced = replaced.replace(',', ";");
    let replaced = replaced.replace(PLACEHOLDER, ",");

    for s in replaced.split(';') {
        if is_label_style_bug_compatible(s) {
            style_class.text_styles.push(s.to_string());
        }
        style_class.styles.push(s.to_string());
    }

    classes.insert(id.to_string(), style_class);
}

fn validate_class_def_style(style: &str) -> std::result::Result<(), String> {
    let style = style.trim().trim_end_matches(';').trim();
    if style.is_empty() {
        return Ok(());
    }

    const PLACEHOLDER: &str = "ก์ก์ก์";
    let replaced = style.replace("\\,", PLACEHOLDER);
    let replaced = replaced.replace(',', ";");
    let replaced = replaced.replace(PLACEHOLDER, ",");

    for raw in replaced.split(';') {
        let s = raw.trim();
        if s.is_empty() {
            continue;
        }
        let Some((k, v)) = s.split_once(':') else {
            return Err(format!("invalid classDef style token `{s}`"));
        };
        if k.trim().is_empty() || v.trim().is_empty() {
            return Err(format!("invalid classDef style token `{s}`"));
        }
    }

    Ok(())
}

fn get_styles_for_class(
    classes: &std::collections::HashMap<String, StyleClassDef>,
    class_selector: &str,
) -> Vec<String> {
    classes
        .get(class_selector)
        .map(|c| c.styles.clone())
        .unwrap_or_default()
}

fn is_label_style_bug_compatible(s: &str) -> bool {
    matches!(
        s.trim(),
        "color"
            | "font-size"
            | "font-family"
            | "font-weight"
            | "font-style"
            | "text-decoration"
            | "text-align"
            | "text-transform"
            | "line-height"
            | "letter-spacing"
            | "word-spacing"
            | "text-shadow"
            | "text-overflow"
            | "white-space"
            | "word-wrap"
            | "word-break"
            | "overflow-wrap"
            | "hyphens"
    )
}

fn strip_inline_comment_aware(line: &str) -> &str {
    let mut in_quote: Option<char> = None;

    let mut it = line.char_indices().peekable();
    while let Some((idx, ch)) = it.next() {
        if let Some(q) = in_quote {
            if ch == q {
                in_quote = None;
            }
            continue;
        }

        if ch == '"' || ch == '\'' {
            in_quote = Some(ch);
            continue;
        }

        if ch == '%' && it.peek().is_some_and(|(_, next)| *next == '%') {
            return &line[..idx];
        }
    }

    line
}

fn is_treemap_header(line: &str) -> bool {
    let t = line.trim_start();
    t == "treemap" || t == "treemap-beta"
}

fn split_indent(line: &str) -> (usize, &str) {
    let mut indent_chars = 0usize;
    let mut byte_idx = line.len();
    for (idx, ch) in line.char_indices() {
        if ch == ' ' || ch == '\t' {
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

fn parse_title(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("title") {
        return None;
    }
    let rest = t.strip_prefix("title")?.trim_start();
    Some(rest.to_string())
}

fn parse_key_value(line: &str, key: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with(key) {
        return None;
    }
    let rest = t.strip_prefix(key)?.trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    Some(rest.to_string())
}

fn parse_acc_descr(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return None;
    }
    let rest = t.strip_prefix("accDescr")?.trim_start();
    if let Some(rest) = rest.strip_prefix(':') {
        return Some(rest.trim_start().to_string());
    }
    if let Some(rest) = rest.strip_prefix('{') {
        let end = rest.find('}')?;
        return Some(rest[..end].to_string());
    }
    None
}

fn parse_class_def(line: &str) -> Option<ClassDefStatement> {
    let t = line.trim_start();
    if !t.starts_with("classDef") {
        return None;
    }
    let mut rest = t.strip_prefix("classDef")?;
    rest = rest.trim_start();
    let (class_name, tail) = parse_id2(rest)?;
    let mut style_text = tail.trim_start();
    if let Some(semi) = style_text.find(';') {
        style_text = &style_text[..semi];
    }
    let style_text = style_text.trim();
    Some(ClassDefStatement {
        class_name,
        style_text: if style_text.is_empty() {
            None
        } else {
            Some(style_text.to_string())
        },
    })
}

fn parse_item_row(indent: usize, line: &str) -> std::result::Result<ItemRow, String> {
    let mut p = Parser::new(line);
    p.skip_ws();
    let name = p
        .parse_string2()
        .ok_or_else(|| "expected quoted string".to_string())?;
    p.skip_ws();

    // Section: "Name" (:::class)?
    if p.try_consume_str(":::") {
        p.skip_ws();
        let (cls, _) = parse_id2(p.rest()).ok_or_else(|| "expected class selector".to_string())?;
        p.pos += cls.len();
        p.skip_ws();
        if !p.eof() {
            return Err("unexpected tokens after section".to_string());
        }
        return Ok(ItemRow {
            indent,
            name,
            item_type: ItemType::Section,
            value: None,
            class_selector: Some(cls),
        });
    }

    // Leaf: "Name" : 10 (:::class)?
    if p.try_consume(':') || p.try_consume(',') {
        p.skip_ws();
        let token = p
            .parse_number2_token()
            .ok_or_else(|| "expected number".to_string())?;
        let value = parse_number2_value(&token).ok_or_else(|| "expected number".to_string())?;
        p.skip_ws();
        let mut class_selector = None;
        if p.try_consume_str(":::") {
            p.skip_ws();
            let (cls, _) =
                parse_id2(p.rest()).ok_or_else(|| "expected class selector".to_string())?;
            p.pos += cls.len();
            class_selector = Some(cls);
            p.skip_ws();
        }
        if !p.eof() {
            return Err("unexpected tokens after leaf".to_string());
        }
        return Ok(ItemRow {
            indent,
            name,
            item_type: ItemType::Leaf,
            value: Some(value),
            class_selector,
        });
    }

    if p.eof() {
        return Ok(ItemRow {
            indent,
            name,
            item_type: ItemType::Section,
            value: None,
            class_selector: None,
        });
    }

    Err("expected ':' or ':::' or end of line".to_string())
}

fn parse_id2(input: &str) -> Option<(String, &str)> {
    let mut chars = input.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    let mut idx = first.len_utf8();
    for ch in chars {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            idx += ch.len_utf8();
        } else {
            break;
        }
    }
    Some((input[..idx].to_string(), &input[idx..]))
}

fn parse_number2_value(token: &str) -> Option<Value> {
    let no_commas: String = token.chars().filter(|c| *c != ',').collect();
    let mut saw_dot = false;
    let mut cut = 0usize;
    for ch in no_commas.chars() {
        if ch.is_ascii_digit() {
            cut += ch.len_utf8();
            continue;
        }
        if ch == '.' && !saw_dot {
            saw_dot = true;
            cut += 1;
            continue;
        }
        break;
    }
    if cut == 0 {
        return None;
    }
    let prefix = &no_commas[..cut];

    if saw_dot {
        let frac = prefix.split_once('.').map(|(_, b)| b).unwrap_or("");
        if frac.is_empty() || frac.chars().all(|c| c == '0') {
            let int_part = prefix.split_once('.').map(|(a, _)| a).unwrap_or(prefix);
            let i: i64 = int_part.parse().ok()?;
            return Some(Value::Number(i.into()));
        }
        let f: f64 = prefix.parse().ok()?;
        let n = serde_json::Number::from_f64(f)?;
        return Some(Value::Number(n));
    }

    let i: i64 = prefix.parse().ok()?;
    Some(Value::Number(i.into()))
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn rest(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.rest().chars().next() {
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
                continue;
            }
            break;
        }
    }

    fn try_consume(&mut self, ch: char) -> bool {
        if self.rest().starts_with(ch) {
            self.pos += ch.len_utf8();
            true
        } else {
            false
        }
    }

    fn try_consume_str(&mut self, s: &str) -> bool {
        if self.rest().starts_with(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    fn parse_string2(&mut self) -> Option<String> {
        let rest = self.rest();
        let quote = rest.chars().next()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        let mut idx = 1usize;
        for ch in rest[1..].chars() {
            idx += ch.len_utf8();
            if ch == quote {
                let inner = &rest[1..idx - 1];
                self.pos += idx;
                return Some(inner.to_string());
            }
        }
        None
    }

    fn parse_number2_token(&mut self) -> Option<String> {
        let mut idx = 0usize;
        for ch in self.rest().chars() {
            if ch.is_ascii_digit() || ch == '_' || ch == '.' || ch == ',' {
                idx += ch.len_utf8();
                continue;
            }
            break;
        }
        if idx == 0 {
            return None;
        }
        let token = &self.rest()[..idx];
        self.pos += idx;
        Some(token.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions, RenderSemanticModel};
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
    fn treemap_accepts_treemap_beta_header() {
        let model = parse("treemap-beta\n\"A\"");
        assert_eq!(model["root"]["children"][0]["name"], json!("A"));
    }

    #[test]
    fn treemap_accepts_treemap_header() {
        let model = parse("treemap\n\"A\"");
        assert_eq!(model["root"]["children"][0]["name"], json!("A"));
    }

    #[test]
    fn treemap_render_model_uses_typed_variant_without_changing_json_parse() {
        let engine = Engine::new();
        let input = r#"treemap
title Treemap Title
accTitle: Treemap accTitle
accDescr: Treemap accDescr
"Root"
  "Leaf": 42
"#;

        let parsed = engine
            .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        assert_eq!(parsed.meta.diagram_type, "treemap");
        match parsed.model {
            RenderSemanticModel::Treemap(model) => {
                assert_eq!(model.title.as_deref(), Some("Treemap Title"));
                assert_eq!(model.acc_title.as_deref(), Some("Treemap accTitle"));
                assert_eq!(model.acc_descr.as_deref(), Some("Treemap accDescr"));
                assert_eq!(model.root.name, "");
                let root_children = model.root.children.as_ref().unwrap();
                assert_eq!(root_children.len(), 1);
                assert_eq!(root_children[0].name, "Root");
                assert_eq!(root_children[0].children.as_ref().unwrap()[0].name, "Leaf");
            }
            other => panic!("treemap render parse should return typed model, got {other:?}"),
        }

        let parsed_json = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        assert_eq!(parsed_json.model["type"], json!("treemap"));
        assert_eq!(parsed_json.model["title"], json!("Treemap Title"));
        assert_eq!(parsed_json.model["accTitle"], json!("Treemap accTitle"));
        assert_eq!(
            parsed_json.model["root"]["children"][0]["name"],
            json!("Root")
        );
        assert_eq!(
            parsed_json.model["root"]["children"][0]["children"][0]["value"],
            json!(42)
        );
        assert!(parsed_json.model.get("config").is_some());
    }

    fn parse_error(text: &str) -> String {
        let engine = Engine::new();
        let err = block_on(engine.parse_diagram(text, ParseOptions::default())).unwrap_err();
        err.to_string()
    }

    fn deep_treemap_chain(depth: usize) -> String {
        let mut input = String::from("treemap\n");
        for level in 0..depth {
            input.push_str(&" ".repeat(level));
            input.push('"');
            input.push_str(&format!("section{level}"));
            input.push_str("\"\n");
        }
        input.push_str(&" ".repeat(depth));
        input.push_str("\"leaf\": 1\n");
        input
    }

    fn count_render_nodes(root: &TreemapNodeRenderModel) -> usize {
        let mut count = 0usize;
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            count += 1;
            if let Some(children) = node.children.as_ref() {
                for child in children.iter().rev() {
                    stack.push(child);
                }
            }
        }
        count
    }

    #[test]
    fn treemap_deep_chain_semantic_and_render_model_use_heap_traversal() {
        const DEPTH: usize = 1200;
        let input = deep_treemap_chain(DEPTH);

        let model = parse(&input);
        let nodes = model["nodes"].as_array().expect("nodes array");
        assert_eq!(nodes.len(), DEPTH + 1);
        assert_eq!(nodes[0]["name"], json!("section0"));
        assert_eq!(
            nodes
                .last()
                .and_then(|node| node.get("name"))
                .and_then(Value::as_str),
            Some("leaf")
        );

        let parsed = Engine::new()
            .parse_diagram_for_render_model_sync(&input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        match parsed.model {
            RenderSemanticModel::Treemap(model) => {
                assert_eq!(count_render_nodes(&model.root), DEPTH + 2);
            }
            other => panic!("treemap render parse should return typed model, got {other:?}"),
        }
    }

    #[test]
    fn treemap_errors_on_trailing_whitespace_only_line() {
        let msg = parse_error("treemap\n\"A\": 1\n    \n");
        assert!(
            msg.contains("unexpected trailing whitespace-only line"),
            "{msg}"
        );
    }

    #[test]
    fn treemap_rejects_header_with_colon() {
        let msg = parse_error("treemap:\n\"A\": 1\n");
        assert!(msg.contains("expected treemap"), "{msg}");
    }

    #[test]
    fn treemap_rejects_header_with_suffix_tokens() {
        let msg = parse_error("treemap utilities\n\"A\": 1\n");
        assert!(msg.contains("expected treemap"), "{msg}");
    }

    #[test]
    fn treemap_allows_whitespace_only_lines_in_the_middle() {
        let model = parse("treemap\n\"A\": 1\n    \n\"B\": 2\n");
        assert_eq!(model["root"]["children"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn treemap_parses_basic_hierarchy_from_docs() {
        let model = parse(
            r#"treemap-beta
"Section 1"
    "Leaf 1.1": 12
    "Section 1.2"
      "Leaf 1.2.1": 12
"Section 2"
    "Leaf 2.1": 20
    "Leaf 2.2": 25
"#,
        );

        assert_eq!(model["root"]["children"].as_array().unwrap().len(), 2);
        assert_eq!(model["root"]["children"][0]["name"], json!("Section 1"));
        assert_eq!(
            model["root"]["children"][0]["children"][0]["name"],
            json!("Leaf 1.1")
        );
        assert_eq!(
            model["root"]["children"][0]["children"][0]["value"],
            json!(12)
        );
        assert_eq!(model["root"]["children"][1]["name"], json!("Section 2"));
        assert_eq!(
            model["root"]["children"][1]["children"][1]["value"],
            json!(25)
        );
    }

    #[test]
    fn treemap_classdef_applies_compiled_styles() {
        let model = parse(
            r#"treemap-beta
"Main":::important
  "A": 20

classDef important fill:#f96,stroke:#333,stroke-width:2px;
"#,
        );
        assert_eq!(
            model["classes"]["important"]["styles"][0],
            json!("fill:#f96")
        );
        assert_eq!(
            model["root"]["children"][0]["cssCompiledStyles"][0],
            json!("fill:#f96;stroke:#333;stroke-width:2px")
        );
    }

    #[test]
    fn treemap_classdef_rejects_bare_label_style_tokens_like_mermaid_parser() {
        let msg = parse_error(
            r#"treemap
classDef c fill:#ff0000, stroke:rgb(1\,2\,3), color;
"Root":::c
  "Leaf": 1000.00:::c
"#,
        );
        assert!(
            msg.contains("invalid classDef style token `color`"),
            "{msg}"
        );
    }

    #[test]
    fn treemap_build_hierarchy_matches_upstream_utils_test() {
        let items = vec![
            FlatItem {
                level: 0,
                name: "Root".to_string(),
                item_type: ItemType::Section,
                value: None,
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 4,
                name: "Branch 1".to_string(),
                item_type: ItemType::Section,
                value: None,
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 8,
                name: "Leaf 1.1".to_string(),
                item_type: ItemType::Leaf,
                value: Some(json!(10)),
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 8,
                name: "Leaf 1.2".to_string(),
                item_type: ItemType::Leaf,
                value: Some(json!(15)),
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 4,
                name: "Branch 2".to_string(),
                item_type: ItemType::Section,
                value: None,
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 8,
                name: "Leaf 2.1".to_string(),
                item_type: ItemType::Leaf,
                value: Some(json!(20)),
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 8,
                name: "Leaf 2.2".to_string(),
                item_type: ItemType::Leaf,
                value: Some(json!(25)),
                class_selector: None,
                css_compiled_styles: None,
            },
            FlatItem {
                level: 8,
                name: "Leaf 2.3".to_string(),
                item_type: ItemType::Leaf,
                value: Some(json!(30)),
                class_selector: None,
                css_compiled_styles: None,
            },
        ];

        let (arena, roots) = build_hierarchy(&items);
        let root_value = roots
            .iter()
            .map(|&idx| node_to_value(&arena, idx))
            .collect::<Vec<_>>();
        assert_eq!(
            root_value,
            vec![json!({
                "name": "Root",
                "children": [
                    {
                        "name": "Branch 1",
                        "children": [
                            { "name": "Leaf 1.1", "value": 10 },
                            { "name": "Leaf 1.2", "value": 15 },
                        ]
                    },
                    {
                        "name": "Branch 2",
                        "children": [
                            { "name": "Leaf 2.1", "value": 20 },
                            { "name": "Leaf 2.2", "value": 25 },
                            { "name": "Leaf 2.3", "value": 30 },
                        ]
                    }
                ]
            })]
        );
    }
}
