use super::{Edge, Node, Stmt, TitleKind, apply_shape_data_to_node};
use std::collections::{HashMap, HashSet};

pub(super) struct FlowchartBuildState {
    pub(super) nodes: Vec<Node>,
    pub(super) node_index: HashMap<String, usize>,
    pub(super) edges: Vec<Edge>,
    pub(super) used_edge_ids: HashSet<String>,
    pub(super) edge_pair_counts: HashMap<(String, String), usize>,
    pub(super) vertex_calls: Vec<String>,
}

impl FlowchartBuildState {
    pub(super) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_index: HashMap::new(),
            edges: Vec::new(),
            used_edge_ids: HashSet::new(),
            edge_pair_counts: HashMap::new(),
            vertex_calls: Vec::new(),
        }
    }

    pub(super) fn add_statements(
        &mut self,
        statements: &[Stmt],
    ) -> std::result::Result<(), String> {
        // Keep Mermaid's preorder statement handling without using the Rust call stack for
        // deeply nested subgraphs.
        let mut stack = vec![statements.iter()];
        while let Some(iter) = stack.last_mut() {
            let Some(stmt) = iter.next() else {
                stack.pop();
                continue;
            };

            match stmt {
                Stmt::Chain { nodes, edges } => {
                    let mut deferred_shape_data_vertex_calls: Vec<String> = Vec::new();
                    for mut n in nodes.iter().cloned() {
                        // Mermaid FlowDB `vertexCounter` increments on every `addVertex(...)` call.
                        // Our grammar models `shapeData` attachments in the AST, so we can replay the
                        // observable call sequence:
                        // - once for the vertex token itself
                        // - once more if a `@{ ... }` shapeData block is present
                        self.vertex_calls.push(n.id.clone());
                        if n.shape_data.is_some() {
                            // For multi-vertex statements (notably `&`-separated nodes), the upstream
                            // parser's reduction order can apply shapeData after the statement's
                            // vertices have already been introduced. Record these shapeData calls
                            // after we've visited every vertex in the statement.
                            deferred_shape_data_vertex_calls.push(n.id.clone());
                        }
                        if let Some(sd) = n.shape_data.take() {
                            apply_shape_data_to_node(&mut n, &sd)?;
                        }
                        self.upsert_node(n);
                    }
                    self.vertex_calls
                        .extend(deferred_shape_data_vertex_calls.into_iter());
                    for e in edges.iter().cloned() {
                        self.push_edge(e);
                    }
                }
                Stmt::Node(n) => {
                    let mut n = n.as_ref().clone();
                    self.vertex_calls.push(n.id.clone());
                    if n.shape_data.is_some() {
                        self.vertex_calls.push(n.id.clone());
                    }
                    if let Some(sd) = n.shape_data.take() {
                        apply_shape_data_to_node(&mut n, &sd)?;
                    }
                    self.upsert_node(n);
                }
                Stmt::ShapeData { target, .. } => {
                    // Mermaid applies shapeData to edges if (and only if) an edge with that ID exists.
                    // For ordering parity we only insert a placeholder node when this currently refers to a node.
                    if !self.used_edge_ids.contains(target) {
                        // The upstream flowchart parser calls `addVertex(id)` and then
                        // `addVertex(id, ..., shapeData)` for `id@{...}` statements.
                        self.vertex_calls.push(target.clone());
                        self.vertex_calls.push(target.clone());
                    }
                    if !self.used_edge_ids.contains(target) && !self.node_index.contains_key(target)
                    {
                        let idx = self.nodes.len();
                        self.nodes.push(Node {
                            id: target.clone(),
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
                        self.node_index.insert(target.clone(), idx);
                    }
                }
                Stmt::Subgraph(sg) => stack.push(sg.statements.iter()),
                Stmt::Direction(_)
                | Stmt::ClassDef(_)
                | Stmt::ClassAssign(_)
                | Stmt::Click(_)
                | Stmt::LinkStyle(_) => {}
                Stmt::Style(s) => {
                    // Mermaid's `style` statement routes through FlowDB `addVertex(id, ..., styles)`.
                    // This increments `vertexCounter` for nodes (but is a no-op for edges).
                    if !self.used_edge_ids.contains(&s.target) {
                        self.vertex_calls.push(s.target.clone());
                        if !self.node_index.contains_key(&s.target) {
                            let idx = self.nodes.len();
                            self.nodes.push(Node {
                                id: s.target.clone(),
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
                            self.node_index.insert(s.target.clone(), idx);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn upsert_node(&mut self, n: Node) {
        if let Some(&idx) = self.node_index.get(&n.id) {
            if n.label.is_some() {
                self.nodes[idx].label = n.label;
                self.nodes[idx].label_type = n.label_type;
            }
            if n.shape.is_some() {
                self.nodes[idx].shape = n.shape;
            }
            if n.icon.is_some() {
                self.nodes[idx].icon = n.icon;
            }
            if n.form.is_some() {
                self.nodes[idx].form = n.form;
            }
            if n.pos.is_some() {
                self.nodes[idx].pos = n.pos;
            }
            if n.img.is_some() {
                self.nodes[idx].img = n.img;
            }
            if n.constraint.is_some() {
                self.nodes[idx].constraint = n.constraint;
            }
            if n.asset_width.is_some() {
                self.nodes[idx].asset_width = n.asset_width;
            }
            if n.asset_height.is_some() {
                self.nodes[idx].asset_height = n.asset_height;
            }
            self.nodes[idx].styles.extend(n.styles);
            self.nodes[idx].classes.extend(n.classes);
            return;
        }
        let idx = self.nodes.len();
        self.node_index.insert(n.id.clone(), idx);
        self.nodes.push(n);
    }

    fn push_edge(&mut self, mut e: Edge) {
        let key = (e.from.clone(), e.to.clone());
        let existing = *self.edge_pair_counts.get(&key).unwrap_or(&0);

        let mut final_id = e.id.clone();
        let mut is_user_defined_id = false;
        if let Some(user_id) = e.id.clone() {
            if !self.used_edge_ids.contains(&user_id) {
                is_user_defined_id = true;
                self.used_edge_ids.insert(user_id);
            } else {
                final_id = None;
            }
        }

        if final_id.is_none() {
            let counter = if existing == 0 { 0 } else { existing + 1 };
            final_id = Some(format!("L_{}_{}_{}", e.from, e.to, counter));
            if let Some(id) = final_id.clone() {
                self.used_edge_ids.insert(id);
            }
        }

        self.edge_pair_counts.insert(key, existing + 1);

        e.id = final_id;
        e.is_user_defined_id = is_user_defined_id;
        e.link.length = e.link.length.min(10);
        self.edges.push(e);
    }
}
