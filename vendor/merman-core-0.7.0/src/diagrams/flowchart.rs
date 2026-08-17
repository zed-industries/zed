use crate::sanitize::sanitize_text;
use crate::{Error, MermaidConfig, ParseMetadata, Result};
use indexmap::IndexMap;
use serde_json::{Value, json};
use std::collections::HashMap;

lalrpop_util::lalrpop_mod!(
    #[allow(
        clippy::empty_line_after_outer_attr,
        clippy::type_complexity,
        clippy::result_large_err
    )]
    flowchart_grammar,
    "/diagrams/flowchart_grammar.rs"
);

mod accessibility;
mod ast;
mod build;
mod lex;
mod lexer;
mod lexer_iter;
mod link;
mod model;
mod semantic;
mod shape_data;
mod subgraph;
mod text;
mod tokens;

use text::{
    parse_edge_label_text, parse_label_text, strip_wrapping_backticks, title_kind_str, unquote,
};

pub use model::{FlowEdge, FlowEdgeDefaults, FlowNode, FlowSubgraph, FlowchartV2Model};

pub(crate) use model::{
    Edge, EdgeDefaults, LabeledText, LinkToken, Node, SubgraphHeader, TitleKind,
};

pub(crate) use ast::{
    ClassAssignStmt, ClassDefStmt, ClickAction, ClickStmt, FlowchartAst, LinkStylePos,
    LinkStyleStmt, Stmt, StyleStmt, SubgraphBlock,
};

pub(crate) use tokens::{LexError, NodeLabelToken, Tok};

use accessibility::extract_flowchart_accessibility_statements;
use build::FlowchartBuildState;
use lexer::Lexer;
use link::{destruct_end_link, destruct_start_link};
use semantic::{FlowchartSemanticContext, apply_semantic_statements};
use shape_data::{apply_shape_data_to_node, parse_shape_data_yaml, yaml_to_bool, yaml_to_string};
use subgraph::SubgraphBuilder;

#[derive(Debug, Clone)]
pub(crate) struct FlowSubGraph {
    pub id: String,
    pub nodes: Vec<String>,
    pub title: String,
    pub classes: Vec<String>,
    pub styles: Vec<String>,
    pub dir: Option<String>,
    pub label_type: String,
}

struct FlowchartSemanticSource {
    keyword: String,
    direction: Option<String>,
    acc_title: Option<String>,
    acc_descr: Option<String>,
    class_defs: IndexMap<String, Vec<String>>,
    tooltips: HashMap<String, String>,
    edge_defaults: EdgeDefaults,
    vertex_calls: Vec<String>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    subgraphs: Vec<FlowSubGraph>,
}

pub fn parse_flowchart(code: &str, meta: &ParseMetadata) -> Result<Value> {
    Ok(parse_flowchart_semantic_source(code, meta)?
        .into_compat_json(&meta.diagram_type, &meta.effective_config))
}

pub fn parse_flowchart_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<FlowchartV2Model> {
    parse_flowchart_semantic_source(code, meta)?.into_render_model(meta)
}

fn parse_flowchart_semantic_source(
    code: &str,
    meta: &ParseMetadata,
) -> Result<FlowchartSemanticSource> {
    let (code, acc_title, acc_descr) = extract_flowchart_accessibility_statements(code);
    let ast = flowchart_grammar::FlowchartAstParser::new()
        .parse(Lexer::new(&code))
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("{e:?}"),
        })?;

    let mut build = FlowchartBuildState::new();
    build
        .add_statements(&ast.statements)
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: e,
        })?;
    let FlowchartBuildState {
        nodes,
        edges,
        vertex_calls,
        ..
    } = build;
    let mut nodes = nodes;
    let mut edges = edges;

    let inherit_dir = meta
        .effective_config
        .as_value()
        .get("flowchart")
        .and_then(|v| v.get("inheritDir"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let mut builder = SubgraphBuilder::new(inherit_dir, ast.direction.clone());
    builder.visit_statements(&ast.statements);

    let mut class_defs: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut tooltips: HashMap<String, String> = HashMap::new();
    let mut edge_defaults = EdgeDefaults {
        style: Vec::new(),
        interpolate: None,
    };

    let mut node_index: HashMap<String, usize> = HashMap::new();
    for (idx, n) in nodes.iter().enumerate() {
        node_index.insert(n.id.clone(), idx);
    }
    let mut subgraph_index: HashMap<String, usize> = HashMap::new();
    for (idx, sg) in builder.subgraphs.iter().enumerate() {
        subgraph_index.insert(sg.id.clone(), idx);
    }

    let security_level_loose = meta.effective_config.get_str("securityLevel") == Some("loose");
    {
        let mut semantic_ctx = FlowchartSemanticContext {
            nodes: &mut nodes,
            node_index: &mut node_index,
            edges: &mut edges,
            subgraphs: &mut builder.subgraphs,
            subgraph_index: &mut subgraph_index,
            class_defs: &mut class_defs,
            tooltips: &mut tooltips,
            edge_defaults: &mut edge_defaults,
            security_level_loose,
            diagram_type: &meta.diagram_type,
            config: &meta.effective_config,
        };
        apply_semantic_statements(&ast.statements, &mut semantic_ctx)?;
    }

    Ok(FlowchartSemanticSource {
        keyword: ast.keyword,
        direction: ast.direction,
        acc_descr,
        acc_title,
        class_defs,
        tooltips,
        edge_defaults,
        vertex_calls,
        nodes,
        edges,
        subgraphs: builder.subgraphs,
    })
}

impl FlowchartSemanticSource {
    fn into_compat_json(self, diagram_type: &str, config: &MermaidConfig) -> Value {
        json!({
            "type": diagram_type,
            "keyword": self.keyword,
            "direction": self.direction,
            "accTitle": self.acc_title,
            "accDescr": self.acc_descr,
            "classDefs": self.class_defs,
            "tooltips": self.tooltips,
            "edgeDefaults": {
                "style": self.edge_defaults.style,
                "interpolate": self.edge_defaults.interpolate,
            },
            "vertexCalls": self.vertex_calls,
            "nodes": self
                .nodes
                .into_iter()
                .map(|node| flow_node_to_json(node, config))
                .collect::<Vec<_>>(),
            "edges": self
                .edges
                .into_iter()
                .map(|edge| flow_edge_to_json(edge, config))
                .collect::<Vec<_>>(),
            "subgraphs": self
                .subgraphs
                .into_iter()
                .map(flow_subgraph_to_json)
                .collect::<Vec<_>>(),
        })
    }

    fn into_render_model(self, meta: &ParseMetadata) -> Result<FlowchartV2Model> {
        Ok(FlowchartV2Model {
            acc_descr: self.acc_descr,
            acc_title: self.acc_title,
            class_defs: self.class_defs,
            direction: self.direction,
            edge_defaults: Some(FlowEdgeDefaults {
                style: self.edge_defaults.style,
                interpolate: self.edge_defaults.interpolate,
            }),
            vertex_calls: self.vertex_calls,
            nodes: self
                .nodes
                .into_iter()
                .map(|node| flow_node_to_model(node, &meta.effective_config))
                .collect::<Vec<_>>(),
            edges: self
                .edges
                .into_iter()
                .map(|edge| flow_edge_to_model(edge, meta))
                .collect::<Result<Vec<_>>>()?,
            subgraphs: self
                .subgraphs
                .into_iter()
                .map(flow_subgraph_to_model)
                .collect::<Vec<_>>(),
            tooltips: self.tooltips.into_iter().collect(),
        })
    }
}

fn flow_node_to_json(n: Node, config: &MermaidConfig) -> Value {
    let layout_shape = layout_shape_for_node(&n);
    let label = sanitized_node_label(&n, config);

    json!({
        "id": n.id,
        "label": label,
        "labelType": title_kind_str(&n.label_type),
        "shape": n.shape,
        "layoutShape": layout_shape,
        "icon": n.icon,
        "form": n.form,
        "pos": n.pos,
        "img": n.img,
        "constraint": n.constraint,
        "assetWidth": n.asset_width,
        "assetHeight": n.asset_height,
        "styles": n.styles,
        "classes": n.classes,
        "link": n.link,
        "linkTarget": n.link_target,
        "haveCallback": n.have_callback,
    })
}

fn flow_node_to_model(n: Node, config: &MermaidConfig) -> FlowNode {
    let layout_shape = layout_shape_for_node(&n);
    let label = sanitized_node_label(&n, config);

    FlowNode {
        id: n.id,
        label: Some(label),
        label_type: Some(title_kind_str(&n.label_type).to_string()),
        layout_shape: Some(layout_shape),
        icon: n.icon,
        form: n.form,
        pos: n.pos,
        img: n.img,
        constraint: n.constraint,
        asset_width: n.asset_width,
        asset_height: n.asset_height,
        classes: n.classes,
        styles: n.styles,
        link: n.link,
        link_target: n.link_target,
        have_callback: n.have_callback,
    }
}

fn flow_edge_to_json(e: Edge, config: &MermaidConfig) -> Value {
    let label = sanitized_optional_label(e.label.as_deref(), config);

    json!({
        "from": e.from,
        "to": e.to,
        "id": e.id,
        "isUserDefinedId": e.is_user_defined_id,
        "arrow": e.link.end,
        "type": e.link.edge_type,
        "stroke": e.link.stroke,
        "length": e.link.length,
        "label": label,
        "labelType": title_kind_str(&e.label_type),
        "style": e.style,
        "classes": e.classes,
        "interpolate": e.interpolate,
        "animate": e.animate,
        "animation": e.animation,
    })
}

fn flow_edge_to_model(e: Edge, meta: &ParseMetadata) -> Result<FlowEdge> {
    let label = sanitized_optional_label(e.label.as_deref(), &meta.effective_config);
    let id = e.id.ok_or_else(|| Error::DiagramParse {
        diagram_type: meta.diagram_type.clone(),
        message: "flowchart edge id missing".to_string(),
    })?;

    Ok(FlowEdge {
        id,
        from: e.from,
        to: e.to,
        label,
        label_type: Some(title_kind_str(&e.label_type).to_string()),
        edge_type: Some(e.link.edge_type),
        stroke: Some(e.link.stroke),
        length: e.link.length,
        style: e.style,
        classes: e.classes,
        interpolate: e.interpolate,
        animate: e.animate,
        animation: e.animation,
    })
}

fn layout_shape_for_node(n: &Node) -> String {
    // Mirrors Mermaid FlowDB `getTypeFromVertex` logic at 11.12.2.
    if n.img.is_some() {
        return "imageSquare".to_string();
    }
    if n.icon.is_some() {
        match n.form.as_deref() {
            Some("circle") => return "iconCircle".to_string(),
            Some("square") => return "iconSquare".to_string(),
            Some("rounded") => return "iconRounded".to_string(),
            _ => return "icon".to_string(),
        }
    }
    match n.shape.as_deref() {
        Some("square") | None => "squareRect".to_string(),
        Some("round") => "roundedRect".to_string(),
        Some("ellipse") => "ellipse".to_string(),
        Some(other) => other.to_string(),
    }
}

fn sanitized_node_label(n: &Node, config: &MermaidConfig) -> String {
    let label_raw = n.label.as_ref().unwrap_or(&n.id);
    let mut label = sanitized_label(label_raw, config);
    if label.len() >= 2 && label.starts_with('\"') && label.ends_with('\"') {
        label = label[1..label.len() - 1].to_string();
    }
    label
}

fn sanitized_optional_label(label: Option<&str>, config: &MermaidConfig) -> Option<String> {
    label.map(|s| sanitized_label(s, config))
}

fn sanitized_label(raw: &str, config: &MermaidConfig) -> String {
    let decoded = decode_mermaid_hash_entities(raw);
    sanitize_text(&decoded, config)
}

fn decode_mermaid_hash_entities(input: &str) -> std::borrow::Cow<'_, str> {
    // Mermaid runs `encodeEntities(...)` before parsing and later decodes with browser
    // `entityDecode(...)`. In our headless pipeline we decode into Unicode during parsing so
    // layout + SVG output match upstream.
    crate::entities::decode_mermaid_entities_to_unicode(input)
}

fn flow_subgraph_to_json(sg: FlowSubGraph) -> Value {
    let title = crate::entities::decode_mermaid_entities_to_unicode(&sg.title).into_owned();
    json!({
        "id": sg.id,
        "nodes": sg.nodes,
        "title": title,
        "classes": sg.classes,
        "styles": sg.styles,
        "dir": sg.dir,
        "labelType": sg.label_type,
    })
}

fn flow_subgraph_to_model(sg: FlowSubGraph) -> FlowSubgraph {
    FlowSubgraph {
        id: sg.id,
        nodes: sg.nodes,
        title: crate::entities::decode_mermaid_entities_to_unicode(&sg.title).into_owned(),
        classes: sg.classes,
        styles: sg.styles,
        dir: sg.dir,
        label_type: Some(sg.label_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flowchart_subgraphs_exist_matches_mermaid_flowdb_spec() {
        let subgraphs = vec![
            FlowSubGraph {
                id: "sg0".to_string(),
                nodes: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "e".to_string(),
                ],
                title: "".to_string(),
                classes: Vec::new(),
                styles: Vec::new(),
                dir: None,
                label_type: "text".to_string(),
            },
            FlowSubGraph {
                id: "sg1".to_string(),
                nodes: vec!["f".to_string(), "g".to_string(), "h".to_string()],
                title: "".to_string(),
                classes: Vec::new(),
                styles: Vec::new(),
                dir: None,
                label_type: "text".to_string(),
            },
            FlowSubGraph {
                id: "sg2".to_string(),
                nodes: vec!["i".to_string(), "j".to_string()],
                title: "".to_string(),
                classes: Vec::new(),
                styles: Vec::new(),
                dir: None,
                label_type: "text".to_string(),
            },
            FlowSubGraph {
                id: "sg3".to_string(),
                nodes: vec!["k".to_string()],
                title: "".to_string(),
                classes: Vec::new(),
                styles: Vec::new(),
                dir: None,
                label_type: "text".to_string(),
            },
        ];

        assert!(super::subgraph::subgraphs_exist(&subgraphs, "a"));
        assert!(super::subgraph::subgraphs_exist(&subgraphs, "h"));
        assert!(super::subgraph::subgraphs_exist(&subgraphs, "j"));
        assert!(super::subgraph::subgraphs_exist(&subgraphs, "k"));

        assert!(!super::subgraph::subgraphs_exist(&subgraphs, "a2"));
        assert!(!super::subgraph::subgraphs_exist(&subgraphs, "l"));
    }
}
