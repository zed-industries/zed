use crate::error::{Error, Result};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// Optional compound node definitions (e.g. Mermaid Architecture groups).
    pub compounds: Vec<Compound>,
}

impl Graph {
    pub fn validate(&self) -> Result<()> {
        let mut node_exists: FxHashSet<&str> = FxHashSet::default();
        node_exists.reserve(self.nodes.len().saturating_mul(2));
        for n in &self.nodes {
            node_exists.insert(n.id.as_str());
        }

        let mut compound_exists: FxHashSet<&str> = FxHashSet::default();
        compound_exists.reserve(self.compounds.len().saturating_mul(2));
        for c in &self.compounds {
            compound_exists.insert(c.id.as_str());
        }

        for n in &self.nodes {
            if let Some(p) = n.parent.as_deref() {
                if !compound_exists.contains(p) {
                    return Err(Error::MissingEndpoint {
                        edge_id: format!("node-parent:{}/{}", n.id, p),
                    });
                }
            }
        }
        for c in &self.compounds {
            if let Some(p) = c.parent.as_deref() {
                if !compound_exists.contains(p) {
                    return Err(Error::MissingEndpoint {
                        edge_id: format!("compound-parent:{}/{}", c.id, p),
                    });
                }
            }
        }
        for e in &self.edges {
            if !node_exists.contains(e.source.as_str()) || !node_exists.contains(e.target.as_str())
            {
                return Err(Error::MissingEndpoint {
                    edge_id: e.id.clone(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    /// Optional parent (compound node) id, mirroring Cytoscape's compound node model.
    ///
    /// This is currently used as structural metadata for higher-fidelity FCoSE parity work.
    pub parent: Option<String>,
    pub width: f64,
    pub height: f64,
    /// Optional initial position (center), mirroring Cytoscape's `position` field.
    pub x: f64,
    pub y: f64,
    /// Optional extra bounds (e.g. label extents) included in `eles.boundingBox()`-style
    /// computations (relocation centering, compound sizing wrt labels).
    ///
    /// These extras expand the node's rect bounds without changing its layout size (`width`,
    /// `height`). Values are expected to be non-negative.
    pub bounds_extras: BoundsExtras,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BoundsExtras {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone)]
pub struct Compound {
    pub id: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    /// Optional edge label dimensions (CSS px) used when approximating Cytoscape
    /// `edge.boundingBox()` / `eles.boundingBox()` during relocation centering.
    ///
    /// Mermaid Architecture uses Cytoscape edge labels for layout-time bounding boxes, even
    /// though the final SVG is rendered separately. Storing the measured label size here keeps
    /// the layout crate renderer-agnostic while still enabling parity-level centering.
    pub label_width: Option<f64>,
    pub label_height: Option<f64>,
    /// Optional endpoint anchors used by layout algorithms that model port-like attachments.
    ///
    /// Mermaid Architecture uses Cytoscape edge endpoints (e.g. `0 50%`, `100% 50%`) to force
    /// horizontal/vertical edges. We model this with a small discrete anchor set.
    pub source_anchor: Option<Anchor>,
    pub target_anchor: Option<Anchor>,
    /// Optional ideal edge length (border-to-border) used by algorithms that model edge springs.
    /// When unset or non-positive, the algorithm's default is used.
    pub ideal_length: f64,
    /// Optional spring strength for this edge (Cytoscape FCoSE: `edgeElasticity`).
    /// When unset or non-positive, the algorithm's default is used.
    pub elasticity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutRect {
    /// Top-left x coordinate in the layout engine coordinate space.
    pub left: f64,
    /// Top-left y coordinate in the layout engine coordinate space.
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub positions: std::collections::BTreeMap<String, Point>,
}
