use crate::sanitize::sanitize_text;
use crate::{Error, MermaidConfig, ParseMetadata, Result};
use serde_json::{Map, Value, json};
use std::collections::{HashMap, hash_map::Entry};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BlockDiagramRenderModel {
    #[serde(default, rename = "blocksFlat")]
    pub blocks_flat: Vec<BlockNodeRenderModel>,
    #[serde(default)]
    pub edges: Vec<BlockEdgeRenderModel>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BlockNodeRenderModel {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default, rename = "type")]
    pub block_type: String,
    #[serde(default)]
    pub children: Vec<BlockNodeRenderModel>,
    #[serde(default)]
    pub columns: Option<i64>,
    #[serde(default, rename = "widthInColumns")]
    pub width_in_columns: Option<i64>,
    #[serde(default)]
    pub width: Option<i64>,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default)]
    pub directions: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BlockEdgeRenderModel {
    pub id: String,
    pub start: String,
    pub end: String,
    #[serde(default, rename = "arrowTypeEnd")]
    pub arrow_type_end: Option<String>,
    #[serde(default, rename = "arrowTypeStart")]
    pub arrow_type_start: Option<String>,
    #[serde(default)]
    pub label: String,
}

#[derive(Debug, Clone, Default)]
struct Block {
    id: String,
    block_type: String,
    label: Option<String>,
    children: Vec<Block>,

    start: Option<String>,
    end: Option<String>,
    arrow_type_end: Option<String>,
    arrow_type_start: Option<String>,

    width: Option<i64>,
    columns: Option<i64>,
    width_in_columns: Option<i64>,
    directions: Option<Vec<String>>,

    classes: Vec<String>,
    styles: Option<Vec<String>>,

    css: Option<String>,
    style_class: Option<String>,
    styles_str: Option<String>,
}

impl Block {
    fn new(id: String) -> Self {
        Self {
            id,
            block_type: "na".to_string(),
            ..Default::default()
        }
    }
}

fn clone_block_shallow(block: &Block) -> Block {
    Block {
        id: block.id.clone(),
        block_type: block.block_type.clone(),
        label: block.label.clone(),
        children: Vec::new(),
        start: block.start.clone(),
        end: block.end.clone(),
        arrow_type_end: block.arrow_type_end.clone(),
        arrow_type_start: block.arrow_type_start.clone(),
        width: block.width,
        columns: block.columns,
        width_in_columns: block.width_in_columns,
        directions: block.directions.clone(),
        classes: block.classes.clone(),
        styles: block.styles.clone(),
        css: block.css.clone(),
        style_class: block.style_class.clone(),
        styles_str: block.styles_str.clone(),
    }
}

fn clone_block_tree_nonrecursive(block: &Block) -> Block {
    let mut completed: HashMap<*const Block, Block> = HashMap::new();
    let mut stack = vec![(block, false)];

    while let Some((block, visited)) = stack.pop() {
        if visited {
            let children = block
                .children
                .iter()
                .filter_map(|child| completed.remove(&(child as *const Block)))
                .collect();
            let mut cloned = clone_block_shallow(block);
            cloned.children = children;
            completed.insert(block as *const Block, cloned);
        } else {
            stack.push((block, true));
            for child in block.children.iter().rev() {
                stack.push((child, false));
            }
        }
    }

    completed
        .remove(&(block as *const Block))
        .unwrap_or_else(|| clone_block_shallow(block))
}

#[derive(Debug, Clone, Default)]
struct ClassDef {
    id: String,
    styles: Vec<String>,
    text_styles: Vec<String>,
}

#[derive(Debug, Default)]
struct BlockDb {
    root_id: String,
    block_database: HashMap<String, Block>,
    block_database_order: Vec<String>,
    blocks: Vec<Block>,
    edges: Vec<Block>,
    edge_count: HashMap<String, i64>,
    classes: HashMap<String, ClassDef>,
    warnings: Vec<String>,
}

impl BlockDb {
    fn clear(&mut self) {
        self.root_id = "root".to_string();
        self.block_database.clear();
        self.block_database_order.clear();
        self.blocks.clear();
        self.edges.clear();
        self.edge_count.clear();
        self.classes.clear();
        self.warnings.clear();

        let root = Block {
            id: self.root_id.clone(),
            block_type: "composite".to_string(),
            children: Vec::new(),
            columns: Some(-1),
            label: Some("".to_string()),
            ..Default::default()
        };
        self.insert_block(self.root_id.clone(), root);
    }

    fn insert_block(&mut self, id: String, block: Block) {
        let existed = self.block_database.contains_key(&id);
        self.block_database.insert(id.clone(), block);
        if !existed {
            self.block_database_order.push(id);
        }
    }

    fn ensure_block_exists(&mut self, id: &str) -> &mut Block {
        match self.block_database.entry(id.to_string()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                self.block_database_order.push(id.to_string());
                entry.insert(Block::new(id.to_string()))
            }
        }
    }

    fn add_style_class(&mut self, id: &str, style_attributes: &str) {
        let entry = self
            .classes
            .entry(id.to_string())
            .or_insert_with(|| ClassDef {
                id: id.to_string(),
                styles: Vec::new(),
                text_styles: Vec::new(),
            });

        for raw in style_attributes.split(',') {
            let fixed = raw.split(';').next().unwrap_or("").trim().to_string();
            if fixed.is_empty() {
                continue;
            }

            if raw.contains("color") {
                let new_style1 = fixed.replace("fill", "bgFill");
                let new_style2 = new_style1.replace("color", "fill");
                entry.text_styles.push(new_style2);
            }
            entry.styles.push(fixed);
        }
    }

    fn add_style_to_node(&mut self, id: &str, styles: &str) {
        let parts: Vec<String> = styles
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if let Some(block) = self.block_database.get_mut(id) {
            block.styles = Some(parts);
            return;
        }

        let mut placeholder = Block::new(id.to_string());
        placeholder.styles = Some(parts);
        self.insert_block(id.to_string(), placeholder);
    }

    fn set_css_class(&mut self, item_ids: &str, css_class_name: &str) {
        for raw_id in item_ids.split(',') {
            let id = raw_id.trim();
            if id.is_empty() {
                continue;
            }

            let entry = self.ensure_block_exists(id);
            entry.classes.push(css_class_name.to_string());
        }
    }

    fn set_hierarchy(&mut self, blocks: Vec<Block>, config: &MermaidConfig) -> Result<()> {
        let root_id = self.root_id.clone();
        self.populate_block_database(blocks, &root_id, config)?;
        self.blocks = self
            .block_database
            .get(&self.root_id)
            .map(|root| {
                root.children
                    .iter()
                    .map(clone_block_tree_nonrecursive)
                    .collect()
            })
            .unwrap_or_default();
        Ok(())
    }

    fn populate_block_database(
        &mut self,
        blocks: Vec<Block>,
        parent_id: &str,
        config: &MermaidConfig,
    ) -> Result<()> {
        let mut stack = vec![PopulateFrame::new(parent_id.to_string(), blocks)];

        while !stack.is_empty() {
            let next = {
                let Some(frame) = stack.last_mut() else {
                    break;
                };
                frame
                    .blocks
                    .next()
                    .map(|block| (block, frame.parent_id.clone(), frame.col))
            };

            let Some((mut block, parent_id, col)) = next else {
                let Some(frame) = stack.pop() else {
                    break;
                };
                let child_blocks: Vec<Block> = frame
                    .child_ids
                    .iter()
                    .filter_map(|id| self.block_database.get(id))
                    .map(clone_block_tree_nonrecursive)
                    .collect();
                if let Some(parent) = self.block_database.get_mut(&frame.parent_id) {
                    parent.children = child_blocks;
                }
                continue;
            };

            if col > 0
                && block.block_type != "column-setting"
                && block.width_in_columns.is_some_and(|w| w > col)
            {
                self.warnings.push(format!(
                    "Block {} width {} exceeds configured column width {}",
                    block.id,
                    block.width_in_columns.unwrap_or(1),
                    col
                ));
            }

            if let Some(label) = &block.label {
                block.label = Some(sanitize_text(label, config));
            }

            match block.block_type.as_str() {
                "classDef" => {
                    let css = block.css.clone().unwrap_or_default();
                    self.add_style_class(&block.id, &css);
                    continue;
                }
                "applyClass" => {
                    let style_class = block.style_class.clone().unwrap_or_default();
                    self.set_css_class(&block.id, &style_class);
                    continue;
                }
                "applyStyles" => {
                    if let Some(styles) = block.styles_str.clone() {
                        self.add_style_to_node(&block.id, &styles);
                    }
                    continue;
                }
                "column-setting" => {
                    if let Some(parent) = self.block_database.get_mut(&parent_id) {
                        parent.columns = block.columns;
                    }
                    continue;
                }
                "edge" => {
                    let base_id = block.id.clone();
                    let count = self.edge_count.get(&base_id).copied().unwrap_or(0) + 1;
                    self.edge_count.insert(base_id.clone(), count);
                    block.id = format!("{count}-{base_id}");
                    self.edges.push(block);
                    continue;
                }
                _ => {}
            }

            if block.label.is_none() {
                if block.block_type == "composite" {
                    block.label = Some("".to_string());
                } else {
                    block.label = Some(block.id.clone());
                }
            }

            let parsed_children = std::mem::take(&mut block.children);
            let block_id = block.id.clone();

            let existed = self.block_database.contains_key(&block.id);
            if !existed {
                self.insert_block(block.id.clone(), clone_block_shallow(&block));
            } else {
                let mut existing = self
                    .block_database
                    .get(&block.id)
                    .map(clone_block_tree_nonrecursive)
                    .unwrap_or_else(|| Block::new(block.id.clone()));
                // Mermaid's blockDB only merges a small subset of fields when a block id is
                // encountered multiple times. In particular, later occurrences do *not* override
                // arrow directions (see upstream cypress BL6), so keep the first-seen properties
                // and only patch in "obviously relevant" updates.
                if block.block_type != "na" {
                    existing.block_type = block.block_type.clone();
                }
                if let Some(lbl) = &block.label {
                    if lbl != &block.id {
                        existing.label = Some(lbl.clone());
                    }
                }
                self.insert_block(block.id.clone(), existing);
            }

            if block.block_type == "space" {
                let w = block.width.unwrap_or(1).max(0);
                for j in 0..w {
                    let id = format!("{}-{}", block.id, j);
                    let mut new_block = clone_block_shallow(&block);
                    new_block.id = id.clone();
                    self.insert_block(id.clone(), new_block);
                    if let Some(frame) = stack.last_mut() {
                        frame.child_ids.push(id);
                    }
                }
                if !parsed_children.is_empty() {
                    stack.push(PopulateFrame::new(block_id, parsed_children));
                }
                continue;
            }

            if !existed {
                if let Some(frame) = stack.last_mut() {
                    frame.child_ids.push(block.id.clone());
                }
            }

            if !parsed_children.is_empty() {
                stack.push(PopulateFrame::new(block_id, parsed_children));
            }
        }

        Ok(())
    }

    fn blocks_flat(&self) -> Vec<&Block> {
        self.block_database_order
            .iter()
            .filter_map(|id| self.block_database.get(id))
            .collect()
    }
}

struct PopulateFrame {
    parent_id: String,
    blocks: std::vec::IntoIter<Block>,
    col: i64,
    child_ids: Vec<String>,
}

impl PopulateFrame {
    fn new(parent_id: String, blocks: Vec<Block>) -> Self {
        let col = blocks
            .iter()
            .find(|b| b.block_type == "column-setting")
            .and_then(|b| b.columns)
            .unwrap_or(-1);
        Self {
            parent_id,
            blocks: blocks.into_iter(),
            col,
            child_ids: Vec::new(),
        }
    }
}

fn block_to_value_shallow(b: &Block, children: Vec<Value>) -> Value {
    let mut obj = Map::new();
    obj.insert("id".to_string(), json!(b.id));
    obj.insert("type".to_string(), json!(b.block_type));
    if let Some(label) = &b.label {
        obj.insert("label".to_string(), json!(label));
    }
    obj.insert("children".to_string(), Value::Array(children));

    if let Some(v) = &b.start {
        obj.insert("start".to_string(), json!(v));
    }
    if let Some(v) = &b.end {
        obj.insert("end".to_string(), json!(v));
    }
    if let Some(v) = &b.arrow_type_end {
        obj.insert("arrowTypeEnd".to_string(), json!(v));
    }
    if let Some(v) = &b.arrow_type_start {
        obj.insert("arrowTypeStart".to_string(), json!(v));
    }
    if let Some(v) = b.width {
        obj.insert("width".to_string(), json!(v));
    }
    if let Some(v) = b.columns {
        obj.insert("columns".to_string(), json!(v));
    }
    if let Some(v) = b.width_in_columns {
        obj.insert("widthInColumns".to_string(), json!(v));
    }
    if let Some(v) = &b.directions {
        obj.insert("directions".to_string(), json!(v));
    }
    if !b.classes.is_empty() {
        obj.insert("classes".to_string(), json!(b.classes));
    }
    if let Some(v) = &b.styles {
        obj.insert("styles".to_string(), json!(v));
    }
    if let Some(v) = &b.css {
        obj.insert("css".to_string(), json!(v));
    }
    if let Some(v) = &b.style_class {
        obj.insert("styleClass".to_string(), json!(v));
    }
    if let Some(v) = &b.styles_str {
        obj.insert("stylesStr".to_string(), json!(v));
    }

    Value::Object(obj)
}

fn block_to_value(b: &Block) -> Value {
    let mut stack: Vec<(&Block, bool)> = vec![(b, false)];
    let mut completed: HashMap<*const Block, Value> = HashMap::new();

    while let Some((block, visited)) = stack.pop() {
        if visited {
            let children = block
                .children
                .iter()
                .filter_map(|child| completed.remove(&(child as *const Block)))
                .collect();
            completed.insert(
                block as *const Block,
                block_to_value_shallow(block, children),
            );
        } else {
            stack.push((block, true));
            for child in block.children.iter().rev() {
                stack.push((child, false));
            }
        }
    }

    completed
        .remove(&(b as *const Block))
        .unwrap_or_else(|| block_to_value_shallow(b, Vec::new()))
}

fn class_def_map_to_value(classes: &HashMap<String, ClassDef>) -> Value {
    let mut obj = Map::new();
    for (k, v) in classes {
        obj.insert(
            k.clone(),
            json!({
                "id": v.id,
                "styles": v.styles,
                "textStyles": v.text_styles,
            }),
        );
    }
    Value::Object(obj)
}

fn block_to_render_node_shallow(
    b: &Block,
    children: Vec<BlockNodeRenderModel>,
) -> BlockNodeRenderModel {
    BlockNodeRenderModel {
        id: b.id.clone(),
        label: b.label.clone().unwrap_or_default(),
        block_type: b.block_type.clone(),
        children,
        columns: b.columns,
        width_in_columns: b.width_in_columns,
        width: b.width,
        classes: b.classes.clone(),
        styles: b.styles.clone().unwrap_or_default(),
        directions: b.directions.clone().unwrap_or_default(),
    }
}

fn block_to_render_node(b: &Block) -> BlockNodeRenderModel {
    let mut stack: Vec<(&Block, bool)> = vec![(b, false)];
    let mut completed: HashMap<*const Block, BlockNodeRenderModel> = HashMap::new();

    while let Some((block, visited)) = stack.pop() {
        if visited {
            let children = block
                .children
                .iter()
                .filter_map(|child| completed.remove(&(child as *const Block)))
                .collect();
            completed.insert(
                block as *const Block,
                block_to_render_node_shallow(block, children),
            );
        } else {
            stack.push((block, true));
            for child in block.children.iter().rev() {
                stack.push((child, false));
            }
        }
    }

    completed
        .remove(&(b as *const Block))
        .unwrap_or_else(|| block_to_render_node_shallow(b, Vec::new()))
}

fn block_to_render_edge(b: &Block) -> BlockEdgeRenderModel {
    BlockEdgeRenderModel {
        id: b.id.clone(),
        start: b.start.clone().unwrap_or_default(),
        end: b.end.clone().unwrap_or_default(),
        arrow_type_end: b.arrow_type_end.clone(),
        arrow_type_start: b.arrow_type_start.clone(),
        label: b.label.clone().unwrap_or_default(),
    }
}

fn block_db_to_render_model(db: &BlockDb) -> BlockDiagramRenderModel {
    BlockDiagramRenderModel {
        blocks_flat: db
            .blocks_flat()
            .into_iter()
            .map(block_to_render_node)
            .collect(),
        edges: db.edges.iter().map(block_to_render_edge).collect(),
    }
}

fn parse_block_db(code: &str, meta: &ParseMetadata) -> Result<BlockDb> {
    let mut parser = Parser::new(code);
    parser.parse_header()?;
    let blocks = parser.parse_document(false)?;

    let mut db = BlockDb::default();
    db.clear();
    db.set_hierarchy(blocks, &meta.effective_config)?;
    Ok(db)
}

pub fn parse_block_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<BlockDiagramRenderModel> {
    let db = parse_block_db(code, meta)?;
    Ok(block_db_to_render_model(&db))
}

fn type_str_to_type(type_str: &str) -> String {
    match type_str {
        "[]" => "square",
        "()" => "round",
        "(())" => "circle",
        ">]" => "rect_left_inv_arrow",
        "{}" => "diamond",
        "{{}}" => "hexagon",
        "([])" => "stadium",
        "[[]]" => "subroutine",
        "[()]" => "cylinder",
        "((()))" => "doublecircle",
        "[//]" => "lean_right",
        "[\\\\]" => "lean_left",
        "[/\\]" => "trapezoid",
        "[\\/]" => "inv_trapezoid",
        "<[]>" => "block_arrow",
        _ => "na",
    }
    .to_string()
}

fn edge_str_to_edge_data(type_str: &str) -> String {
    let trimmed = type_str.trim_matches(|c: char| c.is_whitespace() || c == '-');
    match trimmed {
        "x" => "arrow_cross",
        "o" => "arrow_circle",
        ">" => "arrow_point",
        _ => "",
    }
    .to_string()
}

fn is_valid_link_token(raw: &str) -> bool {
    let s = raw.trim();
    if s.is_empty() {
        return false;
    }

    if s.chars().all(|c| c == '~') {
        return s.len() >= 3;
    }

    let (prefix, rest) = match s.chars().next() {
        Some('x') | Some('o') | Some('<') => (&s[..1], &s[1..]),
        _ => ("", s),
    };
    let _ = prefix;

    is_valid_solid_link(rest) || is_valid_thick_link(rest) || is_valid_dotted_link(rest)
}

fn is_valid_solid_link(rest: &str) -> bool {
    if rest.is_empty() || !rest.starts_with('-') {
        return false;
    }

    if rest.chars().all(|c| c == '-') {
        return rest.len() >= 3;
    }

    let (body, tail) = rest.split_at(rest.len() - 1);
    let last = tail.chars().next().unwrap_or('\0');
    if !matches!(last, '-' | 'x' | 'o' | '>') {
        return false;
    }

    let dash_count = body.chars().filter(|c| *c == '-').count();
    dash_count >= 2 && body.chars().all(|c| c == '-')
}

fn is_valid_thick_link(rest: &str) -> bool {
    if rest.is_empty() || !rest.starts_with('=') {
        return false;
    }

    if rest.chars().all(|c| c == '=') {
        return rest.len() >= 3;
    }

    let (body, tail) = rest.split_at(rest.len() - 1);
    let last = tail.chars().next().unwrap_or('\0');
    if !matches!(last, '=' | 'x' | 'o' | '>') {
        return false;
    }

    let eq_count = body.chars().filter(|c| *c == '=').count();
    eq_count >= 2 && body.chars().all(|c| c == '=')
}

fn is_valid_dotted_link(rest: &str) -> bool {
    if rest.is_empty() {
        return false;
    }

    let mut chars = rest.chars().peekable();
    if matches!(chars.peek(), Some('-')) {
        chars.next();
    }

    let mut dot_count = 0usize;
    while matches!(chars.peek(), Some('.')) {
        dot_count += 1;
        chars.next();
    }
    if dot_count == 0 {
        return false;
    }

    if chars.next() != Some('-') {
        return false;
    }

    let tail: String = chars.collect();
    if tail.is_empty() {
        return true;
    }
    if tail.len() == 1 {
        return matches!(tail.chars().next(), Some('x' | 'o' | '>'));
    }
    false
}

struct NodeDelims {
    start: &'static str,
    ends: &'static [&'static str],
}

fn node_delims_at_start(input: &str) -> Option<NodeDelims> {
    let delims: &[NodeDelims] = &[
        NodeDelims {
            start: "([",
            ends: &["])"],
        },
        NodeDelims {
            start: "[[",
            ends: &["]]"],
        },
        NodeDelims {
            start: "[(",
            ends: &[")]"],
        },
        NodeDelims {
            start: "(((",
            ends: &[")))"],
        },
        NodeDelims {
            start: "((",
            ends: &["))", ")"],
        },
        NodeDelims {
            start: "{{",
            ends: &["}}"],
        },
        NodeDelims {
            start: "[/",
            // Upstream Mermaid's block lexer ends NODE state on `]` as a fallback, even when the
            // node started with a more specific delimiter like `[/` (see cypress BL21).
            // Accepting `]` here matches that behavior (and yields an unknown typeStr like `[/]`,
            // which upstream maps to the default `na` type).
            ends: &["/]", "\\]", "]"],
        },
        NodeDelims {
            start: "[\\",
            // Same as `[/`: accept `]` as a fallback end delimiter for parity with upstream.
            ends: &["\\]", "/]", "]"],
        },
        NodeDelims {
            start: "[",
            // Upstream ends NODE state on `\]` and `/]` before falling back to `]`.
            ends: &["\\]", "/]", "]"],
        },
        NodeDelims {
            start: "(",
            ends: &[")"],
        },
        NodeDelims {
            start: "{",
            ends: &["}"],
        },
        NodeDelims {
            start: ">",
            ends: &["]"],
        },
    ];

    for d in delims {
        if input.starts_with(d.start) {
            return Some(NodeDelims {
                start: d.start,
                ends: d.ends,
            });
        }
    }

    None
}

enum DocumentFrameKind {
    Root,
    IdBlock(Block),
    AnonymousBlock,
}

struct DocumentFrame {
    kind: DocumentFrameKind,
    children: Vec<Block>,
}

impl DocumentFrame {
    fn root() -> Self {
        Self {
            kind: DocumentFrameKind::Root,
            children: Vec::new(),
        }
    }

    fn id_block(header: Block) -> Self {
        Self {
            kind: DocumentFrameKind::IdBlock(header),
            children: Vec::new(),
        }
    }

    fn anonymous_block() -> Self {
        Self {
            kind: DocumentFrameKind::AnonymousBlock,
            children: Vec::new(),
        }
    }

    fn into_block(self, parser: &mut Parser<'_>) -> Block {
        match self.kind {
            DocumentFrameKind::Root => {
                let mut b = Block::new(parser.generate_id());
                b.block_type = "composite".to_string();
                b.label = Some("".to_string());
                b.children = self.children;
                b
            }
            DocumentFrameKind::IdBlock(mut header) => {
                header.block_type = "composite".to_string();
                header.children = self.children;
                header
            }
            DocumentFrameKind::AnonymousBlock => {
                let mut b = Block::new(parser.generate_id());
                b.block_type = "composite".to_string();
                b.label = Some("".to_string());
                b.children = self.children;
                b
            }
        }
    }
}

fn block_document_frame_error() -> Error {
    Error::DiagramParse {
        diagram_type: "block".to_string(),
        message: "internal block document frame stack is empty".to_string(),
    }
}

fn current_document_frame_mut(frames: &mut [DocumentFrame]) -> Result<&mut DocumentFrame> {
    frames.last_mut().ok_or_else(block_document_frame_error)
}

fn push_document_child(frames: &mut [DocumentFrame], block: Block) -> Result<()> {
    current_document_frame_mut(frames)?.children.push(block);
    Ok(())
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    gen_counter: i64,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            gen_counter: 0,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn generate_id(&mut self) -> String {
        self.gen_counter += 1;
        let rand = uuid::Uuid::new_v4().simple().to_string();
        let rand = &rand[..12.min(rand.len())];
        format!("id-{rand}-{}", self.gen_counter)
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            while self.peek_char().is_some_and(|c| c.is_whitespace()) {
                self.bump();
            }

            if self.starts_with("%%") {
                while let Some(c) = self.bump() {
                    if c == '\n' {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    fn peek_keyword(&mut self, kw: &str) -> bool {
        self.skip_ws_and_comments();
        if !self.starts_with(kw) {
            return false;
        }
        if kw.ends_with(':') {
            return true;
        }
        let after = &self.input[self.pos + kw.len()..];
        after
            .chars()
            .next()
            .is_none_or(|c| c.is_whitespace() || c == ':')
    }

    fn consume_keyword(&mut self, kw: &str) -> bool {
        if !self.peek_keyword(kw) {
            return false;
        }
        self.pos += kw.len();
        true
    }

    fn consume_keyword_same_line(&mut self, kw: &str) -> bool {
        // Like `consume_keyword`, but does not skip newlines/comments. This is used for
        // statement-local infix tokens (e.g. `id1 space id2`), where treating the next line's
        // `space` statement as an infix separator would be incorrect.
        while self.peek_char().is_some_and(|c| c == ' ' || c == '\t') {
            self.bump();
        }
        if self.starts_with("%%") {
            return false;
        }
        if !self.starts_with(kw) {
            return false;
        }
        if kw.ends_with(':') {
            self.pos += kw.len();
            return true;
        }
        let after = &self.input[self.pos + kw.len()..];
        if after
            .chars()
            .next()
            .is_none_or(|c| c.is_whitespace() || c == ':')
        {
            self.pos += kw.len();
            return true;
        }
        false
    }

    fn consume_exact(&mut self, s: &str) -> bool {
        self.skip_ws_and_comments();
        if !self.starts_with(s) {
            return false;
        }
        self.pos += s.len();
        true
    }

    fn parse_header(&mut self) -> Result<()> {
        self.skip_ws_and_comments();
        if self.consume_keyword("block-beta") {
            return Ok(());
        }
        if self.consume_keyword("block") {
            return Ok(());
        }
        Err(Error::DiagramParse {
            diagram_type: "block".to_string(),
            message: "expected block header".to_string(),
        })
    }

    fn parse_document(&mut self, stop_on_end: bool) -> Result<Vec<Block>> {
        let mut frames = vec![DocumentFrame::root()];

        loop {
            self.skip_ws_and_comments();
            if self.is_eof() {
                break;
            }

            let current_is_root = frames.len() == 1;
            if ((!current_is_root) || stop_on_end) && self.peek_keyword("end") {
                self.consume_keyword("end");
                if current_is_root {
                    break;
                }
                self.finish_document_frame(&mut frames)?;
                continue;
            }

            if self.peek_keyword("block:") {
                self.consume_keyword("block:");
                let mut stm = self.parse_node_statement()?;
                let header = stm
                    .drain(..)
                    .find(|b| b.block_type != "edge")
                    .unwrap_or_else(|| Block::new(self.generate_id()));
                frames.push(DocumentFrame::id_block(header));
                continue;
            }

            if self.peek_keyword("block-beta") || self.peek_keyword("block") {
                if !(self.consume_keyword("block-beta") || self.consume_keyword("block")) {
                    return Err(Error::DiagramParse {
                        diagram_type: "block".to_string(),
                        message: "expected block".to_string(),
                    });
                }
                frames.push(DocumentFrame::anonymous_block());
                continue;
            }

            if self.peek_keyword("columns") {
                let block = self.parse_columns_statement()?;
                push_document_child(&mut frames, block)?;
                continue;
            }
            if self.peek_keyword("space") {
                let block = self.parse_space_statement()?;
                push_document_child(&mut frames, block)?;
                continue;
            }
            if self.peek_keyword("classDef") {
                let block = self.parse_classdef_statement()?;
                push_document_child(&mut frames, block)?;
                continue;
            }
            if self.peek_keyword("class") {
                let block = self.parse_apply_class_statement()?;
                push_document_child(&mut frames, block)?;
                continue;
            }
            if self.peek_keyword("style") {
                let block = self.parse_style_statement()?;
                push_document_child(&mut frames, block)?;
                continue;
            }

            let mut blocks = self.parse_node_statement()?;
            current_document_frame_mut(&mut frames)?
                .children
                .append(&mut blocks);
        }

        while frames.len() > 1 {
            self.finish_document_frame(&mut frames)?;
        }

        let Some(frame) = frames.pop() else {
            return Err(block_document_frame_error());
        };
        Ok(frame.children)
    }

    fn finish_document_frame(&mut self, frames: &mut Vec<DocumentFrame>) -> Result<()> {
        let Some(frame) = frames.pop() else {
            return Err(block_document_frame_error());
        };
        let block = frame.into_block(self);
        current_document_frame_mut(frames)?.children.push(block);
        Ok(())
    }

    fn parse_columns_statement(&mut self) -> Result<Block> {
        self.skip_ws_and_comments();
        if !self.consume_keyword("columns") {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected columns".to_string(),
            });
        }
        self.skip_ws_and_comments();
        let value = if self.consume_keyword("auto") {
            -1
        } else {
            self.parse_int()?
        };

        // Mermaid does not require a unique id for column-setting statements (they are not part of
        // the rendered block list); avoid consuming a generated id so generated composite ids
        // match upstream counters.
        let mut b = Block::new("columns".to_string());
        b.block_type = "column-setting".to_string();
        b.columns = Some(value);
        Ok(b)
    }

    fn parse_space_statement(&mut self) -> Result<Block> {
        self.skip_ws_and_comments();
        if !self.consume_keyword("space") {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected space".to_string(),
            });
        }
        let mut width = 1;
        self.skip_ws_and_comments();
        if self.consume_exact(":") {
            width = self.parse_int()?;
        }
        let mut b = Block::new(self.generate_id());
        b.block_type = "space".to_string();
        b.label = Some("".to_string());
        b.width = Some(width);
        Ok(b)
    }

    fn parse_classdef_statement(&mut self) -> Result<Block> {
        self.skip_ws_and_comments();
        if !self.consume_keyword("classDef") {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected classDef".to_string(),
            });
        }
        self.skip_ws_and_comments();
        let id = self.parse_identifier_like()?;
        let css = self.take_rest_of_line_trimmed();
        let mut b = Block::new(id);
        b.block_type = "classDef".to_string();
        b.css = Some(css);
        Ok(b)
    }

    fn parse_apply_class_statement(&mut self) -> Result<Block> {
        self.skip_ws_and_comments();
        if !self.consume_keyword("class") {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected class".to_string(),
            });
        }
        self.skip_ws_and_comments();
        let ids = self.parse_identifier_like()?;
        let style_class = self.take_rest_of_line_trimmed();
        let mut b = Block::new(ids);
        b.block_type = "applyClass".to_string();
        b.style_class = Some(style_class);
        Ok(b)
    }

    fn parse_style_statement(&mut self) -> Result<Block> {
        self.skip_ws_and_comments();
        if !self.consume_keyword("style") {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected style".to_string(),
            });
        }
        self.skip_ws_and_comments();
        let ids = self.parse_identifier_like()?;
        let styles_str = self.take_rest_of_line_trimmed();
        let mut b = Block::new(ids);
        b.block_type = "applyStyles".to_string();
        b.styles_str = Some(styles_str);
        Ok(b)
    }

    fn take_rest_of_line_trimmed(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c == '\n' || c == '\r' {
                break;
            }
            self.bump();
        }
        self.input[start..self.pos].trim().to_string()
    }

    fn parse_node_statement(&mut self) -> Result<Vec<Block>> {
        let mut left = self.parse_node()?;
        if self.consume_keyword_same_line("space") {
            let mut width = 1;
            while self.peek_char().is_some_and(|c| c == ' ' || c == '\t') {
                self.bump();
            }
            if self.peek_char() == Some(':') {
                self.bump();
                while self.peek_char().is_some_and(|c| c == ' ' || c == '\t') {
                    self.bump();
                }
                let start = self.pos;
                while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                    self.bump();
                }
                if self.pos == start {
                    return Err(Error::DiagramParse {
                        diagram_type: "block".to_string(),
                        message: "expected integer width after space:".to_string(),
                    });
                }
                width = self.input[start..self.pos].parse::<i64>().unwrap_or(1);
            }
            let mut space = Block::new(self.generate_id());
            space.block_type = "space".to_string();
            space.label = Some("".to_string());
            space.width = Some(width);

            left.width_in_columns.get_or_insert(1);
            while self.peek_char().is_some_and(|c| c == ' ' || c == '\t') {
                self.bump();
            }
            if self.starts_with("%%") || matches!(self.peek_char(), None | Some('\n' | '\r')) {
                return Ok(vec![left, space]);
            }

            let mut right = self.parse_node()?;
            right.width_in_columns.get_or_insert(1);
            return Ok(vec![left, space, right]);
        }

        self.skip_ws_and_comments();
        if let Some((label, edge_marker)) = self.parse_link()? {
            let mut right = self.parse_node()?;
            let arrow_type_end = edge_str_to_edge_data(&edge_marker);
            let edge_id = format!("{}-{}", left.id, right.id);
            let edge = Block {
                id: edge_id,
                block_type: "edge".to_string(),
                label: Some(label),
                children: Vec::new(),
                start: Some(left.id.clone()),
                end: Some(right.id.clone()),
                arrow_type_end: Some(arrow_type_end),
                arrow_type_start: Some("arrow_open".to_string()),
                directions: right.directions.clone(),
                ..Default::default()
            };

            left.width_in_columns.get_or_insert(1);
            right.width_in_columns.get_or_insert(1);
            return Ok(vec![left, edge, right]);
        }

        self.skip_ws_and_comments();
        if self.consume_exact(":") {
            let w = self.parse_int()?;
            left.width_in_columns = Some(w);
        } else {
            left.width_in_columns.get_or_insert(1);
        }

        Ok(vec![left])
    }

    fn parse_link(&mut self) -> Result<Option<(String, String)>> {
        self.skip_ws_and_comments();
        if self.is_eof() {
            return Ok(None);
        }

        let snapshot = self.pos;
        if self.try_read_link_start_marker().is_some() {
            self.skip_ws_and_comments();
            if self.peek_char() == Some('"') {
                let label = self.parse_string_literal()?;
                self.skip_ws_and_comments();
                if let Some(edge_marker) = self.try_read_link_full_marker() {
                    return Ok(Some((label, edge_marker)));
                }
                self.pos = snapshot;
                return Ok(None);
            }
            self.pos = snapshot;
        }

        if let Some(edge_marker) = self.try_read_link_full_marker() {
            return Ok(Some(("".to_string(), edge_marker)));
        }

        Ok(None)
    }

    fn try_read_link_start_marker(&mut self) -> Option<String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        if self
            .peek_char()
            .is_some_and(|c| c == 'x' || c == 'o' || c == '<')
        {
            self.bump()?;
        }
        if self.starts_with("--") || self.starts_with("==") || self.starts_with("-.") {
            self.bump()?;
            self.bump()?;
            return Some(self.input[start..self.pos].to_string());
        }
        self.pos = start;
        None
    }

    fn try_read_link_full_marker(&mut self) -> Option<String> {
        self.skip_ws_and_comments();
        let start = self.pos;

        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                break;
            }
            // Mermaid block edge markers can be directly adjacent to node ids
            // (e.g. `a-->b`). Stop once we hit a non-marker character so we don't consume the
            // right-hand node into the marker token.
            if !matches!(c, '-' | '=' | '.' | 'x' | 'o' | '<' | '>' | '~') {
                break;
            }
            self.bump();
        }

        if self.pos == start {
            return None;
        }

        let token = &self.input[start..self.pos];
        if !is_valid_link_token(token) {
            self.pos = start;
            return None;
        }
        Some(token.to_string())
    }

    fn parse_node(&mut self) -> Result<Block> {
        self.skip_ws_and_comments();
        let id = self.parse_node_id()?;
        let mut b = Block::new(id);
        b.label = None;
        b.block_type = "na".to_string();

        self.skip_ws_and_comments();

        if self.starts_with("<[") {
            self.pos += 2;
            self.skip_ws_and_comments();
            let label = self.parse_string_literal()?;
            self.skip_ws_and_comments();
            if !self.consume_exact("]>") {
                return Err(Error::DiagramParse {
                    diagram_type: "block".to_string(),
                    message: "expected ]> in block arrow".to_string(),
                });
            }
            self.skip_ws_and_comments();
            if !self.consume_exact("(") {
                return Err(Error::DiagramParse {
                    diagram_type: "block".to_string(),
                    message: "expected '(' in block arrow".to_string(),
                });
            }
            let dirs = self.parse_direction_list()?;
            if !self.consume_exact(")") {
                return Err(Error::DiagramParse {
                    diagram_type: "block".to_string(),
                    message: "expected ')' in block arrow".to_string(),
                });
            }

            b.label = Some(label);
            b.block_type = "block_arrow".to_string();
            b.directions = Some(dirs);
            b.width_in_columns = Some(1);
            return Ok(b);
        }

        if let Some(delims) = node_delims_at_start(&self.input[self.pos..]) {
            let start_delim = delims.start;
            self.pos += start_delim.len();
            self.skip_ws_and_comments();
            let label = self.parse_string_literal_or_md()?;
            self.skip_ws_and_comments();
            let mut matched_end: Option<&'static str> = None;
            for end in delims.ends {
                if self.consume_exact(end) {
                    matched_end = Some(end);
                    break;
                }
            }
            let end_delim = match matched_end {
                Some(e) => e,
                None => {
                    return Err(Error::DiagramParse {
                        diagram_type: "block".to_string(),
                        message: "unterminated node delimiter".to_string(),
                    });
                }
            };
            if end_delim.is_empty() {
                return Err(Error::DiagramParse {
                    diagram_type: "block".to_string(),
                    message: "unterminated node delimiter".to_string(),
                });
            }

            let type_str = format!("{start_delim}{end_delim}");
            b.label = Some(label);
            b.block_type = type_str_to_type(&type_str);
            b.width_in_columns = Some(1);
            return Ok(b);
        }

        Ok(b)
    }

    fn parse_direction_list(&mut self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        loop {
            self.skip_ws_and_comments();
            let w = self.parse_direction()?;
            out.push(w);
            self.skip_ws_and_comments();
            if self.consume_exact(",") {
                continue;
            }
            break;
        }
        Ok(out)
    }

    fn parse_direction(&mut self) -> Result<String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == ',' || c == ')' {
                break;
            }
            self.bump();
        }
        if self.pos == start {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected direction".to_string(),
            });
        }
        let dir = self.input[start..self.pos].trim().to_string();
        match dir.as_str() {
            "right" | "left" | "x" | "y" | "up" | "down" => Ok(dir),
            _ => Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: format!("invalid direction: {dir}"),
            }),
        }
    }

    fn parse_node_id(&mut self) -> Result<String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_whitespace()
                || matches!(
                    c,
                    '(' | '[' | '\n' | '-' | ')' | '{' | '}' | '<' | '>' | ':'
                )
            {
                break;
            }
            self.bump();
        }
        if self.pos == start {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected node id".to_string(),
            });
        }
        Ok(self.input[start..self.pos].to_string())
    }

    fn parse_identifier_like(&mut self) -> Result<String> {
        self.skip_ws_and_comments();
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == '\n' || c == '\r' {
                break;
            }
            self.bump();
        }
        if self.pos == start {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected identifier".to_string(),
            });
        }
        Ok(self.input[start..self.pos].trim().to_string())
    }

    fn parse_int(&mut self) -> Result<i64> {
        self.skip_ws_and_comments();
        let start = self.pos;
        while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            self.bump();
        }
        if self.pos == start {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected integer".to_string(),
            });
        }
        self.input[start..self.pos]
            .parse::<i64>()
            .map_err(|e| Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: e.to_string(),
            })
    }

    fn parse_string_literal_or_md(&mut self) -> Result<String> {
        self.skip_ws_and_comments();
        if self.starts_with("\"`") {
            self.pos += 2;
            let start = self.pos;
            while self.pos < self.input.len() && !self.input[self.pos..].starts_with("`\"") {
                self.bump();
            }
            if self.pos >= self.input.len() {
                return Err(Error::DiagramParse {
                    diagram_type: "block".to_string(),
                    message: "unterminated markdown string".to_string(),
                });
            }
            let inner = self.input[start..self.pos].to_string();
            self.pos += 2;
            return Ok(inner);
        }
        self.parse_string_literal()
    }

    fn parse_string_literal(&mut self) -> Result<String> {
        self.skip_ws_and_comments();
        if self.peek_char() != Some('"') {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "expected string literal".to_string(),
            });
        }
        self.bump();
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c == '"' {
                break;
            }
            self.bump();
        }
        if self.peek_char() != Some('"') {
            return Err(Error::DiagramParse {
                diagram_type: "block".to_string(),
                message: "unterminated string literal".to_string(),
            });
        }
        let inner = self.input[start..self.pos].to_string();
        self.bump();
        Ok(inner)
    }
}

pub fn parse_block(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let db = parse_block_db(code, meta)?;

    let blocks = db.blocks.iter().map(block_to_value).collect::<Vec<_>>();
    let edges = db.edges.iter().map(block_to_value).collect::<Vec<_>>();
    let blocks_flat = db
        .blocks_flat()
        .into_iter()
        .map(block_to_value)
        .collect::<Vec<_>>();
    let classes = class_def_map_to_value(&db.classes);
    let warnings = db.warnings.into_iter().map(Value::String).collect();

    let mut out = Map::new();
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert("blocks".to_string(), Value::Array(blocks));
    out.insert("edges".to_string(), Value::Array(edges));
    out.insert("blocksFlat".to_string(), Value::Array(blocks_flat));
    out.insert("classes".to_string(), classes);
    out.insert("warnings".to_string(), Value::Array(warnings));
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );
    Ok(Value::Object(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions, RenderSemanticModel};
    use futures::executor::block_on;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    fn deep_block_chain(depth: usize) -> String {
        let mut input = String::from("block\n");
        for level in 0..depth {
            input.push_str(&format!("block:n{level}[\"n{level}\"]\n"));
        }
        input.push_str("leaf[\"leaf\"]\n");
        for _ in 0..depth {
            input.push_str("end\n");
        }
        input
    }

    fn blocks(model: &Value) -> Vec<Value> {
        model["blocks"].as_array().cloned().unwrap_or_default()
    }

    fn edges(model: &Value) -> Vec<Value> {
        model["edges"].as_array().cloned().unwrap_or_default()
    }

    fn columns_for_id(model: &Value, id: &str) -> Option<i64> {
        for b in model["blocksFlat"].as_array()? {
            if b["id"].as_str()? == id {
                return b.get("columns").and_then(|v| v.as_i64());
            }
        }
        None
    }

    #[test]
    fn block_diagram_with_node() {
        let model = parse("block-beta\n  id\n");
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0]["id"].as_str().unwrap(), "id");
        assert_eq!(blocks[0]["label"].as_str().unwrap(), "id");
    }

    #[test]
    fn node_with_square_shape_and_label() {
        let model = parse("block\n  id[\"A label\"]\n");
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0]["id"].as_str().unwrap(), "id");
        assert_eq!(blocks[0]["label"].as_str().unwrap(), "A label");
        assert_eq!(blocks[0]["type"].as_str().unwrap(), "square");
    }

    #[test]
    fn multiple_nodes() {
        let model = parse("block\n  id1\n  id2\n  id3\n");
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0]["id"].as_str().unwrap(), "id1");
        assert_eq!(blocks[1]["id"].as_str().unwrap(), "id2");
        assert_eq!(blocks[2]["id"].as_str().unwrap(), "id3");
    }

    #[test]
    fn nodes_with_edge_basic() {
        let model = parse("block\n  id1[\"first\"]  -->   id2[\"second\"]\n");
        let blocks = blocks(&model);
        let edges = edges(&model);
        assert_eq!(blocks.len(), 2);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0]["start"].as_str().unwrap(), "id1");
        assert_eq!(edges[0]["end"].as_str().unwrap(), "id2");
        assert_eq!(edges[0]["arrowTypeEnd"].as_str().unwrap(), "arrow_point");
    }

    #[test]
    fn block_render_model_uses_typed_variant_without_changing_json_parse() {
        let engine = Engine::new();
        let input = "block-beta\n  A[\"first\"] --> B[\"second\"]\n";

        let parsed = engine
            .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();

        assert_eq!(parsed.meta.diagram_type, "block");
        match parsed.model {
            RenderSemanticModel::Block(model) => {
                let a = model
                    .blocks_flat
                    .iter()
                    .find(|block| block.id == "A")
                    .unwrap();
                assert_eq!(a.label, "first");
                assert_eq!(model.edges.len(), 1);
                assert_eq!(model.edges[0].start, "A");
                assert_eq!(model.edges[0].end, "B");
                assert_eq!(
                    model.edges[0].arrow_type_end.as_deref(),
                    Some("arrow_point")
                );
            }
            other => panic!("block render parse should return typed model, got {other:?}"),
        }

        let parsed_json = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        assert_eq!(parsed_json.model["type"], json!("block"));
        assert_eq!(parsed_json.model["blocks"][0]["id"], json!("A"));
        assert_eq!(parsed_json.model["edges"][0]["start"], json!("A"));
        assert!(parsed_json.model.get("config").is_some());
    }

    #[test]
    fn block_deep_chain_semantic_and_render_model_use_heap_traversal() {
        const DEPTH: usize = 1200;
        let input = deep_block_chain(DEPTH);

        let model = parse(&input);
        let blocks_flat = model["blocksFlat"].as_array().expect("blocksFlat array");
        assert_eq!(blocks_flat.len(), DEPTH + 2);
        assert_eq!(blocks_flat[0]["id"].as_str(), Some("root"));
        assert_eq!(
            blocks_flat
                .last()
                .and_then(|block| block.get("id"))
                .and_then(Value::as_str),
            Some("leaf")
        );

        let parsed = Engine::new()
            .parse_diagram_for_render_model_sync(&input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        match parsed.model {
            RenderSemanticModel::Block(model) => {
                assert_eq!(model.blocks_flat.len(), DEPTH + 2);
                assert_eq!(model.blocks_flat[0].id, "root");
                assert_eq!(
                    model.blocks_flat.last().map(|block| block.id.as_str()),
                    Some("leaf")
                );
            }
            other => panic!("block render parse should return typed model, got {other:?}"),
        }
    }

    #[test]
    fn nodes_with_edge_label() {
        let model = parse("block\n  id1[\"first\"]  -- \"a label\" -->   id2[\"second\"]\n");
        let edges = edges(&model);
        assert_eq!(edges[0]["label"].as_str().unwrap(), "a label");
    }

    #[test]
    fn diagram_with_column_statements() {
        let model = parse("block\n  columns 2\n  block1[\"Block 1\"]\n");
        assert_eq!(columns_for_id(&model, "root").unwrap(), 2);
        assert_eq!(blocks(&model).len(), 1);
    }

    #[test]
    fn diagram_without_column_statements() {
        let model = parse("block\n  block1[\"Block 1\"]\n");
        assert_eq!(columns_for_id(&model, "root").unwrap(), -1);
        assert_eq!(blocks(&model).len(), 1);
    }

    #[test]
    fn diagram_with_auto_column_statements() {
        let model = parse("block\n  columns auto\n  block1[\"Block 1\"]\n");
        assert_eq!(columns_for_id(&model, "root").unwrap(), -1);
        assert_eq!(blocks(&model).len(), 1);
    }

    #[test]
    fn blocks_next_to_each_other() {
        let model = parse("block\n  columns 2\n  block1[\"Block 1\"]\n  block2[\"Block 2\"]\n");
        assert_eq!(columns_for_id(&model, "root").unwrap(), 2);
        assert_eq!(blocks(&model).len(), 2);
    }

    #[test]
    fn blocks_on_top_of_each_other() {
        let model = parse("block\n  columns 1\n  block1[\"Block 1\"]\n  block2[\"Block 2\"]\n");
        assert_eq!(columns_for_id(&model, "root").unwrap(), 1);
        assert_eq!(blocks(&model).len(), 2);
    }

    #[test]
    fn compound_blocks() {
        let model =
            parse("block\n  block\n    aBlock[\"ABlock\"]\n    bBlock[\"BBlock\"]\n  end\n");
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0]["type"].as_str().unwrap(), "composite");
        assert_eq!(blocks[0]["children"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn compound_blocks_of_compound_blocks() {
        let model = parse(
            "block\n  block\n    aBlock[\"ABlock\"]\n    block\n      bBlock[\"BBlock\"]\n    end\n  end\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        let first = &blocks[0];
        assert_eq!(first["children"].as_array().unwrap().len(), 2);
        let a_block = &first["children"][0];
        assert_eq!(a_block["label"].as_str().unwrap(), "ABlock");
        let second_composite = &first["children"][1];
        assert_eq!(second_composite["type"].as_str().unwrap(), "composite");
        assert_eq!(second_composite["children"].as_array().unwrap().len(), 1);
        let b_block = &second_composite["children"][0];
        assert_eq!(b_block["label"].as_str().unwrap(), "BBlock");
    }

    #[test]
    fn compound_blocks_with_title() {
        let model = parse(
            "block\n  block:compoundBlock[\"Compound block\"]\n    columns 1\n    block2[\"Block 2\"]\n  end\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        let compound = &blocks[0];
        assert_eq!(compound["id"].as_str().unwrap(), "compoundBlock");
        assert_eq!(compound["label"].as_str().unwrap(), "Compound block");
        assert_eq!(compound["type"].as_str().unwrap(), "composite");
        assert_eq!(compound["children"].as_array().unwrap().len(), 1);
        assert_eq!(compound["children"][0]["id"].as_str().unwrap(), "block2");
    }

    #[test]
    fn blocks_mixed_with_compound_blocks() {
        let model = parse(
            "block\n  columns 1\n  block1[\"Block 1\"]\n\n  block\n    columns 2\n    block2[\"Block 2\"]\n    block3[\"Block 3\"]\n  end\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 2);
        let compound = &blocks[1];
        assert_eq!(compound["type"].as_str().unwrap(), "composite");
        assert_eq!(compound["children"].as_array().unwrap().len(), 2);
        assert_eq!(compound["children"][0]["id"].as_str().unwrap(), "block2");
    }

    #[test]
    fn arrow_blocks() {
        let model = parse(
            "block\n  columns 3\n  block1[\"Block 1\"]\n  blockArrow<[\"&nbsp;&nbsp;&nbsp;\"]>(right)\n  block2[\"Block 2\"]\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[1]["type"].as_str().unwrap(), "block_arrow");
        assert!(
            blocks[1]["directions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("right"))
        );
    }

    #[test]
    fn arrow_blocks_with_multiple_points() {
        let model = parse(
            "block\n  columns 1\n  A\n  blockArrow<[\"&nbsp;&nbsp;&nbsp;\"]>(up, down)\n  block\n    columns 3\n    B\n    C\n    D\n  end\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 3);
        let arrow = &blocks[1];
        assert_eq!(arrow["type"].as_str().unwrap(), "block_arrow");
        let dirs: Vec<&str> = arrow["directions"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect();
        assert!(dirs.contains(&"up"));
        assert!(dirs.contains(&"down"));
        assert!(!dirs.contains(&"right"));
    }

    #[test]
    fn blocks_with_different_widths() {
        let model = parse("block\n  columns 3\n  one[\"One Slot\"]\n  two[\"Two slots\"]:2\n");
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[1]["widthInColumns"].as_i64().unwrap(), 2);
    }

    #[test]
    fn empty_blocks_space() {
        let model = parse("block\n  columns 3\n  space\n  middle[\"In the middle\"]\n  space\n");
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0]["type"].as_str().unwrap(), "space");
        assert_eq!(blocks[2]["type"].as_str().unwrap(), "space");
        assert_eq!(blocks[1]["label"].as_str().unwrap(), "In the middle");
    }

    #[test]
    fn classdef_and_apply_class() {
        let model = parse(
            "block\n  classDef black color:#ffffff, fill:#000000;\n  mc[\"Memcache\"]\n  class mc black\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        assert!(
            blocks[0]["classes"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("black"))
        );
        let classes = model["classes"].as_object().unwrap();
        let black = classes.get("black").unwrap();
        assert_eq!(black["id"].as_str().unwrap(), "black");
        assert_eq!(black["styles"][0].as_str().unwrap(), "color:#ffffff");
    }

    #[test]
    fn style_statement_applied() {
        let model = parse(
            "block\n  columns 1\n  B[\"A wide one in the middle\"]\n  style B fill:#f9F,stroke:#333,stroke-width:4px\n",
        );
        let blocks = blocks(&model);
        assert_eq!(blocks.len(), 1);
        let styles: Vec<&str> = blocks[0]["styles"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect();
        assert!(styles.contains(&"fill:#f9F"));
    }

    #[test]
    fn warns_when_block_width_exceeds_column_width() {
        let model = parse("block-beta\n  columns 1\n  A:1\n  B:2\n  C:3\n");
        let warnings: Vec<&str> = model["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect();
        assert!(warnings.contains(&"Block B width 2 exceeds configured column width 1"));
    }

    #[test]
    fn prototype_property_ids_do_not_crash() {
        for prop in ["__proto__", "constructor"] {
            let text = format!("block\n{prop}\n");
            let _ = parse(&text);
            let text =
                format!("block\nA\nclassDef {prop} color:#ffffff,fill:#000000;\nclass A {prop}\n");
            let _ = parse(&text);
            let text =
                format!("block\nA; classDef {prop} color:#ffffff,fill:#000000; class A {prop}");
            let _ = parse(&text);
        }
    }
}
