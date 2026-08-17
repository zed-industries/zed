use crate::sanitize::sanitize_text;
use crate::{Error, MermaidConfig, Result};
use serde_json::{Map, Value, json};

use super::render_model::{MindmapDiagramRenderEdge, MindmapDiagramRenderNode};
use super::utils::get_i64;
use super::{
    NODE_TYPE_BANG, NODE_TYPE_CIRCLE, NODE_TYPE_CLOUD, NODE_TYPE_DEFAULT, NODE_TYPE_HEXAGON,
    NODE_TYPE_RECT, NODE_TYPE_ROUNDED_RECT,
};

fn mindmap_look(config: &MermaidConfig) -> String {
    config.get_str("look").unwrap_or("classic").to_string()
}

fn mindmap_default_shape(config: &MermaidConfig) -> &'static str {
    let theme = config
        .get_str("theme")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if theme.contains("redux") {
        "rounded"
    } else {
        "defaultMindmapNode"
    }
}

fn shape_from_type(ty: i32, default_shape: &'static str) -> &'static str {
    match ty {
        NODE_TYPE_CIRCLE => "mindmapCircle",
        NODE_TYPE_RECT => "rect",
        NODE_TYPE_ROUNDED_RECT => "rounded",
        NODE_TYPE_CLOUD => "cloud",
        NODE_TYPE_BANG => "bang",
        NODE_TYPE_HEXAGON => "hexagon",
        NODE_TYPE_DEFAULT => default_shape,
        _ => "rect",
    }
}

#[derive(Debug, Clone)]
pub(super) struct MindmapNode {
    pub(super) id: i32,
    pub(super) node_id: String,
    pub(super) level: i32,
    pub(super) descr: String,
    pub(super) is_markdown: bool,
    pub(super) ty: i32,
    pub(super) children: Vec<i32>,
    pub(super) width: i64,
    pub(super) padding: i64,
    pub(super) section: Option<i32>,
    pub(super) height: Option<i64>,
    pub(super) class: Option<String>,
    pub(super) icon: Option<String>,
    pub(super) x: Option<f64>,
    pub(super) y: Option<f64>,
    pub(super) is_root: bool,
}

fn mindmap_node_css_classes(node: &MindmapNode) -> String {
    let mut css = vec!["mindmap-node".to_string()];
    if node.is_root {
        css.push("section-root".to_string());
        css.push("section--1".to_string());
    } else if let Some(section) = node.section {
        css.push(format!("section-{section}"));
    }
    if let Some(cls) = &node.class {
        css.push(cls.clone());
    }
    css.join(" ")
}

fn mindmap_layout_node_value(node: &MindmapNode, look: &str, default_shape: &'static str) -> Value {
    let mut map = Map::new();
    map.insert("id".to_string(), json!(node.id.to_string()));
    map.insert("domId".to_string(), json!(format!("node_{}", node.id)));
    map.insert("label".to_string(), json!(node.descr));
    if node.is_markdown {
        map.insert("labelType".to_string(), json!("markdown"));
    }
    map.insert("isGroup".to_string(), json!(false));
    map.insert(
        "shape".to_string(),
        json!(shape_from_type(node.ty, default_shape)),
    );
    map.insert("width".to_string(), json!(node.width));
    map.insert("height".to_string(), json!(node.height.unwrap_or(0)));
    // Keep the DB padding in the semantic model (matches Mermaid mindmapDb.getData()).
    // Shape-specific padding overrides happen at render time (see `mindmap_render_node`).
    map.insert("padding".to_string(), json!(node.padding));
    map.insert(
        "cssClasses".to_string(),
        json!(mindmap_node_css_classes(node)),
    );
    map.insert("cssStyles".to_string(), Value::Array(Vec::new()));
    map.insert("look".to_string(), json!(look));

    if let Some(icon) = &node.icon {
        map.insert("icon".to_string(), json!(icon));
    }
    if let Some(x) = node.x {
        map.insert("x".to_string(), json!(x));
    }
    if let Some(y) = node.y {
        map.insert("y".to_string(), json!(y));
    }

    map.insert("level".to_string(), json!(node.level));
    map.insert("nodeId".to_string(), json!(node.node_id));
    map.insert("type".to_string(), json!(node.ty));
    if let Some(section) = node.section {
        map.insert("section".to_string(), json!(section));
    }

    Value::Object(map)
}

fn mindmap_render_node(
    node: &MindmapNode,
    look: &str,
    default_shape: &'static str,
) -> MindmapDiagramRenderNode {
    let padding = if node.ty == NODE_TYPE_ROUNDED_RECT {
        15_f64
    } else {
        node.padding as f64
    };

    MindmapDiagramRenderNode {
        id: node.id.to_string(),
        dom_id: format!("node_{}", node.id),
        label: node.descr.clone(),
        label_type: if node.is_markdown {
            "markdown".to_string()
        } else {
            String::new()
        },
        is_group: false,
        shape: shape_from_type(node.ty, default_shape).to_string(),
        width: node.width as f64,
        height: node.height.unwrap_or(0) as f64,
        padding,
        css_classes: mindmap_node_css_classes(node),
        css_styles: Vec::new(),
        look: look.to_string(),
        icon: node.icon.clone(),
        x: node.x,
        y: node.y,
        level: node.level as i64,
        node_id: node.node_id.clone(),
        node_type: node.ty,
        section: node.section,
    }
}

fn mindmap_edge_classes(parent: &MindmapNode, child: &MindmapNode) -> String {
    let mut classes = "edge".to_string();
    if let Some(section) = child.section {
        classes.push_str(&format!(" section-edge-{section}"));
    }
    let edge_depth = parent.level + 1;
    classes.push_str(&format!(" edge-depth-{edge_depth}"));
    classes
}

fn mindmap_edge_value(parent: &MindmapNode, child: &MindmapNode, look: &str) -> Value {
    let mut map = Map::new();
    map.insert(
        "id".to_string(),
        json!(format!("edge_{}_{}", parent.id, child.id)),
    );
    map.insert("start".to_string(), json!(parent.id.to_string()));
    map.insert("end".to_string(), json!(child.id.to_string()));
    map.insert("type".to_string(), json!("normal"));
    map.insert("curve".to_string(), json!("basis"));
    map.insert("thickness".to_string(), json!("normal"));
    map.insert("look".to_string(), json!(look));
    map.insert(
        "classes".to_string(),
        json!(mindmap_edge_classes(parent, child)),
    );
    map.insert("depth".to_string(), json!(parent.level));
    if let Some(section) = child.section {
        map.insert("section".to_string(), json!(section));
    }
    Value::Object(map)
}

fn mindmap_render_edge(
    parent: &MindmapNode,
    child: &MindmapNode,
    look: &str,
) -> MindmapDiagramRenderEdge {
    MindmapDiagramRenderEdge {
        id: format!("edge_{}_{}", parent.id, child.id),
        start: parent.id.to_string(),
        end: child.id.to_string(),
        edge_type: "normal".to_string(),
        curve: "basis".to_string(),
        thickness: "normal".to_string(),
        look: look.to_string(),
        classes: mindmap_edge_classes(parent, child),
        depth: parent.level as i64,
        section: child.section,
    }
}

#[derive(Debug, Default)]
pub(super) struct MindmapDb {
    pub(super) nodes: Vec<MindmapNode>,
    base_level: Option<i32>,
}

pub(super) struct MindmapNodeInput<'a> {
    pub(super) indent_level: i32,
    pub(super) id_raw: &'a str,
    pub(super) descr_raw: &'a str,
    pub(super) descr_is_markdown: bool,
    pub(super) ty: i32,
    pub(super) diagram_type: &'a str,
}

impl MindmapDb {
    pub(super) fn clear(&mut self) {
        self.nodes.clear();
        self.base_level = None;
    }

    pub(super) fn get_mindmap(&self) -> Option<&MindmapNode> {
        self.nodes.first()
    }

    fn get_parent_index(&self, level: i32) -> Option<usize> {
        self.nodes.iter().rposition(|n| n.level < level)
    }

    pub(super) fn add_node(
        &mut self,
        input: MindmapNodeInput<'_>,
        config: &MermaidConfig,
    ) -> Result<()> {
        let mut level = input.indent_level;
        let is_root;
        if self.nodes.is_empty() {
            self.base_level = Some(level);
            level = 0;
            is_root = true;
        } else if let Some(base) = self.base_level {
            level -= base;
            is_root = false;
        } else {
            is_root = false;
        }

        let mut padding = get_i64(config, "mindmap.padding").unwrap_or(10);
        let width = get_i64(config, "mindmap.maxNodeWidth").unwrap_or(200);

        match input.ty {
            NODE_TYPE_ROUNDED_RECT | NODE_TYPE_RECT | NODE_TYPE_HEXAGON => {
                padding *= 2;
            }
            _ => {}
        }

        let id = self.nodes.len() as i32;
        let node = MindmapNode {
            id,
            node_id: sanitize_text(input.id_raw, config),
            level,
            descr: if input.descr_is_markdown {
                input.descr_raw.to_string()
            } else {
                sanitize_text(input.descr_raw, config)
            },
            is_markdown: input.descr_is_markdown,
            ty: input.ty,
            children: Vec::new(),
            width,
            padding,
            section: None,
            height: None,
            class: None,
            icon: None,
            x: None,
            y: None,
            is_root,
        };

        if let Some(parent_idx) = self.get_parent_index(level) {
            self.nodes[parent_idx].children.push(id);
            self.nodes.push(node);
            return Ok(());
        }

        if is_root {
            self.nodes.push(node);
            return Ok(());
        }

        Err(Error::DiagramParse {
            diagram_type: input.diagram_type.to_string(),
            message: format!(
                "There can be only one root. No parent could be found for (\"{}\")",
                node.descr
            ),
        })
    }

    pub(super) fn decorate_last(
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
            last.class = Some(sanitize_text(&class, config));
        }
    }

    pub(super) fn assign_sections(&mut self, node_id: i32, section: Option<i32>) {
        let mut stack = vec![(node_id, section)];
        while let Some((node_id, section)) = stack.pop() {
            let Ok(node_idx) = usize::try_from(node_id) else {
                continue;
            };
            let Some(node) = self.nodes.get_mut(node_idx) else {
                continue;
            };
            let node_level = node.level;
            if node_level == 0 {
                node.section = None;
            } else {
                node.section = section;
            }

            let children = node.children.clone();
            for (index, child_id) in children.into_iter().enumerate().rev() {
                let child_section = if node_level == 0 {
                    Some(index as i32)
                } else {
                    section
                };
                stack.push((child_id, child_section));
            }
        }
    }

    pub(super) fn to_root_node_value(&self, node_id: i32) -> Value {
        let Ok(node_idx) = usize::try_from(node_id) else {
            return Value::Null;
        };
        if self.nodes.get(node_idx).is_none() {
            return Value::Null;
        }

        let mut values = vec![None; self.nodes.len()];
        let mut stack = vec![(node_id, false)];
        while let Some((node_id, visited)) = stack.pop() {
            let Ok(node_idx) = usize::try_from(node_id) else {
                continue;
            };
            let Some(node) = self.nodes.get(node_idx) else {
                continue;
            };

            if visited {
                let mut map = Map::new();
                map.insert("id".to_string(), json!(node.id));
                map.insert("nodeId".to_string(), json!(node.node_id));
                map.insert("level".to_string(), json!(node.level));
                map.insert("descr".to_string(), json!(node.descr));
                map.insert("type".to_string(), json!(node.ty));
                let children = node
                    .children
                    .iter()
                    .map(|child_id| {
                        let Ok(child_idx) = usize::try_from(*child_id) else {
                            return Value::Null;
                        };
                        values
                            .get_mut(child_idx)
                            .and_then(Option::take)
                            .unwrap_or(Value::Null)
                    })
                    .collect();
                map.insert("children".to_string(), Value::Array(children));
                map.insert("width".to_string(), json!(node.width));
                map.insert("padding".to_string(), json!(node.padding));

                if let Some(section) = node.section {
                    map.insert("section".to_string(), json!(section));
                }
                if let Some(height) = node.height {
                    map.insert("height".to_string(), json!(height));
                }
                if let Some(class) = &node.class {
                    map.insert("class".to_string(), json!(class));
                }
                if let Some(icon) = &node.icon {
                    map.insert("icon".to_string(), json!(icon));
                }
                if let Some(x) = node.x {
                    map.insert("x".to_string(), json!(x));
                }
                if let Some(y) = node.y {
                    map.insert("y".to_string(), json!(y));
                }
                if node.is_root {
                    map.insert("isRoot".to_string(), json!(true));
                }

                if let Some(slot) = values.get_mut(node_idx) {
                    *slot = Some(Value::Object(map));
                }
            } else {
                stack.push((node_id, true));
                for child_id in node.children.iter().rev() {
                    stack.push((*child_id, false));
                }
            }
        }

        values
            .get_mut(node_idx)
            .and_then(Option::take)
            .unwrap_or(Value::Null)
    }

    pub(super) fn to_layout_node_values(&self, root_id: i32, config: &MermaidConfig) -> Vec<Value> {
        let mut out = Vec::new();
        let look = mindmap_look(config);
        let default_shape = mindmap_default_shape(config);
        let mut stack = vec![root_id];
        while let Some(node_id) = stack.pop() {
            let Ok(node_idx) = usize::try_from(node_id) else {
                continue;
            };
            let Some(node) = self.nodes.get(node_idx) else {
                continue;
            };

            out.push(mindmap_layout_node_value(node, &look, default_shape));

            for child in node.children.iter().rev() {
                stack.push(*child);
            }
        }
        out
    }

    pub(super) fn to_layout_nodes_for_render(
        &self,
        root_id: i32,
        config: &MermaidConfig,
    ) -> Vec<MindmapDiagramRenderNode> {
        let mut out = Vec::new();
        let look = mindmap_look(config);
        let default_shape = mindmap_default_shape(config);
        let mut stack = vec![root_id];
        while let Some(node_id) = stack.pop() {
            let Ok(node_idx) = usize::try_from(node_id) else {
                continue;
            };
            let Some(node) = self.nodes.get(node_idx) else {
                continue;
            };

            out.push(mindmap_render_node(node, &look, default_shape));

            for child in node.children.iter().rev() {
                stack.push(*child);
            }
        }
        out
    }

    pub(super) fn to_edge_values(&self, root_id: i32, config: &MermaidConfig) -> Vec<Value> {
        struct EdgeFrame {
            node_id: i32,
            next_child_index: usize,
        }

        let mut edges = Vec::new();
        let look = mindmap_look(config);
        let mut stack = vec![EdgeFrame {
            node_id: root_id,
            next_child_index: 0,
        }];
        while let Some(frame) = stack.last_mut() {
            let Ok(node_idx) = usize::try_from(frame.node_id) else {
                stack.pop();
                continue;
            };
            let Some(node) = self.nodes.get(node_idx) else {
                stack.pop();
                continue;
            };
            let Some(child_id) = node.children.get(frame.next_child_index).copied() else {
                stack.pop();
                continue;
            };
            frame.next_child_index += 1;

            let Ok(child_idx) = usize::try_from(child_id) else {
                continue;
            };
            let Some(child) = self.nodes.get(child_idx) else {
                continue;
            };

            edges.push(mindmap_edge_value(node, child, &look));
            stack.push(EdgeFrame {
                node_id: child_id,
                next_child_index: 0,
            });
        }
        edges
    }

    pub(super) fn to_edges_for_render(
        &self,
        root_id: i32,
        config: &MermaidConfig,
    ) -> Vec<MindmapDiagramRenderEdge> {
        struct EdgeFrame {
            node_id: i32,
            next_child_index: usize,
        }

        let mut edges = Vec::new();
        let look = mindmap_look(config);
        let mut stack = vec![EdgeFrame {
            node_id: root_id,
            next_child_index: 0,
        }];
        while let Some(frame) = stack.last_mut() {
            let Ok(node_idx) = usize::try_from(frame.node_id) else {
                stack.pop();
                continue;
            };
            let Some(node) = self.nodes.get(node_idx) else {
                stack.pop();
                continue;
            };
            let Some(child_id) = node.children.get(frame.next_child_index).copied() else {
                stack.pop();
                continue;
            };
            frame.next_child_index += 1;

            let Ok(child_idx) = usize::try_from(child_id) else {
                continue;
            };
            let Some(child) = self.nodes.get(child_idx) else {
                continue;
            };

            edges.push(mindmap_render_edge(node, child, &look));
            stack.push(EdgeFrame {
                node_id: child_id,
                next_child_index: 0,
            });
        }
        edges
    }
}
