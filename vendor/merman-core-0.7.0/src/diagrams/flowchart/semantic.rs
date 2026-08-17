use crate::sanitize::sanitize_text;
use crate::utils::format_url;
use crate::{Error, MermaidConfig, Result};
use indexmap::IndexMap;
use std::collections::HashMap;

use super::{
    ClickAction, Edge, EdgeDefaults, FlowSubGraph, LinkStylePos, Node, Stmt, TitleKind,
    apply_shape_data_to_node, parse_shape_data_yaml, yaml_to_bool, yaml_to_string,
};

pub(super) struct FlowchartSemanticContext<'a> {
    pub(super) nodes: &'a mut Vec<Node>,
    pub(super) node_index: &'a mut HashMap<String, usize>,
    pub(super) edges: &'a mut Vec<Edge>,
    pub(super) subgraphs: &'a mut Vec<FlowSubGraph>,
    pub(super) subgraph_index: &'a mut HashMap<String, usize>,
    pub(super) class_defs: &'a mut IndexMap<String, Vec<String>>,
    pub(super) tooltips: &'a mut HashMap<String, String>,
    pub(super) edge_defaults: &'a mut EdgeDefaults,
    pub(super) security_level_loose: bool,
    pub(super) diagram_type: &'a str,
    pub(super) config: &'a MermaidConfig,
}

pub(super) fn apply_semantic_statements(
    statements: &[Stmt],
    ctx: &mut FlowchartSemanticContext<'_>,
) -> Result<()> {
    ctx.apply_statements(statements)
}

impl<'a> FlowchartSemanticContext<'a> {
    fn apply_statements(&mut self, statements: &[Stmt]) -> Result<()> {
        // Preserve the recursive preorder semantics while avoiding stack growth on nested
        // subgraphs.
        let mut stack = vec![statements.iter()];
        while let Some(iter) = stack.last_mut() {
            let Some(stmt) = iter.next() else {
                stack.pop();
                continue;
            };

            match stmt {
                Stmt::Subgraph(sg) => stack.push(sg.statements.iter()),
                Stmt::Style(s) => {
                    if let Some(&idx) = self.subgraph_index.get(&s.target) {
                        self.subgraphs[idx].styles.extend(s.styles.iter().cloned());
                    } else {
                        let idx = self.ensure_node(&s.target);
                        self.nodes[idx].styles.extend(s.styles.iter().cloned());
                    }
                }
                Stmt::ClassDef(c) => {
                    for id in &c.ids {
                        self.class_defs.insert(id.clone(), c.styles.clone());
                    }
                }
                Stmt::ClassAssign(c) => {
                    for target in &c.targets {
                        self.add_class_to_target(target, &c.class_name);
                    }
                }
                Stmt::Click(c) => {
                    for id in &c.ids {
                        if let Some(tt) = &c.tooltip {
                            self.tooltips
                                .insert(id.clone(), sanitize_text(tt, self.config));
                        }
                        self.add_class_to_target(id, "clickable");

                        match &c.action {
                            ClickAction::Link { href, target } => {
                                if let Some(&idx) = self.node_index.get(id) {
                                    self.nodes[idx].link = format_url(href, self.config);
                                    self.nodes[idx].link_target = target.clone();
                                }
                            }
                            ClickAction::Callback => {
                                if self.security_level_loose {
                                    if let Some(&idx) = self.node_index.get(id) {
                                        self.nodes[idx].have_callback = true;
                                    }
                                }
                            }
                        }
                    }
                }
                Stmt::LinkStyle(ls) => {
                    if let Some(algo) = &ls.interpolate {
                        for pos in &ls.positions {
                            match pos {
                                LinkStylePos::Default => {
                                    self.edge_defaults.interpolate = Some(algo.clone())
                                }
                                LinkStylePos::Index(i) => {
                                    if *i >= self.edges.len() {
                                        return Err(Error::DiagramParse {
                                            diagram_type: self.diagram_type.to_string(),
                                            message: format!(
                                                "The index {i} for linkStyle is out of bounds. Valid indices for linkStyle are between 0 and {}. (Help: Ensure that the index is within the range of existing edges.)",
                                                self.edges.len().saturating_sub(1)
                                            ),
                                        });
                                    }
                                    self.edges[*i].interpolate = Some(algo.clone());
                                }
                            }
                        }
                    }

                    if !ls.styles.is_empty() {
                        for pos in &ls.positions {
                            match pos {
                                LinkStylePos::Default => {
                                    self.edge_defaults.style = ls.styles.clone()
                                }
                                LinkStylePos::Index(i) => {
                                    if *i >= self.edges.len() {
                                        return Err(Error::DiagramParse {
                                            diagram_type: self.diagram_type.to_string(),
                                            message: format!(
                                                "The index {i} for linkStyle is out of bounds. Valid indices for linkStyle are between 0 and {}. (Help: Ensure that the index is within the range of existing edges.)",
                                                self.edges.len().saturating_sub(1)
                                            ),
                                        });
                                    }
                                    self.edges[*i].style = ls.styles.clone();
                                    if !self.edges[*i].style.is_empty()
                                        && !self.edges[*i]
                                            .style
                                            .iter()
                                            .any(|s| s.trim_start().starts_with("fill"))
                                    {
                                        self.edges[*i].style.push("fill:none".to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                Stmt::ShapeData { target, yaml } => {
                    // Mermaid syntax uses the same `@{...}` form for both nodes and edges:
                    // - if an edge with the given ID exists, it updates the edge metadata
                    // - otherwise it updates (and may create) a node
                    let v = parse_shape_data_yaml(yaml).map_err(|e| Error::DiagramParse {
                        diagram_type: self.diagram_type.to_string(),
                        message: format!("Invalid shapeData: {e}"),
                    })?;

                    let map = v.as_mapping();
                    let is_edge_target = self
                        .edges
                        .iter()
                        .any(|e| e.id.as_deref() == Some(target.as_str()));
                    if is_edge_target {
                        if let Some(map) = map {
                            for e in self.edges.iter_mut() {
                                if e.id.as_deref() != Some(target.as_str()) {
                                    continue;
                                }
                                for (k, v) in map {
                                    let Some(key) = k.as_str() else { continue };
                                    match key {
                                        "animate" => {
                                            if let Some(b) = yaml_to_bool(v) {
                                                e.animate = Some(b);
                                            }
                                        }
                                        "animation" => {
                                            if let Some(s) = yaml_to_string(v) {
                                                e.animation = Some(s);
                                            }
                                        }
                                        "curve" => {
                                            if let Some(s) = yaml_to_string(v) {
                                                e.interpolate = Some(s);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        continue;
                    }

                    let idx = self.ensure_node(target);
                    apply_shape_data_to_node(&mut self.nodes[idx], yaml).map_err(|e| {
                        Error::DiagramParse {
                            diagram_type: self.diagram_type.to_string(),
                            message: e,
                        }
                    })?;
                }
                Stmt::Chain { .. } | Stmt::Node(_) | Stmt::Direction(_) => {}
            }
        }
        Ok(())
    }

    fn add_class_to_target(&mut self, target: &str, class_name: &str) {
        if let Some(&idx) = self.subgraph_index.get(target) {
            self.subgraphs[idx].classes.push(class_name.to_string());
        }
        if let Some(&idx) = self.node_index.get(target) {
            self.nodes[idx].classes.push(class_name.to_string());
        }
        for e in self.edges.iter_mut() {
            if e.id.as_deref() == Some(target) {
                e.classes.push(class_name.to_string());
            }
        }
    }

    fn ensure_node(&mut self, id: &str) -> usize {
        if let Some(&idx) = self.node_index.get(id) {
            return idx;
        }
        let idx = self.nodes.len();
        self.nodes.push(Node {
            id: id.to_string(),
            label: None,
            label_type: TitleKind::Text,
            shape: None,
            shape_data: None,
            icon: None,
            form: None,
            pos: None,
            img: None,
            constraint: None,
            asset_width: None,
            asset_height: None,
            styles: Vec::new(),
            classes: Vec::new(),
            link: None,
            link_target: None,
            have_callback: false,
        });
        self.node_index.insert(id.to_string(), idx);
        idx
    }
}
