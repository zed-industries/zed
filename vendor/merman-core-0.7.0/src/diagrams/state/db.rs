use crate::sanitize::{sanitize_text, sanitize_text_or_array};
use crate::{Error, MermaidConfig, ParseMetadata, Result};
use indexmap::IndexMap;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use super::{
    Note, StateDiagramRenderEdge, StateDiagramRenderLink, StateDiagramRenderLinks,
    StateDiagramRenderModel, StateDiagramRenderNode, StateDiagramRenderNote,
    StateDiagramRenderState, StateDiagramRenderStyleClass, StateStmt, Stmt,
};

#[derive(Debug, Clone, Default)]
struct StyleClass {
    id: String,
    styles: Vec<String>,
    text_styles: Vec<String>,
}

#[derive(Debug, Clone)]
struct RelationEdge {
    id1: String,
    id2: String,
    relation_title: Option<String>,
}

#[derive(Debug, Clone)]
struct StateRecord {
    id: String,
    ty: String,
    descriptions: Vec<String>,
    note: Option<Note>,
    classes: Vec<String>,
    styles: Vec<String>,
    text_styles: Vec<String>,
    start: Option<bool>,
}

impl StateRecord {
    fn to_json(&self, doc: Option<Value>) -> Value {
        let mut obj = Map::new();
        obj.insert("id".to_string(), Value::String(self.id.clone()));
        obj.insert("type".to_string(), Value::String(self.ty.clone()));
        obj.insert(
            "descriptions".to_string(),
            string_array_value(&self.descriptions),
        );
        obj.insert("doc".to_string(), doc.unwrap_or(Value::Null));
        obj.insert(
            "note".to_string(),
            self.note.as_ref().map(note_to_json).unwrap_or(Value::Null),
        );
        obj.insert("classes".to_string(), string_array_value(&self.classes));
        obj.insert("styles".to_string(), string_array_value(&self.styles));
        obj.insert(
            "textStyles".to_string(),
            string_array_value(&self.text_styles),
        );
        obj.insert("start".to_string(), option_bool_value(self.start));
        Value::Object(obj)
    }
}

#[derive(Debug, Default)]
pub(super) struct StateDb {
    root_doc: Vec<Stmt>,
    states: HashMap<String, StateRecord>,
    state_order: Vec<String>,
    relations: Vec<RelationEdge>,
    style_classes: IndexMap<String, StyleClass>,
    direction: Option<String>,
    acc_title: Option<String>,
    acc_descr: Option<String>,
    generated_id_cnt: usize,
    links: HashMap<String, Vec<Link>>,
}

impl StateDb {
    pub(super) fn new() -> Self {
        Self::default()
    }

    fn generate_id(&mut self) -> String {
        self.generated_id_cnt += 1;
        let cnt = self.generated_id_cnt as u64;

        // Mermaid `@11.12.2` uses `Math.random().toString(36).substr(2, 12)` plus a monotonically
        // increasing counter. We keep the same `id-<base36>-<n>` shape, but generate the middle
        // segment deterministically to keep snapshots stable.
        let mut x = cnt ^ 0x9e37_79b9_7f4a_7c15u64;
        x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9u64);
        x ^= x >> 32;
        x = x.wrapping_mul(0x94d0_49bb_1331_11ebu64);
        x ^= x >> 32;

        fn to_base36(mut v: u64) -> String {
            const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
            if v == 0 {
                return "0".to_string();
            }
            let mut buf = [0u8; 32];
            let mut i = buf.len();
            while v > 0 {
                let rem = (v % 36) as usize;
                v /= 36;
                i -= 1;
                buf[i] = DIGITS[rem];
            }
            String::from_utf8_lossy(&buf[i..]).to_string()
        }

        let mut mid = to_base36(x);
        if mid.len() < 12 {
            let pad = "0".repeat(12 - mid.len());
            mid = format!("{pad}{mid}");
        } else if mid.len() > 12 {
            mid = mid[mid.len() - 12..].to_string();
        }

        format!("id-{mid}-{}", self.generated_id_cnt)
    }

    pub(super) fn set_root_doc(&mut self, mut doc: Vec<Stmt>) {
        self.translate_doc("root", &mut doc);
        self.extract(&doc);
        self.root_doc = doc;
    }

    fn translate_state_ref(&self, parent_id: &str, s: &mut StateStmt, first: bool) {
        if s.id.trim() == "[*]" {
            s.id = format!("{}_{}", parent_id, if first { "start" } else { "end" });
            s.start = Some(first);
        } else {
            s.id = s.id.trim().to_string();
        }
    }

    fn translate_state_concurrency_split(&mut self, doc: &mut Vec<Stmt>) {
        let old = std::mem::take(doc);
        let mut out: Vec<Stmt> = Vec::new();
        let mut current: Vec<Stmt> = Vec::new();
        let mut saw_divider = false;

        for stmt in old {
            match &stmt {
                Stmt::State(s) if s.ty == "divider" => {
                    saw_divider = true;
                    let mut divider = s.clone();
                    divider.doc = Some(std::mem::take(&mut current));
                    out.push(Stmt::State(divider));
                }
                _ => current.push(stmt),
            }
        }

        if saw_divider && !current.is_empty() {
            let mut divider = StateStmt::new_typed(self.generate_id(), "divider");
            divider.doc = Some(std::mem::take(&mut current));
            out.push(Stmt::State(divider));
        }

        if saw_divider {
            *doc = out;
        } else {
            *doc = current;
        }
    }

    fn translate_doc(&mut self, parent_id: &str, doc: &mut [Stmt]) {
        struct TranslateFrame<'a> {
            parent_id: String,
            iter: std::slice::IterMut<'a, Stmt>,
        }

        let mut stack = vec![TranslateFrame {
            parent_id: parent_id.to_string(),
            iter: doc.iter_mut(),
        }];

        while let Some(frame) = stack.last_mut() {
            let Some(stmt) = frame.iter.next() else {
                stack.pop();
                continue;
            };
            let parent_id = frame.parent_id.clone();

            match stmt {
                Stmt::Relation(relation) => {
                    self.translate_state_ref(&parent_id, &mut relation.state1, true);
                    self.translate_state_ref(&parent_id, &mut relation.state2, false);
                }
                Stmt::State(s) => {
                    self.translate_state_ref(&parent_id, s, true);
                    let child_parent_id = s.id.clone();
                    if let Some(inner) = s.doc.as_mut() {
                        self.translate_state_concurrency_split(inner);
                        stack.push(TranslateFrame {
                            parent_id: child_parent_id,
                            iter: inner.iter_mut(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    fn extract(&mut self, root_doc: &[Stmt]) {
        self.states.clear();
        self.state_order.clear();
        self.relations.clear();
        self.style_classes.clear();
        self.direction = None;
        self.acc_title = None;
        self.acc_descr = None;
        self.generated_id_cnt = 0;
        self.links.clear();

        for stmt in root_doc {
            match stmt {
                Stmt::State(s) => self.add_state(s),
                Stmt::Relation(relation) => self.add_relation(
                    &relation.state1,
                    &relation.state2,
                    relation.description.as_deref(),
                ),
                Stmt::ClassDef { id, classes } => self.add_style_class(id, classes),
                Stmt::ApplyClass { ids, class_name } => self.set_css_class(ids, class_name),
                Stmt::Style { ids, styles } => self.handle_style_def(ids, styles),
                Stmt::Direction(dir) => self.direction = Some(dir.clone()),
                Stmt::AccTitle(t) => self.acc_title = Some(t.clone()),
                Stmt::AccDescr(d) => self.acc_descr = Some(normalize_multiline_ws(d)),
                Stmt::Click(c) => self.add_link(&c.id, &c.url, &c.tooltip),
                Stmt::Noop => {}
            }
        }
    }

    fn add_link(&mut self, state_id: &str, url: &str, tooltip: &str) {
        self.links
            .entry(state_id.to_string())
            .or_default()
            .push(Link {
                url: url.to_string(),
                tooltip: tooltip.to_string(),
            });
    }

    fn ensure_state(&mut self, id: &str) -> &mut StateRecord {
        let id = id.trim().to_string();
        match self.states.entry(id.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                self.state_order.push(id.clone());
                entry.insert(StateRecord {
                    id,
                    ty: "default".to_string(),
                    descriptions: Vec::new(),
                    note: None,
                    classes: Vec::new(),
                    styles: Vec::new(),
                    text_styles: Vec::new(),
                    start: None,
                })
            }
        }
    }

    fn add_description(&mut self, id: &str, descr: &str) {
        let clean = descr.trim().trim_start_matches(':').trim().to_string();
        if clean.is_empty() {
            return;
        }
        self.ensure_state(id).descriptions.push(clean);
    }

    fn add_state(&mut self, state: &StateStmt) {
        let id = state.id.trim();
        let st = self.ensure_state(id);
        if st.ty == "default" && state.ty != "default" {
            st.ty = state.ty.clone();
        }
        if st.start.is_none() {
            st.start = state.start;
        }
        if state.note.is_some() {
            st.note = state.note.clone();
        }

        if let Some(d) = state.description.as_deref() {
            self.add_description(id, d);
        }
        for d in &state.descriptions {
            self.add_description(id, d);
        }
        for c in &state.classes {
            self.set_css_class(id, c);
        }
        for s in &state.styles {
            self.set_style(id, s);
        }
        for ts in &state.text_styles {
            self.set_text_style(id, ts);
        }
    }

    fn add_relation(&mut self, s1: &StateStmt, s2: &StateStmt, title: Option<&str>) {
        let id1 = s1.id.trim();
        let id2 = s2.id.trim();

        self.add_state(s1);
        self.add_state(s2);

        let relation_title = title
            .map(|t| t.trim().to_string())
            .filter(|s| !s.is_empty());

        // Mermaid `@11.12.2` self-loops are special-cased during rendering/layout (fixed
        // `*-cyclic-special-*` ids). Multiple self-loop statements on the same node effectively
        // overwrite; keep the latest label/title.
        if id1 == id2 {
            if let Some(existing) = self
                .relations
                .iter_mut()
                .find(|r| r.id1 == id1 && r.id2 == id2)
            {
                existing.relation_title = relation_title;
                return;
            }
        }

        self.relations.push(RelationEdge {
            id1: id1.to_string(),
            id2: id2.to_string(),
            relation_title,
        });
    }

    fn add_style_class(&mut self, id: &str, style_attributes: &str) {
        let entry = self
            .style_classes
            .entry(id.trim().to_string())
            .or_insert_with(|| StyleClass {
                id: id.trim().to_string(),
                styles: Vec::new(),
                text_styles: Vec::new(),
            });

        for attrib in style_attributes.split(',') {
            let fixed = attrib
                .split_once(';')
                .map(|(a, _)| a)
                .unwrap_or(attrib)
                .trim()
                .to_string();
            if fixed.is_empty() {
                continue;
            }
            if attrib.contains("color") {
                let t1 = fixed.replace("fill", "bgFill");
                let t2 = t1.replace("color", "fill");
                entry.text_styles.push(t2);
            }
            entry.styles.push(fixed);
        }
    }

    fn set_css_class(&mut self, item_ids: &str, css_class_name: &str) {
        for id in item_ids.split(',') {
            let trimmed = id.trim();
            if trimmed.is_empty() {
                continue;
            }
            self.ensure_state(trimmed)
                .classes
                .push(css_class_name.trim().to_string());
        }
    }

    fn set_style(&mut self, item_id: &str, style_text: &str) {
        self.ensure_state(item_id)
            .styles
            .push(style_text.trim().to_string());
    }

    fn set_text_style(&mut self, item_id: &str, style_text: &str) {
        self.ensure_state(item_id)
            .text_styles
            .push(style_text.trim().to_string());
    }

    fn handle_style_def(&mut self, ids: &str, styles: &str) {
        let styles_vec: Vec<String> = styles
            .split(',')
            .map(|s| s.replace(';', "").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        for id in ids.split(',') {
            let trimmed = id.trim();
            if trimmed.is_empty() {
                continue;
            }
            self.ensure_state(trimmed).styles = styles_vec.clone();
        }
    }

    pub(super) fn to_model(&self, meta: &ParseMetadata) -> Result<Value> {
        let mut doc_json_by_state_id = root_state_doc_json_by_id(&self.root_doc);
        let mut states_json = Map::new();
        for id in &self.state_order {
            let Some(state) = self.states.get(id) else {
                continue;
            };
            let doc = doc_json_by_state_id.remove(&state.id);
            states_json.insert(state.id.clone(), state.to_json(doc));
        }

        let relations_json: Vec<Value> = self
            .relations
            .iter()
            .map(|r| {
                json!({
                    "id1": r.id1,
                    "id2": r.id2,
                    "relationTitle": r.relation_title,
                })
            })
            .collect();

        let style_classes_json: serde_json::Map<String, Value> = self
            .style_classes
            .iter()
            .map(|(k, sc)| {
                (
                    k.clone(),
                    json!({
                        "id": sc.id,
                        "styles": sc.styles,
                        "textStyles": sc.text_styles,
                    }),
                )
            })
            .collect();

        let look = meta
            .effective_config
            .as_value()
            .as_object()
            .and_then(|o| o.get("look"))
            .cloned()
            .unwrap_or(Value::Null);

        let (nodes_json, edges_json) = build_layout_data(
            &self.root_doc,
            &self.states,
            &self.style_classes,
            &meta.effective_config,
            &look,
        )
        .map_err(|message| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message,
        })?;

        let links_json: serde_json::Map<String, Value> = self
            .links
            .iter()
            .map(|(k, links)| {
                let mut out_links: Vec<Value> = links
                    .iter()
                    .map(|l| {
                        json!({
                            "url": l.url,
                            "tooltip": l.tooltip,
                        })
                    })
                    .collect();
                let v = if out_links.len() == 1 {
                    out_links.pop().unwrap_or(Value::Null)
                } else {
                    Value::Array(out_links)
                };
                (k.clone(), v)
            })
            .collect();

        let mut root = Map::new();
        root.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
        // Mermaid's `StateDB.getData()` returns a layout-ready `{ nodes, edges, other, config, direction }`.
        // We keep additional keys (`states`, `relations`, `styleClasses`, `links`) to help downstream
        // integrations and parity debugging.
        root.insert("nodes".to_string(), Value::Array(nodes_json));
        root.insert("edges".to_string(), Value::Array(edges_json));
        root.insert("other".to_string(), Value::Object(Map::new()));
        root.insert(
            "config".to_string(),
            crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
        );
        root.insert(
            "direction".to_string(),
            Value::String(self.direction.clone().unwrap_or_else(|| "TB".to_string())),
        );
        root.insert("accTitle".to_string(), option_string_value(&self.acc_title));
        root.insert("accDescr".to_string(), option_string_value(&self.acc_descr));
        root.insert("states".to_string(), Value::Object(states_json));
        root.insert("relations".to_string(), Value::Array(relations_json));
        root.insert(
            "styleClasses".to_string(),
            Value::Object(style_classes_json),
        );
        root.insert("links".to_string(), Value::Object(links_json));
        Ok(Value::Object(root))
    }

    pub(super) fn to_model_for_render_typed(
        &self,
        meta: &ParseMetadata,
    ) -> Result<StateDiagramRenderModel> {
        let (nodes, edges) = build_layout_data_typed(
            &self.root_doc,
            &self.states,
            &self.style_classes,
            &meta.effective_config,
        )
        .map_err(|message| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message,
        })?;

        let states: HashMap<String, StateDiagramRenderState> = self
            .state_order
            .iter()
            .filter_map(|id| self.states.get(id))
            .map(|s| {
                let note = s.note.as_ref().map(|n| StateDiagramRenderNote {
                    position: n.position.clone(),
                    text: n.text.clone(),
                });
                (s.id.clone(), StateDiagramRenderState { note })
            })
            .collect();

        let style_classes: IndexMap<String, StateDiagramRenderStyleClass> = self
            .style_classes
            .iter()
            .map(|(k, sc)| {
                (
                    k.clone(),
                    StateDiagramRenderStyleClass {
                        id: sc.id.clone(),
                        styles: sc.styles.clone(),
                        text_styles: sc.text_styles.clone(),
                    },
                )
            })
            .collect();

        let links: HashMap<String, StateDiagramRenderLinks> = self
            .links
            .iter()
            .map(|(k, v)| {
                let links = if v.len() == 1 {
                    let l = &v[0];
                    StateDiagramRenderLinks::One(StateDiagramRenderLink {
                        url: l.url.clone(),
                        tooltip: l.tooltip.clone(),
                    })
                } else {
                    StateDiagramRenderLinks::Many(
                        v.iter()
                            .map(|l| StateDiagramRenderLink {
                                url: l.url.clone(),
                                tooltip: l.tooltip.clone(),
                            })
                            .collect(),
                    )
                };
                (k.clone(), links)
            })
            .collect();

        Ok(StateDiagramRenderModel {
            direction: self.direction.clone().unwrap_or_else(|| "TB".to_string()),
            acc_title: self.acc_title.clone(),
            acc_descr: self.acc_descr.clone(),
            nodes,
            edges,
            links,
            states,
            style_classes,
        })
    }
}

impl Drop for StateDb {
    fn drop(&mut self) {
        let root_doc = std::mem::take(&mut self.root_doc);
        drop_doc_nonrecursive(root_doc);
    }
}

#[derive(Debug, Clone)]
struct Link {
    url: String,
    tooltip: String,
}

const DEFAULT_NESTED_DOC_DIR: &str = "TB";

const G_EDGE_STYLE: &str = "fill:none";
const G_EDGE_ARROWHEADSTYLE: &str = "fill: #333";
const G_EDGE_LABELPOS: &str = "c";
const G_EDGE_LABELTYPE: &str = "text";
const G_EDGE_THICKNESS: &str = "normal";

const DOMID_STATE: &str = "state";
const DOMID_TYPE_SPACER: &str = "----";

const NOTE: &str = "note";
const PARENT: &str = "parent";
const NOTE_ID: &str = "----note";
const PARENT_ID: &str = "----parent";

const SHAPE_STATE: &str = "rect";
const SHAPE_STATE_WITH_DESC: &str = "rectWithTitle";
const SHAPE_START: &str = "stateStart";
const SHAPE_END: &str = "stateEnd";
const SHAPE_DIVIDER: &str = "divider";
const SHAPE_GROUP: &str = "roundedWithTitle";
const SHAPE_NOTE: &str = "note";
const SHAPE_NOTEGROUP: &str = "noteGroup";

const CSS_EDGE: &str = "transition";
const CSS_EDGE_NOTE_EDGE: &str = "transition note-edge";
const CSS_DIAGRAM_STATE: &str = "statediagram-state";
const CSS_DIAGRAM_NOTE: &str = "statediagram-note";
const CSS_DIAGRAM_CLUSTER: &str = "statediagram-cluster";
const CSS_DIAGRAM_CLUSTER_ALT: &str = "statediagram-cluster-alt";

fn state_dom_id(item_id: &str, counter: usize, ty: Option<&str>) -> String {
    let type_str = ty
        .filter(|t| !t.is_empty())
        .map(|t| format!("{DOMID_TYPE_SPACER}{t}"))
        .unwrap_or_default();
    format!("{DOMID_STATE}-{item_id}{type_str}-{counter}")
}

fn string_array_value(values: &[String]) -> Value {
    Value::Array(values.iter().cloned().map(Value::String).collect())
}

fn option_string_value(value: &Option<String>) -> Value {
    value
        .as_ref()
        .map(|v| Value::String(v.clone()))
        .unwrap_or(Value::Null)
}

fn option_bool_value(value: Option<bool>) -> Value {
    value.map(Value::Bool).unwrap_or(Value::Null)
}

fn note_to_json(note: &Note) -> Value {
    let mut obj = Map::new();
    obj.insert("position".to_string(), option_string_value(&note.position));
    obj.insert("text".to_string(), Value::String(note.text.clone()));
    Value::Object(obj)
}

fn drop_doc_nonrecursive(doc: Vec<Stmt>) {
    let mut stack = vec![doc];
    while let Some(mut current_doc) = stack.pop() {
        while let Some(mut stmt) = current_doc.pop() {
            if let Stmt::State(state) = &mut stmt {
                if let Some(child_doc) = state.doc.take() {
                    stack.push(child_doc);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct NodeScratch {
    id: String,
    shape: String,
    label: Value,
    css_classes: String,
    css_styles: Vec<String>,
    node_type: Option<String>,
    dir: Option<String>,
    is_group: bool,
    parent_id: Option<String>,
}

fn get_dir_for_doc(doc: &[Stmt], default_dir: &str) -> String {
    let mut dir = default_dir.to_string();
    for stmt in doc {
        if let Stmt::Direction(d) = stmt {
            dir = d.clone();
        }
    }
    dir
}

fn compiled_styles(css_classes: &str, classes: &IndexMap<String, StyleClass>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for class_name in css_classes.split_whitespace() {
        if let Some(c) = classes.get(class_name) {
            out.extend(c.styles.iter().cloned());
        }
    }
    out
}

fn upsert_node(nodes: &mut Vec<Value>, index: &mut HashMap<String, usize>, node: Value) {
    let Some(id) = node.get("id").and_then(|v| v.as_str()) else {
        return;
    };
    match index.entry(id.to_string()) {
        Entry::Occupied(o) => {
            if let Some(dst) = nodes.get_mut(*o.get()) {
                if let (Some(dst_obj), Some(src_obj)) = (dst.as_object_mut(), node.as_object()) {
                    for (k, v) in src_obj {
                        dst_obj.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        Entry::Vacant(v) => {
            v.insert(nodes.len());
            nodes.push(node);
        }
    }
}

fn upsert_node_typed(
    nodes: &mut Vec<StateDiagramRenderNode>,
    index: &mut HashMap<String, usize>,
    node: StateDiagramRenderNode,
) {
    match index.entry(node.id.clone()) {
        Entry::Occupied(o) => {
            if let Some(dst) = nodes.get_mut(*o.get()) {
                *dst = node;
            }
        }
        Entry::Vacant(v) => {
            v.insert(nodes.len());
            nodes.push(node);
        }
    }
}

fn value_as_f64(v: &Value) -> Option<f64> {
    v.as_f64()
        .or_else(|| v.as_i64().map(|n| n as f64))
        .or_else(|| v.as_u64().map(|n| n as f64))
}

fn arrow_type_end_for_look_name(look: Option<&str>) -> &'static str {
    if look == Some("neo") {
        "arrow_barb_neo"
    } else {
        "arrow_barb"
    }
}

fn arrow_type_end_for_look(look: &Value) -> &'static str {
    arrow_type_end_for_look_name(look.as_str())
}

fn build_layout_data_typed(
    root_doc: &[Stmt],
    states: &HashMap<String, StateRecord>,
    classes: &IndexMap<String, StyleClass>,
    config: &MermaidConfig,
) -> std::result::Result<(Vec<StateDiagramRenderNode>, Vec<StateDiagramRenderEdge>), String> {
    let mut nodes: Vec<StateDiagramRenderNode> = Vec::new();
    let mut edges: Vec<StateDiagramRenderEdge> = Vec::new();
    let mut node_index: HashMap<String, usize> = HashMap::new();

    let mut node_db: HashMap<String, NodeScratch> = HashMap::new();
    let mut graph_item_count: usize = 0;

    struct TypedLayoutContext<'a> {
        states: &'a HashMap<String, StateRecord>,
        classes: &'a IndexMap<String, StyleClass>,
        config: &'a MermaidConfig,
        nodes: &'a mut Vec<StateDiagramRenderNode>,
        node_index: &'a mut HashMap<String, usize>,
        edges: &'a mut Vec<StateDiagramRenderEdge>,
        node_db: &'a mut HashMap<String, NodeScratch>,
        graph_item_count: &'a mut usize,
    }

    fn setup_doc(
        ctx: &mut TypedLayoutContext<'_>,
        parent: Option<&StateStmt>,
        doc: &[Stmt],
        alt_flag: bool,
    ) -> std::result::Result<(), String> {
        struct DocFrame<'a> {
            parent: Option<&'a StateStmt>,
            doc: &'a [Stmt],
            index: usize,
            alt_flag: bool,
        }

        let mut stack = vec![DocFrame {
            parent,
            doc,
            index: 0,
            alt_flag,
        }];

        while let Some(frame) = stack.last_mut() {
            let Some(item) = frame.doc.get(frame.index) else {
                stack.pop();
                continue;
            };
            frame.index += 1;
            let parent = frame.parent;
            let alt_flag = frame.alt_flag;

            match item {
                Stmt::State(s) => {
                    data_fetcher(ctx, parent, s, alt_flag)?;
                    if let Some(doc) = s.doc.as_ref() {
                        stack.push(DocFrame {
                            parent: Some(s),
                            doc,
                            index: 0,
                            alt_flag: !alt_flag,
                        });
                    }
                }
                Stmt::Relation(relation) => {
                    let relation = relation.as_ref();
                    data_fetcher(ctx, parent, &relation.state1, alt_flag)?;
                    data_fetcher(ctx, parent, &relation.state2, alt_flag)?;

                    let edge_label_raw = relation.description.clone().unwrap_or_default();
                    let edge_label = sanitize_text(&edge_label_raw, ctx.config);
                    ctx.edges.push(StateDiagramRenderEdge {
                        id: format!("edge{}", *ctx.graph_item_count),
                        start: relation.state1.id.clone(),
                        end: relation.state2.id.clone(),
                        arrow_type_end: arrow_type_end_for_look_name(ctx.config.get_str("look"))
                            .to_string(),
                        classes: CSS_EDGE.to_string(),
                        label: edge_label,
                    });
                    *ctx.graph_item_count += 1;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn data_fetcher(
        ctx: &mut TypedLayoutContext<'_>,
        parent: Option<&StateStmt>,
        parsed_item: &StateStmt,
        alt_flag: bool,
    ) -> std::result::Result<(), String> {
        let item_id = parsed_item.id.clone();
        if item_id == "root" || item_id.is_empty() {
            return Ok(());
        }

        let states = ctx.states;
        let classes = ctx.classes;
        let config = ctx.config;

        let db_state = states.get(&item_id);
        let class_str = db_state.map(|s| s.classes.join(" ")).unwrap_or_default();
        let styles = db_state.map(|s| s.styles.clone()).unwrap_or_default();

        let entry = ctx.node_db.entry(item_id.clone()).or_insert_with(|| {
            let mut css_classes = String::new();
            if !class_str.trim().is_empty() {
                css_classes.push_str(class_str.trim());
                css_classes.push(' ');
            }
            css_classes.push_str(CSS_DIAGRAM_STATE);

            let mut shape = SHAPE_STATE.to_string();
            if parsed_item.start == Some(true) {
                shape = SHAPE_START.to_string();
            } else if parsed_item.start == Some(false) {
                shape = SHAPE_END.to_string();
            }
            if parsed_item.ty != "default" {
                shape = parsed_item.ty.clone();
            }

            NodeScratch {
                id: item_id.clone(),
                shape,
                label: json!(sanitize_text(&item_id, config)),
                css_classes,
                css_styles: styles.clone(),
                node_type: None,
                dir: None,
                is_group: false,
                parent_id: None,
            }
        });

        // Apply description statements like Mermaid's `dataFetcher.ts`.
        if let Some(descr) = parsed_item.description.as_deref() {
            let base_label = sanitize_text(&item_id, config);

            match &mut entry.label {
                Value::Array(arr) => {
                    entry.shape = SHAPE_STATE_WITH_DESC.to_string();
                    arr.push(Value::String(descr.to_string()));
                }
                Value::String(s) => {
                    if !s.is_empty() {
                        entry.shape = SHAPE_STATE_WITH_DESC.to_string();
                        if *s == base_label {
                            entry.label = Value::Array(vec![Value::String(descr.to_string())]);
                        } else {
                            entry.label = Value::Array(vec![
                                Value::String(s.clone()),
                                Value::String(descr.to_string()),
                            ]);
                        }
                    } else {
                        entry.shape = SHAPE_STATE.to_string();
                        entry.label = Value::String(descr.to_string());
                    }
                }
                _ => {
                    entry.shape = SHAPE_STATE.to_string();
                    entry.label = Value::String(descr.to_string());
                }
            }

            entry.label = sanitize_text_or_array(&entry.label, config);
        }

        // If there's only 1 description entry, just use a regular state shape.
        if entry.shape == SHAPE_STATE_WITH_DESC {
            if let Some(arr) = entry.label.as_array() {
                if arr.len() == 1 {
                    entry.shape = if entry.node_type.as_deref() == Some("group") {
                        SHAPE_GROUP.to_string()
                    } else {
                        SHAPE_STATE.to_string()
                    };
                }
            }
        }

        // Group handling (composite states)
        if entry.node_type.is_none() {
            if let Some(doc) = parsed_item.doc.as_ref() {
                entry.node_type = Some("group".to_string());
                entry.is_group = true;
                let dir = get_dir_for_doc(doc, DEFAULT_NESTED_DOC_DIR);
                entry.dir = Some(dir);
                entry.shape = if parsed_item.ty == "divider" {
                    SHAPE_DIVIDER.to_string()
                } else {
                    SHAPE_GROUP.to_string()
                };

                let mut css = entry.css_classes.clone();
                css.push(' ');
                css.push_str(CSS_DIAGRAM_CLUSTER);
                if alt_flag {
                    css.push(' ');
                    css.push_str(CSS_DIAGRAM_CLUSTER_ALT);
                }
                entry.css_classes = css;
            }
        }

        if let Some(p) = parent {
            if p.id != "root" {
                entry.parent_id = Some(p.id.clone());
            }
        }

        let dom_id = state_dom_id(&item_id, *ctx.graph_item_count, None);
        let mut label = Some(entry.label.clone());
        if entry.shape == SHAPE_DIVIDER {
            label = Some(Value::String(String::new()));
        }

        let mut node = StateDiagramRenderNode {
            id: entry.id.clone(),
            label_style: String::new(),
            label,
            description: None,
            dom_id,
            is_group: entry.is_group,
            parent_id: entry.parent_id.clone(),
            css_classes: entry.css_classes.clone(),
            css_compiled_styles: Vec::new(),
            css_styles: entry.css_styles.clone(),
            dir: entry.dir.clone(),
            padding: Some(8.0),
            rx: Some(10.0),
            ry: Some(10.0),
            shape: entry.shape.clone(),
            position: None,
        };
        node.css_compiled_styles = compiled_styles(&node.css_classes, classes);

        // Notes create a note node + note group + note edge
        if let Some(mut n) = parsed_item.note.clone() {
            let flowchart_padding = config
                .as_value()
                .as_object()
                .and_then(|o| o.get("flowchart"))
                .and_then(|v| v.as_object())
                .and_then(|o| o.get("padding"))
                .and_then(value_as_f64);

            n.text = sanitize_text(&n.text, config);

            let note_id = format!("{item_id}{NOTE_ID}-{}", *ctx.graph_item_count);
            let parent_node_id = format!("{item_id}{PARENT_ID}");
            let note_dom_id = state_dom_id(&item_id, *ctx.graph_item_count, Some(NOTE));
            let group_dom_id = state_dom_id(&item_id, *ctx.graph_item_count, Some(PARENT));

            let mut group_node = StateDiagramRenderNode {
                id: parent_node_id.clone(),
                label_style: String::new(),
                label: Some(Value::String(n.text.clone())),
                description: None,
                dom_id: group_dom_id,
                is_group: true,
                parent_id: None,
                css_classes: node.css_classes.clone(),
                css_compiled_styles: Vec::new(),
                css_styles: Vec::new(),
                dir: None,
                padding: Some(16.0),
                rx: None,
                ry: None,
                shape: SHAPE_NOTEGROUP.to_string(),
                position: n.position.clone(),
            };
            group_node.css_compiled_styles = compiled_styles(&group_node.css_classes, classes);

            let mut note_node = StateDiagramRenderNode {
                id: note_id.clone(),
                label_style: String::new(),
                label: Some(Value::String(n.text.clone())),
                description: None,
                dom_id: note_dom_id,
                is_group: entry.is_group,
                parent_id: Some(parent_node_id.clone()),
                css_classes: CSS_DIAGRAM_NOTE.to_string(),
                css_compiled_styles: Vec::new(),
                css_styles: Vec::new(),
                dir: None,
                padding: flowchart_padding,
                rx: None,
                ry: None,
                shape: SHAPE_NOTE.to_string(),
                position: n.position.clone(),
            };
            note_node.css_compiled_styles = compiled_styles(&note_node.css_classes, classes);

            upsert_node_typed(ctx.nodes, ctx.node_index, group_node);
            upsert_node_typed(ctx.nodes, ctx.node_index, note_node);
            upsert_node_typed(ctx.nodes, ctx.node_index, node.clone());

            let (mut from, mut to) = (item_id.clone(), note_id);
            if n.position.as_deref() == Some("left of") {
                std::mem::swap(&mut from, &mut to);
            }

            ctx.edges.push(StateDiagramRenderEdge {
                id: format!("{from}-{to}"),
                start: from,
                end: to,
                arrow_type_end: String::new(),
                classes: CSS_EDGE_NOTE_EDGE.to_string(),
                label: String::new(),
            });
            *ctx.graph_item_count += 1;
        } else {
            upsert_node_typed(ctx.nodes, ctx.node_index, node);
        }

        Ok(())
    }

    {
        let mut ctx = TypedLayoutContext {
            states,
            classes,
            config,
            nodes: &mut nodes,
            node_index: &mut node_index,
            edges: &mut edges,
            node_db: &mut node_db,
            graph_item_count: &mut graph_item_count,
        };
        setup_doc(&mut ctx, None, root_doc, false)?;
    }

    // Post-process label arrays into (label, description) like Mermaid's StateDB.extract().
    for node in nodes.iter_mut() {
        let Some(label_val) = node.label.clone() else {
            continue;
        };
        let Some(arr) = label_val.as_array() else {
            continue;
        };
        if arr.is_empty() {
            continue;
        }
        let label0 = arr[0].clone();
        let rest: Vec<String> = arr
            .iter()
            .skip(1)
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if node.is_group && !rest.is_empty() {
            return Err("Group nodes can only have label".to_string());
        }
        node.label = Some(label0);
        if !rest.is_empty() {
            node.description = Some(rest);
        }
    }

    Ok((nodes, edges))
}

fn build_layout_data(
    root_doc: &[Stmt],
    states: &HashMap<String, StateRecord>,
    classes: &IndexMap<String, StyleClass>,
    config: &MermaidConfig,
    look: &Value,
) -> std::result::Result<(Vec<Value>, Vec<Value>), String> {
    let mut nodes: Vec<Value> = Vec::new();
    let mut edges: Vec<Value> = Vec::new();
    let mut node_index: HashMap<String, usize> = HashMap::new();

    let mut node_db: HashMap<String, NodeScratch> = HashMap::new();
    let mut graph_item_count: usize = 0;

    struct JsonLayoutContext<'a> {
        states: &'a HashMap<String, StateRecord>,
        classes: &'a IndexMap<String, StyleClass>,
        config: &'a MermaidConfig,
        look: &'a Value,
        nodes: &'a mut Vec<Value>,
        node_index: &'a mut HashMap<String, usize>,
        edges: &'a mut Vec<Value>,
        node_db: &'a mut HashMap<String, NodeScratch>,
        graph_item_count: &'a mut usize,
    }

    fn setup_doc(
        ctx: &mut JsonLayoutContext<'_>,
        parent: Option<&StateStmt>,
        doc: &[Stmt],
        alt_flag: bool,
    ) -> std::result::Result<(), String> {
        struct DocFrame<'a> {
            parent: Option<&'a StateStmt>,
            doc: &'a [Stmt],
            index: usize,
            alt_flag: bool,
        }

        let mut stack = vec![DocFrame {
            parent,
            doc,
            index: 0,
            alt_flag,
        }];

        while let Some(frame) = stack.last_mut() {
            let Some(item) = frame.doc.get(frame.index) else {
                stack.pop();
                continue;
            };
            frame.index += 1;
            let parent = frame.parent;
            let alt_flag = frame.alt_flag;

            match item {
                Stmt::State(s) => {
                    data_fetcher(ctx, parent, s, alt_flag)?;
                    if let Some(doc) = s.doc.as_ref() {
                        stack.push(DocFrame {
                            parent: Some(s),
                            doc,
                            index: 0,
                            alt_flag: !alt_flag,
                        });
                    }
                }
                Stmt::Relation(relation) => {
                    let relation = relation.as_ref();
                    data_fetcher(ctx, parent, &relation.state1, alt_flag)?;
                    data_fetcher(ctx, parent, &relation.state2, alt_flag)?;

                    let edge_label_raw = relation.description.clone().unwrap_or_default();
                    let edge_label = sanitize_text(&edge_label_raw, ctx.config);
                    ctx.edges.push(json!({
                        "id": format!("edge{}", *ctx.graph_item_count),
                        "start": relation.state1.id,
                        "end": relation.state2.id,
                        "arrowhead": "normal",
                        "arrowTypeEnd": arrow_type_end_for_look(ctx.look),
                        "style": G_EDGE_STYLE,
                        "labelStyle": "",
                        "label": edge_label,
                        "arrowheadStyle": G_EDGE_ARROWHEADSTYLE,
                        "labelpos": G_EDGE_LABELPOS,
                        "labelType": G_EDGE_LABELTYPE,
                        "thickness": G_EDGE_THICKNESS,
                        "classes": CSS_EDGE,
                        "look": ctx.look,
                    }));
                    *ctx.graph_item_count += 1;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn data_fetcher(
        ctx: &mut JsonLayoutContext<'_>,
        parent: Option<&StateStmt>,
        parsed_item: &StateStmt,
        alt_flag: bool,
    ) -> std::result::Result<(), String> {
        let item_id = parsed_item.id.clone();
        if item_id == "root" || item_id.is_empty() {
            return Ok(());
        }

        let states = ctx.states;
        let classes = ctx.classes;
        let config = ctx.config;
        let look = ctx.look;

        let db_state = states.get(&item_id);
        let class_str = db_state.map(|s| s.classes.join(" ")).unwrap_or_default();
        let styles = db_state.map(|s| s.styles.clone()).unwrap_or_default();

        let entry = ctx.node_db.entry(item_id.clone()).or_insert_with(|| {
            let mut css_classes = String::new();
            if !class_str.trim().is_empty() {
                css_classes.push_str(class_str.trim());
                css_classes.push(' ');
            }
            css_classes.push_str(CSS_DIAGRAM_STATE);

            let mut shape = SHAPE_STATE.to_string();
            if parsed_item.start == Some(true) {
                shape = SHAPE_START.to_string();
            } else if parsed_item.start == Some(false) {
                shape = SHAPE_END.to_string();
            }
            if parsed_item.ty != "default" {
                shape = parsed_item.ty.clone();
            }

            NodeScratch {
                id: item_id.clone(),
                shape,
                label: json!(sanitize_text(&item_id, config)),
                css_classes,
                css_styles: styles.clone(),
                node_type: None,
                dir: None,
                is_group: false,
                parent_id: None,
            }
        });

        // Apply description statements like Mermaid's `dataFetcher.ts`.
        //
        // Note: Mermaid supports a compact form that combines a quoted label and a trailing `: ...`
        // description on the same line:
        //
        //   state "Some long name" as S1: The description
        //
        // The lexer captures `S1: The description` as a single `Id` token, which the grammar splits
        // into `id="S1"` + `descriptions=["The description"]`. Apply both the primary `description`
        // and any extra `descriptions` in order so the resulting `label` array becomes:
        //   ["Some long name", "The description"]
        // which later converts to (label, description[]) via `StateDB.extract()`.
        let mut descrs: Vec<&str> = Vec::new();
        if let Some(d) = parsed_item.description.as_deref() {
            if !d.trim().is_empty() {
                descrs.push(d);
            }
        }
        for d in &parsed_item.descriptions {
            if !d.trim().is_empty() {
                descrs.push(d);
            }
        }

        if !descrs.is_empty() {
            let base_label = sanitize_text(&item_id, config);
            for descr in descrs {
                match &mut entry.label {
                    Value::Array(arr) => {
                        entry.shape = SHAPE_STATE_WITH_DESC.to_string();
                        arr.push(Value::String(descr.to_string()));
                    }
                    Value::String(s) => {
                        if !s.is_empty() {
                            entry.shape = SHAPE_STATE_WITH_DESC.to_string();
                            if *s == base_label {
                                entry.label = Value::Array(vec![Value::String(descr.to_string())]);
                            } else {
                                entry.label = Value::Array(vec![
                                    Value::String(s.clone()),
                                    Value::String(descr.to_string()),
                                ]);
                            }
                        } else {
                            entry.shape = SHAPE_STATE.to_string();
                            entry.label = Value::String(descr.to_string());
                        }
                    }
                    _ => {
                        entry.shape = SHAPE_STATE.to_string();
                        entry.label = Value::String(descr.to_string());
                    }
                }
            }

            entry.label = sanitize_text_or_array(&entry.label, config);

            // If there's only 1 description entry, just use a regular state shape.
            if entry.shape == SHAPE_STATE_WITH_DESC {
                if let Some(arr) = entry.label.as_array() {
                    if arr.len() == 1 {
                        entry.shape = if entry.node_type.as_deref() == Some("group") {
                            SHAPE_GROUP.to_string()
                        } else {
                            SHAPE_STATE.to_string()
                        };
                    }
                }
            }
        }

        // Group handling (composite states)
        if entry.node_type.is_none() {
            if let Some(doc) = parsed_item.doc.as_ref() {
                entry.node_type = Some("group".to_string());
                entry.is_group = true;
                let dir = get_dir_for_doc(doc, DEFAULT_NESTED_DOC_DIR);
                entry.dir = Some(dir);
                entry.shape = if parsed_item.ty == "divider" {
                    SHAPE_DIVIDER.to_string()
                } else {
                    SHAPE_GROUP.to_string()
                };

                let mut css = entry.css_classes.clone();
                css.push(' ');
                css.push_str(CSS_DIAGRAM_CLUSTER);
                if alt_flag {
                    css.push(' ');
                    css.push_str(CSS_DIAGRAM_CLUSTER_ALT);
                }
                entry.css_classes = css;
            }
        }

        if let Some(p) = parent {
            if p.id != "root" {
                entry.parent_id = Some(p.id.clone());
            }
        }

        let mut node_data = json!({
            "labelStyle": "",
            "shape": entry.shape,
            "label": entry.label,
            "cssClasses": entry.css_classes,
            "cssCompiledStyles": [],
            "cssStyles": entry.css_styles,
            "id": entry.id,
            "dir": entry.dir,
            "domId": state_dom_id(&item_id, *ctx.graph_item_count, None),
            "type": entry.node_type,
            "isGroup": entry.is_group,
            "padding": 8,
            "rx": 10,
            "ry": 10,
            "look": look,
            "parentId": entry.parent_id,
            "centerLabel": true,
        });

        if node_data["shape"].as_str() == Some(SHAPE_DIVIDER) {
            node_data["label"] = json!("");
        }

        // Notes create a note node + note group + note edge
        if let Some(mut n) = parsed_item.note.clone() {
            let flowchart_padding = config
                .as_value()
                .as_object()
                .and_then(|o| o.get("flowchart"))
                .and_then(|v| v.as_object())
                .and_then(|o| o.get("padding"))
                .cloned()
                .unwrap_or(Value::Null);

            n.text = sanitize_text(&n.text, config);

            let note_id = format!("{item_id}{NOTE_ID}-{}", *ctx.graph_item_count);
            let parent_node_id = format!("{item_id}{PARENT_ID}");
            let note_dom_id = state_dom_id(&item_id, *ctx.graph_item_count, Some(NOTE));
            let group_dom_id = state_dom_id(&item_id, *ctx.graph_item_count, Some(PARENT));

            let group_data = json!({
                "labelStyle": "",
                "shape": SHAPE_NOTEGROUP,
                "label": n.text,
                "cssClasses": entry.css_classes,
                "cssStyles": [],
                "cssCompiledStyles": [],
                "id": parent_node_id,
                "domId": group_dom_id,
                "type": "group",
                "isGroup": true,
                "padding": 16,
                "look": look,
                "position": n.position,
            });

            let note_data = json!({
                "labelStyle": "",
                "shape": SHAPE_NOTE,
                "label": n.text,
                "cssClasses": CSS_DIAGRAM_NOTE,
                "cssStyles": [],
                "cssCompiledStyles": [],
                "id": note_id,
                "domId": note_dom_id,
                "type": entry.node_type,
                "isGroup": entry.is_group,
                "padding": flowchart_padding,
                "look": look,
                "position": n.position,
                "parentId": format!("{item_id}{PARENT_ID}"),
            });

            upsert_node(ctx.nodes, ctx.node_index, group_data);
            upsert_node(ctx.nodes, ctx.node_index, note_data.clone());
            upsert_node(ctx.nodes, ctx.node_index, node_data.clone());

            // style compilation after insertion
            for id in [parent_node_id.as_str(), note_id.as_str(), item_id.as_str()] {
                if let Some(idx) = ctx.node_index.get(id).copied() {
                    let css = ctx.nodes[idx]["cssClasses"].as_str().unwrap_or("");
                    let compiled = compiled_styles(css, classes);
                    if let Some(obj) = ctx.nodes[idx].as_object_mut() {
                        obj.insert("cssCompiledStyles".to_string(), json!(compiled));
                    }
                }
            }

            let (mut from, mut to) = (item_id.clone(), note_id);
            if n.position.as_deref() == Some("left of") {
                std::mem::swap(&mut from, &mut to);
            }

            ctx.edges.push(json!({
                "id": format!("{from}-{to}"),
                "start": from,
                "end": to,
                "arrowhead": "none",
                "arrowTypeEnd": "",
                "style": G_EDGE_STYLE,
                "labelStyle": "",
                "classes": CSS_EDGE_NOTE_EDGE,
                "arrowheadStyle": G_EDGE_ARROWHEADSTYLE,
                "labelpos": G_EDGE_LABELPOS,
                "labelType": G_EDGE_LABELTYPE,
                "thickness": G_EDGE_THICKNESS,
                "look": look,
            }));
            *ctx.graph_item_count += 1;
        } else {
            let css = node_data["cssClasses"].as_str().unwrap_or("");
            let compiled = compiled_styles(css, classes);
            if let Some(obj) = node_data.as_object_mut() {
                obj.insert("cssCompiledStyles".to_string(), json!(compiled));
            }
            upsert_node(ctx.nodes, ctx.node_index, node_data);
        }

        Ok(())
    }

    {
        let mut ctx = JsonLayoutContext {
            states,
            classes,
            config,
            look,
            nodes: &mut nodes,
            node_index: &mut node_index,
            edges: &mut edges,
            node_db: &mut node_db,
            graph_item_count: &mut graph_item_count,
        };
        setup_doc(&mut ctx, None, root_doc, false)?;
    }

    // Post-process label arrays into (label, description) like Mermaid's StateDB.extract().
    for node in nodes.iter_mut() {
        let Some(obj) = node.as_object_mut() else {
            continue;
        };
        let Some(label_val) = obj.get("label").cloned() else {
            continue;
        };
        let Some(arr) = label_val.as_array() else {
            continue;
        };
        if arr.is_empty() {
            continue;
        }
        let label0 = arr[0].clone();
        let rest: Vec<Value> = arr.iter().skip(1).cloned().collect();

        if obj.get("isGroup").and_then(|v| v.as_bool()) == Some(true) && !rest.is_empty() {
            return Err("Group nodes can only have label".to_string());
        }
        obj.insert("label".to_string(), label0);
        obj.insert("description".to_string(), Value::Array(rest));
    }

    Ok((nodes, edges))
}

fn root_state_doc_json_by_id(root_doc: &[Stmt]) -> HashMap<String, Value> {
    let mut out = HashMap::new();
    for stmt in root_doc {
        let Stmt::State(state) = stmt else {
            continue;
        };
        if out.contains_key(&state.id) {
            continue;
        }
        let Some(doc) = state.doc.as_ref() else {
            continue;
        };
        out.insert(state.id.clone(), Value::Array(doc_to_json(doc)));
    }
    out
}

fn doc_to_json(doc: &[Stmt]) -> Vec<Value> {
    let mut out = Vec::with_capacity(doc.len());
    for stmt in doc {
        out.push(stmt_to_json(stmt));
    }
    out
}

fn state_stmt_ref_to_json(state: &StateStmt) -> Value {
    let mut obj = Map::new();
    obj.insert("id".to_string(), Value::String(state.id.clone()));
    obj.insert("type".to_string(), Value::String(state.ty.clone()));
    obj.insert("classes".to_string(), string_array_value(&state.classes));
    Value::Object(obj)
}

fn stmt_to_json_shallow(stmt: &Stmt, doc: Option<Vec<Value>>) -> Value {
    match stmt {
        Stmt::Noop => Value::Null,
        Stmt::State(s) => {
            let mut obj = Map::new();
            obj.insert("stmt".to_string(), Value::String("state".to_string()));
            obj.insert("id".to_string(), Value::String(s.id.clone()));
            obj.insert("type".to_string(), Value::String(s.ty.clone()));
            obj.insert(
                "description".to_string(),
                option_string_value(&s.description),
            );
            obj.insert(
                "doc".to_string(),
                doc.map(Value::Array).unwrap_or(Value::Null),
            );
            obj.insert("classes".to_string(), string_array_value(&s.classes));
            Value::Object(obj)
        }
        Stmt::Relation(relation) => {
            let mut obj = Map::new();
            obj.insert("stmt".to_string(), Value::String("relation".to_string()));
            obj.insert(
                "state1".to_string(),
                state_stmt_ref_to_json(&relation.state1),
            );
            obj.insert(
                "state2".to_string(),
                state_stmt_ref_to_json(&relation.state2),
            );
            obj.insert(
                "description".to_string(),
                option_string_value(&relation.description),
            );
            Value::Object(obj)
        }
        Stmt::ClassDef { id, classes } => {
            json!({ "stmt": "classDef", "id": id, "classes": classes })
        }
        Stmt::ApplyClass { ids, class_name } => {
            json!({ "stmt": "applyClass", "id": ids, "styleClass": class_name })
        }
        Stmt::Style { ids, styles } => json!({ "stmt": "style", "id": ids, "styleClass": styles }),
        Stmt::Direction(v) => json!({ "stmt": "dir", "value": v }),
        Stmt::AccTitle(t) => json!(t),
        Stmt::AccDescr(d) => json!(d),
        Stmt::Click(c) => {
            json!({ "stmt": "click", "id": c.id, "url": c.url, "tooltip": c.tooltip })
        }
    }
}

fn stmt_to_json(stmt: &Stmt) -> Value {
    let mut stack: Vec<(&Stmt, bool)> = vec![(stmt, false)];
    let mut completed: HashMap<*const Stmt, Value> = HashMap::new();

    while let Some((current, visited)) = stack.pop() {
        if visited {
            let doc = match current {
                Stmt::State(s) => s.doc.as_ref().map(|children| {
                    let mut values = Vec::with_capacity(children.len());
                    for child in children {
                        values.push(
                            completed
                                .remove(&(child as *const Stmt))
                                .unwrap_or(Value::Null),
                        );
                    }
                    values
                }),
                _ => None,
            };
            completed.insert(current as *const Stmt, stmt_to_json_shallow(current, doc));
        } else {
            stack.push((current, true));
            if let Stmt::State(s) = current {
                if let Some(doc) = s.doc.as_ref() {
                    for child in doc.iter().rev() {
                        stack.push((child, false));
                    }
                }
            }
        }
    }

    completed
        .remove(&(stmt as *const Stmt))
        .unwrap_or(Value::Null)
}

fn normalize_multiline_ws(input: &str) -> String {
    let trimmed = input.trim();
    let mut out = String::with_capacity(trimmed.len());
    let mut chars = trimmed.chars().peekable();
    while let Some(ch) = chars.next() {
        out.push(ch);
        if ch == '\n' {
            while chars.peek().is_some_and(|c| c.is_whitespace()) {
                chars.next();
            }
        }
    }
    out
}
